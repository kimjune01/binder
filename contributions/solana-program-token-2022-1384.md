# solana-program/token-2022 #1384 — Recover nested associated token accounts

- **Status:** queued
- **PR:** https://github.com/solana-program/token-2022/pull/1384
- **Head reviewed:** `dde896ae150059d64dbf15c360f82f77bf2c6ae3`
- **Selected:** 2026-08-30 because account recovery has explicit authority, derivation, and asset-preservation invariants.
- **Time spent:** selection only; audit not started

## Claim

The CLI can recover nested associated token accounts.

## Evidence ledger

### Observed

The PR changes three files with 238 additions and one deletion. No source or
tests have yet been inspected.

### Inferred

The consequential boundaries are correct address derivation for both token
programs, authority checks, destination selection, and preservation of tokens
and lamports on failure.

### Attested

The PR title claims support for recovering nested associated token accounts.

### Unknown

The precise supported account variants, failure behavior, and distinguishing
test coverage are not yet known.

## Distinguishing test

To determine during review: construct valid and near-valid nested accounts
across legacy Token and Token-2022 program IDs and verify authorization and
asset destinations. Not yet run.

## Potential contribution

Not drafted.

## Outcome

Not reviewed.

## Regret

None recorded.

## Follow-up

Inspect the implementation and enumerate every derivation/program-ID branch
before choosing a counterexample.
