#!/bin/sh
# Universal launcher for the idfkit MCP server, spawned by Zed's context-server host.
#
# Zed spawns context-server commands through a non-login shell with a
# GUI-minimal PATH (zed-industries/zed #60800) and the project root as the
# working directory. A bare "uvx" therefore fails to resolve, and uv may
# resolve against the project's pyproject.toml instead of the user's
# environment. This launcher:
#   1. pins UV_NO_PROGRESS=1 so uv's first-download progress bar cannot
#      interfere with the MCP stdio handshake;
#   2. neutralizes the working directory by cd-ing to $HOME;
#   3. resolves uvx through a fallback chain: the user's IDFKIT_UVX_PATH
#      override (passed by the extension from its `uvx_path` setting), then
#      PATH, then the standard Homebrew locations (Apple Silicon and
#      Intel), then ~/.local/bin (the default `uv installer` location);
#   4. execs the resolved uvx with the server package name passed by Zed.
export UV_NO_PROGRESS=1
cd "${HOME:-/tmp}" || exit 1
for candidate in \
    "${IDFKIT_UVX_PATH:-}" \
    "$(command -v uvx 2>/dev/null)" \
    /opt/homebrew/bin/uvx \
    /usr/local/bin/uvx \
    "${HOME}/.local/bin/uvx"
do
    [ -n "$candidate" ] && [ -x "$candidate" ] && exec "$candidate" "$@"
done
echo "idfkit-mcp: uvx binary not found (install uv: https://docs.astral.sh/uv/)" >&2
exit 127
