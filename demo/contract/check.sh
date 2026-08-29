#!/usr/bin/env bash
set -euo pipefail
mkdir -p target/binder-demo
rustc demo/contract/check.rs -o target/binder-demo/escrow-check
target/binder-demo/escrow-check
