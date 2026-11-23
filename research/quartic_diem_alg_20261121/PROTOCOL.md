# Plane-quartic Diem: algebraic decomposition and SGE (lead C3, frozen protocol, 2026-11-21)

Lead C3 of `research/hecdlp_campaign_20261121/CAMPAIGN.md`. Every figure `[pcc:unverified]`.
Synthetic random smooth plane quartics over `F_q`; no deployed-curve statement. Measurement on the
shared container (not an isolated host), so no row reaches the leaderboard (pre-gate).

## 1. Question

The decomposition rows of the double-large-prime Diem arm find the degree-3 divisor `D` of a class
`x = [a]G + [b]T` (given as the reduced-echelon net of three conics through `D`) by evaluating the
conics at every rational point of `C`: `O(q)` multiplications per trial, 70-80% of the arm's primary
cost at `q = 1031..8209` (`research/genus_ladder_bound_r_20261114/RESULTS.md`). Two additive options:

1. `Decomp::Algebraic`: intersect two conics of the net (resultant in one variable, degree <= 4,
   `proots`, back-substitution), keep the points on the third conic and on `C`, add the at most four
   rational points at `x = 0` by direct test; a degenerate pair is replaced by another (three pairs,
   two elimination orders); if every choice is degenerate the trial falls back to the scan (counted
   and reported). Every `F_q` operation is counted into `rel` (and its decomposition share) as the
   scan is; `proots` is the counted routine of the file, with a private RNG derived from the class.
2. `sge: Option<SgeParams>`: `sge_reduce` between the peel and Wiedemann, bookkeeping as in
   `genus_ladder::run_ic` (`la_sge` into `la`; the main stream advanced as the unmerged solve would).

Defaults (`Scan`, no SGE) leave every count byte-identical.

## 2. Instrument and grid

Binary `genus_ladder` built from this branch with the two options and the flag `--quartic-alg`
(default off: adds arms `Q_diem_2lp+alg`, `Q_diem_2lp+alg+sge(8,3)`), SHA-256 in `binary.sha256`.

```sh
genus_ladder run --primes 2053,4099,8209 --genera 2 --curves 3 --seeds 2 --arms R \
    --quartic --quartic-alg --max-h 2 --qc 3.0 --threads 4 --out quartic_alg.jsonl
```

(The genus-2 arm is filler; the quartic block is the experiment.) The quartic instances and the
`Q_diem_full`, `Q_diem_2lp`, `Q_rho_neg` rows of the bound-r study
(`/tmp/.../scratchpad/boundr/bound_r.jsonl`) are reused as the cross-check: instance lines (curve seed,
`f`, order) and the counted fields of the default arms must match byte for byte. 3 curves x 2 seeds per q.

## 3. Predictions (frozen before data)

FPM_mid = `pic_mul + 10 pic_inv + rel_mul + 9 la_mul` (`la_mul` includes `la_sge`).

| id | prediction | falsified if |
|---|---|---|
| Q1 | Scan and Algebraic give identical relations, relation rows, `decomp_trials`, core shape and recovered `t` on every (instance, seed); `rel_enum` equal; only `rel_decomp` differs | any difference outside `rel_decomp`/`rel` |
| Q2 | `rel_mul` ratio Algebraic/Scan <= 0.4 at q = 8209, falling with q | ratio > 0.4 at 8209, or not falling over 2053, 4099, 8209 |
| Q3 | FPM_mid ratio `2lp+alg+sge(8,3)` / `2lp` <= 0.5 at 8209 | ratio > 0.5 |
| Q4 | exponent of `2lp+alg+sge` FPM_mid over 2053..8209 <= 1.2 (old 2LP arm: 1.56 [0.97, 1.96]) | fitted exponent > 1.2 |
| Q5 | `Q_diem_2lp+alg+sge` / `Q_rho_neg` (FPM_mid) <= 0.005 at 8209 (was 0.013) | ratio > 0.005 |

Unit tests (`cargo test --release --lib quartic_ladder`): Scan and Algebraic give the same point sets
on >= 200 random classes at q in {101, 251, 1031}; identical relation rows and recovered `t` in a
full run with the same seed; the old 5 tests keep passing.

## 4. Analysis (fixed before data)

Ratios are ratios of per-q means over the 6 (instance, seed) runs, with a percentile bootstrap
(2000 resamples over runs, seed 20261121); exponents are least-squares slopes of `ln FPM_mid` on
`ln q` over the three primes with the bootstrap interval. Failures are reported as failures. No
GAE-band reading is made here (single-method component study); the rho reference is `Q_rho_neg`
unchanged, so the matched-rho band of AGENTS.md section 5a is not re-derived and no ranking claim
beyond FPM_mid is made.

## 5. Limits

Three primes, six runs per cell: wide exponent intervals. One host, shared. Single quartic family
(random smooth, `h <= 2`). The algebraic decomposition's node lookups `idx[pt]` (hash membership of a
found point among the rational points of `C`) are uncounted, as the line relations' residual-point
lookups are.
