use axum::http::request;
use rmcp::{model::{CallToolResult, Content, GetPromptRequestParam, GetPromptResult, ListPromptsResult, PaginatedRequestParam}, tool};
use anyhow::Result;
use rmcp::{ErrorData, RoleServer, ServerHandler, handler::server::{router::prompt::PromptRouter, tool::ToolRouter}, model::{Implementation, InitializeRequestParam, InitializeResult, ProtocolVersion, ServerCapabilities, ServerInfo}, prompt_handler, prompt_router, service::RequestContext, tool_handler, tool_router};
use tracing::info;

#[derive(Clone)]
pub struct AternityMCPFactory {
    tool_router: ToolRouter<AternityMCPFactory>,
    prompt_router: PromptRouter<AternityMCPFactory>,
}

#[tool_router]
impl AternityMCPFactory {
    pub fn new() -> Self {
        AternityMCPFactory {
            tool_router: Self::tool_router(),
            prompt_router: Self::prompt_router(),
        }
    }

    #[tool(description = "Get all Aternity Remediations")]
    async fn get_all_remediations(&self) -> Result<CallToolResult, ErrorData> {
        Ok(CallToolResult::success(vec![Content::text("Remediation1".to_string())]))
    }
}

#[prompt_router]
impl AternityMCPFactory {
    
}

#[tool_handler]
#[prompt_handler]
impl ServerHandler for AternityMCPFactory {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::LATEST,
            capabilities: ServerCapabilities::builder().enable_prompts().enable_tools().build(),
            server_info: Implementation::from_build_env(),
            instructions: Some("Aternity MCP Server with tools and prompts".to_string()),
        }
    }
    async fn initialize(&self, _request: InitializeRequestParam, context: RequestContext<RoleServer>) -> Result<InitializeResult, ErrorData> {
        if let Some(http_request_part) = context.extensions.get::<request::Parts>() {
            let initialize_headers = &http_request_part.headers;
            let initialize_uri = &http_request_part.uri;
            info!(?initialize_headers, %initialize_uri, "initialize from http server");
        }
        Ok(self.get_info())
    }
}