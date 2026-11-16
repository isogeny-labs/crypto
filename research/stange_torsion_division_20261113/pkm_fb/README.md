# Track A: PKM composed-map factor bases on prime fields

Question: does a factor base described by a composition of low-degree maps
(Petit-Kosters-Messeng, PKC 2016) give a decomposition whose per-target
Gröbner cost grows more slowly in |V| than the dense degree-|V| encodings? In
`../growth_fit/` the dense encodings had per-target exponents of about
3.9-4.7 at m = 3. `../growth_fit/` left this as the open question.

Short answer, on the evidence below: no. The PKM tower keeps msolve's largest
F4 step degree constant (4-5 at m = 2, 12-13 at m = 3, as PKM reported).
Per-target time still grows like |V|^2.6-3.0 at m = 2 and about |V|^7 at m = 3.
The dense encoding of the same factor base grows like |V|^1.9-2.2 (m = 2) and
|V|^4.2 (m = 3). At every |V| >= 32 the tower costs more than the dense encoding, by
5x to 130x (m = 2) and up to about 120x (m = 3) [pcc:unverified]. No
encoding has an exponent below 1.

This is an exploratory study, not a bench.record benchmark. Every number is
[pcc:unverified]. Units: msolve wall seconds minus a trivial-system baseline
for the decomposition step. Point additions for the end-to-end toy against
rho.

## Setup

**Factor bases.**
- PKM section 3.3 ("subgroup"). V = mu_n, the subgroup of order n = 2^k in
  F_p^*.
- PKM section 3.4 ("isog", B = 2). An auxiliary curve E'/F_p has a point G
  of order 2^k. V = x(T + <G>) for a point T outside <G>, so |V| = 2^k
  distinct x-coordinates (PKM Lemma 1). phi = phi_k o ... o phi_1 is a chain
  of Vélu 2-isogenies with kernel <G>, and chi = x(phi(T)).

**Primes.** p is the smallest prime >= 2^(m k) with 2^k | p - 1. That is
PKM's choice for m = 2, and we use the same rule for m = 3. msolve 0.6.5
segfaults on these systems for every prime we tried between 2^16 and 2^18
(see Errors), so a p in that range is bumped to the first valid prime above
2^18. That gives p = 262657 for m = 2 at k = 8 and k = 9. A second m = 2 grid
holds p fixed at 1073153 (2^13 | p - 1) for k = 2-10. It removes the
confound between n and p that `../growth_fit/` flagged.

**Target curves.** The target curve is y^2 = x^3 + A x + B with A and B
seeded per (m, k). Each target is a random point R; only xR is used.

**System.** Decompose R = sum_{i=1..m} ±P_i with x(P_i) in V, through
S_{m+1}(X_1, ..., X_m, xR) = 0: S3 for m = 2, as PKM did, and S4 = Res(S3, S3)
for m = 3. The arms differ only in how "X_i in V" is encoded. V, the
summation polynomial and the targets are the same for every arm.

| arm | encoding of X_i in V | variables | equation degrees |
|---|---|---|---|
| D (subgroup) | X_i^n - 1 (= prod (X_i - v), a binomial) | m | n |
| D (isog) | prod_{v in V} (X_i - v), fully dense | m | n |
| T2 | PKM system (4): X_{i,j+1} = X_{i,j}^2 (j < k), 1 - X_{i,k}^2 = 0 | m k | 2 |
| T4 | the same tower with x -> x^4 steps (k even) | m k/2 | 4 |
| I2 | X_{i,j+1} (X_{i,j} - x0_j) = X_{i,j}^2 - x0_j X_{i,j} + t_j (Vélu x-map), last level = chi | m k | 2 |

In every arm the S_{m+1} equation has degree 2 (m = 2) or 4 (m = 3) in each
variable.

**Checks asserted on every target and every arm (420 arm-target systems, 0
failures).** Every solution over the algebraic closure lies in F_p, because
the tower or univariate splits completely over V. Two checks follow:
- The quotient dimension, read from msolve's reduced DRL basis (staircase of
  the grevlex leading monomials), equals the number of ordered m-tuples in V^m
  with S_{m+1} = 0, found by exhaustive search.
- Every brute-force tuple, lifted along the tower, is a zero of every basis
  element.

Together these mean msolve's F_p-solution set equals the brute-force set.

**Timing.** Per arm and target we take the minimum of 3 msolve wall times
(`-g 2 -t 1`; 1 time in the largest cells) and subtract a trivial-system
baseline. An extra untimed run with `-v 2` gives the largest F4 step degree
(`max deg`, the "deg" column of msolve's round log). The arm order is
shuffled per target. Up to 3 runs were in flight at once on a 4-core shared
host.

**Fits.** We fit log(median time) against log|V| by least squares, giving
the slope alpha. The 95% intervals come from a bootstrap over targets: 4000
replicates, with targets resampled within each |V| and the medians
recomputed. Seed 20261115. A cell with median < 3 ms is excluded (the
noise-floor rule of `../growth_fit/`). The table also gives the slope between
the two largest sizes.

**End-to-end toy** (`pkm_e2e.py`). m = 2, subgroup factor base, on a curve of
prime order r with p < 2^16.
- Factor base F: one point per x in V that is an x-coordinate.
- Targets: R = aP + bQ with fresh random a and b. Each target is charged 1
  point addition, as in `../ic_e2e/`, and the scalar multiplication is not
  counted.
- Relations come from msolve's basis. Signs are found with 4 trial
  additions, which are counted.
- The run collects |F| + 1 relations, does the linear algebra mod r, and asserts that the log
  equals the true w.
- The reference is `rho_store_neg` from `../ic_e2e/ic_mt.py`, the strongest
  rho used in the prior work. It stores every point and matches on x, which
  is the negation map. 20 runs per curve, every log checked.

## Commands

```
cd research/stange_torsion_division_20261113/pkm_fb
./run_all.sh stream_m2; ./run_all.sh stream_m3; ./run_all.sh stream_fixed
./run_all.sh stream_e2e; ./run_all.sh stream_isog
python3 fit.py > fit.out      # fits.json, fit_table.md, pkm_fb.{png,svg,pdf}
```

`run_all.sh` lists every `pkm_tower.py` / `pkm_e2e.py` invocation exactly.
For example, `python3 pkm_tower.py 2 8 --targets=6` and
`python3 pkm_tower.py 2 8 --fb=isog --arms=D,I2 --targets=4`. Outputs are
in `runs/`: one JSON per cell with per-target times, xR, quotient
dimensions, step degrees and the curve and isogeny data, plus a `.err`
progress log. Total compute was about 35 minutes of wall time, with up to 3
runs in parallel.

## Results

### Fitted exponents, per-target cost ~ |V|^alpha

From `fit_table.md`. All values are [pcc:unverified].

| grid | arm | |V| used | alpha | 95% bootstrap | slope of last two |
|---|---|---|---|---|---|
| m=2, PKM primes | D (X^n - 1) | 64-2048 (6 sizes) | 2.17 | [2.03, 2.26] | 2.05 |
| m=2, PKM primes | T2 tower | 32-512 (5 sizes) | 2.98 | [2.91, 3.13] | 2.96 |
| m=2, p = 1073153 fixed | D | 64-1024 (5) | 2.04 | [1.94, 2.07] | 2.25 |
| m=2, p = 1073153 fixed | T2 tower | 16-256 (5) | 2.59 | [2.56, 2.71] | 3.29 |
| m=2, isogeny V | D (dense prod) | 64-256 (3) | 1.85 | [1.60, 2.03] | 2.04 |
| m=2, isogeny V | I2 Vélu tower | 32-256 (4) | 2.96 | [2.85, 2.99] | 3.28 |
| m=3, PKM primes | D (X^n - 1) | 8-32 (3) | 4.18 | [4.07, 4.30] | 4.25 |
| m=3, PKM primes | T2 tower | 8-32 (3) | 7.15 | [7.08, 7.18] | 5.50 |

The m = 3 rows rest on 3 sizes, and |V| = 32 has a single target (see
Limits), so their bootstrap intervals are too narrow. The T4 tower is defined
only for even k. On the fixed-p grid its two-point slope is 2.6 (20.7 ms at
|V| = 64, 685 ms at 256), between D and T2.

### Per-target medians and step degrees (selected rows)

The format is median time (largest F4 step degree), then the ratio to D.

| grid | |V| | p | D | T4 | T2 / I2 |
|---|---|---|---|---|---|
| m=2 PKM | 32 | 1153 | 2.2 ms (d34) | - | 14.5 ms (d4), x6.6 |
| m=2 PKM | 64 | 4289 | 5.1 ms (d66) | 26 ms (d7), x5.1 | 66 ms (d5), x12.8 |
| m=2 PKM | 128 | 17921 | 23.5 ms (d130) | - | 412 ms (d5), x17.5 |
| m=2 PKM | 256 | 262657 | 92 ms (d258) | 921 ms (d8), x10.1 | 6.0 s (d5), x66 |
| m=2 PKM | 512 | 262657 | 361 ms (d514) | - | 47 s (d5), x130 |
| m=2 PKM | 2048 | 4206593 | 9.4 s (d2050) | - | - |
| m=2 fixed p | 256 | 1073153 | 78 ms (d258) | 685 ms (d8), x8.8 | 4.2 s (d5), x54 |
| m=2 isog | 256 | 262657 | 78 ms (d258) | - | I2 4.8 s (d5), x61 |
| m=3 PKM | 16 | 4129 | 262 ms (d24) | 3.3 s (d12), x12.7 | 13.4 s (d12), x51 |
| m=3 PKM | 32 | 32801 | 5.0 s (d40) | - | 607 s (d12), x122 |
| m=3 isog | 16 | 4129 | 203 ms (d24) | - | I2 7.4 s (d13), x37 |

Full table: `fit_table.md`. Plot: `pkm_fb.png` / `.svg` / `.pdf`. The plot
shows time against |V| for m = 2 and m = 3 with the fitted lines, and the
largest step degree against |V|.

### End-to-end toy IC against the matched rho reference, in point additions

The unit is elliptic-curve point additions. The IC count is 1 per target plus
4 trial additions per relation, with msolve charged nothing. msolve wall
seconds are listed next to it. Values are means over seeds, every discrete
log is checked against the true w (22 IC runs, 440 rho runs), and every value
is [pcc:unverified].

| |V| | p | arm | seeds | |F| | targets | IC adds | rho_store_neg adds | IC/rho adds | IC msolve s | rho Python s | wall ratio |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 16 | 257 | D / T2 | 3 / 3 | 8.3 | 19 | 58 | 13.8 | 4.2 | 0.08 / 0.09 | 0.26 ms | 295 / 344 |
| 32 | 1153 | D / T2 | 3 / 3 | 15.3 | 44 | 111 | 31.2 | 3.5 | 0.17 / 0.57 | 0.4 ms | 417 / 1610 |
| 64 | 4289 | D / T2 | 3 / 3 | 27.0 | 87 | 199 | 59.2 | 3.4 | 0.71 / 5.97 | 0.7 ms | 1128 / 8659 |
| 128 | 17921 | D | 3 | 61.7 | 153 | 404 | 119.8 | 3.4 | 3.78 | 0.8 ms | 4833 |
| 128 | 17921 | T2 | 1 | 68.0 | 113 | 389 | 116.8 | 3.3 | 47.2 | 0.9 ms | 50838 |

On the same seed, D and T2 produced identical targets and relation counts, as
they should: the encodings have the same solutions. The wall ratio compares
compiled msolve with pure-Python rho, which favours IC. It is shown only to
make the scale visible.

## Verdict

- **Does the composed-map encoding grow more slowly than |V|?** No
  [pcc:unverified]. Every tower arm has alpha well above 1: T2 2.6-3.0, I2
  3.0, T4 about 2.6 at m = 2, and T2 about 7 at m = 3. Every fitted 95%
  lower bound is at or above 2.56.
- **Against the dense encoding of the same V,** the tower is slower at every
  |V| >= 32 and grows faster [pcc:unverified]. The difference is about
  +0.55 to +1.1 in alpha at m = 2 and about +3 at m = 3, and the ratio rises
  from about 5x to 130x. The dense arm wins with the genuinely dense prod(X - v)
  (isogeny V) as well as with the binomial X^n - 1, so the result does not
  depend on the binomial being sparse.
- **A constant degree of regularity does not mean low cost.** msolve's largest
  F4 step degree stays at 4-5 for T2 and I2 (m = 2) and at 12-13 (m = 3).
  That matches PKM's corrected D_reg of 4-6. The cost still grows like a
  power of |V|, because the number of variables grows (m log2|V|). The work
  per step degree also grows: the T2 time ratio per doubling of |V| is 4x to
  15x, and it rises with |V|.
- **This matches PKM's own data.** Their Table 4 (Magma, m = 2, subgroup)
  grows about 7-11x per doubling of |V| from n1 = 7 to 11 (1.14 s to
  4975 s), which is alpha of about 3. Their Table 5 (isogeny) grows the same
  way. Our T2 and I2 slopes of 2.6-3.0 agree with it.
- **There is no improvement over the index-calculus baseline**
  [pcc:unverified].

## In the end-to-end cost model of `../ic_e2e/README.md`

The model is from `../growth_fit/README.md`. The factor base has n = p^beta
points, each relation takes m points, and a target decomposes with
probability about n^m/(m! p). About n relations are needed. If the per-target
cost is n^alpha, the relation phase costs about p n^(alpha+1-m). When alpha
>= m - 1, a larger factor base never helps, and the relation phase stays at
order p or more.

Measured [pcc:unverified]:
- At m = 2, alpha is 1.9-2.2 (D) and 2.6-3.0 (towers). Both are above
  m - 1 = 1, so the relation phase costs about p n^(alpha-1) >= p.
- At m = 3, alpha is 4.2 (D) and about 7 (T2). Both are above m - 1 = 2,
  with the same conclusion.
- Either way this is no better than exhaustive search, far above rho's
  p^(1/2), and above the generic hash-join route (Mt, p^(2/3)) in
  `../growth_fit/`.
- With the decomposition charged nothing, the m = 2 subgroup IC still needs
  3.3-4.2x the point additions of `rho_store_neg` on the same curves. For
  m = 2, n^2 ~ p, so the target count alone is already of order sqrt(p).
  With msolve charged at wall time, the gap is 300x to 50000x and widens with
  p.
- Distance remaining against the matched rho: the measured end-to-end gap is
  3.3-4.2x in additions with free decomposition. The baseline it would have
  to improve on is LP-IC at 2.2-2.4x `rho_store_neg` and plain IC at
  7.9-12.9x Brent rho (`../ic_e2e/`, different primes). The composed-map
  factor base does not narrow either gap [pcc:unverified].

## Limits

- The m = 3 tower stops at |V| = 32 with one target. One T2 msolve run took
  607 s, and the second target was stopped to stay inside the compute budget.
  `runs/m3_k5.json` was rebuilt from the `.err` line, and its xR was not
  saved. The m = 3 fits use 3 sizes.
- The m = 2 T2 arm reaches |V| = 512 (3 targets, 1 timed run each). D reaches
  2048. D at |V| = 4096 was started (38 s for the first run) and stopped. It
  is not saved.
- Small cells (|V| <= 16 or 32) are below the 3 ms noise floor and are
  excluded from the fits. Several cells show negative baseline-subtracted
  times there.
- Wall time comes from a shared 4-core host with up to 3 concurrent runs, and
  msolve 0.6.5 run time is nondeterministic. The per-target times, xR, seeds
  and curve and isogeny data are saved so every system can be regenerated
  and recounted.
- **Primes.** The PKM-rule grid bumps p from about 2^16 to 2^18 at k = 8, so
  the k = 8 and 9 cells use a larger p than PKM's rule gives. The fixed-p
  grid gives similar exponents for D, and lower ones for T2 (2.59 against
  2.98).
- **Encodings tested.** Only DRL F4 in msolve, one variable order, no
  symmetrization, and no Magma. Smooth-order towers with odd prime steps
  (p_j > 2) and cosets aG of the subgroup were not tested.
- **3.4 auxiliary curves** were found by random search with numpy point
  counting, not by CM. With B = 2 the isogeny chain consists only of
  2-isogenies.
- The end-to-end toy runs at m = 2 only, on p < 2^16 (3 curves per size, 1
  for T2 at |V| = 128). It charges 1 addition per target, as `../ic_e2e/`
  does, and leaves the linear algebra uncounted.
- **Knowledge base.** The research-kb MCP server was not available in this
  session, and `scripts/research-kb` has no executable. Sources checked
  instead: the PKM author PDF text (scratch `pkm/pkm.txt`, sections 3.1-3.4
  and 4, Tables 4 and 5) and `../sym_m4/PRIOR_ART.md`.

## Errors hit

- **msolve 0.6.5 segfaults (rc -11).** It segfaults on the m = 2 systems for
  primes 65537, about 70000, 100609, 133121 and 201473 (all arms, every
  retry). It also segfaults on a small 2-variable test system at 65537,
  65539 and 70001 (not at 65521), and in its default parametrization mode (no `-g`) over F_p even on
  `x^4 - 1, y^2 - x - 3` at p = 65537. The same systems run at 65521, 262657,
  1048577 and 2^31 - 1. Workaround: skip (2^16, 2^18), and use only `-g 2`
  output. The cause was not investigated. This matches the segfaults at
  p = 71941 and p = 106277 in `../growth_fit/`.
- **The first I2 encoding wrote uncombined like terms.** For example,
  `220*x-159-x^2+86*x-149`. msolve returned a basis that did not vanish on a
  brute-force solution, and the assertion caught it. The equations now
  combine like terms, and every later I2 cell passes both checks. This
  suggests msolve 0.6.5 mis-parses repeated monomials. That was not
  investigated further.
- **The first end-to-end version had two bugs.**
  - Rho crashed when the walk hit the point at infinity (r = 241). It is now
    handled.
  - IC used an r-adding walk for targets. At r of about 1000 the walk cycles
    after about sqrt(r) steps and repeats targets, so the run stalled
    collecting duplicate relations. Targets are now fresh random aP + bQ.
  Both versions were discarded, and `runs/e2e.jsonl` holds only the final
  version.
- **The run shells were edited while running.** Bash reads scripts
  incrementally, so editing `run_all.sh` while its stream shells ran could
  re-execute commands. The shells were stopped, and their in-flight Python
  children were left to finish. The remaining cells ran from the same
  commands through one-off driver scripts in the scratch directory. No saved
  cell comes from a corrupted invocation.

## Independent review notes (2026-11-15)

A separate reviewer reran cells and recounted from the saved outputs. Verdict: holds with caveats [pcc:unverified].

The negative result stands and reproduces: at matched |V| both PKM towers are slower than the dense encoding, and no exponent is below 1. The quoted numbers are weaker than stated: the m = 3 exponent is a 3-point fit (local slopes 8.8 and 5.5), and the end-to-end ratio overcharges sign recovery (4 additions per relation). Charging 1 addition, as `../ic_e2e` does, gives about 1.8-2.1x rho_store_neg, not 3.3-4.2x. [pcc:unverified]

### Corrections to the text above

Where these conflict with the text above, these take precedence [pcc:unverified].

- The claim that towers are '5x to 130x slower' than dense 'at |V| >= 32' (headline and short answer) is wrong. fit_table.md shows m2_fixp T2 at |V|=32 at x4.8 and m2_fixp T4 at |V|=64 at x3.9.
- '420 arm-target systems': the saved runs/*.json hold 422 arm-target records.
- '22 IC runs, 440 rho runs': the D and T2 lines on the same seed reuse identical rho runs, so there are 240 distinct rho runs over 12 distinct curve instances.
- 'its two-point slope is 2.6 (20.7 ms at |V| = 64, 685 ms at 256)' for T4 at fixed p: log(685.43/20.69)/log 4 = 2.53, so it should read 2.5.
- 'the T2 time ratio per doubling of |V| is 4x to 15x, and it rises with |V|': on the PKM grid the ratios are 3.4 to 14.7 and they fall from 14.7 to 7.8 at the last step. The ratio rises monotonically only on the fixed-p grid.
- 'PKM Table 4 ... grows about 7-11x per doubling from n1 = 7 to 11': the table gives 8.0x, 6.6x, 7.6x and 10.9x, so the range is about 6.6-11x.
- 'Signs are found with 4 trial additions, which are counted' contradicts the claim of matching the ../ic_e2e convention: ic_e2e charges no extra ops for the sign, and the code comment in pkm_e2e.py says one addition per hit. The README's 'IC/rho adds' therefore does not use the ic_e2e unit it compares against.
- The m=3 T2 interval [7.08, 7.18] is reported as a 95% interval. The two local slopes, 8.8 and 5.5, both lie outside it, so it is not a valid 95% interval. The README says it is 'too narrow' but still quotes 7.2 as the exponent in the headline.

### Caveats

- The end-to-end sign recovery is overcharged, and this sets the headline '3.3-4.2x rho_store_neg' figure. In pkm_e2e.py all four sign trials call add(), so 4 additions are counted per relation. The code's own comment says 'charged one addition per hit, as in ../ic_e2e', ic_e2e/ic_mt.py charges no extra ops for the sign, and one addition is enough (compute F1+F2 and compare with ±X). Recounted from runs/e2e.jsonl with 1 add per relation, the ratio is 1.17-2.53 per seed and about 1.8-2.1 per size. That is below the LP-IC 2.2-2.4x baseline the README cites, although on different primes. So this arm is not counted in the same unit as the prior work, and the 3.3-4.2x number is a convention artefact.
- The rho reference leaves out its setup. rho_store_neg builds 20 multipliers and a start point (21 counted adds plus 41 scalar multiplications) and then resets OPS to 0. At these sizes the walk is only 14-130 adds, so the setup is comparable to or larger than the counted work. Charging it changes the ratio a lot: with the 4-add sign charge kept, the ratio is 1.5-3.0. The IC targets' scalar multiplications and the IC's O(|V|) per-target scan for candidate solutions (field square roots) are also uncounted. The additions ratio therefore depends on the counting convention and is not a robust matched comparison.
- The 'honest interval' check fails at m=3. The T2 point estimate of 7.15 comes with a bootstrap CI of [7.08, 7.18], yet the two local slopes are 8.8 and 5.5. The largest size has one target, so the bootstrap gives it zero variance, and run-to-run timing noise is never resampled. The power-law model is also misspecified (curvature), and p grows with |V| (521, 4129, 32801). The headline '|V|^7.2 at m=3' should be reported as a range of about 5.5 to 8.8 with no trustworthy CI.
- The bootstrap intervals at m=2 come from target resampling alone. They leave out timing noise (shared 4-core host, up to 3 concurrent runs, nondeterministic msolve) and model error. On every T2/I2 grid the slope between the last two sizes (3.28-3.29) lies outside or at the edge of the 95% CI (for example fixed-p T2: 3.29 against [2.56, 2.71]). The cost is accelerating, so 'grows like |V|^2.6-3.0' understates the growth at the largest sizes.
- The largest cells mix estimators. m2_k9 (|V|=512) and m3_k5 (|V|=32) used reps=1 (a single run), while every other cell used the minimum of 3. A single run is biased upward against a min-of-3, so the final point of each fit is inflated.
- The m3 |V|=32 point is one target with brute-force solutions = 0, an inconsistent system. Its record was rebuilt from the .err line. The xR, quotient dimension and gb_len are not stored, so that system cannot be recounted from the saved data.
- On the PKM-rule grid p changes with |V| (and jumps from 17921 to 262657 at k=8). Exponents fitted on that grid confound n with p. The fixed-p grid controls for this only at m=2. The m=3 grid has no fixed-p control.
- No interval is given for the e2e ratios. There are 3 curves per size, and only 1 for T2 at |V|=128. Each IC run is a single instance, and the per-seed ratios spread from 2.7 to 5.5.
- The D and T2 e2e runs on the same seed are deterministic duplicates: identical targets, relations and rho runs. Counting them as separate runs (22 IC, 440 rho) inflates the apparent sample size. There are 12 distinct curve instances and 240 distinct rho runs.
- The qualitative negative result stands and reproduces: at matched |V| the towers are slower than the dense encoding, the step degree stays bounded, and every exponent is well above 1. The caveats above weaken the quoted numbers, not this direction.
