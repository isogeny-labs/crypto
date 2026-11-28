# Preregistration — encoding control for the subgroup factor base (2026-11-18)

Frozen before any arm data (only a k = 2 tower-H plumbing smoke run, discarded,
preceded this file). Everything here is `[pcc:unverified]` pilot work: no pcc
claim, no leaderboard or `benchmarks/` row, no `S`, no rho comparison.

## 1. Question and claims (classified per the `controlled-experiment` skill)

`research/rand_vs_struct_fb_20261118/` showed (m = 2): subgroup base written as
the Kummer chain has F4 solving degree `D` flat at 4–5; a random base written as
one polynomial `∏(x − f)` has `D = |F| + 2`. It could not separate structure
from encoding. Here every arm uses the **same generators** `u_j = x^{2^j}`, the
**same chain** `u_j² = u_{j+1}`, the **same `S₃`**, the same curve, the same
random targets; only the **closing relation** `u_{k−1}² = R_F(u)`,
`R_F = x^{2^k} mod ∏_{f∈F}(x − f)` in u-monomials (degree ≤ k), differs.

Competing claims (class: **comparative**, decided by this experiment):

- **C-enc** "any set with a log-variable low-degree (tower-shaped) encoding is as
  easy": tower-random `D` = tower-H `D` (± A/A spread, at most +1) at every k.
- **C-struct** "the subgroup's low `D` comes from algebraic structure beyond having
  such an encoding": tower-random `D` exceeds tower-H `D` and grows with k.
- An intermediate outcome (bounded but larger constant) is reported as such.

Exact-computation claims (no control needed, verified): each encoding's variety
is exactly `F` (all `2^k` points satisfy the block; F4 quotient dimension `2^k`;
univariate eliminant has `2^k` roots in `F_p`); a wrong closing relation
(`R_F + 1`) admits a different point set (negative control).

Out of scope: any index-calculus cost, security, m ≥ 3, cryptographic sizes.
§5a pricing band: not applicable (no cross-method ECDLP cost comparison).

## 2. Arms (m = 2, p = P1 = 2013265921, |F| = 2^k)

1. **tower-H** — `F = μ_{2^k}`, `R_F = 1` (positive control / instrument gate).
2. **tower-rand** — `F` = 2^k random curve abscissae, dense `R_F`; 3 draws (A/A).
3. **dose** — `H` with `r` random positions replaced by random abscissae,
   `r ∈ {1, 2, 4, …, 2^{k−1}}` (`r = 2^k` is arm 2); 2 draws per r for k ≤ 6, 1 above.
4. **coset** — `F = gH`, `g ∉ H` random, `R_F = g^{2^k}` constant; 3 draws.
5. **bits-H / bits-rand** — `b_j² = b_j`, `x = P_F(b)` multilinear interpolant
   (degree k; for H with the natural labelling `v ↦ ζ^v`); bits-rand 3 draws.
6. **randbase** — `∏(x − f)` single polynomial (ties back to the earlier table), 1 draw.

## 3. Instrument (same on every arm)

`examples/enc_control.rs` built by `harness/` (release; binary SHA-256 recorded
in `binary.sha256`). F4 = `src/cryptanalysis/f4_fp.rs` (grevlex, degree bound
`max(4k+8,16)`, `2^k+8` for randbase, 300 s budget per system). Per system:
`D = solving_degree_max`, degree reached, `field_ops` (counted `F_p`
multiplications), step degree profile, Hilbert first-fall diagnostic, timeout and
pairs-above-bound flags. HKY closure (`hky_fp_tower`, c_max 14) on random targets
where the closing relation is constant (tower-H, coset, and any dose/r=0) as a
`d_F` cross-check.

Targets per (arm, k, draw): 1 planted (two random members of `F` that are curve
abscissae, `x_R = x(P_1 ± P_2)`) + 2 random targets `x_R` (random abscissae,
**shared by all arms at a given k**, inconsistent w.h.p.).

**Primary metric:** `D` on the random (inconsistent) targets, i.e. the degree at
which F4 derives 1 (= `d_F`, verified = HKY last fall on tower/null earlier).
Secondary: `field_ops`, planted `D`.

Gate: tower-H must give `D` = 4,4,4,4,5 at k = 2..6 and equal HKY last fall;
every encoding must pass `verify` (exact variety + negative control detected),
else the run stops (the binary asserts).

## 4. Order, seeds, load

Master seed 20261118; every set, plant and target seed is SplitMix-derived from
(master, label, k, r, draw). Within each k the arm order is rotated by k
(interleaved, not blocked by arm). `RAYON_NUM_THREADS=2` (shared 4-vCPU box);
load average recorded before every system. `D` and `field_ops` are deterministic;
wall time is context only.

## 5. k range and stop rule

k = 2, 3, … 8. An arm that times out or hits the degree bound at k is not run at
larger k (recorded as censored). The ceiling is whatever k the 1–2 h local
budget reaches; reported as such.

## 6. Predictions (written before data)

- **tower-H:** `D` = 4,4,4,4,5 (k = 2..6), ≤ 6 to k = 8; HKY last fall = `D`.
- **coset:** identical to tower-H at every k (constant closing relation).
- **tower-rand:** `D` grows with k, above tower-H from k ≈ 4, but stays
  O(k): within [k+2, 2k+4], far below `|F|+2` for k ≥ 4. A/A spread ≤ 1.
- **dose:** sharp jump already at r = 1 to (about) the tower-rand level, then
  flat in r (r = 1 already makes `R_F` dense): all-or-nothing.
- **bits-H ≈ bits-rand:** both dense degree-k interpolants; both above tower-H
  and growing with k.
- **randbase:** `D = 2^k + 2`.

If tower-rand comes out = tower-H, C-enc is supported and the predictions above
are wrong; that is reported as is.
