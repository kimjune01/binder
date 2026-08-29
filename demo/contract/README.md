# Escrow contract example

This example begins with a human judgment about a contract change:

> The claim is warranted when the check refutes it on the base revision and it
> stands on the candidate revision.

The manifest records that entitlement rule and attributes it to the escrow
maintainers. Binder does not invent this edge from a passing command.

On command, Binder materializes the rest of the claim-specific evidence graph.
It resolves both Git revisions, overlays the candidate check, compiles and runs
the contract kernel, captures balance witnesses, evaluates the authored rule,
and writes a content-addressed receipt.

Run it from the repository root:

```sh
bash demo/contract/run.sh
```

The JSON result is intended for agents and CI. Read it in this order:

1. `claim_id`, `statement`, and `entitlement` preserve the human judgment.
2. `subject` identifies the exact base and candidate commits.
3. `trials` records execution, observations, and concrete balance witnesses.
4. `policy` says whether the materialized evidence satisfies the authored rule.
5. `receipt_digest` identifies the preserved result under `.binder/receipts/`.
