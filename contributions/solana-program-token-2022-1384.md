# solana-program/token-2022 #1384 — Recover nested associated token accounts

- **Status:** draft contribution
- **PR:** https://github.com/solana-program/token-2022/pull/1384
- **Head reviewed:** `dde896ae150059d64dbf15c360f82f77bf2c6ae3`
- **Selected:** 2026-08-30 because account recovery has explicit authority, derivation, and asset-preservation invariants.
- **Time spent:** not recorded

## Claim

The CLI can recover nested associated token accounts.

## Evidence ledger

### Observed

At `dde896ae`, `command_recover_nested` derives the owner, nested, and
destination ATAs, then submits only the `recover_nested` instruction
([`command.rs` lines 3392-3431](https://github.com/solana-program/token-2022/blob/dde896ae150059d64dbf15c360f82f77bf2c6ae3/clients/cli/src/command.rs#L3392-L3431)).
The added integration test creates and leaves the owner ATA live before invoking
the command
([`command.rs` lines 1605-1624](https://github.com/solana-program/token-2022/blob/dde896ae150059d64dbf15c360f82f77bf2c6ae3/clients/cli/tests/command.rs#L1605-L1624)).

The Associated Token Account processor checks that the owner ATA is a live
token account and returns `IllegalOwner` with “recreate the owner associated
token account first” otherwise
([upstream processor lines 221-237](https://github.com/solana-program/associated-token-account/blob/c0c821e7792054c1034ff368f33cc593ccdb425e/program/src/processor.rs#L221-L237)).

### Inferred

The motivating closed-owner-ATA case will fail unless the user has separately
recreated that ATA. With `same_mint`, the owner ATA is also the destination, so
the prerequisite is unavoidable. The CLI can either prepend idempotent create
instructions for the required ATAs or state and diagnose the prerequisite.

### Attested

The PR says the nested account arises after the original ATA “had been closed”
and describes the new command as transferring the stranded balance and closing
the nested account.

### Unknown

The predicted failure has not been run against a validator. It remains unknown
whether maintainers intentionally expect callers to recreate the owner and
destination ATAs with separate CLI commands.

## Distinguishing test

Proposed, not run: create the owner ATA and nested ATA, fund the nested ATA,
close the now-empty owner ATA, then invoke `spl-token recover-nested` directly.
At the current head, predict failure before transfer because the command does
not recreate the owner ATA. Run for both token program IDs; the `same_mint`
variant is the smallest case.

## Potential contribution

> The motivating closed-owner-ATA case appears to be missing from the test and
> command flow. The test leaves `owner_associated_account` live, while
> `command_recover_nested` submits only `recover_nested`; the ATA processor
> rejects a missing owner ATA and explicitly says to recreate it first. In the
> `same_mint` case that account is also the destination. Could this test close
> the owner ATA before invoking the CLI, and could the command either prepend
> the idempotent ATA creation(s) or document the prerequisite?

Draft only; not posted.

## Outcome

Actionable gap found on the pinned live head; awaiting approval before any
GitHub action.

## Regret

None recorded.

## Follow-up

If approved, run the closed-owner-ATA test against a validator before posting.
