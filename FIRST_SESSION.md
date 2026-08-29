# First Binder session

This is a one-screen moderator sheet for a 25-minute conversation. The
participant should do most of the talking.

## Preflight

- Open the participant's public artifact and the
  [subscriptions case](validation/cases/subscriptions-stale-cancel/context.md).
- Keep [control.md](validation/cases/subscriptions-stale-cancel/control.md) and
  [binder.md](validation/cases/subscriptions-stale-cancel/binder.md) in separate
  tabs; show the control first.
- Run `bash demo/contract/run.sh` once locally so the fallback demo is warm.
- Copy [validation/session-template.md](validation/session-template.md) to an
  untracked or private note with an anonymous ID.
- Start a visible timer when the review task begins.

## 0:00–0:02 — frame

> I’m trying to understand a review handoff, not test you or sell an audit
> product. I’ll first ask about a real case, then show public material with no
> explanation. Please say what you inspect and what you believe as you go.

Do not mention evidence graphs, epistemology, marketplaces, or agents unless
the participant raises them.

## 0:02–0:09 — their last case

> Tell me about the last contract change where believing the fix mattered.

Follow the event:

- What exactly was claimed?
- Who had authority to state that claim?
- What did you inspect or run before relying on it?
- How did you know the check could distinguish the old behavior?
- Which revision or deployment did the evidence cover?
- What had to be reconstructed manually, by whom, and for how long?

If answers become hypothetical, ask: “What happened in that specific case?”

## 0:09–0:15 — control

Show only [control.md](validation/cases/subscriptions-stale-cancel/control.md).

> Imagine this is the material handed to you for remediation review. Decide:
> approve, reject, or request more evidence. Talk through what you open.

Record time, decision, confidence, order of artifacts, and missing information.
Do not help them find the seeded ambiguity.

## 0:15–0:20 — Binder artifact

Show [binder.md](validation/cases/subscriptions-stale-cancel/binder.md).

> What do you believe now? What does this still not establish?

Then ask:

- Which field, if any, changed the decision?
- Who is entitled to author the warrant rule?
- What becomes stale after another code or dependency change?
- Is this less work than the reconstruction you just performed?
- What would make this receipt actively misleading?

The artifact is a proposed rendering based on public evidence, not output from
a Binder integration in the upstream repository. Say so if asked.

## 0:20–0:25 — commitment

> Where would this have entered your last real workflow, if anywhere?

> Is there one upcoming or public fix where we can pair on the claim and check?

Capture the repository, base, head, claim owner, existing check, and a concrete
next action. “Keep me posted” is not a pilot.

## Recovery paths

- If the public case is too unfamiliar, switch to the participant's own public
  change and use the questions without showing the Binder artifact.
- If Binder is mistaken, stop defending it. Ask what conclusion is invalid and
  what source establishes that.
- If screen sharing fails, send the two Markdown links in order.
- If there are ten minutes left, skip the artifact tour and ask only what work
  the proposed receipt removes or adds.
- If they want a working demonstration, run `bash demo/contract/run.sh` and
  clearly label the escrow fixture as synthetic.

## Immediately after

Complete the note before starting another call. Separate direct observation
from interpretation. Write one sentence answering: “What did Binder change in
the decision?” Then send any promised follow-up within one day.
