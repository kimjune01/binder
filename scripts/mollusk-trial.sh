#!/usr/bin/env bash
set -euo pipefail

revision="${1:-}"
case "$revision" in
  fixed)
    feature=""
    ;;
  vulnerable)
    feature="vulnerable"
    ;;
  *)
    echo "usage: scripts/mollusk-trial.sh <vulnerable|fixed>" >&2
    exit 2
    ;;
esac

export PATH="$HOME/.cargo/bin:$PATH"
if [[ -n "$feature" ]]; then
  cargo-build-sbf \
    --manifest-path crates/vault-program/Cargo.toml \
    --sbf-out-dir "target/deploy/$revision" \
    --features "$feature"
else
  cargo-build-sbf \
    --manifest-path crates/vault-program/Cargo.toml \
    --sbf-out-dir "target/deploy/$revision"
fi
cargo run --quiet -p vault-mollusk --bin mollusk-trial -- "$revision"
