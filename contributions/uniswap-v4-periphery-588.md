# Uniswap/v4-periphery #588 — Safely convert V4 quoter swap deltas

- **Status:** closed
- **PR:** https://github.com/Uniswap/v4-periphery/pull/588
- **Head reviewed:** `7a1a256207c25fcf025ba7abd6dc005c282fd323`
- **Selected:** 2026-08-30 because the desired invariant is exact semantic agreement between quoting and execution.
- **Time spent:** not recorded

## Claim

The quoter rejects unsafe signed-delta conversions and mirrors router behavior,
including a fully hook-funded exact-output hop.

## Evidence ledger

### Observed

At `7a1a2562`, all four signed-delta conversions use `SafeCast`; exact-input
amounts widen through `uint256` before negation; exact-output inputs widen to
`int256` before negating the signed delta. Multi-hop exact output stops when a
fully funded hop produces zero input. These forms match the corresponding
router conversions and zero-input traversal behavior at the reviewed revision.
The added mock can drive both signs and the `int128.min` boundary, and the ten
focused tests exercise all four sites plus multi-hop propagation.

### Inferred

The strongest review oracle is behavioral parity: for supported pool and hook
states, successful router execution should not be excluded or mispriced by the
quoter.

### Attested

The author reports eight of ten new tests distinguish the original safe-cast
fix and explains that an earlier test accidentally passed against unfixed code.

### Unknown

Foundry is not installed in the review environment, so the reported 779-test
run and base/head discrimination were not independently reproduced. Router and
quoter still differ in interfaces and settlement by design; no additional
conversion or zero-input mismatch was established.

## Distinguishing test

Source-level parity check completed for the changed conversion and loop sites.
The PR's ten tests are attested, not locally run. A broader property-based
router/quoter differential test remains a possible future enhancement, not a
specific gap in this PR.

## Potential contribution

No useful comment found. The initial known divergence was fixed by the latest
commit, and each changed boundary has a focused regression or no-regression
case.

## Outcome

Review complete on the pinned live head; no public comment proposed.

## Regret

None recorded.

## Follow-up

Reopen if the focused Foundry suite fails on the pinned head or a concrete
router-success/quoter-failure case is produced.
