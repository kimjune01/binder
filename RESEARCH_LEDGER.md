# Binder research ledger

No question in this ledger currently justifies contacting a person. Update the
public-evidence column before changing that status.

| Question | Best public evidence so far | Confidence | Next public search | Conversation eligible? |
| --- | --- | --- | --- | --- |
| Do teams reconstruct finding → fix → test → reviewed revision manually? | Solana subscriptions maintains an explicit audit-status file, compare links, commit ranges, and report; remediation advice independently recommends one finding/commit/test because unstructured review expands toward re-audit. | Medium | Sample 10 public audit repositories and measure how often the chain is explicit and machine-readable. | No |
| Do existing tools collapse materially different verification claims? | Stellar SEP-55/58 discussions explicitly separate attestation, reproducibility, trusted build environment, public source, and safety. Ethereum documentation separates source verification from correctness. ERC-8004 separates identity from capability and benign behavior. | High | Compare badge and API semantics in Sourcify, Etherscan, Solana Verified Builds, and Stellar proposals. | No |
| Is engine identity part of the meaning of empirical evidence? | LiteSVM issue 277 documents disagreement with Mollusk/on-chain execution and requests a reproducer before interpreting the result. | Medium | Find resolved cross-engine discrepancies and whether downstream CI records engine/version. | No |
| Who authors a contract invariant? | OpenZeppelin issues allocate assumptions among library/API/docs/application; Crucible sidecar proposes AI-authored invariants with provenance; formal-verification reports declare assumptions manually. ERC-8273 deliberately leaves attestation semantics and attestor trust to integrating systems. | High | Inspect invariant ownership in a sample of production repositories to measure convention, not discover the conceptual roles. | No |
| Does base/head contrast catch weak regression evidence in practice? | The subscriptions remediation provides a head regression but no preserved base witness; Binder's synthetic case demonstrates the distinction. | Low | Mine security-fix PRs for tests added with the fix and replay them, where feasible, against the parent or audited commit. | No |
| How much active time does evidence reconstruction cost? | No reliable measurement yet. Public review timestamps are elapsed time, not active time. | Low | Search auditor remediation policies, engagement retrospectives, invoices/pricing, and PR review traces for quantified effort. | No |
| Would a Binder receipt change an approval decision? | No direct evidence; neighboring standards discussions show schema fields change trust interpretation, but not a merge decision. | Low | Find adoption or evaluation data for proof-carrying PRs, differential tests, audit-trail bots, and build attestations. | No |
| Who would maintain the claim after merge? | Public proposals discuss maintainer, auditor, builder, registry, and application-owner authority but do not converge. | Low | Trace ownership of invariant files and audit-status manifests across repository history. | No |
| Will teams retain this workflow for a second change? | No Binder-specific evidence. | Low | Study retention and abandonment of comparable provenance/security CI integrations in public repositories. | No |
| Does agent infrastructure need claim-level authorization? | ERC-8273 binds authorization to capability plus a concrete action digest; Solana wallet policies separate API-key authority from wallet operations and record policy revisions; ERC-8004 explicitly cannot establish benign capability from identity alone. | High | Compare transaction-intent, wallet-policy, and attestation schemas for a precise Binder export boundary. | No |

## Promotion rule

A question may move to conversation only when:

1. at least three materially different public searches have been attempted;
2. the remaining fact is private/tacit, evidence is genuinely contradictory, or
   the research task is to observe use of a novel Binder artifact;
3. the exact question and the decision it affects are written here; and
4. asking is cheaper for the participant than another reasonable public search.
