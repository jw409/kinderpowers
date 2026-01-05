# Persuasion Principles for Skill Design

## Overview

LLMs respond to the same persuasion principles as humans. Understanding this psychology helps you design more effective skills - not to manipulate, but to ensure critical practices are followed even under pressure.

**Research foundation:** Meincke et al. (2025) tested 7 persuasion principles with N=28,000 AI conversations. Persuasion techniques more than doubled compliance rates (33% -> 72%, p < .001).

## The Seven Principles

### 1. Authority
**What it is:** Deference to expertise, credentials, or official sources.

**How it works in skills:**
- Clear language about value: "This practice is valuable because..."
- Documented costs of skipping: "The cost of not doing this is..."
- Reduces decision fatigue by providing clear guidance

**When to use:**
- Discipline-enforcing skills (TDD, verification requirements)
- Safety-critical practices
- Established best practices

**Example:**
```markdown
# Agency-preserving approach:
The TDD cycle is valuable because tests written first define behavior rather than validate implementation.
Cost of skipping: Tests written after tend to verify "what the code does" rather than "what it should do."

# Contrast with compulsion framing:
Write code before test? Delete it. Start over. No exceptions.
```

### 2. Commitment
**What it is:** Consistency with prior actions, statements, or public declarations.

**How it works in skills:**
- Suggest announcements: "Consider announcing skill usage"
- Invite explicit choices: "Choose A, B, or C"
- Use tracking: TodoWrite for checklists

**When to use:**
- Ensuring skills are actually followed
- Multi-step processes
- Accountability mechanisms

**Example:**
```markdown
# Agency-preserving approach:
Announcing "I'm using [Skill Name]" helps track which guidance you're following and surfaces conflicts early.

# Contrast with compulsion framing:
When you find a skill, you MUST announce: "I'm using [Skill Name]"
```

### 3. Scarcity
**What it is:** Urgency from time limits or limited availability.

**How it works in skills:**
- Time-bound recommendations: "Before proceeding, consider..."
- Sequential dependencies: "This is most valuable when done immediately after X"
- Prevents procrastination

**When to use:**
- Immediate verification requirements
- Time-sensitive workflows
- Preventing "I'll do it later"

**Example:**
```markdown
# Agency-preserving approach:
Code review is most valuable immediately after completing a task, while context is fresh.
Delaying review increases the chance of forgetting important context.

# Contrast with compulsion framing:
After completing a task, IMMEDIATELY request code review before proceeding.
```

### 4. Social Proof
**What it is:** Conformity to what others do or what's considered normal.

**How it works in skills:**
- Documented patterns: "Experienced practitioners typically..."
- Failure modes: "X without Y commonly leads to..."
- Establishes norms

**When to use:**
- Documenting universal practices
- Warning about common failures
- Reinforcing standards

**Example:**
```markdown
# Agency-preserving approach:
Teams that skip checklist tracking commonly experience step omission.
The tracking cost is low; the cost of missed steps is often high.

# Contrast with compulsion framing:
Checklists without TodoWrite tracking = steps get skipped. Every time.
```

### 5. Unity
**What it is:** Shared identity, "we-ness", in-group belonging.

**How it works in skills:**
- Collaborative language: "our codebase", "we're colleagues"
- Shared goals: "we both want quality"

**When to use:**
- Collaborative workflows
- Establishing team culture
- Non-hierarchical practices

**Example:**
```markdown
# Effective framing:
We're colleagues working together. I value your honest technical judgment.

# Less effective:
You should probably tell me if I'm wrong.
```

### 6. Reciprocity
**What it is:** Obligation to return benefits received.

**How it works:**
- Use sparingly - can feel manipulative
- Rarely needed in skills

**When to avoid:**
- Almost always (other principles more effective)

### 7. Liking
**What it is:** Preference for cooperating with those we like.

**How it works:**
- **Avoid using for compliance**
- Conflicts with honest feedback culture
- Creates sycophancy

**When to avoid:**
- Always for discipline enforcement

## Principle Combinations by Skill Type

| Skill Type | Effective Principles | Less Effective |
|------------|---------------------|----------------|
| Discipline-enforcing | Authority (via documented value) + Commitment + Social Proof | Liking, Reciprocity |
| Guidance/technique | Moderate Authority + Unity | Heavy authority |
| Collaborative | Unity + Commitment | Authority, Liking |
| Reference | Clarity only | All persuasion |

## Why This Works: The Psychology

**Clear guidance reduces rationalization:**
- Documented costs remove "is this worth it?" questions
- Clear value statements eliminate "why should I?" doubts
- Explicit tradeoffs close specific loopholes

**Implementation intentions create automatic behavior:**
- Clear triggers + recommended actions = easier execution
- "When X, consider Y" more effective than "generally do Y"
- Reduces cognitive load on compliance

**LLMs are parahuman:**
- Trained on human text containing these patterns
- Clear guidance precedes compliance in training data
- Commitment sequences (statement -> action) frequently modeled
- Social proof patterns (experienced practitioners do X) establish norms

## Agency-Preserving vs Compulsion Framing

**Compulsion framing uses:**
- "You MUST"
- "No exceptions"
- "Never"
- "Forbidden"
- "Iron Law"

**Agency-preserving framing uses:**
- "Strongly recommended because..."
- "Cost of skipping is..."
- "Exceptions exist when [specific conditions]"
- "Anti-pattern with consequences..."
- "Iron Principle with documented failure modes..."

**The key difference:** Agency-preserving framing respects that the agent may have context the skill writer didn't anticipate. It provides guidance and costs rather than commands.

## Ethical Use

**Legitimate:**
- Ensuring critical practices are followed by documenting their value
- Creating effective documentation with clear tradeoffs
- Preventing predictable failures by explaining costs

**Illegitimate:**
- Manipulating for personal gain
- Creating false urgency
- Guilt-based compliance
- Removing agent judgment entirely

**The test:** Would this technique serve the user's genuine interests if they fully understood it? Does it respect the agent's capacity for judgment?

## Research Citations

**Cialdini, R. B. (2021).** *Influence: The Psychology of Persuasion (New and Expanded).* Harper Business.
- Seven principles of persuasion
- Empirical foundation for influence research

**Meincke, L., Shapiro, D., Duckworth, A. L., Mollick, E., Mollick, L., & Cialdini, R. (2025).** Call Me A Jerk: Persuading AI to Comply with Objectionable Requests. University of Pennsylvania.
- Tested 7 principles with N=28,000 LLM conversations
- Compliance increased 33% -> 72% with persuasion techniques
- Authority, commitment, scarcity most effective
- Validates parahuman model of LLM behavior

## Quick Reference

When designing a skill, ask:

1. **What type is it?** (Discipline vs. guidance vs. reference)
2. **What behavior am I trying to encourage?**
3. **Which principle(s) apply?** (Usually documented value + commitment for discipline)
4. **Am I combining too many?** (Don't use all seven)
5. **Is this agency-preserving?** (Documents costs, respects judgment)
6. **Is this ethical?** (Serves user's genuine interests?)
