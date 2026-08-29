# Binder validation study

## Decision

Do not build `binder init`, release packaging, a registry, or deployment integrations until this study shows that Binder improves a real review decision.

The hypothesis is:

> For consequential agent-produced changes, a named claim with differential, fresh, replayable evidence helps reviewers reach a correct decision faster than ordinary PR context and CI output.

## What we need to learn

1. Can a reviewer correctly state what was claimed and what was not established?
2. Does base-fail/head-pass evidence expose weak or insensitive checks that ordinary CI misses?
3. Does freshness information prevent reliance on evidence for the wrong revision or artifact?
4. Is the improvement large enough to justify authoring and maintaining a claim?
5. Which information changes the merge or release decision?

## Method

Run five paired, moderated review sessions using real fixes. Each session uses two comparable fixes or two reviewers of the same fix:

- **Control:** normal PR description, diff, and CI/test output.
- **Binder:** the same material plus the Binder report and replay bundle.

Alternate which condition appears first. Do not explain Binder before the participant makes an initial interpretation; the report must carry its own meaning.

Use real changes from repositories the participant understands. Prefer agent-authored regression fixes involving payments, authorization, accounting, migrations, or state transitions. At least two cases should contain a seeded review problem:

- a new test that passes on both base and head;
- evidence generated before a relevant dependency change;
- a claim broader than the observation actually supports;
- a missing required check;
- an artifact that does not match the reviewed revision.

Binder should detect only conditions its contract genuinely models. Record unsupported conditions rather than quietly expanding the product during the study.

## Participants

Recruit five to eight people who regularly approve consequential code changes:

- smart-contract maintainers;
- engineers reviewing agent-authored changes;
- audit-remediation or release owners;
- auditors performing fix review.

Exclude people who only write code but never make merge or release decisions. Prior familiarity with Binder is a disqualifier for the comprehension portion, though such participants can still provide workflow interviews.

## Session guide (45 minutes)

### Context — 10 minutes

- Tell me about the last change where a bad review could have caused material harm.
- What did the author claim was fixed?
- Where did you look for evidence?
- How did you know the tests were sensitive to the bug?
- What would have made old evidence invalid?

Ask about the last real event, not opinions about hypothetical tooling.

### Review task — 20 minutes

Give the participant a fixed starting state and ask:

1. What behavior does this change claim to preserve or restore?
2. What evidence supports that claim?
3. What does the evidence not establish?
4. Does the check distinguish the fix from the prior version?
5. Is the evidence current for what would be deployed?
6. Would you approve, reject, or request more evidence? Why?

Do not lead them toward the seeded problem. Record the decision, confidence, elapsed time, opened artifacts, and whether they replay anything.

### Reaction — 10 minutes

- What, if anything, changed your decision?
- Which line of the report mattered?
- What was confusing or untrustworthy?
- What did you still reconstruct manually?
- Would you require this on the next similar change?
- Who should author and maintain the claim?

### Commitment — 5 minutes

Ask for behavior, not praise:

- Will you provide one upcoming change for a pilot?
- Will you add the check to that repository if we pair on setup?
- Who else must agree for it to remain enabled?

## Measures

For every task, record:

| Measure | Definition |
| --- | --- |
| Decision correctness | Participant reaches the predetermined approve/reject/request-evidence outcome |
| Claim comprehension | Correctly states the promise without broadening it |
| Boundary comprehension | Names at least one important thing the evidence does not prove |
| Seed detection | Finds the insensitive, stale, missing, or mismatched evidence |
| Decision time | Time from materials shown to stated decision |
| Confidence | Participant rating from 1–5 after deciding |
| Replay behavior | Whether and why the participant opens or runs replay material |
| Adoption commitment | Supplies a real pilot and agrees to keep the workflow for another change |
| Authoring cost | Active minutes to turn an existing check into a Binder claim |

Do not use confidence alone as evidence of value; a clearer report can create false confidence.

## Build gate

Proceed to `binder init`, binaries, and the GitHub Action only if:

- Binder improves correct decisions or seeded-problem detection in at least three of five paired cases;
- at least four of five participants accurately explain the guarantee boundary;
- median claim setup is under 15 active minutes when a suitable test already exists;
- at least two teams provide an upcoming real change and keep Binder enabled for a second change; and
- no repeated misunderstanding causes reviewers to treat `WARRANTED` as whole-program safety.

If comprehension improves but decisions do not, improve the report before building distribution. If decisions improve but authoring is too costly, build `binder init`. If nobody commits a real change, stop: the problem may be interesting but not urgent.

## Study artifacts

For each case, preserve:

```text
validation/cases/<case-id>/
  context.md             # repository, change, expected decision
  control.md             # ordinary PR and CI material
  binder.md              # Binder-enhanced material
  answer-key.md          # seeded issue and guarantee boundary
  sessions/<participant>.md
```

Do not commit private repository material or participant identity. Use stable anonymous participant IDs and link to restricted source material outside this repository when necessary.

## Synthesis

After five cases, group observations by job rather than requested feature:

- reconstruct the intended behavior;
- decide whether a check is sensitive;
- decide whether evidence is current;
- understand the evidence boundary;
- reproduce the result;
- maintain the claim across changes.

Rank product changes by observed decision impact and frequency. A feature request without an observed review failure stays out of the build plan.

