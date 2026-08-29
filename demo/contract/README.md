# Escrow contract example

This example asks Binder a narrow question about a contract change: does an
escrow release now require both parties' approval?

The base Git revision releases funds after only one approval. The head revision
returns without moving funds. Binder overlays the candidate check onto both
revisions, compiles and runs it, and warrants the change only when the check
refutes the base and stands on the head.

Run it from the repository root:

```sh
bash demo/contract/run.sh
```

The JSON result is intended for agents and CI. It distinguishes trial execution
from the observation, includes the concrete balance witness for each revision,
and records a content-addressed receipt under `.binder/receipts/`.
