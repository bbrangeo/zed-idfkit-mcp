# idfkit MCP Server — Zed Extension

[EnergyPlus](https://energyplus.net/) building energy simulation MCP
server for the [Zed](https://zed.dev) Agent Panel, powered by
[idfkit](https://github.com/idfkit/idfkit-mcp).

Once connected, the Zed agent can:

- **Build & edit IDF models** programmatically
- **Run EnergyPlus simulations** and collect results
- **Explore the EnergyPlus schema** (objects, fields, defaults)
- **Search EnergyPlus documentation** (Input Output Reference)

## Prerequisites

- [Zed](https://zed.dev) with Agent Panel support
- [uv](https://docs.astral.sh/uv/) — the server is downloaded from PyPI
  and launched automatically via `uvx idfkit-mcp`
- (Optional, for simulations) a local
  [EnergyPlus](https://energyplus.net/downloads) installation, discovered
  via `PATH` or `ENERGYPLUS_DIR`

## Installation

Once published, install from the Zed extension marketplace
(Extensions → search "idfkit"). Until then, you can install it as a dev
extension:

```sh
git clone https://github.com/bbrangeo/zed-idfkit-mcp
```

then in Zed: **Extensions → Install Dev Extension** and select the
cloned directory.

## Configuration

The only setting is `uvx_path`, the path to the `uvx` binary (defaults
to `"uvx"`, resolved against your `PATH`):

```jsonc
{
  "context_servers": {
    "idfkit-mcp": {
      "settings": {
        "uvx_path": "/opt/homebrew/bin/uvx"
      }
    }
  }
}
```

## Troubleshooting

If the server stays on "Connecting…" (MCP initialize times out after
60 s): Zed spawns context servers with a GUI-minimal `PATH`
(`/usr/bin:/bin:...`), no login shell — see
[zed-industries/zed #60800](https://github.com/zed-industries/zed/issues/60800).
Fix: run `which uvx` in a terminal and set `uvx_path` to the absolute
path (e.g. `/opt/homebrew/bin/uvx`) or to a `sh` wrapper that sets up
`PATH` itself. The extension sets `UV_NO_PROGRESS=1` so uv's
first-download progress bar doesn't interfere with the handshake.

## Resources

- idfkit-mcp repository: <https://github.com/idfkit/idfkit-mcp>
- idfkit MCP documentation: <https://mcp.idfkit.com/docs/>
- uv documentation: <https://docs.astral.sh/uv/>

## License

MIT — see [LICENSE](LICENSE).
