# Binder public cases

This is the five-case Phase 0 research hub defined in [ROADMAP.md](../ROADMAP.md).
It is a sourced static collection, not a registry or safety-status service.

Install and build it from the repository root:

```sh
pnpm install
pnpm hub:build
```

For local development, first check whether port 4321 is free, then run
`pnpm hub:dev`. The production build is emitted to `hub/dist/`.

Each JSON fixture has four validation packets: raw-source control, curated
view, fixed questions, and answer key. Factual edges require citations;
inferred and missing edges remain visibly labeled. Rust remains the Binder CLI
and receipt implementation; versioned JSON is the interface between it and the
Astro hub.
