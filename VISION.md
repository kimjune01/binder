# Binder vision: verifiable agreements

## Vision

Two people who want to transact should be able to shop for an established
contract, parameterize it without changing its implementation, ask independent
agents to evaluate it and its dependencies, export one exact package, sign it
with their wallets, and enter an agreement that needs neither Binder nor
another contract-aware third party after activation.

```text
contract catalog
        ↓
versioned template + parameter schema
        ↓
party-specific parameters
        ↓
party A's agent       party B's agent
independently checks  independently checks
        \              /
         same rooted package
                 ↓
       export complete package
                 ↓
       both wallets sign its digest
                 ↓
     deterministic execution and settlement
```

The difficult human work happens before signing. The parties decide which
claims matter, which future observations are admissible, and what those
observations entitle the contract to do. Binder preserves those entitlement
edges and lets each agent populate the surrounding evidence graph on demand.

## Product boundary

Binder has two layers with different audiences:

- **Binder Core** is the epistemic tool: a CLI, portable format, and verifier
  for claims, roots, entitlement edges, checks, witnesses, and receipts.
- **Binder Agreements** is the human interface: shop, parameterize, review,
  export, sign, and follow an agreement assembled on Core.

Core is not specific to contracts. Agreements should not ask humans to operate
a knowledge graph. It turns the graph into decisions they can understand:
what each party must do, what each may receive, how funds can move, what can
change the outcome, and what remains unsupported.

Binder records and reproduces the parties' choices. It does not choose material
terms for them, judge whether the bargain is fair, provide legal advice, take
custody, underwrite either side, or adjudicate subjective outcomes. Wallets,
chains, external evidence sources, and any dispute mechanism remain separate.
Binder need not participate after signing.

These boundaries are connection points, not dead ends. Binder should link
directly to appropriate adjacent services and carry enough structured context
to make the handoff easy:

- wallets and signing tools for key-controlled assent;
- explorers and RPC providers for deployment and live state;
- auditors, simulators, fuzzers, and formal verifiers for additional evidence;
- reproducible-build and attestation systems for artifact identity;
- identity and credential providers when a party requires them;
- independent data publishers and relayers for external observations; and
- qualified legal or financial professionals when a party asks questions
  outside Binder's competence.

Binder should prefer open interfaces and deep links over recreating these
services. A party must remain free to choose a different provider or complete
the workflow manually.

```text
Binder establishes                 Binder does not establish
who accepted which exact package  whether the bargain is wise or fair
how the artifact was built        legality in every jurisdiction
which disclosed checks passed     absence of undiscovered defects
what can trigger each entitlement truth of unsupported external facts
what changed and who accepted it  custody, underwriting, or adjudication
```

## Agreement workflow

The human-facing path is deliberately linear:

1. **Shop:** choose a versioned template with its interface, dependencies,
   operating history, evidence, audits, and known limitations.
2. **Parameterize:** fill the parties, assets, amounts, deadlines, evidence
   sources, thresholds, exits, and dispute path without changing code when
   possible.
3. **Review independently:** each party's agent explains obligations,
   entitlements, loss conditions, external powers, and every departure from the
   reviewed template.
4. **Build and verify:** reproduce the artifact, run declared checks, and bind
   the parameters to an exact digest.
5. **Export:** produce a self-contained package that can be inspected,
   deployed, or archived without a Binder service.
6. **Sign:** each wallet signs the same manifest, including chain, parties,
   parameters, artifact digest, expiry, and activation conditions.
7. **Deploy or enter:** a party or independent service activates the package;
   the resulting address and artifact identity become new evidence.

Parameterization and modification are different trust events. Parameterization
instantiates a reviewed interface. A code change creates a fork and visibly
invalidates any evidence that depended on the unchanged implementation.

## The contract package

The unit of agreement is more than deployed code. It is a content-addressed
package containing:

- contract source, executable artifact, and public interface;
- exact dependencies and upgrade rules;
- participant wallet addresses and chain identity;
- business parameters and initial state;
- a parameter schema separating allowed configuration from code changes;
- allowed transitions, deadlines, defaults, and exit paths;
- behavioral claims and their authored entitlement rules;
- admissible external observation schemas and authenticity requirements;
- current evidence receipts and known guarantee boundaries; and
- the deterministic deployment or activation procedure.

The export should include source, executable artifact, ABI or IDL, parameters,
dependency lockfile, build recipe, evidence graph, receipts, attestations, and
signing manifest. No proprietary Binder reference may be necessary to inspect,
rebuild, deploy, or execute it.

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

## Portability

A useful contract package should not be trapped in one deployment, chain, or
implementation language. Like a model package that can target different
inference backends, an agreement should carry its semantics and evaluation
requirements into another compatible environment.

The layers have different portability:

```text
most portable
  intent and behavioral claims
  entitlement rules and failure paths
  interface and parameter schema
  fixtures, test vectors, and expected observations
  source implementation
  compiled artifact
  deployment address and live state
least portable
```

Porting among compatible runtimes may reuse source while changing chain ID,
dependency addresses, tokens, timing assumptions, and deployment identity.
Porting across runtimes may require a new implementation: an EVM escrow and a
Solana escrow can expose the same agreement while using different execution,
storage, and authorization models.

The agreement and its questions travel; unsupported answers do not. A port may
reuse claims, specifications, entitlement rules, and evaluation vectors. It
must regenerate build identity, dependency evidence, runtime observations,
deployment identity, and receipts for the target environment.

```text
portable semantic package
        ↓
target-specific implementation and dependencies
        ↓
target-specific evidence graph
        ↓
new rooted package for both parties to sign
```

Binder should make selective re-verification possible by distinguishing shared
semantic nodes from runtime-specific evidence. Popularity or assurance earned
by one deployment may help a user choose the package, but it must not silently
warrant a different port.

> Port the agreement and its questions; regenerate the evidence wherever it
> lands.

## The shopping experience

A credible contract catalog should let an agent answer:

- What does this contract claim to do?
- Which exact code and dependencies would we enter?
- Can any dependency or administrator change the behavior later?
- Which claims have current, replayable evidence?
- Which external observations can affect settlement?
- What happens on silence, disagreement, missing data, or timeout?
- Does the deployed artifact match the package we evaluated?
- Which parts of this package are portable to our target runtime, and which
  evidence must be regenerated?
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

Binder is the claim and evidence layer plus an optional agreement-formation
interface. It is not the wallet, custodian, underwriter, chain, oracle, identity
provider, adjudicator, legal adviser, or contract runtime.

For now, Binder is a public service rather than a value-capture strategy. Its
job is to lower the cost of reaching an inspectable agreement and leave behind
a portable public artifact. Useful completion may happen in somebody else's
wallet, explorer, verifier, deployment service, or professional workflow.
Possible business models can be evaluated after repeated use exists; they must
not create lock-in at the protocol or package layer.

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

Deferring these functions does not mean hiding them. The human interface may
explain when one is needed and link to independent providers without endorsing
one as part of Binder's guarantee.

Those products become relevant only if the narrow Binder workflow first proves
that explicit entitlement edges and independently replayable evidence improve
real smart-contract decisions.

## North star

> Frontload the judgment. Let independent agents verify the same agreement.
> Sign once, then execute without interpretation.
