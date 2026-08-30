# Binder roadmap

Binder's first roadmap hypothesis is that a curated public index can improve
contract understanding without waiting for users to populate it. That index is
research infrastructure, not yet a proven product. Binder's first product
remains the local typed receipt: an authored claim, exact subject,
discriminating observation, and independently replayable evidence.

This ordering resolves two different bootstrap problems:

- **Empty hall:** begin with consequential public work that already exists.
- **Evidence quality:** earn it through local receipts inside real review work,
  not through a busy-looking catalog.

```text
public artifacts
  → five curated case fixtures (research)
  → local typed receipts (first product)
  → existing-workflow publication
  → maintenance automation
  → attributed corrections
  → portable packages
  → community and market surfaces
```

Every stage is conditional. A later stage does not rescue a failed earlier
one.

## Principles

- Import existing public activity instead of asking people to enter an empty
  system.
- Label public fact, agent inference, human attestation, observation, and policy
  judgment separately.
- Link canonical sources and exact revisions; do not silently replace them with
  Binder summaries.
- Make missing, contradictory, and stale evidence visible.
- Prefer five excellent cases to a large shallow catalog.
- Keep verification local and receipts portable until hosted execution is
  clearly necessary.
- Authenticate concrete statements, never infer competence or authority from
  identity alone.
- Keep popularity separate from evidence quality.

## Phase 0A — build five manual case fixtures

### Outcome

A populated static research artifact that organizes existing activity and
reveals whether consequential contract cases share a useful structure.

### Cases

Choose one case from each class:

1. authorization or replay;
2. upgrade, migration, or persistent-state compatibility;
3. source/build/deployment identity;
4. invariant or proof-scope boundary; and
5. exploit or postmortem with a later remediation.

The Solana stale-cancellation remediation is the first fixture.

### Shape

Each case provides:

- exact public pointers;
- a narrow claim and responsible roles;
- affected, remediated, reviewed, built, and deployed identities where known;
- checks and witnesses already available;
- explicit assumptions and scope;
- source-backed versus inferred edges;
- missing or stale edges;
- a raw-source control packet;
- a curated case-page packet; and
- a fixed question set and answer key.

Use one small versioned JSON shape and a static renderer. Do not build accounts,
a database, crawler, search, collections, activity feed, global status, or
receipt requirement.

### Exit gate

- The fifth case adds no major primitive to the data shape.
- Every factual edge is traceable to a canonical source.
- Every inference is labeled and reversible.
- Every case has a predetermined consequential decision and known missing edge.

If public sources cannot support this, narrow Binder to the artifact class that
can be represented honestly.

## Phase 0B — validate the curated interface

### Outcome

Evidence that the joined view improves interpretation rather than merely
looking organized.

### Method

For each case compare:

1. the raw-source/control packet; and
2. the curated case-page packet.

Use self-contained asynchronous tasks first, with randomized or blinded order
where feasible. Measure decision correctness, claim and boundary comprehension,
missing-edge detection, reconstruction time, and sources opened.

Prefer observable public behavior: submitted answers, corrections,
reproduction logs, citations, and repository changes. Use at most three short
cold-reader sessions only when asynchronous evidence is sparse or when tacit
process and timing cannot be observed otherwise. Do not conduct broad discovery
interviews already answered by [SECONDARY_RESEARCH.md](SECONDARY_RESEARCH.md).

### Exit gate

Curated pages improve correct interpretation, missing-edge detection, or
reconstruction time in at least three of five cases without increasing
guarantee overclaiming.

If they do not, stop the hub framing. Keep the public corpus and local CLI;
better presentation or more pages cannot substitute for decision value.

## Phase 0C — publish the five-case research hub

### Outcome

A small public collection that tests whether target practitioners find the
joined view worth correcting, reproducing, citing, or sharing.

### Work

- Publish only the five validated pages and their JSON.
- Display canonical outbound links and provenance labels prominently.
- Accept corrections through existing public channels rather than building a
  discussion system.
- Record meaningful external actions, not vanity traffic.

### Exit gate

At least three relevant external actions: a substantive correction, attempted
reproduction, citation, share with technical context, source contribution, or
request for another case.

Do not expand to 10–20 cases, add search, or build a hosted registry before this
gate passes. Hub engagement validates the joining and legibility problem; it
does not validate demand for receipts.

## Phase 1 — ship the first product: local typed receipts

### Outcome

At least some organized claims become independently replayable rather than
remaining curated interpretations.

### Work

- Use one precise claim with an explicitly authored base/head oracle.
- Bind real Git revisions, relevant roots, engine, evidence kind, and command.
- Require structured `stood`, `refuted`, or `no-verdict` observations with
  witnesses.
- Preserve freshness, policy, guarantee boundary, receipt identity, and generic
  replay instructions.
- Attach receipts only to cases with a meaningful executable contrast.
- Keep build provenance and deployed identity as separate evidence kinds.

The graph remains implicit in the versioned receipt. Do not build a graph
database.

### Exit gate

- Independent replay succeeds on two public cases without private coordination.
- A receipt catches or clarifies a decision-relevant edge in at least one case.
- Setup takes under 15 active minutes when a suitable check exists.
- Clean-environment replay succeeds at least 80% of the time.
- Receivers can explain the claim and evidence boundary without author help and
  do not systematically read `warranted` as whole-contract safety.

## Phase 2 — distribute through existing workflows

### Outcome

Receipts are produced as a byproduct of work people already perform, solving
the supply problem before a registry exists.

### Work

- Release installable CLI binaries.
- Add a thin GitHub Action only after a real repository asks for automatic
  refresh.
- Publish receipts as workflow or release artifacts with a reviewer-sized job
  summary.
- Let case pages link receipt URLs and digests; do not create a mutable “latest
  safe” badge.

### Exit gate

- Two independent repositories keep Binder for a second consequential change.
- A receipt changes at least one real merge, request-evidence,
  audit-remediation, or release decision.
- Maintenance and CI costs remain acceptable to the repository owners.

If nobody retains it, do not build a registry/API to manufacture distribution.

## Phase 3 — automate maintenance, not ontology

### Outcome

Agents reduce repeated case-maintenance work without corrupting provenance.

### Work

- Build importers only for repeated work observed across the original fixtures:
  GitHub revisions and discussions, audit reports, test locations, build
  verification, deployments, and receipt publication.
- Produce proposed page diffs with citations.
- Detect link drift, code changes after audit, missing base observations,
  unlinked deployments, and stale roots.
- Require review for every proposed semantic edge.

The fixture JSON is already a small structured graph. Add a separate graph
model only after flat receipts demonstrably cause duplicated verification or
incorrect freshness decisions.

### Exit gate

- Automation materially reduces update effort across the original cases.
- It introduces zero unlabeled factual edges.
- At least three recurring examples show that selective dependency traversal or
  recomputation would save work before adding richer graph machinery.

## Phase 4 — add attributed corrections and entitlement

### Outcome

Qualified parties can accept, reject, narrow, challenge, or supersede concrete
edges without becoming general-purpose social users.

### Work

- Carry author and approver references in receipts from the beginning using Git
  and pull-request provenance.
- Reuse GitHub, wallet signatures, or in-toto mechanisms when proof of identity
  becomes decision-relevant; do not invent Binder identity infrastructure.
- Support claim-specific actions: authorship, audit-scope confirmation,
  deployment binding, reproduction, challenge, and supersession.
- Preserve history rather than rewriting prior evidence.
- Separate identity control from qualification and authority, which remain
  explicit policy claims.

### Exit gate

- At least two real decisions were blocked specifically because authorship
  authenticity—not claim quality—was unresolved before custom authentication
  is built.
- At least three qualified external contributors correct or authorize real
  edges, and one returns for a later revision or second case.

## Phase 5 — package contracts for inspection and reuse

### Outcome

A portable versioned package connects intent, interface, implementation roots,
dependencies, evidence, builds, deployments, initialization, and limitations.

### Work

- Define immutable versions and mutable development channels.
- Provide CLI/API operations for search, inspect, dependency traversal,
  verification, comparison, download, and fork.
- Separate immutable implementation versions from typed parameter schemas so
  ordinary agreement formation does not require a code fork.
- Export a self-contained package containing source, artifact, ABI or IDL,
  parameters, dependency lockfile, build recipe, evidence, attestations, and a
  signing manifest.
- Record supported chains, interfaces, dependency constraints, governance,
  upgrades, licenses, and portability notes.
- Explicitly identify which semantic claims travel across a fork or port and
  which runtime evidence must be regenerated.

### Exit gate

Two independent ports or reuses show that semantic artifacts travel while
target-specific evidence is correctly invalidated and regenerated, with a
measurable reduction in re-review work.

Without demonstrated reuse, keep claims revision-specific and defer packages.

## Phase 5B — validate the human agreement interface

### Outcome

Two parties can shop, parameterize, independently review, export, and sign the
same package without Binder taking custody or becoming a runtime dependency.

### Work

- Render obligations, entitlements, loss conditions, external powers, missing
  evidence, and code changes as human decisions rather than graph operations.
- Give each party an independent agent-readable review path.
- Bind both signatures to the exact implementation, parameters, chain, parties,
  expiry, and activation conditions.
- Keep deployment external and verify its resulting artifact identity.
- Treat any implementation change as a fork whose affected evidence must be
  regenerated; do not present it as parameterization.

### Exit gate

- Two independent pairs reach the same signed package without author help.
- Each participant can accurately state their obligations, entitlements, and
  principal loss conditions before signing.
- Export, rebuild, and deployment work without a Binder account or service.
- No pilot requires Binder custody, underwriting, legal judgment, or runtime
  adjudication.

If people value inspection but do not proceed to a shared package, retain Core
and do not build signing or deployment orchestration.

## Phase 6 — community and market surfaces

### Outcome

Existing participation becomes discoverable and coordinatable without becoming
counterfeit trust.

### Work

- Surface independent reproductions, downstream uses, maintained forks,
  attributed attestations, open gaps, and freshness.
- Add following, discussion, contribution queues, and organization pages only
  where existing activity needs them.
- Rank search using transparent relevance and factual activity signals;
  popularity never warrants a behavioral claim.
- Prefer interoperable manifests and independent catalogs before a
  Binder-operated marketplace.
- Explore paid verification, remediation, maintenance, deployment, or support
  only when people already request those transactions.

### Exit gate

Dozens of maintained packages, repeated independent demand, external
evaluators, version-specific reuse, and a credible curation/dispute/liability
model. Marketplace features must organize activity that exists beforehand.

## Assumptions and cheapest tests

| Assumption | Cheapest test | Failure response |
| --- | --- | --- |
| Joined case pages beat raw links | Five control/case tasks with answer keys | Stop the hub framing |
| Public artifacts can seed useful cases | Manually complete five case classes | Narrow the supported artifact class |
| Missing edges are decision-relevant | Measure detection and resulting decisions | Focus pages on supported conclusions |
| Receipts improve more than presentation | Replay two cases and compare conclusions | Keep curation; do not force receipts |
| Workflow artifacts create durable supply | Observe second use in two repositories | Do not build a registry |
| Agents reduce maintenance cost safely | Compare reviewed agent diffs with hand updates | Keep manual editorial control |
| Authentication blocks real decisions | Record identity-specific rejection | Reuse ordinary Git provenance |
| Claims travel across reuse or ports | Observe two real adaptations | Keep cases revision-specific |
| Community signals aid discovery | Observe existing repeated activity first | Do not add social surfaces |

## Immediate sequence

1. Define the minimal case JSON schema using the subscriptions case.
2. Add its control packet, curated packet, fixed questions, and answer key.
3. Manually author four cases from the saturated public corpus.
4. Render the five fixtures statically and validate them with self-contained
   tasks.
5. Publish only after the comprehension gate passes.
6. Attach real Binder receipts to executable cases and test independent replay.
7. Automate only repeated work revealed by maintenance history.

## Explicitly deferred

- A blank user-submitted registry or global graph database.
- Search, activity feeds, collections, and accounts for five fixtures.
- Fake profiles, comments, stars, reproductions, or usage.
- A universal safety, audit, reputation, or popularity score.
- Hosted execution of untrusted contract code.
- Custom identity, credential, or signing infrastructure.
- Automatic cross-chain contract translation.
- Wallet execution, billing, token incentives, governance, and moderation.
