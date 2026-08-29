# Binder competitive scan

_As of 2026-08-28._

## Finding

There are working demos for nearly every part of Binder, but no product found binds a named semantic invariant to fresh, independently rerunnable evidence from multiple verification engines and carries that claim across source, build, and deployed artifact revisions.

This means Binder should not present itself as a generic claims ledger, receipt format, formal verifier, or reproducible-build service. Those categories already have credible implementations. Its useful boundary is the integration layer that answers:

> Which security claims still hold for this exact revision, according to which independently rerunnable checks, under which assumptions?

## Closest systems

| System | What already works | Unit of assurance | What it does not establish | Implication for Binder |
|---|---|---|---|---|
| [Itself](https://github.com/Greater-Expanse/itself) | Python SDK and CLI; claims, hypotheses, external-check records, append-only ledger, deterministic replay, portable bundles, receipts | A claim and its recorded evidence/authority transitions | Replay rebuilds state from retained records; it does not rerun the underlying verification command | Closest protocol competitor. Do not invent a generic evidence ledger as the product |
| [Reelier](https://github.com/seldonframe/reelier) | npm CLI/GitHub integration; records agent workflows, replays them without another model call, detects drift, emits signed receipts | An agent workflow and its observable effects | Explicitly proves scope/change, not program correctness or safety | Closest contributor-to-reviewer UX. Binder receipts should concern semantic properties, not agent provenance |
| [Vela](https://github.com/vela-science/vela) | Signed CLI; claims, submissions, scoped verification records, authorized decisions, standing, exact state replay | Governed scientific claims | Replay verifies retained objects and decisions but does not rerun the source-owned scientific method | Strong prior art for governance and claim standing, not executable software assurance |
| [Certora Solana Prover](https://docs.certora.com/en/latest/docs/solana/index.html) | Production formal verification for Rust/Solana using CVLR rules, Solana models, prover output, and rule sanity checks | A formal rule over a modeled program | One verification engine; proof artifacts are not a cross-engine, revision-aware claim lifecycle | Integrate it as an evidence producer rather than compete with it |
| [Otter Solana Verify](https://github.com/otter-sec/solana-verify) | Prototype Anchor-compatible annotations such as success/error postconditions and account invariants | Source-level Solana invariants | Last repository activity found was 2023; current checkout did not compile on the present toolchain | Useful syntax precedent, not a current platform foundation |
| [Solana Verified Builds](https://github.com/solana-foundation/solana-com/blob/main/apps/docs/content/docs/en/programs/verified-builds.mdx) | Docker-reproducible builds and public comparison of repository commit, executable hash, and on-chain program | Source-to-deployed-binary identity | Correct identity is not semantic correctness; the docs explicitly make that distinction | Use its artifact identity as the last link in Binder's evidence chain |
| [AuditBase](https://www.auditbase.com/) | Continuous Solidity re-analysis and alerts when deployed risk changes | Scanner findings for deployed contracts | Centralized analysis result, Solidity-focused, without a portable named-claim graph or heterogeneous replay | Confirms demand for freshness; Binder can make freshness inspectable and engine-independent |

## Demo validation

I ran the public reference paths rather than relying only on landing-page claims:

- **Itself:** its checked-in cache-key diagnosis bundle validated; replay reconstructed one supported and two refuted hypotheses; its receipt validated against the evidence ledger.
- **Reelier:** the TypeScript build completed and a large test suite exercised real receipt, drift, authority, and recovery behavior. The full suite is not clean from a macOS temp path because its anti-symlink ancestry checks treat `/tmp` via `/private/tmp` as linked; this is an environment-sensitive test failure, not evidence that the core demo is fictitious.
- **Otter Solana Verify:** after fetching its pinned submodules, it failed against the current Rust environment on obsolete nightly-only behavior. It is a real prototype, but not an immediately usable contemporary demo.
- **Vela:** the repository ships a current signed CLI and end-to-end public quickstart. Its own documentation carefully limits replay to repository state and explicitly excludes rerunning the scientific method.

## The unoccupied boundary

The market is split into four layers:

1. Verifiers produce proof, fuzz, simulation, or static-analysis results.
2. Reproducible-build systems identify which source produced which deployed binary.
3. Receipt and governance systems preserve what was observed and who accepted it.
4. Continuous scanners say whether their own findings have changed.

Binder should join those layers around the **semantic claim**, while leaving each engine authoritative for its own result. Its minimal object is not merely a receipt:

```yaml
claim: fixture identity includes return data
scope:
  source: <git tree and paths>
  build: <toolchain and ELF digest>
assumptions: <runtime and external specifications>
evidence:
  - kind: flux-proof
    rerun: <hermetic command>
  - kind: mollusk-regression
    rerun: <hermetic command>
status: fresh | stale | failed | unsupported
```

The differentiator is that `binder verify` reruns the producers, checks their pinned inputs, and recomputes claim status. A changed source root, external specification, toolchain, runtime fixture, or deployed ELF makes the claim stale until the appropriate evidence is regenerated.

## Best first demo

Dogfood Binder on the Mollusk `return_data` fixture-hash fix:

1. Declare the claim: fixtures differing only in return data have distinct identities.
2. Run the regression against the parent revision and record the expected failure.
3. Run it against the fix revision and record the pass.
4. Bind both observations to commits, command, toolchain, fixture inputs, and output digests.
5. Change the hashing implementation and show Binder automatically marking the claim stale.
6. Render the result as a compact PR check with a downloadable evidence bundle.

That demo is small, real, and highlights exactly what the closest competitors do not: semantic correctness evidence, differential base/head behavior, actual rerun, and revision-aware invalidation.

## Strategic conclusion

The competition makes Binder narrower and better. The product is not “Git for claims” or “formal verification made easy.” It is the assurance dependency graph and CI interface between existing verification engines and a reviewer deciding whether a security claim remains justified for the code actually being shipped.
