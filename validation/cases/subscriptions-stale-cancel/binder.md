# Remediation review — proposed Binder artifact

This is a hand-authored preview of the artifact Binder should generate for the
public case. It is not a receipt produced by an upstream Binder integration.

```yaml
claim_id: stale-cancel-approval-is-incarnation-bound
claim: >-
  A CancelSubscriptionNow approval carrying the prior subscription's
  current_period_start_ts is rejected after cancellation, closure, and
  re-subscription at the same plan/subscriber PDA.

entitlement:
  authored_by: subscriptions-maintainers
  authority: behavioral claim for cancel_subscription_now remediation
  warrant_when:
    base: refuted
    head: stood

subject:
  repository: solana-foundation/subscriptions
  base: d6b3a5dc7ab18c4168441af733c81ab0a599d414
  head: d4b29e80e2b3db3fc5cd449ffb7b563055644d51

check:
  kind: empirical
  engine: LiteSVM integration test
  observation: >-
    Create the first subscription; retain its period-start value; cancel,
    close, advance time, and re-subscribe at the same PDA; submit immediate
    cancellation with the retained value.

observations:
  base:
    verdict: missing
    witness: null
  head:
    verdict: stood
    witness: StaleSubscriptionApproval

policy:
  result: no-verdict
  reason: required base observation has not been supplied
```

## Interpretation

The candidate observation is consistent with the narrow claim, but the
authored warrant requires a discriminating base/head contrast. Binder must not
turn a successful head test or process exit into `WARRANTED`. A real run should
produce a base witness showing the old approval succeeds against the recreated
subscription, and a head witness showing `StaleSubscriptionApproval` for the
same scenario.

Even after that contrast, the receipt would not establish:

- whole-program or deployment safety;
- absence of other replay paths;
- correctness outside the named engine and scenario;
- source-to-deployed-bytecode identity; or
- continuing validity after relevant code, test, engine, or dependency changes.

The entitlement edge is the product: an authorized human says this particular
contrast warrants this particular claim. Binder preserves and evaluates that
edge; the test runner merely supplies observations.
