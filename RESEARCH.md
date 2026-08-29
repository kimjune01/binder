# Binder: initial boundary research

## Recommendation

Binder should serve the boundary between a software claim and the heterogeneous evidence that supports it.

The first user-facing interface is a repository-committed claim manifest, replayed locally or in CI. The first differentiated Solana adapter should link a Flux-verified pure Rust transition kernel to selected observations of the compiled program through deterministic Mollusk fixtures.

Binder must not initially claim that Flux proves an Anchor program, that the verified Rust source is equivalent to the deployed sBPF, or that Mollusk reproduces a complete validator.

Its honest verdict is:

> Property P was proved for kernel K at source S under assumptions A. Compiled artifact E exhibited the corresponding behavior for fixture set F under runtime configuration R.

## Why this boundary

Most adjacent execution engines already have strong owners:

| Boundary | Existing tools or companies | Assessment |
| --- | --- | --- |
| Rust/Anchor source to vulnerability findings | Sec3 and audit firms | Moderately crowded |
| Specification to formal proof | Certora CVLR, OtterSec | Sparse but occupied |
| Program to stateful exploration | Trident, Crucible, private fuzzers | Increasingly crowded |
| Instruction or transaction to deterministic execution | Mollusk, LiteSVM, Surfpool | Well served |
| Codebase to expert assessment | OtterSec, Neodyme, Trail of Bits, Zellic, Adevar, Cantina | Very crowded |
| Deployed protocol to monitoring and response | Hypernative, Blockaid, Range, Sec3 | Very crowded |
| Semantic claim to proof, fuzzing, replay, and invalidation evidence | No clear owner | Underserved |

The engines require developers to restate intended behavior in incompatible forms: a proof rule, fuzz invariant, simulator assertion, audit finding, or monitoring policy. Their results rarely persist as one portable claim whose roots and invalidation conditions remain explicit across releases.

Binder's unit of value is not another finding. It is a durable answer to:

> Which important claims still hold for the artifact we are about to deploy, and why?

## Product boundary

The immediate handoff is contributor or coding agent to reviewer and CI:

```text
semantic claim
    -> pinned subject and roots
    -> executable check
    -> explicit oracle and kill condition
    -> replay by CI or another agent
    -> content-addressed receipt
```

The strongest initial primitive is differential:

- the named regression fails at the base commit;
- it passes at the proposed commit;
- the broader suite remains green.

This establishes more than head-only CI: the check is sensitive to the proposed repair. It does not, by itself, prove that the prose claim is complete.

## Minimal claim model

```yaml
claim:
  id: fixture-return-data-affects-hash
  statement: Fixtures differing only in return_data have different hashes
  subject:
    repo: anza-xyz/mollusk
    base: <commit>
    head: <commit>

check:
  runner: process
  argv:
    - cargo
    - test
    - -p
    - mollusk-svm-fuzz-fixture
    - test_hash_includes_return_data
  timeout_seconds: 600

oracle:
  base: { exit: nonzero }
  head: { exit: 0 }

roots:
  - kind: git_commit
    digest: <sha>
  - kind: toolchain
    digest: <digest-or-version>

kill:
  - base_check_passes
  - head_check_fails
  - dependency_root_changes
```

A receipt records resolved roots, output hashes, replay cost, and one of three verdicts: `supported`, `refuted`, or `indeterminate`. Timeout, missing infrastructure, and unavailable dependencies are indeterminate rather than refutations. A signed receipt authenticates its producer but does not confer truth; the replay command remains the entitlement.

## First deep adapter: Flux plus Mollusk

The technically defensible seam is a small pure-Rust state-transition kernel:

```text
Anchor/native wrapper
    -> decode and validate accounts
    -> Flux-verified transition kernel
    -> serialize result
    -> compile to sBPF
    -> execute boundary fixtures in Mollusk
```

Flux can prove arithmetic relations, bounds, conservation, authorization predicates supplied to the kernel, legal state transitions, and selected panic freedom. It relies on extern specifications for dependencies; those are assumptions and must be roots in the claim graph.

Anchor constraints remain runtime checks in the first version. They should be tested through compiled instruction execution, not automatically translated into Flux predicates. Mollusk executes the compiled ELF using lower-level Agave SVM components, making it a strong instruction-level witness, but it deliberately omits the full validator, Bank, and AccountsDB.

The proof and executable witness must therefore remain separate evidence nodes joined under one semantic claim.

## Smallest credible prototype

1. Build a dependency-light escrow or ledger transition kernel.
2. Prove conservation, authorization, no underflow or overflow, failure atomicity, and legal state progression with Flux.
3. Wrap it in a thin native or Anchor entrypoint.
4. Compile the real program to sBPF.
5. Exercise success, boundary, malformed-account, wrong-authority, duplicate-account, readonly-account, and rollback cases through Mollusk.
6. Emit a Binder bundle containing:
   - claim text and stable ID;
   - source, dependency, extern-spec, and toolchain hashes;
   - separate proof and observation verdicts;
   - the sBPF ELF hash;
   - Mollusk and Agave versions, feature set, compute budget, and sysvars;
   - deterministic fixtures and replay commands;
   - explicit assumptions and replay cost.
7. Invalidate the verdict when any rooted input changes.

## Necessary components

### Protocol core

- Canonical claim manifest and content-addressed claim IDs.
- Typed provenance roots and dependency edges.
- Explicit checks, oracles, and kill conditions.
- Three-state verdicts with machine-readable reasons.
- Content-addressed receipts that never replace replay.
- Staleness and downstream invalidation propagation.

### Execution

- A sandboxed local runner with pinned working directory, arguments, environment, timeout, and artifacts.
- Base/head worktree execution for differential checks.
- Structured JSON output and stable exit codes.
- A GitHub Action that invokes the same local command.

### Solana adapters

- Mollusk fixture ingestion and deterministic replay.
- Source, IDL, ELF, SDK, runtime-configuration, and fixture hashing.
- Flux result and extern-spec ingestion.
- Later: Trident or Crucible counterexample ingestion, LiteSVM transaction evidence, verified-build identity, and Agave conformance fixtures.

### User surface

- `binder verify <claim>` locally.
- A concise PR check showing statement, roots, assumptions, base/head behavior, evidence, replay command, and cost.
- Minimized counterexamples preserved as replayable fixtures.

## Do not build in v0

- A new prover, fuzzer, simulator, or proof language.
- Automatic translation of arbitrary Anchor macros.
- Proof of CPI behavior or unsafe SDK internals.
- Source-to-sBPF semantic-equivalence claims.
- Agave/Firedancer consensus-equivalence claims.
- A hosted-only system whose receipts cannot be replayed without Binder.
- A generic dashboard before the local claim protocol works.

## Principal failure modes

- A replayable but vacuous test.
- A mocked dependency that removes the real boundary.
- Hidden time, network, randomness, hardware, or environment roots.
- Green receipts surviving dependency changes.
- Flaky checks oscillating between verdicts.
- Treating infrastructure failure as refutation.
- Running arbitrary contributed checks as an unsafe CI service.
- Claiming independent verification when every replay shares one implementation and blind spot.
- Letting users infer deployed correctness from source-only evidence.

Binder should describe itself as replayable software assurance, not trustless verification.

## Falsifiable first trial

Before committing to the full Solana verifier integration, implement manifest parsing, base/head execution, three-state verdicts, hashed receipts, and a GitHub check. Apply it to 15-30 real agent-authored fixes, including seeded controls for insensitive tests, flaky tests, missing tools, and stale roots.

Measure:

- fresh-machine replay success;
- false fix claims caught by base-fail/head-pass;
- reviewer time needed to understand what was checked;
- authoring and replay overhead;
- correct classification of indeterminate outcomes;
- agreement between independent receiving agents.

Kill the initial hypothesis if fewer than 80% of claims replay independently, differential verification catches nothing meaningful beyond ordinary CI, median overhead exceeds roughly ten minutes for a small repair, or reviewers still reconstruct the claim/check relationship manually.

If the protocol survives that test, the Flux/Mollusk prototype becomes Binder's first high-value domain adapter rather than a prerequisite for discovering whether Binder itself works.

## Strategic sequence

```text
Phase 1: contributor -> replayable PR claim
Phase 2: invariant -> heterogeneous evidence bundle
Phase 3: Flux source proof + Mollusk compiled witness
Phase 4: fuzzing, transaction, verified-build, and conformance adapters
Phase 5: cross-release and cross-runtime truth maintenance
```

The moat is the claim/evidence graph and its invalidation semantics. Flux, Mollusk, Trident, Crucible, LiteSVM, Agave, and future tools are evidence-producing adapters into that graph.

## Sources

- [Flux documentation](https://flux-rs.github.io/)
- [Flux paper](https://doi.org/10.1145/3591283)
- [Flux extern specifications](https://flux-rs.github.io/flux/tutorial/07-externs.html)
- [Anchor account constraints](https://www.anchor-lang.com/docs/references/account-constraints)
- [Mollusk fixtures and execution model](https://github.com/anza-xyz/mollusk/blob/main/README.md)
- [Solana program execution](https://solana.com/docs/core/programs/program-execution)
- [Solana testing strategy](https://github.com/solana-foundation/solana-dev-skill/blob/main/skills/solana-dev/references/testing.md)
- [Certora Solana Prover](https://docs.certora.com/en/latest/docs/solana/index.html)
- [Trident](https://github.com/Ackee-Blockchain/trident)
- [Crucible](https://blog.asymmetric.re/introducing-crucible-an-invariant-fuzzing-framework-for-solana/)
- [STRIDE](https://stride.asymmetric.re/)
