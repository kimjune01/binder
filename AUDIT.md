# Demo completion audit

Audited against the twelve acceptance tests in `DEMO.md`. The authoritative reproduction command is:

```sh
bash scripts/replay.sh
```

## Acceptance criteria

| # | Result | Evidence |
|---|---|---|
| 1 | Satisfied | `crates/vault-transition/tests/invariant.rs::vulnerable_revision_has_a_counterexample` |
| 2 | Satisfied within the stated bounded-exhaustive fallback | `fixed_revision_preserves_balances_on_every_explored_error_path` |
| 3 | Satisfied | `crates/vault-mollusk/tests/runtime.rs::vulnerable_account_metas_allow_the_unauthorized_transfer` |
| 4 | Satisfied | `fixed_account_metas_reject_and_rollback_the_unauthorized_transfer` |
| 5 | Satisfied | `crates/binder-core/tests/warrant.rs::warrants_head_only_when_base_fails_and_every_required_trial_passes` and `fails_closed_when_required_evidence_is_missing_or_failed` |
| 6 | Satisfied | Receipt base evidence uses `expected-fail`; CLI renders `Base FAIL (expected; bug reproduced)` |
| 7 | Satisfied | All source, specification, fixture, toolchain, and runtime build inputs are declared in the claim; dependency and runtime-artifact mutation tests produce `STALE` |
| 8 | Satisfied | `dependency_change_makes_evidence_stale_but_unrelated_change_does_not` |
| 9 | Satisfied | Receipt validation rejects mismatched snapshots, missing or duplicate trials, malformed digests, status/evidence disagreement, changed artifacts, and a noncanonical latest receipt |
| 10 | Satisfied | `snapshot_identity_is_deterministic_and_path_sensitive`; `scripts/replay.sh` compares two complete serialized receipts byte-for-byte |
| 11 | Satisfied | `failed_evidence_reports_its_observed_predicate` ensures a failed row includes the first normalized trial observation |
| 12 | Satisfied | `crates/binder-cli/tests/replay_bundle.rs` checks packaged declared inputs, raw stdout/stderr, and replay instructions; Cargo and sBPF inputs are externally pinned by `Cargo.lock`, `rust-toolchain.toml`, and `scripts/replay.sh` |

## Milestone status

- M0–M2: complete. Both evidence boundaries execute against vulnerable and fixed revisions and feed one receipt.
- M3: complete. The terminal report is compact, status is freshness-aware, and the content-addressed bundle contains replay material and raw evidence.
- M4: intentionally not started. Deployed identity is optional and outside the local demonstrator.

## Honest limitations

- The transition evidence is bounded exhaustive testing, not Flux or an unbounded proof. The report and documentation do not call it stronger than it is.
- Artifact identity covers the locally built sBPF programs. It does not claim identity with a deployed program.
- Timestamps and host diagnostics are not placed in the deterministic receipt. Raw output is retained for diagnosis; adding nondeterministic run metadata remains a future schema extension.
