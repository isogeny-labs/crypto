# Explicit twist group-map formalization — protocol frozen before execution

Starting revision: `3c51d745f9b885690162d03253db69a860062e11`.
Date: 2026-10-23. This is a proof iteration, not a speed experiment.

## Requirement and scope

Continue the user's search for ways to falsify a proposed quadratic-twist
index-calculus advantage, excluding invalid-curve attacks. The next missing
bridge is the addition law for the already constructed actual-point map
`psi(O)=O`, `psi(X,Y)=(X/d,Y/(d*u))`, with `u^2=d` in an extension.
Prove this bridge rather than assuming it, then transport source annihilators
and derive an odd-order rational-relation obstruction on source points.

Use a new Lean module importing the existing `TwistEmbedding.lean`; preserve
that source and every source-bound historical certificate byte for byte.
Use the existing pinned Lean/Mathlib toolchain and unmodified repository
proof gate. No new research axioms, sorry, native evaluation, assumed group
compatibility, assumed Frobenius conclusion, or skipped exceptional additions.

## Proof targets and acceptance

1. Check coordinate formulas for addition under scaling, including the
   secant, tangent, inverse-pair, identity and two-torsion cases.
2. Construct an actual `AddMonoidHom` whose underlying map is precisely the
   existing point map. Reuse the existing proved injection.
3. Deduce natural and integer scalar compatibility and source-annihilator
   transport. Under the existing finite odd-field/nonsquare hypotheses,
   replace the old image-annihilator hypothesis by `r • P = 0` on the source.
4. Derive the corresponding finite-sum obstruction without claiming each
   summand vanishes when only the aggregate vanishes.
5. Build all study modules, run the full scoped repository proof gate and
   inspect every new declaration's transitive axioms. Preserve failed source
   attempts as `.lean.txt` and their actual compiler logs. Record whether an
   independent `lean4checker --fresh` is available.

If feasible after these targets, characterize the image through its actual
Frobenius action; otherwise leave full eigenspace surjectivity explicit.
Any such theorem must state the extension's fixed-field hypotheses exactly.
No finite test or successful conditional theorem closes that larger gap.

## Controls, evidence and limits

Retain the existing native square-parameter and nonzero two-torsion controls.
The original 942-model study, 274 subgroup pairs, 74,693,760 mixed-word
candidates, independent native auditor and positive pairing/IC connection
remain frozen evidence. No new timing, solver advantage, universal hardness,
GHS or universal no-connection claim is intended.

New proof receipts, failed attempts, source digests and any native identity
certificates go under `validation/group-map-20261023/`; the source-linked
report, explanatory diagram and inspected PDF go under
`reports/group-map-20261023/`. Run the mandatory release library suite before
pushing. Keep the PR open for unfinished CI/review, formal and actual shared
Conductor gates. A real Conductor check was attempted and returned command
not found; no reservation or replacement coordinator is claimed.

Current PR maintenance also found a pending Copilot conflict request and
conflicts in `docs/browser/data.json`, `docs/curves/TRAITS.md`, and
`docs/ic-leaderboard.html` against current main. This proof iteration does
not edit those files or resolve claim/evidence conflicts by choosing a side.
Unrelated inherited/service CI failures stay separate from this proof.
