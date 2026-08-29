# Binder demo

Binder turns a small, named claim into locally replayable evidence. This demonstrator compares a deliberately broken vault revision with its fixed revision across a pure Rust transition check and a Mollusk runtime replay.

## Reproduce

Prerequisite: [rustup](https://rustup.rs/). The replay script installs the pinned `cargo-build-sbf` version when necessary, builds both runtime artifacts before testing, runs the full workspace suite, verifies the claim twice, checks deterministic receipt bytes, and checks the recorded status.

```sh
bash scripts/replay.sh
```

Success ends with `Replay complete: deterministic receipt …`. The earlier `FAIL` lines reproduce the deliberately broken base and are expected; the report must identify them as `Base FAIL (expected; bug reproduced)` and the fixed head as `PASS`.

For a quick status check after verification:

```sh
cargo run --quiet --locked -p binder-cli -- status demo/claims/failed-withdrawal-preserves-balances.yaml
```

The report includes the human-readable claim, the expected base failure, the fixed-head result, and each required evidence row. In GitHub Actions, `verify` and `status` also append the same report to the job summary through `GITHUB_STEP_SUMMARY`; no separate formatter is required.

Each verification writes a content-addressed directory under `.binder/receipts/` containing the normalized receipt, declared inputs, raw trial output, and standalone replay instructions. Generated receipts and build products are intentionally ignored by Git.

## Guarantee boundary

`WARRANTED` means every required trial distinguished the broken base from the fixed head over the exact recorded inputs, and those inputs and compiled artifacts remain current. The Rust trial is bounded exhaustive evidence, not an unbounded formal proof. The runtime trial is a local Mollusk execution, not deployed-program identity or a general audit.

See [DEMO.md](DEMO.md) for the goal and [AUDIT.md](AUDIT.md) for the acceptance-criteria audit.
