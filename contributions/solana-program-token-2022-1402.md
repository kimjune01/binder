# solana-program/token-2022 #1402 — Base confidential-mint supply on its ciphertext

- **Status:** validated draft contribution
- **PR:** https://github.com/solana-program/token-2022/pull/1402
- **Head reviewed:** `1feea7b7f8e30660e66b8ce4de2717a2ae3ec4d2`
- **Selected:** 2026-08-30 because it claims synchronization across two representations of confidential supply.
- **Time spent:** not recorded

## Claim

Confidential mint proof generation derives the new supply consistently after
pending burns or a nonzero initialized decryptable supply.

## Evidence ledger

### Observed

At `1feea7b7`, `buildConfidentialMintProofPlan` now decrypts
`confidentialSupply`, specifically so minting works after `ApplyPendingBurn`
without a manual AES-supply re-sync
([lines 1541-1549](https://github.com/solana-program/token-2022/blob/1feea7b7f8e30660e66b8ce4de2717a2ae3ec4d2/clients/js/src/confidentialTransferHelpers.ts#L1541-L1549)).
The new regression exercises that exact sequence.

The exported `getConfidentialMintInstructionPlan` documentation still says the
two representations “must be in sync,” says the proof will be rejected after
`ApplyPendingBurn`, and instructs callers to re-sync first
([lines 1693-1698](https://github.com/solana-program/token-2022/blob/1feea7b7f8e30660e66b8ce4de2717a2ae3ec4d2/clients/js/src/confidentialTransferHelpers.ts#L1693-L1698)).

### Inferred

The public API documentation now states the pre-fix behavior and can make users
perform the extra update that this PR is intended to remove.

### Attested

The author says the prior helper paired a commitment derived from decryptable
supply with a ciphertext derived from confidential supply, causing equality
proof rejection when they diverged.

### Unknown

The proof regression has not been independently run. It is unknown whether the
same stale warning appears in generated API documentation or other guides.

## Distinguishing test

Source-level distinguishing check, rerun on 2026-08-30 in a clean checkout of
the pinned head: the JSDoc describes `AES_decrypt(decryptableSupply) + amount`,
while the implementation at the same head constructs the proof from decrypted
`confidentialSupply`. The PR remained open and its live head still matched the
pinned SHA. No runtime test is needed to establish the documentation
contradiction.

## Potential contribution

> The public helper docs still describe the bug this PR fixes: they say the two
> supply representations must be synchronized, that minting after
> `ApplyPendingBurn` is rejected, and that callers must manually re-sync first.
> `buildConfidentialMintProofPlan` now deliberately derives from
> `confidentialSupply`, and the new test establishes that the manual re-sync is
> no longer required. Could we update this JSDoc so callers do not keep the
> obsolete workaround?

Draft only; not posted.

## Outcome

Actionable documentation gap validated on the pinned live head; awaiting
approval before any GitHub action.

## Regret

None recorded.

## Follow-up

If approved, recheck the live head and post the documentation comment.
