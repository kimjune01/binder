#!/usr/bin/env bash
set -euo pipefail

claim="demo/claims/failed-withdrawal-preserves-balances.yaml"
expected_sbf_builder="4.2.0"

sha256_file() {
  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$1" | awk '{print $1}'
  else
    shasum -a 256 "$1" | awk '{print $1}'
  fi
}

if ! command -v rustup >/dev/null 2>&1; then
  echo "replay requires rustup: https://rustup.rs" >&2
  exit 2
fi

rustup_bin_dir="$(dirname "$(command -v rustup)")"
export PATH="$rustup_bin_dir:$PATH"

if ! command -v cargo-build-sbf >/dev/null 2>&1 || \
  [[ "$(cargo-build-sbf --version | head -n 1 | awk '{print $2}')" != "$expected_sbf_builder" ]]; then
  cargo install cargo-build-sbf --version "$expected_sbf_builder" --locked
fi

if bash scripts/mollusk-trial.sh vulnerable; then
  echo "expected vulnerable runtime trial to fail" >&2
  exit 1
fi
bash scripts/mollusk-trial.sh fixed
cargo test --workspace --locked
cargo run --quiet --locked -p binder-cli -- verify "$claim"
first_receipt="$(sha256_file .binder/claims/failed-withdrawal-preserves-balances.yaml)"
cargo run --quiet --locked -p binder-cli -- verify "$claim"
second_receipt="$(sha256_file .binder/claims/failed-withdrawal-preserves-balances.yaml)"
test "$first_receipt" = "$second_receipt"
cargo run --quiet --locked -p binder-cli -- status "$claim"
echo "Replay complete: deterministic receipt $second_receipt"
