use zed_extension_api as zed;
use std::path::PathBuf;

struct GemmaZed;

impl zed::Extension for GemmaZed {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        context_server_id: &zed::ContextServerId,
        worktree: Option<&zed::Worktree>,
    ) -> zed::Result<zed::Command> {
        let settings = worktree
            .and_then(|w| {
                w.settings()
                    .context_servers
                    .get(context_server_id)
                    .cloned()
            })
            .unwrap_or_default();

        let port = settings
            .get("port")
            .and_then(|v| v.as_str())
            .unwrap_or("1234");

        let model = settings
            .get("model")
            .and_then(|v| v.as_str())
            .unwrap_or("gemma-4-e4b");

        let bridge_path = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.to_path_buf()))
            .unwrap_or_else(|| PathBuf::from("."))
            .join("gemma-zed-bridge");

        let mut env = std::collections::HashMap::new();
        env.insert("LM_STUDIO_PORT".to_string(), port.to_string());
        env.insert("LM_MODEL".to_string(), model.to_string());

        Ok(zed::Command {
            command: bridge_path,
            args: vec![],
            env,
        })
    }
}

zed::register_extension!(GemmaZed);