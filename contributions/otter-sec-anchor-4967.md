# otter-sec/anchor #4967 — Avoid panic after a top-level instruction returns

- **Status:** queued
- **PR:** https://github.com/otter-sec/anchor/pull/4967
- **Head reviewed:** `948c73c019de4785b69a144f47d02f953995c803`
- **Selected:** 2026-08-30 because it changes a stateful log parser around an empty-stack boundary.
- **Time spent:** selection only; audit not started

## Claim

Logs arriving after a top-level instruction returns no longer panic the
subscriber, while a later top-level invocation still restores event parsing.

## Evidence ledger

### Observed

The PR description reports three regression tests and says they distinguish the
old implementation from the proposed fix. The changed files are
`client/src/lib.rs` and `CHANGELOG.md`.

### Inferred

The most useful review boundary is whether every stack transition after an
unscoped log preserves later valid invocation/event parsing.

### Attested

The author reports 18 passing and 3 failing tests when reverting the source
change, and 21 passing tests with it.

### Unknown

The implementation and tests have not been independently inspected or run.

## Distinguishing test

To determine during review: exercise consecutive top-level invocations with
unscoped runtime logs, malformed success/failure ordering, and nested program
returns. Not yet run.

## Potential contribution

Not drafted.

## Outcome

Not reviewed.

## Regret

None recorded.

## Follow-up

Inspect the parser as a state machine and look for a sequence accepted by the
runtime that the three new examples do not cover.
