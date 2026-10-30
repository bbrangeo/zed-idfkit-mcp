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
