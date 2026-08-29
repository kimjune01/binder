# Binder first-conversation kit

This is the operating sheet for the first six conversations. The goal is not
to pitch Binder. It is to learn whether preserving a narrow chain from a human
claim to revision-bound evidence removes consequential review work.

Use [OUTREACH.md](OUTREACH.md) for the full interview guide and
[FIRST_SESSION.md](FIRST_SESSION.md) while moderating. Start with the public
Solana subscriptions case in
[validation/cases/subscriptions-stale-cancel/context.md](validation/cases/subscriptions-stale-cancel/context.md).

## What to test

> When a contract finding becomes a fix, do reviewers lose time or confidence
> reconstructing which claim, check, revisions, and assumptions belong
> together?

The conversation succeeds if it reveals a real workflow and an observable
cost. Interest in the idea alone is not evidence. The best outcome is an offer
of a real upcoming change; the second best is a precise reason the artifact
would not help.

## Launch queue

Contact one person from each row before adding more names. Use a public channel
they already invite; do not open an unrelated repository issue merely to reach
someone.

| Priority | Person or route | Public work to begin from | Learning goal |
| --- | --- | --- | --- |
| 1 | Jo D (`dev-jodee`), Solana Foundation subscriptions maintainer | [Audit status and exact audited/fixed revisions](https://github.com/solana-foundation/subscriptions/blob/main/audits/AUDIT_STATUS.md) and [incarnation-binding fix](https://github.com/solana-foundation/subscriptions/commit/d4b29e80e2b3db3fc5cd449ffb7b563055644d51) | Who reconstructs the audit-to-fix chain now, and would a durable claim receipt remove any work? |
| 2 | A Cantina researcher who performed the subscriptions review, routed through the report or Cantina | [July 2026 subscriptions audit](https://github.com/solana-foundation/subscriptions/blob/main/audits/report-cli-cantina-a1f6fc40-7817-446d-bb88-abd0f2b96106-2026-07-30-solana-foundation-subscriptions.pdf) | During remediation review, what must an auditor re-establish and what evidence can legitimately travel with the finding? |
| 3 | Umar (`meumar-osec`), verified-builds contributor | [Verified-builds FAQ update](https://github.com/solana-foundation/solana-com/pull/1241) and [deprecated remote-flow update](https://github.com/solana-foundation/solana-com/pull/1522) | Where does source/build/deployment identity stop, and where must behavioral claims begin? |
| 4 | `a-maggi`, LiteSVM user, and a LiteSVM maintainer from the same thread | [Compute-unit discrepancy across LiteSVM, Mollusk, and on-chain execution](https://github.com/LiteSVM/litesvm/issues/277) | How do users communicate an engine boundary without overgeneralizing a result? |
| 5 | Luka (`lukacan`), Trident invariant contributor | [Trident invariant additions](https://github.com/Ackee-Blockchain/trident/pull/463) | Who authors the semantic invariant, and how should it survive tool output and later revisions? |
| 6 | Michael Moffett and a Crucible maintainer from their public exchange | [AI-assisted invariant-authoring sidecar discussion](https://github.com/asymmetric-research/crucible/issues/13) | Does agent-authored evidence increase the need for provenance and entitlement, or merely add metadata? |

Rows 1 and 2 are the paired sides of one real handoff, not two independent
market signals. Rows 3–6 test adjacent boundaries and should not displace the
maintainer/auditor case.

## Personalized openers

### Subscriptions maintainer

> I was reading the July Cantina remediation trail for `subscriptions`,
> especially the fix that binds `cancel_subscription_now` to
> `current_period_start_ts`. The audit finding, audited baseline, remediation
> commit, and regression test are all public, but a reviewer still has to join
> them into one conclusion. I’m testing a small CLI for preserving that exact
> claim-to-evidence relationship—not for auditing the whole program. Could I
> ask how that handoff worked in practice and show you a 10-minute rendering of
> your public case? Twenty-five minutes total; criticism is the useful result.

### Subscriptions auditor

> I’m studying the remediation-review step after a smart-contract finding. I
> used the public stale `CancelSubscriptionNow` approval finding as a sample:
> finding, exact baseline, incarnation-binding fix, and regression observation.
> I’m testing whether packaging that narrow chain saves an auditor any
> reconstruction without overstating what was reviewed. Could I ask how you
> verified this case and then show the artifact? I’m looking for where it is
> epistemically wrong, not an endorsement.

### Verified-builds contributor

> Your verified-builds work makes source-to-deployed-program identity much
> more legible. I’m exploring the next, separate edge: binding a human-authored
> behavioral claim to a discriminating check and the exact revisions it ran
> on. Could I show you one public Solana fix and ask where that artifact should
> meet—or stay separate from—verified builds?

### Execution-tool user or maintainer

> The LiteSVM/Mollusk/on-chain compute discrepancy is a crisp example of a
> result whose meaning depends on the execution boundary. I’m testing a CLI
> receipt that records the human claim, engine, revisions, and observations so
> downstream reviewers do not silently broaden it. Could I ask how you would
> want that boundary represented, using that thread as the concrete case?

### Invariant-tool contributor

> I’m looking at the handoff between a human or agent-authored invariant and
> the evidence a fuzzer produces. The Trident/Crucible work makes that boundary
> concrete. I’m testing a thin claim receipt above existing tools, not another
> engine. Could I show you a real audit-remediation example and ask whether it
> preserves something your workflow currently loses?

## Scheduling reply

> Great—25 minutes is enough. I’ll use only public material. We’ll spend about
> 10 minutes on your last comparable review, 10 minutes cold-reading one case,
> and 5 minutes on whether it would remove or add work. No preparation needed.
> If you prefer, send one public fix you know well and I’ll use that instead.

## Before each message

- Read the linked artifact and replace one generic sentence with a precise
  detail from it.
- Confirm the person actually made the contribution attributed to them.
- Use the least invasive public contact route available.
- Ask for one conversation, not product feedback by email.
- Create an anonymous session ID before the call; keep identity outside the
  committed research notes.

## Stop conditions

After three conversations, pause if nobody describes manual reconstruction
before seeing Binder. After five, apply the gates in [VALIDATION.md](VALIDATION.md).
Do not broaden the pitch to a marketplace, formal verification, or deployment
automation to rescue weak demand.
