# Binder public cases

This is the five-case Phase 0 research hub defined in [ROADMAP.md](../ROADMAP.md).
It is a sourced static collection, not a registry or safety-status service.

Build it with:

```sh
cargo run --quiet -p binder-hub
```

Then open `hub/dist/index.html`. Pass another output directory as the first
argument to render elsewhere.

Each JSON fixture has four validation packets: raw-source control, curated
view, fixed questions, and answer key. Factual edges require citations;
inferred and missing edges remain visibly labeled.
