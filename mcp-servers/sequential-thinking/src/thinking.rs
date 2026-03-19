use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::logging::PersistentLogger;
use crate::profiles::TuningProfile;

// ============================================================================
// ThoughtData — all fields from the TS interface
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThoughtData {
    // Required
    pub thought: String,
    pub thought_number: u32,
    pub total_thoughts: u32,
    #[serde(default)]
    pub next_thought_needed: bool,

    // Original optional (promoted to first-class)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_revision: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revises_thought: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_from_thought: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needs_more_thoughts: Option<bool>,

    // Wide exploration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explore_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposals: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegate_to_next_layer: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_strategy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub done_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_window: Option<String>,

    // Search-aware
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_context: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incorporate_search: Option<bool>,
}

// ============================================================================
// Compliance tracking
// ============================================================================

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComplianceStats {
    pub consecutive_linear_thoughts: u32,
    pub low_conf_without_branch_count: u32,
    pub explore_count_used: bool,
    pub needs_branching: bool,
}

// ============================================================================
// ThinkingEngine — core processing logic
// ============================================================================

pub struct ThinkingEngine {
    thought_history: Vec<ThoughtData>,
    branches: HashMap<String, Vec<ThoughtData>>,
    profile: TuningProfile,
    model_id: String,
    client_type: String,
    disable_logging: bool,
    logger: PersistentLogger,

    // Compliance counters
    consecutive_linear_thoughts: u32,
    low_conf_without_branch_count: u32,
    explore_count_usage_count: u32,
}

impl ThinkingEngine {
    pub fn new(profile: TuningProfile, model_id: String, client_type: String) -> Self {
        let disable_logging = std::env::var("DISABLE_THOUGHT_LOGGING")
            .map(|v| v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);

        let logger = PersistentLogger::new(&model_id, &client_type, &profile.display_name);

        Self {
            thought_history: Vec::new(),
            branches: HashMap::new(),
            profile,
            model_id,
            client_type,
            disable_logging,
            logger,
            consecutive_linear_thoughts: 0,
            low_conf_without_branch_count: 0,
            explore_count_usage_count: 0,
        }
    }

    pub fn profile(&self) -> &TuningProfile {
        &self.profile
    }

    /// Validate and clamp input fields, returning a clean ThoughtData.
    fn validate(&self, mut data: ThoughtData) -> Result<ThoughtData, String> {
        if data.thought.is_empty() {
            return Err("Invalid thought: must be a non-empty string".into());
        }
        if data.thought_number == 0 {
            return Err("Invalid thoughtNumber: must be >= 1".into());
        }
        if data.total_thoughts == 0 {
            return Err("Invalid totalThoughts: must be >= 1".into());
        }

        // Derive nextThoughtNeeded from continuationMode if not explicitly set
        if let Some(ref mode) = data.continuation_mode {
            // If the caller provided continuationMode, use it to derive nextThoughtNeeded
            data.next_thought_needed = mode != "done";
        }
        // If neither continuationMode nor a meaningful nextThoughtNeeded — default true
        // (backwards compat: the TS version required one or the other, but we're lenient)

        // Clamp exploreCount
        if let Some(ref mut ec) = data.explore_count {
            *ec = (*ec).clamp(1, self.profile.max_explore_count);
        }

        // Clamp confidence
        if let Some(ref mut c) = data.confidence {
            *c = c.clamp(0.0, 1.0);
        }

        // Clamp layer
        if let Some(ref mut l) = data.layer {
            *l = (*l).clamp(1, 5);
        }

        // Auto-adjust totalThoughts if exceeded
        if data.thought_number > data.total_thoughts {
            data.total_thoughts = data.thought_number;
        }

        Ok(data)
    }

    /// Format a thought for stderr display (plain text, Unicode box drawing).
    fn format_thought(&self, data: &ThoughtData) -> String {
        let mut lines = Vec::new();

        // Header
        let prefix;
        let context;
        if data.is_revision.unwrap_or(false) {
            prefix = ">> Revision";
            context = format!(
                " (revising thought {})",
                data.revises_thought.unwrap_or(0)
            );
        } else if data.branch_from_thought.is_some() {
            prefix = "~> Branch";
            context = format!(
                " (from thought {}, ID: {})",
                data.branch_from_thought.unwrap_or(0),
                data.branch_id.as_deref().unwrap_or("?")
            );
        } else {
            prefix = ".. Thought";
            context = String::new();
        }

        let header = format!(
            "{} {}/{}{}",
            prefix, data.thought_number, data.total_thoughts, context
        );

        // Extras line
        let mut extras = Vec::new();
        if let Some(layer) = data.layer {
            extras.push(format!("Layer {}", layer));
        }
        if data.continuation_mode.as_deref() == Some("explore") {
            if let Some(ec) = data.explore_count {
                extras.push(format!("Exploring {} alternatives", ec));
            }
        }
        if let Some(conf) = data.confidence {
            extras.push(format!("{}% confident", (conf * 100.0).round() as u32));
        }
        if data.delegate_to_next_layer.unwrap_or(false) {
            extras.push("Delegating to next layer".into());
        }
        if let Some(ref sq) = data.search_query {
            extras.push(format!("Search: \"{}\"", sq));
        }

        let width = 64;
        let border: String = std::iter::repeat('-').take(width + 4).collect();

        lines.push(format!("+{}+", border));
        lines.push(format!("| {:<width$} |", header, width = width + 2));
        if !extras.is_empty() {
            lines.push(format!("| {:<width$} |", extras.join(" | "), width = width + 2));
        }
        lines.push(format!("+{}+", border));

        // Wrap thought text
        for wrapped in wrap_text(&data.thought, width) {
            lines.push(format!("| {:<width$} |", wrapped, width = width + 2));
        }

        // Proposals
        if let Some(ref proposals) = data.proposals {
            if !proposals.is_empty() {
                lines.push(format!("+{}+", border));
                for (i, p) in proposals.iter().enumerate() {
                    lines.push(format!(
                        "| Option {}: {:<width$} |",
                        i + 1,
                        p,
                        width = width - 8 - i.to_string().len()
                    ));
                }
            }
        }

        lines.push(format!("+{}+", border));

        // Confidence bar
        if let Some(conf) = data.confidence {
            let filled = (conf * 20.0).floor() as usize;
            let empty = 20 - filled;
            let bar: String = "#".repeat(filled) + &".".repeat(empty);
            let pct = (conf * 100.0).round() as u32;
            if conf < 0.5 {
                lines.push(format!(
                    "  CONFIDENCE [{}] {}%  LOW - CONSIDER BRANCHING",
                    bar, pct
                ));
            } else {
                lines.push(format!("  CONFIDENCE [{}] {}%", bar, pct));
            }
        } else {
            lines.push(format!(
                "  CONFIDENCE: NOT PROVIDED (thought {})",
                data.thought_number
            ));
        }

        lines.join("\n")
    }

    /// Process a thought and return the JSON response.
    pub fn process(
        &mut self,
        data: ThoughtData,
    ) -> Result<serde_json::Value, String> {
        let validated = self.validate(data)?;

        self.thought_history.push(validated.clone());

        // Persist to JSONL log
        self.logger.persist(&validated);

        // Track branches
        if let (Some(_from), Some(ref bid)) =
            (validated.branch_from_thought, &validated.branch_id)
        {
            self.branches
                .entry(bid.clone())
                .or_default()
                .push(validated.clone());
            self.consecutive_linear_thoughts = 0;
        } else {
            self.consecutive_linear_thoughts += 1;
        }

        // Track explore_count usage
        if validated.explore_count.unwrap_or(0) > 1 {
            self.explore_count_usage_count += 1;
        }

        // Track low-confidence without branch
        if let Some(conf) = validated.confidence {
            if conf < self.profile.branching_threshold && validated.branch_from_thought.is_none() {
                self.low_conf_without_branch_count += 1;
            }
        }

        // Formatted output to stderr
        if !self.disable_logging {
            let formatted = self.format_thought(&validated);
            eprintln!("{}", formatted);

            // Compliance warnings
            if self.consecutive_linear_thoughts >= 4 {
                eprintln!(
                    "\n  LINEAR CHAIN DETECTED: {} thoughts\n  \
                     SKILL SAYS: \"1 branch per 4 linear thoughts\"\n  \
                     USE: branchFromThought + branchId NOW\n",
                    self.consecutive_linear_thoughts
                );
            }

            if self.explore_count_usage_count == 0 && validated.thought_number >= 3 {
                eprintln!(
                    "  [HINT] exploreCount is available but unused. Try: exploreCount: 4, proposals: [...]"
                );
            }

            if self.low_conf_without_branch_count >= 2 {
                eprintln!(
                    "  [COMPLIANCE] {} low-confidence thoughts without branching (target: <30%)",
                    self.low_conf_without_branch_count
                );
            }
        }

        // Build response
        let branch_keys: Vec<String> = self.branches.keys().cloned().collect();
        let compliance = ComplianceStats {
            consecutive_linear_thoughts: self.consecutive_linear_thoughts,
            low_conf_without_branch_count: self.low_conf_without_branch_count,
            explore_count_used: self.explore_count_usage_count > 0,
            needs_branching: self.consecutive_linear_thoughts >= 4,
        };

        let mut response = serde_json::json!({
            "thoughtNumber": validated.thought_number,
            "totalThoughts": validated.total_thoughts,
            "nextThoughtNeeded": validated.next_thought_needed,
            "branches": branch_keys,
            "thoughtHistoryLength": self.thought_history.len(),
            "compliance": compliance,
        });

        // First-call guidance
        if validated.thought_number == 1 {
            response["firstCallGuidance"] = serde_json::Value::String(
                first_call_guidance(&self.profile),
            );
        }

        // Search query passthrough
        if let Some(ref sq) = validated.search_query {
            response["pendingSearchQuery"] = serde_json::Value::String(sq.clone());
            response["hint"] =
                serde_json::Value::String("Agent should execute search before next thought".into());
        }

        // Confidence-based guidance
        if let Some(conf) = validated.confidence {
            if conf >= self.profile.confidence_threshold {
                response["guidance"] = serde_json::Value::String(
                    "High confidence reached. Consider continuation_mode=\"done\" if answer is sufficient.".into(),
                );
            } else if conf < self.profile.branching_threshold {
                response["guidance"] = serde_json::Value::String(
                    "Low confidence. Consider branching to explore alternatives.".into(),
                );
            }
        }

        Ok(response)
    }
}

/// Generate full guidance text returned on the first thought.
fn first_call_guidance(profile: &TuningProfile) -> String {
    let bt = (profile.branching_threshold * 100.0).round() as u32;
    let ct = (profile.confidence_threshold * 100.0).round() as u32;

    format!(
        r#"
====================================================================
SEQUENTIAL THINKING - FULL GUIDANCE (KinderPowers v1.0)
====================================================================

CORE WORKFLOW:
1. Estimate total_thoughts (adjustable as understanding evolves)
2. Each step: analyze -> filter irrelevant -> focus on what matters
3. Generate hypotheses, verify against reasoning chain
4. Merge insights from explored paths
5. Set next_thought_needed=false ONLY when complete

BRANCHING IS PRIMARY (not "rarely used"):
- Multiple valid approaches -> BRANCH to explore each
- Confidence below {}% -> BRANCH to validate
- Uncertainty about direction -> BRANCH with descriptive branch_id

THIRD ALTERNATIVE (Brenner Pattern):
- When facing A vs B, ALWAYS enumerate a third option: "both could be wrong"
- Ask: What assumption makes A vs B the only choices?
- Ask: What if that assumption is wrong?
- Ask: What would a completely different framing look like?

FOUR SELF-CHECKS (before confidence > 0.6):
1. VERIFY BEFORE ASSUMING: Have I actually tested it, not just claimed "this should work"?
2. DISCOVERY BEFORE CREATION: Have I searched for existing solutions before proposing new?
3. DEEP INSPECTION REQUIRED: Did I see the full picture or just the first 20 lines?
4. EXTEND OVER DUPLICATE: Have I considered extending what exists before designing new?

CONTINUATION MODES:
- "explore": Generate {}-{} alternatives (use explore_count, proposals)
- "branch": Create alternative path from previous thought
- "merge": Combine insights from multiple branches
- "continue": Standard linear progression (default)
- "done": Answer sufficient, stop thinking

CONFIDENCE THRESHOLDS:
- Below {}%: Consider branching
- Above {}%: Consider early exit with done_reason

LAYER ABSTRACTION:
- layer=1: Problem understanding
- layer=2: Approach selection
- layer=3: Implementation details

SEARCH INTEGRATION:
- search_query: What to search before next thought
- search_context: Previous search results
- incorporate_search: Enable search+think interleaving

MODEL: {} | Explore: {}-{} | Budget: {}x
{}

EXAMPLE - Wide Exploration:
  Thought 1: continuation_mode="explore", explore_count=3, proposals=[...], confidence=0.4
  Thought 2: layer=2, continuation_mode="continue", confidence=0.75
  Thought 3: continuation_mode="done", done_reason="sufficient", confidence=0.85
"#,
        bt,
        profile.default_explore_count,
        profile.max_explore_count,
        bt,
        ct,
        profile.display_name,
        profile.default_explore_count,
        profile.max_explore_count,
        profile.token_budget_multiplier,
        profile.guidance,
    )
}

/// Generate tool description text (embedded in tool schema).
pub fn tool_description(profile: &TuningProfile) -> String {
    let bt = (profile.branching_threshold * 100.0).round() as u32;
    let ct = (profile.confidence_threshold * 100.0).round() as u32;

    format!(
        r#"Sequential thinking for multi-step problem-solving with branching and exploration.

REQUIRED: thought, thoughtNumber, totalThoughts
OPTIONAL: confidence (0-1), branchFromThought, branchId, continuationMode, proposals, layer

KEY PATTERNS:
- Estimate totalThoughts, adjust freely as understanding evolves
- Use confidence to track certainty (branch if <{bt}, exit if >{ct})
- Use branchFromThought+branchId to explore alternatives
- continuationMode: explore|branch|merge|continue|done

FIRST RESPONSE includes full exploration guidance. Supporting skills: jw-planning, jw-metathinking

===============================================================================
EXPLORATION GUIDANCE
===============================================================================

CORE WORKFLOW:
1. Estimate total_thoughts (adjustable as understanding evolves)
2. Each step: analyze -> filter irrelevant -> focus on what matters
3. Generate hypotheses, verify against reasoning chain
4. Merge insights from explored paths
5. Set next_thought_needed=false ONLY when complete

BRANCHING IS PRIMARY (not "rarely used"):
- Multiple valid approaches -> BRANCH to explore each
- Confidence below {bt}% -> BRANCH to validate
- Uncertainty about direction -> BRANCH with descriptive branch_id

THIRD ALTERNATIVE (Brenner Pattern):
- When facing A vs B, ALWAYS enumerate "both could be wrong"
- What assumption makes A vs B the only choices? What if it's wrong?

FOUR SELF-CHECKS (before confidence > 0.6):
1. VERIFY BEFORE ASSUMING: tested, not just claimed?
2. DISCOVERY BEFORE CREATION: searched existing before proposing new?
3. DEEP INSPECTION: full picture or partial?
4. EXTEND OVER DUPLICATE: considered extending existing?

CONTINUATION MODES:
- "explore": Generate {de}-{me} alternatives (use explore_count, proposals)
- "branch": Create alternative path from previous thought
- "merge": Combine insights from multiple branches
- "continue": Standard linear progression (default)
- "done": Answer sufficient, stop thinking

CONFIDENCE THRESHOLDS:
- Below {bt}%: Consider branching
- Above {ct}%: Consider early exit with done_reason

LAYER ABSTRACTION:
- layer=1: Problem understanding
- layer=2: Approach selection
- layer=3: Implementation details

SEARCH INTEGRATION:
- search_query: What to search before next thought
- search_context: Previous search results
- incorporate_search: Enable search+think interleaving

MODEL: {dn} | Explore: {de}-{me} | Budget: {tbm}x
{guide}

EXAMPLE - Wide Exploration:
  Thought 1: continuation_mode="explore", explore_count=3, proposals=[...], confidence=0.4
  Thought 2: layer=2, continuation_mode="continue", confidence=0.75
  Thought 3: continuation_mode="done", done_reason="sufficient", confidence=0.85"#,
        bt = bt,
        ct = ct,
        de = profile.default_explore_count,
        me = profile.max_explore_count,
        dn = profile.display_name,
        tbm = profile.token_budget_multiplier,
        guide = profile.guidance,
    )
}

pub(crate) fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();

    for word in text.split_whitespace() {
        if current.len() + word.len() + 1 <= max_width {
            if !current.is_empty() {
                current.push(' ');
            }
            current.push_str(word);
        } else {
            if !current.is_empty() {
                lines.push(current);
            }
            current = word.to_string();
        }
    }
    if !current.is_empty() {
        lines.push(current);
    }

    if lines.is_empty() {
        lines.push(String::new());
    }

    lines
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::profiles::{fallback_profile, default_profiles, get_profile_for_model};

    fn make_engine() -> ThinkingEngine {
        std::env::set_var("DISABLE_THOUGHT_LOGGING", "true");
        let profile = fallback_profile();
        ThinkingEngine::new(profile, "test-model".into(), "test-client".into())
    }

    fn make_thought(num: u32, total: u32) -> ThoughtData {
        ThoughtData {
            thought: format!("Thought number {}", num),
            thought_number: num,
            total_thoughts: total,
            next_thought_needed: true,
            is_revision: None,
            revises_thought: None,
            branch_from_thought: None,
            branch_id: None,
            needs_more_thoughts: None,
            continuation_mode: None,
            explore_count: None,
            proposals: None,
            layer: None,
            delegate_to_next_layer: None,
            branch_strategy: None,
            confidence: None,
            done_reason: None,
            context_window: None,
            search_context: None,
            search_query: None,
            incorporate_search: None,
        }
    }

    // ---- validate tests ----

    #[test]
    fn validate_valid_input() {
        let engine = make_engine();
        let t = make_thought(1, 5);
        let result = engine.validate(t);
        assert!(result.is_ok());
        let v = result.unwrap();
        assert_eq!(v.thought_number, 1);
        assert_eq!(v.total_thoughts, 5);
    }

    #[test]
    fn validate_empty_thought_rejected() {
        let engine = make_engine();
        let mut t = make_thought(1, 5);
        t.thought = String::new();
        let result = engine.validate(t);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("non-empty"));
    }

    #[test]
    fn validate_zero_thought_number_rejected() {
        let engine = make_engine();
        let mut t = make_thought(1, 5);
        t.thought_number = 0;
        let result = engine.validate(t);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("thoughtNumber"));
    }

    #[test]
    fn validate_zero_total_thoughts_rejected() {
        let engine = make_engine();
        let mut t = make_thought(1, 0);
        t.thought_number = 1; // valid
        t.total_thoughts = 0;
        let result = engine.validate(t);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("totalThoughts"));
    }

    #[test]
    fn validate_clamps_confidence() {
        let engine = make_engine();
        let mut t = make_thought(1, 5);
        t.confidence = Some(1.5);
        let v = engine.validate(t).unwrap();
        assert_eq!(v.confidence, Some(1.0));

        let mut t2 = make_thought(1, 5);
        t2.confidence = Some(-0.5);
        let v2 = engine.validate(t2).unwrap();
        assert_eq!(v2.confidence, Some(0.0));
    }

    #[test]
    fn validate_clamps_layer() {
        let engine = make_engine();
        let mut t = make_thought(1, 5);
        t.layer = Some(10);
        let v = engine.validate(t).unwrap();
        assert_eq!(v.layer, Some(5));

        let mut t2 = make_thought(1, 5);
        t2.layer = Some(0);
        let v2 = engine.validate(t2).unwrap();
        assert_eq!(v2.layer, Some(1));
    }

    #[test]
    fn validate_clamps_explore_count() {
        let engine = make_engine();
        let mut t = make_thought(1, 5);
        t.explore_count = Some(100);
        let v = engine.validate(t).unwrap();
        // fallback profile max_explore_count = 5
        assert_eq!(v.explore_count, Some(5));
    }

    #[test]
    fn validate_auto_adjusts_total_thoughts() {
        let engine = make_engine();
        let t = make_thought(10, 5); // thoughtNumber > totalThoughts
        let v = engine.validate(t).unwrap();
        assert_eq!(v.total_thoughts, 10);
    }

    #[test]
    fn validate_continuation_mode_done_sets_next_false() {
        let engine = make_engine();
        let mut t = make_thought(1, 5);
        t.continuation_mode = Some("done".into());
        t.next_thought_needed = true;
        let v = engine.validate(t).unwrap();
        assert!(!v.next_thought_needed);
    }

    #[test]
    fn validate_continuation_mode_continue_sets_next_true() {
        let engine = make_engine();
        let mut t = make_thought(1, 5);
        t.continuation_mode = Some("continue".into());
        t.next_thought_needed = false;
        let v = engine.validate(t).unwrap();
        assert!(v.next_thought_needed);
    }

    // ---- process tests ----

    #[test]
    fn process_returns_correct_structure() {
        let mut engine = make_engine();
        let t = make_thought(1, 5);
        let result = engine.process(t).unwrap();
        assert_eq!(result["thoughtNumber"], 1);
        assert_eq!(result["totalThoughts"], 5);
        assert!(result["nextThoughtNeeded"].is_boolean());
        assert!(result["branches"].is_array());
        assert!(result["compliance"].is_object());
        assert_eq!(result["thoughtHistoryLength"], 1);
    }

    #[test]
    fn process_first_thought_has_guidance() {
        let mut engine = make_engine();
        let t = make_thought(1, 5);
        let result = engine.process(t).unwrap();
        assert!(result.get("firstCallGuidance").is_some());
        let guidance = result["firstCallGuidance"].as_str().unwrap();
        assert!(guidance.contains("SEQUENTIAL THINKING"));
    }

    #[test]
    fn process_second_thought_no_first_call_guidance() {
        let mut engine = make_engine();
        engine.process(make_thought(1, 5)).unwrap();
        let result = engine.process(make_thought(2, 5)).unwrap();
        assert!(result.get("firstCallGuidance").is_none());
    }

    #[test]
    fn process_branch_tracking() {
        let mut engine = make_engine();
        engine.process(make_thought(1, 5)).unwrap();

        let mut branch_thought = make_thought(2, 5);
        branch_thought.branch_from_thought = Some(1);
        branch_thought.branch_id = Some("alternative-a".into());
        let result = engine.process(branch_thought).unwrap();

        let branches = result["branches"].as_array().unwrap();
        assert_eq!(branches.len(), 1);
        assert!(branches.iter().any(|b| b.as_str() == Some("alternative-a")));
    }

    #[test]
    fn process_compliance_consecutive_linear() {
        let mut engine = make_engine();
        for i in 1..=5 {
            engine.process(make_thought(i, 10)).unwrap();
        }
        let result = engine.process(make_thought(6, 10)).unwrap();
        let compliance = &result["compliance"];
        assert_eq!(compliance["consecutiveLinearThoughts"], 6);
        assert_eq!(compliance["needsBranching"], true);
    }

    #[test]
    fn process_compliance_resets_on_branch() {
        let mut engine = make_engine();
        for i in 1..=4 {
            engine.process(make_thought(i, 10)).unwrap();
        }

        let mut branch = make_thought(5, 10);
        branch.branch_from_thought = Some(3);
        branch.branch_id = Some("reset-branch".into());
        let result = engine.process(branch).unwrap();
        assert_eq!(result["compliance"]["consecutiveLinearThoughts"], 0);
        assert_eq!(result["compliance"]["needsBranching"], false);
    }

    #[test]
    fn process_low_confidence_tracking() {
        let mut engine = make_engine();
        // fallback profile branching_threshold = 0.6
        let mut t1 = make_thought(1, 5);
        t1.confidence = Some(0.3);
        engine.process(t1).unwrap();

        let mut t2 = make_thought(2, 5);
        t2.confidence = Some(0.4);
        let result = engine.process(t2).unwrap();
        assert_eq!(result["compliance"]["lowConfWithoutBranchCount"], 2);
    }

    #[test]
    fn process_explore_count_used_flag() {
        let mut engine = make_engine();
        let mut t = make_thought(1, 5);
        t.explore_count = Some(3);
        let result = engine.process(t).unwrap();
        assert_eq!(result["compliance"]["exploreCountUsed"], true);
    }

    #[test]
    fn process_explore_count_not_used() {
        let mut engine = make_engine();
        let t = make_thought(1, 5);
        let result = engine.process(t).unwrap();
        assert_eq!(result["compliance"]["exploreCountUsed"], false);
    }

    #[test]
    fn process_search_query_passthrough() {
        let mut engine = make_engine();
        let mut t = make_thought(1, 5);
        t.search_query = Some("how to branch".into());
        let result = engine.process(t).unwrap();
        assert_eq!(result["pendingSearchQuery"], "how to branch");
        assert!(result.get("hint").is_some());
    }

    #[test]
    fn process_done_mode() {
        let mut engine = make_engine();
        let mut t = make_thought(1, 5);
        t.continuation_mode = Some("done".into());
        let result = engine.process(t).unwrap();
        assert_eq!(result["nextThoughtNeeded"], false);
    }

    #[test]
    fn process_high_confidence_guidance() {
        let mut engine = make_engine();
        let mut t = make_thought(1, 5);
        t.confidence = Some(0.9); // above 0.75 threshold
        let result = engine.process(t).unwrap();
        let guidance = result["guidance"].as_str().unwrap();
        assert!(guidance.contains("done"));
    }

    #[test]
    fn process_low_confidence_guidance() {
        let mut engine = make_engine();
        let mut t = make_thought(2, 5); // not thought 1, to avoid firstCallGuidance noise
        t.confidence = Some(0.3); // below 0.6 threshold
        // Need to process thought 1 first
        engine.process(make_thought(1, 5)).unwrap();
        let result = engine.process(t).unwrap();
        let guidance = result["guidance"].as_str().unwrap();
        assert!(guidance.contains("branching"));
    }

    // ---- wrap_text tests ----

    #[test]
    fn wrap_text_short_line() {
        let lines = wrap_text("hello world", 80);
        assert_eq!(lines, vec!["hello world"]);
    }

    #[test]
    fn wrap_text_wraps_long_line() {
        let lines = wrap_text("one two three four five", 10);
        assert!(lines.len() > 1);
        for line in &lines {
            // Each line should be <= 10 chars (words permitting)
            assert!(line.len() <= 10 || !line.contains(' '));
        }
    }

    #[test]
    fn wrap_text_empty_returns_one_empty() {
        let lines = wrap_text("", 80);
        assert_eq!(lines, vec![""]);
    }

    // ---- format_thought tests ----

    #[test]
    fn format_thought_revision() {
        let engine = make_engine();
        let mut t = make_thought(2, 5);
        t.is_revision = Some(true);
        t.revises_thought = Some(1);
        let output = engine.format_thought(&t);
        assert!(output.contains("Revision"));
    }

    #[test]
    fn format_thought_branch() {
        let engine = make_engine();
        let mut t = make_thought(2, 5);
        t.branch_from_thought = Some(1);
        t.branch_id = Some("test-branch".into());
        let output = engine.format_thought(&t);
        assert!(output.contains("Branch"));
        assert!(output.contains("test-branch"));
    }

    #[test]
    fn format_thought_with_confidence() {
        let engine = make_engine();
        let mut t = make_thought(1, 5);
        t.confidence = Some(0.85);
        let output = engine.format_thought(&t);
        assert!(output.contains("CONFIDENCE"));
        assert!(output.contains("85%"));
    }

    #[test]
    fn format_thought_low_confidence_warning() {
        let engine = make_engine();
        let mut t = make_thought(1, 5);
        t.confidence = Some(0.3);
        let output = engine.format_thought(&t);
        assert!(output.contains("CONSIDER BRANCHING"));
    }

    #[test]
    fn format_thought_no_confidence() {
        let engine = make_engine();
        let t = make_thought(1, 5);
        let output = engine.format_thought(&t);
        assert!(output.contains("NOT PROVIDED"));
    }

    #[test]
    fn format_thought_with_proposals() {
        let engine = make_engine();
        let mut t = make_thought(1, 5);
        t.proposals = Some(vec!["Option A".into(), "Option B".into()]);
        let output = engine.format_thought(&t);
        assert!(output.contains("Option 1"));
        assert!(output.contains("Option 2"));
    }

    #[test]
    fn format_thought_explore_mode() {
        let engine = make_engine();
        let mut t = make_thought(1, 5);
        t.continuation_mode = Some("explore".into());
        t.explore_count = Some(4);
        let output = engine.format_thought(&t);
        assert!(output.contains("Exploring 4 alternatives"));
    }

    #[test]
    fn format_thought_with_layer() {
        let engine = make_engine();
        let mut t = make_thought(1, 5);
        t.layer = Some(2);
        let output = engine.format_thought(&t);
        assert!(output.contains("Layer 2"));
    }

    #[test]
    fn format_thought_with_search() {
        let engine = make_engine();
        let mut t = make_thought(1, 5);
        t.search_query = Some("test query".into());
        let output = engine.format_thought(&t);
        assert!(output.contains("Search: \"test query\""));
    }

    #[test]
    fn format_thought_delegate() {
        let engine = make_engine();
        let mut t = make_thought(1, 5);
        t.delegate_to_next_layer = Some(true);
        let output = engine.format_thought(&t);
        assert!(output.contains("Delegating"));
    }

    // ---- tool_description test ----

    #[test]
    fn tool_description_contains_key_sections() {
        let profile = fallback_profile();
        let desc = tool_description(&profile);
        assert!(desc.contains("REQUIRED"));
        assert!(desc.contains("BRANCHING"));
        assert!(desc.contains("CONFIDENCE"));
        assert!(desc.contains("CONTINUATION MODES"));
    }

    // ---- first_call_guidance test ----

    #[test]
    fn first_call_guidance_contains_key_sections() {
        let profile = fallback_profile();
        let guidance = first_call_guidance(&profile);
        assert!(guidance.contains("CORE WORKFLOW"));
        assert!(guidance.contains("BRANCHING IS PRIMARY"));
        assert!(guidance.contains("THIRD ALTERNATIVE"));
        assert!(guidance.contains("FOUR SELF-CHECKS"));
    }

    // ---- engine with specific profile ----

    #[test]
    fn engine_with_claude_profile() {
        std::env::set_var("DISABLE_THOUGHT_LOGGING", "true");
        let profiles = default_profiles();
        let profile = get_profile_for_model("claude-3-opus", &profiles);
        let mut engine = ThinkingEngine::new(profile.clone(), "claude-3-opus".into(), "test".into());
        assert_eq!(engine.profile().display_name, "Claude");

        let mut t = make_thought(1, 3);
        t.explore_count = Some(10);
        let result = engine.process(t).unwrap();
        // Claude max_explore_count = 5, so it should be clamped
        assert_eq!(result["thoughtNumber"], 1);
    }

    // ---- multiple branches ----

    #[test]
    fn multiple_branches_tracked() {
        let mut engine = make_engine();
        engine.process(make_thought(1, 10)).unwrap();

        let mut b1 = make_thought(2, 10);
        b1.branch_from_thought = Some(1);
        b1.branch_id = Some("branch-a".into());
        engine.process(b1).unwrap();

        let mut b2 = make_thought(3, 10);
        b2.branch_from_thought = Some(1);
        b2.branch_id = Some("branch-b".into());
        let result = engine.process(b2).unwrap();

        let branches = result["branches"].as_array().unwrap();
        assert_eq!(branches.len(), 2);
    }

    // ---- history length increments ----

    #[test]
    fn history_length_increments() {
        let mut engine = make_engine();
        for i in 1..=3 {
            let result = engine.process(make_thought(i, 5)).unwrap();
            assert_eq!(result["thoughtHistoryLength"], i as u64);
        }
    }

    // ---- Tests with logging ENABLED (exercises stderr compliance warning paths) ----

    /// Create an engine with logging enabled (DISABLE_THOUGHT_LOGGING unset/false).
    fn make_engine_with_logging() -> ThinkingEngine {
        std::env::remove_var("DISABLE_THOUGHT_LOGGING");
        let profile = fallback_profile();
        ThinkingEngine::new(profile, "test-model-logging".into(), "test-client".into())
    }

    #[test]
    fn process_with_logging_linear_chain_warning() {
        // Exercise lines 308-309 (eprintln of formatted thought) and 312-313 (linear chain warning)
        let mut engine = make_engine_with_logging();
        for i in 1..=5 {
            let result = engine.process(make_thought(i, 10)).unwrap();
            // After 4+ consecutive linear thoughts, compliance should flag it
            if i >= 4 {
                assert_eq!(result["compliance"]["needsBranching"], true);
            }
        }
        // The 5th thought has consecutive_linear_thoughts=5 >= 4, so the warning path ran
        assert_eq!(engine.consecutive_linear_thoughts, 5);
    }

    #[test]
    fn process_with_logging_explore_count_nudge() {
        // Exercise lines 321-322 (explore_count nudge for thought >= 3 without explore_count usage)
        let mut engine = make_engine_with_logging();
        // Process 3 thoughts without using explore_count
        for i in 1..=3 {
            engine.process(make_thought(i, 5)).unwrap();
        }
        // explore_count_usage_count should still be 0
        assert_eq!(engine.explore_count_usage_count, 0);
    }

    #[test]
    fn process_with_logging_low_confidence_compliance() {
        // Exercise lines 327-328 (low-confidence compliance warning)
        let mut engine = make_engine_with_logging();

        // Submit 2+ low-confidence thoughts without branching
        let mut t1 = make_thought(1, 5);
        t1.confidence = Some(0.3);
        engine.process(t1).unwrap();

        let mut t2 = make_thought(2, 5);
        t2.confidence = Some(0.4);
        engine.process(t2).unwrap();

        assert_eq!(engine.low_conf_without_branch_count, 2);
    }

    #[test]
    fn process_with_logging_all_warnings_at_once() {
        // Trigger all three warning paths in a single engine run
        let mut engine = make_engine_with_logging();

        // 5 linear thoughts with low confidence and no explore_count
        for i in 1..=5 {
            let mut t = make_thought(i, 10);
            t.confidence = Some(0.2); // below 0.6 branching threshold
            engine.process(t).unwrap();
        }

        // All three warning conditions met:
        assert!(engine.consecutive_linear_thoughts >= 4);       // linear chain
        assert_eq!(engine.explore_count_usage_count, 0);        // no explore_count used
        assert!(engine.low_conf_without_branch_count >= 2);     // low-conf without branch
    }
}
