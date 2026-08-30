# solana-foundation/pay #448 — Add support for Token-2022 transfer instructions

- **Status:** posted
- **PR:** https://github.com/solana-foundation/pay/pull/448
- **Head reviewed:** not recorded
- **Selected:** 2026-08-30 because the PR's classifier could be compared with a generated upstream instruction enum.
- **Time spent:** not recorded

## Claim

The PR adds Token-2022 transfer-instruction support.

## Evidence ledger

### Observed

The dependency's generated `Token2022Instruction` enum and
`identifyToken2022Instruction` include `Transfer`, `TransferChecked`, and
`TransferCheckedWithFee`. The PR path used the legacy classifier for both token
programs.

### Inferred

A `TransferCheckedWithFee` instruction would be rejected on the reviewed code
path.

### Attested

The PR describes support for Token-2022 transfer instructions.

### Unknown

Whether fee-bearing transfers were intentionally out of scope and whether the
PR changed after review.

## Distinguishing test

Proposed, not run: construct a `TransferCheckedWithFee` instruction and assert
that the parser accepts and classifies it under the intended transfer semantics.

## Potential contribution

[Posted comment](https://github.com/solana-foundation/pay/pull/448#issuecomment-5470002985)
asking the maintainers either to handle the fee-bearing transfer or document it
as out of scope.

## Outcome

Awaiting maintainer response.

## Regret

None recorded.

## Follow-up

Record the exact head revision in future reviews. Recheck the PR before drawing
any conclusion from the response.
