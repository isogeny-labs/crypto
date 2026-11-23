# L08 ASSESSMENT (assessor, 2026-11-16)

Status: everything below is `[pcc:unverified]`. One producer, one replay that did not finish
and was never compared, and this assessor. No pcc claim exists for any number here.

## Verdict

**INCONCLUSIVE (instrument)**, by the frozen PROTOCOL §8 rule. The A_2LP series is also
**void as a measurement**: in the arms that set it, almost every relation counted as
"useful" is a duplicate of a line already found, and such duplicates add nothing to the rank.

## 1. Integrity checks

- `sha256sum -c frozen.sha256`: PROTOCOL.md OK, SPEC.md OK.
- Replay: I compared `replay/` with `raw/` byte for byte (`cmp`). selftest, both gate
  files and all 26 stage-1 run files are **identical**. Six d=6 files are identical. The
  file `opt_d6m4_p1031` stops after 19 of its 20 rows: the replay process is gone, and the
  rows it wrote are a byte-identical prefix of the raw file. The replay's own `analyze` step
  failed with exit 2 because its script did not quote the glob (`unrecognized arguments`).
  I ran `analyze --glob 'replay/run_d*.jsonl'` into scratch, and the output equals
  `raw/analysis.json` exactly. Stage 1 is therefore deterministic and reproducible. That
  does not make it correct.
- Re-derivation: my own script reads `raw/run_d*.jsonl` (280 rows) and gives the
  following slopes. Both match the analyzer.
  - d=5: A_2LP slope 0.981 over all 7 primes and 1.211 over the 4 largest. The cF that
    minimises cost is [2,2,2,1,1,1,1].
  - d=4: A_2LP slope 1.618.
- Independent mathematics on the d=5, p=131 curve 0. I brute-forced all p^2 + p + 1
  projective points of F and found 131 rational points. Exactly 2 are singular, (0,0) and
  (1,1). The 129 smooth points equal the code's `pts` set.
- The gates ran: G3 Weil reciprocity passed 200/200 and G3-PC failed 197-200/200. That
  verifier is live.

## 2. Instrument defects (decisive)

**D1: duplicate lines are counted as useful relations.** This is a SPEC defect, faithfully
implemented. SPEC §5.3 does not remove repeated lines. A line holding k factor-base points is
produced by C(k,2) different pairs; for d=5, a 2LP line has 3 factor-base points and a full
line has 5. The union-find of SPEC §5.5 treats a repeated edge as a new even cycle, so each
repeat adds +1 to `useful`. The difference of two copies of one relation is the zero vector,
so a repeat adds no rank.

The executor's DEVIATIONS item 1 mentions this, citing "65 of 8232" in the FB arm, where it
is negligible. In the 2LP arms it dominates. From the raw rows, curve 0:

| d | p | arm | useful (code) | rel_dup_lines | needed |
|---|---|---|---|---|---|
| 5 | 8209 | 2LP cF=1 | 419 | 419 | 419 |
| 5 | 8209 | 2LP cF=2 | 830 | 823 | 830 |
| 5 | 8209 | 2LP cF=4 | 1652 | 543 | 1652 |
| 4 | 8209 | 2LP cF=2 | 190 | 111 | 190 |

Averaged over 4 curves, rel_dup_lines / useful is between 0.96 and 1.00 for d=5 at cF=1 and
cF=2, at every p.

**Independent rank check.** I re-ran `collect` and kept only the first copy of each line.
I then computed the true number of large-prime-free combinations: (unique full relations) +
(edges − rank of the incidence matrix over F_ℓ). The table compares this with the code's
`useful`; every run listed stopped as "successful" (not exhausted).

| d | p | cF | useful (code) | genuine (dedup) | needed |
|---|---|---|---|---|---|
| 5 | 1031 | 1 | 111 | **0** | 111 |
| 5 | 1031 | 2 | 214 | **6** | 214 |
| 5 | 1031 | 4 | 420 | 277 | 420 |
| 5 | 2053 | 1 | 171 | **0** | 171 |
| 5 | 2053 | 2 | 334 | **7** | 334 |
| 5 | 2053 | 4 | 660 | 435 | 660 |
| 4 | 1031 | 2 | 72 | 21 | 72 |
| 4 | 1031 | 4 | 137 | 109 | 137 |

At p=131 cF=2, the code counts 60 useful relations, against 1 genuine.

The d=5 A_2LP series uses cF=2 at small p and cF=1 at large p. It therefore measures the
cost of finding about `needed` **repeated lines**, not of solving a relation system. Its
slope of 0.981 (bootstrap 0.973 / 0.989) is an artefact of this defect, and so is its
apparent beat of 4/3. The per-row IC/rho ratios for d=5 (0.069 down to 0.0013 against
rho_393) are understated by an unknown, large factor. The FB arm is only mildly affected:
0.7% of relations are duplicates at p=8209, and 8167 genuine relations remain against nb=8143.

**D2: the LA model is charged on the matrix before large-prime elimination.** Under SPEC §5.6,
cols = nb + the vertices of contributing components. For d=4 at cF=2, mat_cols/nb rises from
3.5 at p=131 to 18 at p=8209, and LA is 55% to 96% of fpm_total. This drives the d=4
control to a slope of 1.618 and is why PC1 fails.

The same d=4 control has a slope of 1.04 when LA is excluded (enumeration plus relations,
cF=2), matching the theoretical 1. This is a diagnostic only, computed after the data, and
not part of the frozen rule. PC1 failing is a correct detection of a broken instrument.

**D3, minor: the G4 tolerance.** Five main-run rows fail G4, with |diff| from 0.0103 to
0.0115 (for example d=5, p=4099, FB, curve 0, with 23,741 eligible lines). Per-curve
Chebotarev deviations of order 1/√p (about 0.016 at p=4099) make ±0.01 too tight for a
single curve. The frozen rule attaches no action to G4, and these failures do not change the
verdict.

Fairness of the baseline: all arms share the same curves, seeds, counted enumeration, FPM
unit and LA model, and nothing in the FB arm is needlessly expensive. The comparison is
nevertheless unfair in the 2LP arm's favour, because of D1.

## 3. The frozen rule, applied as written

1. G1 and G2 failures in main runs: 0 of 480 rows. G3 passes. The lead is not killed for
   verification.
2. Instrument checks:
   - PC1: d=4 A_2LP slope 1.618, outside [0.55, 1.25]. **False.**
   - PC2: d=5 FB slope over the 4 largest primes is 1.910, at least 1.75. True.
   - AA: |1.392 − 1.378| = 0.014. True. Both replicates carry D1 equally, so this
     agreement is not evidence of validity.

   → **INCONCLUSIVE-instrument.** No further verdict.

Effect sizes and spread, for the record only, since D1 voids them:

- d=5 A_2LP: e = 0.981, curve bootstrap [0.973, 0.989].
- d=5 cF=2 cost at p=8209 ranges from 1.75e8 to 1.84e8 FPM across the 4 curves.
- d=5 FB at p=8209 ranges from 1.20e9 to 1.32e9 FPM; its slope is 1.788 over all primes and
  1.910 over the 4 largest.
- d=6 optional slopes (1.11, 1.17) share D1 and D2.

## 4. Distance to matched rho

No valid IC/rho distance for the genus-4 relation ladder can be read from this run. The
A_2LP ratios are not usable because of D1. The only arm close to valid is FB, which includes
full relations only, applies no double-large-prime saving, and charges no descent or Pic^0
arithmetic. Its partial cost at d=5, p=8209 is 1.25e9 FPM. That is **0.038 × rho_393** and
**0.0017 × rho_8900**. Its slope (about 1.9) is close to rho's exponent g/2 = 2, so this
ratio is roughly flat in p.

That FB ratio is computed after the data, is partial, and is a lower bound for IC. The
rho_393 step is a hyperelliptic Cantor step, while a non-hyperelliptic genus-4 Pic step
costs more, which pushes rho upward. Treat it as an unverified indication that a
full-base Diem relation phase on these models is not obviously above rho at p ≈ 2^13. It is
not a T0 claim: descent, real LA and a Pic step charged in matched units are all missing.

## 5. Single most valuable next step

Freeze a v2 SPEC and rerun d=4 (control) and d=5 on the same grid. Change exactly these:

- (a) Count each line once, either by keeping a set of seen line keys before the union-find
  or by accepting a pair only when it is the two lowest-indexed factor-base points on its
  line. Charge `lines_tried` for every pair visited.
- (b) Charge LA on the matrix after large-prime elimination (nb columns, with the weight of
  the merged rows), or report LA separately from collection, so that PC1 can measure the
  relation phase.
- (c) Make the G4 tolerance a multiple of the binomial standard error plus an O(1/√p)
  allowance, or make it advisory.

Expect the cF=1 and cF=2 cells at d=5 to exhaust at small p under v2. That outcome is
informative and should be pre-registered as such.

Files examined: PROTOCOL.md, SPEC.md, DEVIATIONS.md, RESULTS.md, `code/l08.py` (`collect`,
`build_curve`, `analyze`), `raw/*.jsonl`, `raw/analysis.json`, `replay/`. The assessor's
scratch scripts (not part of the lead) are `indep.py`, `indep2.py` and `chk.py` in
`/tmp/claude-0/-home-user/6fd00ecc-ad17-50e3-84b9-4e7db4d013e1/scratchpad/`.
