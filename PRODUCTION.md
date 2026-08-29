# Binder production plan

> **Ordering note:** [ROADMAP.md](ROADMAP.md) is authoritative for product
> sequencing. This document describes a possible later hosted architecture.
> The registry and public API are conditional on retained local receipt use and
> third-party retrieval; they are not the first production product.

This plan is subordinate to the product boundary in [PRODUCT.md](PRODUCT.md) and the validation gate in [VALIDATION.md](VALIDATION.md).

## Product outcome

Binder should help a Solana maintainer answer one release question:

> Which important claims still hold for the program we are about to deploy, and what can another reviewer replay?

If retained local use creates enough public receipt supply, a later hosted
product may combine a GitHub-native release check with a read-only
program-status API. Verification still runs in the program owner's CI or
workstation. Binder stores and serves receipts; it does not execute untrusted
customer code.

## Initial users

- **Author:** a program developer or coding agent proposing a change.
- **Reviewer:** a maintainer deciding whether to merge or deploy it.
- **Consumer:** an explorer, wallet, auditor, or release tool reading current claim status.

Start with Solana teams that upgrade an audited program frequently. Do not optimize for new teams that only need help writing or deploying a contract.

## Production workflow

```text
issue/PR claim + repository change
                |
                v
       binder verify in CI
                |
                +--> GitHub check: base/head evidence and freshness
                |
                `--> content-addressed receipt and replay bundle
                                  |
                                  v
                         binder publish
                                  |
                     registry + object storage
                                  |
                                  v
                 program/claim status API
                    |                 |
                 explorer          release gate
```

The CLI and receipt remain authoritative and independently replayable. The hosted registry is a discovery and distribution layer, not a source of truth.

## Must-have v1

### Repository experience

- Binder accepts a small explicit manifest or a machine claim compiled from issue/PR metadata.
- Stable claim schema names the statement, subject repository, base/head revisions, dependencies, trials, and artifacts.
- `binder verify` supports machine-readable JSON as well as the terminal report.
- An official GitHub Action installs Binder, runs verification, uploads the replay bundle, reports a status check, and writes the job summary.
- Released binaries and pinned Action versions remove the need to compile Binder in every repository.
- Clear execution, observation, freshness, and policy fields map to documented exit behavior for release gates.

### Program identity

- A receipt can name a Solana program ID, cluster, deployed executable digest, source revision, and build digest.
- Local semantic evidence and deployed identity remain separate fields and separate verdicts.
- The first integration consumes an existing verified-build result rather than building another reproducible-build service.

### Registry and API

- `binder publish` uploads a receipt and replay bundle after local validation.
- Immutable bundles live in object storage under their digest.
- A small relational index maps program IDs, revisions, claims, and latest receipt digests.
- Read endpoints expose program claims, status, evidence kinds, checked revision, deployed-artifact match, and replay URL.
- Every API response includes receipt identity and freshness time; consumers can fetch and validate the underlying receipt.

Initial resource shape:

```text
GET /v1/programs/{program_id}?cluster=mainnet-beta
GET /v1/programs/{program_id}/claims
GET /v1/claims/{claim_id}/receipts/{digest}
GET /v1/receipts/{digest}/bundle
```

## Now / next / later

### Now — prove repeated use

1. Apply Binder to three real program upgrades from design partners.
2. Observe the authoring, review, replay, and deployment workflow end to end.
3. Validate explicit manifests against claims compiled from issue/PR metadata; build `binder init` only if manual authoring is the observed constraint.
4. Package an official GitHub Action and downloadable CLI binaries after the workflow passes the validation gate.
5. Measure setup time, replay success, reviewer comprehension, and stale claims caught.

Exit gate: at least three teams complete a real upgrade; two choose to keep Binder enabled; a new claim takes under 15 minutes to configure when a suitable check already exists; and at least 80% of clean CI replays succeed without intervention.

### Next — connect evidence to deployed programs

1. Add the explicit program/build/deployment identity model.
2. Integrate Solana verified-build metadata.
3. Build `binder publish`, immutable bundle storage, and the read API.
4. Ship a minimal public program page generated from the same API.
5. Pilot a release gate that blocks when required claims are stale or refer to a different deployed artifact.

Exit gate: a consumer can start from a program ID, identify its current claims, retrieve the exact receipt, and replay at least one claim without private coordination.

### Later — ecosystem consumption

1. Explorer integration for claim and evidence summaries.
2. Wallet-facing warnings for stale or mismatched program evidence.
3. Auditor-authored incremental attestations as an additional evidence kind.
4. Claim history across upgrades and dependency-aware selective reruns.
5. Additional evidence adapters such as Certora, Trident, Crucible, and LiteSVM.

These remain contingent on consumers using the API; do not build wallet UI or a broad adapter marketplace before that demand exists.

## Service boundaries

| Component | Responsibility | Suggested implementation |
| --- | --- | --- |
| Binder CLI | Own all claim, execution, receipt, replay, status, and machine-interface semantics | Existing Rust workspace |
| GitHub Action | Install and invoke the CLI, then map its outputs into GitHub surfaces | Thin composite or JavaScript action |
| Registry API | Validate uploads, index metadata, serve queries | Small stateless service |
| Metadata index | Programs, claims, revisions, receipt pointers | PostgreSQL |
| Bundle store | Immutable receipts, inputs, and raw output | S3-compatible object storage |
| Program page | Human-readable view of API data | Static or server-rendered frontend |

Do not add a queue or worker fleet until Binder owns long-running execution. For the initial model, publishing is a synchronous metadata transaction plus object upload.

## Product metrics

- Median time to author a claim when the underlying test already exists.
- Clean-environment replay success rate.
- Median reviewer time to correctly explain the claim and evidence boundary.
- Number of stale or artifact-mismatched claims caught before deployment.
- Percentage of published receipts independently downloaded or replayed.
- Design-partner retention across a second program upgrade.

The primary metric is **program upgrades reviewed with current Binder claims**, not receipts generated.

## Explicitly deferred

- Hosted execution of arbitrary verification commands.
- A universal security score.
- Automatic generation of smart-contract invariants.
- A new verifier, fuzzer, simulator, or reproducible-build system.
- Wallet integration before the registry has useful coverage.
- Multi-tenant billing, organizations, and broad access-control machinery during the design-partner phase.

## Immediate implementation sequence

1. Replace the demo's combined status model with typed execution, observation, freshness, and policy contracts; version the replacement receipt schema.
2. Define the smallest claim-specific witness protocol that existing test runners can emit without adopting a Binder DSL.
3. Define the PR claim convention, real Git base/candidate identity, and test-only-patch behavior in validation fixtures.
4. Run the paired review study using manually prepared claims and the typed report.
5. If authoring cost is the demonstrated constraint, add failing acceptance tests for `binder init` or PR-to-claim compilation.
6. Only after the study passes, release cross-platform binaries and build the GitHub Action.
7. Onboard the first real repository before starting the registry.

This sequence keeps the next milestone falsifiable: if the claim–evidence relationship does not improve decisions inside the existing pull-request workflow, a new interface or hosted registry will not rescue the product.
