# Contribution worklog

This ledger records public review contributions made with Binder's method. It is
validation data, not a list of activity: log what was established, how the
maintainer responded, and whether the contribution changed code, tests, or
scope.

## What to record

For each contribution, capture:

- **Target:** repository, PR, and exact revision reviewed.
- **Claim:** the behavior the PR appeared to promise.
- **Contribution:** the smallest consequential gap raised.
- **Evidence:** what was observed, inferred, attested, and still unknown.
- **Distinguishing test:** whether it was run or only proposed.
- **Cost:** time from opening the PR to a postable comment.
- **Outcome:** acknowledged, code changed, test added, scope clarified, rejected,
  no response, or withdrawn.
- **Regret:** whether later evidence made the comment misleading or unnecessary.
- **Follow-up:** the next question worth testing.

Do not count comments as successes by themselves. The primary signal is a code,
test, or explicit scope change. Maintainer acknowledgment is a weaker positive
signal; silence is inconclusive. Record false positives and withdrawn comments.

## Contributions

### 2026-08-30 — solana-foundation/pay #448

- **Target:** [Add support for Token-2022 transfer instructions](https://github.com/solana-foundation/pay/pull/448); exact reviewed revision was not recorded.
- **Claim:** the PR adds Token-2022 transfer-instruction support.
- **Contribution:** [comment](https://github.com/solana-foundation/pay/pull/448#issuecomment-5470002985) identifying `TransferCheckedWithFee` as a legitimate Token-2022 transfer classification that appeared to be rejected by the legacy instruction classifier.
- **Observed:** the dependency's generated `Token2022Instruction` enum and `identifyToken2022Instruction` include `Transfer`, `TransferChecked`, and `TransferCheckedWithFee`. The PR path used the legacy classifier for both token programs.
- **Inferred:** a `TransferCheckedWithFee` instruction would be rejected on the reviewed code path.
- **Attested:** the PR describes support for Token-2022 transfer instructions.
- **Unknown:** whether fee-bearing transfers were intentionally out of scope and whether the PR changed after review.
- **Distinguishing test:** proposed, not run: construct a `TransferCheckedWithFee` instruction and assert that the parser accepts and classifies it under the intended transfer semantics.
- **Cost:** not recorded.
- **Outcome:** awaiting maintainer response.
- **Regret:** none recorded.
- **Follow-up:** record the exact head revision in future reviews before analysis; update this entry if the maintainer fixes the case or narrows the documented scope.

## Summary

| Contributions | Code changes | Tests added | Scope clarified | Acknowledged only | Rejected | No response | Withdrawn |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |

Update the summary only when an outcome is observed. “Awaiting response” is not
the same as “no response”; choose a review window before classifying silence.
