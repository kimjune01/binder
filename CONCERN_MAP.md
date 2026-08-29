# What smart-contract practitioners are worried about

The public conversation is not mainly “how do we prove code correct?” It is
“how do we stop one narrow fact from being mistaken for a much larger promise?”

Practitioners have built many strong deterministic mechanisms: signatures,
hashes, tests, fuzzers, proofs, reproducible builds, attestations, bytecode
matching, simulations, and on-chain execution. Their recurring difficulty is
preserving the meaning between those mechanisms.

## 1. The claim can be wrong or missing

A test engine can evaluate a property, but it cannot decide that the property
captures what the humans actually care about. If an economic or sequencing
assumption was never written down, a perfect proof of the written properties
can coexist with a broken contract.

This is why auditors ask teams to explain intent, trust assumptions, privileged
roles, and expected interactions before relying on tools. It is also why
invariant authors debate assumptions and harness design: a mock, input filter,
or model can hide a real issue or manufacture an impossible one.

Their concern is:

> Did we test the right proposition, or merely test a proposition well?

## 2. Someone must be entitled to state the claim

Library maintainers cannot promise that every application uses a primitive
safely. A fuzzer cannot authorize an economic invariant. A build service cannot
declare business intent. An agent can propose a claim, but its fluency does not
give it authority.

OpenZeppelin discussions repeatedly allocate responsibility among the library,
API, documentation, wrapper contract, and application developer. Agent
standards do the same among agent identity, attestor, integrating application,
wallet policy, and end user.

Their concern is:

> Who is qualified to say that this observation is enough for this decision?

## 3. A green result may not discriminate

A regression test passing on the candidate does not show it would have exposed
the old bug. A fuzzer that finds no counterexample has explored a bounded
campaign, not proved absence. A command exiting successfully may mean only that
the tool ran.

The strongest remediation practices isolate one finding, one fix, and one test.
Yet public audit trails commonly preserve a passing fix-side result without the
same observation against the exact audited baseline.

Their concern is:

> Did this check distinguish the fix from the broken state?

## 4. The engine is part of the evidence

LiteSVM, Mollusk, local EVMs, fork tests, and on-chain execution can disagree.
Compiler target and execution target can be accidentally coupled. Mocks can
behave unlike live dependencies. Fuzz assumptions change the reachable state
space.

The result therefore belongs to an engine, version, configuration, and model.
Moving the result elsewhere is a new inference, not clerical reuse.

Their concern is:

> Where was this observed, and what makes that environment representative?

## 5. The subject is larger than a source commit

Runtime bytecode matching source is useful, but practitioners point out what it
can omit: creation code, constructor arguments, linked libraries,
initialization, proxy state, storage layout, migration steps, governance
configuration, and wrapper contracts.

A particularly revealing audit concluded that a primitive was safe only when
used through an atomic wrapper. The code was public; the safe unit of behavior
was the composition, not the primitive alone.

Their concern is:

> What exact system does this fact describe?

## 6. Provenance, reproducibility, and faithfulness differ

An attestation can establish that a named pipeline produced bytes. A
reproducible build can establish that source and environment reproduce bytes.
Neither alone establishes that the compiler was benign or that the source is
safe. A hostile compiler image can reproducibly inject behavior.

The Stellar verification discussions are unusually explicit about keeping
these trust claims separate and exposing multiple signals rather than one
“verified” badge.

Their concern is:

> Which link did we verify, and which parties or tools are still trusted?

## 7. Evidence expires when its dependency closure changes

Audits name commits and scope because code keeps moving. A small fix can expand
into a refactor large enough to justify another audit. A new dependency,
compiler, oracle, integration, role assignment, or accepted token can invalidate
an old conclusion without changing the original test.

Upgradeable contracts make this concrete: logic and persistent state evolve
separately. Returning to an old Wasm hash does not necessarily restore old
state after a migration. “Rollback” may itself be another upgrade.

Their concern is:

> What change makes this evidence stale?

## 8. Composition multiplies assumptions

Contracts behave like public APIs, which makes reuse powerful. It also means a
component inherits assumptions about tokens, oracles, callbacks, governance,
transaction ordering, and downstream contracts. Individual audits do not
automatically compose into a system audit.

OpenZeppelin warns that using its audited library is not a substitute for
auditing the application. Its release process distinguishes audited releases
from the moving development branch, and major versions may be storage-layout
incompatible.

Their concern is:

> When I reuse a trusted component, which guarantees travel with it and which
> must be re-established in my composition?

## 9. A badge creates social risk

Most users will not rebuild bytecode, replay fuzz campaigns, or inspect audit
scope. Wallets and explorers will consume APIs and display summaries. A single
badge is attractive precisely because it hides complexity, which makes it easy
to turn “source matched” into “safe,” “audited once” into “still audited,” or
“agent registered” into “agent trustworthy.”

Agent reputation adds Sybil manipulation, incomparable scores, and feedback
that may not be grounded in verified interactions. Identity registries openly
acknowledge that they cannot prove advertised capability or benign behavior.

Their concern is:

> What false confidence will the interface manufacture at scale?

## 10. Authorization must bind to the concrete act

The emerging agent infrastructure is converging on narrow, sign-time controls:
exact target, function, arguments, nonce, spend caps, allowlists, time window,
and immutable policy revision. A broad capability or wallet key is too much
authority, and a reusable approval invites replay.

ERC-8273 deliberately separates coarse capability from an action digest bound
to one concrete call. Solana wallet policies distinguish caller permissions
from what the wallet may do and record the active policy revision.

Their concern is:

> Did this human authorize this exact action under this exact policy, or merely
> authorize something vaguely similar?

## The common structure

These concerns reduce to a graph of narrow edges:

```text
human authority
    ↓ authors
claim + assumptions
    ↓ evaluated by
check + engine + configuration
    ↓ observed on
source + dependencies + state
    ↓ built by
toolchain + provenance
    ↓ corresponds to
deployed code + initialization + policy
    ↓ permits
one concrete action
```

Every arrow is a separate claim. Existing systems are often strong at the
nodes and ambiguous at the arrows.

## What this means for Binder

Binder should not become another auditor, fuzzer, prover, build service,
registry, reputation score, wallet, or chain. It should preserve typed edges
among their outputs:

- who authored a claim and under what authority;
- what exact observations warrant it;
- which revisions, engines, dependencies, and state were involved;
- what the evidence does not establish;
- what changes make it stale; and
- which concrete action, if any, the warranted claim is allowed to authorize.

The smallest useful product remains a CLI receipt for one claim and one
base/head contrast. The longer-term opportunity is portability: the claim can
travel across tools and chains without pretending that its evidence travels
farther than it does.
