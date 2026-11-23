# L03 assessment: re-balance the genus-3 2LP factor base when relations are cheap

Assessor: independent of the producer and the replayer. All numbers are `[pcc:unverified]`.
This is not a benchmark: no leaderboard row, no `benchmarks/` row, no speed claim.

## Verdict: **supported** (frozen rule PROTOCOL §6.4, applied as written)

H1 decision inputs (d = 10 synthetic discount, lambda = 1, band mid (I, kappa) = (10, 9), primary
reading; cc* picked on seeds {1,2}, evaluated on seeds {3,4}):

| p | cc* | nb* | G_10 | 95% CI (paired hierarchical bootstrap) | per-run G over the 6 E runs (min, max) |
|---|---|---|---|---|---|
| 8209 | 0.354 | 144 | 0.8225 | [0.8173, 0.8291] | 0.815, 0.834 |
| 16411 | 0.354 | 229 | 0.8320 | [0.8302, 0.8335] | 0.830, 0.834 |
| 32771 | 0.354 | 363 | 0.8402 | [0.8383, 0.8416] | 0.837, 0.842 |

Rule: supported if the 95% lower bound of G_10 >= 0.15 at >= 2 of 3 primes including 32771.
Lower bounds are 0.817, 0.830, 0.838 at all three primes. The kill branch (point < 0.15 at >= 2
primes, or 32771 upper bound < 0.15) does not fire. Every gate (Q1 to Q10, Q_count) passed.
No deviation touches the instrument, the thresholds or the decision path, so the verdict stands.

**What "supported" means.** The claim is conditional on a synthetic per-trial discount. No engine
yet delivers 10x cheaper relations. The H1 number also includes a large part that does not
depend on cheap relations (see below). The measured, non-synthetic result is the d = 1 re-tuning.

## Decomposition (the honest reading of the 82 to 84%)

The protocol itself flagged H1 as confounded (§1.7). The data confirm it.

- **G_1 (measured, d = 1, lab-baseline re-tuning):** cc* = 1.0 at all three primes. FPM_mid falls
  by 0.362 / 0.372 / 0.396 (CI lower bounds 0.343 / 0.362 / 0.391). The lab's cc = 2 is past the
  optimum even at measured relation cost: the LA share at cc = 2 is 0.745 / 0.757 / 0.772.
  This is the only part of the result that does not rest on a synthetic parameter.
- **G_inc (H2, cheap-relation-specific):** at d = 10, moving from cc*_1 = 1.0 to
  cc*_10 = 0.354 saves another 0.474 / 0.488 / 0.496 (32771 CI lower bound 0.4946 >= 0.10).
  H2 held.
- **Attribution at d = 10 (E, mid):** engine discount alone at cc = 2 gives E_10 = 0.772 / 0.782 /
  0.795 (only about 21% saved, because LA dominates). Re-balancing then gives R_10 = 0.177 / 0.168 /
  0.160. So a cheaper relation engine without re-balancing buys little; with re-balancing it
  buys about 5x more. This is the useful finding for L01-type leads.
- **H3 (cost model):** slope of ln trials on ln fb_cols = -0.809 / -0.817 / -0.843 (in
  [-1.3, -0.7]); slope of ln la_mul on ln core_cols = 1.766 / 1.813 / 1.839 (in [1.7, 2.4]).
  H3 held, but slope_T is shallower than the ideal -1 and slope_L sits near the low edge.
  The frozen model's predictions (cc*_1 about 1.1, gain about 32%; cc*_10 about 0.5, gain
  about 81%) were close: observed cc*_1 = 1.0 (gain 36 to 40%), cc*_10 = 0.354 (gain 82 to 84%).

## Audit of the instrument

1. **Frozen files.** `sha256sum -c frozen.sha256`: PROTOCOL.md OK, SPEC.md OK.
2. **Port fidelity.** `mat_vec`, `mat_t_vec` and `m_apply` in `sim/src/la.rs` differ from
   `src/cryptanalysis/chained_transfer/ic_control.rs` 1127-1188 only by `pub(crate)`. The relation
   loop in `sim/src/run.rs` matches `genus_ladder.rs` 1380-1470 statement by statement for the
   2LP arm. Counter placement is the same, plus the declared M1 to M8 splits. Non-+-1 matrix
   entries are charged as `la_mul`, so heavier rows at small cc are priced.
3. **Counters.** On all 396 runs, `cantor_mul == cm_setup + cm_loop + cm_verify`. Every run is
   certified, `recovered == t_planted`, `attempts == 1`, none exhausted. So no failed-certificate
   or retry work sits in `cm_loop`, and D2 (the exhausted-run split) has no effect.
4. **Calibration is real, not copied.** Per-run values at cc = 2 differ from the lab's
   `scale_g3.jsonl` runs, for example (8209, c6, s1): lab trials 42975 against the port's 43977.
   Run-to-run CV is 0.9 to 1.7% on both sides, so the 15 Q7 agreements of 0.01 to 0.6% on
   pooled means are consistent with independent RNG streams. The SPEC changed the RNG
   (SplitMix64, not StdRng).
5. **Baseline fairness.** Same instrument, curves, seeds and unit on every arm. Common random
   numbers across cc. The baseline is the lab's own cc = 2 setting (`C_double_lp+pre`), with
   nothing extra added. The port's cc = 2 IC/rho ratios (0.592 / 0.741 / 0.538, derived from
   the table) reproduce the lab's 0.591 / 0.741 / 0.537. The baseline is not artificially
   inflated. It is simply mis-tuned, and that is the d = 1 finding.
6. **Relations verified independently of the walk.** Q6 re-added each relation's places with
   Cantor and compared the sum with the walk divisor. Result: 105,879 verified, 0 bad, at
   p = 8209 over all cc. This uses the port's own Cantor, so I added the independent check in 8.
7. **Could a bug produce the effect?** The effect needs LA to dominate at cc = 2 and fall roughly
   as nb^1.8, while trials grow only as nb^-0.8. Both are visible in raw per-cc means, and both
   match the lab's LA share. Pricing LA additions makes the gain larger, not smaller (S_add:
   G_10 = 0.853 / 0.863 / 0.871). Band lo (I = 1, kappa = 1) shrinks it to 0.41 to 0.44, still
   far above 0.15. I found no mechanism by which a counting error would produce the effect.
8. **Independent mathematics spot-check.** I wrote a separate pure-Python Cantor/Mumford
   implementation (scratchpad `cantor_check.py`, not the port's code). On instances
   (8209, c6), (16411, c27), (32771, c16) and (32771, c31) it confirms: G is a valid divisor,
   [r]G = 0, [t_planted]G = T, and h·r = order. Every cc run in those files recovered t_planted.
9. **Re-derivation from raw.** From `raw/runs_*.jsonl` with my own code (formula from SPEC §7),
   I reproduce cc* = 1.0 (d = 1) and 0.354 (d = 10) at every prime. I reproduce G_1 = 0.3623 /
   0.3716 / 0.3956 and G_10 = 0.8225 / 0.8320 / 0.8402 to 4 digits. The E-set argmin equals
   the S-set cc* in all six cases, so winner's curse plays no role.
10. **Replay.** The replayer's 83 outputs are byte-identical to `raw/` and `analysis/`.
    Its binary sha256 matches `raw/binary.sha256`.

Deviations D1 to D5: D1 changes only a unit-test construction (T13), not the solver. D3 and D4
are bookkeeping. None of them invalidates the rule.

## Concerns (do not change the verdict)

- **The H1 test is a weak one.** At d = 10, cc = 2 is about 6x past its optimum, so the 15% bar
  was certain to be cleared once G_1 > 0.15. Read the result through G_1, G_inc and the
  attribution, not through G_10.
- d is synthetic: it scales the whole per-trial cost and keeps yield and relation
  distribution fixed. A real cheaper engine (L01) may change both.
- lambda (SGE) is held fixed across cc. Those rows are parameter-labelled and never decisive.
  With lambda = 0.255 the d = 1 re-tuning gain drops to 4.5 to 5.8% (cc* = 1.41). SGE and
  re-balancing overlap: both attack the LA term.
- The unit prices LA multiplications only. The +-1 additions that carry matrix weight are
  unpriced in the primary reading.
- Small primes only (p <= 32771). Only 3 curves per prime and 2 evaluation seeds. p = 65537 was
  not run (declared). The optimal cc may drift with p. The slope_T trend (-0.81 to -0.84)
  suggests it is stable over this range.
- Instrument is the standalone port, not `src/bin/genus_ladder`. It is calibrated statistically,
  not bit-for-bit.
- No validation gate: no replay certificate, no §5a GAE band. No ordering against rho is
  stated.

## Distance to matched rho (band mid; distance remaining, not a gate)

Reading (i), the lab's negation-rho FPM_mid on these curves `[pcc:unverified]`:

| setting | 8209 | 16411 | 32771 |
|---|---|---|---|
| lab baseline, cc = 2, d = 1 | 0.591 | 0.741 | 0.537 |
| re-tuned, cc* = 1, d = 1 (**measured**) | 0.377 | 0.466 | 0.325 |
| cc* = 0.354, d = 10 (**synthetic**) | 0.081 | 0.097 | 0.068 |

Model-rho reading (ii) at d = 1, cc*: 0.434 / 0.664 / 0.444.

The measured re-tuning moves IC/rho from about 0.54 to 0.74 down to about 0.33 to 0.47 at
these toy sizes. At these sizes, genus-3 IC was already below 1 against the lab rho reference.
This lead does not change the exponent. It improves the constant (a lower base constant
`cc` in nb = cc·p^{2/3}). Without the §5a band and a validated rho reference, no ordering
claim is made.

## Single most valuable next step

Re-measure the d = 1 re-tuning in the lab instrument itself. Run `src/bin/genus_ladder --cc 1.0`
against `--cc 2.0` on the same nine curves, under a frozen paired spec with a replay
certificate and the §5a band. That turns the only non-synthetic gain here (36 to 40% FPM_mid)
into a validated improvement of the index-calculus baseline, and makes cc = 1 the default that
L01-type cheap-relation leads are charged against. Add p = 65537 or genus 4 if the budget
allows, to check that cc* stays near 1.
