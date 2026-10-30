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

None required — the extension is fully self-contained: it ships a
built-in launcher that resolves `uvx` independently of Zed's `PATH`,
so the server starts as soon as [uv](https://docs.astral.sh/uv/) is
installed.

## Advanced override (optional)

In the rare case where `uvx` lives in a non-standard location, set
`uvx_path`:

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
60 s), make sure [uv](https://docs.astral.sh/uv/) is installed and on
your `PATH` (`which uvx`). The extension sets `UV_NO_PROGRESS=1` so
uv's first-download progress bar doesn't interfere with the MCP
handshake.

## Resources

- idfkit-mcp repository: <https://github.com/idfkit/idfkit-mcp>
- idfkit MCP documentation: <https://mcp.idfkit.com/docs/>
- uv documentation: <https://docs.astral.sh/uv/>

## License

MIT — see [LICENSE](LICENSE).
