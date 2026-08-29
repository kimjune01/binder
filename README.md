# Binder demo

Binder lets humans state what observations would warrant a software claim, then materializes the evidence graph for an exact change. The smallest walkthrough is the [escrow contract example](demo/contract/README.md):

```sh
bash demo/contract/run.sh
```

It preserves an attributed base/refutation → candidate/standing rule, runs the same empirical check against both Git revisions, captures concrete balance witnesses, and emits an agent-readable receipt.

The original, deeper demonstrator compares a deliberately broken Solana vault revision with its fixed revision across a pure Rust transition check and a Mollusk runtime replay.

See [PRODUCT.md](PRODUCT.md) for the canonical product framing and scope, and
[VISION.md](VISION.md) for the longer-term vision of independently verified
agreements.

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

For CI and other tools, append `--format json` to either command. JSON is the only content written to stdout in this mode; trial diagnostics remain on stderr. Exit code `0` means warranted, `1` means a completed non-warranted result, and `2` means a usage, configuration, or execution error.

The report includes the human-readable claim, the expected base failure, the fixed-head result, and each required evidence row. In GitHub Actions, `verify` and `status` also append the same report to the job summary through `GITHUB_STEP_SUMMARY`; no separate formatter is required.

Each verification writes a content-addressed directory under `.binder/receipts/` containing the normalized receipt, declared inputs, raw trial output, and standalone replay instructions. Generated receipts and build products are intentionally ignored by Git.

## Guarantee boundary

`WARRANTED` means every required trial distinguished the broken base from the fixed head over the exact recorded inputs, and those inputs and compiled artifacts remain current. The Rust trial is bounded exhaustive evidence, not an unbounded formal proof. The runtime trial is a local Mollusk execution, not deployed-program identity or a general audit.

See [DEMO.md](DEMO.md) for the goal and [AUDIT.md](AUDIT.md) for the acceptance-criteria audit.
See [PRODUCTION.md](PRODUCTION.md) for the staged plan from local demo to a GitHub workflow and public program-status API.
The [validation study](VALIDATION.md) is the gate for any further product build.
See [ONLINE_RESEARCH.md](ONLINE_RESEARCH.md) for the firsthand complaints and incidents motivating that study.
