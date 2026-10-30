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
  and launched automatically via `uvx idfkit-mcp`. Install it with:

  ```sh
  curl -LsSf https://astral.sh/uv/install.sh | sh
  ```

  Verify: `which uvx` (typically `~/.local/bin/uvx`)
- (Optional, for simulations) a local
  [EnergyPlus](https://energyplus.net/downloads) installation, discovered
  via `PATH` or `ENERGYPLUS_DIR`

## Installation

### 1. Install the extension

Once published, install from the Zed extension marketplace
(Extensions → search "idfkit"). Until then, install it as a dev
extension:

```sh
git clone https://github.com/bbrangeo/zed-idfkit-mcp
```

then in Zed: **Extensions → Install Dev Extension** and select the
cloned directory.

### 2. Install uv (one-time, required)

The extension launches the server via `uvx` (bundled with
[uv](https://docs.astral.sh/uv/)). If you don't have it:

```sh
curl -LsSf https://astral.sh/uv/install.sh | sh
```

No other setup is needed — the first server start downloads
`idfkit-mcp` from PyPI automatically.

### 3. Verify

Open the Agent Panel in Zed. The `idfkit-mcp` context server should
appear and the agent should be able to call its tools (e.g. ask it to
"list the zones of my model"). If it stays on "Connecting…", see
Troubleshooting below.

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

**Stuck on "Connecting…" (MCP initialize times out after 60 s)** —
make sure [uv](https://docs.astral.sh/uv/) is installed and reachable:
`which uvx`. The first start downloads `idfkit-mcp` from PyPI, which
can take a few seconds on slow connections. The extension sets
`UV_NO_PROGRESS=1` so uv's progress bar doesn't interfere with the
MCP handshake.

**"uvx not found"** — install uv (step 2 above) or set `uvx_path`
(Advanced override) to the absolute path of the `uvx` binary.

**Simulations fail** — the agent needs a local
[EnergyPlus](https://energyplus.net/downloads) install. Either put
`energyplus` on your `PATH` or point the agent to it via
`ENERGYPLUS_DIR`.

## Resources

- idfkit-mcp repository: <https://github.com/idfkit/idfkit-mcp>
- idfkit MCP documentation: <https://mcp.idfkit.com/docs/>
- uv documentation: <https://docs.astral.sh/uv/>
- Sibling LSP extension (syntax highlighting + language server for
  `.idf`): <https://github.com/bbrangeo/zed-idfkit>

## License

MIT — see [LICENSE](LICENSE).
