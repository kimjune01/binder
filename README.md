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

The boundary is deliberate: **Binder Core** is the portable epistemic CLI and
receipt format; a later **Binder Agreements** interface may help parties shop,
parameterize, independently review, export, and sign an exact contract package.
Binder does not need to custody funds, choose terms, underwrite risk, adjudicate
outcomes, or participate at runtime.

Binder is being developed as public infrastructure. Adjacent work should link
out to existing wallets, explorers, auditors, verifiers, data providers,
deployment tools, and professional services through open handoffs. Completing a
workflow elsewhere is success; value capture can be considered after useful,
repeated use exists.

See [OUTREACH.md](OUTREACH.md) for the soft-discovery and demo conversations
used to test the product with contract maintainers and auditors.
The [five-case public research hub](hub/README.md) organizes real audit,
verification, proof-scope, and postmortem artifacts into source-traceable case
fixtures with explicit missing edges.

## Review a pull request

The repository includes a local Codex skill for turning a GitHub PR into one
source-grounded contribution. Ask Codex to use `$binder-pr-review` with a PR URL.
It reconstructs the claim, looks for a counterexample or claim/test mismatch,
and drafts a concise comment with a distinguishing test. It reports when no
useful comment is warranted and never posts without explicit approval.

Public contributions and their observed outcomes are recorded in the
[contribution worklog](WORKLOG.md).

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

Binder's [case checklist](CHECKLIST.md) documents how the chronology becomes a
current projection and policy-relative recommendations. Binder-authored content
is licensed under [CC BY-SA-NS](cc-by-sa-ns.md), which adds a corresponding-source
requirement for network-service derivatives.
