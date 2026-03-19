use rmcp::handler::server::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{
    CallToolResult, Content, ListResourceTemplatesResult, ReadResourceRequestParams,
    ReadResourceResult, ResourceContents, ServerInfo,
};
use rmcp::{ErrorData as McpError, ServerHandler, ServiceExt};
use schemars::JsonSchema;
use serde::Deserialize;
use std::sync::Mutex;

use crate::profiles;
use crate::thinking::{ThinkingEngine, ThoughtData};

// ============================================================================
// MCP Parameter struct — maps to the tool's JSON Schema
// ============================================================================

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SequentialThinkingParams {
    /// Your current thinking step
    pub thought: String,

    /// Current thought number
    #[schemars(range(min = 1))]
    pub thought_number: u32,

    /// Estimated total thoughts needed
    #[schemars(range(min = 1))]
    pub total_thoughts: u32,

    /// Whether another thought step is needed (backwards compat, prefer continuation_mode)
    pub next_thought_needed: Option<bool>,

    /// Whether this revises previous thinking
    pub is_revision: Option<bool>,

    /// Which thought is being reconsidered
    #[schemars(range(min = 1))]
    pub revises_thought: Option<u32>,

    /// PRIMARY: Branching point thought number - USE LIBERALLY
    #[schemars(range(min = 1))]
    pub branch_from_thought: Option<u32>,

    /// PRIMARY: Branch identifier - descriptive name for this exploration path
    pub branch_id: Option<String>,

    /// If more thoughts are needed
    pub needs_more_thoughts: Option<bool>,

    /// How to continue: explore (generate alternatives), done (early exit), delegate (pass to next layer), branch (alternative path), merge (combine branches), continue (default linear)
    pub continuation_mode: Option<String>,

    /// Number of parallel alternatives to generate (default: 4, max: 5)
    #[schemars(range(min = 1, max = 5))]
    pub explore_count: Option<u32>,

    /// Lightweight descriptions of alternatives being considered
    pub proposals: Option<Vec<String>>,

    /// Abstraction layer: 1=problem, 2=approach, 3=details
    #[schemars(range(min = 1, max = 5))]
    pub layer: Option<u32>,

    /// Pass proposals to smarter agent/layer for selection
    pub delegate_to_next_layer: Option<bool>,

    /// How to handle branches: parallel (explore all), sequential (one at a time), converge (merge results)
    pub branch_strategy: Option<String>,

    /// Merge insights from these branch IDs into this thought (continuation_mode should be "merge")
    pub merge_branches: Option<Vec<String>>,

    /// Confidence in current answer (0.0-1.0). Below 0.6: consider branching. Above 0.75: consider early exit.
    #[schemars(range(min = 0.0, max = 1.0))]
    pub confidence: Option<f64>,

    /// Why stopping: complete (fully solved), sufficient (good enough), blocked (can't proceed), delegate (passing up)
    pub done_reason: Option<String>,

    /// Token efficiency: compact (last 2 thoughts), normal (last 5), expanded (all)
    pub context_window: Option<String>,

    /// What to search for before next thought (orchestrator will execute)
    pub search_query: Option<String>,

    /// Previous search results (provided by orchestrator)
    pub search_context: Option<String>,

    /// Should orchestrator search before next thought?
    pub incorporate_search: Option<bool>,
}

impl From<SequentialThinkingParams> for ThoughtData {
    fn from(p: SequentialThinkingParams) -> Self {
        ThoughtData {
            thought: p.thought,
            thought_number: p.thought_number,
            total_thoughts: p.total_thoughts,
            next_thought_needed: p.next_thought_needed.unwrap_or(true),
            is_revision: p.is_revision,
            revises_thought: p.revises_thought,
            branch_from_thought: p.branch_from_thought,
            branch_id: p.branch_id,
            needs_more_thoughts: p.needs_more_thoughts,
            continuation_mode: p.continuation_mode,
            explore_count: p.explore_count,
            proposals: p.proposals,
            layer: p.layer,
            delegate_to_next_layer: p.delegate_to_next_layer,
            branch_strategy: p.branch_strategy,
            merge_branches: p.merge_branches,
            confidence: p.confidence,
            done_reason: p.done_reason,
            context_window: p.context_window,
            search_query: p.search_query,
            search_context: p.search_context,
            incorporate_search: p.incorporate_search,
        }
    }
}

// ============================================================================
// MCP Server
// ============================================================================

pub struct SeqThinkServer {
    engine: Mutex<ThinkingEngine>,
    tool_router: ToolRouter<Self>,
}

#[rmcp::tool_router]
impl SeqThinkServer {
    /// Sequential thinking for multi-step problem-solving with branching and exploration.
    #[rmcp::tool(name = "sequentialthinking")]
    fn sequentialthinking(
        &self,
        Parameters(params): Parameters<SequentialThinkingParams>,
    ) -> Result<CallToolResult, McpError> {
        let data: ThoughtData = params.into();
        let mut engine = self.engine.lock().map_err(|e| {
            McpError::internal_error(format!("engine lock poisoned: {}", e), None)
        })?;

        match engine.process(data) {
            Ok(response) => {
                let text = serde_json::to_string_pretty(&response).unwrap_or_default();
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            Err(msg) => {
                let err_json = serde_json::json!({
                    "error": msg,
                    "status": "failed"
                });
                let text = serde_json::to_string_pretty(&err_json).unwrap_or_default();
                Ok(CallToolResult::error(vec![Content::text(text)]))
            }
        }
    }
}

#[rmcp::tool_handler]
impl ServerHandler for SeqThinkServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            server_info: rmcp::model::Implementation {
                name: "kp-sequential-thinking".into(),
                title: Some("Enhanced Sequential Thinking MCP Server".into()),
                version: env!("CARGO_PKG_VERSION").into(),
                description: None,
                icons: None,
                website_url: None,
            },
            capabilities: rmcp::model::ServerCapabilities {
                tools: Some(rmcp::model::ToolsCapability::default()),
                resources: Some(rmcp::model::ResourcesCapability::default()),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn list_resource_templates(
        &self,
        _request: Option<rmcp::model::PaginatedRequestParams>,
        _context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> impl std::future::Future<Output = Result<ListResourceTemplatesResult, McpError>> + Send + '_
    {
        use rmcp::model::{Annotated, RawResourceTemplate};

        let templates = vec![
            Annotated::new(
                RawResourceTemplate {
                    uri_template: "seqthink://sessions/current/thoughts".into(),
                    name: "current_thoughts".into(),
                    title: Some("Current Session Thoughts".into()),
                    description: Some(
                        "Returns the current session's thought history as compressed JSON"
                            .into(),
                    ),
                    mime_type: Some("application/json".into()),
                    icons: None,
                },
                None,
            ),
            Annotated::new(
                RawResourceTemplate {
                    uri_template: "seqthink://sessions/current/branches".into(),
                    name: "current_branches".into(),
                    title: Some("Current Session Branches".into()),
                    description: Some(
                        "Returns branch names and thought counts for the current session".into(),
                    ),
                    mime_type: Some("application/json".into()),
                    icons: None,
                },
                None,
            ),
            Annotated::new(
                RawResourceTemplate {
                    uri_template: "seqthink://sessions/current/compliance".into(),
                    name: "current_compliance".into(),
                    title: Some("Current Session Compliance".into()),
                    description: Some(
                        "Returns compliance stats for the current session".into(),
                    ),
                    mime_type: Some("application/json".into()),
                    icons: None,
                },
                None,
            ),
        ];

        std::future::ready(Ok(ListResourceTemplatesResult::with_all_items(templates)))
    }

    fn read_resource(
        &self,
        request: ReadResourceRequestParams,
        _context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> impl std::future::Future<Output = Result<ReadResourceResult, McpError>> + Send + '_ {
        async move {
            let uri = &request.uri;

            let path = uri
                .strip_prefix("seqthink://sessions/current/")
                .ok_or_else(|| {
                    McpError::invalid_params(format!("Unknown resource URI: {uri}"), None)
                })?;

            let engine = self.engine.lock().map_err(|e| {
                McpError::internal_error(format!("engine lock poisoned: {e}"), None)
            })?;

            let json_text = match path {
                "thoughts" => {
                    let history = engine.thought_history();
                    let compact: Vec<serde_json::Value> = history
                        .iter()
                        .map(|t| {
                            serde_json::json!({
                                "n": t.thought_number,
                                "total": t.total_thoughts,
                                "thought": t.thought,
                                "confidence": t.confidence,
                                "branch": t.branch_id,
                                "layer": t.layer,
                                "mode": t.continuation_mode,
                            })
                        })
                        .collect();
                    serde_json::to_string(&compact).unwrap_or_default()
                }
                "branches" => {
                    let branches = engine.branches();
                    let summary: serde_json::Value = branches
                        .iter()
                        .map(|(name, thoughts)| {
                            (name.clone(), serde_json::json!(thoughts.len()))
                        })
                        .collect::<serde_json::Map<String, serde_json::Value>>()
                        .into();
                    serde_json::to_string(&summary).unwrap_or_default()
                }
                "compliance" => {
                    let stats = engine.compliance_stats();
                    serde_json::to_string(&stats).unwrap_or_default()
                }
                other => {
                    return Err(McpError::invalid_params(
                        format!("Unknown resource path: {other}"),
                        None,
                    ));
                }
            };

            Ok(ReadResourceResult {
                contents: vec![ResourceContents::text(json_text, &request.uri)],
            })
        }
    }
}

pub async fn run() -> anyhow::Result<()> {
    let model_id =
        std::env::var("SEQUENTIAL_THINKING_MODEL").unwrap_or_else(|_| "unknown".into());
    let client_type = detect_client_type();

    let all_profiles = profiles::load_profiles();
    let profile = profiles::get_profile_for_model(&model_id, &all_profiles);

    tracing::info!(
        model = %model_id,
        profile = %profile.display_name,
        client = %client_type,
        "sequential thinking server ready"
    );

    let server = SeqThinkServer {
        engine: Mutex::new(ThinkingEngine::new(
            profile,
            model_id,
            client_type,
        )),
        tool_router: SeqThinkServer::tool_router(),
    };

    let service = server.serve(rmcp::transport::io::stdio()).await?;
    service.waiting().await?;
    Ok(())
}

fn detect_client_type() -> String {
    if std::env::var("CLAUDE_CODE_VERSION").is_ok() || std::env::var("CLAUDE_AGENT_SDK").is_ok() {
        "claude-code".into()
    } else if std::env::var("GEMINI_CLI").is_ok() || std::env::var("GOOGLE_CLI").is_ok() {
        "gemini-cli".into()
    } else if std::env::var("TALENTOS_AGENT").is_ok() {
        "talentos".into()
    } else {
        "unknown".into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_params(thought: &str, num: u32, total: u32) -> SequentialThinkingParams {
        SequentialThinkingParams {
            thought: thought.into(),
            thought_number: num,
            total_thoughts: total,
            next_thought_needed: None,
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
            merge_branches: None,
            confidence: None,
            done_reason: None,
            context_window: None,
            search_query: None,
            search_context: None,
            incorporate_search: None,
        }
    }

    fn make_server() -> SeqThinkServer {
        std::env::set_var("DISABLE_THOUGHT_LOGGING", "true");
        let profile = crate::profiles::fallback_profile();
        SeqThinkServer {
            engine: Mutex::new(ThinkingEngine::new(
                profile,
                "test-model".into(),
                "test-client".into(),
            )),
            tool_router: SeqThinkServer::tool_router(),
        }
    }

    // ---- From<SequentialThinkingParams> for ThoughtData ----

    #[test]
    fn params_to_thought_data_required_fields() {
        let params = make_params("hello", 1, 5);
        let data: ThoughtData = params.into();
        assert_eq!(data.thought, "hello");
        assert_eq!(data.thought_number, 1);
        assert_eq!(data.total_thoughts, 5);
        // next_thought_needed defaults to true when None
        assert!(data.next_thought_needed);
    }

    #[test]
    fn params_to_thought_data_all_optional_fields() {
        let params = SequentialThinkingParams {
            thought: "test".into(),
            thought_number: 2,
            total_thoughts: 10,
            next_thought_needed: Some(false),
            is_revision: Some(true),
            revises_thought: Some(1),
            branch_from_thought: Some(1),
            branch_id: Some("branch-x".into()),
            needs_more_thoughts: Some(true),
            continuation_mode: Some("explore".into()),
            explore_count: Some(3),
            proposals: Some(vec!["A".into(), "B".into()]),
            layer: Some(2),
            delegate_to_next_layer: Some(true),
            branch_strategy: Some("parallel".into()),
            merge_branches: Some(vec!["branch-a".into(), "branch-b".into()]),
            confidence: Some(0.8),
            done_reason: Some("complete".into()),
            context_window: Some("expanded".into()),
            search_query: Some("query".into()),
            search_context: Some("context".into()),
            incorporate_search: Some(true),
        };
        let data: ThoughtData = params.into();
        assert!(!data.next_thought_needed);
        assert_eq!(data.is_revision, Some(true));
        assert_eq!(data.revises_thought, Some(1));
        assert_eq!(data.branch_from_thought, Some(1));
        assert_eq!(data.branch_id.as_deref(), Some("branch-x"));
        assert_eq!(data.needs_more_thoughts, Some(true));
        assert_eq!(data.continuation_mode.as_deref(), Some("explore"));
        assert_eq!(data.explore_count, Some(3));
        assert_eq!(data.proposals.as_ref().unwrap().len(), 2);
        assert_eq!(data.layer, Some(2));
        assert_eq!(data.delegate_to_next_layer, Some(true));
        assert_eq!(data.branch_strategy.as_deref(), Some("parallel"));
        assert_eq!(data.merge_branches.as_ref().unwrap().len(), 2);
        assert_eq!(data.confidence, Some(0.8));
        assert_eq!(data.done_reason.as_deref(), Some("complete"));
        assert_eq!(data.context_window.as_deref(), Some("expanded"));
        assert_eq!(data.search_query.as_deref(), Some("query"));
        assert_eq!(data.search_context.as_deref(), Some("context"));
        assert_eq!(data.incorporate_search, Some(true));
    }

    // ---- sequentialthinking tool method ----

    /// Extract the text string from the first content item of a CallToolResult.
    fn extract_text(result: &CallToolResult) -> String {
        result.content[0].raw.as_text().expect("expected text content").text.clone()
    }

    #[test]
    fn tool_method_valid_thought_returns_success() {
        let server = make_server();
        let params = make_params("Analyzing the problem", 1, 5);
        let result = server.sequentialthinking(Parameters(params));
        assert!(result.is_ok());
        let call_result = result.unwrap();
        assert!(!call_result.is_error.unwrap_or(false));
        let text = extract_text(&call_result);
        let parsed: serde_json::Value = serde_json::from_str(&text).unwrap();
        assert_eq!(parsed["thoughtNumber"], 1);
        assert_eq!(parsed["totalThoughts"], 5);
        assert!(parsed.get("firstCallGuidance").is_some());
    }

    #[test]
    fn tool_method_invalid_thought_returns_error() {
        let server = make_server();
        let params = make_params("", 1, 5); // empty thought = invalid
        let result = server.sequentialthinking(Parameters(params));
        assert!(result.is_ok()); // MCP errors are returned as OK with error content
        let call_result = result.unwrap();
        assert!(call_result.is_error.unwrap_or(false));
        let text = extract_text(&call_result);
        let parsed: serde_json::Value = serde_json::from_str(&text).unwrap();
        assert!(parsed.get("error").is_some());
        assert_eq!(parsed["status"], "failed");
    }

    #[test]
    fn tool_method_sequential_thoughts_accumulate_state() {
        let server = make_server();
        // First thought
        let p1 = make_params("Step 1", 1, 5);
        let r1 = server.sequentialthinking(Parameters(p1)).unwrap();
        assert!(!r1.is_error.unwrap_or(false));

        // Second thought
        let p2 = make_params("Step 2", 2, 5);
        let r2 = server.sequentialthinking(Parameters(p2)).unwrap();
        let text2 = extract_text(&r2);
        let parsed2: serde_json::Value = serde_json::from_str(&text2).unwrap();
        assert_eq!(parsed2["thoughtHistoryLength"], 2);
        assert!(parsed2.get("firstCallGuidance").is_none());

        // Third thought with branch
        let mut p3 = make_params("Branch from step 1", 3, 5);
        p3.branch_from_thought = Some(1);
        p3.branch_id = Some("alt-path".into());
        let r3 = server.sequentialthinking(Parameters(p3)).unwrap();
        let text3 = extract_text(&r3);
        let parsed3: serde_json::Value = serde_json::from_str(&text3).unwrap();
        assert_eq!(parsed3["thoughtHistoryLength"], 3);
        let branches = parsed3["branches"].as_array().unwrap();
        assert!(branches.iter().any(|b| b.as_str() == Some("alt-path")));
    }

    // ---- get_info ----

    #[test]
    fn server_info_has_correct_name() {
        let server = make_server();
        let info = server.get_info();
        assert_eq!(info.server_info.name, "kp-sequential-thinking");
        assert!(info.server_info.title.as_deref().unwrap().contains("Sequential Thinking"));
    }

    // ---- detect_client_type ----
    // NOTE: env var tests must be serialized to avoid races. We test all
    // branches in a single test to avoid parallel env var conflicts.

    #[test]
    fn tool_method_poisoned_lock_returns_error() {
        let server = make_server();
        // Poison the mutex by panicking inside a lock
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _guard = server.engine.lock().unwrap();
            panic!("intentional panic to poison the mutex");
        }));
        // Now the mutex is poisoned
        let params = make_params("This should fail", 1, 5);
        let result = server.sequentialthinking(Parameters(params));
        assert!(result.is_err()); // McpError from poisoned lock
    }

    // ---- resource tests ----

    #[test]
    fn server_info_has_resources_capability() {
        let server = make_server();
        let info = server.get_info();
        assert!(info.capabilities.resources.is_some());
    }

    // ---- engine getter tests (used by resource handlers) ----

    #[test]
    fn engine_thought_history_empty() {
        let engine = crate::thinking::ThinkingEngine::new(
            crate::profiles::fallback_profile(),
            "test".into(),
            "test".into(),
        );
        assert!(engine.thought_history().is_empty());
    }

    #[test]
    fn engine_thought_history_after_processing() {
        std::env::set_var("DISABLE_THOUGHT_LOGGING", "true");
        let mut engine = crate::thinking::ThinkingEngine::new(
            crate::profiles::fallback_profile(),
            "test".into(),
            "test".into(),
        );
        let data: ThoughtData = make_params("First thought", 1, 3).into();
        engine.process(data).unwrap();
        let data2: ThoughtData = make_params("Second thought", 2, 3).into();
        engine.process(data2).unwrap();
        assert_eq!(engine.thought_history().len(), 2);
        assert_eq!(engine.thought_history()[0].thought_number, 1);
        assert_eq!(engine.thought_history()[1].thought_number, 2);
    }

    #[test]
    fn engine_branches_after_branching() {
        std::env::set_var("DISABLE_THOUGHT_LOGGING", "true");
        let mut engine = crate::thinking::ThinkingEngine::new(
            crate::profiles::fallback_profile(),
            "test".into(),
            "test".into(),
        );
        let data: ThoughtData = make_params("Start", 1, 5).into();
        engine.process(data).unwrap();

        let mut p2 = make_params("Branch A", 2, 5);
        p2.branch_from_thought = Some(1);
        p2.branch_id = Some("branch-a".into());
        let data2: ThoughtData = p2.into();
        engine.process(data2).unwrap();

        let branches = engine.branches();
        assert_eq!(branches.len(), 1);
        assert_eq!(branches["branch-a"].len(), 1);
    }

    #[test]
    fn engine_compliance_stats() {
        std::env::set_var("DISABLE_THOUGHT_LOGGING", "true");
        let mut engine = crate::thinking::ThinkingEngine::new(
            crate::profiles::fallback_profile(),
            "test".into(),
            "test".into(),
        );
        let data: ThoughtData = make_params("Think", 1, 5).into();
        engine.process(data).unwrap();
        let stats = engine.compliance_stats();
        assert_eq!(stats.consecutive_linear_thoughts, 1);
        assert!(!stats.needs_branching);
        assert!(!stats.explore_count_used);
    }

    #[test]
    fn detect_client_type_all_branches() {
        // Clear all detection env vars first
        std::env::remove_var("CLAUDE_CODE_VERSION");
        std::env::remove_var("CLAUDE_AGENT_SDK");
        std::env::remove_var("GEMINI_CLI");
        std::env::remove_var("GOOGLE_CLI");
        std::env::remove_var("TALENTOS_AGENT");

        // Unknown
        assert_eq!(detect_client_type(), "unknown");

        // Talentos (test last-priority first so higher-priority vars don't interfere)
        std::env::set_var("TALENTOS_AGENT", "1");
        assert_eq!(detect_client_type(), "talentos");
        std::env::remove_var("TALENTOS_AGENT");

        // Gemini CLI via GEMINI_CLI
        std::env::set_var("GEMINI_CLI", "1");
        assert_eq!(detect_client_type(), "gemini-cli");
        std::env::remove_var("GEMINI_CLI");

        // Gemini CLI via GOOGLE_CLI
        std::env::set_var("GOOGLE_CLI", "1");
        assert_eq!(detect_client_type(), "gemini-cli");
        std::env::remove_var("GOOGLE_CLI");

        // Claude Code via CLAUDE_CODE_VERSION
        std::env::set_var("CLAUDE_CODE_VERSION", "1.0");
        assert_eq!(detect_client_type(), "claude-code");
        std::env::remove_var("CLAUDE_CODE_VERSION");

        // Claude Code via CLAUDE_AGENT_SDK
        std::env::set_var("CLAUDE_AGENT_SDK", "1");
        assert_eq!(detect_client_type(), "claude-code");
        std::env::remove_var("CLAUDE_AGENT_SDK");
    }
}
