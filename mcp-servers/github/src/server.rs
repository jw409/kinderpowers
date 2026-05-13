use chrono::Utc;
use rmcp::handler::server::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{
    CallToolResult, Content, ListResourceTemplatesResult, ReadResourceRequestParams,
    ReadResourceResult, ResourceContents, ServerInfo,
};
use rmcp::{ErrorData as McpError, ServerHandler, ServiceExt};
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::Value;
use std::sync::Arc;

use crate::compress::{compress, CompressConfig, OutputFormat};
use crate::format::format_output;
use crate::github::client::GithubClient;
use crate::tools;

pub struct KpGithubServer {
    client: Arc<GithubClient>,
    tool_router: ToolRouter<Self>,
}

impl KpGithubServer {
    fn new(token: &str) -> Self {
        Self {
            client: Arc::new(GithubClient::new(token)),
            tool_router: Self::tool_router(),
        }
    }

    #[cfg(test)]
    fn with_client(client: GithubClient) -> Self {
        Self {
            client: Arc::new(client),
            tool_router: Self::tool_router(),
        }
    }

    /// Compress search results: compress the items array but preserve total_count/truncated metadata.
    fn compress_and_format_search(&self, value: Value, fields: Option<Vec<String>>, format: Option<String>) -> String {
        // If this is a search wrapper {items, total_count, truncated}, extract items for compression
        if let Value::Object(ref map) = value {
            if let Some(items) = map.get("items") {
                let compressed_items = self.compress_and_format(items.clone(), fields, format);
                let total = map.get("total_count").and_then(|v| v.as_u64()).unwrap_or(0);
                let truncated = map.get("truncated").and_then(|v| v.as_bool()).unwrap_or(false);
                if truncated {
                    return format!("{compressed_items}\n\n[{total} total results, showing first batch]");
                }
                return compressed_items;
            }
        }
        // Fallback: not a search wrapper
        self.compress_and_format(value, fields, format)
    }

    fn compress_and_format(&self, value: Value, fields: Option<Vec<String>>, format: Option<String>) -> String {
        let fmt = format
            .as_deref()
            .map(|s| match s {
                "json" => OutputFormat::Json,
                "table" => OutputFormat::Table,
                "text" => OutputFormat::Text,
                _ => OutputFormat::Auto,
            })
            .unwrap_or(OutputFormat::Auto);

        let config = CompressConfig {
            fields,
            format: fmt,
            ..CompressConfig::default()
        };

        let compressed = compress(&value, &config, Utc::now());
        format_output(&compressed, config.format)
    }
}

// --- Action enums ---

#[derive(Debug, Default, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum IssueAction {
    #[default]
    List,
    Get,
    Search,
    Create,
    Update,
    Comment,
    Comments,
    Labels,
    SubIssues,
    ListTypes,
}

#[derive(Debug, Default, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PrAction {
    #[default]
    List,
    Get,
    Diff,
    Files,
    Create,
    Update,
    Merge,
    Reviews,
    ReviewComments,
    Comments,
    CheckRuns,
    Status,
    CreateReview,
    UpdateBranch,
    Search,
    AddReviewComment,
    ReplyToComment,
    SubmitReview,
    DeleteReview,
}

#[derive(Debug, Default, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum FileAction {
    #[default]
    Get,
    CreateOrUpdate,
    Delete,
    Push,
}

#[derive(Debug, Default, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RepoAction {
    #[default]
    Get,
    Create,
    Fork,
    Compare,
    Search,
}

#[derive(Debug, Default, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum BranchAction {
    #[default]
    List,
    Create,
}

#[derive(Debug, Default, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CommitAction {
    #[default]
    List,
    Get,
}

#[derive(Debug, Default, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReleaseAction {
    #[default]
    List,
    GetByTag,
    GetLatest,
    Create,
}

#[derive(Debug, Default, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TagAction {
    #[default]
    List,
    Get,
}

#[derive(Debug, Default, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TeamAction {
    #[default]
    List,
    Members,
}

#[derive(Debug, Default, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum UserAction {
    #[default]
    Me,
    Search,
}

#[derive(Debug, Default, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum LabelAction {
    #[default]
    List,
    Get,
    Create,
    Update,
    Delete,
}

#[derive(Debug, Default, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ActionAction {
    #[default]
    ListRuns,
    GetRun,
    Rerun,
    ListWorkflows,
    RunLogs,
}

// --- Consolidated parameter structs ---

#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct IssuesParams {
    pub action: IssueAction,
    /// Repository owner (also used as org name for list_types)
    pub owner: String,
    #[serde(default)] pub repo: Option<String>,
    #[serde(default)] pub number: Option<u32>,
    #[serde(default)] pub state: Option<String>,
    #[serde(default)] pub limit: Option<u32>,
    #[serde(default)] pub query: Option<String>,
    #[serde(default)] pub title: Option<String>,
    #[serde(default)] pub body: Option<String>,
    #[serde(default)] pub labels: Option<Vec<String>>,
    #[serde(default)] pub assignees: Option<Vec<String>>,
    #[serde(default)] pub fields: Option<Vec<String>>,
    #[serde(default)] pub format: Option<String>,
}

#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct PrsParams {
    pub action: PrAction,
    pub owner: String,
    pub repo: String,
    #[serde(default)] pub number: Option<u32>,
    #[serde(default)] pub state: Option<String>,
    #[serde(default)] pub limit: Option<u32>,
    #[serde(default)] pub query: Option<String>,
    #[serde(default)] pub title: Option<String>,
    #[serde(default)] pub head: Option<String>,
    #[serde(default)] pub base: Option<String>,
    #[serde(default)] pub body: Option<String>,
    #[serde(default)] pub draft: Option<bool>,
    #[serde(default)] pub merge_method: Option<String>,
    #[serde(default)] pub commit_title: Option<String>,
    #[serde(default)] pub commit_message: Option<String>,
    #[serde(default)] pub event: Option<String>,
    #[serde(default)] pub review_id: Option<u64>,
    #[serde(default)] pub comment_id: Option<u64>,
    #[serde(default)] pub path: Option<String>,
    #[serde(default)] pub line: Option<u32>,
    #[serde(default)] pub side: Option<String>,
    #[serde(default)] pub subject_type: Option<String>,
    #[serde(default)] pub include_patches: bool,
    #[serde(default)] pub fields: Option<Vec<String>>,
    #[serde(default)] pub format: Option<String>,
}

#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct FilesParams {
    pub action: FileAction,
    pub owner: String,
    pub repo: String,
    #[serde(default)] pub path: Option<String>,
    #[serde(default)] pub git_ref: Option<String>,
    #[serde(default)] pub content: Option<String>,
    #[serde(default)] pub message: Option<String>,
    #[serde(default)] pub branch: Option<String>,
    #[serde(default)] pub sha: Option<String>,
    #[serde(default)] pub files_json: Option<String>,
    #[serde(default)] pub author_name: Option<String>,
    #[serde(default)] pub author_email: Option<String>,
    #[serde(default)] pub committer_name: Option<String>,
    #[serde(default)] pub committer_email: Option<String>,
    #[serde(default)] pub max_content_bytes: Option<u32>,
    #[serde(default)] pub fields: Option<Vec<String>>,
    #[serde(default)] pub format: Option<String>,
}

#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct ReposParams {
    pub action: RepoAction,
    #[serde(default)] pub owner: Option<String>,
    #[serde(default)] pub repo: Option<String>,
    #[serde(default)] pub name: Option<String>,
    #[serde(default)] pub description: Option<String>,
    #[serde(default)] pub private: Option<bool>,
    #[serde(default)] pub org: Option<String>,
    #[serde(default)] pub base: Option<String>,
    #[serde(default)] pub head: Option<String>,
    #[serde(default)] pub query: Option<String>,
    #[serde(default)] pub limit: Option<u32>,
    #[serde(default)] pub include_patches: bool,
    #[serde(default)] pub fields: Option<Vec<String>>,
    #[serde(default)] pub format: Option<String>,
}

#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct BranchesParams {
    pub action: BranchAction,
    pub owner: String,
    pub repo: String,
    #[serde(default)] pub branch: Option<String>,
    #[serde(default)] pub from_sha: Option<String>,
    #[serde(default)] pub limit: Option<u32>,
    #[serde(default)] pub fields: Option<Vec<String>>,
    #[serde(default)] pub format: Option<String>,
}

#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct CommitsParams {
    pub action: CommitAction,
    pub owner: String,
    pub repo: String,
    #[serde(default)] pub sha: Option<String>,
    #[serde(default)] pub limit: Option<u32>,
    #[serde(default)] pub include_patches: bool,
    #[serde(default)] pub fields: Option<Vec<String>>,
    #[serde(default)] pub format: Option<String>,
}

#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct ReleasesParams {
    pub action: ReleaseAction,
    pub owner: String,
    pub repo: String,
    #[serde(default)] pub tag: Option<String>,
    #[serde(default)] pub tag_name: Option<String>,
    #[serde(default)] pub name: Option<String>,
    #[serde(default)] pub body: Option<String>,
    #[serde(default)] pub draft: Option<bool>,
    #[serde(default)] pub prerelease: Option<bool>,
    #[serde(default)] pub limit: Option<u32>,
    #[serde(default)] pub fields: Option<Vec<String>>,
    #[serde(default)] pub format: Option<String>,
}

#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct TagsParams {
    pub action: TagAction,
    pub owner: String,
    pub repo: String,
    #[serde(default)] pub tag: Option<String>,
    #[serde(default)] pub limit: Option<u32>,
    #[serde(default)] pub fields: Option<Vec<String>>,
    #[serde(default)] pub format: Option<String>,
}

#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct TeamsParams {
    pub action: TeamAction,
    #[serde(default)] pub org: Option<String>,
    #[serde(default)] pub team_slug: Option<String>,
    #[serde(default)] pub limit: Option<u32>,
    #[serde(default)] pub fields: Option<Vec<String>>,
    #[serde(default)] pub format: Option<String>,
}

#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct UserParams {
    pub action: UserAction,
    #[serde(default)] pub query: Option<String>,
    #[serde(default)] pub limit: Option<u32>,
    #[serde(default)] pub fields: Option<Vec<String>>,
    #[serde(default)] pub format: Option<String>,
}

#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct LabelsParams {
    pub action: LabelAction,
    pub owner: String,
    pub repo: String,
    #[serde(default)] pub name: Option<String>,
    #[serde(default)] pub new_name: Option<String>,
    #[serde(default)] pub color: Option<String>,
    #[serde(default)] pub description: Option<String>,
    #[serde(default)] pub limit: Option<u32>,
    #[serde(default)] pub fields: Option<Vec<String>>,
    #[serde(default)] pub format: Option<String>,
}

#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct ActionsParams {
    pub action: ActionAction,
    pub owner: String,
    pub repo: String,
    #[serde(default)] pub run_id: Option<u64>,
    #[serde(default)] pub limit: Option<u32>,
    #[serde(default)] pub fields: Option<Vec<String>>,
    #[serde(default)] pub format: Option<String>,
}

/// Single-operation: code search has no peer verbs.
#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct CodeSearchParams {
    pub query: String,
    #[serde(default)] pub limit: Option<u32>,
    #[serde(default)] pub fields: Option<Vec<String>>,
    #[serde(default)] pub format: Option<String>,
}

// --- Tool router ---

#[rmcp::tool_router]
impl KpGithubServer {
    #[rmcp::tool(
        name = "github_issues",
        description = "GitHub issues: list, get, search, create, update, comment, comments, labels, sub_issues, list_types. action=list: owner,repo,state?,limit?. action=get: owner,repo,number. action=search: owner,query,limit?. action=create: owner,repo,title,body?,labels?,assignees?. action=update: owner,repo,number + any of title/body/state/labels/assignees. action=comment: owner,repo,number,body. action=comments: owner,repo,number,limit?. action=labels: owner,repo,number. action=sub_issues: owner,repo,number,limit?. action=list_types: owner (as org)."
    )]
    async fn github_issues(&self, Parameters(p): Parameters<IssuesParams>) -> Result<CallToolResult, McpError> {
        let repo = || p.repo.as_deref().ok_or_else(|| McpError::invalid_params("repo required", None));
        let number = || p.number.ok_or_else(|| McpError::invalid_params("number required", None));

        match p.action {
            IssueAction::List => {
                let result = tools::issues::list(&self.client, &p.owner, repo()?, p.state.as_deref(), p.limit)
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                let text = self.compress_and_format(result, p.fields, p.format);
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            IssueAction::Get => {
                let result = tools::issues::get(&self.client, &p.owner, repo()?, number()?)
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                let text = self.compress_and_format(result, p.fields, p.format);
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            IssueAction::Search => {
                let q = p.query.as_deref().ok_or_else(|| McpError::invalid_params("query required", None))?;
                let result = tools::issues::search(&self.client, q, p.limit)
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                let text = self.compress_and_format_search(result, p.fields, p.format);
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            IssueAction::Create => {
                let title = p.title.as_deref().ok_or_else(|| McpError::invalid_params("title required", None))?;
                let result = tools::issues::create(&self.client, &p.owner, repo()?, title, p.body.as_deref(), p.labels, p.assignees)
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                let text = self.compress_and_format(result, p.fields, p.format);
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            IssueAction::Update => {
                let result = tools::issues::update(&self.client, &p.owner, repo()?, number()?, p.title, p.body, p.state, p.labels, p.assignees)
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                let text = self.compress_and_format(result, p.fields, p.format);
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            IssueAction::Comment => {
                let body = p.body.as_deref().ok_or_else(|| McpError::invalid_params("body required", None))?;
                let result = tools::issues::comment(&self.client, &p.owner, repo()?, number()?, body)
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                let text = self.compress_and_format(result, p.fields, p.format);
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            IssueAction::Comments => {
                let result = tools::issues::comments(&self.client, &p.owner, repo()?, number()?, p.limit)
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                let text = self.compress_and_format(result, p.fields, p.format);
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            IssueAction::Labels => {
                let result = tools::issues::labels(&self.client, &p.owner, repo()?, number()?)
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                let text = self.compress_and_format(result, p.fields, p.format);
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            IssueAction::SubIssues => {
                let result = tools::issues::sub_issues(&self.client, &p.owner, repo()?, number()?, p.limit)
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                let text = self.compress_and_format(result, p.fields, p.format);
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            IssueAction::ListTypes => {
                let result = tools::issues::list_issue_types(&self.client, &p.owner)
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                let text = self.compress_and_format(result, p.fields, p.format);
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
        }
    }

    #[rmcp::tool(
        name = "github_prs",
        description = "GitHub pull requests: list, get, diff, files, create, update, merge, reviews, review_comments, comments, check_runs, status, create_review, update_branch, search, add_review_comment, reply_to_comment, submit_review, delete_review. action=list: owner,repo,state?,limit?. action=get: owner,repo,number. action=diff: owner,repo,number. action=files: owner,repo,number,limit?,include_patches?. action=create: owner,repo,title,head,base,body?,draft?. action=update: owner,repo,number + any of title/body/state/base. action=merge: owner,repo,number,merge_method?,commit_title?,commit_message?. action=reviews: owner,repo,number,limit?. action=review_comments: owner,repo,number,limit?. action=comments: owner,repo,number,limit?. action=check_runs: owner,repo,number,limit?. action=status: owner,repo,number. action=create_review: owner,repo,number,event,body?. action=update_branch: owner,repo,number. action=search: owner,query,limit?. action=add_review_comment: owner,repo,number,path,body,line?,side?,subject_type?. action=reply_to_comment: owner,repo,number,comment_id,body. action=submit_review: owner,repo,number,review_id,event,body?. action=delete_review: owner,repo,number,review_id."
    )]
    async fn github_prs(&self, Parameters(p): Parameters<PrsParams>) -> Result<CallToolResult, McpError> {
        let number = || p.number.ok_or_else(|| McpError::invalid_params("number required", None));
        let review_id = || p.review_id.ok_or_else(|| McpError::invalid_params("review_id required", None));
        let comment_id = || p.comment_id.ok_or_else(|| McpError::invalid_params("comment_id required", None));

        match p.action {
            PrAction::List => {
                let result = tools::prs::list(&self.client, &p.owner, &p.repo, p.state.as_deref(), p.limit)
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                let text = self.compress_and_format(result, p.fields, p.format);
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            PrAction::Get => {
                let result = tools::prs::get(&self.client, &p.owner, &p.repo, number()?)
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                let text = self.compress_and_format(result, p.fields, p.format);
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            PrAction::Diff => {
                let text = tools::prs::diff(&self.client, &p.owner, &p.repo, number()?)
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            PrAction::Files => {
                let result = tools::prs::files(&self.client, &p.owner, &p.repo, number()?, p.limit, p.include_patches)
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                let text = self.compress_and_format(result, p.fields, p.format);
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            PrAction::Create => {
                let title = p.title.as_deref().ok_or_else(|| McpError::invalid_params("title required", None))?;
                let head = p.head.as_deref().ok_or_else(|| McpError::invalid_params("head required", None))?;
                let base = p.base.as_deref().ok_or_else(|| McpError::invalid_params("base required", None))?;
                let result = tools::prs::create(&self.client, &p.owner, &p.repo, title, head, base, p.body.as_deref(), p.draft)
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                let text = self.compress_and_format(result, p.fields, p.format);
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            PrAction::Update => {
                let result = tools::prs::update(&self.client, &p.owner, &p.repo, number()?, p.title.as_deref(), p.body.as_deref(), p.state.as_deref(), p.base.as_deref())
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                let text = self.compress_and_format(result, p.fields, p.format);
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            PrAction::Merge => {
                let result = tools::prs::merge(&self.client, &p.owner, &p.repo, number()?, p.merge_method.as_deref(), p.commit_title.as_deref(), p.commit_message.as_deref())
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                let text = self.compress_and_format(result, p.fields, p.format);
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            PrAction::Reviews => {
                let result = tools::prs::reviews(&self.client, &p.owner, &p.repo, number()?, p.limit)
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                let text = self.compress_and_format(result, p.fields, p.format);
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            PrAction::ReviewComments => {
                let result = tools::prs::review_comments(&self.client, &p.owner, &p.repo, number()?, p.limit)
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                let text = self.compress_and_format(result, p.fields, p.format);
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            PrAction::Comments => {
                let result = tools::prs::comments(&self.client, &p.owner, &p.repo, number()?, p.limit)
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                let text = self.compress_and_format(result, p.fields, p.format);
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            PrAction::CheckRuns => {
                let result = tools::prs::check_runs(&self.client, &p.owner, &p.repo, number()?, p.limit)
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                let text = self.compress_and_format(result, p.fields, p.format);
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            PrAction::Status => {
                let result = tools::prs::status(&self.client, &p.owner, &p.repo, number()?)
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                let text = self.compress_and_format(result, p.fields, p.format);
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            PrAction::CreateReview => {
                let event = p.event.as_deref().ok_or_else(|| McpError::invalid_params("event required", None))?;
                let result = tools::prs::create_review(&self.client, &p.owner, &p.repo, number()?, event, p.body.as_deref())
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                let text = self.compress_and_format(result, p.fields, p.format);
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            PrAction::UpdateBranch => {
                let result = tools::prs::update_branch(&self.client, &p.owner, &p.repo, number()?)
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                let text = self.compress_and_format(result, p.fields, p.format);
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            PrAction::Search => {
                let q = p.query.as_deref().ok_or_else(|| McpError::invalid_params("query required", None))?;
                let result = tools::prs::search(&self.client, q, p.limit)
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                let text = self.compress_and_format_search(result, p.fields, p.format);
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            PrAction::AddReviewComment => {
                let path = p.path.as_deref().ok_or_else(|| McpError::invalid_params("path required", None))?;
                let body = p.body.as_deref().ok_or_else(|| McpError::invalid_params("body required", None))?;
                let result = tools::prs::add_review_comment(&self.client, &p.owner, &p.repo, number()?, path, body, p.line, p.side.as_deref(), p.subject_type.as_deref())
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                let text = self.compress_and_format(result, p.fields, p.format);
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            PrAction::ReplyToComment => {
                let body = p.body.as_deref().ok_or_else(|| McpError::invalid_params("body required", None))?;
                let result = tools::prs::reply_to_comment(&self.client, &p.owner, &p.repo, number()?, comment_id()?, body)
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                let text = self.compress_and_format(result, p.fields, p.format);
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            PrAction::SubmitReview => {
                let event = p.event.as_deref().ok_or_else(|| McpError::invalid_params("event required", None))?;
                let result = tools::prs::submit_review(&self.client, &p.owner, &p.repo, number()?, review_id()?, event, p.body.as_deref())
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                let text = self.compress_and_format(result, p.fields, p.format);
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            PrAction::DeleteReview => {
                let result = tools::prs::delete_review(&self.client, &p.owner, &p.repo, number()?, review_id()?)
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                let text = self.compress_and_format(result, p.fields, p.format);
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
        }
    }

    #[rmcp::tool(
        name = "github_files",
        description = "GitHub file operations: get, create_or_update, delete, push. action=get: owner,repo,path,git_ref?. action=create_or_update: owner,repo,path,content(base64),message,branch,sha?(for updates),author_name?,author_email?,committer_name?,committer_email?. action=delete: owner,repo,path,message,branch,sha,author_name?,author_email?,committer_name?,committer_email?. action=push: owner,repo,branch,message,files_json(JSON array of {path,content}),author_name?,author_email?,committer_name?,committer_email?."
    )]
    async fn github_files(&self, Parameters(p): Parameters<FilesParams>) -> Result<CallToolResult, McpError> {
        match p.action {
            FileAction::Get => {
                let path = p.path.as_deref().ok_or_else(|| McpError::invalid_params("path required", None))?;
                let result = tools::files::get_contents(&self.client, &p.owner, &p.repo, path, p.git_ref.as_deref())
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                let text = self.compress_and_format(result, p.fields, p.format);
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            FileAction::CreateOrUpdate => {
                let path = p.path.as_deref().ok_or_else(|| McpError::invalid_params("path required", None))?;
                let content = p.content.as_deref().ok_or_else(|| McpError::invalid_params("content required", None))?;
                let message = p.message.as_deref().ok_or_else(|| McpError::invalid_params("message required", None))?;
                let branch = p.branch.as_deref().ok_or_else(|| McpError::invalid_params("branch required", None))?;
                let identity = tools::files::CommitIdentity {
                    author_name: p.author_name,
                    author_email: p.author_email,
                    committer_name: p.committer_name,
                    committer_email: p.committer_email,
                };
                let result = tools::files::create_or_update(&self.client, &p.owner, &p.repo, path, content, message, branch, p.sha.as_deref(), &identity)
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                let text = self.compress_and_format(result, p.fields, p.format);
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            FileAction::Delete => {
                let path = p.path.as_deref().ok_or_else(|| McpError::invalid_params("path required", None))?;
                let message = p.message.as_deref().ok_or_else(|| McpError::invalid_params("message required", None))?;
                let branch = p.branch.as_deref().ok_or_else(|| McpError::invalid_params("branch required", None))?;
                let sha = p.sha.as_deref().ok_or_else(|| McpError::invalid_params("sha required", None))?;
                let identity = tools::files::CommitIdentity {
                    author_name: p.author_name,
                    author_email: p.author_email,
                    committer_name: p.committer_name,
                    committer_email: p.committer_email,
                };
                let result = tools::files::delete(&self.client, &p.owner, &p.repo, path, message, branch, sha, &identity)
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                let text = self.compress_and_format(result, p.fields, p.format);
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            FileAction::Push => {
                let branch = p.branch.as_deref().ok_or_else(|| McpError::invalid_params("branch required", None))?;
                let message = p.message.as_deref().ok_or_else(|| McpError::invalid_params("message required", None))?;
                let files_json = p.files_json.as_deref().ok_or_else(|| McpError::invalid_params("files_json required", None))?;
                let identity = tools::files::CommitIdentity {
                    author_name: p.author_name,
                    author_email: p.author_email,
                    committer_name: p.committer_name,
                    committer_email: p.committer_email,
                };
                let result = tools::files::push_files(&self.client, &p.owner, &p.repo, branch, message, files_json, &identity)
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                let text = self.compress_and_format(result, p.fields, p.format);
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
        }
    }

    #[rmcp::tool(
        name = "github_repos",
        description = "GitHub repositories: get, create, fork, compare, search. action=get: owner,repo. action=create: name,description?,private?,org?. action=fork: owner,repo,org?. action=compare: owner,repo,base,head,include_patches?. action=search: query,limit?."
    )]
    async fn github_repos(&self, Parameters(p): Parameters<ReposParams>) -> Result<CallToolResult, McpError> {
        let owner = || p.owner.as_deref().ok_or_else(|| McpError::invalid_params("owner required", None));
        let repo = || p.repo.as_deref().ok_or_else(|| McpError::invalid_params("repo required", None));

        match p.action {
            RepoAction::Get => {
                let result = tools::repos::get(&self.client, owner()?, repo()?)
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                let text = self.compress_and_format(result, p.fields, p.format);
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            RepoAction::Create => {
                let name = p.name.as_deref().ok_or_else(|| McpError::invalid_params("name required", None))?;
                let result = tools::repos::create(&self.client, name, p.description.as_deref(), p.private, p.org.as_deref())
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                let text = self.compress_and_format(result, p.fields, p.format);
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            RepoAction::Fork => {
                let result = tools::repos::fork(&self.client, owner()?, repo()?, p.org.as_deref())
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                let text = self.compress_and_format(result, p.fields, p.format);
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            RepoAction::Compare => {
                let base = p.base.as_deref().ok_or_else(|| McpError::invalid_params("base required", None))?;
                let head = p.head.as_deref().ok_or_else(|| McpError::invalid_params("head required", None))?;
                let result = tools::repos::compare(&self.client, owner()?, repo()?, base, head, p.include_patches)
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                let text = self.compress_and_format(result, p.fields, p.format);
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            RepoAction::Search => {
                let q = p.query.as_deref().ok_or_else(|| McpError::invalid_params("query required", None))?;
                let result = tools::repos::search(&self.client, q, p.limit)
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                let text = self.compress_and_format_search(result, p.fields, p.format);
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
        }
    }

    #[rmcp::tool(
        name = "github_branches",
        description = "GitHub branches: list, create. action=list: owner,repo,limit?. action=create: owner,repo,branch,from_sha."
    )]
    async fn github_branches(&self, Parameters(p): Parameters<BranchesParams>) -> Result<CallToolResult, McpError> {
        match p.action {
            BranchAction::List => {
                let result = tools::branches::list(&self.client, &p.owner, &p.repo, p.limit)
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                let text = self.compress_and_format(result, p.fields, p.format);
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            BranchAction::Create => {
                let branch = p.branch.as_deref().ok_or_else(|| McpError::invalid_params("branch required", None))?;
                let from_sha = p.from_sha.as_deref().ok_or_else(|| McpError::invalid_params("from_sha required", None))?;
                let result = tools::branches::create(&self.client, &p.owner, &p.repo, branch, from_sha)
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                let text = self.compress_and_format(result, p.fields, p.format);
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
        }
    }

    #[rmcp::tool(
        name = "github_commits",
        description = "GitHub commits: list, get. action=list: owner,repo,sha?,limit?. action=get: owner,repo,sha,include_patches?."
    )]
    async fn github_commits(&self, Parameters(p): Parameters<CommitsParams>) -> Result<CallToolResult, McpError> {
        match p.action {
            CommitAction::List => {
                let result = tools::commits::list(&self.client, &p.owner, &p.repo, p.sha.as_deref(), p.limit)
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                let text = self.compress_and_format(result, p.fields, p.format);
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            CommitAction::Get => {
                let sha = p.sha.as_deref().ok_or_else(|| McpError::invalid_params("sha required", None))?;
                let result = tools::commits::get(&self.client, &p.owner, &p.repo, sha, p.include_patches)
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                let text = self.compress_and_format(result, p.fields, p.format);
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
        }
    }

    #[rmcp::tool(
        name = "github_releases",
        description = "GitHub releases: list, get_by_tag, get_latest, create. action=list: owner,repo,limit?. action=get_by_tag: owner,repo,tag. action=get_latest: owner,repo. action=create: owner,repo,tag_name,name?,body?,draft?,prerelease?."
    )]
    async fn github_releases(&self, Parameters(p): Parameters<ReleasesParams>) -> Result<CallToolResult, McpError> {
        match p.action {
            ReleaseAction::List => {
                let result = tools::releases::list(&self.client, &p.owner, &p.repo, p.limit)
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                let text = self.compress_and_format(result, p.fields, p.format);
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            ReleaseAction::GetByTag => {
                let tag = p.tag.as_deref().ok_or_else(|| McpError::invalid_params("tag required", None))?;
                let result = tools::releases::get_by_tag(&self.client, &p.owner, &p.repo, tag)
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                let text = self.compress_and_format(result, p.fields, p.format);
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            ReleaseAction::GetLatest => {
                let result = tools::releases::get_latest(&self.client, &p.owner, &p.repo)
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                let text = self.compress_and_format(result, p.fields, p.format);
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            ReleaseAction::Create => {
                let tag_name = p.tag_name.as_deref().ok_or_else(|| McpError::invalid_params("tag_name required", None))?;
                let result = tools::releases::create(&self.client, &p.owner, &p.repo, tag_name, p.name.as_deref(), p.body.as_deref(), p.draft, p.prerelease)
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                let text = self.compress_and_format(result, p.fields, p.format);
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
        }
    }

    #[rmcp::tool(
        name = "github_tags",
        description = "GitHub tags: list, get. action=list: owner,repo,limit?. action=get: owner,repo,tag."
    )]
    async fn github_tags(&self, Parameters(p): Parameters<TagsParams>) -> Result<CallToolResult, McpError> {
        match p.action {
            TagAction::List => {
                let result = tools::tags::list(&self.client, &p.owner, &p.repo, p.limit)
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                let text = self.compress_and_format(result, p.fields, p.format);
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            TagAction::Get => {
                let tag = p.tag.as_deref().ok_or_else(|| McpError::invalid_params("tag required", None))?;
                let result = tools::tags::get(&self.client, &p.owner, &p.repo, tag)
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                let text = self.compress_and_format(result, p.fields, p.format);
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
        }
    }

    #[rmcp::tool(
        name = "github_teams",
        description = "GitHub teams: list, members. action=list: limit?. action=members: org,team_slug,limit?."
    )]
    async fn github_teams(&self, Parameters(p): Parameters<TeamsParams>) -> Result<CallToolResult, McpError> {
        match p.action {
            TeamAction::List => {
                let result = tools::teams::list(&self.client, p.limit)
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                let text = self.compress_and_format(result, p.fields, p.format);
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            TeamAction::Members => {
                let org = p.org.as_deref().ok_or_else(|| McpError::invalid_params("org required", None))?;
                let team_slug = p.team_slug.as_deref().ok_or_else(|| McpError::invalid_params("team_slug required", None))?;
                let result = tools::teams::members(&self.client, org, team_slug, p.limit)
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                let text = self.compress_and_format(result, p.fields, p.format);
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
        }
    }

    #[rmcp::tool(
        name = "github_user",
        description = "GitHub user operations: me, search. action=me: (no params). action=search: query,limit?."
    )]
    async fn github_user(&self, Parameters(p): Parameters<UserParams>) -> Result<CallToolResult, McpError> {
        match p.action {
            UserAction::Me => {
                let result = tools::user::me(&self.client)
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                let text = self.compress_and_format(result, p.fields, p.format);
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            UserAction::Search => {
                let q = p.query.as_deref().ok_or_else(|| McpError::invalid_params("query required", None))?;
                let result = tools::user::search(&self.client, q, p.limit)
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                let text = self.compress_and_format_search(result, p.fields, p.format);
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
        }
    }

    #[rmcp::tool(
        name = "github_labels",
        description = "GitHub repo labels: list, get, create, update, delete. action=list: owner,repo,limit?. action=get: owner,repo,name. action=create: owner,repo,name,color?,description?. action=update: owner,repo,name,new_name?,color?,description?. action=delete: owner,repo,name."
    )]
    async fn github_labels(&self, Parameters(p): Parameters<LabelsParams>) -> Result<CallToolResult, McpError> {
        let name = || p.name.as_deref().ok_or_else(|| McpError::invalid_params("name required", None));

        match p.action {
            LabelAction::List => {
                let result = tools::labels::list(&self.client, &p.owner, &p.repo, p.limit)
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                let text = self.compress_and_format(result, p.fields, p.format);
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            LabelAction::Get => {
                let result = tools::labels::get(&self.client, &p.owner, &p.repo, name()?)
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                let text = self.compress_and_format(result, p.fields, p.format);
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            LabelAction::Create => {
                let result = tools::labels::create(&self.client, &p.owner, &p.repo, name()?, p.color.as_deref(), p.description.as_deref())
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                let text = self.compress_and_format(result, p.fields, p.format);
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            LabelAction::Update => {
                let result = tools::labels::update(&self.client, &p.owner, &p.repo, name()?, p.new_name.as_deref(), p.color.as_deref(), p.description.as_deref())
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                let text = self.compress_and_format(result, p.fields, p.format);
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            LabelAction::Delete => {
                let result = tools::labels::delete(&self.client, &p.owner, &p.repo, name()?)
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                let text = self.compress_and_format(result, p.fields, p.format);
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
        }
    }

    #[rmcp::tool(
        name = "github_actions",
        description = "GitHub Actions: list_runs, get_run, rerun, list_workflows, run_logs. action=list_runs: owner,repo,limit?. action=get_run: owner,repo,run_id. action=rerun: owner,repo,run_id. action=list_workflows: owner,repo. action=run_logs: owner,repo,run_id."
    )]
    async fn github_actions(&self, Parameters(p): Parameters<ActionsParams>) -> Result<CallToolResult, McpError> {
        let run_id = || p.run_id.ok_or_else(|| McpError::invalid_params("run_id required", None));

        match p.action {
            ActionAction::ListRuns => {
                let result = tools::actions::list_runs(&self.client, &p.owner, &p.repo, p.limit)
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                let text = self.compress_and_format(result, p.fields, p.format);
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            ActionAction::GetRun => {
                let result = tools::actions::get_run(&self.client, &p.owner, &p.repo, run_id()?)
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                let text = self.compress_and_format(result, p.fields, p.format);
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            ActionAction::Rerun => {
                let result = tools::actions::rerun(&self.client, &p.owner, &p.repo, run_id()?)
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                let text = self.compress_and_format(result, p.fields, p.format);
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            ActionAction::ListWorkflows => {
                let result = tools::actions::list_workflows(&self.client, &p.owner, &p.repo, p.limit)
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                let text = self.compress_and_format(result, p.fields, p.format);
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            ActionAction::RunLogs => {
                let text = tools::actions::run_logs(&self.client, &p.owner, &p.repo, run_id()?)
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
        }
    }

    #[rmcp::tool(
        name = "github_code_search",
        description = "Search code across all GitHub repositories. query: GitHub search syntax. limit?: max results. fields?: field projection. format?: output format (json|table|text)."
    )]
    async fn github_code_search(&self, Parameters(p): Parameters<CodeSearchParams>) -> Result<CallToolResult, McpError> {
        let result = tools::code_search::search(&self.client, &p.query, p.limit)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        let text = self.compress_and_format_search(result, p.fields, p.format);
        Ok(CallToolResult::success(vec![Content::text(text)]))
    }
}

#[rmcp::tool_handler]
impl ServerHandler for KpGithubServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            server_info: rmcp::model::Implementation {
                name: "kp-github-mcp".into(),
                title: Some("Token-Compressed GitHub MCP Server".into()),
                version: env!("CARGO_PKG_VERSION").into(),
                description: Some("13-tool GitHub MCP server with 10-40x token compression via field projection, smart formatting, and 5-stage compression pipeline. Reqwest HTTP primary, gh CLI fallback.".into()),
                icons: None,
                website_url: None,
            },
            capabilities: rmcp::model::ServerCapabilities {
                tools: Some(rmcp::model::ToolsCapability { list_changed: Some(true) }),
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
        use rmcp::model::RawResourceTemplate;
        use rmcp::model::Annotated;

        let templates = vec![
            Annotated::new(
                RawResourceTemplate {
                    uri_template: "github://repos/{owner}/{repo}/issues".into(),
                    name: "repo_issues".into(),
                    title: Some("Repository Issues".into()),
                    description: Some("List open issues for a repository (compressed)".into()),
                    mime_type: Some("text/plain".into()),
                    icons: None,
                },
                None,
            ),
            Annotated::new(
                RawResourceTemplate {
                    uri_template: "github://repos/{owner}/{repo}/pulls".into(),
                    name: "repo_pulls".into(),
                    title: Some("Repository Pull Requests".into()),
                    description: Some("List open pull requests for a repository (compressed)".into()),
                    mime_type: Some("text/plain".into()),
                    icons: None,
                },
                None,
            ),
            Annotated::new(
                RawResourceTemplate {
                    uri_template: "github://repos/{owner}/{repo}/readme".into(),
                    name: "repo_readme".into(),
                    title: Some("Repository README".into()),
                    description: Some("Get the README content for a repository".into()),
                    mime_type: Some("text/markdown".into()),
                    icons: None,
                },
                None,
            ),
            Annotated::new(
                RawResourceTemplate {
                    uri_template: "github://repos/{owner}/{repo}".into(),
                    name: "repo_info".into(),
                    title: Some("Repository Info".into()),
                    description: Some("Get repository metadata (compressed)".into()),
                    mime_type: Some("text/plain".into()),
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

            // Parse github://repos/{owner}/{repo}[/suffix]
            let path = uri
                .strip_prefix("github://repos/")
                .ok_or_else(|| McpError::invalid_params(format!("unsupported URI scheme: {uri}"), None))?;

            let parts: Vec<&str> = path.splitn(3, '/').collect();
            if parts.len() < 2 {
                return Err(McpError::invalid_params(
                    format!("URI must include owner and repo: {uri}"),
                    None,
                ));
            }

            let owner = parts[0];
            let repo = parts[1];
            let suffix = parts.get(2).copied().unwrap_or("");

            let (text, mime) = match suffix {
                "issues" => {
                    let result = tools::issues::list(&self.client, owner, repo, Some("open"), Some(30))
                        .await
                        .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                    let output = self.compress_and_format(result, None, None);
                    (output, "text/plain")
                }
                "pulls" => {
                    let result = tools::prs::list(&self.client, owner, repo, Some("open"), Some(30))
                        .await
                        .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                    let output = self.compress_and_format(result, None, None);
                    (output, "text/plain")
                }
                "readme" => {
                    let result = tools::files::get_contents(&self.client, owner, repo, "README.md", None)
                        .await
                        .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                    // The GitHub contents API returns base64-encoded content;
                    // decode it so the resource is human-readable.
                    let text = if let Some(encoded) = result.get("content").and_then(|v| v.as_str()) {
                        let cleaned: String = encoded.chars().filter(|c| !c.is_whitespace()).collect();
                        use base64::Engine;
                        match base64::engine::general_purpose::STANDARD.decode(&cleaned) {
                            Ok(bytes) => match String::from_utf8(bytes) {
                                Ok(s) => s,
                                Err(_) => "[binary content — cannot display as text]".to_string(),
                            },
                            Err(e) => format!("[base64 decode error: {e}]"),
                        }
                    } else {
                        serde_json::to_string_pretty(&result).unwrap_or_default()
                    };
                    // Cap the resource payload — a 1 MB monorepo README will
                    // otherwise exhaust MCP token budgets.
                    let text = crate::util::truncate_to_bytes(&text, 64 * 1024);
                    (text, "text/markdown")
                }
                "" => {
                    let result = tools::repos::get(&self.client, owner, repo)
                        .await
                        .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                    let output = self.compress_and_format(result, None, None);
                    (output, "text/plain")
                }
                other => {
                    return Err(McpError::invalid_params(
                        format!("unknown resource path suffix: {other}"),
                        None,
                    ));
                }
            };

            Ok(ReadResourceResult {
                contents: vec![ResourceContents::TextResourceContents {
                    uri: request.uri,
                    mime_type: Some(mime.into()),
                    text,
                    meta: None,
                }],
            })
        }
    }
}

pub async fn run(token: String) -> anyhow::Result<()> {
    let server = KpGithubServer::new(&token);
    let service = server.serve(rmcp::transport::io::stdio()).await?;
    service.waiting().await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::github::client::MockGithubClient;

    fn make_server() -> KpGithubServer {
        KpGithubServer::with_client(MockGithubClient::new())
    }

    #[tokio::test]
    async fn test_issues_list() {
        let server = make_server();
        let result = server.github_issues(Parameters(IssuesParams {
            action: IssueAction::List,
            owner: "owner".into(),
            repo: Some("repo".into()),
            state: Some("open".into()),
            limit: Some(5),
            ..Default::default()
        })).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_issues_get() {
        let server = make_server();
        let result = server.github_issues(Parameters(IssuesParams {
            action: IssueAction::Get,
            owner: "owner".into(),
            repo: Some("repo".into()),
            number: Some(1),
            ..Default::default()
        })).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_issues_search() {
        let server = make_server();
        let result = server.github_issues(Parameters(IssuesParams {
            action: IssueAction::Search,
            owner: "owner".into(),
            query: Some("bug".into()),
            limit: Some(10),
            ..Default::default()
        })).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_issues_missing_repo_returns_error() {
        let server = make_server();
        let result = server.github_issues(Parameters(IssuesParams {
            action: IssueAction::List,
            owner: "owner".into(),
            ..Default::default()
        })).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_prs_list() {
        let server = make_server();
        let result = server.github_prs(Parameters(PrsParams {
            action: PrAction::List,
            owner: "owner".into(),
            repo: "repo".into(),
            state: Some("open".into()),
            limit: Some(5),
            ..Default::default()
        })).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_prs_diff() {
        let server = make_server();
        let result = server.github_prs(Parameters(PrsParams {
            action: PrAction::Diff,
            owner: "owner".into(),
            repo: "repo".into(),
            number: Some(1),
            ..Default::default()
        })).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_prs_create() {
        let server = make_server();
        let result = server.github_prs(Parameters(PrsParams {
            action: PrAction::Create,
            owner: "owner".into(),
            repo: "repo".into(),
            title: Some("My PR".into()),
            head: Some("feature".into()),
            base: Some("main".into()),
            ..Default::default()
        })).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_files_get() {
        let server = make_server();
        let result = server.github_files(Parameters(FilesParams {
            action: FileAction::Get,
            owner: "owner".into(),
            repo: "repo".into(),
            path: Some("README.md".into()),
            ..Default::default()
        })).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_files_create_or_update() {
        let server = make_server();
        let result = server.github_files(Parameters(FilesParams {
            action: FileAction::CreateOrUpdate,
            owner: "owner".into(),
            repo: "repo".into(),
            path: Some("test.txt".into()),
            content: Some("aGVsbG8=".into()),
            message: Some("add test.txt".into()),
            branch: Some("main".into()),
            ..Default::default()
        })).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_repos_get() {
        let server = make_server();
        let result = server.github_repos(Parameters(ReposParams {
            action: RepoAction::Get,
            owner: Some("owner".into()),
            repo: Some("repo".into()),
            ..Default::default()
        })).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_repos_compare() {
        let server = make_server();
        let result = server.github_repos(Parameters(ReposParams {
            action: RepoAction::Compare,
            owner: Some("owner".into()),
            repo: Some("repo".into()),
            base: Some("main".into()),
            head: Some("feature".into()),
            include_patches: true,
            ..Default::default()
        })).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_branches_list() {
        let server = make_server();
        let result = server.github_branches(Parameters(BranchesParams {
            action: BranchAction::List,
            owner: "owner".into(),
            repo: "repo".into(),
            limit: Some(10),
            ..Default::default()
        })).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_branches_create() {
        let server = make_server();
        let result = server.github_branches(Parameters(BranchesParams {
            action: BranchAction::Create,
            owner: "owner".into(),
            repo: "repo".into(),
            branch: Some("feat/new".into()),
            from_sha: Some("abc123".into()),
            ..Default::default()
        })).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_commits_list() {
        let server = make_server();
        let result = server.github_commits(Parameters(CommitsParams {
            action: CommitAction::List,
            owner: "owner".into(),
            repo: "repo".into(),
            limit: Some(10),
            ..Default::default()
        })).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_commits_get_with_patches() {
        let server = make_server();
        let result = server.github_commits(Parameters(CommitsParams {
            action: CommitAction::Get,
            owner: "owner".into(),
            repo: "repo".into(),
            sha: Some("abc123def456".into()),
            include_patches: true,
            ..Default::default()
        })).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_releases_list() {
        let server = make_server();
        let result = server.github_releases(Parameters(ReleasesParams {
            action: ReleaseAction::List,
            owner: "owner".into(),
            repo: "repo".into(),
            limit: Some(5),
            ..Default::default()
        })).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_releases_get_latest() {
        let server = make_server();
        let result = server.github_releases(Parameters(ReleasesParams {
            action: ReleaseAction::GetLatest,
            owner: "owner".into(),
            repo: "repo".into(),
            ..Default::default()
        })).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_tags_list() {
        let server = make_server();
        let result = server.github_tags(Parameters(TagsParams {
            action: TagAction::List,
            owner: "owner".into(),
            repo: "repo".into(),
            limit: Some(10),
            ..Default::default()
        })).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_teams_list() {
        let server = make_server();
        let result = server.github_teams(Parameters(TeamsParams {
            action: TeamAction::List,
            limit: Some(10),
            ..Default::default()
        })).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_teams_members() {
        let server = make_server();
        let result = server.github_teams(Parameters(TeamsParams {
            action: TeamAction::Members,
            org: Some("myorg".into()),
            team_slug: Some("backend".into()),
            ..Default::default()
        })).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_user_me() {
        let server = make_server();
        let result = server.github_user(Parameters(UserParams {
            action: UserAction::Me,
            ..Default::default()
        })).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_user_search() {
        let server = make_server();
        let result = server.github_user(Parameters(UserParams {
            action: UserAction::Search,
            query: Some("jw".into()),
            limit: Some(5),
            ..Default::default()
        })).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_labels_list() {
        let server = make_server();
        let result = server.github_labels(Parameters(LabelsParams {
            action: LabelAction::List,
            owner: "owner".into(),
            repo: "repo".into(),
            limit: Some(10),
            ..Default::default()
        })).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_labels_get() {
        let server = make_server();
        let result = server.github_labels(Parameters(LabelsParams {
            action: LabelAction::Get,
            owner: "owner".into(),
            repo: "repo".into(),
            name: Some("bug".into()),
            ..Default::default()
        })).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_labels_create() {
        let server = make_server();
        let result = server.github_labels(Parameters(LabelsParams {
            action: LabelAction::Create,
            owner: "owner".into(),
            repo: "repo".into(),
            name: Some("new-label".into()),
            color: Some("ff0000".into()),
            ..Default::default()
        })).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_labels_update() {
        let server = make_server();
        let result = server.github_labels(Parameters(LabelsParams {
            action: LabelAction::Update,
            owner: "owner".into(),
            repo: "repo".into(),
            name: Some("bug".into()),
            new_name: Some("confirmed-bug".into()),
            ..Default::default()
        })).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_labels_delete() {
        let server = make_server();
        let result = server.github_labels(Parameters(LabelsParams {
            action: LabelAction::Delete,
            owner: "owner".into(),
            repo: "repo".into(),
            name: Some("old-label".into()),
            ..Default::default()
        })).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_actions_list_runs() {
        let server = make_server();
        let result = server.github_actions(Parameters(ActionsParams {
            action: ActionAction::ListRuns,
            owner: "owner".into(),
            repo: "repo".into(),
            limit: Some(5),
            ..Default::default()
        })).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_actions_run_logs() {
        let server = make_server();
        let result = server.github_actions(Parameters(ActionsParams {
            action: ActionAction::RunLogs,
            owner: "owner".into(),
            repo: "repo".into(),
            run_id: Some(12345678),
            ..Default::default()
        })).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_code_search() {
        let server = make_server();
        let result = server.github_code_search(Parameters(CodeSearchParams {
            query: "fn main language:rust".into(),
            limit: Some(5),
            ..Default::default()
        })).await;
        assert!(result.is_ok());
    }
}
