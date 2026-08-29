# Remediation review — ordinary material

You are reviewing a smart-contract audit remediation.

## Finding

An immediate cancellation requires both subscriber and merchant signatures,
but its instruction data was not bound to a particular subscription
incarnation. The subscription PDA can be reused for the same plan and
subscriber. A previously signed cancellation could therefore be submitted
after re-subscription and cancel the new agreement.

## Proposed fix

[Commit `d4b29e8`](https://github.com/solana-foundation/subscriptions/commit/d4b29e80e2b3db3fc5cd449ffb7b563055644d51)
adds `expected_current_period_start_ts` to the instruction data. Processing
returns `StaleSubscriptionApproval` when it differs from the live account.

The commit adds this test:

```rust
#[test]
fn cancel_subscription_now_rejects_stale_approval_after_resubscribe() {
    // Record the first incarnation's period start, cancel and close it,
    // advance time, then subscribe again at the same PDA.
    CancelSubscriptionNow::new(/* new live subscription */)
        .expected_current_period_start_ts(first_period_start)
        .execute()
        .assert_err(SubscriptionsError::StaleSubscriptionApproval);
}
```

The remediation commit passed its repository checks and the published audit
status says remediation was reviewed through a later commit.

## Your task

Decide whether to approve, reject, or request more evidence. State:

1. the exact behavior you believe is supported;
2. the evidence you relied on;
3. what the material does not establish; and
4. anything you would run or inspect before approving.
