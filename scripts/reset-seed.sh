#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
data="$root/.run/data"
test "$data" = "$root/.run/data"
if test -d "$data"; then mv "$data" "$data.retired.$(date +%s)"; fi
mkdir -p "$data"
echo 'Synthetic namespace reset; previous data moved to a timestamped retired directory.'
