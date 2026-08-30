# Uniswap/v4-periphery #593 — Decode V4 swap parameters with `abi.decode`

- **Status:** queued
- **PR:** https://github.com/Uniswap/v4-periphery/pull/593
- **Head reviewed:** `0cc6e1644b1068ace65af019ea58ce91da03a2ce`
- **Selected:** 2026-08-30 because it claims calldata-length integrity across several nested dynamic encodings.
- **Time spent:** selection only; audit not started

## Claim

V4 swap decoding cannot consume trailing bytes outside the authenticated
parameter slice, including through nested path and hook-data offsets.

## Evidence ledger

### Observed

The PR replaces assembly-backed calldata struct views with `abi.decode`, adds
17 focused tests, and reports a 408-byte Universal Router runtime increase with
76 bytes remaining below the deployment limit.

### Inferred

The main correctness boundary is nested ABI offset validation. The operational
boundary is whether the size increase survives composition with pending router
features.

### Attested

The author reports that 10 tests fail on the prior implementation and that the
full isolated suite passes on the proposed head.

### Unknown

The adversarial encodings and exact Universal Router size measurement have not
been independently reproduced.

## Distinguishing test

To determine during review: generate noncanonical and overlapping nested
offsets that remain within calldata but cross the declared `bytes` slice, and
compare old and new heads. Not yet run.

## Potential contribution

Not drafted.

## Outcome

Not reviewed.

## Regret

None recorded.

## Follow-up

Audit the ABI-layout test matrix first; only raise contract-size headroom if a
current downstream build establishes a concrete conflict.
