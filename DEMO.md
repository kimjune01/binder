# Binder demonstrator: the warranted vault

## Verifiable goal

Build a deliberately vulnerable Solana vault and use Binder to establish one claim:

> An unsuccessful withdrawal cannot reduce the vault balance or increase the recipient balance.

The demo is complete only when a stranger can reproduce the claim's current status from a clean checkout with one command. Binder must show the claim failing on the vulnerable base revision, passing on the fixed head revision, and becoming stale when any declared dependency changes.

## Audience and decision

The audience is a smart-contract maintainer reviewing a pull request. Within 90 seconds, the demo must let them decide:

- what security property the patch claims to restore;
- whether the bug existed before the patch;
- which independent checks support the fix;
- exactly which code, specifications, runtime, and binary were checked;
- whether that evidence is still current; and
- how to rerun it without trusting Binder's service.

## Scenario

The vault exposes a withdrawal instruction. The vulnerable implementation credits or transfers to the recipient before all authorization and failure conditions are settled. An unauthorized withdrawal returns an error but can still mutate the modeled transition or observable account state.

The fixed implementation validates first and commits state changes only on success.

The demo repository preserves both revisions:

- `demo/vulnerable`: the known-bad base;
- `demo/fixed`: the reviewed fix.

## Evidence

The same semantic claim receives evidence from independent boundaries.

### 1. Rust transition proof

A small pure Rust transition kernel represents the vault's balance and authorization rules. A refinement or formal-verification check proves that every unsuccessful transition preserves both balances.

Preferred engine: Flux, provided the pinned version can verify the example reproducibly. If Flux cannot provide a stable clean-machine run, the demonstrator may temporarily use Kani or bounded exhaustive Rust tests, but the evidence kind must state the weaker guarantee accurately.

### 2. Solana runtime replay

Mollusk executes the compiled instruction with concrete accounts and an unauthorized-withdrawal fixture. The checker asserts:

- the instruction records an application-level rejection;
- the vault balance is unchanged;
- the recipient balance is unchanged; and
- the relevant account data and return data match the declared predicate.

This checks the integration boundary that the pure transition proof abstracts away.
The SVM instruction itself returns success because Solana rolls back account
writes on an instruction error. The vulnerable program instead exposes the bug
by recording rejection in account state while committing the transfer; the
fixed program records rejection and preserves balances.

### 3. Artifact identity

Binder records the compiled program digest and, when feasible, verifies that it was produced from the declared source and toolchain. A public demo may optionally bind that digest to a deployed devnet program using Solana Verified Builds.

Deployment identity strengthens the story but is not required for the first local milestone.

## Required user experience

From a clean checkout:

```bash
cargo run -p binder-cli -- verify demo/claims/failed-withdrawal-preserves-balances.yaml
```

The fixed revision should render a compact report resembling:

```text
WARRANTED  failed-withdrawal-preserves-balances

Base  FAIL  recipient balance changed after rejected withdrawal
Head  PASS

Evidence
  PASS  rust-transition-proof   source 81d9…  spec c024…
  PASS  mollusk-runtime-replay  elf 94aa…     fixture 71bc…

Assumptions
  Rust toolchain …  Solana runtime …  Mollusk …

Replay bundle  .binder/receipts/<digest>/
```

After changing a dependency without regenerating its evidence:

```text
STALE  failed-withdrawal-preserves-balances

Changed
  src/transition.rs  81d9… -> e412…

Required
  rerun rust-transition-proof
  rerun mollusk-runtime-replay
```

## Claim contract

The authored claim should remain small and reviewable:

```yaml
version: 1
id: failed-withdrawal-preserves-balances
claim: An unsuccessful withdrawal cannot reduce the vault balance or increase the recipient balance.

dependencies:
  source:
    - programs/vault/src/**
    - crates/vault-transition/src/**
  specifications:
    - demo/specs/withdrawal.rs
  fixtures:
    - demo/fixtures/unauthorized-withdrawal.json
  toolchain:
    - rust-toolchain.toml
    - Cargo.lock

trials:
  - id: rust-transition-proof
    adapter: rust
    command: cargo test -p vault-transition --test invariant
  - id: mollusk-runtime-replay
    adapter: mollusk
    command: cargo test -p vault-program --test unauthorized_withdrawal

policy:
  require:
    - rust-transition-proof
    - mollusk-runtime-replay
  compare: base-and-head
```

The actual schema may evolve, but every field above represents an acceptance requirement. Commands are executed as argument vectors in the implementation rather than through an implicit shell.

## Architecture

```text
claim YAML
    |
    v
Binder CLI (Rust)
    |-- resolve and hash declared dependencies
    |-- execute pinned trial adapters
    |-- normalize observations into receipts
    |-- evaluate base/head and freshness policy
    `-- render terminal + PR summary
             |
             v
      content-addressed local bundle

Optional later:
GitHub Actions / remote runner <- provisioned with Pulumi
```

### Language boundaries

- **Rust:** CLI, claim parser, hashing, process execution, receipt validation, policy evaluation, renderers, vault program, transition kernel, and local adapters. This keeps the trusted path compact and makes the demo self-hosting in its target ecosystem.
- **Go:** no first-milestone component. Introduce it only if Binder gains a separately deployed coordinator whose concurrency, networking, or operational lifecycle justifies another language.
- **Pulumi:** optional CI infrastructure after local replay works. Prefer Pulumi Go if a Go control plane exists by then; otherwise Pulumi TypeScript is acceptable infrastructure glue. Pulumi must not be needed to verify a local receipt.

## Receipt boundary

Binder owns normalization and dependency validity, not the truth of an engine's internal reasoning. Each receipt records:

- claim and trial identifiers;
- command and adapter version;
- exact input digests;
- environment and toolchain identifiers;
- exit status and normalized observations;
- raw-output artifact digests;
- predicate verdict;
- start/end timestamps for diagnosis, excluded from deterministic identity; and
- receipt schema version.

The receipt is warranted only while all declared dependency digests match. A pass from one engine cannot substitute for a required pass from another.

## Acceptance tests

Implementation begins with failing tests for these behaviors:

1. The vulnerable transition violates the balance-preservation claim.
2. The fixed transition satisfies it.
3. The vulnerable Mollusk instruction replay fails the predicate.
4. The fixed Mollusk instruction replay passes it.
5. Binder reports `WARRANTED` only when every required trial passes on head.
6. Binder records the expected base failure, demonstrating that the trial distinguishes the patch.
7. Modifying source, specification, fixture, toolchain lock, or ELF makes affected evidence `STALE`.
8. Modifying an unrelated file does not invalidate the claim.
9. A missing, malformed, mismatched, or tampered receipt fails closed.
10. Two clean-machine runs over identical inputs produce the same receipt identity.
11. The report identifies the exact failed predicate without requiring inspection of raw logs.
12. The replay bundle contains everything permitted and necessary to rerun the trials, or explicitly names externally pinned inputs.

## Milestones

### M0 — Freeze the experiment

- Create the Rust workspace and pin the toolchain.
- Commit the vulnerable vault, fixed patch, claim, fixtures, and expected output snapshots.
- Write the acceptance tests before implementing Binder.

Exit: tests describe all twelve behaviors and fail for the expected missing implementation.

### M1 — Local warrant

- Parse the claim.
- Hash declared dependencies.
- Execute trials and produce deterministic receipts.
- Evaluate `WARRANTED`, `FAILED`, `STALE`, and `UNSUPPORTED`.

Exit: the pure Rust trial gives base-fail/head-pass and invalidates on dependency change.

### M2 — Heterogeneous evidence

- Add the Mollusk adapter and compiled-program binding.
- Require both trials without collapsing their guarantees.

Exit: one command produces a warrant supported by both source-level and runtime evidence.

### M3 — Reviewer surface

- Render concise terminal and GitHub step summaries.
- Package a portable, content-addressed replay bundle.
- Document clean-machine reproduction.

Exit: a reviewer can understand the result in 90 seconds and independently rerun it.

### M4 — Optional deployed identity

- Provision a reproducible remote runner with Pulumi.
- Bind the verified ELF to a devnet deployment.

Exit: the report connects the semantic warrant to the program artifact actually deployed, without making the hosted service authoritative.

## Explicit non-goals

- Designing a universal knowledge-graph protocol.
- Replacing Flux, Kani, Mollusk, Certora, auditors, or Solana Verified Builds.
- Claiming that a verified build is secure merely because source and ELF match.
- Automatically inferring arbitrary semantic claims in the first release.
- A dashboard, multi-tenant service, billing system, or general orchestration control plane before the local demonstrator works.

## Kill conditions

Reconsider the product boundary if any of these survive honest attempts:

- Existing tooling can already express this claim, rerun both engines, track dependency-level freshness, and emit an independently verifiable PR warrant with negligible glue.
- Users consistently care only about scanner findings or audit PDFs, not named claims and their continuing warrant.
- The source/runtime evidence cannot share a meaningful semantic claim without a large trusted translation layer.
- Clean-machine replay requires so much environment reconstruction that a receipt is no cheaper to verify than manually redoing the investigation.
- Reviewers cannot correctly explain the report's guarantee and limitations after the 90-second walkthrough.
