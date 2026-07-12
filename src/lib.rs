use zed_extension_api as zed;

struct PhpstanExtension;

impl zed::Extension for PhpstanExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        let command = worktree
            .which("phpstan-language-server")
            .ok_or_else(|| {
                "phpstan-language-server was not found on PATH. Install it with `cargo install --path .` from https://github.com/laraochan/phpstan-language-server, or add its binary directory to PATH.".to_string()
            })?;

        Ok(zed::Command {
            command,
            args: Vec::new(),
            env: worktree.shell_env(),
        })
    }
}

zed::register_extension!(PhpstanExtension);
