# strapi/strapi #27088 — Ignore invalid document IDs

- **Status:** posted — awaiting response
- **PR:** https://github.com/strapi/strapi/pull/27088
- **Comment:** https://github.com/strapi/strapi/pull/27088#issuecomment-5470638609
- **Head reviewed:** `8a3d895c0d0495b8611753ed69c7c5193018dabc`
- **Selected:** 2026-08-30 because invalid persisted document IDs create records that cannot be managed normally.
- **Time spent:** not recorded

## Claim

Create and update cannot persist invalid document IDs; missing IDs are generated
and valid explicit IDs are preserved.

## Evidence ledger

### Observed

At `8a3d895c`, both new helpers define an explicit ID as any value other than
`null` or `undefined`, so they preserve `documentId: ''`. The pre-existing
uniqueness check is `if (data.documentId)`, so it does not run for the empty
string. The generated model declares `documentId` as a string with a default;
no non-empty validation or database constraint was found.

### Inferred

An empty ID bypasses generation and persists another unusable document identity,
the same defect class as the reported null case. The repair invariant should be
a non-empty document ID rather than merely a non-null one.

### Attested

Issue #27085 reports that a null ID creates an entity that cannot be updated or
deleted. The PR says it prevents null IDs while keeping valid explicit IDs.

### Unknown

The empty-string case was established from the create path and model contract,
not run against a full Strapi instance. Whitespace-only IDs were not assessed.

## Distinguishing test

Repeat the PR's create and update regressions with `documentId: ''`. Creation
should generate a non-empty ID, and update should preserve the existing ID. The
current helpers preserve the empty input instead.

## Potential contribution

Posted on 2026-08-30 after confirming the live head still matched the pinned
revision.

## Outcome

Sibling corruption path identified and posted; awaiting maintainer response.

## Regret

None recorded.

## Follow-up

Watch for an expanded normalization rule, regression tests, scope clarification,
or evidence that a downstream layer rejects the empty string.
