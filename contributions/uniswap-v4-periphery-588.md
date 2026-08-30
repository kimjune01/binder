# Uniswap/v4-periphery #588 — Safely convert V4 quoter swap deltas

- **Status:** queued
- **PR:** https://github.com/Uniswap/v4-periphery/pull/588
- **Head reviewed:** `7a1a256207c25fcf025ba7abd6dc005c282fd323`
- **Selected:** 2026-08-30 because the desired invariant is exact semantic agreement between quoting and execution.
- **Time spent:** selection only; audit not started

## Claim

The quoter rejects unsafe signed-delta conversions and mirrors router behavior,
including a fully hook-funded exact-output hop.

## Evidence ledger

### Observed

The PR changes `V4Quoter.sol`, adds a mock hook and 206 lines of focused tests,
and updates gas snapshots. Its latest commit expands the original scope to stop
reverse traversal when a hook funds the whole input.

### Inferred

The strongest review oracle is behavioral parity: for supported pool and hook
states, successful router execution should not be excluded or mispriced by the
quoter.

### Attested

The author reports eight of ten new tests distinguish the original safe-cast
fix and explains that an earlier test accidentally passed against unfixed code.

### Unknown

The latest combined head has not been independently tested for additional
quoter/router divergence or boundary values.

## Distinguishing test

To determine during review: differential-test quoter and router behavior over
signed delta boundaries and hook-funded multi-hop paths. Not yet run.

## Potential contribution

Not drafted.

## Outcome

Not reviewed.

## Regret

None recorded.

## Follow-up

Inspect whether the latest traversal change invalidates any earlier test oracle
or leaves another zero-input path inconsistent.
