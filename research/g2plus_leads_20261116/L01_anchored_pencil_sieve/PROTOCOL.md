# L01 — Anchored pencil sieve for hyperelliptic genus 2 and 3 (frozen protocol, 2026-11-16)

Status of everything this lead produces: `[pcc:unverified]` (one producer, shared
4-core host, no independent replay). This is not a benchmark: no leaderboard row,
no `benchmarks/` row, no speed claim, no wall-time metric. Synthetic random curves
only; nothing here is a statement about a deployed curve. Lab numbers quoted below
are `[pcc:unverified]` assertions copied from the files named.

Written-order note: this file and `SPEC.md` were written and hashed
(`frozen.sha256`) before any code of this lead existed and before any datum. The
hand values in `SPEC.md` section 11 were computed by the coordinator with a
throw-away script (brute-force group enumeration and naive pencil scan at
p <= 101) to fix unit-test expectations; they are test fixtures, not data.

## 0. Prior art (read before writing; research-kb is not installed here, grep only)

| path | what it shows | consequence for L01 |
|---|---|---|
| `crypto/research/more_leads_20261019/README.md` (section G1), `genus2_pencil_gate.py`, `native/target_labelled_pencil.rs` | The point-driven pencil sieve `lambda = (y - v(x)) / u(x)` was built and executed at genus 2 (Vitse–Wallet sieve) with the pencil anchored on the two points of a **Cantor-walk divisor** `D = (u, v)` (target-labelled). F_739 fixture: 12,233 complete relations of 73,900 members (0.1655 vs 1/6); batched inversions; 20/20 targets solved. Its priced prototype **lost** to the two-point collector and to source rho, but it charged a Cantor walk and transfer per pencil and full Jacobian verification. | The sieve mechanism is executed prior art at g = 2 and is not claimed as new. L01 differs in three tested respects: anchors are **fixed factor-base points** (no Cantor arithmetic at all in the relation phase; relations are homogeneous and need a separate descent for G and T), genus 3 is included, and the comparison is against the lab's native Cantor-walk IC instrument (`genus_ladder`) in its FPM unit at matched p and base. Not a duplicate. |
| `crypto/research/pencil_markowitz_20261022/PROTOCOL.md`, `pencil_sparse_20261022/`, `pencil_large_prime_20261029/`, `pencil_early_rejection_20261030/`, `pencil_progress_20261030/` | Follow-ups of the same walk-anchored genus-2 pencil on Scholten transfers: sparse pivoting, signed large-prime graph, early rejection, diagnostics. | Same mechanism, transfer setting, target-anchored. No fixed-anchor or genus-3 measurement. |
| `crypto/research/genus_ladder_20261114/PROTOCOL.md`, `genus_ladder_scale_20261115/RESULTS.md`, `genus_ladder_harvest_20261115/RESULTS.md`, `genus_ladder_sge_20261115/` | Lab baseline: Gaudry r-adding Cantor walk, one Cantor addition per trial, parity prefilter, `x^p mod u` split test, Cantor–Zassenhaus roots; FPM = cantor_mul + 10 cantor_inv + rel_mul + kappa la_mul. Scale study: at g = 2 the 2LP arm is 3–5x above rho and "the cost is the relation walk (one Cantor step per trial)"; at g = 3 the LA is three quarters of the 2LP cost `[pcc:unverified]`. Harvest study: a group-op saving that raised native work was not an improvement. | Defines the baseline arm B_lab (section 3) and the unit. The harvest result is why L01 charges every native multiplication and reports LA weight. |
| `crypto/src/cryptanalysis/genus_ladder.rs` (Cantor `Fk` tallies, `run_ic`, `disc_character`, `splits_completely`, `roots_of`, `key_of`, `random_f`) | Exact counting conventions. | Transcribed in `SPEC.md` section 5. |
| `crypto/research/leads_20261111/EXISTING_LEADS.txt` | No fixed-anchor pencil lead; only `LEAD-TRF-jv-sieve-fp6-weak-class` (unrelated JV sieve). | — |
| `cryptanalysis/research/handbook-ff-ic-ideas-20261103/IDEAS.md` | "pencil" hits are Hankel/MinRank pencils, unrelated. | — |
| `crypto/research/notes/index-calculus/` (listing), `crypto/research/notes/ecc2k130/RESEARCH_ECC2K130_IC_LITERATURE.md` | Vitse–Wallet cited as literature for binary/E(F_{q^n}) settings only. | — |

## 1. Mathematics (checked; corrections to the brief)

Curve `C: y^2 = f(x)`, `f` monic squarefree of degree `2g + 1` over `F_p`, one point
at infinity `inf`. For `a in F_p[x]` with `deg a <= g`, `deg a^2 <= 2g < 2g + 1`, so
`N_a = f - a^2` is monic of degree exactly `2g + 1` and
`div(y - a(x)) = sum over roots x_k of N_a of (x_k, a(x_k)) - (2g+1) inf`
(with multiplicity). Hence `sum_k ([P_k] - [inf]) = 0` in `J(F_p)`: a homogeneous
relation of weight `2g + 1`. **Correct as stated.**

Anchors `A_i = (x_i, y_i)`, `i = 1..g`, distinct abscissae. `a0` = Lagrange
interpolant (`deg a0 <= g - 1`), `Pi(x) = prod (x - x_i)`. The affine space of
`a` with `deg a <= g` through all anchors is exactly `{a0 + lambda Pi}`
(dimension `g + 1 - g = 1`): a pencil of `p` members. `N_a / Pi` is monic of
degree `g + 1` (the residual `R_lambda`). **Correct.**

Point-to-lambda map: for an affine point `(x0, y0)` with `x0` not an anchor
abscissa, `(x0, y0)` lies on `y = a_lambda(x)` iff
`lambda = (y0 - a0(x0)) / Pi(x0)`, exactly one lambda. **Correct.** Additions to
the brief: (i) points over an anchor abscissa are never in a residual (the one
with `y0 = y_i` is the anchor, the other is never on the pencil since
`y_i != 0`), so they are skipped; (ii) `lambda` is the value at the point of the
rational function `phi = (y - a0(x)) / Pi(x)` of degree `g + 1` on `C`, so the
residual divisors are the fibres of `phi`: **fibres are disjoint**, so a bucket
never holds more than `g + 1` points (a hard correctness gate), and every point
appears in exactly one fibre per anchor tuple; (iii) a fibre with a repeated
root or a root at an anchor abscissa has fewer than `g + 1` bucket hits and is
not collected (rare, `O(1)` per tuple).

Split probability: a fibre is fully split with distinct roots for about
`p / (g+1)!` of the `p` values of lambda (Chebotarev for a generic degree-(g+1)
cover; F_739 prior: 0.1655 vs 1/6). **1/6 at g = 2 and 1/24 at g = 3: correct.**

Relation usefulness (not in the brief, controls below):
- Rows are homogeneous (`sum eps_j L_j = 0`, `L_j = log_G(h P_j)`). The DLP needs
  in addition one descent of `G` and of `T` into the base (`[a]G + [b]T` split
  over the base; expected `g!` Cantor trials each up to the base-coverage factor).
  This is `O(g!)` group operations per target, negligible against `~p/2`
  relations, but it is required: the tiny-curve DLP gate (G3) does it end to end.
- Row weight is `2g + 1` (minus 2-torsion points) against `g` base entries plus a
  target coefficient for the walk relation; anchors are dense columns (each anchor
  is in every row of its tuple, about `p/(g+1)!` rows). The linear-algebra effect
  is measured (secondary, section 6) and not hidden.
- `-a` gives the negated relation; duplicates are removed up to sign in both arms.

Cost prediction, corrected. The brief predicted about 25–30 FPM per relation at
g = 2 and about 100 at g = 3. With values of `a0` and `Pi` at consecutive `x` by
forward differences (additions only), one Montgomery batch inversion per tuple
(`3(n-1)` multiplications, `n ~ p/2` abscissae carrying points) and one
multiplication per point (`~p` points), a tuple costs about `2.5p` multiplications
and yields about `p/(g+1)!` relations: **about `2.5 (g+1)!` FPM per relation,
i.e. ~15 at g = 2 and ~60 at g = 3**, plus `O(g^2)` per tuple. Horner evaluation
instead of forward differences would give ~21 and ~108 (close to the brief). The
additions this moves out of FPM are charged in a sensitivity reading (R3 below).

Verdict of the check: the lead is mathematically sound and not void; it is not a
duplicate of executed work (section 0). `proceed = true`.

## 2. Hypothesis and exact claim

H1 (primary, g = 2). On random genus-2 curves `y^2 = f(x)` over `F_p`,
`p in {1031, 4099, 16411, 65537}`, with the full base (all abscissae `x` with
`f(x)` a non-zero square), the relation phase of the anchored pencil sieve
(arm P_full) costs at least 3x fewer FPM per distinct full relation than the
lab's Cantor-walk relation phase (arm B_lab), at matched p, matched curve and
matched base.

H2 (secondary, g = 3): the same at g = 3 (threshold 3x, no kill weight).

H3 (secondary): against the explicit-formula floor B_strong (g = 2: Lange affine
addition priced 1I + 22M + 3S, doubling 1I + 22M + 5S, table-based quadratic
split and roots) the ratio is still at least 3x.

H4 (secondary, reported, no threshold): the counted sparse elimination over the
same number of rows costs more for P_full rows than for B_lab rows; the net
(relation-phase FPM saved minus LA FPM added at `kappa = g^2`) is reported at
p = 1031 and 4099.

H5 (secondary): the 2-large-prime variant P_2lp against B_2lp, FPM per graph
cycle (useful relation), ratio reported, threshold 3x, no kill weight.

## 3. Arms

All arms are Python 3 standard library, one thread, one counted field (`SPEC.md`
section 4). FPM = multiplications (squarings included) + 10 x inversions. Every
inversion anywhere is one counted inversion (B_lab's Fermat inversions in the
relation phase are counted as inversions, not as their multiplications: this
makes the baseline cheaper, conservative against the pencil).

- **P_full** (candidate). Disjoint anchor tuples of `g` base points from a seeded
  permutation of the base; per tuple: interpolation, forward-difference values of
  `a0` and `Pi` over `x = 0..p-1`, one batch inversion, one multiplication per
  point, buckets indexed by lambda; every lambda with exactly `g + 1` hits is a
  relation. Tuples are processed in order until `N_target` distinct relations
  exist; the tuple in progress is finished, and every distinct relation of the
  processed tuples counts in `FPM_per_rel` (rows for LA are the first
  `N_target`). If the permutation runs out of disjoint tuples, a fresh
  permutation is drawn from the same stream (only happens at tiny p, G3).
- **P_naive1** (positive control). For the first tuple only, for every lambda in
  `F_p`: build `R_lambda` explicitly, parity prefilter, `x^p mod R` split test,
  Cantor–Zassenhaus roots (all counted). Must produce exactly the relation set of
  P_full's first tuple (gate G4) at a predictably higher cost (sensitivity check).
- **B_lab** (baseline, the lab instrument): transcription of `genus_ladder::run_ic`
  arm A_full with the parity prefilter on: r-adding walk with 16 jumps, one
  generic counted Cantor addition per trial, restart every 4096 steps by two
  left-to-right scalar multiplications with `g * bitlen(p)`-bit scalars,
  `disc_character` prefilter (decides at g = 2), `splits_completely` at g = 3,
  `roots_of` (Cantor–Zassenhaus), `v` evaluated at the roots. Stops at `N_target`
  distinct relations. Labels are not tracked in counting mode (they are tracked
  in the gate, G3); the lab's 2 label multiplications per relation are omitted.
- **B_strong** (g = 2 only, same walk as B_lab, re-priced in the same run):
  each walk addition 25M + 1I, each restart doubling 27M + 1I and addition
  25M + 1I (Lange 2005 affine counts, taken from literature, `[pcc:unverified]`);
  1M per candidate for the discriminant via the character table; per split 2M for
  the roots via a square-root table and 2M for `v` at the roots.
- **P_2lp / B_2lp** (secondary, H5). Base `S` = first `nb = ceil(2 p^{1-1/g})`
  base abscissae in increasing x (the lab's `fb_size_2lp` with c = 2.0). P_2lp
  sieves only the points over `S` (anchors from `S`); a lambda with at least
  `g - 1` hits gets its residual built and divided by the known roots, and the
  quotient of degree at most 2 is solved by the character and square-root
  tables; at most 2 large primes. B_2lp is B_lab with base `S` and at most 2
  large primes. Both stop when the large-prime graph has
  `nb + 8 + floor(nb/100)` cycles (or at 2e9 FPM: `exhausted`).

## 4. Instances, seeds, variables held fixed

- Relation-phase primes: g = 2 and g = 3 at p in {1031, 4099, 16411, 65537}.
- Gate primes (tiny DLP, G3): g = 2 at p in {131, 257}; g = 3 at p in {67, 97}
  (both `1 mod 3`, needed for the `F_{p^3}` model). Kept tiny so that the dense
  solve mod r and the BSGS logs of every base point stay within budget.
- Curves: `f = random_f(p, g, s)` (the lab's SplitMix64 rule, `SPEC.md` 3.2).
  Relation phase: the first 3 seeds `s = 1, 2, ...` with `f` squarefree
  ("curve index" 1, 2, 3). Gate: the first seed with `f` squarefree and
  `#J = h r`, `r` prime, `h <= 64`, `r > h` (at most 400 seeds tried).
- Solver seeds 1 and 2 per curve (relation phase); seed 1 (2LP, LA, gate).
  All randomness is derived SplitMix64 streams (`SPEC.md` 3.3): one stream per
  (arm family, g, p, curve seed, solver seed). P_full and B_lab use the same
  curve and base and different streams; pairing is by (curve, solver seed).
- Held fixed across arms: field, curve, base columns, `N_target = n + 8 +
  floor(n/100)` (n = base columns, the lab margin), counting rules, dedup rule
  (canonical up to negation), price `I = 10` for the decision.
- Not part of any arm's primary count (identical for both, reported as `setup_*`):
  character/square-root table (`(p-1)/2` squarings), base enumeration (`2g+1`
  Horner multiplications per x), B_lab's 16 jumps and two restart bases.
- Verification work (V1, V2, G3) runs on an uncounted path and is never in FPM.

## 5. Metrics

Per run (arm, g, p, curve, seed): `mul`, `inv`, `add`, `FPM = mul + 10 inv`,
distinct relations, duplicates, `FPM_per_rel`; P_full also bucket increments,
forward-difference additions, tuples, relations per tuple; B also trials,
candidates, prefilter rejects, splits, walk additions, doublings, restarts,
`cantor_mul/inv`, `rel_mul/inv`, `strong_fpm`; rows: weight histogram, mean row
weight, max column weight, columns never hit, a SHA-256 of the canonical rows.

Readings (pair = same g, p, curve, solver seed):
- R1 (primary): `rho_lab = FPM_per_rel(B_lab) / FPM_per_rel(P_full)` at I = 10.
- R2: `rho_strong = strong_fpm_per_rel(B_strong) / FPM_per_rel(P_full)` (g = 2).
- R3 (sensitivity, conservative): P_full charged `+1` FPM per addition, per
  forward-difference addition and per bucket increment/scan, B_lab not:
  `rho_lab_adds`.
- R4: R1 at I = 1 and I = 30.
- Cell statistic: geometric mean over the 6 pairs of the cell, with min and max.

## 6. Correctness gates (all must pass before any metric is read)

- **G1** unit tests (`SPEC.md` section 11) pass, including hand values: mix64,
  random_f, `#J` by the L-polynomial vs brute-force enumeration, the p = 13 pencil
  example, exact operation counts of the batch inversion, interpolation,
  `disc_character` and `splits_completely`.
- **G2** every P_full / P_2lp relation passes V1 (polynomial identity
  `f - a_lambda^2 = prod (x - x_k)` over all `2g + 1` points, exact, uncounted);
  every B relation and a sample of P relations (all at p <= 4099, 200 seeded per
  run above) pass V2 (independent Cantor sum of the point divisors equals the
  identity for P, equals the walk divisor for B). Any failure voids the run.
- **G3** tiny DLP gate at (g, p) in {(2,131), (2,257), (3,67), (3,97)}: `#J` from
  `N_1..N_g` with `[#J] D = 0` on 16 random divisors; every P_full row and every
  labelled B_lab row satisfies its log identity against BSGS logs of all base
  points; `3n` pencil rows plus 3 descents of G and 3 of T solved mod r recover the
  planted t, and every determined `L_j` equals its BSGS log.
- **G4** P_naive1 relation set (lambda and points) equals P_full's first-tuple
  set exactly in every relation-phase run.
- **G5** no bucket ever exceeds `g + 1` hits; no 2LP residual root lies in `S`;
  every synthetic-division remainder is zero.
- **G6** A/A determinism: the rerun of (g = 2, p = 4099, curve 1, seeds 1 and 2)
  reproduces every integer field and every rows digest exactly.
- **G7** sanity of the split model: P_full relations per tuple, pooled per cell,
  divided by `p/(g+1)!` lies in [0.85, 1.15] in every cell.

A failed gate voids the affected runs; the verdict is then "inconclusive (void)"
for any cell it touches, and the failure is reported verbatim. No threshold moves.

## 7. Controls

- Positive control (instrument sees an effect of known sign): P_naive1 vs P_full
  on the same tuple, same relation set (G4). Predicted cost ratio naive/sieve
  >= 5 in every cell. If any cell has ratio < 5, the instrument is not trusted:
  verdict inconclusive.
- A/A: G6 (bit-identical rerun); between-seed spread of `FPM_per_rel` within each
  arm and cell is reported as the noise floor (max/min over the 6 runs).
- Negative / null check: G7 (split model) and, reported only, B_lab's split rate
  against `C(n, g) 2^g / p^g`.

## 8. Decision rule (fixed now)

Let `GM(p)` be the geometric mean of R1 over the 6 g = 2 pairs at p, `MIN(p)` the
smallest pair value.

- **Supported**: all gates pass, positive control passes, `GM(p) >= 3.0` at all four
  g = 2 primes and `MIN(p) >= 2.0` at all four. Qualified as
  "supported, robust" if in addition R2 and R3 geometric means are `>= 3.0` at all
  four g = 2 primes; otherwise "supported against the lab instrument only", with
  the failing reading named.
- **Killed**: all gates pass, positive control passes, and `GM(65537) < 3.0`, or
  `GM(p) < 3.0` at two or more of the four g = 2 primes.
- **Inconclusive**: anything else (including a gate or positive-control failure,
  or a g = 2 cell missing because a run hit its budget guard).
- Secondary verdicts, each by the same `GM >= 3.0 at all its primes` rule: H2
  (g = 3, R1), H3 (R2), H5 (P_2lp vs B_2lp FPM per cycle at the 2LP primes). H4 is
  reported (LA multiplication ratio and net), no threshold.

Predictions (not decision rules): P_full FPM per relation in [10, 40] at g = 2 and
[40, 160] at g = 3; B_lab above 100 at g = 2; R2 around 5 at g = 2; P_full rows
heavier in counted elimination than B_lab rows (H4).

## 9. Confounders and how each is isolated

| confounder | isolation |
|---|---|
| relation usefulness differs (homogeneous rows, weight 2g+1, dense anchor columns) | G3 end-to-end solve; H4 counted elimination on equal row counts; row/column weight statistics |
| additions, bucket memory traffic unpriced by FPM | R3 charges them to the pencil only |
| weak baseline implementation (generic Cantor) | R2 explicit-formula floor B_strong |
| inversion price | R4 at I = 1, 10, 30 |
| per-curve split-rate variation | same 3 curves for both arms; pairing by curve and seed |
| seed luck | 2 solver seeds; A/A rerun; spread reported |
| counter bugs | G1 hand counts, cross-check of per-tuple count formula (SPEC 7.6) |
| setup asymmetry | setup excluded from both primaries, reported separately |
| analyst choices | thresholds and analysis program fixed here and in SPEC 10 |
| host load / time | counts only; CPU time recorded only as a budget guard (diagnostic) |

## 10. Exact commands (executor, from the lead directory, one thread)

```sh
cd /home/user/crypto/research/g2plus_leads_20261116/L01_anchored_pencil_sieve
mkdir -p raw raw/rows
python3 -m unittest discover -s tests -v 2> raw/unittest.log
python3 run.py gate --g 2 --p 131  --out raw/gate_g2_p131.jsonl
python3 run.py gate --g 2 --p 257  --out raw/gate_g2_p257.jsonl
python3 run.py gate --g 3 --p 67   --out raw/gate_g3_p67.jsonl
python3 run.py gate --g 3 --p 97   --out raw/gate_g3_p97.jsonl
for g in 2 3; do for p in 1031 4099 16411 65537; do for c in 1 2 3; do for s in 1 2; do
  python3 run.py relphase --g $g --p $p --curve-index $c --seed $s \
    --arms P_full,P_naive1,B_lab --rows-dir raw/rows \
    --out raw/rel_g${g}_p${p}_c${c}_s${s}.jsonl
done; done; done; done
for s in 1 2; do
  python3 run.py relphase --g 2 --p 4099 --curve-index 1 --seed $s \
    --arms P_full,P_naive1,B_lab --out raw/aa_g2_p4099_c1_s${s}.jsonl
done
for g in 2 3; do for p in 4099 16411; do for c in 1 2 3; do
  python3 run.py lp2 --g $g --p $p --curve-index $c --seed 1 \
    --out raw/lp2_g${g}_p${p}_c${c}.jsonl
done; done; done
for gp in "2 1031" "2 4099" "3 1031"; do set -- $gp; for c in 1 2 3; do
  python3 run.py la --g $1 --p $2 --curve-index $c --seed 1 --rows-dir raw/rows \
    --out raw/la_g$1_p$2_c${c}.jsonl
done; done
python3 analyze.py --raw raw --out-json raw/analysis.json --out-md raw/summary.md
sha256sum raw/*.jsonl raw/rows/* raw/analysis.json raw/summary.md > raw/data.sha256
```

Every `run.py` invocation has a CPU-time guard of 540 s (`--cpu-budget`, default);
a run that hits it writes `status: "budget_exhausted"` with its counts so far and
is a missing cell for the decision (reported, not rerun with other parameters).
Rows files are written only for p <= 4099 (`SPEC.md` 8.2). Peak memory is
expected well under 2 GB (largest object: buckets of size p = 65537).

## 11. Limits

Toy primes (at most 2^16). Relation phase only for the primary claim; the LA
reading is secondary and at p <= 4099. Python counts, not native code; FPM is a
stated convention (I = 10). Three curves per cell. Even a supported H1 says
nothing about beating rho: at g = 2 the lab's full-base cost is dominated by
`p^2` linear algebra and its 2LP arm is 3–5x above rho `[pcc:unverified]`
(`genus_ladder_scale_20261115/RESULTS.md`); any end-to-end claim needs a separate
protocol.
