//! Integration tests for kp-sequential-thinking.
//! No external API calls needed - tests the MCP server via JSON-RPC over stdin/stdout.

use serde_json::{json, Value};
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::Command;

struct McpClient {
    child: tokio::process::Child,
    stdin: tokio::process::ChildStdin,
    reader: BufReader<tokio::process::ChildStdout>,
    next_id: u32,
}

impl McpClient {
    async fn new() -> Self {
        let mut child = Command::new(env!("CARGO_BIN_EXE_kp-sequential-thinking"))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .env("DISABLE_THOUGHT_LOGGING", "true")
            .spawn()
            .expect("Failed to start server");

        let stdin = child.stdin.take().unwrap();
        let stdout = child.stdout.take().unwrap();
        let reader = BufReader::new(stdout);

        let mut client = Self {
            child,
            stdin,
            reader,
            next_id: 1,
        };

        // Initialize
        let _ = client
            .call(
                "initialize",
                json!({
                    "protocolVersion": "2024-11-05",
                    "capabilities": {},
                    "clientInfo": {"name": "test", "version": "0.1"}
                }),
            )
            .await;

        // Send initialized notification
        client.send_notification("notifications/initialized").await;

        client
    }

    async fn send_notification(&mut self, method: &str) {
        let msg = json!({"jsonrpc": "2.0", "method": method});
        let line = format!("{}\n", serde_json::to_string(&msg).unwrap());
        self.stdin.write_all(line.as_bytes()).await.unwrap();
        self.stdin.flush().await.unwrap();
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }

    async fn call(&mut self, method: &str, params: Value) -> Value {
        let id = self.next_id;
        self.next_id += 1;
        let msg = json!({"jsonrpc": "2.0", "id": id, "method": method, "params": params});
        let line = format!("{}\n", serde_json::to_string(&msg).unwrap());
        self.stdin.write_all(line.as_bytes()).await.unwrap();
        self.stdin.flush().await.unwrap();

        let mut buf = String::new();
        self.reader.read_line(&mut buf).await.unwrap();
        serde_json::from_str(&buf).unwrap()
    }

    async fn tool_call(&mut self, name: &str, args: Value) -> Value {
        self.call("tools/call", json!({"name": name, "arguments": args}))
            .await
    }

    fn get_text(resp: &Value) -> &str {
        resp["result"]["content"][0]["text"].as_str().unwrap_or("")
    }

    fn get_parsed(resp: &Value) -> Value {
        let text = Self::get_text(resp);
        serde_json::from_str(text).unwrap_or(json!({"_parse_error": text}))
    }

    fn is_error(resp: &Value) -> bool {
        resp.get("error").is_some()
    }

    fn is_tool_error(resp: &Value) -> bool {
        resp["result"]["isError"].as_bool().unwrap_or(false)
    }
}

impl Drop for McpClient {
    fn drop(&mut self) {
        let _ = self.child.start_kill();
    }
}

#[tokio::test]
async fn test_initialize() {
    let mut client = McpClient::new().await;
    let resp = client.call("tools/list", json!({})).await;
    // Server should respond without error
    assert!(!McpClient::is_error(&resp));
}

#[tokio::test]
async fn test_tools_list() {
    let mut client = McpClient::new().await;
    let resp = client.call("tools/list", json!({})).await;
    let tools = resp["result"]["tools"].as_array().unwrap();
    assert_eq!(tools.len(), 1, "Should have exactly 1 tool");

    let tool = &tools[0];
    assert_eq!(tool["name"].as_str().unwrap(), "sequentialthinking");
    // Should have inputSchema
    assert!(tool["inputSchema"].is_object(), "Should have inputSchema");
}

#[tokio::test]
async fn test_basic_thought() {
    let mut client = McpClient::new().await;
    let resp = client
        .tool_call(
            "sequentialthinking",
            json!({
                "thought": "Analyzing the problem structure",
                "thoughtNumber": 1,
                "totalThoughts": 3
            }),
        )
        .await;
    assert!(!McpClient::is_error(&resp));
    assert!(!McpClient::is_tool_error(&resp));

    let parsed = McpClient::get_parsed(&resp);
    assert_eq!(parsed["thoughtNumber"], 1);
    assert_eq!(parsed["totalThoughts"], 3);
    assert_eq!(parsed["nextThoughtNeeded"], true);
    assert_eq!(parsed["thoughtHistoryLength"], 1);
}

#[tokio::test]
async fn test_thought_chain() {
    let mut client = McpClient::new().await;

    // Thought 1
    let r1 = client
        .tool_call(
            "sequentialthinking",
            json!({
                "thought": "First: understand the problem",
                "thoughtNumber": 1,
                "totalThoughts": 3
            }),
        )
        .await;
    let p1 = McpClient::get_parsed(&r1);
    assert_eq!(p1["thoughtHistoryLength"], 1);

    // Thought 2
    let r2 = client
        .tool_call(
            "sequentialthinking",
            json!({
                "thought": "Second: design the solution",
                "thoughtNumber": 2,
                "totalThoughts": 3
            }),
        )
        .await;
    let p2 = McpClient::get_parsed(&r2);
    assert_eq!(p2["thoughtHistoryLength"], 2);

    // Thought 3
    let r3 = client
        .tool_call(
            "sequentialthinking",
            json!({
                "thought": "Third: implement",
                "thoughtNumber": 3,
                "totalThoughts": 3,
                "nextThoughtNeeded": false
            }),
        )
        .await;
    let p3 = McpClient::get_parsed(&r3);
    assert_eq!(p3["thoughtHistoryLength"], 3);
    assert_eq!(p3["nextThoughtNeeded"], false);
}

#[tokio::test]
async fn test_branching() {
    let mut client = McpClient::new().await;

    // Linear thought first
    let _ = client
        .tool_call(
            "sequentialthinking",
            json!({
                "thought": "Base analysis",
                "thoughtNumber": 1,
                "totalThoughts": 4
            }),
        )
        .await;

    // Branch from thought 1
    let r2 = client
        .tool_call(
            "sequentialthinking",
            json!({
                "thought": "Alternative approach A",
                "thoughtNumber": 2,
                "totalThoughts": 4,
                "branchFromThought": 1,
                "branchId": "approach-a"
            }),
        )
        .await;
    let p2 = McpClient::get_parsed(&r2);
    let branches = p2["branches"].as_array().unwrap();
    assert!(
        branches.iter().any(|b| b.as_str() == Some("approach-a")),
        "Should track branch 'approach-a', got: {:?}",
        branches
    );

    // Another branch
    let r3 = client
        .tool_call(
            "sequentialthinking",
            json!({
                "thought": "Alternative approach B",
                "thoughtNumber": 3,
                "totalThoughts": 4,
                "branchFromThought": 1,
                "branchId": "approach-b"
            }),
        )
        .await;
    let p3 = McpClient::get_parsed(&r3);
    let branches = p3["branches"].as_array().unwrap();
    assert_eq!(branches.len(), 2, "Should have 2 branches");
}

#[tokio::test]
async fn test_confidence_low_guidance() {
    let mut client = McpClient::new().await;
    let resp = client
        .tool_call(
            "sequentialthinking",
            json!({
                "thought": "I'm uncertain about the approach",
                "thoughtNumber": 1,
                "totalThoughts": 3,
                "confidence": 0.3
            }),
        )
        .await;
    let parsed = McpClient::get_parsed(&resp);
    let guidance = parsed["guidance"].as_str().unwrap_or("");
    assert!(
        guidance.to_lowercase().contains("branch"),
        "Low confidence should suggest branching, got: {}",
        guidance
    );
}

#[tokio::test]
async fn test_high_confidence_exit() {
    let mut client = McpClient::new().await;
    let resp = client
        .tool_call(
            "sequentialthinking",
            json!({
                "thought": "The answer is clear now",
                "thoughtNumber": 2,
                "totalThoughts": 5,
                "confidence": 0.85
            }),
        )
        .await;
    let parsed = McpClient::get_parsed(&resp);
    let guidance = parsed["guidance"].as_str().unwrap_or("");
    assert!(
        guidance.to_lowercase().contains("done")
            || guidance.to_lowercase().contains("sufficient")
            || guidance.to_lowercase().contains("exit")
            || guidance.to_lowercase().contains("confidence"),
        "High confidence should suggest early exit, got: {}",
        guidance
    );
}

#[tokio::test]
async fn test_first_call_guidance() {
    let mut client = McpClient::new().await;
    let resp = client
        .tool_call(
            "sequentialthinking",
            json!({
                "thought": "Starting analysis",
                "thoughtNumber": 1,
                "totalThoughts": 5
            }),
        )
        .await;
    let parsed = McpClient::get_parsed(&resp);
    assert!(
        parsed.get("firstCallGuidance").is_some(),
        "First thought should include firstCallGuidance"
    );
    let guidance = parsed["firstCallGuidance"].as_str().unwrap();
    assert!(
        guidance.contains("DECIDE(confidence)"),
        "First call guidance should contain decision tree"
    );

    // Second thought should NOT have firstCallGuidance
    let resp2 = client
        .tool_call(
            "sequentialthinking",
            json!({
                "thought": "Continuing",
                "thoughtNumber": 2,
                "totalThoughts": 5
            }),
        )
        .await;
    let parsed2 = McpClient::get_parsed(&resp2);
    assert!(
        parsed2.get("firstCallGuidance").is_none(),
        "Second thought should not have firstCallGuidance"
    );
}

#[tokio::test]
async fn test_compliance_tracking() {
    let mut client = McpClient::new().await;

    // Send 5 linear thoughts without branching
    for i in 1..=5 {
        let resp = client
            .tool_call(
                "sequentialthinking",
                json!({
                    "thought": format!("Linear thought {}", i),
                    "thoughtNumber": i,
                    "totalThoughts": 8
                }),
            )
            .await;

        if i >= 4 {
            let parsed = McpClient::get_parsed(&resp);
            let compliance = &parsed["compliance"];
            assert!(
                compliance["needsBranching"].as_bool().unwrap_or(false),
                "After {} linear thoughts, needsBranching should be true",
                i
            );
            assert!(
                compliance["consecutiveLinearThoughts"].as_u64().unwrap() >= 4,
                "Should track consecutive linear thoughts"
            );
        }
    }
}

#[tokio::test]
async fn test_explore_mode() {
    let mut client = McpClient::new().await;
    let resp = client
        .tool_call(
            "sequentialthinking",
            json!({
                "thought": "Exploring alternatives",
                "thoughtNumber": 1,
                "totalThoughts": 5,
                "continuationMode": "explore",
                "exploreCount": 3,
                "proposals": ["Use recursion", "Use iteration", "Use memoization"]
            }),
        )
        .await;
    assert!(!McpClient::is_error(&resp));
    assert!(!McpClient::is_tool_error(&resp));

    let parsed = McpClient::get_parsed(&resp);
    assert_eq!(parsed["nextThoughtNeeded"], true);
    assert_eq!(parsed["thoughtHistoryLength"], 1);
}

#[tokio::test]
async fn test_done_mode() {
    let mut client = McpClient::new().await;
    let resp = client
        .tool_call(
            "sequentialthinking",
            json!({
                "thought": "The answer is 42",
                "thoughtNumber": 2,
                "totalThoughts": 5,
                "continuationMode": "done",
                "doneReason": "sufficient",
                "confidence": 0.9
            }),
        )
        .await;
    let parsed = McpClient::get_parsed(&resp);
    assert_eq!(
        parsed["nextThoughtNeeded"], false,
        "done mode should set nextThoughtNeeded=false"
    );
}

#[tokio::test]
async fn test_validation_empty_thought() {
    let mut client = McpClient::new().await;
    let resp = client
        .tool_call(
            "sequentialthinking",
            json!({
                "thought": "",
                "thoughtNumber": 1,
                "totalThoughts": 3
            }),
        )
        .await;
    // Should return tool error (isError=true) not protocol error
    assert!(
        McpClient::is_tool_error(&resp),
        "Empty thought should return tool error"
    );
    let parsed = McpClient::get_parsed(&resp);
    assert!(
        parsed["error"].as_str().unwrap_or("").contains("non-empty"),
        "Error should mention non-empty requirement"
    );
}

#[tokio::test]
async fn test_validation_zero_thought_number() {
    let mut client = McpClient::new().await;
    let resp = client
        .tool_call(
            "sequentialthinking",
            json!({
                "thought": "Test",
                "thoughtNumber": 0,
                "totalThoughts": 3
            }),
        )
        .await;
    assert!(
        McpClient::is_tool_error(&resp),
        "Zero thoughtNumber should return tool error"
    );
}

#[tokio::test]
async fn test_auto_adjust_total_thoughts() {
    let mut client = McpClient::new().await;
    // thoughtNumber > totalThoughts should auto-adjust
    let resp = client
        .tool_call(
            "sequentialthinking",
            json!({
                "thought": "Went over estimate",
                "thoughtNumber": 5,
                "totalThoughts": 3
            }),
        )
        .await;
    let parsed = McpClient::get_parsed(&resp);
    assert_eq!(
        parsed["totalThoughts"], 5,
        "totalThoughts should be adjusted to match thoughtNumber"
    );
}

#[tokio::test]
async fn test_search_query_passthrough() {
    let mut client = McpClient::new().await;
    let resp = client
        .tool_call(
            "sequentialthinking",
            json!({
                "thought": "Need more info about X",
                "thoughtNumber": 1,
                "totalThoughts": 3,
                "searchQuery": "how does X work"
            }),
        )
        .await;
    let parsed = McpClient::get_parsed(&resp);
    assert_eq!(
        parsed["pendingSearchQuery"],
        "how does X work",
        "Should pass through search query"
    );
    assert!(
        parsed.get("hint").is_some(),
        "Should include hint about executing search"
    );
}

#[tokio::test]
async fn test_revision() {
    let mut client = McpClient::new().await;

    // Initial thought
    let _ = client
        .tool_call(
            "sequentialthinking",
            json!({
                "thought": "Initial analysis",
                "thoughtNumber": 1,
                "totalThoughts": 3
            }),
        )
        .await;

    // Revision of thought 1
    let resp = client
        .tool_call(
            "sequentialthinking",
            json!({
                "thought": "Actually, my initial analysis was wrong because...",
                "thoughtNumber": 2,
                "totalThoughts": 3,
                "isRevision": true,
                "revisesThought": 1
            }),
        )
        .await;
    assert!(!McpClient::is_error(&resp));
    assert!(!McpClient::is_tool_error(&resp));

    let parsed = McpClient::get_parsed(&resp);
    assert_eq!(parsed["thoughtHistoryLength"], 2);
}

#[tokio::test]
async fn test_layer_abstraction() {
    let mut client = McpClient::new().await;
    let resp = client
        .tool_call(
            "sequentialthinking",
            json!({
                "thought": "Understanding the problem domain",
                "thoughtNumber": 1,
                "totalThoughts": 3,
                "layer": 1
            }),
        )
        .await;
    assert!(!McpClient::is_error(&resp));
    assert!(!McpClient::is_tool_error(&resp));

    // Layer should be accepted without error
    let parsed = McpClient::get_parsed(&resp);
    assert_eq!(parsed["thoughtNumber"], 1);
}

#[tokio::test]
async fn test_confidence_clamping() {
    let mut client = McpClient::new().await;

    // Confidence > 1.0 should be clamped
    let resp = client
        .tool_call(
            "sequentialthinking",
            json!({
                "thought": "Very confident",
                "thoughtNumber": 1,
                "totalThoughts": 2,
                "confidence": 1.5
            }),
        )
        .await;
    assert!(!McpClient::is_tool_error(&resp));
    // Should not crash - response should be valid
    let parsed = McpClient::get_parsed(&resp);
    assert!(parsed.get("guidance").is_some(), "High confidence should produce guidance");
}

#[tokio::test]
async fn test_merge_mode() {
    let mut client = McpClient::new().await;

    // Create two branches first
    let _ = client
        .tool_call(
            "sequentialthinking",
            json!({
                "thought": "Base",
                "thoughtNumber": 1,
                "totalThoughts": 4
            }),
        )
        .await;
    let _ = client
        .tool_call(
            "sequentialthinking",
            json!({
                "thought": "Branch A",
                "thoughtNumber": 2,
                "totalThoughts": 4,
                "branchFromThought": 1,
                "branchId": "merge-a"
            }),
        )
        .await;
    let _ = client
        .tool_call(
            "sequentialthinking",
            json!({
                "thought": "Branch B",
                "thoughtNumber": 3,
                "totalThoughts": 4,
                "branchFromThought": 1,
                "branchId": "merge-b"
            }),
        )
        .await;

    // Merge
    let resp = client
        .tool_call(
            "sequentialthinking",
            json!({
                "thought": "Combining insights from both branches",
                "thoughtNumber": 4,
                "totalThoughts": 4,
                "continuationMode": "merge"
            }),
        )
        .await;
    assert!(!McpClient::is_error(&resp));
    assert!(!McpClient::is_tool_error(&resp));

    let parsed = McpClient::get_parsed(&resp);
    let branches = parsed["branches"].as_array().unwrap();
    assert_eq!(branches.len(), 2, "Should still have both branches tracked");
}
