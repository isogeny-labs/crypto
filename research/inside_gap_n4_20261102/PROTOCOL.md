# Is the inside fake-to-true gap bounded? An n = 4 search (2026-11-02)

Frozen before any counted computation. Target: `GAP-FAKE-GAP-INSIDE-BOUNDED` /
`RT-FAKE-GAP-INSIDE` in `docs/ecdlp-hardness-corpus`.

**Question.** With the field equations inside the generating set, is there a constant `c`
with `d_F ≤ d'_F + c` for every system over `F_2` (Nagao Definitions 5 and 6, equality
reading)? Known so far (`research/lean_attacks_20261102`):
- the `+1` form is refuted;
- the largest gap over all pairs of degree ≤ 2 in three variables is 2, with witness
  `{YZ + 1, XZ + X + 1}`;
- the family `W_k` stays at gap 1 inside.

**What a computation can and cannot show.**
- A system with gap `g` proves only that any bound is at least `g`.
- A finite search can never prove a bound.
- An unbounded gap needs an infinite family with a proof.
- So the aim is to see whether the maximum gap grows with the number of variables
  (2, 3, 4) and, if it does, to extract a pattern to prove.

## Instrument

`inside_gap_n4.rs`.
- **Core:** lines 1..384 are a byte-for-byte copy of
  `research/blindspots_20261018/instruments/blindspots_19_checks.rs` (sha256 `751277a8…`):
  `fake_first_fall` and `true_first_fall`, unchanged.
- **New driver:**
  - computes only `d'_F` (cap `n`, exhaustive in the multilinear quotient) and
    `d_F(inside)` (cap `dmax`);
  - 4 threads; seeded splitmix64 sampling;
  - keeps a gap histogram and the smallest witness per gap value;
  - counts systems whose inside degree exceeds the cap as undetermined, without guessing
    their gap.

**Control:** `{YZ + 1, XZ + X + 1}` embedded in 4 variables must give gap 2 (PASS before
freezing).

**Timing probes before freezing:** Q1 rows < 4, and 400-pair samples of Q2 and Q3. They
were used only to size the runs and are not counted.

## Populations (`dmax = 6`)

| id | population | size | kind |
|---|---|---:|---|
| S2 | all pairs, degree ≤ 2, n = 2 | 1,891 | exhaustive |
| S3 | all pairs, degree ≤ 2, n = 3 | 521,731 | exhaustive |
| Q1 | all pairs, multilinear degree ≤ 2, n = 4 | 2,092,035 | exhaustive |
| Q2 | pairs, all degree ≤ 2, n = 4 | 2,000,000 | seeded sample (seed tag 2) |
| Q3 | pairs, multilinear degree ≤ 3, n = 4 | 2,000,000 | seeded sample (seed tag 3) |

Undetermined systems (`d_F(inside) > 6`) are rerun individually at `dmax = 9`. If any is
still undetermined, its gap is reported as a lower bound only.

## Decision rule

- **Gap ≥ 3 in any n = 4 population.** The n = 3 maximum is not a universal bound.
  1. Record the smallest witness and confirm it with an independent brute force, or with
     a second exact method where brute force is infeasible.
  2. Compare the maxima for n = 2, 3, 4. If they grow, study the witnesses for a family
     whose gap grows with n, and state any family only with a proof (hand proof plus
     native check; Lean if it fits).
- **Maximum stays 2 at n = 4.** Record `d_F(inside) ≤ d'_F + 2` as the strengthened
  conjecture, with the evidence and its limits.
- **Control fails.** Stop.

No cost or speed claim. Exact computations on the session container (4 cores, shared, L0).

## Amendment 1 (after S2, S3, Q1, Q2, Q3 ran; before the steps below)

Q3 found gap 3 at n = 4: `{XYW + XY + W, XZW + XW + 1}` (`d_F(inside) = 4`, `d'_F = 1`),
confirmed by the independent implementation `independent_falls.py`. Q1 and Q2 (degree ≤ 2)
stay at 2. The witnesses share a shape, `F(A, B, v) = {A v + 1, B v + B + 1}` with
monomials `A, B` and a variable `v`:
- `(1 + v) f_1 + v f_2` reduces to `1`, so `d'_F = 1`;
- `v f_2 + B (v^2 + v) = v` and `(v + 1) f_1 + A (v^2 + v) = v + 1` are true falls (inside)
  at `deg B + 2` and `deg A + 2`.

Conjecture to test: `d_F(inside) = min(deg A, deg B) + 2`, so the gap
`min(deg A, deg B) + 1` grows without bound.

Added steps:
1. Evaluate `F(A_k, B_k, v)` for `k = 1..4` with `A_k = X_1 ... X_k`,
   `B_k = X_1 ... X_{k-1} X_{k+1}` (shared shape, `n = k + 2` variables) and with disjoint
   `A_k, B_k` where n allows. Use `independent_falls.py` (alphabet widened to 8 variables)
   and a second Rust implementation: the verbatim core with only `NV_MAX = 8` and
   `Mono = u32` changed, every change listed in its header.
2. Attempt a proof of the lower bound `d_F(inside) ≥ min(deg A, deg B) + 2` for the family.
   If no proof is found, record the family as a conjecture with its exact evaluations.

## Amendment 2 (after the family evaluation; before the steps below)

The family saturates at `d_F(inside) = 4` for `k >= 3` (both implementations). The
independent implementation's degree-4 fall uses only the field equations:
`a^2 (v^2 + v) + v^2 (a^2 + a) = a^2 v + a v^2`. Hence, for `n >= 2` and every `F`,
`d_F(F ∪ S_fe) <= 4`, and since `d'_F >= 1`, the inside gap is at most 3, attained at n = 4.
For `n = 1` there is no second variable; candidate unbounded family `{X^k, X^k + 1}`
(`d_F(inside) = k`, `d'_F = 1`).

Added steps:
1. Check `d_F(S_fe) = 4` for n = 2..8 (field equations alone) and the n = 1 family
   `k = 2..8` with both implementations.
2. Kernel-check the field-equation fall at degree 4 in two variables (Lean mask encoding).
3. Record the answer: bounded by 3 for n >= 2 (tight), unbounded for n = 1.
