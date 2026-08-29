# Curated: Hyperbridge out-of-bounds leaf

The verifier consumed a legitimate leaf but silently left a forged
out-of-bounds leaf in the iterator, then returned a valid root. Downstream code
treated both inputs as verified. PR 51 added a final invariant requiring no
leaves to remain. The postmortem reports an internal review, an SRLabs audit,
and redeployment against v1.0.0; those reviews also found related duplicate
index vulnerabilities.

Boundary: the chain strongly supports the named library remediation. It does
not itself bind every live deployment or prove the whole verification stack
sound.
