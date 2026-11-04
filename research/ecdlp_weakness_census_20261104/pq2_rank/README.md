# pq2_rank: exact degree lower bounds for the m = 2 descended S₃ system

Target: `HEUR-GB-PQ-ASSUMPTION-2` (Petit–Quisquater Assumption 2: the degree of
regularity, i.e. the maximal degree reached by the Gröbner solver, equals the first
fall degree up to o(1); Kousidis–Wiemers read it as `D_reg = m² − m + 1 + o(1)`, which
is 3 + o(1) at m = 2). At m = 2 the first fall degree of the Weil-descended third
summation polynomial system is 2 for every n (Kosters–Yeo Cor. 4.11), so the literal
reading predicts solver degree 2 and the Kousidis–Wiemers reading predicts 3 for all
large n. This census computes, exactly over F₂ and with no Gröbner basis computation,
a lower bound on both quantities for n from 8 to 56.

Command (from this directory): `cargo run --release -- 8,9,...,56 3 20 results/pq2_rank 44 16`
(the arguments are the list of n, instances per n, the largest n for one-shot degree 4,
the output prefix, the largest n for iterated degree 3, the largest n for iterated
degree 4). Rust only, no dependencies, single thread; seed `0x5eed20261104`.
Outputs: `results/pq2_rank.json`, `results/pq2_rank.csv`, `results/run.log`.

## Instance

* Field F_{2ⁿ} with an irreducible trinomial or pentanomial found by the Ben-Or test.
* Ordinary curve y² + xy = x³ + a x² + b with random a and random b ≠ 0.
* Factor-base subspace V ⊂ F_{2ⁿ}: random F₂-basis of dimension n' = ⌈n/2⌉.
* Target abscissa x_R of a random point; instances whose solution set is empty are
  skipped (then 1 lies in the ideal and no degree question arises).
* Descended system: with x₁ = Σ aᵢvᵢ, x₂ = Σ bⱼvⱼ, S₃(x₁, x₂, x_R) = (x₁x₂ + x₁x_R +
  x₂x_R)² + x₁x₂x_R + b is bilinear in the bits: n equations in N = 2n' variables, plus
  the N field equations aᵢ² + aᵢ, bⱼ² + bⱼ. The program checks bitwise, on 20 random
  assignments per instance, that the descended equations agree with the direct S₃
  evaluation, and that S₃ vanishes on the abscissae of a true sum of points.

## Quantities, all exact

* `Z`: every solution (a, b) ∈ F₂^N, enumerated through x₁ ∈ V (2^{n'} values) by solving
  the quadratic A X² + B X + C = 0 for x₂ and testing membership in V.
* `k = N − affdim(Z)`: the number of linear polynomials in the ideal with distinct
  leading variables (Boolean Nullstellensatz: the ideal with field equations is the
  vanishing ideal of Z; the |Z| = 1 case is kernel-checked in
  `BooleanNullstellensatz.linear_mem_of_unique_zero`).
* `rho`: rank of the bilinear parts of the n descended equations.
* `low_d` (one-shot): dim(rowspace(M_{≤d}) ∩ P_{≤1}) for the Macaulay matrix of the
  original generators at degree d = 2, 3 (and 4 for n ≤ 20).
* `iter_d` (iterated): the same dimension for the degree-d closure: starting from the
  generators, every multiple of the newly found polynomials that stays within degree d
  is added and the matrix re-reduced, until the rank stops growing. This is what a
  Gröbner run whose steps never exceed degree d can reach (F4 multiplies the new
  polynomials of each step again, so its output at max step degree D lies in the
  degree-D closure). Computed for d = 3 (n ≤ 44) and d = 4 (n ≤ 16).
* `d_lin`: least d with `low_d ≥ k`; `d_iter`: least d with `iter_d ≥ k`.

By `SolvingDegreeBound.no_gb_in_rows_of_low_lt` (Lean, kernel-checked), any Gröbner basis
for a degree-compatible order that lies in a row span whose degree-≤1 part has
dimension < k is impossible. Hence `d_lin ≤ sd` (Caminata–Gorla one-shot solving degree)
and `d_iter ≤ D_F4` (F4's maximal step degree), the latter under the containment
property of F4 stated above, which is not formalised.

## Results

RESULTS_PLACEHOLDER
