# Binder case checklist

Binder uses this checklist to turn a provenance-preserving chronology into a
compressed current view and a set of inspectable recommendations. It evaluates
the support for a bounded claim. It is not a comprehensive security audit.

## 1. Fix the subject

- [ ] Name the exact contract, repository, revision, build, or deployment.
- [ ] Keep reviewed source, built artifact, and deployed artifact distinct.
- [ ] Record stable identifiers or digests wherever they exist.

## 2. State the claim

- [ ] Write one consequential claim in language a decision-maker can understand.
- [ ] Identify the decision that depends on it.
- [ ] State the assumptions and guarantee boundary.

## 3. Preserve the chronology

- [ ] Record findings, fixes, observations, reviews, builds, deployments, and challenges in time order.
- [ ] Preserve the original event when a later event corrects or supersedes it.
- [ ] Attribute every event to its source or responsible party.

## 4. Classify the provenance

- [ ] Separate sourced public fact, executable observation, human attestation, agent inference, and policy judgment.
- [ ] Link every sourced edge to its canonical artifact.
- [ ] Mark inferred, missing, disputed, stale, and superseded edges explicitly.

## 5. Test the evidence chain

- [ ] Ask whether the observation discriminates the claimed change from its relevant alternative.
- [ ] Check that evidence concerns the exact revision and dependencies under review.
- [ ] Check whether reviewed source connects to the built and deployed artifact.
- [ ] Keep formal proof, testing, source reproduction, audit review, and deployment identity as separate evidence kinds.

## 6. Derive the current view

- [ ] Carry forward only claims still supported by non-superseded evidence.
- [ ] Show the latest supported conclusion, not merely the latest statement.
- [ ] Display its evidence boundary and unresolved gaps beside it.
- [ ] Never compress the result into an unqualified “safe” or “verified” badge.

## 7. Derive next actions

- [ ] Start from a visible missing or stale edge.
- [ ] Name the policy or decision that makes the edge necessary.
- [ ] Recommend the smallest observation, review, attestation, or identity link that could close it.
- [ ] Label the result as a recommendation rather than a fact or universal requirement.

## How Binder applies it

1. Append provenance events without rewriting history.
2. Join events through explicit relationships such as `affects`, `remediates`,
   `observed on`, `built from`, `deployed as`, `supersedes`, and `invalidates`.
3. Derive the Current view from supported, non-superseded relationships.
4. Display the full event trail in Chronology.
5. Compare unresolved relationships with the declared decision policy to derive
   Next actions.

Every recommendation should therefore be explainable as:

```text
visible gap + declared policy → proposed action
```

## License

This checklist is licensed under [CC BY-SA-NS](cc-by-sa-ns.md): Creative
Commons Attribution-ShareAlike 4.0 with an additional Network Services
condition requiring corresponding source for network-service derivatives.
