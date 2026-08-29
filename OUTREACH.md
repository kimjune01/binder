# Binder outreach guide

For the ready-to-run prospect queue, personalized messages, public case, and
moderator sheet, start with [FIRST_CONVERSATIONS.md](FIRST_CONVERSATIONS.md).

## Purpose

Use outreach to learn whether Binder removes work from consequential contract
review. Do not begin by selling verifiable agreements, an evidence graph, or a
contract marketplace. Begin with the person's last real fix, audit delta, or
agent-authored change.

Run two complementary tracks:

1. **Soft conversation:** discover how people currently decide that a claim is
   supported and where they lose time or trust.
2. **Demo session:** observe whether Binder changes comprehension or a review
   decision when placed beside ordinary code and CI evidence.

Recruit six to eight people across:

- smart-contract maintainers who approve changes;
- auditors who perform fix review;
- protocol security or release owners;
- maintainers of testing, simulation, or verification tools; and
- one or two developers shipping agent-authored contract code.

Someone who only finds the philosophy interesting is not yet a validated user.

## Track A: soft conversation

### Initial message

Keep the invitation specific, brief, and easy to decline.

#### Maintainer or release owner

> Hey — I’m looking into how contract teams decide that a specific fix actually
> addresses the behavior claimed in the PR, especially when agents are writing
> the code or tests. Could I ask you about the last change where you had to make
> that call? I’m not selling an audit tool; I’m trying to understand the review
> handoffs. Fifteen minutes would be useful.

#### Auditor

> I’m studying the fix-review step after a contract finding: how you connect the
> finding, changed code, regression check, and exact reviewed revision. Could I
> ask about one recent example and what you had to reconstruct manually? I have
> a small prototype, but I’d rather understand your workflow before showing it.

#### Tool maintainer

> I’m exploring what gets lost when a test, simulator, fuzzer, or prover result
> moves into a PR or release decision. Could I ask about a case where users
> interpreted your tool’s output more broadly than it warranted? Fifteen minutes
> is enough; I’m looking for failure modes, not a product endorsement.

### Conversation shape — 20 minutes

Treat this as a conversation, not a questionnaire. Follow the concrete event.

#### Open with the last real case

> What was the last contract change where getting the review wrong would have
> mattered?

Useful follow-ups:

- What did the author say was fixed or preserved?
- Where was that claim written?
- What did you inspect before believing it?
- Did you run anything, or rely on CI, an audit, or another person?
- How did you know the new check would have caught the old behavior?
- Which commit, build, or deployment did the evidence actually cover?

Ask “what happened next?” before asking “what would you prefer?” Past behavior
is stronger evidence than feature opinions.

#### Explore the handoffs

> Where did you have to translate between prose, code, tests, tool output, and
> the release decision?

Probe only where relevant:

- Did someone restate the invariant for the reviewer or auditor?
- Were audit assumptions still current after the change?
- Did two tools report results with different execution boundaries?
- Could another engineer reproduce the conclusion without asking the author?
- What evidence would become invalid if a dependency changed?
- How long did this reconstruction take, and who performed it?

#### Introduce the mechanism softly

Only after understanding the workflow:

> I’m testing a small CLI that keeps one relationship explicit: a human states
> which observations would warrant a named claim, and the tool runs the same
> check against exact base and candidate revisions, records witnesses, and
> preserves a receipt. It does not audit the whole contract. Where would that
> have helped—or failed—in the example you just described?

Do not defend Binder immediately. Ask:

- Which part sounds redundant with your current process?
- Who would be qualified to author the warrant rule?
- What would make the receipt untrustworthy?
- Would this replace work, or merely create another artifact?

#### Close on behavior

> Do you have an upcoming or public fix where we could try this against the
> normal review material?

If yes, ask for the repository, base and candidate revisions, claim or finding,
and existing check. Offer to prepare the first manifest together. Do not count
“keep me posted” as commitment.

## Track B: demo conversation

### Invitation

> I built a ten-minute contract-change demo around a narrow question: does the
> proposed evidence actually distinguish the old behavior from the new one? I’d
> like to watch you interpret it without a product explanation, then hear what
> you would still need before relying on it. Would you be open to a 25-minute
> screen-share?

For auditors, replace the first sentence with:

> I built a small fix-review demo that binds an authored claim, exact revisions,
> base/head observations, and concrete witnesses into one receipt.

### Session — 25 minutes

#### 1. Establish their normal decision — 5 minutes

Ask briefly:

> When you see a green regression check on a contract fix, what do you normally
> verify before accepting it?

Do not explain Binder yet.

#### 2. Cold-read the claim — 4 minutes

Show [demo/contract/claim.yaml](demo/contract/claim.yaml), beginning with:

```yaml
claim: Escrow funds move only when both parties approve the release.
entitlement:
  authored_by: escrow-maintainers
  base: refuted
  head: stood
```

Ask:

- What judgment has the human supplied here?
- What do you expect Binder to determine mechanically?
- What is still ambiguous?

#### 3. Run the human report — 4 minutes

```sh
cargo run --quiet -p binder-cli -- verify \
  demo/contract/claim.yaml \
  --base 78d7031c48b7b98af74055dead4002c8dbf8941c \
  --head HEAD
```

Ask before explaining the output:

- What do you believe now?
- What does `WARRANTED` not establish?
- Which line changed or supported your interpretation?
- Would you approve this specific claim?

#### 4. Inspect the agent output — 4 minutes

```sh
bash demo/contract/run.sh
```

Ask them to locate:

- the authored entitlement rule;
- exact subject revisions;
- base and candidate observations;
- concrete balance witnesses;
- evidence kind and execution outcome; and
- receipt identity.

The participant should be able to say:

> The check observed an unauthorized transfer on the base and unchanged
> balances on the candidate; this warrants only the declared approval claim for
> these revisions and this empirical check.

Do not teach that sentence unless they cannot recover it. Record omissions and
overclaims.

#### 5. Challenge the evidence — 4 minutes

Present the two negative controls verbally or from the integration-test output:

1. The check reports `stood` on both base and candidate.
2. The command exits successfully but emits no structured witness.

Ask:

- Should either case warrant the claim?
- What should the machine output and exit behavior be?
- Would your existing CI make the distinction obvious?

Binder should return `not-warranted`; the second case should produce
`no-verdict`, not infer epistemic success from exit code zero.

#### 6. Return to their workflow — 4 minutes

> If this appeared on your next real fix, what work would it remove? What would
> it add?

Then ask:

- Would you want this before audit, during fix review, or before deployment?
- Who owns the claim when code changes later?
- Would you require it on a second change?
- Can we try it on one real case?

## What to record

Immediately after each conversation, capture:

```text
Participant role:
Last real case:
Current decision process:
Manual handoffs:
Time or people involved:
Binder comprehension:
Guarantee-boundary overclaim:
Decision changed by demo:
Work removed:
New work introduced:
Trust objection:
Concrete pilot offered:
Second-use commitment:
Strongest exact quote:
```

Store anonymous notes under the structure defined in
[VALIDATION.md](VALIDATION.md). Do not commit private code or identifying
participant details.

## Signals

### Strong demand

- They describe recent manual reconstruction before hearing about Binder.
- Base/head contrast changes a decision or exposes a weak check.
- They identify a specific person who would own the claim.
- They offer a real fix, finding, or upgrade for a pilot.
- They want Binder retained for the next change after the pilot.
- They say what existing step or review time it would replace.

### Weak demand

- They like the epistemic framing but cannot name a recent costly case.
- They want a dashboard, score, or marketplace before using the CLI.
- They call the receipt “nice documentation” without changing a decision.
- They will view a demo but not supply a real change.
- The manifest duplicates a maintained artifact they already trust.

### Disconfirming evidence

- Reviewers consistently infer whole-contract safety from `WARRANTED`.
- No credible owner will author or maintain entitlement rules.
- Existing regression practice already checks base sensitivity cheaply.
- Receipts add review time without finding weak, stale, or mismatched evidence.
- Teams will not retain Binder through a second real change.

## Outreach discipline

- Personalize with a real repository, audit finding, or tool boundary.
- Never imply that Binder audits or proves the whole contract.
- Do not lead with the long-range marketplace vision.
- Do not ask “Would you use this?” Ask for the last event and the next pilot.
- Offer useful output from their public case even if they decline a call.
- Stop after one follow-up unless they engage.
- Treat criticism and refusal as research data, not an objection to overcome.

The outreach succeeds when it produces observed review behavior and real pilot
material, not compliments or a large contact list.
