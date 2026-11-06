# Protocol: decomposition cost on a conjugate-pair line against F_p (frozen before data)

2026-11-06. This follows
[`lift_dense_conjugate_pair_cost_20261106`](../lift_dense_conjugate_pair_cost_20261106/README.md),
whose derived cost model **assumed** that one decomposition trial costs the
same on a conjugate-pair line as on the standard line F_p. This measures that
assumption with the repository's own counted k = 4 solver. This file is
committed before any code or computation for this study.

## Question

Gaudry's k = 4 decomposition over K = F_{p⁴} works like this:

- substitute the factor-base line into the symmetrised summation polynomial
  S₅;
- Weil-restrict to 4 equations of degree 8 over F_p;
- solve with a Macaulay matrix at degree 29, the eigen step, and the
  quartic split.

On the same curve and the same targets, what does one such solve cost, in
counted F_p multiplications, when the factor base is a conjugate-pair line
x = x₀ + u·t (t ∈ F_p) rather than F_p?

**Algebra, fixed before data.** e_k(x₀ + u·t₁, …) is affine-linear in
e₁(t), …, e_k(t), with coefficients in K. So the substituted polynomial has
the same total degree (8) and the same monomial support (all 495 monomials
of degree ≤ 8), and its Macaulay matrix has the same rows and columns. Any
cost difference must come from rank, quotient dimension or pivot structure.

## Instrument

This is a new example, `examples/conjpair_decomp_cost.rs`, in the main
crate. It calls the public API of `src/cryptanalysis/gaudry_quartic.rs` and
does not change that module:

- **Field and curve.** `Fp4` (F_{p⁴} = F_p[t]/(t⁴ − c), with counted
  multiplications) and `Curve4::with_params`.
- **Full 2-torsion.** Roots r₁, r₂ ∈ K, with r₃ = −r₁ − r₂;
  a = r₁r₂ + r₁r₃ + r₂r₃ and b = −r₁r₂r₃. Curves with j ∈ F_{p²} are
  rejected.
- **Conjugate-pair lines.** Built with the F_p-linear system of the earlier
  pieces, re-implemented on `E4`. The first all-lift line is used.
- **H for a line.** Constructed by the same interpolation as
  `SymmetrisedS5::precompute`, with x = x₀ + u·r and e = elementary symmetric
  functions of r. It is checked against fresh resultant evaluations
  `Curve4::s5`. The standard arm is built by the same code with x₀ = 0, u = 1,
  and must equal `SymmetrisedS5::precompute`'s coefficients (a positive
  control on the construction).
- **Solve.** `SymmetrisedS5::weil_restrict` then `solve_system4_with`, with
  `PlainOptions { chains: true, sparsest: true }`, equation degree 8 and
  Macaulay degree 29. The cost is `QuarticSolve.total_muls` plus `weil_muls`.
  Its parts (echelon, normal forms, characteristic polynomial,
  eigenvectors) and the shape (rows, cols, pivots, dim) are recorded.
- **Oracle.** Solutions are e(t) ∈ F_p⁴; their quartic is split over F_p by
  brute force (not charged; the split is under 10⁻⁶ of C₄ in §11.17). The
  resulting x-multisets must equal those of an exact meet-in-the-middle
  decomposition of R over the line's factor base.
- **Host.** This container. Iso Lab EKS needs an AWS identity, and the
  container has none. The metric is a deterministic operation count, so
  wall time is recorded but not used.

## Cells

- p = 29, 2 curves.
- On each curve, 2 arms: `conj` (all-lift conjugate-pair line) and `std`
  (F_p).
- Each arm gets 3 targets:
  - 1 planted: the sum of 4 random points of that arm's factor base, which
    is decomposable by construction;
  - 2 random points of E(K), the same 2 for both arms.
- That makes 12 solves, run sequentially (about 3.4 GB each).

If a solve exceeds 40 minutes of wall time, the cell is reduced to 1 curve.
That is recorded as a deviation.

## Predictions (decision rules)

**K1 (shape).** On every target, rows, cols and dim agree between the arms.
dim = 4,096, the Bézout dimension.

**K2 (cost).** The per-target ratio C(conj)/C(std) of total_muls on the
**same** random target lies in [0.9, 1.1] on every common target. Planted
targets differ by arm and are compared through the median over all targets
of each arm: median ratio in [0.95, 1.05].

**K3 (correctness, exact).** On every solve, the solver's x-multisets equal
the oracle's. The planted target is found in its own arm.

**K4 (derived, reported).** The relation-phase cost ratio is
T(std)/T(conj) × C(std)/C(conj), using the earlier piece's measured trial
ratio (11.5 at q = 13, 17.2 at q = 101).

- If K2 holds, the earlier assumption holds at this scope, and the
  relation phase is 11–17× cheaper per solve.
- If K2 fails, the measured ratio replaces the assumption.

## Controls, A/A, analyst

- **Positive control (construction).** The std arm's H equals
  `SymmetrisedS5::precompute`.
- **Positive control (solver).** A planted decomposition on each arm.
- **A/A.** One solve is repeated on the same input. The operation count
  must be identical, since the counted arithmetic is deterministic.
- **Analyst.** The producer, using coded rules.

## Amendments

**A1 (2026-11-06, registered after the first main-run solves and before its
own data).**

- **What prompted it.** On curve 1 the conj arm's two decomposable targets,
  the planted one and `common0`, gave quotient dimension 4,090 instead of
  4,096 at Macaulay degree 29. The solver returned no solution there,
  although the oracle found 3 and 2 decompositions. The conj target with no
  decomposition (`common1`) gave dimension 4,096 and solved. The std arm gave
  4,096 and solved on all three targets.
- **Follow-up.** Re-solve every conj-arm target that the oracle says
  decomposes, on every curve of the main run, at Macaulay degrees 30, 31 and
  32. Use the same curves, H and targets (`--only-arm conj
  --only-decomposable --macdeg 30,31,32`). Record shape, dim, solved, match
  and total_muls.
- **Rule M1.** The lowest degree at which the solve succeeds and matches the
  oracle is the working degree on conjugate-pair lines. Its cost, against
  the std arm's degree-29 cost on the same curve, is the per-solve cost
  ratio that replaces K2's for decomposable targets.
- **Ceiling.** If no degree up to 32 works, conjugate-pair lines are
  reported as not solvable by this solver design. In that case the
  relation-phase cost advantage is not established.
- **Memory.** Degree 32 needs about 7.5 GB, so the follow-up runs only after
  the main run finishes.
