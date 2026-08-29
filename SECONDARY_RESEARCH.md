# Binder secondary-research corpus

Read existing practitioner conversations before asking people to repeat them.
This corpus replaces broad discovery outreach. Do not schedule interviews while
a question can still be answered from public issues, reviews, commits, audit
reports, postmortems, standards debates, or adoption history.

Conversation is not inherently better evidence than search. For past behavior
and technical workflow, the public artifact is often better: it is
contemporaneous, inspectable, and connected to the actual code. Ask a person
only when the missing fact is private, tacit, or requires observing their
response to a new Binder artifact.

## Research question

> Where do smart-contract practitioners already describe losing meaning or
> trust between an intended property, a check, a build, a reviewed revision,
> and deployed code?

This is not a market-size survey. It is a collection of observable arguments,
failures, and design decisions around Binder's proposed interfaces.

## Read first

### 1. Reproducible does not mean trustworthy

[Stellar contract source verification using Docker](https://github.com/orgs/stellar/discussions/1923)
is the highest-value conversation in the corpus: 24 comments and 87 replies
among contract developers, explorer operators, and verification implementers.

The thread distinguishes several claims that a single “verified” badge could
incorrectly collapse:

- a source and environment reproduce the deployed Wasm;
- a named builder attested to producing it;
- the build image itself is inspectable and trusted;
- the public source is available independently; and
- the source actually expresses the advertised behavior.

One participant points out that a pinned hostile compiler can reproducibly
produce malicious bytes: reproducibility is not faithfulness to source. Others
debate public repositories versus private source bundles, exact revisions,
trusted registries, long-term rebuildability, and who can afford independent
verification.

**Binder relevance:** preserve these as separate evidence kinds and claim
edges. Do not create a generic trust score or `verified` state.

### 2. Provenance is narrower than safety

[Stellar contract build verification SEP discussion](https://github.com/orgs/stellar/discussions/1573)
debates GitHub attestations, exact commits, workflow identity, hosted versus
self-hosted runners, dependencies, reproducible containers, and on-chain
metadata. The proposal explicitly says build verification reveals how a
contract was built; it does not mean the contract is safe.

**Binder relevance:** source/build/deployment identity is an existing interface
Binder should consume, not reinvent. A behavioral receipt should remain a
different claim with a different warrant.

### 3. A safety mechanism can create the wrong mental model

[OpenZeppelin's read-only `nonReentrant` discussion](https://github.com/OpenZeppelin/openzeppelin-contracts/issues/4422)
is fundamentally about entitlement and interpretation, not implementation.
Maintainers worry that adding an easy modifier will lead users to believe they
are protected when the modifier does not match the invariant. They distinguish
an easy local fix from correct system design and weigh security against
composability.

**Binder relevance:** agent-legible output must say when a mechanism applies,
not merely that it ran. Guarantee-boundary comprehension is a core product
requirement, not documentation polish.

### 4. The library cannot own every application assumption

[OpenZeppelin's MerkleProof issue](https://github.com/OpenZeppelin/openzeppelin-contracts/issues/3091)
includes a reproducer, impact analysis, proposed API changes, documentation
changes, and a decision about responsibility. The primitive is unsafe only
under particular leaf-encoding assumptions, so maintainers must decide whether
the library, type system, documentation, or application owns the guard.

**Binder relevance:** the entitlement edge needs an explicit author and scope.
Observations do not determine who is qualified to say what they warrant.

### 5. Engine agreement is not automatic

[LiteSVM compute-unit discrepancy discussion](https://github.com/LiteSVM/litesvm/issues/277)
starts from an observed disagreement among LiteSVM, Mollusk, and on-chain
execution. Participants request a reproducer and propose JIT and platform
hypotheses before agreeing on what the numbers mean.

**Binder relevance:** record the engine and boundary as part of evidence
identity. Never silently port an empirical conclusion across engines.

### 6. Invariants are authored before they are found

[Crucible sidecar discussion](https://github.com/asymmetric-research/crucible/issues/13)
proposes an AI-assisted tool that authors invariants for an existing fuzzer.
The author provides clean/planted variants, pinned toolchain versions, captured
scorecards, and explicit provenance for AI-suggested invariants, then asks
whether the sidecar and output schema are the right interface.

**Binder relevance:** this is direct evidence for a thin claim layer above
existing engines. It also exposes the unresolved question of who approves an
agent-proposed invariant.

## Read as concrete cases

### Audit finding to remediation

The [Solana Foundation subscriptions audit trail](https://github.com/solana-foundation/subscriptions/blob/main/audits/AUDIT_STATUS.md)
records an audited baseline, a remediation range, report, compare links, and
reproduction commands. The
[incarnation-binding remediation](https://github.com/solana-foundation/subscriptions/commit/d4b29e80e2b3db3fc5cd449ffb7b563055644d51)
connects a specific audit finding to code and a regression test, but the public
trail does not preserve the same observation on the audited base.

**Binder relevance:** this is the best first demo case. Binder can add a
base/head behavioral receipt without replacing the report, Git, test runner,
or audit judgment.

### When a proof's scope matters

The [Uniswap V2 formal-verification report](https://dapp.org.uk/reports/uniswapv2.html)
states that its claims hold only under specified token assumptions and do not
cover multi-call contract-level invariants such as the constant-product
property. This is unusually clear public boundary-setting.

**Binder relevance:** a proof result still needs an authored claim and explicit
assumptions. “Formal” does not eliminate the entitlement edge.

### When a missing invariant becomes operationally concrete

The [Hyperbridge MMR verifier postmortem](https://blog.hyperbridge.network/april-13-post-mortem/)
connects an exploit, proof-system guarantee, audit history, immediate fix, and
a newly added `OutOfBoundsLeaves` invariant. It also reports further critical
work discovered after the incident.

**Binder relevance:** findings and checks evolve. A receipt must be fresh and
narrow enough that later discoveries do not retroactively turn an old result
into a safety certificate.

### Why authoring the right property is irreducibly human

In the [DAIHard community-audit discussion](https://www.reddit.com/r/ethereum/comments/bppypq/),
the contract author argues that formal verification would not have exposed an
omitted assumption if the same conceptual blindness caused the property to be
left out. The proposed “poor man's audit” forces authors to enumerate inputs
and outcomes instead.

**Binder relevance:** this closely matches Binder's epistemic stance. Machines
can evaluate declared relationships; they cannot guarantee that the decisive
claim was declared.

## What the corpus already establishes

Across independent ecosystems, practitioners repeatedly separate:

1. semantic intent or invariant;
2. authority to state that intent;
3. tool observation and execution boundary;
4. exact source revision and dependencies;
5. build provenance or reproducibility;
6. deployed artifact identity; and
7. the narrower conclusion these facts warrant.

They also repeatedly warn against collapsing those into “safe,” “verified,” or
“audited.” This validates Binder's problem framing more strongly than it
validates demand for Binder itself.

The synthesized explanation is in [CONCERN_MAP.md](CONCERN_MAP.md).

## Additional conversations sampled

- [Foundry invariant assumptions](https://github.com/foundry-rs/foundry/issues/4190)
  and [fuzzed-array assumptions](https://github.com/foundry-rs/foundry/discussions/3948):
  the harness's input restrictions affect both feasibility and meaning.
- [Foundry execution-version mismatch](https://github.com/foundry-rs/foundry/issues/6228):
  compiler EVM target and fork execution environment were coupled even though
  real deployments can require them to differ.
- [Crytic's testing guidance](https://github.com/crytic/building-secure-contracts/blob/master/program-analysis/echidna/basic/common-testing-approaches.md):
  mocks and simplified models can introduce false positives or hide real
  failures, and must be maintained as code evolves.
- [OpenZeppelin Primitive audit](https://www.openzeppelin.com/news/primitive-audit):
  safe interaction depended on an atomic wrapper and usage discipline that was
  not enforced by the underlying primitive.
- [OpenZeppelin Endaoment audit](https://www.openzeppelin.com/news/endaoment-audit):
  remediation changed enough code that fixing the findings created a new
  re-audit question.
- [OpenZeppelin Futureswap audit](https://www.openzeppelin.com/news/futureswap-v2-audit):
  runtime bytecode matching did not cover creation code, initialization
  parameters, or linked-library details.
- [Stellar upgrade discussion](https://github.com/orgs/stellar/discussions/1670):
  state migration makes rollback different from merely restoring an old Wasm
  hash; metadata can itself create a false impression of upgradeability.
- [OpenZeppelin Contracts usage boundary](https://github.com/OpenZeppelin/openzeppelin-contracts):
  audited releases are distinguished from the moving branch, major storage
  layouts may be incompatible, and library reuse is not an application audit.
- [ERC-8004 Trustless Agents](https://ercs.ethereum.org/ERCS/erc-8004):
  on-chain identity and public feedback cannot guarantee that advertised agent
  capabilities work or are non-malicious; Sybil attacks remain possible.
- [ERC-8273 Attestation-Gated Agentic Actions](https://ercs.ethereum.org/ERCS/erc-8273):
  authorization can bind to a target, function, arguments, value, and nonce,
  while attestor trust and evidence meaning remain explicitly out of scope.
- [Solana wallet policies](https://platform.solana.com/docs/wallet-operations/policies):
  caller permission and wallet permission are separate; concrete operations,
  caps, allowlists, approval, and policy revisions constrain sign-time action.

## Saturation record

The search proceeded in four rounds:

1. audit remediation, source verification, and invariant discussions;
2. fuzzer assumptions, engine differences, and build provenance;
3. composition, reusable libraries, registries, and agent authorization; and
4. initialization, upgrades, storage migration, oracles, and badge semantics.

Rounds one and two added distinct concern categories. Round three added the
social-interface and concrete-action authorization categories. Round four
added examples but no new category: initialization and migration refine subject
identity and freshness; oracle changes refine dependency closure; badge
semantics refine false-confidence risk. This is qualitative saturation for the
product framing, not statistical prevalence.

## Questions not yet established publicly

The current corpus does not yet tell us reliably:

- how many active minutes the reconstruction consumes;
- who would author and maintain a Binder claim in a real team;
- whether a receipt changes an approve/reject/request-evidence decision;
- whether generating the base observation is normal work or extra burden;
- where the artifact should live in the pull-request and release workflow; or
- whether anyone will keep it enabled for a second change.

Do not assume these require interviews. Search public pull-request timelines,
audit pricing and remediation policies, claim/invariant ownership conventions,
tool adoption histories, and abandoned integrations first. A question becomes
eligible for conversation only after that search fails or yields contradictory
evidence.

## Product implications before interviews

- Integrate with Git commits, existing test engines, build attestations, source
  verification, and deployed-bytecode verification as typed dependencies.
- Keep behavioral warrants separate from provenance and reproducibility.
- Make `no-verdict`, scope, assumptions, engine, and stale evidence prominent.
- Prefer claim-specific receipts over popularity or aggregate trust scores.
- Support agent-proposed claims, but require an explicit human entitlement
  decision before they can warrant anything.
- Use the subscriptions remediation as the first case; use Stellar's threads to
  test the neighboring build/deployment interface.

## Research protocol

1. Read the first six conversations and annotate statements under the seven
   edges above.
2. Add only sources containing practitioner disagreement, a concrete failure,
   or an implemented design decision. Generic audit advice does not qualify.
3. Record disconfirming evidence: workflows where existing artifacts already
   preserve the complete chain, or where Binder would only duplicate work.
4. Maintain a question ledger with: question, searches attempted, best public
   evidence, confidence, contradiction, and whether a conversation is still
   necessary. The live ledger is [RESEARCH_LEDGER.md](RESEARCH_LEDGER.md).
5. Continue until another source no longer changes the interface map or product
   hypothesis. Record that saturation judgment rather than declaring research
   “done” by source count.
6. Build the strongest Binder artifact the corpus supports and inspect public
   reactions to comparable tools or proposals.
7. Conduct at most three targeted sessions only for remaining private facts or
   to observe a decision using the new artifact. Do not ask discovery questions
   already answered here.
