# Uniswap/v4-periphery #593 — Decode V4 swap parameters with `abi.decode`

- **Status:** closed
- **PR:** https://github.com/Uniswap/v4-periphery/pull/593
- **Head reviewed:** `0cc6e1644b1068ace65af019ea58ce91da03a2ce`
- **Selected:** 2026-08-30 because it claims calldata-length integrity across several nested dynamic encodings.
- **Time spent:** not recorded

## Claim

V4 swap decoding cannot consume trailing bytes outside the authenticated
parameter slice, including through nested path and hook-data offsets.

## Evidence ledger

### Observed

At `0cc6e164`, all four swap decoders return memory structs from
`abi.decode(params, (...))`; the router, `_swap`, and `PathKey` helper propagate
the memory data location. The 17 new raw-calldata tests cover the top-level
struct offset, dynamic arrays, array elements, nested `hookData`, crossing the
declared slice, in-bounds noncanonical offsets, and minimum-head behavior. Gas,
bytecode, and router snapshots were updated.

### Inferred

The main correctness boundary is nested ABI offset validation. The operational
boundary is whether the size increase survives composition with pending router
features.

### Attested

The author reports that 10 tests fail on the prior implementation and that the
full isolated suite passes on the proposed head.

### Unknown

Foundry is not installed in the review environment, so the adversarial tests and
the downstream Universal Router size measurement were not independently run.
The reported 76-byte deployment margin is narrow, but the PR states the tradeoff
explicitly and no current conflicting downstream revision was established.

## Distinguishing test

The new raw-calldata matrix contains predicted base/head distinguishing cases
for every described escape. Source inspection confirms that `abi.decode`
receives the bounded `params` slice. Tests were not locally run because Foundry
is unavailable.

## Potential contribution

No useful comment found. The test matrix targets the transitive nested-offset
property rather than only the original examples, and the operational bytecode
tradeoff is already surfaced for maintainer decision.

## Outcome

Review complete on the pinned live head; no public comment proposed.

## Regret

None recorded.

## Follow-up

Reopen if the focused suite fails, the live head changes, or a current
Universal Router composition exceeds EIP-170.
