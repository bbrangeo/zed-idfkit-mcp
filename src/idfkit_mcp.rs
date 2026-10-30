use zed_extension_api as zed;

use serde::Deserialize;
use zed_extension_api::{ContextServerConfiguration, ContextServerId, Os, Project, current_platform};

/// User-configurable settings for the idfkit MCP context server.
///
/// Zed serializes the `[context_servers.idfkit-mcp]` settings block and
/// hands it back to us through `ContextServerSettings::for_project`.
#[derive(Deserialize, Default, schemars::JsonSchema)]
struct IdfkitMcpExtensionSettings {
    /// Custom path to the `uvx` binary.
    ///
    /// This is an **advanced, optional** override. The extension ships a
    /// universal launcher (embedded in the extension binary and run
    /// through `/bin/sh -c`), which already resolves `uvx` through a
    /// robust fallback chain, so no configuration is required. If set to
    /// a non-empty value, this path takes priority: the extension passes
    /// it to the launcher via the `IDFKIT_UVX_PATH` environment variable.
    uvx_path: Option<String>,
}

const CONTEXT_SERVER_ID: &str = "idfkit-mcp";

/// Universal launcher for the idfkit MCP server, embedded in the
/// extension binary and written to no disk location of our own — it is
/// passed directly to `/bin/sh -c` as the script body. See the script
/// header for what it does (PATH-independent uvx resolution, CWD
/// neutralization, progress-bar pinning).
const UVX_LAUNCHER: &str = include_str!("../configuration/uvx_launcher.sh");

const INSTALLATION_INSTRUCTIONS: &str =
    include_str!("../configuration/installation_instructions.md");
const DEFAULT_SETTINGS: &str = include_str!("../configuration/default_settings.jsonc");

struct IdfkitMcpExtension;

impl IdfkitMcpExtension {
    /// Resolves the command launching the idfkit MCP server.
    ///
    /// The server is a PyPI package (`idfkit-mcp`) executed through
    /// `uvx`, which downloads/updates it automatically. Zed's host
    /// process spawns context-server commands with `Shell::System` +
    /// `non_interactive()`, i.e. a GUI-minimal `PATH` (`/usr/bin:/bin:...`)
    /// and no login shell — see zed-industries/zed #60800 — so a bare
    /// `uvx` cannot be relied upon. Instead of depending on the host's
    /// `PATH`, we run a **universal launcher** embedded in this binary:
    ///
    /// ```text
    /// /bin/sh -c <launcher script> sh idfkit-mcp
    /// ```
    ///
    /// `sh -c 'script' name arg...` sets `$0 = name` and `$@ = arg...`,
    /// so the launcher's final `exec "$candidate" "$@"` receives the
    /// server package name. The launcher resolves `uvx` through a
    /// fallback chain (`IDFKIT_UVX_PATH` override, then `PATH`, then the
    /// standard Homebrew and `~/.local/bin` locations), neutralizes the
    /// working directory by cd-ing to `$HOME`, and pins
    /// `UV_NO_PROGRESS=1`.
    ///
    /// If the user sets the optional `uvx_path` override, it is forwarded
    /// to the launcher via the `IDFKIT_UVX_PATH` environment variable, so
    /// it takes priority over every other fallback without giving up the
    /// launcher's other guarantees.
    ///
    /// On **Windows**, the POSIX launcher is not available (`/bin/sh`
    /// does not exist): `uvx` is called directly instead — with the
    /// `uvx_path` override if set, otherwise the bare name resolved by
    /// the host's `PATH`.
    ///
    /// We also pass `UV_NO_PROGRESS=1` at the process level: `uv` writes
    /// its download progress bar to stderr, which can interfere with the
    /// MCP handshake on the first (cache-cold) run. This is redundant
    /// with the launcher's own `export` but harmless.
    fn resolve_command(&self, project: &Project) -> zed::Result<zed::Command> {
        let settings =
            zed::settings::ContextServerSettings::for_project(CONTEXT_SERVER_ID, project)?;
        let settings: IdfkitMcpExtensionSettings =
            serde_json::from_value(settings.settings.unwrap_or_default()).map_err(|err| {
                format!("invalid settings for context server `{CONTEXT_SERVER_ID}`: {err}")
            })?;

        let mut env = vec![("UV_NO_PROGRESS".to_string(), "1".to_string())];

        // Forward the optional `uvx_path` override to the launcher. An
        // empty string is treated as "not set" and dropped entirely.
        if let Some(uvx_path) = settings.uvx_path.as_deref().filter(|p| !p.is_empty()) {
            env.push(("IDFKIT_UVX_PATH".to_string(), uvx_path.to_string()));
        }

        // Windows: the POSIX launcher (`/bin/sh -c`) is not available.
        // Call `uvx` directly — Zed's host process resolves it through the
        // system PATH there, and uv ships a native `uvx.exe`. On macOS and
        // Linux, keep the universal launcher (PATH-independent uvx
        // resolution, CWD neutralization, progress-bar pinning).
        if matches!(current_platform().0, Os::Windows) {
            let command = settings
                .uvx_path
                .filter(|p| !p.trim().is_empty())
                .unwrap_or_else(|| "uvx".to_string());
            return Ok(zed::Command {
                command,
                args: vec!["idfkit-mcp".to_string()],
                env,
            });
        }

        Ok(zed::Command {
            command: "/bin/sh".to_string(),
            args: vec![
                "-c".to_string(),
                UVX_LAUNCHER.to_string(),
                // $0 for the script (used in error messages / argv[0]).
                "idfkit-mcp-launcher".to_string(),
                // Server args, forwarded to the resolved uvx via "$@".
                "idfkit-mcp".to_string(),
            ],
            env,
        })
    }
}

impl zed::Extension for IdfkitMcpExtension {
    fn new() -> Self {
        IdfkitMcpExtension
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        project: &Project,
    ) -> zed::Result<zed::Command> {
        self.resolve_command(project)
    }

    fn context_server_configuration(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> zed::Result<Option<ContextServerConfiguration>> {
        let settings_schema = serde_json::to_string(&schemars::schema_for!(
            IdfkitMcpExtensionSettings
        ))
        .map_err(|err| format!("failed to serialize settings schema: {err}"))?;

        Ok(Some(ContextServerConfiguration {
            installation_instructions: INSTALLATION_INSTRUCTIONS.to_string(),
            default_settings: DEFAULT_SETTINGS.to_string(),
            settings_schema,
        }))
    }
}

zed::register_extension!(IdfkitMcpExtension);
