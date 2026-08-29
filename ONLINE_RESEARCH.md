# Binder: observed complaints and situations

_Research scan: 2026-08-29._

## Finding

There is strong evidence for two Binder jobs:

1. determine whether a passing check actually distinguishes a proposed fix from the bug; and
2. keep review or audit evidence attached to the exact code, artifact, and execution boundary it covered.

There is weaker evidence that teams want to author a new claim manifest, download replay bundles, or consume a universal program-status API. Those remain validation questions rather than established demand.

## 1. Passing agent checks often do not test the bug

A 2026 controlled study analyzed 3,730 validation events from 643 coding-agent rollouts on 110 real repair tasks. Of the positive comparable events, 46.0% were regression-only or misleading: they also passed on the original buggy code. In baseline runs, 23.8% of submitted patches had no positive evidence that distinguished the patch from the bug. Replaying checks on the buggy state reduced that inadequate closure rate, although the authors caution that the practical effect size remains uncertain.

This closely matches Binder's strongest current primitive: execute the same check on base and head and refuse to interpret a head-only pass as sufficient evidence.

Source: [Validation Evidence in LLM Repair Agents](https://arxiv.org/abs/2607.28871)

Firsthand developer discussion points in the same direction. One developer reported that agents changed previously correct tests to make them pass and rationalized failures as external causes. This is anecdotal, but it describes the human-review problem Binder should test directly.

Source: [“LLM generated tests in my experience are really poor”](https://news.ycombinator.com/item?id=46585740)

## 2. Teams manually track audit scope and post-audit delta

The Solana Foundation's `subscriptions` repository maintains a dedicated audit-status document with the audited-through commit, the commit through which fixes were verified, compare links to `main`, and shell commands for inspecting the subsequent delta. It explicitly notes that `main` may contain both audited and unaudited commits.

This is not hypothetical demand: it is a maintained, manual version of Binder's freshness model. Binder would need to prove that claim-level invalidation is more useful than these commit-level compare links.

Source: [Solana Foundation subscriptions audit status](https://github.com/solana-foundation/subscriptions/blob/main/audits/AUDIT_STATUS.md)

A developer running a Compound-derived lending protocol asked whether a few changed lines justified paying for another audit. The concrete job is deciding what prior assurance survives a small delta—not simply finding a cheaper auditor.

Source: [“Smart Contract Audit with few changes”](https://www.reddit.com/r/ethdev/comments/14soigu)

## 3. Post-audit changes create real scope failures

SubQuery's own incident report says a missing access-control modifier was introduced during a refactor after an audit. A later targeted audit covered other components and did not re-examine the changed Settings contract. The issue was eventually exploited, affecting pooled balances and 272 wallets.

Binder would not automatically discover the missing modifier. Its relevant promise is narrower: if a maintained access-control claim depended on the refactored code, the prior evidence should become stale and require a rerun.

Source: [SubQuery Network security incident report](https://subquery.network/blog/subquery-network-security-incident-report)

The PAID Network incident illustrates source/deployment identity drift from another angle. CertiK reported that the exploited burn and mint functions were absent from the audited contract and were introduced when compromised proxy control replaced the deployed implementation. This was primarily a key-management incident, not a validation failure, but it shows why an audit badge without deployed-code identity can mislead consumers.

Source: [CertiK PAID Network post-mortem](https://www.certik.com/blog/paid-network-post-mortem)

## 4. Evidence engines are not interchangeable

A LiteSVM issue reported roughly 20% higher compute-unit use than Mollusk for the same two-hop swap. The issue attributes the difference to execution boundaries: LiteSVM processed the full transaction while Mollusk processed individual instructions. Both executions succeeded, but their measurements meant different things.

Source: [LiteSVM compute-unit discrepancy](https://github.com/LiteSVM/litesvm/issues/277)

The Solana Foundation's crypto-primitives examples use Mollusk because its pinned Agave runtime exposes required curve syscalls while the available Surfpool and LiteSVM versions do not. This is a concrete reason to preserve engine, version, and runtime boundary with a result rather than collapsing everything into “tests pass.”

Source: [Solana crypto-primitives examples](https://github.com/solana-foundation/crypto-primitives-examples)

## 5. Cost and noise are real, but they do not validate Binder directly

Developers report audit quotes ranging from several thousand dollars to $20,000 or more for small contracts. Others report that AI audit tools produce obvious false positives, thin reports, and excessive noise. These complaints validate the cost of making assurance decisions, but Binder is not an auditor or scanner and should not claim to solve either problem directly.

Sources:

- [Current audit landscape](https://www.reddit.com/r/ethdev/comments/1ebhbnb/current_audit_landscape/)
- [AI smart-contract audit tools discussion](https://www.reddit.com/r/ethdev/comments/1r6z12x/ai_smart_contract_audit_tools_anyone_found_one/)

## Product implications

### Supported now

- Base/head contrast is the clearest differentiated value, especially for agent-authored fixes.
- Freshness should initially target audit remediation and consequential refactors.
- Every evidence row must name its engine and execution boundary.
- Deployed-artifact identity matters eventually, but it is not required to test the review workflow.

### Still unproven

- Teams will maintain explicit claim manifests.
- Reviewers will replay bundles rather than trust CI.
- Claim-level dependency mapping saves enough work over commit diffs.
- Wallets and explorers want claim-level data rather than simpler audit/build status.
- Teams will pay for a hosted registry.

## Validation cases to prioritize

1. **Agent fix with weak validation:** use a real repair where the agent's passing check also passes on the buggy base.
2. **Post-audit refactor:** use a change to previously audited code and ask which claims require renewed evidence.
3. **Engine-bound result:** present two successful simulations with materially different observations and ask whether the report prevents overgeneralization.
4. **Deployment mismatch:** show reviewed source whose deployed artifact differs, without implying Binder can identify why.

The first study should concentrate on cases 1 and 2. They have the strongest evidence of an existing job and are already supported by the local demo's core mechanics.

