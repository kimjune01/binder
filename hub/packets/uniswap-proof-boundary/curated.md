# Curated: Uniswap proof boundary

The engagement wrote `act` specifications and proved reachability claims
against built bytecode. The report says these specifications describe almost
all single-call transitions. It expressly says they do not prove multi-call
contract-level invariants such as `x*y=k`, and assumes token semantics matching
Uniswap's token implementation. It separately lists incomplete treatment of
`sqrt`, unknown external calls, and several unspecified behaviors.

Boundary: “formal” describes the evidence method, not unlimited scope.
