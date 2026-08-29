# Case: stale cancellation approval after re-subscription

## Why this case

This public Solana Foundation case has the full chain Binder is intended to
make legible: a human audit finding, an exact audited revision, a remediation
change, and a regression test about a two-party authorization boundary.

The program derives a subscription PDA from a plan and subscriber. That address
can be reused after cancellation and re-subscription. Before remediation, a
`CancelSubscriptionNow` transaction signed by both parties for the earlier
subscription did not identify that particular incarnation. If held and later
submitted, it could cancel the newly created subscription at the same PDA
without fresh approval for the new agreement.

This resembles Binder's product thesis unusually well: signatures alone show
who approved bytes, while a human-authored entitlement rule determines what
those bytes are allowed to mean.

## Public sources

- [Audit status](https://github.com/solana-foundation/subscriptions/blob/main/audits/AUDIT_STATUS.md): audited baseline
  `d6b3a5dc7ab18c4168441af733c81ab0a599d414`; remediation reviewed through
  `debb4f75ff7571218b39de3b633074dd843e70db`.
- [Cantina report](https://github.com/solana-foundation/subscriptions/blob/main/audits/report-cli-cantina-a1f6fc40-7817-446d-bb88-abd0f2b96106-2026-07-30-solana-foundation-subscriptions.pdf): finding 3.2.1, “Stale CancelSubscriptionNow approvals can cancel a newly recreated subscription.”
- [Remediation commit](https://github.com/solana-foundation/subscriptions/commit/d4b29e80e2b3db3fc5cd449ffb7b563055644d51): binds both-party approval to the observed
  `current_period_start_ts` and adds a stale-approval error.
- The regression test in [that commit](https://github.com/solana-foundation/subscriptions/commit/d4b29e80e2b3db3fc5cd449ffb7b563055644d51),
  `cancel_subscription_now_rejects_stale_approval_after_resubscribe`, reuses the
  PDA after cancellation and asserts that the old period timestamp is rejected.

## Review question

> Does the public evidence warrant the narrow claim that an immediate-cancel
> approval bound to the earlier subscription incarnation is rejected after
> re-subscription at the same PDA?

This case does not ask whether the entire program is safe, whether every replay
route is excluded, or whether the deployed bytecode corresponds to the source.

## Expected decision

Request the exact base observation before treating the regression as
differential evidence. The head-side test and code strongly support the fix,
but a green candidate test alone does not establish that the same check would
have exposed the audited baseline. A proper Binder run should execute an
equivalent observation on both exact revisions and preserve both witnesses.
