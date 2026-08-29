# Curated: reproducible Wasm

The prototype records a build image and options, rebuilds referenced source in
that environment, and compares the result with deployed Wasm. A public example
for Wasm hash `e7089d…` reproduced successfully. Participants distinguish this
from attestation and image trust: a digest-pinned but hostile compiler can
reproducibly inject behavior.

Boundary: byte equality establishes reproducibility for the selected source and
environment. It does not establish compiler faithfulness, provenance, or safe
behavior.
