#!/usr/bin/env bash
set -euo pipefail
repo_root="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$repo_root"
cargo run --quiet -p binder-cli -- verify demo/contract/claim.yaml \
  --base 78d7031c48b7b98af74055dead4002c8dbf8941c \
  --head HEAD \
  --format json
