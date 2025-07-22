use zed::{LanguageServerId, Result};
use zed_extension_api as zed;

struct PyreflyExtension;

impl zed::Extension for PyreflyExtension {
    fn new() -> Self {
        PyreflyExtension
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::process::Command> {
        if language_server_id.as_ref() == "pyrefly" {
            Ok(zed::process::Command {
                command: "uv".to_string(),
                args: vec!["run".to_string(), "pyrefly".to_string(), "lsp".to_string()],
                env: worktree.shell_env(),
            })
        } else {
            Err("Unknown language server".to_string())
        }
    }
}

// Registrar la extensión
zed::register_extension!(PyreflyExtension);
