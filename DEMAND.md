# Binder: latent-demand scan

## Conclusion

There is credible latent demand for Binder's problem, but not yet clear demand for Binder's exact product.

The strongest evidence clusters around four jobs:

1. keep security claims current after code and dependency changes;
2. connect audits and verification results to the exact deployed artifact;
3. reconcile conflicting evidence from simulators, fuzzers, proofs, and runtimes;
4. reduce the cost of repeatedly expressing and reviewing the same invariant.

The first two are explicit institutional and developer requests. The latter two are inferred from recurring tooling failures and workflow fragmentation.

## Strong signals

### 1. The Solana Foundation asked for almost this interface

Its Program Verification Tooling RFP asked for program security details consumable by wallets, explorers, and developers, plus incremental audit attestations that could be ingested by the system. The surrounding announcement also called for visualizing whether programs were updated or outdated.

This is direct demand for:

- versioned assurance metadata;
- incremental evidence;
- program-status invalidation;
- downstream consumption through APIs.

What it does not establish is demand for replayable claims rather than attestations. Binder's opportunity is to replace the attestation-only substrate with independently runnable checks.

Source: [Program Verification Tooling RFP](https://forum.solana.com/t/program-verification-tooling/1032)

### 2. Teams explicitly track the unaudited delta

The Solana Foundation's `subscriptions` repository maintains an audit-status file recording audit history, audited-through commits, and the current unaudited delta. This is Binder's staleness problem being handled manually in a production-oriented repository.

Source: [solana-foundation/subscriptions](https://github.com/solana-foundation/subscriptions)

### 3. Developers ask whether small changes require another audit

Forum discussions repeatedly ask whether a handful of changed lines justify paying for another formal audit. The underlying job is not simply cheaper auditing; it is establishing which previous security claims survive a delta and which require renewed evidence.

Source: [Smart Contract Audit with few changes](https://www.reddit.com/r/ethdev/comments/14soigu)

### 4. Post-audit drift is named directly

A recent Ethereum developer discussion asks how teams preserve audited assumptions when circuits, verifier keys, public inputs, and proving artifacts keep changing—and whether such changes should block CI automatically.

That is almost exactly Binder's root-invalidation interface, although the thread is small and should be treated as a qualitative lead rather than market validation.

Source: [Security drift after an audit](https://www.reddit.com/r/ethdev/comments/1u7vf14/)

## Repeated workflow pain

### Simulator and runtime disagreement

A LiteSVM issue reports roughly 20% higher compute-unit consumption than Mollusk for an otherwise identical transaction. The discrepancy comes partly from different execution boundaries: full transaction versus individual instruction.

Separately, a Solana Foundation example repository uses Mollusk because its Agave version supports required curve syscalls while Surfpool and LiteSVM are pinned to an older version and cannot run the same tests.

These are not requests for another simulator. They demonstrate the need to bind every verdict to its engine, version, feature set, and execution boundary—and to prevent evidence from silently being treated as interchangeable.

Sources:

- [LiteSVM compute-unit discrepancy](https://github.com/LiteSVM/litesvm/issues/277)
- [Crypto-primitives test-version constraints](https://github.com/solana-foundation/crypto-primitives-examples)

### Developers search for invariant testing rather than finding one standard path

A Solana Stack Exchange question asks for the equivalent of Foundry invariant testing. Trident and Crucible now address execution and discovery, but users still must define tool-specific actions, state models, and invariants.

The latent Binder job is not providing invariant fuzzing. It is letting the semantic claim persist while evidence engines change.

Sources:

- [Solana invariant-testing question](https://solana.stackexchange.com/questions/22990/what-is-the-equivalent-of-foundrys-invariant-testing-in-solana)
- [Crucible](https://github.com/asymmetric-research/crucible)
- [Trident requests and limitations](https://github.com/Ackee-Blockchain/trident/issues)

### Audit preparation is dominated by unclear intent and incomplete tests

Audit discussions repeatedly identify unclear documentation and insufficient test coverage as major preparation costs. Developers and auditors reconstruct the intended invariants manually before they can assess the implementation.

Binder could make the claim and its executable evidence the review unit, reducing reconstruction work. This is an inference; the discussions do not request a claim manifest explicitly.

Sources:

- [Audit pain-point discussion](https://www.reddit.com/r/solidity/comments/1p76dua/whats_the_biggest_pain_point_youve_faced_during_a/)
- [How to prepare for a smart-contract audit](https://www.reddit.com/r/ethdev/comments/ssr618/how_to_prepare_for_a_smart_contract_audit/)

### Cheap scanners create noise rather than entitlement

Developers evaluating AI audit tools report false positives, thin reports, and difficulty finding tools that consistently identify real issues. Manual audits remain expensive for small projects.

Binder should not compete as another scanner. Its opportunity is to attach durable, replayable evidence to claims and counterexamples produced by existing scanners or agents.

Source: [AI smart-contract audit tools discussion](https://www.reddit.com/r/ethdev/comments/1r6z12x/ai_smart_contract_audit_tools_anyone_found_one/)

## Existing behavior that validates parts of the model

### Verified builds already establish one provenance edge

Solana's verified-build workflow links a repository commit to deployed bytecode through a reproducible Docker build. The service re-verifies programs periodically and marks upgraded programs unverified until their metadata is updated.

This validates demand for replayable source-to-artifact provenance and automatic staleness. Its documentation explicitly warns that verified does not mean audited or safe, leaving room for Binder to attach semantic claims and evidence above that identity edge.

Source: [Solana verified builds](https://github.com/solana-foundation/solana-com/blob/main/apps/docs/content/docs/en/programs/verified-builds.mdx)

### Agave already treats fixtures as cross-client settlement objects

Agave's conformance tooling runs Protobuf fixtures against one or more execution targets, compares expected and actual effects, and uses failures to locate client divergence.

This validates deterministic replay as a native ecosystem behavior. Binder would generalize the artifact from runtime conformance to semantic program claims.

Source: [Agave conformance testing](https://github.com/anza-xyz/agave/wiki/Conformance-Testing)

### Formal foundations are being built but remain fragmented

The Solana Foundation's experimental Lean library aims to provide reusable formal definitions so downstream projects do not redefine primitives. Earlier core-community discussion identified the lack of a specification as a prerequisite blocker for formal verification.

This supports Binder's assumption-first model: specifications and extern models must be explicit roots rather than hidden verifier authority.

Sources:

- [Lean Solana library](https://github.com/solana-foundation/leanprover-solanalib)
- [Solana core-community specification discussion](https://github.com/solana-foundation/core-community-call/blob/main/call-notes/call_1.md)

## Demand assessment

| Hypothesis | Evidence | Confidence |
| --- | --- | --- |
| Teams need assurance to become stale automatically after upgrades | Foundation RFP, verified builds, manual unaudited-delta tracking, developer audit-delta questions | High |
| Teams want machine-readable security evidence attached to exact versions | Foundation RFP and verified-build ecosystem | High |
| Teams need evidence reconciled across execution tools | Recurrent version and semantic discrepancies | Medium-high |
| Teams want to declare an invariant once and reuse it across tools | Repeated tool-specific invariant work, but no direct request for portability | Medium |
| Teams will author and maintain Binder manifests | No direct evidence yet | Low |
| Teams will pay for a hosted claim/evidence graph | No direct evidence yet | Low |
| Flux plus Mollusk is the preferred first adapter | Technically coherent, but no market evidence yet | Low |

## Best initial customer interviews

The next step should not be more broad web research. Interview people already paying the coordination cost:

1. maintainers of repositories that publish audit-status or audited-through commits;
2. Solana protocol security leads shipping frequent upgrades;
3. auditors who perform fix review and incremental re-audits;
4. Mollusk, LiteSVM, Trident, and Crucible maintainers dealing with semantic/version drift;
5. verified-build and explorer maintainers consuming program-assurance metadata;
6. STRIDE participants and vendors.

Ask for the last real upgrade, not opinions about a hypothetical product:

- Which claims from the previous audit were supposed to remain true?
- How did you decide which changes invalidated them?
- Where were those claims written?
- Which checks could another engineer replay without the author?
- What evidence was tied to the deployed binary rather than the source branch?
- How long did fix review and security re-establishment take?
- What would have blocked the release automatically?

## Falsifiable demand test

Take five recently upgraded Solana programs with public audit histories. Produce a Binder-style report containing:

- audited-through commit;
- current deployed binary and verified source;
- unaudited source delta;
- named claims affected by changed roots;
- replayable checks still standing;
- stale or unsupported claims.

Show it to the maintainers and auditors. Demand is supported if at least three say it replaces work they currently perform manually and at least two agree to run it on their next upgrade. If they view it only as nicer reporting, the product boundary is too shallow.

## Bottom line

The latent demand is realest around **security drift and incremental assurance**, not around generic formal verification.

The clearest initial positioning is:

> Binder tells you which security claims survived the change, which became stale, and exactly what another engineer can replay before deployment.

That is narrower and better supported than "verifiable smart contracts" or "declare an invariant once across every tool."
