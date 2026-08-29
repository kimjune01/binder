# Binder product framing and scope

## Product definition

**Binder lets humans frontload judgment into verifiable claims, then materializes the evidence graph on command.**

A human states the consequential judgment once: which observations would entitle a specific claim, about which subject, under which assumptions. Binder preserves that entitlement edge, invokes the existing deterministic tools around it, and records the resulting witnesses and receipts. A receiver can inspect or replay the same graph without trusting the producer's narrative or reconstructing it from a diff and green CI run.

```text
human judgment
  authors: observations ──warrant──▶ claim
                              |
                              v
Binder on command
  resolves subject → roots → checks → executions → witnesses → policy
```

Binder does not automate away human judgment. It moves judgment to the point where it can be made explicit, preserved, and reused; the mechanical work after that becomes smooth and repeatable.

The product wedge is an **evidence-carrying change**:

```text
agent or developer
  proposes code + claim + replayable check
                         |
                         v
               reviewer or receiving agent
  replays or inspects the same rooted check
                         |
                         v
          accepts, rejects, or requests evidence
```

Binder is the software-change application of the verification primitive described in *Verifiable Knowledge*: preserve the authored entitlement edge, then populate the verifiable graph needed to evaluate it.

## Problem

Smart-contract development already contains many deterministic components: Git revisions, compilers, test runners, simulators, fuzzers, provers, build hashes, and reproducible chain execution. The friction and uncertainty live between them. A passing check does not say which claim it supports, why its observation warrants that claim, whether it detects the original defect, which exact subject it covered, or whether the deployed artifact inherits the result.

Those handoffs are reconstructed repeatedly from issue prose, test names, CI configuration, logs, audit reports, and reviewer memory. Agents widen this coordination gap by producing code and checks faster than humans can inspect their correspondence. Provenance can establish what ran; it does not preserve why the resulting observation entitles the claim.

## Initial user and decision

The initial user is a maintainer, auditor, or receiving agent reviewing a consequential smart-contract change. Smart contracts are the first domain because they already depend on verifiable execution, yet intent, specifications, tests, audit findings, source builds, and deployed artifacts remain connected by informal human handoffs.

The decision Binder supports is narrow:

> Does the current rooted evidence satisfy the authored warrant rule for this specific claim and change?

Binder does not answer whether the entire program is safe, whether the change should ship for business reasons, or whether the authored claim is complete.

## Core product contract

Every Binder claim must carry:

| Primitive | Product meaning |
| --- | --- |
| Claim | A precise behavior the change is supposed to establish or preserve |
| Subject | The exact base and candidate revisions or artifacts being compared |
| Check | An executable procedure whose result bears on the claim |
| Oracle | The expected base and candidate outcomes, including what counts as refutation |
| Entitlement edge | The authored rule stating which typed observations warrant the claim |
| Roots | Source, specification, fixture, toolchain, environment, and artifact inputs |
| Kill condition | A failed check or changed root that removes the claim's current entitlement |
| Receipt | The versioned, content-addressed observation produced by running the check |
| Replay | Instructions and inputs sufficient for a receiver to rerun the observation |
| Freshness | Whether the receipt addresses the exact current subject and roots |
| Policy | Whether the available current evidence crosses the action threshold for this context |

The entitlement edge is the irreducible human contribution. Binder may help compile it from acceptance criteria, an audit finding, or a contract specification, but must never silently invent or strengthen it. `WARRANTED` is a policy decision: the required current evidence crosses the configured threshold for action. It is not a truth value, whole-program safety, or final truth.

## Evidence graph

A Binder claim declares only its immediate dependencies. On `verify`, Binder resolves and materializes the relevant local graph:

```text
claim instance
  ├── authored entitlement rule
  ├── exact subject
  ├── rooted specification and fixtures
  └── required observations
        └── evidence producers
              └── executions
                    └── typed witnesses
                          └── receipt and policy evaluation
```

Some edges are authored judgments; others are mechanically derived identities or execution results. Receipts must preserve that distinction and attribute authored edges. Identical current nodes may be reused; changed roots create a new claim instance rather than mutating the old result.

This is not initially a global knowledge graph or shared canon. It is the smallest claim-specific graph needed to make a review decision, populated locally and on demand.

## Epistemic model

Binder separates three questions that ordinary CI collapses:

```text
Did the check execute?       execution
What did it observe?         observation
May we act on that evidence? policy
```

### Execution outcome

The runner records whether the check machinery completed:

```text
completed | error | timeout
```

A compiler failure, missing tool, malformed fixture, or timeout is an execution problem. None is a refutation of the semantic claim.

### Epistemic observation

A completed claim-specific oracle returns:

```text
stood | refuted | no-verdict
```

- **Stood:** the claim survived a check that could have refuted it.
- **Refuted:** the check ran and produced the declared counter-observation.
- **No verdict:** no claim-specific observation was reached or the check cannot settle the claim.

`stood` and `refuted` are sibling verdicts. `no-verdict` is the untested or untestable state. A process exit code cannot supply this classification by itself.

Every `stood` or `refuted` observation must carry a typed witness. For example:

```json
{
  "observation": "refuted",
  "witness": {
    "vault_before": 100,
    "vault_after": 90,
    "recipient_before": 5,
    "recipient_after": 15
  }
}
```

### Freshness and claim instances

Freshness is not another truth value. A receipt addresses one claim instance: statement plus exact subject and roots. If a rooted input changes, the old observation remains historical evidence about the old instance; the new instance has no current verdict.

```text
claim at commit A  stood
claim at commit B  no current verdict; relevant root changed
```

Binder must never rewrite an old verdict as false or erase the subject for which it stood.

### Policy warrant

A policy evaluates current observations against a stakes-dependent action threshold:

```text
warranted | not-warranted
```

The same evidence may warrant a low-stakes merge and fail to warrant a mainnet deployment. Policy records which evidence kinds are required; it does not transform observations into stronger epistemic types.

### Evidence types

Formal and empirical evidence stay distinct:

| Evidence kind | Entitlement supplied | Example rendering |
| --- | --- | --- |
| Formal | Closed relative to declared axioms and specifications | `PROVED transition kernel relative to spec` |
| Empirical | Survived a bounded world-facing trial | `OBSERVED compiled program under fixture/runtime` |

A policy may require both, but Binder must not render either as an undifferentiated `PASS` or imply that formal consistency establishes runtime behavior.

### Target receipt model

```text
ClaimInstance
  statement
  subject: base revision, candidate revision, artifacts
  roots
  evidence kind
  check
  claim-specific oracle
  kill conditions

CheckResult
  execution outcome
  epistemic observation
  typed witness
  receipt identity

PolicyEvaluation
  required evidence kinds
  action context
  warranted decision
```

The typed demo implements this separation for empirical base/candidate checks. Production work must extend it without collapsing execution, observation, authored entitlement, freshness, and policy back into one status.

## Why this is more than CI

Ordinary CI asks:

> Did the configured commands pass on the current branch?

Binder asks:

> Does this check distinguish the candidate from the state where the claimed defect exists, and does its evidence still apply to the exact subject under review?

The differentiated operations are therefore:

1. **Contrast:** run the same claim-sensitive check against base and candidate.
2. **Bind:** attach the observation to all declared roots and relevant artifacts.
3. **Invalidate:** withdraw entitlement when a rooted input changes.
4. **Transmit:** package the claim and receipt so another party can inspect or replay it.

Running tests is commodity infrastructure. Preserving the entitlement edge while materializing the claim–evidence–subject graph is the product.

## Interface strategy

Binder should add epistemic semantics to interfaces teams already use. It should not ask authors or reviewers to move into a parallel knowledge system.

```text
issue or PR
  supplies the claim
        |
existing test runner
  supplies the check
        |
Git base/head commits
  supply the subject
        |
GitHub required check
  supplies the merge decision
        |
job summary + workflow artifact
  supply explanation and replay
        |
deployment protection + verified build
  later connect the claim to what ships
```

### Native interface: an agent-first CLI

The CLI is Binder's product interface. Agents already know how to invoke commands, provide files and revisions, inspect exit codes, and pass structured output to the next tool. GitHub Actions, editor integrations, MCP tools, and hosted services should be thin adapters over the same CLI contract rather than independent implementations of Binder semantics.

Agent-friendly means:

- **Non-interactive:** every operation completes without prompts; consequential choices are explicit arguments or files.
- **Structured:** `--format json` covers execution outcome, epistemic observation, freshness, policy decision, and operational errors with a versioned schema.
- **Stream-safe:** stdout contains only the requested result; diagnostics and trial output go to stderr.
- **Stable:** documented exit codes and field meanings do not depend on terminal wording.
- **Deterministic:** identical rooted inputs produce identical receipt identity and machine output, excluding explicitly separated run metadata.
- **Composable:** commands accept repository paths, Git revisions, claim files, and receipt digests rather than hidden session state.
- **Inspectable:** an agent can ask what Binder will run, which roots it resolved, and which artifacts it expects before executing trials.
- **Bounded:** timeouts, replay cost, and unsupported states are explicit rather than appearing as a generic failure or hanging forever.
- **Local-first:** verification and receipt validation work without a Binder account or hosted service.

The intended command surface is small:

```text
binder verify   <claim> --base <rev> --head <rev> [--format json]
binder status   <claim>                          [--format json]
binder inspect  <claim>                          [--format json]
binder replay   <receipt>                        [--format json]
```

`inspect` resolves the subject, commands, roots, artifacts, and estimated replay work without running evidence producers. `replay` consumes an existing receipt or bundle and re-derives its verdict. Claim compilation or `init` remains undecided until validation establishes the right authoring source.

Human terminal reports are a rendering of the machine contract. They are not a separate semantic interface.

### Claim authoring: issues and pull requests

The human-facing claim should begin in the issue acceptance criteria, audit finding, bug report, or pull-request description where teams already state intent. A Binder manifest is the compiled machine representation of that intent, not necessarily a document a maintainer writes by hand.

The initial convention can be as small as:

```markdown
Claim: A rejected withdrawal does not change balances.
Fixes: #142
```

Binder or the producing agent may turn this into a versioned repository artifact. Validation must determine whether teams prefer to commit that artifact, generate it in CI, or maintain a small explicit manifest.

### Checks: existing test runners

The falsifiable check remains an ordinary `cargo test`, Mollusk, pytest, Jest, Certora, or other project command. Binder supplies revision contrast, expected outcomes, rooted inputs, and receipt semantics around the command. It must not introduce a testing language.

### Subject identity: Git

The subject is the actual pull-request base SHA and candidate SHA, or the merge-group SHA where repositories use a merge queue. Binder must adopt the repository host's revision semantics rather than use symbolic labels such as `vulnerable` and `fixed`.

When a new regression test is part of the candidate, Binder needs a defined test-only-patch mechanism so the same check can run meaningfully against the base revision.

### Review and gating: GitHub checks

The primary product surface is a required status check beside existing build and test checks. Its detailed explanation belongs in the GitHub job summary first; a custom Check Run or GitHub App is justified only if annotations or richer interaction prove necessary.

```text
PASS  build
PASS  unit-tests
PASS  binder/claims
```

The check status gates merge. The summary communicates the claim, contrast, evidence boundary, freshness, and replay cost. Reviewers should not need to visit a Binder dashboard.

### Replay transport: workflow artifacts

The replay bundle should initially ship as a normal GitHub Actions artifact. This fits existing log and test-result workflows but has finite retention, so durable receipts may later attach to a release or content-addressed object store. Artifact upload is the adoption interface; permanent storage is a later lifecycle decision.

### Provenance: existing attestations

GitHub artifact attestations, in-toto, or SLSA should establish that a workflow produced a particular receipt or binary. Binder should not replace them with a bespoke signing layer.

The distinction is:

- provenance attestation: this workflow produced this artifact;
- Binder receipt: this observation supports this claim under these roots.

### Release and deployment: existing gates

After the pull-request workflow is validated, the same claim status can feed a GitHub deployment protection rule. For Solana, verified-build metadata supplies the source-commit-to-deployed-ELF edge. Binder supplies the semantic-claim-to-ELF edge above it.

```text
claim warranted for ELF digest X
                 +
deployed program matches ELF digest X
                 =
claim evidence reaches the deployed subject
```

Program metadata and explorers are later discovery interfaces. They should consume Binder receipts only after repositories produce useful ones.

### Interfaces to avoid initially

- **SARIF:** it is finding- and source-location-oriented, while Binder tracks standing semantic claims.
- **Standalone dashboard:** it duplicates the pull-request surface before demand exists.
- **Custom test DSL:** it competes with the evidence producers Binder should compose.
- **On-chain claim registry:** it adds permanence and governance before the claim contract is validated.
- **Bespoke agent protocol:** agents can invoke the CLI and consume JSON through existing tool access.

## v1 scope: evidence-carrying changes

### Must have

- A versioned machine claim compiled from an issue, pull request, audit finding, or small explicit manifest.
- An explicit, attributable entitlement rule that Binder preserves rather than infers from tool success.
- Real repository base and candidate revision identity.
- Explicit expected outcomes and claim-specific observations.
- Typed execution, observation, freshness, and policy fields; no semantic verdict inferred from a process exit code alone.
- Typed witnesses for every stood or refuted observation.
- Evidence kinds that preserve the formal/empirical boundary.
- Existing project commands executed locally or in the repository owner's CI.
- Versioned, deterministic, content-addressed receipts.
- Declared source, toolchain, fixture, environment, and artifact roots.
- Accurate invalidation when a relevant root changes.
- Human-readable and JSON reports with stable exit behavior.
- Non-interactive `verify`, `status`, `inspect`, and `replay` operations whose JSON covers all terminal outcomes.
- A generic replay contract that does not depend on Binder's demo scripts.
- An official GitHub Action that reports a required check, writes the job summary, and uploads the replay bundle as a workflow artifact.
- A receiver can validate and replay without a hosted Binder service.

### Should have after validation

- `binder init` for turning an existing regression check into a claim.
- Released CLI binaries.
- Review output showing claim, boundary, base/candidate contrast, freshness, and replay cost.
- Adapters that normalize observations from existing tools without collapsing their guarantees.

### Explicitly not v1

- Writing or deploying a smart contract.
- Generating arbitrary invariants automatically.
- Replacing audits, formal verifiers, fuzzers, simulators, or human review.
- Declaring an entire program secure.
- Hosted execution of arbitrary repository commands.
- A universal security score.
- A wallet, explorer, dashboard, marketplace, or billing system.
- A global hypothesis graph or shared canon; v1 materializes only local claim-specific evidence graphs.
- Social identity, reputation, staking, or governance mechanisms.

These exclusions preserve the falsifiable wedge: if evidence-carrying changes do not improve real review decisions, broader infrastructure has no product foundation.

## Relationship to verifiable knowledge

Binder implements a deliberately small, operational part of the full protocol.

### Implemented or directly in scope

- replayable check;
- an explicit, attributable entitlement edge;
- on-demand materialization of a claim-specific evidence graph;
- rooted provenance;
- system-facing kill condition;
- replay by a receiver who need not trust the author;
- a three-way practical distinction among supported, refuted, and unsupported evidence, plus staleness;
- admission to a local record only after checks run.

### Future protocol layers

- graph expansion through claim-to-claim provenance and downstream refutation;
- typed terminal witnesses for empirical roots;
- engineered independence across model families, operators, or evidence engines;
- declared replay cost and selective verification;
- inheritance across repositories and organizations;
- a shared canon of standing, revocable claims.

The product must earn the right to build these layers through demonstrated coordination value. They are a direction, not v1 requirements.

## Relationship to assurance cases

Binder is a lightweight executable assurance case specialized for software changes. Assurance cases contribute the claim–argument–evidence structure and continuous assurance contributes change-triggered invalidation.

Binder's narrower product contribution is to make that structure cheap and operational in agent-assisted repository workflows:

- the argument is an executable base/candidate policy rather than a large certification document;
- evidence is rerun rather than merely cited;
- roots are hashed and staleness is mechanical;
- the receiving agent or reviewer is the evidence consumer.

This framing should acknowledge the assurance-case lineage rather than claim the underlying epistemology as new.

## User stories

- As a domain expert, I want to state once what observations would warrant a claim so that agents can rerun the deterministic work without repeatedly asking me to reconstruct the reasoning.
- As a maintainer, I want to know whether a proposed check fails on the buggy revision so that I do not mistake a generic green test for evidence of the fix.
- As a receiving agent, I want the claim, roots, command, and oracle in a machine-readable record so that I can inherit work without trusting the producing agent's summary.
- As an agent, I want a non-interactive inspection command and stable JSON errors so that I can plan, execute, and recover without parsing human terminal text.
- As a release owner, I want prior evidence to become stale when relevant inputs change so that old approval does not silently cover new code.
- As a release owner, I want the policy threshold separated from the evidence verdict so that a mainnet deployment can require more than an ordinary merge.
- As an auditor performing fix review, I want to see which claim and revision each check addresses so that I can focus renewed review on the affected assurance boundary.
- As a skeptical reviewer, I want a one-command replay so that I can challenge the result without Binder's service or the original author.

## Success criteria

Binder is useful only if it improves decisions, not merely documentation.

Before expanding beyond the local workflow:

- Binder improves correct review decisions or weak-evidence detection in at least three of five paired cases.
- At least four of five first-time reviewers accurately state both the claim and its guarantee boundary.
- Turning a suitable existing regression test into a claim takes under 15 active minutes.
- A reviewer can recover why the evidence bears on the claim without reconstructing the handoffs among issue, test, CI log, and artifact.
- At least two teams keep Binder enabled through a second real change.
- The receiver can replay at least 80% of pilot claims in a clean environment without private help.

The primary metric is **consequential changes reviewed with current, independently checkable claims**. Receipt count, checks executed, and dashboard traffic are not measures of product value.

## Positioning

Use:

> Binder lets humans state what would warrant a software claim once, then builds and evaluates the evidence graph for each change.

For smart contracts:

> Binder carries intent across the smart-contract toolchain: from an authored claim, through checks and witnesses, to the exact revision and artifact under review.

Short form:

> Frontload the judgment. Verify the graph on demand.

Avoid:

- “Binder proves your smart contract is secure.”
- “Continuous smart-contract auditing.”
- “A trustless security score.”
- “A universal knowledge graph.”
- “Git for claims.”

Those descriptions either broaden the guarantee beyond the evidence or lead with infrastructure instead of the receiving party's decision.

## Open questions

1. Who is entitled to author or revise the warrant rule, and how should Binder attribute that judgment without introducing a governance system?
2. Is base/candidate contrast sufficient, or must v1 also compare against a developer gold fix to detect candidate-specific checks?
3. What is the smallest typed witness contract that works across existing test frameworks without creating a Binder testing DSL?
4. Which roots can be discovered automatically without hiding consequential assumptions?
5. How often will receivers actually replay, and is credible replayability valuable even when they do not?
6. Is the first durable workflow audit remediation, agent-authored regression repair, or both?
7. Should the machine claim be committed, generated from PR metadata, or both?
8. Which action contexts and evidence thresholds should v1 support beyond merge review?

These are validation questions. Product work should answer them with real review behavior before adding hosted infrastructure.
