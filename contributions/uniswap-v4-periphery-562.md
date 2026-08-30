# Uniswap/v4-periphery #562 — Track and refund native settlement value

- **Status:** posted — awaiting response
- **PR:** https://github.com/Uniswap/v4-periphery/pull/562
- **Comment:** https://github.com/Uniswap/v4-periphery/pull/562#issuecomment-5470673993
- **Head reviewed:** `f6d9987f3f24a1b11e2cff910cca64191b478869`
- **Selected:** 2026-08-30 because native/ERC20 settlement must conserve funds while tolerating bounded native overpayment.
- **Time spent:** not recorded

## Claim

`modifyLiquidities` consumes the required native value, rejects value on
non-native settlement paths, and refunds unused ETH at the end of execution.

## Evidence ledger

### Observed

At `f6d9987f`, native `_settle` subtracts the amount from the execution-level
transient `msg.value` balance. Every non-native `_settle` reverts whenever that
balance remains nonzero. `_settlePair` settles `currency0` before `currency1`;
for a native/ERC20 pool, native currency is first. Existing native-position test
helpers deliberately send a small rounding buffer such as `amount0 + 1`.

### Inferred

For `SETTLE_PAIR(native, ERC20)`, a native overpayment survives the first leg,
then causes the ERC20 leg to revert. Execution never reaches the end-of-call
refund. The guard confuses residual execution-level value awaiting refund with
value assigned to the non-native settlement.

### Attested

The PR claims unused ETH is refunded at the end of execution and ETH is rejected
on non-native settle paths. It includes no changed tests.

### Unknown

The distinguishing Foundry test was not run locally. The PR is older and marked
blocked; maintainer intent and whether it will be revived are unknown.

## Distinguishing test

Mint or increase a native/ERC20 position through `SETTLE_PAIR` with
`requiredNative + 1 wei`; assert that the ERC20 leg settles and the extra wei is
refunded. The reviewed implementation reverts on the ERC20 leg instead.

## Potential contribution

Posted on 2026-08-30 after confirming the live head matched the pinned revision.
The comment was then amended with pinned line links for settlement order,
transient-value handling, the ERC20 guard, and the existing overpayment helper.

## Outcome

Mixed-currency settlement regression identified and posted; awaiting maintainer
response.

## Regret

The initial contract-balance subsidy hypothesis was rejected before posting
because residual PositionManager balances are already intentionally sweepable.

## Follow-up

Watch for a regression test, guard redesign, rejection, or closure of the stale
PR.
