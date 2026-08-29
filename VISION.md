# Binder vision: verifiable agreements

## Vision

Two people who want to transact should be able to shop for a contract, ask
independent agents to evaluate it and its dependencies, agree on one exact
package, sign it with their wallets, and enter an agreement that needs no
contract-aware third party after activation.

```text
contract marketplace
        ↓
candidate contract package
        ↓
party A's agent       party B's agent
independently checks  independently checks
        \              /
         same rooted package
                 ↓
       both wallets sign its digest
                 ↓
     deterministic execution and settlement
```

The difficult human work happens before signing. The parties decide which
claims matter, which future observations are admissible, and what those
observations entitle the contract to do. Binder preserves those entitlement
edges and lets each agent populate the surrounding evidence graph on demand.

## The contract package

The unit of agreement is more than deployed code. It is a content-addressed
package containing:

- contract source, executable artifact, and public interface;
- exact dependencies and upgrade rules;
- participant wallet addresses and chain identity;
- business parameters and initial state;
- allowed transitions, deadlines, defaults, and exit paths;
- behavioral claims and their authored entitlement rules;
- admissible external observation schemas and authenticity requirements;
- current evidence receipts and known guarantee boundaries; and
- the deterministic deployment or activation procedure.

Both parties sign the same package digest. A signature proves control of the
named wallet; any claim about the legal or social identity behind that wallet
must be established separately before it is included in the package.

## Frontload judgment, not necessarily facts

An agreement may depend on facts that do not exist at sign-time. What must be
fixed at sign-time is how those facts will be recognized and interpreted.

For a weather contract, the parties might commit to:

```text
source:       named weather service
station:      YVR
measurement:  daily rainfall
date:         2027-01-01
authenticity: source signature or verifiable web proof
threshold:    greater than 10 mm
missing data: refund both parties after deadline
```

The weather service need not know about the agreement or exercise discretion
over it. It publishes a general observation. Any relayer may transport an
authenticatable observation to the chain; the contract applies the rule that
the parties already signed.

```text
independent observation
        ↓ authenticated transport
precommitted entitlement rule
        ↓
deterministic transition
```

If a source does not authenticate its output, the reporter becomes trusted.
If a condition is subjective or cannot be authenticated, it cannot be settled
without a later participant decision or adjudicator. Binder should expose that
boundary rather than disguise it.

## Onchain facts still require interpretation

Onchain prices are derived observations, not primitive facts. A useful price
claim must bind the market, pair, quote asset, time window, aggregation method,
liquidity conditions, settlement block, and failure behavior.

```text
pool state and trades
        ↓
signed price definition
        ↓
typed price witness
        ↓
settlement rule
```

An API may provide a convenient view, but an agent should be able to reproduce
the result from rooted chain state when that state is the declared authority.
Calling an ETH/USDC ratio a dollar price additionally depends on a separate
claim that USDC represents the dollar.

## Composition

Smart contracts compose execution. Verifiable agreements must also compose
the claims that justify using those components.

```text
settlement contract
  ├── escrow transition
  ├── token behavior
  ├── price definition
  ├── observation authenticity
  └── timeout behavior
```

Evidence for each dependency does not automatically establish the parent
claim. A human-authored entitlement edge must state why the component claims
jointly warrant the agreement-level claim. Agents may then resolve identities,
reuse current receipts, execute missing checks, and surface unsupported edges.

## The shopping experience

A credible contract marketplace should let an agent answer:

- What does this contract claim to do?
- Which exact code and dependencies would we enter?
- Can any dependency or administrator change the behavior later?
- Which claims have current, replayable evidence?
- Which external observations can affect settlement?
- What happens on silence, disagreement, missing data, or timeout?
- Does the deployed artifact match the package we evaluated?
- What would invalidate the current evidence?

Each party may use different agents, evidence policies, and private context.
They need not trust the same evaluator. They must converge only on the exact
package they are willing to sign.

## Adoption hurdles

The technical primitives are largely available. Adoption depends on reducing
the cost of reaching informed agreement without hiding the remaining trust.

### Public is not legible

Source code, bytecode, state, and transaction history may all be public while
their consequences remain inaccessible to ordinary users. Agents can surface
claims, material terms, dependency powers, failure paths, and unsupported
assumptions. That explanation is useful only while every conclusion stays
connected to inspectable evidence; fluent reassurance is not verification.

### The agreement is fragmented

Source lives in repositories, packages in registries, deployments in explorers,
interfaces in ABIs or IDLs, audits in reports, and operational control in
multisigs or upgrade authorities. A user cannot shop confidently until these
pieces resolve to one versioned package. Project reputation must never silently
cover a different deployment, dependency set, or upgrade.

### Trust has a cold start

New contract packages lack operating history, while established packages can
make direct inspection unnecessary for low-stakes users. Popularity is valid
social evidence, but not proof of correctness. Useful signals attach to an
exact version and remain separate:

- unique funded counterparties and repeat use;
- time in operation and economic value exposed;
- independent evidence replays and evaluator diversity;
- current audits and unresolved incidents; and
- dependency, configuration, and upgrade recency.

A single universal trust score would conceal these distinctions and invite
gaming. Each agent should apply a stakes-dependent policy to the evidence
vector. A small transaction may inherit community evidence; a consequential
one may trigger direct replay or professional review.

```text
convenience             agent explanation        independent verification
social evidence    →    claims and risks     →    selective graph replay
low stakes              medium stakes             high stakes
```

### Reuse must change the economics

The system is valuable only if it lowers recurring transaction costs. Human
experts may author and review a reusable contract, its entitlement rules, and
its failure paths once. Subsequent parties should verify only their parameters,
wallets, changed dependencies, and final package digest. Legal or professional
help can then concentrate on identity, consent, jurisdiction, or novel terms
rather than repeatedly reconstructing deterministic behavior.

### Some boundaries remain external

A wallet signature proves key control, not legal identity, capacity, or freedom
from coercion. Some jurisdictions require recognized formalities or remedies.
External observations also need authenticated publication and explicit
missing-data behavior. Verifiable agreements can eliminate contract-aware
discretion after signing; they cannot make every social or legal fact
endogenous to a chain.

### Agreement UX is still consequential

Both parties must understand that they are signing the same package, see the
terms their agents consider material, and retain safe control of their wallets.
Version hashes and evidence graphs are insufficient if the signing interface
encourages blind approval or obscures irreversible outcomes.

## Binder's role

Binder is the claim and evidence layer, not the marketplace, wallet, chain,
oracle, identity provider, or contract runtime.

Its long-term responsibility is to preserve the path:

```text
human intent
  → authored entitlement edges
  → rooted contract and dependency graph
  → independently materialized evidence
  → dual acceptance of one package
  → signatures
  → deterministic execution
```

The current product wedge—evidence-carrying software changes—earns the core
primitives required for this vision: exact subjects, explicit warrant rules,
typed witnesses, dependency invalidation, content-addressed receipts, and
independent replay.

## What this vision does not imply now

This is not a feature roadmap. Binder should not yet build:

- a contract marketplace or deployment service;
- wallet custody or transaction signing;
- legal identity or credential issuance;
- a proprietary oracle network;
- automated negotiation or dispute resolution;
- a universal safety score;
- a global canon of trusted contracts; or
- automatic judgment about which claims humans should accept.

Those products become relevant only if the narrow Binder workflow first proves
that explicit entitlement edges and independently replayable evidence improve
real smart-contract decisions.

## North star

> Frontload the judgment. Let independent agents verify the same agreement.
> Sign once, then execute without interpretation.
