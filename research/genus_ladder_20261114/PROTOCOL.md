# Genus ladder: the discrete logarithm at genus 2, 3, 4, 5 over one field (frozen protocol, 2026-11-14)

Every numeric statement this study produces is `[pcc:unverified]`: one session, a
shared 4-vCPU container, no independent replay. No row moves to the IC leaderboard
or to `benchmarks/README.md` (validation gate). Synthetic random curves only; nothing
here is a statement about a deployed curve.

Written-order note (disclosed): the code (`src/cryptanalysis/genus_ladder.rs`,
`src/cryptanalysis/quartic_ladder.rs`, `src/bin/genus_ladder.rs`,
`src/bin/genus_ladder_analyze.rs`) and its unit tests existed before this file.
Smoke runs were made before the freeze to size the grid (section 8); they are
labelled smoke and are not data. No measured datum of the grid in section 6 existed
when this file was frozen (`frozen.sha256`).

## 1. Questions

Fix a prime field `F_p`. How does the measured cost of the discrete logarithm in a
Jacobian over `F_p` change with the genus and with the kind of curve?

- Q1. Hyperelliptic, `g = 2, 3, 4, 5`: cost of index calculus (full base; double large
  prime) and of negation-map rho on the same instances, and how each grows with `p`.
- Q2. Genus 3 that is not hyperelliptic: smooth plane quartics over the same `F_p`,
  Diem's line relations, against rho on the same quartic and against the genus-3
  hyperelliptic rows.
- Q3. Smith's isogeny: what share of the genus-3 hyperelliptic curves passes Smith's
  Propositions 2 and 4 (so that their logarithm problem moves to a quartic)?
- Q4. Does the discriminant-parity prefilter reduce the native work of the relation
  phase without changing anything else?

Known theory this is measured against, not new claims: rho `~ sqrt(r) ~ p^{g/2}`;
full-base index calculus `~ g! p` relation trials plus `~ p^2` linear algebra;
double large prime `O~(p^{2 - 2/g})` (Gaudry, Thome, Theriault, Diem); plane quartics
`O~(p)` (Diem; Diem and Thome); Smith (2009) about 18.57% of genus-3 hyperelliptic
curves.

## 2. Instances

Hyperelliptic: for each `(p, g)`, `y^2 = f(x)`, `f` monic of degree `2g + 1` with
coefficients from SplitMix64 of `(curve_seed, p, g)` (`random_f`), `f` squarefree,
`curve_seed = 1, 2, ...`. `#J` exactly: `N_1` (and `N_2` for `g >= 3`) by enumeration,
then baby-step giant-step over the window the Weil bounds leave for the remaining
`L`-polynomial coefficients, every candidate checked on 32 random divisors, exactly
one survivor required. Admitted iff `#J = h r`, `r` prime, `r > h`, `h <= 64`. The
first `K` admitted seeds are used; the search stops at `seed_search` seeds.
`G = [h] D` for a random divisor `D`, checked of order `r`; `T = [t] G`, `t` uniform.

Plane quartics: `quartic_ladder::make_instance` (random smooth quartic with a
rational point; `#Pic^0` by `N_1` and baby-step giant-step; same admission rule).

Admission conditions on `#J`. That is a selection: the instances are the near-prime
Jacobians, which is the population a discrete-logarithm cost is about. The search
record (`cell` lines: seeds tried, admitted, reject reasons) is kept.

Instance set-up (point counts, group order, `G`, `T`) is charged to no arm.

## 3. Arms

All hyperelliptic arms run on one group implementation (`JCurve`, Cantor) and one
linear algebra (`ic_control::build_rows`, `peel`, `wiedemann`).

- **A** full base: every abscissa with `f(x)` a non-zero square; relations are the
  reduced divisors `[a]G + [b]T` of full degree whose `u` splits completely. The
  relation source is an r-adding walk (16 jumps, restart every 4096 steps), one group
  addition per trial, as in `ic_positive_control_20261112`.
- **C** double large prime: base `ceil(2 p^{1 - 1/g})` abscissae (capped at all of
  them); at most two places outside the base; cycles of the large-prime graph.
- **a, c**: A and C with the discriminant-parity prefilter.
- **R** rho with the negation map: `rho_reference_walk`, `RhoWalk::negation()`.
  Skipped, and recorded as skipped, when `sqrt(pi r / 4) > rho_cap`.
- **Q_full, Q_double_lp, Q_rho_neg** on plane quartics (section 1, Q2): Diem line
  relations over all rational points; over `ceil(3 sqrt(p))` points with at most two
  large primes; rho. The quartic rho cap is `rho_cap / 100` steps.

Every logarithm is certified: `[t]G = T` by the solver's multiplication and by an
independent right-to-left multiplication. A run that is not certified, or that
returns a `t` other than the planted one, is reported and excluded from the means.

## 4. Units

Recorded per run: group operations; `F_p` multiplications and inversions inside
them (`cantor_mul`, `cantor_inv`; for quartics the `Pic` counter); native
multiplications of the relation phase (`rel_mul`) and of the linear algebra modulo
`r` (`la_mul`); for rho its canonicalisations (`native`).

Two readings, each a band, and two arms are ranked only where the order holds over
the whole band (AGENTS.md 5a):

1. **Within a genus**: `GAE_w = group_ops + w (rel_mul + la_mul + native)`,
   `w in {0, 0.1, 1}`.
2. **Across genera and curve kinds**: `F_p`-multiplication equivalents
   `FPM = cantor_mul + I cantor_inv + rel_mul + kappa la_mul`, with
   `(I, kappa) in {(1, 1), (10, g^2), (30, g^2)}`; the middle one is the primary
   reading. `kappa = g^2` prices a schoolbook multiplication modulo `r ~ p^g`
   against one modulo `p`. Rho's canonicalisation has no multiplication and is not
   in `FPM`.

Wall time is recorded and never used for a verdict: four workers share four vCPUs.

## 5. Controls

- **Certificate** on every run (above).
- **Known answer**: every `t` is planted; a wrong `t` cannot certify.
- **Group law**: unit tests compare `JCurve` with `H64Curve` divisor for divisor at
  genus 2 and 3, check the group axioms at genus 2 to 5, and compare `#J` with a
  brute-force zeta function (`N_1 ... N_g` over `F_{p^k}`) at genus 2 to 5.
- **Prefilter identity (A/A in everything but the treated quantity)**: arms a and c
  must reproduce A and C exactly in trials, candidates, splits, relations, group
  operations, `cantor_mul` and `la_mul` on every paired run. Any mismatch voids Q4.
- **Random-source check**: the fraction of full-degree candidates whose `u` splits
  is compared with the exact share of split divisors, `C(n_x, g) 2^g / #J` (H5).
- **Same instrument**: every hyperelliptic arm uses the same group, the same counters
  and the same linear algebra; rho is the lab's reference walk on that group.

Confounders and how they are held: host and scheduling (counted units only); arm
order (all arms of one task run back to back on one instance, tasks are scheduled
cheapest first across four workers); seeds (derived: curve seeds count up from 1,
solver seeds are `1..S`, the target seed is the curve seed); analyst (decision rules
below are fixed before data; the analysis program existed before data).

## 6. Grid

Two invocations of one binary, concatenated in order into `ladder.jsonl`:

```sh
genus_ladder run --primes 67,131,257,521,1031 --genera 2,3,4,5 --curves 4 --seeds 3 \
    --arms AaCcR --cc 2.0 --qc 3.0 --max-h 8 --threads 4 --rho-cap 3e7 --seed-search 600 \
    --smith --smith-census 2000 --quartic --out part_a.jsonl
genus_ladder run --primes 2053,4099 --genera 2,3,4 --curves 4 --seeds 3 \
    --arms AaCcR --cc 2.0 --qc 3.0 --max-h 8 --threads 4 --rho-cap 3e7 --seed-search 600 \
    --smith --smith-census 2000 --quartic --out part_b.jsonl
```

- Primes: 67, 131, 257, 521, 1031 (first prime at or above `2^6 ... 2^10`) at every
  genus, and 2053, 4099 at genus 2, 3, 4 and for the quartics. Genus 5 stops at
  1031 because its exact group order and its rho are out of budget beyond (the
  order search is about `p^{1.75}` Cantor steps, rho about `p^{2.5}`).
- 4 admitted curves per cell, 3 solver seeds per curve, cofactor `h <= 8` (tighter
  than the positive control's 64, so that `r` stays within a factor 8 of `#J`).
- Rho cap `3e7` expected steps (hyperelliptic), `3e5` (quartic).
- Smith: Propositions 2 and 4 on every admitted genus-3 curve, and a census of 2000
  random squarefree genus-3 curves per prime with no admission filter.
- One session, four workers, counted units. If the session is cut short, the cells
  that are complete are reported and the missing ones are listed; nothing is rerun
  with other parameters under this protocol.

## 7. Analysis and predictions (fixed before data)

`genus_ladder_analyze` computes, per cell `(family, g, p, arm)`: the mean of each
reading with a hierarchical bootstrap 95% interval (resample curves, then runs
within a curve; 2000 resamples; seed 20261114); the ratio to the cell's rho with an
interval; the band verdict (below, above, unresolved: below only if the interval's
upper end is under 1 at every price of the band); the exponent of each reading in
`p` by least squares on `ln mean` against `ln p` (cells with at least three primes),
with a bootstrap interval; the paired prefilter ratios.

Predictions:

| id | prediction | falsified if |
|---|---|---|
| H1 | rho group operations grow as `p^{g/2}`: fitted exponent within `g/2 +- 0.2` at each genus with at least three primes | the 95% interval excludes that range |
| H2 | full-base trials grow as `p^1`: exponent of arm A relation trials in `[0.8, 1.25]` at each genus | the 95% interval excludes that range |
| H3 | at every prime, `GAE_0(A) / GAE_0(R)` falls as the genus rises (2 to 5, where rho ran) | any reversal outside the intervals |
| H4 | genus 2: no IC arm is below rho across the GAE band at any prime. Genus 5: arm a or c is below rho across the GAE band and across the FPM band at p = 1031. Genus 4: arm a or c is below rho across the FPM band at p = 4099 (no prediction for its GAE band) | the opposite band verdict |
| H5 | split rate = `C(n_x, g) 2^g / #J` per instance (`n_x` = full base size): observed / model, pooled over the instances of a `(g, p)` cell, in `[0.9, 1.1]` in every cell with at least 500 splits; and the instance-to-instance spread of `model x g!` widens with the genus | a cell outside, or no widening |
| H6 | prefilter: identical counts on every pair; mean `rel_mul` ratio (on/off) in `[0.45, 0.85]` in every `(g, p, arm)` cell | a mismatch, or a cell mean outside |
| H7 | Smith: share of random genus-3 curves with `full_success` in `[0.15, 0.22]` in the census at each prime | the binomial 95% interval excludes that range |
| H8 | quartics: `FPM(Q_double_lp)` and `FPM(Q_full)` are below `FPM(Q_rho_neg)` across the FPM band at the largest prime where the quartic rho ran | the opposite verdict |
| H9 | every run certifies | any uncertified run (reported with its identity) |

A prediction that fails is reported as failed with its numbers; no prediction is
revised after the freeze. What the study can support is a measured table over the
stated primes, not an asymptotic statement.

## 8. Smoke runs before the freeze (not data)

All on the build at the commit that contains this file, before the freeze:

- `smoke1`: primes 67, 131, 257, genus 2 to 5, 1 curve, 1 seed, `h <= 64`, Smith census
  200 per prime. Every run certified. Used to time the cells and to see the size of
  the constant set-up cost (16 jumps of two short scalar multiplications).
- `smoke2`: primes 67, 257, genus 3 and quartics, 1 curve, 1 seed. Every run certified.
- `smoke3`: primes 257, 521, genus 2 to 5, 5 curves, 2 seeds, arm a only, `h <= 64`.
  Showed that the split rate differs between curves by far more than sampling noise
  (0.65 to 1.32 of `1/g!`), and that `C(n_x, g) 2^g / #J` accounts for it
  (observed / model between 0.84 and 1.16 on 40 instances, mean 1.00 to 1.01 per genus).

Four predictions were restated after these smoke runs and before the freeze
(disclosed; the first drafts are in the Git history of this file's commit only as
this note): H2 was moved from group operations to relation trials, because the
constant walk set-up dominates group operations at the small primes; H4 lost its
GAE-band clause at genus 4, because at `w = 1` the smoke rows already showed the
native smoothness work above rho at `p <= 257`; H5 was replaced by the point-count
model (H5 below), which smoke3 suggested. H5 is therefore a confirmation on new
instances (different primes, `h <= 8`, new seeds at 257 and 521), not a blind
prediction. H6 lost its separate genus-2 clause (below 0.5): the smoke rows
put genus 2 near 0.6, because the fixed factor-base work is inside `rel_mul`.

## 9. Limits

- Toy sizes. The largest groups are about 2^50.
- Smith's explicit isogeny is not built: Q3 measures who passes Propositions 2 and 4,
  and Q2 measures quartics drawn at random, not the images of the Smith-positive
  curves. Joining the two is a model, stated as such in the report.
- Single large prime, sieving, and the Laine-Lauter / Diem-Kochinke variants for
  non-hyperelliptic curves of genus above 3 are not implemented.
- `r` is not bound to a fixed cofactor: `h <= 64`, so `ln r` moves by up to `ln 64`
  between curves of one cell; `mean_ln_r` is reported per cell.
