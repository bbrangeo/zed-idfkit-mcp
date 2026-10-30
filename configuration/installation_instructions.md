# idfkit MCP Server — Installation

The idfkit MCP server is distributed on PyPI and launched with
[`uvx`](https://docs.astral.sh/uv/), which downloads and runs it
automatically — there is nothing to install manually.

## Prerequisites

1. **uv** — install it with:

   ```sh
   curl -LsSf https://astral.sh/uv/install.sh | sh
   ```

   See the [uv documentation](https://docs.astral.sh/uv/) for other
   installation methods.

2. **EnergyPlus** (for running simulations) — install a local copy of
   [EnergyPlus](https://energyplus.net/downloads). The server discovers
   it automatically via your `PATH` or the `ENERGYPLUS_DIR`
   environment variable.

## Usage

Once Zed has `uv` on its `PATH`, the extension starts the server with
`uvx idfkit-mcp` the first time you use it from the Agent Panel. You can
point the extension at a custom `uvx` binary with the `uvx_path` setting.

## Troubleshooting

If the server stays on **"Connecting…"** in the Agent Panel (the MCP
initialize handshake times out after 60 s), Zed most likely spawned
`uvx` with a GUI-minimal `PATH` (`/usr/bin:/bin:/usr/sbin:/sbin`) that
does not contain it — context servers are not launched through a login
shell (see [zed-industries/zed #60800](https://github.com/zed-industries/zed/issues/60800)).

Fix: give the extension the **absolute path** of the binary (or of a
wrapper script):

```sh
# in a terminal, find the absolute path:
which uvx        # e.g. /opt/homebrew/bin/uvx
```

then in Zed settings:

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

Alternatively, point `uvx_path` at a small `sh` wrapper that sets up
`PATH` itself and then `exec`s `uvx "$@"`. The extension also sets
`UV_NO_PROGRESS=1` so uv's first-download progress bar does not
interfere with the MCP handshake.
