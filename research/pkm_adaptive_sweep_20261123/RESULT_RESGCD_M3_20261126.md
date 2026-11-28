# m = 3 resultant–GCD against the grid and F4 — result

Companion to `RESULT_F4_DEGREE32_16BIT_SLOPE_RESGCD_20261125.md` (m = 2).
Every claim is [pcc:unverified]. Claude Code, local 4-vCPU container,
`RAYON_NUM_THREADS=1`, binary `3cfeb4b7…` (merged tree with the restored
engines, isogeny-labs/crypto#5207). Driver `sweep_t_engines_m3.sh`; data
`t_scaling_engines_m3_16bit_20261400.jsonl`.

## Engine

For each w ∈ V (the largest-index summand), S₄(Y, X, w, x_R) =
Res_U(S₃(Y, X, U), S₃(w, x_R, U)) has degree ≤ 4 in X. So
R_w(X) = ∏_{v∈V} S₄(v, X, w, x_R) (degree ≤ 4|V|) is built by the Karatsuba
product tree, then gcd(R_w, F_V); roots are read off V and pairs tested. That
is |V| univariate eliminations per target, each O(|V|²) with classical Euclid,
so O(|V|³) in total.

Correctness: `agrees_with_grid_m3` (isogeny-tower and bit-interval m = 3
forms) crosschecks resgcd against the grid with zero solution-set
disagreements; it passed locally (CI settings) and on CI (`library`,
isogeny-labs/crypto#5207). In this sweep resgcd and the grid find identical
relation counts at every t.

## Per-call cost against t (16-bit curve `icv1-fp16-t351-d433f24e`, CLI seed 20261400)

F_p operations per solver call (F4: multiply-adds; grid: its native
multiplication counter per trial).

| t | \|V\| | F4, chain | resgcd | grid | resgcd / grid | relations (resgcd = grid) |
|--:|--:|--:|--:|--:|--:|--:|
| 3 | 8 | 267,148,734 | 11,175 | 1,224 | 9.1 | 0 / 50 |
| 4 | 16 | 17,464,131,718 | 66,976 | 7,440 | 9.0 | 0 / 50 |
| 5 | 32 | censored: < 5 calls in 900 s | 418,997 | 51,232 | 8.2 | 4 / 50 |
| 6 | 64 | — | 2,723,936 | 378,944 | 7.2 | 13 / 50 |
| 7 | 128 | — | 18,334,518 | 2,912,384 | 6.3 | 45 / 45 |
| 8 | 256 | — | 127,299,689 | 22,831,360 | 5.6 | 50 / 50 |

Growth per unit t: F4 ×65 (t 3→4); grid ×6.1 → ×7.8 (towards |V|³ = ×8);
resgcd ×6.0 → ×6.9. resgcd stays above the grid over the whole range, with
the gap closing slowly (9.1× → 5.6×). Both are Θ(|V|³) with classical Euclid;
a half-GCD would bring resgcd to about |V|·M(|V|) log |V|, i.e. |V|^{2+o(1)}.

## Reading

- At m = 3 the F4 tower formulation is worse still: 267M multiply-adds per
  call at |V| = 8, against 1,224 for the grid (2.2·10⁵×), and growing ×65 per
  unit t.
- The univariate route does not beat exhaustive search at these sizes, and
  even a fast-GCD version only reaches |V|^{2+o(1)} per target. With
  |V| ≈ p^{1/3} that is p^{2/3} per target, already above rho's p^{1/2} for
  the whole DLP before relation collection is counted.
- Consistent with the m = 2 verdict: per-system elimination of the PKM chain
  system is exponential in t whatever the solver, and no tested solver moves
  the exponent.

## Boundary

One curve, one target seed, 50 trials per point (45 at t = 7, where the
pipeline solved early). Inversions are charged 32 multiplications.
Operation counts are deterministic; wall time is host-dependent. The F4 t = 5
point is censored, not measured. The first attempt was killed by a container
restart; the driver resumes by skipping recorded (label, t) rows, and the
rerun repeated only the F4 t = 5 call onward.
