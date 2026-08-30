# solana-program/token-2022 #1402 — Base confidential-mint supply on its ciphertext

- **Status:** queued
- **PR:** https://github.com/solana-program/token-2022/pull/1402
- **Head reviewed:** `1feea7b7f8e30660e66b8ce4de2717a2ae3ec4d2`
- **Selected:** 2026-08-30 because it claims synchronization across two representations of confidential supply.
- **Time spent:** selection only; audit not started

## Claim

Confidential mint proof generation derives the new supply consistently after
pending burns or a nonzero initialized decryptable supply.

## Evidence ledger

### Observed

The PR changes one helper, adds a mint/burn/apply/mint regression test, and also
adds two audit-advisory ignores to the Makefile.

### Inferred

Useful review targets include key ownership assumptions, boundary amounts, and
whether decrypting the confidential supply is valid in every public helper path.

### Attested

The author says the prior helper paired a commitment derived from decryptable
supply with a ciphertext derived from confidential supply, causing equality
proof rejection when they diverged.

### Unknown

The proof construction and new regression have not been independently run. The
necessity and scope of the unrelated audit ignores have not been verified.

## Distinguishing test

To determine during review: vary initialization, pending-burn state, and amount
boundaries while asserting both proof acceptance and supply synchronization.
Not yet run.

## Potential contribution

Not drafted.

## Outcome

Not reviewed.

## Regret

None recorded.

## Follow-up

Inspect the helper's preconditions and separate review of the functional fix
from the dependency-audit policy change.
