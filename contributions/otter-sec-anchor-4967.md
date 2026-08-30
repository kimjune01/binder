# otter-sec/anchor #4967 — Avoid panic after a top-level instruction returns

- **Status:** closed
- **PR:** https://github.com/otter-sec/anchor/pull/4967
- **Head reviewed:** `948c73c019de4785b69a144f47d02f953995c803`
- **Selected:** 2026-08-30 because it changes a stateful log parser around an empty-stack boundary.
- **Time spent:** not recorded

## Claim

Logs arriving after a top-level instruction returns no longer panic the
subscriber, while a later top-level invocation still restores event parsing.

## Evidence ledger

### Observed

At `948c73c0`, the loop checks `Execution::try_program()` before parsing each
line. An empty stack ignores unscoped logs and accepts only a syntactically
valid depth-1 invocation to restore context. The following log is then processed
normally. `pop()` is idempotent on an empty stack. Three focused tests cover the
original trailing-log panic, an extra pop, and event recovery after a later
top-level invocation.

### Inferred

The state transition is internally consistent with the parser's existing
model: an empty stack has no event-bearing program context, and only a new
depth-1 invoke can establish one.

### Attested

The author reports 18 passing and 3 failing tests when reverting the source
change, and 21 passing tests with it.

### Unknown

The focused parser suite passed locally on the pinned head: five passed, none
failed. Runtime log validity beyond the documented Solana invocation/success
grammar was not independently established.

## Distinguishing test

Ran `cargo test -p anchor-client test_parse_logs_response -- --nocapture` at
`948c73c0`: five passed, including the trailing-log panic and later-event
recovery tests; none failed. The author reports that reverting the source change
causes the new regression tests to fail; that base-side result was not
independently rerun.

## Potential contribution

No useful comment found. The obvious dropped-event regression is explicitly
tested, and source inspection did not reveal a valid runtime transition that
the new empty-stack branch mishandles.

## Outcome

Review complete on the pinned live head; no public comment proposed.

## Regret

None recorded.

## Follow-up

Reopen if an authoritative runtime log sequence contradicts the assumed
grammar.
