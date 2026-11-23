# T6: the Edwards t = 5, G = 0 per-target F4 cost: replication

Date 2026-11-03. Wave 2, lead T6. Protocol: [`PROTOCOL.md`](PROTOCOL.md), frozen in
commit `2b330f250a1bca382b81023d95d556fccd7078d3` before any data was taken. Runner:
[`scripts/run.sh`](scripts/run.sh) (frozen). Analysis: [`scripts/analyze.py`](scripts/analyze.py)
(written after the freeze; it applies the frozen rules and writes
[`analysis/summary.json`](analysis/summary.json)). Raw outputs: `data/*.json`, `.status`,
`.stderr`; hashes in [`data/SHA256SUMS`](data/SHA256SUMS).

All numbers are counted F4 multiply-adds per target (`muladds_per_target`, "mpt") from
`curve25519_family f4cost`. No timing claims. R = mpt(edwards-bits) / mpt(bits), paired on
the same k, t, G, seed and target count.

## Headline

On this binary, Edwards-y costs **0.6386x** of bits in F4 mul-adds per target at
t = 5, G = 0. That holds on all 7 seeds at k = 20 (R from 0.638553 to 0.638568) and on 3 of 3
seeds at k = 24. The saving is **uniform**: every one of 40 single targets costs
81.28M (Edwards) against 127.28M (bits), with a CV near 1e-6. Both arms solve at
degree 6. The original 0.646 did not reproduce exactly, because the Edwards count moved by
1.16% on the new binary (P0 failed, binary drift). The effect is gone at G >= 1 (R = 0.9986,
0.9972, 1.0000) and at t = 4 and 6 (0.9969, 0.9870). The `bits` form panicked at k = 16, 18, 22
(9 cells, F2), so P3 could not be evaluated on 3 of its 4 curves.

**Exploratory reading** (not preregistered): Edwards mpt is about 81.28M at t = 5, G = 0 on
every curve k = 16 to 24 (bits is 127.28M on k = 20 and 24). In Block A and Block E (k = 20) each Edwards target runs
**10 F4 steps** where bits runs **11**, both at degree 6 with 256 terms per specialisation
(the k = 18, S2 Edwards cell is an exception: 1006 steps over 100 targets, see Block B note).
So at the measured cells the 0.64x appears to come from the shape of the Edwards polynomial
system: F4 refutes it one step sooner. Within the measured cells it did not vary with the
curve or the target. Lower Edwards cost per target at the
best cell (G = 3) is not seen (R = 1.000), so this does not improve the best algebraic
configuration. **Rho remains ahead.**

## Instrument

- Binary `/home/user/ctarget/release/curve25519_family`, sha256
  `7a438b1d8ce9d7a0b8049a4331599727e9574ea4201f9c7957befa3184ba98be` (checked by `run.sh`,
  recorded in `data/binary.sha256`). Matches the frozen value, so F1 did not trigger.
- Command per cell: `f4cost <k> <form> <t> <G> <targets> <seed> 3600 2 full 0 30000000`.
- Run 2026-11-03T16:55:07Z to 17:02:42Z, strictly sequential within this lead. Another wave-2
  lead (T3) ran its own f4cost process on the same 4-core host at the same time. Load
  average was about 2 to 7 (1-min values in `data/*.status` range 2.04 to 6.89). The counts are deterministic and no cell was capped, so load does not
  affect them.
- 151 cells, 160 `f4cost` invocations counting the 9 F2 reruns: 142 exit 0 and 18 exit 101
  (9 cells failed twice, F2, below).

## Results

### Block A: k = 20, t = 5, G = 0, 100 targets

| seed | mpt bits | mpt edwards-bits | mpt bits@976503 | R = edw/bits | bits@976503 / bits |
|---|--:|--:|--:|--:|--:|
| 2026111505 (orig) | 127,284,344.31 | 81,279,130.34 | 127,284,307.74 | 0.638563 | 0.9999997 |
| 3110300001 | 127,283,626.51 | 81,279,224.78 | 127,284,340.49 | 0.638568 | 1.0000056 |
| 3110300002 | 127,283,611.54 | 81,279,176.14 | 127,283,669.58 | 0.638567 | 1.0000005 |
| 3110300003 | 127,284,368.25 | 81,279,233.23 | 127,284,329.40 | 0.638564 | 0.9999997 |
| 3110300004 | 127,284,363.99 | 81,277,869.00 | 127,284,314.12 | 0.638553 | 0.9999996 |
| 3110300005 | 127,284,348.09 | 81,279,234.33 | 127,283,709.87 | 0.638564 | 0.9999950 |
| 3110300006 | 127,283,593.93 | 81,278,545.35 | 127,284,360.82 | 0.638563 | 1.0000060 |

Columns and coverage on every seed: bits 11 cols, 42 points covered; edwards-bits 14 cols,
58 points; bits@976503 24 cols, 198 points. All three arms: `solving_degree_max` 6,
`poly_terms_per_specialisation` 256, `calls_per_target` 1.0, `undecided` 0, `hits` 0,
`capped` false, 100/100 targets.

P0 reference values (lead 5, old binary) against the new binary at seed 2026111505:
bits 127,284,344.31 against 127,284,344.31 (rel. diff 0); bits@976503 127,284,307.74 against
127,284,307.74 (0); edwards-bits 82,230,218.94 against **81,279,130.34 (rel. diff 1.157e-2)**.

### Block B: other family curves, t = 5, G = 0, 100 targets

| k | seed | mpt bits | mpt edwards-bits | R | cols b / e | covered b / e |
|---|---|--:|--:|--:|---|---|
| 16 | S1 / S2 / S3 | **failed (F2)** | 81,272,955.37 / 81,275,020.22 / 81,275,983.04 | n/a | – / 16 | – / 56 |
| 18 | S1 / S2 / S3 | **failed (F2)** | 81,275,916.23 / 81,344,125.24 / 81,278,264.47 | n/a | – / 13 | – / 58 |
| 22 | S1 / S2 / S3 | **failed (F2)** | 81,279,252.26 / 81,279,270.00 / 81,279,232.71 | n/a | – / 18 | – / 78 |
| 24 | 3110300001 | 127,284,442.35 | 81,279,270.00 | 0.638564 | 14 / 13 | 58 / 34 |
| 24 | 3110300002 | 127,284,437.56 | 81,279,270.00 | 0.638564 | 14 / 13 | 58 / 34 |
| 24 | 3110300003 | 127,284,440.94 | 81,279,265.38 | 0.638564 | 14 / 13 | 58 / 34 |

Median R at k = 24: 0.6385641. Note: the k = 18, S2 Edwards cell (81,344,125.24, the maximum of
the Edwards range) has hits = 1 (hits_verified 1, pkm_consistent_systems 1, refutations 99,
steps 1006, max_rows_max 1457, max_cols_max 919, pkm_grid_fp_mults_uncharged 1178); its count
includes one consistent (solving) system, not only refutations. The pair is dropped (bits F2),
so no verdict changes. The `bits` (lo = 0) form exited with code 101 on both attempts
at k = 16, 18 and 22, on all three seeds. The panic was
`src/bin/curve25519_family.rs:1657:13: assertion failed: fb.abscissa_list.iter().all(|&x| x < span)`
(see `data/B_k{16,18,22}_bits_*.stderr` and `*.try1.stderr`). The Edwards arms at those k ran
cleanly.

### Block C: guess sweep, k = 20, t = 5, 100 targets

| G | seed | mpt bits | mpt edwards-bits | R |
|---|---|--:|--:|--:|
| 0 | S1 / S2 / S3 | (block A) | (block A) | 0.638568 / 0.638567 / 0.638564 |
| 1 | 3110300001 | 32,293,196.46 | 32,248,538.73 | 0.998617 |
| 1 | 3110300002 | 32,293,144.94 | 32,249,521.44 | 0.998649 |
| 1 | 3110300003 | 32,293,394.34 | 32,248,662.37 | 0.998615 |
| 2 | 3110300001 | 12,787,948.47 | 12,751,518.36 | 0.997151 |
| 2 | 3110300002 | 12,787,952.54 | 12,751,518.92 | 0.997151 |
| 2 | 3110300003 | 12,787,952.64 | 12,751,543.85 | 0.997153 |
| 3 | 3110300001 | 3,036,909.50 | 3,036,911.20 | 1.000001 |
| 3 | 3110300002 | 3,036,909.24 | 3,036,907.97 | 1.000000 |
| 3 | 3110300003 | 3,036,910.92 | 3,036,907.18 | 0.999999 |

Median R by G: 0.638567, 0.998617, 0.997151, 1.000000 (the G = 3 median is the S2 value, 0.99999958).

### Block D: neighbouring t, k = 20, G = 0, 100 targets

| t | seed | mpt bits | mpt edwards-bits | R |
|---|---|--:|--:|--:|
| 4 | 3110300001 | 4,046,084.46 | 4,033,476.32 | 0.996884 |
| 4 | 3110300002 | 4,046,079.22 | 4,033,476.50 | 0.996885 |
| 4 | 3110300003 | 4,046,085.32 | 4,033,474.85 | 0.996883 |
| 6 | 3110300001 | 1,673,238,631.64 | 1,651,468,755.09 | 0.986989 |

At t = 6 both arms run at degree 7 with 484 terms and 1800 F4 steps over 100 targets.

### Block E: 40 single targets, k = 20, t = 5, G = 0

| arm | n | median mpt | mean mpt | CV | distinct values |
|---|--:|--:|--:|--:|---|
| bits | 40 | 127,284,443 | 127,284,304.45 | 3.89e-6 | 12 values, 127,281,326 to 127,284,443 |
| edwards-bits | 40 | 81,279,270 | 81,279,235.70 | 1.43e-6 | 5 values, 81,278,757 to 81,279,270 |

Median Edwards / median bits = 0.638564. Every target in both arms: `solving_degree_max` 6,
1 refutation, 0 hits. F4 `steps` is **11 for every bits target and 10 for every Edwards
target**. "Cheap" Edwards targets (mpt ≤ 0.8 × median bits): 40 of 40. Within ±5% of the
median bits: 0 of 40. Cheap targets at degree ≤ 5: 0 of 40.

## Predictions

| # | prediction (frozen rule) | measured | outcome |
|---|---|---|---|
| P0 | new binary reproduces bits, edwards-bits and bits@976503 at seed 2026111505 within 1e-3 | bits 0, bits@976503 0, edwards-bits 1.157e-2 | **failed** (binary drift in the Edwards count. Analysis continues on the new binary, and the comparison with lead 5's 0.646 is flagged) |
| P1 | replicates: R ≤ 0.90 in ≥ 5 of 6 fresh seeds at k = 20 | 6/6 at R ≤ 0.90 (0.638553 to 0.638568); 0/6 in [0.97, 1.03] | **held** (replicates) |
| P2 | 0.99 ≤ bits@976503 / bits ≤ 1.01 in 7/7 seeds | 7/7 (0.999995 to 1.000006) | **held** |
| P3 | median R ≤ 0.90 for ≥ 3 of 4 curves k ∈ {16, 18, 22, 24} | only k = 24 evaluable (0.638564); k = 16, 18, 22 bits arms failed (F2) | **inconclusive**. With the denominator shrunk to 1, the strict rule cannot give "holds" (needs ≥ 3), so P3 did not hold. The frozen rule outcome is "mixed". The cause is missing data, not a counter-observation |
| P4 | G = 3: 0.98 ≤ R ≤ 1.02 on S1–S3, and median R non-decreasing in G with 0.01 slack | G = 3: 1.000001, 1.000000, 0.999999; medians 0.6386 ≤ 0.9986, 0.9986 → 0.9972 (drop 0.0015 < 0.01), 0.9972 ≤ 1.0000 | **held** |
| P5 | 0.97 ≤ R ≤ 1.03 at t = 4 (S1–S3) and t = 6 (S1) | 0.996884, 0.996885, 0.996883, 0.986989 | **held** (4/4) |
| P6(a) | bits single-target CV ≤ 0.02 | 3.89e-6 | **held** |
| P6(b) | Edwards is a mixture: ≥ 20% cheap and ≥ 20% within ±5% of median bits | 40/40 cheap, 0/40 near | **failed**. The alternative "uniform" holds (Edwards CV 1.43e-6 ≤ 0.05, median ratio 0.6386 ≤ 0.90) |
| P6(c) | ≥ 80% of cheap Edwards targets at degree ≤ 5 | not judged, because P6(b) failed (rule). For the record, 0/40 cheap targets are at degree ≤ 5; all are at 6 | **not judged** |

## Exploratory (not preregistered)

1. **Same cost on every curve.** At t = 5, G = 0 Edwards mpt is 81.27M to 81.34M on
   k = 16, 18, 20, 22, 24, even though the columns (13 to 18) and coverage (34 to 78) differ.
   Bits is 127.28M on k = 20 and 24. In the one (t, G) cell measured (t = 5, G = 0; Edwards on
   5 curves, bits on only 2: k = 20 and 24) the per-target count did not vary with the curve or
   the window's coverage; this is not shown for other (t, G).
2. **The mechanism is one fewer F4 step.** Per target, Edwards takes 10 steps against 11 for bits
   (block A: 1000 against 1100 steps per 100 targets; block E: 10 against 11 for every target).
   Both reach degree 6. Edwards has `max_rows_max` 1271 against 1159, `max_nnz_max` 196,710
   against 282,072, and the same `max_cols_max` 848. Edwards uses more rows but fewer non-zeros
   and one fewer step. That fits 46.0M fewer mul-adds per target: the dropped step would be the
   last and densest degree-6 step of the bits run. This is a reading of aggregate fields, not a
   per-step trace. Confirming it needs a protocol with `trace` or per-step counts.
3. **Why only here.** Once G ≥ 1 or t ≠ 5, the two forms are within 1.3% (the largest gap is
   t = 6: 0.9870). This fits a refutation that ends one step earlier only when the t = 5,
   G = 0 system's degree-6 staircase happens to close early in Edwards-y coordinates. That is a
   hypothesis. It was not tested.
4. **Native work.** `pkm_system_fp_mults_uncharged` is 31,300 for Edwards against 19,300 for bits
   per 100 targets at k = 20, t = 5, G = 0, that is +120 Fp mults per target. Compared in this
   lead's own unit (counted F4 mul-adds, plus native Fp mults at w per operation), the extra at
   most 120 native mults per target against a saving of about 4.6·10^7 counted mul-adds leaves
   Edwards cheaper at every w in the band. In ecbench's accounting F4 mul-adds are themselves
   unpriced native work priced at w, so at w = 0 both arms' F4 work prices to 0 and the arms are
   tied at w = 0, not ordered. The within-method conclusion stands.
5. **Per relation** (driven by coverage): at k = 20, `muladds_per_relation_at_exact_coverage` is
   1.84·10^11 (Edwards) against 3.98·10^11 (bits lo = 0) and 8.43·10^10 (bits@976503). The
   selected translation frame is still the cheapest per relation, because it covers 198 points
   against 58. At k = 24 Edwards covers fewer points than bits (34 against 58): 5.01·10^12
   against 4.60·10^12 per relation.
6. **Old binary.** On lead 5's binary Edwards was 82,230,218.94 (0.646). The new binary takes
   about 951k fewer mul-adds per target on Edwards, and the bits count is unchanged. At the one
   cell compared (k = 20, t = 5, G = 0, seed 2026111505) the change between binaries affected
   only the Edwards path.

## Costs

151 `f4cost` processes, 7 min 35 s wall (16:55:07Z to 17:02:42Z). The 142 successful
cells reported `wall_s` summing to 445.6 s and counted 742,901,867,611 F4 mul-adds in total.
The 9 failed cells panicked within about 1 s each (18 failed invocations; 160 invocations in
all).

## Distance remaining

This is a constant factor of 0.64 on the per-target F4 cost of the algebraic
(pkm-semaev, m = 2) track, at the one cell t = 5, G = 0 only. At the cheapest measured
cell of that t (G = 3, about 3.04M mul-adds per target) Edwards and bits are equal (R = 1.000). So
this finding does not improve the best algebraic configuration. It is not an improvement over
the index-calculus baseline unless a later end-to-end measurement shows one. The best algebraic arm
measured is 18x (w = 0), 9.6·10^5x (w = 0.1) and 5.9·10^6x (w = 1) behind rho-neg at
k = 20 (`curve25519_algebraic_ecbench_20261102`), and the generic baseline (proj-a) is itself 8.3–8.5x rho-neg (session `ECBS1h405f20219036`).
A factor of about 1.5 at a non-optimal cell changes no ordering across the §5a band. **Rho remains ahead.**

## Deviations

- **F2 triggered on 9 cells** (block B, `bits` at k = 16, 18, 22, all three seeds). Each got its
  one identical rerun, as frozen. Both attempts panicked (rc 101) on a factor-base assertion
  (`abscissa_list ... < span`). As the protocol requires, they are reported as failed and their
  pairs are dropped. P3 has a denominator of 1 curve instead of 4. No replacement runs were
  made (the protocol forbids extra runs after data are seen, and the binary must not be
  rebuilt). Testing `bits` at those k, e.g. with an explicit `bits@0` form or a binary fix, needs
  a new protocol.
- The T3 lead's `f4cost` process ran concurrently on the same host (also noted in
  Instrument). Within this lead the runs were sequential; counts are deterministic.
- **bits-arm panic cause (F2), added context.** The `bits` form panic at k = 16, 18, 22 was a
  bug in `curve25519_family`'s bits arm: `prime_factor_base` scans from x = 1 while the count
  included x = 0, which carries a point on those curves. It is fixed in the code now, but not in
  the binary used for this run. P3 therefore stays inconclusive as frozen, and a re-run with the
  fixed binary (new protocol) is a follow-up.
- **P0 failed (binary drift).** As the protocol requires, the analysis continued on the new binary.
  The replication ratio is 0.6386, against 0.646 on lead 5's binary.
- `scripts/analyze.py` and `analysis/summary.json` were written after the freeze. They only apply
  the frozen rules to the data. The P6(c) gate ("judged only if (b) holds") is applied as frozen.
- No git operations were run. All files were written inside this lead directory.

## Validator errata (applied)

Source: `validator-report.json`. No prediction verdict changed.

- E1: Block B note added for the k = 18, S2 Edwards cell (hits = 1, 1006 steps over 100
  targets, the 81,344,125.24 maximum); the "10 F4 steps per Edwards target" wording scoped to
  Blocks A and E.
- E2: process counts corrected: 151 cells, 160 `f4cost` invocations (142 exit 0, 18 exit 101).
- E3: load average corrected to about 2 to 7 (2.04 to 6.89 in `data/*.status`).
- E4: Block C G = 3 median printed as 1.000000 (0.99999958), not 0.999999. P4 unaffected.
- E5: Exploratory 4 reworded to state its unit and the w = 0 tie (F4 mul-adds are unpriced
  native work in ecbench accounting). Within-method conclusion unchanged.
- E6: Headline exploratory reading and Exploratory 1 and 6 scoped to the cells measured.
- E7: concurrent T3 run now also listed under Deviations. No number affected.
- Additional context (F2 cause): the bits-form panic at k = 16, 18, 22 was a `prime_factor_base`
  bug in curve25519_family's bits arm (scan from x = 1, count included x = 0); fixed in code,
  not in the binary used. P3 stays inconclusive as frozen; a re-run is a follow-up.
