# solana-program/token-2022 #1384 — Recover nested associated token accounts

- **Status:** validated draft contribution
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

It remains unknown whether maintainers intentionally expect callers to recreate
the owner and destination ATAs with separate CLI commands.

## Distinguishing test

Run on 2026-08-30 against a local test validator from a clean checkout of the
pinned head. The PR's integration test was modified only to close the empty
owner ATA immediately before invoking `spl-token recover-nested`. The first
case (`Token-2022`, distinct mints) failed before transfer with `Provided owner
is not allowed`; the Associated Token Account program logged: “Owner associated
token account not owned by provided token program, recreate the owner associated
token account first.” The remaining matrix cases did not run because the first
failure stopped the test. The PR remained open and its live head still matched
the pinned SHA.

## Potential contribution

> `recover-nested` fails in the closed-owner-ATA case described by this PR. I
> closed the empty owner ATA before the CLI call, and the command failed with
> `Provided owner is not allowed`. The ATA processor said to recreate the owner
> ATA first.
>
> The current test leaves that account open, and the command submits only the
> `recover_nested` instruction. In the `same_mint` case, the missing owner ATA is
> also the destination.
>
> The closed-account setup should be covered by the test. The required ATA(s)
> should either be recreated by the command or documented as a prerequisite.

Draft only; not posted.

## Outcome

Actionable gap reproduced on a local validator at the pinned live head; awaiting
approval before any GitHub action.

## Regret

None recorded.

## Follow-up

If approved, recheck the live head and post the validated contribution.
