use std::sync::Arc;

use forge_app::{EnvironmentService, Infrastructure};
use forge_domain::{
    AgentMessage, App, ChatRequest, ChatResponse, Orchestrator, SystemContext, ToolService,
};
use forge_stream::MpscStream;
use forge_walker::Walker;

pub struct ForgeExecutorService<F> {
    app: Arc<F>,
}
impl<F: Infrastructure + App> ForgeExecutorService<F> {
    pub fn new(app: Arc<F>) -> Self {
        Self { app }
    }
}

impl<F: Infrastructure + App> ForgeExecutorService<F> {
    pub async fn chat(
        &self,
        request: ChatRequest,
    ) -> anyhow::Result<MpscStream<anyhow::Result<AgentMessage<ChatResponse>>>> {
        let env = self.app.environment_service().get_environment();
        let mut files = Walker::max_all()
            .max_depth(2)
            .cwd(env.cwd.clone())
            .get()
            .await?
            .iter()
            .map(|f| f.path.to_string())
            .collect::<Vec<_>>();

        // Sort the files alphabetically to ensure consistent ordering
        files.sort();

        let ctx = SystemContext {
            env: Some(env),
            tool_information: Some(self.app.tool_service().usage_prompt()),
            tool_supported: Some(true),
            files,
        };

        let app = self.app.clone();

        Ok(MpscStream::spawn(move |tx| async move {
            let tx = Arc::new(tx);
            let orch = Orchestrator::new(app, request, ctx, Some(tx.clone()));
            match orch.execute().await {
                Ok(_) => {}
                Err(err) => tx.send(Err(err)).await.unwrap(),
            }
        }))
    }
}
