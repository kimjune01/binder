# Binder demo completion plan

## Objective

Finish the Binder demo as a reproducible, independently replayable assurance workflow whose real Mollusk runtime evidence distinguishes the vulnerable and fixed vault revisions, feeds deterministic receipts, invalidates stale evidence, and can be audited against every stated requirement.

## Work plan

1. Make the Mollusk runtime adapter compile and prove vulnerable/fixed behavior with tests.
   - Exercise the actual Solana runtime boundary through Mollusk.
   - Show that the vulnerable revision permits the unauthorized balance transfer.
   - Show that the fixed revision rejects it and preserves balances.
   - Keep the fixture deterministic and bind the observation to the relevant program/input bytes.

2. Integrate runtime evidence into Binder receipts and verify stale-input behavior.
   - Run the Mollusk trial through the claim manifest.
   - Record its command, verdict, dependency snapshot, and output in the receipt.
   - Prove that changing a declared runtime input makes prior evidence stale rather than silently reusable.

3. Add and execute a one-command clean-machine replay.
   - Provide one documented command that installs/resolves pinned prerequisites, builds, tests, runs the vulnerable/fixed trials, and validates the resulting receipt.
   - Run it from a clean checkout or equivalent isolated environment.
   - Record any unavoidable externally pinned inputs and the observed replay result.

4. Audit the full demo against every goal requirement.
   - Check each acceptance criterion in `DEMO.md` and the recovered objective against concrete code, tests, receipts, or replay output.
   - Fix any gaps that remain in scope.
   - Produce a concise final report distinguishing satisfied, partially satisfied, and unsupported claims.

## Completion standard

The work is complete when the adapter and Binder tests pass, vulnerable/fixed runtime behavior is demonstrated, stale evidence fails closed, the clean replay command succeeds, and every demo requirement has an explicit evidence-backed audit result.
