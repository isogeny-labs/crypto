# N4: relation-collection index calculus on the GHS plane quartic vs a matched rho on E0(K)

2026-10-19. Branch `claude/quartic-ic-n4-20261019`, stacked on #2227 (canonical `Pic^0` arithmetic).
Frozen protocol: [PROTOCOL.md](PROTOCOL.md) (commit `9a63c9161`, not edited). Report with graphs:
[REPORT.pdf](REPORT.pdf) (source [REPORT.typ](REPORT.typ)). Records: `results/`. Sources: `common.rs`
(counted `F_q`/`K`), `ghs.rs` (instance, `E0`, transfer), `ic.rs` (root finding, relations, Lanczos, split of a
degree-3 divisor), `solver.rs` (IC solve, matched rho), `summ.rs` (aggregation), `n4.rs` (driver),
`t1_rerun.rs` (T1). Native Rust; Python only lays out `reports/` figures from `results/summary_all.json`.

## Answer in one paragraph

The old plane-quartic "pure IC" was subgroup enumeration (T1), so no earlier plane-quartic number was an
index-calculus cost. A real pipeline (relation collection on lines, Lanczos mod `ell`, descent through the
canonical `Pic^0` arithmetic, no enumeration) solves every instance (51 of 51 IC-B0 units verified: 5 instances at each q <= 65479, 3 at q = 131071 and 262111) and costs, cold and
end to end, `IC/rho = 0.122 [0.10, 0.16]` at q = 262111 (`ell` about 4.5e15), below rho and band-resolved for all
q >= 4051, with a measured exponent of **1.083 [1.040, 1.128] against rho's 1.485 [1.437, 1.525]** on the primary
range q >= 4051 (0.998 [0.971, 1.023] against 1.486 over all 11 sizes q = 223..262111). By the frozen decision
rule the lead N4 stays **open**, it is not closed. The scope is the GHS-weak family only (`E0/F_{q^3}`, `4 | #E0`,
special `m = 2` form), plain rho, a shared VM: no generic-curve, prime-order, deployed-curve or key-recovery claim.
Addition cost is not the bottleneck where IC wins (T3).

## T1. The three split GHS solves (q = 5, 7, 11) re-run with canonical keys

`t1_rerun.rs` includes `quartic_ic.rs` unmodified, reproduces its cost exactly (S = 650 896.03, 2 751 662.59,
37 920 187.89, 30/30 targets per variant correct) and re-answers every target with the keys of #2227.

| q | ell | old S (before targets) | share of `ic_full` spent in the loop over `n = 1..ell` | line relations built, rank / columns | old deg-6 keys / canonical keys | old deg-3 `pic_canon` merges (distinct classes) | targets: old correct / canonical correct (n/a) |
|--:|--:|--:|--:|:--|:--|--:|:--|
| 5 | 37 | 650 443 | 99.82% | 0, rank 0 / 6 | 21 / 18 | 1 | 10/10, 5 (5) |
| 7 | 89 | 2 751 342 | 99.95% | 1, rank 1 / 8 | 41 / 38 | 7 | 10/10, 7 (3) |
| 11 | 331 | 37 920 004 | 99.99% | 3, rank 3 / 12 | 164 / 164 | 20 | 10/10, 10 (0) |

* **Survives:** the planted scalar is recovered in 30/30 per variant (verified on `E0`); the deg-6 divisor table
  is correct (0 collisions, one-to-one with the canonical keys wherever the canonical path applies); the
  cost figures are real costs of what the program did; the three quartics are the ones of the build study.
* **Wrong or misleading:** (i) the `recovery:ic` label: the dlog comes from a table of `phi(nG)` for all `n < ell`
  (the loop above), the relation rows are built and discarded (`let _ = rows`), and the relations that exist are
  0, 1 and 3 (rank 0, 1, 3 of 6, 8, 12 unknowns: nothing is determined by them); (ii) the degree-3 `pic_canon`
  reduction merges distinct classes (1, 7, 20 pairs, confirmed independently: same old key, different canonical
  key), so any log read through it was unsafe (the 7 of 90 transfer divisors that left the primary table were
  still answered correctly, through other paths); (iii) `2lp` is `full` (`ic_2lp` calls `ic_full`); (iv) the rho
  arm records steps only (`ops = 0`, `S = 0`), so no ratio to rho existed; (v) the note that the factor-base
  points are 4-primary is not supported: all points except `P0` have a nonzero `ell`-part
  (5 of 6, 7 of 8, 11 of 12).
* **Not applicable, not failures:** the canonical path (`reduce_deg6`) needs four independent cubics through the
  divisor; 5, 3 and 0 of the 10 targets per q had a non-reduced transfer divisor and are listed as n/a.
* This is the same flaw that the null lead L5 of #2168 found (cost `ell^2.5`): enumerating the subgroup.

## T2. A real relation-collection solve, protocol frozen before data

Design (PROTOCOL.md): factor base `N = ceil(3.0 q^(3/5))` rational points; every line through two factor-base
points is bucketed; 3 points on a line give a single-large-prime relation, 4 a full relation; Lanczos on
`A^T D A` mod `ell`; descent `x <- x + [P_j - P0]` until the canonical degree-3 representative splits into three
known points (the split test is the base locus of its net of conics, cubic in the pencil parameter);
`k = (y2 - y0)/(y1 - y0)`, verified `kG = Q`. Baseline pipeline for the improvement statement: the same code with
all `q + 1` points (A0; A0r adds retries). Matched rho: plain, same counters.

Checks before the grid (`results/instance_checks.txt`, 22 of 22 `CHECK OK`, q = 223..262111, two seeds each): `N1 = q+1`
(brute force for q <= 8191), transfer homomorphism (8 triples, 0 failures), `#Jac * x = 0` on random classes.
Controls during the grid: all IC runs verified; sampled relations summed to one class through the canonical
group law (`relations_bad = 0`); hold-out rows in the kernel (`holdout_bad = 0`); rho verified on every run, steps
over `sqrt(ell)` between 1.20 and 1.46 at every size (theory 1.25; criterion [1.0, 1.5]); independent same-target
replay at 4 sizes (IC and rho recover the same `k`).

### Cold single-target cost, M-eq (F_q multiplication equivalents), mean over 3-5 instances; rho over 18-150 runs

| arm | q | units ok/total | rho runs | IC M-eq | rho M-eq | IC/rho [95% boot] w=0 | w=0.1 | w=1 | resolved |
|:--|--:|--:|--:|--:|--:|:--|:--|:--|:--|
| A0 | 223 | 3/3 | 90 | 2.148e6 | 2.001e5 | 10.73 [9.70,11.94] | 10.74 [9.72,11.94] | 10.79 [9.81,12.00] | rho lower |
| A0 | 499 | 3/3 | 90 | 8.544e6 | 5.869e5 | 14.56 [13.42,15.81] | 14.72 [13.57,15.99] | 15.50 [14.27,16.88] | rho lower |
| A0 | 991 | 2/3 | 90 | 3.090e7 | 1.450e6 | 21.31 [19.31,23.66] | 21.63 [19.58,24.01] | 23.07 [20.85,25.67] | rho lower |
| A0 | 2011 | 1/3 | 90 | 1.257e8 | 4.508e6 | 27.89 [25.08,31.11] | 28.39 [25.52,31.67] | 30.63 [27.52,34.20] | rho lower |
| A0 | 4051 | 1/3 | 90 | 5.029e8 | 1.340e7 | 37.53 [33.96,41.93] | 38.27 [34.63,42.76] | 41.62 [37.65,46.52] | rho lower |
| A0r | 223 | 3/3 | 90 | 2.040e6 | 1.997e5 | 10.22 [9.33,11.26] | 10.27 [9.38,11.32] | 10.51 [9.59,11.68] | rho lower |
| A0r | 499 | 3/3 | 90 | 8.488e6 | 5.839e5 | 14.54 [13.35,15.82] | 14.70 [13.51,16.01] | 15.49 [14.24,16.88] | rho lower |
| A0r | 991 | 3/3 | 90 | 3.117e7 | 1.503e6 | 20.74 [18.79,22.91] | 21.10 [19.12,23.33] | 22.78 [20.60,25.25] | rho lower |
| A0r | 2011 | 3/3 | 90 | 2.242e8 | 4.493e6 | 49.91 [25.87,91.79] | 50.42 [26.34,92.31] | 52.77 [28.47,94.68] | rho lower |
| A0r | 4051 | 3/3 | 90 | 5.030e8 | 1.334e7 | 37.70 [34.13,42.22] | 38.43 [34.78,43.03] | 41.71 [37.74,46.72] | rho lower |
| B0 | 223 | 5/5 | 150 | 8.592e5 | 2.036e5 | 4.22 [3.55,4.92] | 4.01 [3.38,4.67] | 2.94 [2.55,3.36] | rho lower |
| B0 | 499 | 5/5 | 150 | 1.591e6 | 5.782e5 | 2.75 [2.24,3.19] | 2.62 [2.14,3.02] | 1.97 [1.69,2.22] | rho lower |
| B0 | 991 | 5/5 | 150 | 2.545e6 | 1.507e6 | 1.69 [1.49,1.91] | 1.62 [1.44,1.83] | 1.32 [1.19,1.45] | rho lower |
| B0 | 2011 | 5/5 | 150 | 4.843e6 | 4.482e6 | 1.08 [0.97,1.21] | 1.05 [0.94,1.17] | 0.90 [0.82,0.99] | unresolved |


(`resolved`: the order holds at w = 0, 0.1 and 1 GAE per unpriced operation with the bootstrap interval of the
ratio excluding 1.) Rows `A0` are the frozen A0 arm, which had no retry rule: 10 of 15 units verified (the others
stopped at the hold-out check "kernel not unique" at 1.2 rows per column), so its means are over successes only.
Rows `A0r` are a post-hoc arm with +0.3 rows per retry, every attempt charged: 15 of 15 verified.

### Exponents (fit of the size means in `ln q`, bootstrap over units and runs)

| arm | sizes | IC exponent [95% boot] (w=0) | rho exponent [95% boot] (w=0) | exponent gap rho-IC [95% boot] | IC (w=1) | rho (w=1) |
|:--|--:|:--|:--|:--|:--|:--|
| A0 | 5 | 1.889 [1.868,1.903] | 1.451 [1.410,1.489] | -0.438 [-0.483,-0.395] | 1.955 [1.943,1.962] | 1.485 [1.442,1.525] |
phase exponents (A0): fb=1.09 relations=1.96 linear_algebra=1.98 descent=-0.00 verify=0.21
| A0r | 5 | 1.985 [1.894,2.072] | 1.451 [1.408,1.492] | -0.535 [-0.626,-0.422] | 2.038 [1.957,2.123] | 1.484 [1.439,1.528] |
phase exponents (A0r): fb=1.16 relations=2.03 linear_algebra=2.06 descent=0.05 verify=0.19
| B0 | 11 | 0.998 [0.971,1.023] | 1.486 [1.464,1.505] | 0.487 [0.455,0.521] | 1.086 [1.061,1.110] | 1.496 [1.474,1.516] |
phase exponents (B0): fb=0.67 relations=1.07 linear_algebra=1.15 log_recovery=0.87 descent=0.39 verify=0.13


Predictions: P1 (all verified) holds. P2 (rho `1.50 +- 0.10`) holds (1.485). P3 (IC-B0 in [1.0, 1.4]) holds
(1.083 primary, 0.998 all sizes; phase exponents: factor base 0.69, relations 1.11, linear algebra 1.13,
descent 0.48). P4 (A0 about 2 and above B0 for q >= 991) holds (A0r 1.9855). P5 (crossover) holds: rho is cheaper below
q about 2000, the arms are unresolved at 2011 (w = 1 favours IC, w = 0 does not), IC is band-resolved lower from
q = 4051 on.

Phase shares of IC-B0 (the headline): linear algebra rises from 22% (q = 223) to 57% (4051) and 71% (262111);
relations 13% to 25% to 27%; descent falls from 55% to 13% to 1%; factor base 9% to 5% to 1%
(`reports/phases.svg`).

**Improvement over the baseline.** IC-B0 / IC-A0r cost: A0r is 2.4x B0 at q = 223, 12x at 991, 46x at 2011 and 53x at 4051
(M-eq, from the two tables above), and 0.998 against 1.9855 in exponent: reduced factor base with a large prime
is a measured improvement of the (first valid) baseline. **Distance to rho** is the column above (ratio, with its
interval); it is below 1 for q >= 4051 in this family.

## T3. Is the addition cost the bottleneck?

Literature (not implemented here): Flon, Oyono, Ritzenthaler, *Fast addition on non-hyperelliptic genus 3 curves*
(ePrint 2004/118; Number Theory and its Applications, 2008) report 148M + 15SQ + 2I (curves with a rational flex,
characteristic > 5; a later version 163M + 2I for doubling), against 3523M + 44I for our generic add in #2227: about
20 to 25 times more multiplications. Abu Salem and Khuri-Makdisi give 117M + 2I for `C_{3,4}` curves only, which
these quartics are not. I did not read the full texts (search results only), and the FOR algorithm needs a rational
flex and a specific divisor representation, so porting it is a separate piece of work (scoped, not done).

Measured need: the descent (the only phase that adds classes) is 55% of IC-B0 at q = 223 but 13% at 4051, 5% at 8191
and 1% at q = 262111. With the descent charged at zero (a bound for any faster addition) the ratio to rho at w = 0
moves 4.22 -> 1.88 (q = 223), 1.08 -> 0.86 (2011), 0.735 -> 0.637 (4051), 0.122 -> 0.120 (262111); with a 20x cheaper
add (descent / 20) 0.866 at 2011 and 0.642 at 4051 (derived from the recorded phase means, `ic_meq_descent_free`,
`ic_meq_descent_div20`). A cheaper addition would move the crossover (log-interpolated between sizes, not measured) from about 2300 to about 1500 and does not change
the exponent. **Not worth doing before the linear algebra (57-74% for q >= 4051) and the relation phase (22-30%).**

## T4. Stop rule

The rule (exponent not below q^(3/2) over >= 3 sizes closes the lead) is not triggered: the gap `e_rho - e_IC` is 0.40
[0.34, 0.47] at w = 0 and 0.33 at w = 1 on the primary range (7 sizes), 0.49 [0.46, 0.52] over all 11 sizes. **N4 is
open.** What the data do not show: the exponent over a longer range (the fit covers 2^12 to 2^18; the model's
asymptote for this design is `q^(6/5)` and the measured 1.08 sits below it because lower-order phases are still
large), anything for generic or prime-order `E/F_{q^3}`, or the distance to a rho with a negation map.

## Deviations, limits, recorded

* Protocol revisions after the freeze: (1) `n4 check` above q = 8191 draws sampled points instead of the O(q^2) point
  enumeration (control tool only). (2) Arm A0 as frozen failed 5 of 15 units (hold-out check); A0r (rows +0.3 per retry,
  all attempts charged) was added post hoc and is labelled as such; B0 data are unchanged. (3) After the grid the
  summarizer gained wall-time, descent-bound and rho-calibration fields (recomputed from the same records; the grid
  binary's counts are unchanged).
* Not bit-reproducible run to run: Rust `HashMap` iteration order randomises the order of relations, so a unit re-run
  can take a different (valid) path (one A0 unit failed in the grid and passed in A0r with the same internal seed).
  Counts, correctness and verification do not depend on it; replay needs the recorded relation order, which is not
  stored. Fix by sorting buckets before use (next version).
* Wall time (practicality note only): IC 10.7 s vs rho 18.9 s per run at q = 262111 (ratio 0.57, M-eq ratio 0.12); the
  relation phase costs 29.7 ns per M-eq (random hash access, priced only through the band) against 3.0 for rho. IC is
  slower than rho in wall time below q about 3e4. Shared 4-core Xeon VM, L0 isolation, four single-thread processes.
* Conversion constants are measured time ratios per size (median over units): inv/mul 13 to 18, lmul/mul about 1.0 to 1.2,
  one `E0` addition 56 mults + 1 inversion (about 70 M-eq). Not re-run with other constants (the exponent fit of each
  phase uses counts; only the sums use the ratios).
* Unpriced native work per arm and the band: IC hash operations (14.4 million at q = 262111) and rho distinguished-point
  tests (one per step, about 1.3e8) are priced at w = 0, 0.1 and 1 GAE; `F_q` additions are counted, outside the band.
* Negation-map sensitivity (not measured): rho could be about `sqrt(2)` cheaper; IC/rho at q = 4051 would be about 1.0
  (unresolved), 0.17 at q = 262111; the exponent gap is unchanged.
* Only the first seeds' instances; 3 instances at k = 17, 18; the k = 17 interval is wide (24 rho runs).

## Next improvement (iterative criterion)

Linear algebra is 57-74% of the cost for q >= 4051: replace Lanczos on `A^T D A` by block Lanczos or structured
Gaussian elimination before it (smaller `n`), sort-based bucketing for the relation stage (memory-bound), tune `c`
and the row margin (currently `c = 3.0`, 1.2 rows per column, chosen from pilots at q = 2011 to 16363), add the negation
map and the second large prime. Each is a candidate improvement against this frozen B0 baseline.

## Reproduce

```bash
cd research/quartic_n4_20261019
rustc --edition 2021 -O n4.rs -o n4 && rustc --edition 2021 -O t1_rerun.rs -o t1_rerun
./n4 check 4051 1                      # instance control
./n4 unit 16363 1 3.0 24 1.2           # one B0 unit with 24 rho runs (flat JSONL)
./run_grid.sh $PWD/n4 results/grid     # the frozen grid
./n4 summarize results/summary_all.json 0 results/grid/all_records.jsonl
./t1_rerun 5 && ./t1_rerun 7 && ./t1_rerun 11
python3 reports/plot.py results/summary_all.json reports   # layout only
```
