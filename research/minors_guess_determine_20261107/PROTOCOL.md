# PROTOCOL: minors / guess-and-determine ECDLP versus matched negation rho (toy sizes)

Status: FROZEN 2026-11-07, before any panel record exists. Panel instances are
`idx 10000..10199`; every development run used `idx <= 96` (and the self-test
`idx 1000..1005` / `5000..5039`), so no panel instance has been seen. The caps and
arm list in section 5 were informed by development calibration runs on those
disjoint indices (declared in README "What was done"); the estimators, decision
rule and sizes were fixed by the lead text (atlas lead `LEAD-EXO-minors-guess-and-determine`).
Experiment class (controlled-experiment skill): comparative claim "method A (reconstructed
minors line) costs less than / scales better than B (matched rho)". The detector is an
exact count, so there is no wall-clock or host confound; the arm-order/interleaving,
host and spot-preemption rows of a variable ledger are inert for the counted unit and are
recorded only as informational wall time.

## 1. Question and what an outcome can close

Q: With every field multiplication and inversion charged, how does the cost per recovered
discrete logarithm of the Mahalanobis minors line (arXiv:1703.07544, 2005.05039, 2310.04132,
2607.09814; see `sources/NOTES.md`) scale with the group-order size from 20 to 40 bits, and
how does it compare with negation rho on the same planted instances?

* If the upper 95% bound of a minors arm's fitted exponent is >= 1/2 and its cost exceeds
  rho's at every size, the lead is closed **for the arms implemented here** (the
  reconstruction in section 2), not for the papers in general.
* If an upper bound is < 1/2, or an arm costs less than rho at some size >= 32 bits, that is
  a real lead and requires independent replication before any claim.
* Gaps to rho are reported as distance remaining (ratio with interval), not pass/fail.
  Relabelling or accounting changes are not improvements.

## 2. Arms (all prime fields `F_p`, `p = 3 mod 4`, prime group order `n` of exactly `bits` bits)

Common reduction (papers 1-4, `sources/NOTES.md`): `l = 3n'`, `n' = bits` (`gd`, `gd_dp1`: `n'`
rounded up to even), `l` distinct random multiples `P_j = n_j G` and `l` of `-n'_i Q`, all `2l`
points distinct; evaluation on the Riemann-Roch basis of `L(l O)`; `C = M2 M1^{-1}` (the
dense part of the papers' `K = [A | J]`; the left kernel and every minor are identical to the
papers' complete-monomial `M`); a zero `s x s` minor `C[alpha|beta]` gives `l` points summing
to `O` and `m = (sum_{j not in beta} n_j) / (sum_{i in alpha} n'_i)`. Each candidate `m` is
checked by a counted `[m]G = Q`. A fresh set of `2l` points and a fresh `C` is one *attempt*.

| arm | scan of `C` (what is searched) | source |
|---|---|---|
| `mgd.all2` | zero entries, then all `2 x 2` minors by slope hashing | 2005.05039 Table 1; 2310.04132 Table 1 |
| `mgd.gesc` | Schur-complement chain `k' = 0..l/2`, zero entries and all `2 x 2` minors of each complement | 2005.05039 sec. 4.1, Table 2; 2310.04132 Table 2 |
| `mgd.apm2` | almost principal minors: contiguous `2 x 2` principal block at each of the `l-1` positions, two row and two column deviations (all `4 x 4` minors containing it) | 2310.04132 sec. 5 (two deviations) |
| `gd` | guess `a` (`l/2` dense columns), reduce `K' = K[a]^T`, `defect d` = smallest `d >= 2` with Eq. (4) success probability >= 0.5, random `b` (`l'-d` columns), signature matrix, hash of all `(d-1)`-subset lines, stop on first verified duplicate | 2607.09814 secs. 3-6, Algorithms 1-2 |
| `gd_dp1` | as `gd` with `d+1` (the development runs showed Eq.-(4)'s smallest `d` is not the cheapest; this is the method's own tuning knob) | 2607.09814 sec. 6 |
| `gd_half` | as `gd` with `n' = 2*ceil(bits/4)` (the paper says `l` may be reduced: "it is the probability that matters", sec. 5) | 2607.09814 sec. 5 |
| `rho` | reference: batched (Montgomery) r-adding rho, `R = 64`, 32 walkers, negation map (canonical `y <= p/2`), 8-entry cycle ring with counted doubling escape, distinguished points `bits/2-5`, restarts by counted scalar multiplications | own implementation (not ecbench's) |
| `rho_aa` | identical code, different RNG salt: the A/A noise floor of the rho estimate | control |

Not implemented, with reason: three-deviation `apm3` (the paper's headline "one kernel up to
47 bits" claim; at `l = 120` a full scan is about `1e12` charged operations per attempt, outside
this panel's budget; by the uniform-minor model below it cannot beat `apm2` per solution, which
the development controls support; this is a declared gap, not a closure), Gray-code/rank-one
updates for `gd` beyond the shared-prefix depth-first elimination already used, two-level
recursion, binary fields (the papers' experiments are in `F_{2^m}`; only prime fields are in scope
of the lead), and the papers' naive determinant enumeration (it is only *counted*, as
`naive_minor_evals`, and never charged: the slope/line hashing is cheaper, so the charged cost is
the reading most favourable to the method).

## 3. Counted unit and charging

* One unit = one multiplication or squaring in `F_p` (`muls`). One inversion costs
  `w = 1.5*bits(p)` units (Fermat-equivalent, primary); sensitivity `w = 3` and `w = 10`.
  Montgomery batch inversion is used by every arm that inverts many values (3 mults per value).
* Everything done through the counted field is charged: instance reduction (the `2l` scalar
  multiplications by double-and-add in affine coordinates, basis evaluation, the `l x l` solve),
  every scan step, every kernel/signature/line computation, rho's jump table, walker starts,
  restarts, steps, escapes, and the final `[m]G = Q` check (both arms).
* Not charged (counted separately): additions/subtractions (`adds`), hash-table inserts and probes
  (`hash_inserts`, rho's `steps` for partition/canonicalisation/DP tests), integer bookkeeping
  mod `n`. Section 7 prices these at 0, 0.1 and 1 unit each and requires the ordering to hold
  across that band, as in the repository's rule for unpriced native work (AGENTS.md 5a); an
  ordering that does not hold across the band is reported as unordered.
* ecbench GAE equivalent: one affine group addition = 3 mults + 1 inversion, so
  `GAE = cost / (3 + w)`. This panel is a standalone count; it is not an ecbench session and
  yields no ecbench record. A cross-check of `rho` against ecbench `rho.negation` GAE on the same
  curves is listed in the README (curve parameters are emitted by `minors_gd gen`).

## 4. Instances and seeds

* Master seed `0x4D494E4F52535F47`; instance seed `mix(mix(master, bits), idx)` (splitmix64
  based, `src/rng.rs`); solver seed `mix(instance seed, FNV1a(arm name))`. Deterministic, no clock,
  no OS randomness; the counts reproduce bit for bit on any host.
* Instance: prime `p = 3 mod 4` uniform in the window that makes the group order exactly `bits`
  bits; random `a, b`; `#E` by Hasse-interval baby-step giant-step; accept only a prime `#E`;
  random point `G` (order `n`); planted `k` uniform in `[1, n-1]`; `Q = kG`. Panel indices 10000..10199.
* The same 200 instances are used for every arm at a size (matched); resampling in the analysis
  draws instance indices jointly for all arms.

## 5. Sizes, caps, budget

Sizes (group-order bits): 20, 24, 28, 32, 36, 40 (six). Per size and arm: 200 instances, each
run until success or until its charged work reaches the size's cap (a capped run is censored and
contributes its work and no success):

| bits | 20 | 24 | 28 | 32 | 36 | 40 |
|---|---|---|---|---|---|---|
| cap per run (charged units) | 1e8 | 5e8 | 2e9 | 3e9 | 2e9 | 2e9 |

Expected total work, from development-run per-attempt costs and the uniform-minor model
(`[pcc:unverified]`, estimates not measurements): about `7e12` charged units, i.e. 25-55 core-hours
at the development throughput of `4-8e7` charged units per core-second; wall time is informational.
Launcher: `run_panel.sh` (48 shards of 25 instances); merge and analysis in the same script.

## 6. Controls

* Positive: the self-test (`minors_gd selftest`) recovers the planted `k` for every arm on
  6 instances at 8-12 bits (rho also at 16, 20), with the explicit point sum of every reported
  minor checked to be `O` (`--dev-check`); a panel record is accepted only if `correct = true`.
* Negative: (a) the verifier rejects `k+1`; (b) `minors_gd controls` compares the number of zero
  `2 x 2` minors of the curve-derived `C` with an i.i.d. uniform matrix and with the uniform
  expectation `C(l,2)^2/(p-1)` (a hidden structure in `C` beyond the uniform `1/p` heuristic would
  show as a ratio far from 1); (c) every panel record's `bad_candidates` must be 0 or explained.
* A/A: `rho` versus `rho_aa` per size (ratio with interval); the width of that interval is the
  noise floor against which every rho ratio is read.

## 7. Estimators (implemented in `minors_gd_analyze.py`, fixed here)

* `rho`, `rho_aa`: `cost_b` = mean charged work over the 200 instances.
* Other arms: `cost_b = (sum of charged work of all runs) / (number of successes)` (exponential /
  Poisson-process MLE on work exposure; assumes the hazard is constant in work inside an attempt, an
  approximation stated here and checked by comparing `cost_b` with the per-attempt estimate in the
  README). Exact Garwood 95% interval on the success count; with 0 successes only the one-sided
  95% lower bound `exposure / 2.996` is reported and that size is excluded from slope fits.
* Exponent `e`: ordinary least squares slope of `log2(cost_b)` against `bits` over the sizes that
  have an estimate (at least three). Two fits are pre-registered: all six sizes (primary, as the
  lead states) and the top three sizes 32, 36, 40 (the asymptotic regime; the all-sizes fit mixes a
  polynomial-cost regime where one attempt suffices with the exponential one). Interval: 5000-draw
  nonparametric bootstrap over instance indices (matched across arms), percentile 2.5% and 97.5%.
  Also reported: `e_arm - e_rho` with the same bootstrap (rho's all-sizes slope is depressed by
  its fixed set-up cost at 20-28 bits, so the difference and the ratios are the readable statistics).
* Ratio to rho per size with bootstrap interval; for sizes with zero successes the lower bound of
  the ratio.
* Headline comparator: the cheapest minors arm per size (an envelope over `mgd.*`, `gd*`), also
  computed by the script's per-arm tables; no arm is dropped.

## 8. Decision rule (mechanical; the script prints it, the README interprets it)

For each arm: `closes` iff `upper95(e_all) >= 0.5` and the arm's cost exceeds rho's at every size
(zero-success sizes use their lower bound); `real lead flag` iff `upper95(e_all) < 0.5` or the
arm is cheaper than rho at a size >= 32 with an interval excluding 1. The lead is reported closed
**for the reconstruction** only if every arm closes; any flagged arm is escalated to independent
replication and nothing is claimed. Results that depend on the inversion price or the unpriced
band (section 3) are reported as unordered.

## 9. Known limits (declared before data)

1. Reconstruction, not the authors' code: the scans are the families the papers name, with the
   cheapest correct implementation; unknown details (instance generation, "kernel amplification",
   principal-minor position) are choices made here.
2. Prime fields; the papers' tables are binary fields.
3. rho is an own, reasonably tuned but unoptimised implementation with set-up cost that dominates
   at 20-28 bits.
4. The censored-MLE treats work within an attempt as memoryless.
5. A standalone count is not an ecbench session: it carries no isolab admission, no independent
   replay certificate and no validator report, so it cannot reach the IC leaderboard or
   `benchmarks/README.md` (validation gate) until re-run through those routes.
