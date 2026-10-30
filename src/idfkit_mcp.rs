use zed_extension_api as zed;

use serde::Deserialize;
use zed_extension_api::{ContextServerConfiguration, ContextServerId, Project};

/// User-configurable settings for the idfkit MCP context server.
///
/// Zed serializes the `[context_servers.idfkit-mcp]` settings block and
/// hands it back to us through `ContextServerSettings::for_project`.
#[derive(Deserialize, Default, schemars::JsonSchema)]
struct IdfkitMcpExtensionSettings {
    /// Custom path to the `uvx` binary. Defaults to `"uvx"`.
    ///
    /// A bare name relies on the `PATH` Zed gives to spawned context
    /// servers, which is a GUI-minimal `PATH` (`/usr/bin:/bin:...`) —
    /// see zed-industries/zed #60800. If `uvx` is not on that `PATH`
    /// (e.g. Homebrew installs), the process dies at spawn and the
    /// server never connects (initialize times out after 60 s). Prefer
    /// an absolute path such as `/opt/homebrew/bin/uvx`, or a wrapper
    /// script that sets up the environment itself.
    uvx_path: Option<String>,
}

const CONTEXT_SERVER_ID: &str = "idfkit-mcp";

const INSTALLATION_INSTRUCTIONS: &str =
    include_str!("../configuration/installation_instructions.md");
const DEFAULT_SETTINGS: &str = include_str!("../configuration/default_settings.jsonc");

struct IdfkitMcpExtension;

impl IdfkitMcpExtension {
    /// Resolves the command launching the idfkit MCP server.
    ///
    /// The server is a PyPI package (`idfkit-mcp`) executed through
    /// `uvx`, which downloads/updates it automatically. We return the
    /// bare binary name (or the user's `uvx_path` override): Zed's host
    /// process spawns context-server commands with `Shell::System` +
    /// `non_interactive()`, i.e. a GUI-minimal `PATH` (`/usr/bin:/bin:...`)
    /// and no login shell — see zed-industries/zed #60800. If `uvx` is
    /// not on that `PATH`, the process dies at spawn (broken pipe →
    /// initialize timeout after 60 s); users should set `uvx_path` to an
    /// absolute path (e.g. `/opt/homebrew/bin/uvx`) or a wrapper script.
    /// No `std::fs` access is needed (and none is possible from the WASM
    /// sandbox), so we cannot resolve the path ourselves.
    ///
    /// We also pass `UV_NO_PROGRESS=1`: `uv` writes its download progress
    /// bar to stderr, which can interfere with the MCP handshake on the
    /// first (cache-cold) run.
    fn resolve_command(&self, project: &Project) -> zed::Result<zed::Command> {
        let settings =
            zed::settings::ContextServerSettings::for_project(CONTEXT_SERVER_ID, project)?;
        let settings: IdfkitMcpExtensionSettings =
            serde_json::from_value(settings.settings.unwrap_or_default()).map_err(|err| {
                format!("invalid settings for context server `{CONTEXT_SERVER_ID}`: {err}")
            })?;

        let command = settings.uvx_path.unwrap_or_else(|| "uvx".to_string());

        Ok(zed::Command {
            command,
            args: vec!["idfkit-mcp".to_string()],
            env: vec![("UV_NO_PROGRESS".to_string(), "1".to_string())],
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
