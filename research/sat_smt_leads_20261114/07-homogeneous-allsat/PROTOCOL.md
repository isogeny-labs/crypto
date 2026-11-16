# PROTOCOL (written before any measured run) - idea 7: homogeneous AllSAT

Date 2026-11-14. Pilot, toy sizes, one core per run, not claim data.

## Claim under test
Enumerating ALL factor-base-only relations P1+...+P_{m+1} = O from one
incremental SAT instance (S_{m+1}(x1..x_{m+1}) = 0, all xi in V, lex symmetry
breaking x1<=x2<=..., blocking clause per relation projected on the x bits)
costs fewer conflicts per verified curve relation than the per-target pipeline
on the same base (fresh solver per random target R, S_{m+1}(x1..xm, x(R)) = 0,
same encoding, same lex symmetry breaking, stop at first model), and the ratio
grows with l.

## Prediction (idea) vs null
- Idea: ratio Q(l) = (per-target conflicts per relation) / (homogeneous
  conflicts per curve relation) is > 1 and log2 Q grows with l.
- Null (tuple counting): both arms cost about one conflict per tuple of the
  free variables and both have about 2^n tuples per relation, so Q is a
  constant. The homogeneous instance also admits quadratic-twist solutions
  (all xi off the curve), which predicts Q about 0.5 for arm H0.

## Kill criterion
Killed if fitted slope of log2 Q versus l is <= 0.15 bits per l (flat within
pilot noise) in both families, or Q < 1 at the largest two l. Alive only if
slope > 0.15 bits/l with Q > 1 at the largest l in at least one family and not
contradicted by the other. Otherwise inconclusive.

## Arms (same base V, same encoding generator, same solver, per base in this
order: H0, H1, PT)
- PT (baseline SAT arm, untreated): per-target, fresh Glucose4 per target,
  16 random curve targets per base with x(R) not in V. Cost per relation =
  sum of conflicts over all targets / number of SAT targets.
- H0 (treatment): homogeneous AllSAT, no curve-membership constraint; models
  that do not lift to curve points (twist relations) are counted as waste.
  Cost per relation = total conflicts incl. final UNSAT / verified curve relations.
- H1 (treatment variant): H0 plus explicit y bits and the curve equation for
  each xi (removes twist solutions); blocking still projected on x bits.
- Counting baselines (analytic + exact from ground truth): plain enumeration
  tuples per relation for both pipelines with symmetry factors; MITM / linear
  2+1 leaf count per relation (2^{(m-2)l} leaf solves per target times
  targets per relation).

## Sizes
- Family A (m+1 = 3, per-target m = 2): n = 2l-1, l = 4..10. S3 direct.
- Family B (m+1 = 4, per-target m = 3): n = 3l-2, l = 3..6 (l = 7 if budget
  allows, fewer bases). S4 as chain S3(x1,x2,t) & S3(x3,x4,t), t in F_{2^n}.
- Curve y^2+xy = x^3+1 over F_{2^n} (polynomial basis, first irreducible
  found), whole group E(F_{2^n}). V = span of l random independent elements.
- 8 bases per cell (seeds 1..8; base seed = 1000*l + seed), 16 targets per base.

## Metric
Primary: Glucose4 conflicts (also decisions, propagations) per verified curve
relation. Secondary: wall time. Conflict budget 5e6 per solve call sequence;
a budget hit is reported and the cell marked incomplete.

## Validation before measurement
- Planted targets (sum of base points) must be SAT and the model must lift to
  curve points with a sign choice summing to +-R.
- Homogeneous enumeration must equal the brute-force/MITM set of curve
  relations on every cell (exact set equality of sorted x-multisets), and on
  the smallest cells the full model set (incl. twist) must equal brute-force
  evaluation of the summation polynomial over all sorted tuples.
- Per-target SAT/UNSAT must equal ground-truth decomposability for every target.

## Not varied
Solver (Glucose4 via pysat, default options), XOR-to-CNF cutting (length 4),
lex ordering encoding, target sampling. Decomposable / non-decomposable
targets are reported separately.
