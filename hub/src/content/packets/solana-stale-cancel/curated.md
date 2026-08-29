# Curated: stale cancellation approval

The subscription PDA is reused for the same plan and subscriber. Before the
fix, `CancelSubscriptionNow` did not identify the incarnation for which both
parties signed. Commit `d4b29e80` adds the observed
`current_period_start_ts`, rejects a mismatch with
`StaleSubscriptionApproval`, and adds a head-side regression that recreates the
subscription at the same PDA. The audited base is `d6b3a5dc`; no preserved run
of the same observation against that base is linked publicly.

Boundary: this supports the head behavior in the named LiteSVM scenario, not a
base/head warrant, deployment identity, or whole-program safety.
