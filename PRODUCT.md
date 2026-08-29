# Binder product framing and scope

## Product definition

**Binder makes agent-produced software claims independently inheritable.**

A producer presents a consequential code change with a named claim, the check that could refute it, the exact roots the check used, and a receipt another party can replay. The receiver does not have to trust the producer's account of its work or reconstruct the justification from a diff and a green CI run.

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

Binder is the software-change application of the verification primitive described in *Verifiable Knowledge*: a claim that travels with the falsifiable check another agent can rerun.

## Problem

Agents can produce changes faster than humans can reconstruct their justification. A passing check says that something ran successfully; it does not say which claim the check supports, whether it would detect the original defect, which system version it covered, or whether later changes invalidated it.

Today the receiver must either trust the author's summary or repeat the investigation from the beginning. Provenance logs improve accountability for what happened, but do not establish that a belief earned entitlement.

## Initial user and decision

The initial user is a maintainer reviewing a consequential, agent-assisted software change. Smart-contract maintainers are the first domain because their important claims are unusually explicit, changes are costly to get wrong, and source/runtime evidence is often split across tools.

The decision Binder supports is narrow:

> Is this specific claim about this specific change currently supported by checks I can inspect or replay?

Binder does not answer whether the entire program is safe, whether the change should ship for business reasons, or whether the authored claim is complete.

## Core product contract

Every Binder claim must carry:

| Primitive | Product meaning |
| --- | --- |
| Claim | A precise behavior the change is supposed to establish or preserve |
| Subject | The exact base and candidate revisions or artifacts being compared |
| Check | An executable procedure whose result bears on the claim |
| Oracle | The expected base and candidate outcomes, including what counts as refutation |
| Roots | Source, specification, fixture, toolchain, environment, and artifact inputs |
| Kill condition | A failed check or changed root that removes the claim's current entitlement |
| Receipt | The versioned, content-addressed observation produced by running the check |
| Replay | Instructions and inputs sufficient for a receiver to rerun the observation |
| Status | `WARRANTED`, `FAILED`, `STALE`, or `UNSUPPORTED`, always scoped to the claim and roots |

`WARRANTED` means the required candidate checks passed, the expected base checks failed, and the recorded roots remain current. It never means whole-program safety or final truth.

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

Running tests is commodity infrastructure. Maintaining the claim–evidence–subject relationship is the product.

## v1 scope: evidence-carrying changes

### Must have

- Repository-committed, versioned claim manifests.
- Real repository base and candidate revision identity.
- Explicit expected outcomes and claim-specific observations.
- Commands executed locally or in the repository owner's CI.
- Versioned, deterministic, content-addressed receipts.
- Declared source, toolchain, fixture, environment, and artifact roots.
- Accurate invalidation when a relevant root changes.
- Human-readable and JSON reports with stable exit behavior.
- A generic replay contract that does not depend on Binder's demo scripts.
- A receiver can validate and replay without a hosted Binder service.

### Should have after validation

- `binder init` for turning an existing regression check into a claim.
- Released CLI binaries and an official GitHub Action.
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
- A global hypothesis graph or shared canon.
- Social identity, reputation, staking, or governance mechanisms.

These exclusions preserve the falsifiable wedge: if evidence-carrying changes do not improve real review decisions, broader infrastructure has no product foundation.

## Relationship to verifiable knowledge

Binder implements a deliberately small part of the full protocol.

### Implemented or directly in scope

- replayable check;
- rooted provenance;
- system-facing kill condition;
- replay by a receiver who need not trust the author;
- a three-way practical distinction among supported, refuted, and unsupported evidence, plus staleness;
- admission to a local record only after checks run.

### Future protocol layers

- claim-to-claim provenance and downstream refutation;
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

- As a maintainer, I want to know whether a proposed check fails on the buggy revision so that I do not mistake a generic green test for evidence of the fix.
- As a receiving agent, I want the claim, roots, command, and oracle in a machine-readable record so that I can inherit work without trusting the producing agent's summary.
- As a release owner, I want prior evidence to become stale when relevant inputs change so that old approval does not silently cover new code.
- As an auditor performing fix review, I want to see which claim and revision each check addresses so that I can focus renewed review on the affected assurance boundary.
- As a skeptical reviewer, I want a one-command replay so that I can challenge the result without Binder's service or the original author.

## Success criteria

Binder is useful only if it improves decisions, not merely documentation.

Before expanding beyond the local workflow:

- Binder improves correct review decisions or weak-evidence detection in at least three of five paired cases.
- At least four of five first-time reviewers accurately state both the claim and its guarantee boundary.
- Turning a suitable existing regression test into a claim takes under 15 active minutes.
- At least two teams keep Binder enabled through a second real change.
- The receiver can replay at least 80% of pilot claims in a clean environment without private help.

The primary metric is **consequential changes reviewed with current, independently checkable claims**. Receipt count, checks executed, and dashboard traffic are not measures of product value.

## Positioning

Use:

> Binder turns an agent's software claim into evidence another agent or reviewer can independently check.

For smart contracts:

> Binder tells a reviewer whether a specific claim about a contract change has current, replayable support for the exact revision and artifact under review.

Avoid:

- “Binder proves your smart contract is secure.”
- “Continuous smart-contract auditing.”
- “A trustless security score.”
- “A universal knowledge graph.”
- “Git for claims.”

Those descriptions either broaden the guarantee beyond the evidence or lead with infrastructure instead of the receiving party's decision.

## Open questions

1. Will maintainers author and maintain claims, or must claims be generated from existing tests and issue context?
2. Is base/candidate contrast sufficient, or must v1 also compare against a developer gold fix to detect candidate-specific checks?
3. What observation contract proves that a base failure represents the claimed defect rather than an unrelated command error?
4. Which roots can be discovered automatically without hiding consequential assumptions?
5. How often will receivers actually replay, and is credible replayability valuable even when they do not?
6. Is the first durable workflow audit remediation, agent-authored regression repair, or both?

These are validation questions. Product work should answer them with real review behavior before adding hosted infrastructure.

