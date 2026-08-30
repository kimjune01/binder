# strapi/strapi #26619 — Include the Content API prefix in generated paths

- **Status:** drafted — not posted
- **PR:** https://github.com/strapi/strapi/pull/26619
- **Head reviewed:** `5b6ca3db4949d96c083f43c47ed8bd74e6df2a7f`
- **Selected:** 2026-08-30 because generated OpenAPI paths should exactly match runtime routing under every supported REST prefix.
- **Time spent:** not recorded

## Claim

Generated API and plugin paths include the configured `api.rest.prefix`
without double-prefixing routes that already contain it.

## Evidence ledger

### Observed

At `5b6ca3db`, `withContentApiPrefix` adds a leading slash to the configured
prefix but does not remove trailing slashes. Its already-prefixed check compares
`/api/articles` with `/api/` and `/api//`; neither matches. It then joins
`/api/` and `/api/articles`, collapses repeated slashes, and emits
`/api/api/articles`.

Strapi's configuration loader accepts `api.rest.prefix` as supplied; no schema
or loader normalization was found that rejects or trims a trailing slash. The
runtime server also receives that configured value directly.

### Inferred

A trailing-slash prefix is semantically equivalent to the same prefix without
the slash for this join, so OpenAPI generation should normalize it before both
the containment check and concatenation.

### Attested

PR #26619 says already-prefixed paths are left unchanged and custom content API
prefixes are supported.

### Unknown

The focused package tests were not run locally. The exact runtime URL produced
by the router for a trailing-slash prefix has not been exercised end to end.

## Distinguishing test

Add a provider case with `api.rest.prefix = '/api/'` and an input route path of
`/api/articles`; expect `/api/articles`, not `/api/api/articles`. The current
implementation fails by direct evaluation of its normalization and join logic.

## Potential contribution

> The already-prefixed guard misses configured prefixes with a trailing slash.
> With `api.rest.prefix = '/api/'` and route path `/api/articles`,
> `normalizedPrefix` remains `/api/`, so neither equality nor
> `startsWith(`${normalizedPrefix}/`)` matches. The subsequent join collapses
> `/api//api/articles` to `/api/api/articles`.
>
> Could we trim trailing slashes from the prefix before the guard and add this
> case to the custom-prefix test? That would preserve the PR's no-double-prefix
> guarantee for both `/api` and `/api/` configurations.

## Outcome

Actionable edge case validated against the pinned head; comment awaiting user
approval.

## Regret

None recorded.

## Follow-up

Recheck the live head, post the comment if unchanged, and watch for a test or
normalization change.
