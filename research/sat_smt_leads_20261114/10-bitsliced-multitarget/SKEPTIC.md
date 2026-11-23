# Skeptic review: idea 10, bit-sliced multi-target propagation

Reviewed 2026-11-14, one core, about 15 minutes. Every statement here is an
unverified review assertion [pcc:unverified]. Scratch:
`/tmp/claude-0/-home-user/570c81c4-197a-558d-aa76-c9ff505d9731/scratchpad/skeptic-10-bitsliced-multitarget/`
(`chk.py`, `rerun_l5.jsonl`, `inst/`).

## Outcome

- Reproduced: **partly**. Part (a) at l = 5 reproduces exactly in counts and
  units, and within timing noise in wall time. Part (b) was not re-run; its raw
  data were re-analysed.
- Verdict supported: **weaker than stated**.
- Corrected verdict: **kill-criterion-met**, but only on the letter of K2 and
  on the E2 side arm. The pre-registered K2 statistic (E1, all variables) is a
  variable-numbering artifact and carries no evidence either way.

## 1. Instance generator and answers (own code, l = 5)

Own GF(2^31) arithmetic, own curve lift and addition, none of the producer's
field or curve functions:

- Brute force over all pairs of V for all 512 targets of `instance_l5.json`:
  0 mismatches with the stored reference counts. Planted: 256 distinct targets,
  each with exactly 2 ordered solutions. Random: 256 distinct, all 0.
- 18 of the 32 x-values in V carry rational points (matches `validation.json`).
  S3(x1, x2, x(P1 +- P2)) = 0 on all 306 sums and differences of those points.
- SAT cross-check (producer's encoders, my arithmetic for the check): 12
  planted + 12 random instance targets + 12 targets I built as R = P1 + P2 from
  real points; E1 under CaDiCaL 1.5.3 and MiniSat, E2 under CaDiCaL: 108 of 108
  SAT/UNSAT answers agree with my brute force; every model satisfies S3 = 0 with
  x1 != x2; 24 of 24 E1 models on point-planted targets lift to points with
  P1 +- P2 = +-R.
- Finding: the "planted" class of part (a) is x-level only. Of the 256 planted
  targets at l = 5, 155 have a solution pair that lifts to rational points
  summing to the target; 101 do not. The protocol declares the compiled arms
  x-level, so this is disclosed, but "decomposable" in part (a) means
  "S3 has a root in V x V", not "the point decomposes".

## 2. Re-run of the smallest cell

`bs 5 5` rebuilt from `bs.rs`; the regenerated `instance_l5.json` is
byte-identical to the stored one. Both `check` rows match exactly (word-op
counts, fallbacks, 256/0 decomposable). Units per target match exactly in all
arms.

Wall ns per target, minimum of 3 (stored / mine):

| arm | planted | random |
|---|---|---|
| enum_scalar | 5086 / 6385 | 5374 / 5479 |
| enum_bitsliced | 476 / 849 | 475 / 734 |
| scan_scalar | 2191 / 2434 | 2179 / 2439 |
| scan_bitsliced | 2023 / 1938 | 550 / 682 |
| scan_bs / enum_bs | 4.25 / 2.28 | 1.16 / 0.93 |
| scan_bs / scan_scalar | 0.92 / 0.80 | 0.25 / 0.28 |

`enum_bitsliced` moved by 1.5x to 1.8x between the two runs. Inside the stored
data the max/min spread of the three repetitions reaches 1.86x
(`scan_bitsliced`, random). Any wall ratio in the README is good to about a
factor of 2, not to three digits. The fitted slopes and README tables
recompute from `raw_part_a.jsonl`.

## 3. Fairness

Holds:

- Same V, same targets, same seeds for every arm; all arms asserted to return
  the same count per target.
- Baselines are the right ones for an m = 2 leaf: bit-sliced enumeration, the
  scalar linear 2+1 scan, and a join. Per-batch plane construction is inside
  the timed region of both bit-sliced arms. No symmetry factor is taken by any
  arm.
- Slopes in part (a) are fitted on 6 sizes; planted and random are reported
  separately.
- All five arms that were run are reported, including `join`.
- File times are consistent with the protocol preceding the data
  (PROTOCOL.md 09:19:38, bs.rs 09:22:13, raw_part_a 09:24:53) and the protocol
  still lists l = 4 and l = 8, which were later dropped, so it was not edited
  to fit. There is no git history, so this is not proof.

Problems:

1. **K2 on E1 measures variable numbering, not search sharing.** The E1
   formula has a different number of variables for each target (238 to 268 at
   l = 5) because the XOR chunk auxiliaries depend on X. In
   `raw_part_b.json` the common prefix is exactly 0 for every pair whose two
   formulas have different variable counts (118/118, 117/117, 102/102 at
   l = 5, 6, 7) and is long for every pair with equal counts (230 and 233;
   318, 322, 326; 433, 433, 433 decisions). The "unexplained 2-3 long-prefix
   pairs per cell" are exactly the same-count pairs. So the median of 0 was
   decided by the encoder, before the solver ran, and the long prefixes are
   most likely the solver's first conflict-free descent in index order, which
   is not shared work either. The pre-registered K2 statistic is uninformative.
2. **The E2 arm is what actually supports the kill**, and it is not the arm K2
   names. With one clause set and identical numbering the common prefix is 0 to
   10 decisions, median fraction at most 0.2%, maximum 2.6%, in every class
   combination. This is a real measurement, with thin support: 8 + 8 targets
   (7 random at l = 7), one seed, l = 5..7 only, one solver for traces, Python
   callbacks.
3. **A common decision prefix of an unmodified CDCL does not bound lane
   sharing.** The idea is a solver whose decisions are restricted to summand
   bits in a fixed shared order. The treatment arm of part (a) is such a
   procedure and shares 100% of its x1 decisions across 64 targets. A stock
   solver with per-target activity scores diverging says nothing about that
   design. The statistic follows the task card, so the kill follows the
   criterion, but the claim "this bounds what lane sharing could ever give" is
   not established. The (b2) figure, 40% to 60% of implied literals
   target-independent, points the other way and is above the 10% bar.
4. Pair statistics pool planted (SAT) and random (UNSAT) targets. The
   per-class medians in `results.json` are also near 0, so pooling does not
   change the outcome here.
5. Part (a) wall ratios: no pinning, no A/A arm, arm order fixed rather than
   rotated, noise up to about 1.9x. K1 "not met" at 0.084 is safe against that.
   The credited ratio scan_bs / scan_scalar "1.02 at l = 10" is 1 within noise.
   Its upward trend is supported by the counted units (1.29 against 1.00 bits
   per l), not by wall time alone. The wall slope of `scan_bitsliced` on
   planted targets is 0.83, not the 1.17 quoted for random; the README table
   shows the random class only.
6. Neither scan arm is tuned (fixed l + 16 rows, 64-bit lanes only; the scalar
   arm carries a fixed setup that dominates at l = 5). The constant between
   them is not settled by this pilot in either direction.
7. Part (b) slopes rest on 3 sizes and 7 to 8 targets per cell.
8. n = 31 only, m = 2 leaf only, as the producer states.

## 4. Does the verdict follow?

- K1 (bit-sliced scan not below bit-sliced enumeration at the largest l): not
  met, 0.084 at l = 10, robust to the timing noise. Agreed. The producer is
  right that this gap is 2^l against 2^(2l) and was already known from the
  linear 2+1 scan.
- K2 (median E1 all-variable prefix fraction below 10% in every cell): met on
  the letter in all three cells run. The protocol planned l = 4..8; 5..7 were
  measured.
- The kill criterion is an OR, so "kill-criterion-met" follows formally. The
  statistic that triggers it is void (problem 1). The E2 side arm gives the
  same answer honestly on small data (problem 2). Neither bounds a purpose-built
  lane-shared solver (problem 3).

What the pilot does show, independent of K2: on this leaf at n = 31,
bit-slicing the scan gives a constant between about 0.25x and 1x of the scalar
scan with a rising word-operation count per target (about l^2 per leaf), far
from the protocol's own "below 1/8 with no upward trend" bar for a lead. That
is the more useful negative result, and it is a statement about a constant
factor on an untuned pair of implementations, not about an exponent.

Corrected headline: kill criterion met on the letter of K2; the E1 statistic
is an encoding artifact; E2 shows at most 2.6% common prefix on 8 + 8 targets,
one seed, l = 5..7; the common-prefix statistic does not bound a fixed-order
lane-shared solver; the part (a) ratio to the scalar scan is about 1 within a
factor of 2 at l = 10 and gives no exponent gain.

## Side effect of this review

Importing `part_b.py` for the SAT cross-check created a `__pycache__`
directory in the pilot directory. I removed it. No producer file was changed.
