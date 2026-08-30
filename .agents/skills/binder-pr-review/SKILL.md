---
name: binder-pr-review
description: Inspect a GitHub pull request for one concrete, source-grounded review contribution—preferably a reproducible counterexample, missing valid case, or claim/test mismatch—and draft a concise comment. Use when reviewing a live or historical PR to find a useful gap. Never post without explicit user approval.
---

# Binder PR Review

Turn an opaque review into a small, checkable contribution. Optimize for truth and usefulness, not for finding a problem.

## Review the exact change

1. Read the repository's instructions. Use `gh` to inspect the PR body, exact head SHA, changed files, commits, checks, reviews, and discussion.
2. State the PR's consequential claim in plain language. Distinguish what the PR claims from what reviewers or tests actually establish.
3. Inspect the relevant implementation and tests. Consult authoritative upstream source, generated types, specifications, or documentation when behavior depends on an external interface.
4. Look for one material gap, in this order:
   - a counterexample that still fails on the current head;
   - a legitimate input omitted by an allowlist, classifier, enum, parser, or dispatch path;
   - a test that does not distinguish the base revision from the proposed revision;
   - a claim broader than the tested behavior;
   - a review conclusion made stale by later commits;
   - an artifact, deployment, or configuration that differs from the reviewed source.
5. Prefer proving the gap with the smallest failing test on the PR's current head. Follow the target repository's test instructions and avoid changing the contributor's working tree; use a temporary clone or worktree when execution is needed.
6. If there is no concrete, consequential gap, report that honestly. Do not turn a speculative caveat into a public comment.

## Keep an evidence ledger

Classify every important statement:

- **Observed:** directly read or executed, with file, line, command, result, and revision where practical.
- **Inferred:** follows from observed source but was not executed; state the reasoning.
- **Attested:** claimed by an author, reviewer, bot, or document.
- **Unknown:** required to settle the question but not established.

Never say a test fails unless it was run on the stated revision. When execution is impractical, offer a predicted failing test and label it as a prediction.

## Produce a contribution brief

Return these sections, compactly:

1. **Verdict:** `actionable gap`, `scope question`, or `no useful comment found`.
2. **Claim:** what the PR appears to promise.
3. **Evidence:** only the facts needed to support the verdict, with exact links or locations.
4. **Distinguishing test:** a minimal test that should pass or fail, clearly saying whether it was run.
5. **Draft comment:** outcome first, then mechanism, test, and the smallest useful fix or scope clarification.
6. **Confidence and unknowns:** what could overturn the conclusion.

Use plain project language in the draft. Do not mention Binder, epistemic rigor, reputation, or personal promotion unless the repository maintainer asks. Do not call the result a security audit. One strong point is better than a list of weak observations.

Prefer this comment shape:

> This appears to miss **[specific valid case or counterexample]**. **[Current code path]** handles **[cases]**, but **[authoritative interface or observed behavior]** also includes **[missing case]**, so **[consequence]**. A focused regression test would **[setup and expected result]**; on the current head I **[observed it fail / predict it will fail]**. Would you prefer to handle that case here or document it as out of scope?

## Persist the review

When working in the Binder repository, create or update
`contributions/<owner>-<repo>-<pr>.md`. Create the record as soon as a PR enters
the queue; do not wait for a publishable finding. Use this structure:

```markdown
# <owner>/<repo> #<number> — <title>

- **Status:** queued | reviewing | draft contribution | posted | closed
- **PR:** <url>
- **Head reviewed:** `<full SHA>` or `not yet reviewed`
- **Selected:** <date and reason>
- **Time spent:** <duration or not recorded>

## Claim
## Evidence ledger
### Observed
### Inferred
### Attested
### Unknown
## Distinguishing test
## Potential contribution
## Outcome
## Regret
## Follow-up
```

Treat the file as a living record. Pin every substantive conclusion to the
exact head SHA. If the PR changes, preserve the old conclusion and add a dated
recheck rather than silently rewriting history. Label proposed tests and draft
comments as unverified until run. Link the record from `WORKLOG.md`, but keep
that file as a compact index and aggregate outcome table.

## External-action boundary

Drafting and local verification are allowed. Before posting, editing, submitting a review, pushing a branch, or otherwise acting on GitHub, show the exact proposed text and obtain explicit user approval for that action. Treat approval as specific to the shown text and target PR.

For historical closed PRs, default to analysis only. For live PRs, check immediately before posting that the head revision and relevant discussion have not changed.
