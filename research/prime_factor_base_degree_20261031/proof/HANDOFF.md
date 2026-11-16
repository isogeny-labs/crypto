# Formalization handoff, 2026-10-31

User goal: continue until a formal Lean proof of the degree reduction is
identified. The polynomial-ring/canonical-top degree theorem is now checked.

## What was done

Eleven Lean4.19.0 modules build with warnings treated as failures. Mathlib is
pinned to v4.19.0 commit c44e0c8ee63ca166450922a373c7409c5d26b00b, with all
transitive Git dependencies locked. Canonical top forms use each original
polynomial's actual computed total degree. Generic d≥2 comparison and the
exact F17 four-sign fixture are checked. The unchanged auditor accepts121
declarations including generated equations, with0 failures and0 native trust.

## Decisions and remaining scope

The headline theorem is `DegreeBridge.canonical_strict_reduction`; the
fixture is `CanonicalFixture.degree_five_to_three`. The invariant is the
first vanishing homogeneous image in the canonical top-ideal quotient.
Full elliptic-curve group addition/equivalence is still written, not Lean
formalized. No affine F4 degree or runtime conclusion is inferred. The earlier
experiment PR#3077 merged as ff7ad61cfed11908fb79de9dd485b696d0ada779; this is
a new formalization follow-up. Keep existing sealed data and reports intact.

## Errors retained

Mathlib's public binary-cache endpoint returned proxy CONNECT403. Source
compilation of the pinned library succeeded without a policy bypass. Local
cache storage initially pointed at a read-only home path; XDG_CACHE_HOME
provided writable workspace storage. Initial proof-development attempts had
ordinary elaboration/API errors; their logs and initial source snapshots
remain in the deterministic archive. Final published Lean source contains no
placeholders, custom axioms or native evaluation. `lean4checker` was absent;
no fresh-checker result is claimed. Conductor was absent and its documented
fail-open behavior applied. No new cryptographic experiment or CAS run occurred.

## Publication boundary

Publish only this proof directory and the parent README link from a fresh
remote main parent. The local feature branch retains older, unrelated shared
history and must not be pushed wholesale. Exclude broad session logs.
Enable auto-merge on the follow-up PR, then require current-head gate/size/
artifacts checks before merge. The existing proof gate discovers the new
Lake project automatically; neither gate nor trust policy is changed.
