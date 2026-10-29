#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
for name in api ui; do if test -f "$root/.run/$name.pid"; then kill "$(cat "$root/.run/$name.pid")" 2>/dev/null || true; fi; done
