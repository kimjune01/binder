# White-hat scanning log

This ledger prevents duplicate research and preserves the authorization boundary
for bounty work. It is an index, not a vulnerability database. Never commit an
unresolved vulnerability, exploit, or sensitive reproduction detail here.
Keep private working evidence under `.binder/whitehat/`, which is gitignored.

The first-success criterion is a valid private report that causes a fix, test,
advisory, scope clarification, or named acknowledgment. Payment is optional.

## Rules

1. Snapshot the live program policy before reviewing or testing.
2. Confirm the exact repository, release, deployment, impact, and testing method
   are in scope. A public repository alone is not authorization.
3. Use local forks, local validators, or other explicitly permitted environments.
   Never test mainnet, public testnets, hosted services, or user accounts without
   written authorization.
4. Record a hypothesis here before sustained work so it is not repeated later.
5. Search audits, advisories, issues, PRs, and prior disclosures before building
   a PoC.
6. Report a credible vulnerability promptly through the program's private
   channel. Never open a public issue or PR first.
7. Do not submit theory. Establish reachability, attacker control, reproducible
   impact, affected production versions, and why existing mitigations fail.
8. Keep the report confidential until the program gives written permission to
   disclose it.

## Target queue

| ID | Target | Status | Why | Authorization / friction |
|---|---|---|---|---|
| WH-001 | Strapi released open-source code | first candidate | TypeScript/JavaScript fit; rich authorization, plugin, upload, and API boundaries; private GitHub advisory channel with explicit reporter credit | Security policy checked 2026-08-30. Use private GitHub Security Advisories and local instances only. Confirm supported affected versions and the policy's report requirements before testing. No payout assumed. |
| WH-002 | Uniswap v4 periphery and supporting contracts | secondary candidate | Existing repo familiarity; clear authorization, accounting, calldata, settlement, and signature boundaries | Live Cantina bounty checked 2026-08-30. Local Foundry testing only; no mainnet, public testnet, or hosted-service testing. Functional PoC required. KYC and $50 deposit required to submit, and public disclosure is usually not permitted. Exact scoped contract must be confirmed before work. |
| WH-003 | Eclipse released SVM/L2 code | blocked — policy unavailable | Strong Rust/Solana fit and an official page advertises an Immunefi bounty | Official documentation prohibits public-network testing, but its linked Immunefi program page did not resolve on 2026-08-30. Do not begin until the live scope, assets, rewards, and reporting channel are available and snapshotted. |
| WH-004 | Cosmos released stack | defer — advanced PoC burden | Source code is explicitly in scope and the program is currently funded and triaged | Live Immunefi policy checked 2026-08-30. KYC and pay-to-submit apply. All reports need code PoCs; medium and above require an external, end-to-end exploit on a local four-node network. Only tagged, actively maintained releases qualify. |

## Scan ledger

| ID | Target revision | Security claim / hypothesis | Policy snapshot | Duplicate search | Test status | Outcome | Next action |
|---|---|---|---|---|---|---|---|
| — | — | — | — | — | — | — | Start WH-001 by mapping supported Strapi releases to recent authorization and input-handling PRs. |

## Candidate gate

A candidate advances to private PoC work only when all answers are yes:

- Is the exact asset and released version bounty-eligible?
- Is the allowed testing method explicit?
- Can an unprivileged attacker reach the behavior?
- Is there concrete security impact rather than incorrect behavior alone?
- Is the issue absent from audits, known issues, advisories, and public reports?
- Can the impact be reproduced locally without touching real systems or funds?
- Would the evidence let the maintainer reproduce and fix it?

If any answer is unknown, investigate that question before testing further.

## Report lifecycle

`candidate → policy-cleared → duplicate-cleared → locally reproduced → privately submitted → triaged → fixed/rejected → disclosure approved/closed`

Record dates and exact revisions at every transition. “Rejected” and “duplicate”
are useful outcomes; they improve future target selection.
