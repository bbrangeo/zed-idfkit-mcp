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

No configuration is required: the extension starts the server the first
time you use it from the Agent Panel, through a built-in launcher that
resolves `uvx` independently of Zed's `PATH`.

## Advanced override (optional)

In the rare case where `uvx` lives in a non-standard location, you can
point the extension at it explicitly with the `uvx_path` setting:

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
