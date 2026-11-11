# Proposition A: algebraic content in Lean 4

Files: `TraceRelation.lean`, `check.log`. Date 2026-11-11.

## Toolchain
- `lean --version`: Lean (version 4.34.1, x86_64-unknown-linux-gnu, commit 5045d0056413266e57c625dcd7c365b10e377c52, Release)
- Mathlib rev d13f23b723b8a846827a245b89c10fc7d3f11612 (from `research/guarded_fall_degree_20261028/formal/lake-manifest.json`)
- Check: `cd research/guarded_fall_degree_20261028/formal && lake env lean <path>/TraceRelation.lean`; exit 0, output in `check.log`.
- `check.log`: axioms are only propext, Classical.choice, Quot.sound (traceRelation and map_smul2 use only propext, Quot.sound). No `sorry` in the file (grep count 0).

## Sources checked
- `research/notes/index-calculus/RESEARCH_FALL_DEGREE_BOUNDS.md` §0, §5 (Proposition A statement and proof sketch).
- `papers/ecdlp-index-calculus-treatise/sections/falldegree.tex` around cor:ffd2 (Corollary 'First fall degree 2'; proof parts (1)-(3), Kosters-Yeo Cor. 4.11).
- research-kb: server unavailable this session; no query made. Conductor: not installed; no task attached, no files outside `propA_lean/` edited.

## Setting (abstract hypotheses)
Commutative ring `R` with `CharP R 2`, additive `Tr : R →+ ZMod 2` with `hTr : ∀ a, Tr (a*a) = Tr a`. `S3 β a b c := (a*b+a*c+b*c)^2 + a*b*c + β`.

## PROVED (Lean)
- `traceRelation`: `x3 * w = 1 → Tr (S3 β X1 X2 x3 * w^2) = Tr X1 + Tr X2 + Tr (β * w^2)`.
- `coordinateCombination` (with helper `map_smul2`): for a finite `ZMod 2`-basis `bR` of `R` and any additive `μ : R →+ ZMod 2`, `μ r = ∑ i, μ (bR i) * bR.repr r i`.
- `traceFunctional_in_span`: `Tr (s * w^2) = ∑ i, Tr (bR i * w^2) * bR.repr s i`.
- `traceRelation_in_span`: `Tr X1 + Tr X2 + Tr (β*w^2) = ∑ i, Tr (bR i * w^2) * bR.repr (S3 β X1 X2 x3) i`, i.e. the functional is an F2-combination of the coordinate functionals of S3 (coordinates in the basis `bR`).
- `affineInBits`: `Tr (∑ j, y j • v j) = ∑ j, y j * Tr (v j)`; `L_affine`: `Tr(∑ y1 j • v j) + Tr(∑ y2 j • v j) + c = ∑ j, Tr (v j) * (y1 j + y2 j) + c`.
- `L_const_iff`: that expression is constant in the bits iff `Tr (v j) = 0` for all `j` (finite family `v`, `DecidableEq`).
- Sanity instance `R = ZMod 2`, `Tr = AddMonoidHom.id` (`hTr_zmod2`, `sanity_inCoordinates`, with basis `Module.Basis.singleton`): hypotheses are satisfiable.

## NOT done
- Non-trivial second instance (GF(4) with Tr a = a + a^2): NOT FORMALISED. `AdjoinRoot` is present in the Mathlib cache, but irreducibility/CharP/enumeration of elements and the trace as a `ZMod 2`-valued map were not attempted.

## NOT FORMALISED
- Polynomial rings over `F_2` in the bits, degree, first fall degree, last fall degree, Macaulay matrices.
- Boolean reduction modulo field equations; the actual Weil descent (descended equations are modelled only as coordinates of `S3` in `bR`).
- The hypothesis "every descended equation has degree exactly 2" of Corollary ffd2, and the conclusion d_ff = 2 (including the statement that L has degree 1 and lies in span of degree-2 rows).
- The concrete trace of GF(2^n) (axiomatised by additivity and `Tr (a*a) = Tr a`); the existence of a rational point with x-coordinate x3 and the choice of `a_2` (steps 1-3 of the prose proof); Kosters-Yeo Cor. 4.11 itself.
- PROVED (prose, cited): Prop. A as a whole and Corollary ffd2 in the note and treatise, relying on Kosters-Yeo; this Lean file re-derives only the algebraic identity directly (without the Kosters-Yeo detour through a point P), for any `x3` that is invertible.

## Claims
No security or cost claims. Any numerical statement in the cited note (e.g. 112/112 draws) is [pcc:unverified] and is not used here.
