# T6: does the Edwards t = 5, G = 0 per-target F4 cost of 0.646x replicate?

Date 2026-11-03. Wave 2, lead T6. Status: **frozen before data**. No measurement
has been taken for this lead. The only inputs read while writing this were
existing files: `research/curve25519_edwards_semaev_20261102` (README, `cells/*.json`,
`run_cells.sh`) and `research/curve25519_frame_search_20261103` (README,
`analysis/summary.json`).

## Question

`curve25519_edwards_semaev_20261102` measured, at k = 20, m = 2, t = 5, G = 0,
100 targets, target seed 2026111505:

| form | cols | covered | deg | terms | mul-adds/target |
|---|--:|--:|--:|--:|--:|
| bits (lo = 0) | 11 | 42 | 6 | 256 | 127,284,344.31 |
| edwards-bits | 14 | 58 | 6 | 256 | 82,230,218.94 |

That is a ratio of 0.646 in F4 mul-adds per target. The other 7 (t, G) pairs of that
study were within about 1.1% of 1. At G = 3 and the same t the ratio was 1.000.
The frame study showed that a translation frame `bits@976503` (24 cols) costs
127,284,307.74 per target at the same cell, so per-target cost does not move with
the frame or the column count under translation. Nothing explained the Edwards value.

T6 asks:

1. Does the per-target ratio R = mpt(edwards-bits) / mpt(bits) stay well below 1 over
   fresh target seeds at k = 20, t = 5, G = 0, or was it specific to seed 2026111505?
2. Does it also hold on the other family curves (k = 16, 18, 22, 24)?
3. Where does it go away: in G (0 to 3) and in t (4, 5, 6)?
4. If it replicates, is it a uniform per-target saving or a mixture of cheap and normal
   targets, and do the cheap targets solve at a lower degree?

Here mpt is the `muladds_per_target` field of `f4cost`, which counts F4 multiply-adds.
No timing is used anywhere in this lead.

## Instrument

- Binary `/home/user/ctarget/release/curve25519_family`, sha256
  `7a438b1d8ce9d7a0b8049a4331599727e9574ea4201f9c7957befa3184ba98be`, read
  2026-11-03. Do not rebuild or edit it. This is **not** the binary of either earlier study
  (lead 5 used `ec9cb40d…5f`, the frame study used `0459a725…b4c5`). Block A therefore
  starts with a reproduction check (P0). `run.sh` refuses to start if the hash differs (F1).
- Command per cell:
  `curve25519_family f4cost <k> <form> <t> <G> <targets> <seed> 3600 2 full 0 30000000`
  (m = 2, mode full, lifts 0, max_nnz 30,000,000, wall cap 3600 s so that no cell is
  truncated). The wrapper adds `timeout 4000` and `ulimit -v 3500000`.
- Strictly sequential, one process at a time, on the shared 4-core host. Within a seed the
  form order is interleaved (bits, edwards-bits, translation control). Because the counts are
  deterministic, order and load cannot change a count. Load can only trip the cap (F3).

## Curves (family rule)

k = 16 A = 1058; k = 18 A = 78; k = 20 A = 926 (p = 1048573, r = 131203);
k = 22 A = 43186; k = 24 A = 318. The binary derives each curve from k.

## Frame choice (computed now from existing census data)

The translation control is the frame-study selected frame for k = 20, t = 5,
from `curve25519_frame_search_20261103/analysis/summary.json`:
L = 976503 (24 cols, covered 198; for comparison bits lo = 0 has 11 cols and covers 42,
edwards covers 58). Edwards y in [0, 2^t) is the only Edwards frame the CLI has.
The selected frames for the other k (16: 61492, 18: 133370, 22: 3678753, 24: 12304347 at
t = 5) are listed for reference only. They are not run, because at k = 20 the
translation control already pins the instrument.

## Seeds

- Reproduction: 2026111505, the lead-5 seed (2026111500 + t).
- Fresh: S1–S6 = 3110300001 … 3110300006. Blocks B, C and D use S1–S3.
- Per-target (block E): 3110301001 … 3110301040, one target each.
- The fresh seeds avoid the `2026111500+t` scheme of the earlier studies and the
  `20261104+k` scheme of the torsion leads. They are below 2^32.

## Design (exactly what `scripts/run.sh` runs, 151 processes)

| block | k | forms | t | G | targets | seeds | cells |
|---|---|---|---|---|--:|---|--:|
| A | 20 | bits, edwards-bits, bits@976503 | 5 | 0 | 100 | 2026111505, S1–S6 | 21 |
| B | 16, 18, 22, 24 | bits, edwards-bits | 5 | 0 | 100 | S1–S3 | 24 |
| C | 20 | bits, edwards-bits | 5 | 1, 2, 3 | 100 | S1–S3 | 18 |
| D | 20 | bits, edwards-bits | 4 | 0 | 100 | S1–S3 | 6 |
| D | 20 | bits, edwards-bits | 6 | 0 | 100 | S1 | 2 |
| E | 20 | bits, edwards-bits | 5 | 0 | 1 | 3110301001–040 | 80 |

Output goes to `data/<block>_k<k>_<form>_t<t>_g<G>[_n1]_s<seed>.{json,status,stderr}`,
plus `data/binary.sha256` and `data/ALL_DONE`. Expected wall time is about
15–30 min at the earlier rates (about 0.1 s per target at t = 5, G = 0, and about 0.9 s per
target at t = 6, G = 0), and more on a loaded host.

Ratios are always paired: same k, t, G, seed and target count, so both forms see
the same target points. Statistic per pair: R = mpt(edwards-bits) / mpt(bits).
A cell enters a ratio only if `targets_done == targets_requested`, `capped == false`
and `undecided == 0` (otherwise F3).

## Predictions (numbered, with decision rules, fixed before data)

- **P0 (reproduction, new binary).** At seed 2026111505 the new binary reproduces
  mpt(bits) = 127,284,344.31 and mpt(edwards-bits) = 82,230,218.94, and also
  mpt(bits@976503) = 127,284,307.74, each within a relative difference of 1e-3.
  *Rule:* all three within 1e-3 gives **reproduced**. Otherwise **binary drift**: report
  the new values, keep analysing on the new binary, and flag every comparison with the
  old studies.
- **P1 (main: the outlier replicates at k = 20).** Prediction: **replicates**.
  *Rule:* over the 6 fresh seeds S1–S6 at k = 20, t = 5, G = 0:
  **replicates** if R ≤ 0.90 in at least 5 of 6 seeds;
  **vanishes** if 0.97 ≤ R ≤ 1.03 in at least 5 of 6 seeds;
  **mixed / inconclusive** otherwise.
  P1 holds only under "replicates". "Vanishes" is reported as P1 failed, and the
  0.646 is then recorded as a seed-specific value.
- **P2 (instrument control: translation costs the same).** At k = 20, t = 5, G = 0,
  0.99 ≤ mpt(bits@976503) / mpt(bits) ≤ 1.01 in all 7 seeds.
  *Rule:* holds if 7/7. Any seed outside the band means the per-target count is
  frame-sensitive on this binary. P1 is then read against that band, and the result says so.
- **P3 (the outlier holds on the other curves).** Per k in {16, 18, 22, 24}, take the
  median R over S1–S3. Prediction: median R ≤ 0.90 for at least 3 of the 4 curves.
  *Rule:* holds if ≥ 3/4. **k-specific** if at most 1/4 are ≤ 0.90 and the rest are in
  [0.97, 1.03]. Mixed otherwise.
- **P4 (guessing removes it).** At k = 20, t = 5: for G = 3, 0.98 ≤ R ≤ 1.02 for all of
  S1–S3 (lead 5 had 1.000). The median R over S1–S3 does not decrease as G increases,
  which means median R(G=0) ≤ R(G=1) ≤ R(G=2) ≤ R(G=3), with 0.01 slack per step.
  *Rule:* holds if both parts hold.
- **P5 (only t = 5).** At k = 20, G = 0: 0.97 ≤ R ≤ 1.03 at t = 4 for all of S1–S3,
  and at t = 6 for S1 (lead 5 had 0.997 and 0.990). *Rule:* holds if 4/4.
- **P6 (per-target shape, block E; applies only if P1 replicates).**
  (a) bits is near-constant per target: the coefficient of variation of the 40
  single-target mpt values for bits is ≤ 0.02.
  (b) Prediction: Edwards is a **mixture**. At least 20% of the 40 Edwards targets
  have mpt ≤ 0.8 × median bits mpt (call them "cheap"), and at least 20% are within ±5%
  of the median bits mpt.
  The alternative **uniform** is Edwards CV ≤ 0.05 with median Edwards / median bits ≤ 0.90.
  (c) Of the cheap Edwards targets, at least 80% report `solving_degree_max` ≤ 5
  (lower degree than the 6 of bits).
  *Rule:* each of (a), (b), (c) is judged on its own. (c) is judged only if (b) holds.
  If P1 does not replicate, P6(b) and P6(c) are reported as not applicable.

## Analysis that is exploratory (not preregistered)

Everything below is labelled exploratory in the report:

- Any mechanism reading beyond P6(c). This covers comparisons of `calls_per_target`,
  `poly_terms_per_specialisation`, `native.pkm_system_fp_mults_uncharged`,
  `native.pkm_system_terms`, `undecided` and `hits` between forms.
- Any relation between the cheap Edwards targets and properties of the target (coset,
  residuosity). It is computed only from fields the JSON already prints.
- Per-relation cost (mpt / `coverage_exact`). This is reported, but it is driven by
  coverage, which the frame study already explains, so it is not part of the T6 question.

No extra runs are added after the data are seen. A follow-up runs under a new protocol.

## Failure handling

- **F1:** the binary hash differs from the frozen value. `run.sh` exits before any cell
  and nothing is measured. Re-freeze under an amendment written before data.
- **F2:** a process exits non-zero. It gets one identical rerun (the first attempt is kept
  as `*.try1.*`). If that also fails, the cell is reported as failed and excluded, and its
  pair is dropped from the counts in P1–P5. The denominators shrink and are stated.
- **F3:** `capped`, `targets_done < targets_requested` or `undecided > 0` excludes the
  cell from ratios, and it is reported. The cap is not raised and nothing is rerun with
  other parameters.
- Failed predictions are reported as failed. Thresholds, seeds and cells are not retuned.
  Any deviation from this protocol goes into a Deviations section.

## Scope and distance remaining

- Scope: Curve25519-family miniatures k = 16–24, m = 2, t ∈ {4, 5, 6}, G ∈ {0–3}, the
  Edwards-y window [0, 2^t), and counted F4 mul-adds per target. Nothing is said about
  m = 3, other windows, or 255 bits.
- Even a fully replicated 0.646 is a constant factor on the **algebraic** (pkm-semaev)
  track, at one (t, G) cell, and only at G = 0. At this t the best measured cell is G = 3,
  where lead 5 found no Edwards effect. A 0.646 factor at G = 0, t = 5 therefore does not
  improve the best algebraic configuration unless it carries over. It is not an
  improvement over the index-calculus baseline unless a later end-to-end measurement
  shows one.
- Distance remaining: the best algebraic arm measured
  18x (w = 0), 9.6·10^5x (w = 0.1) and 5.9·10^6x (w = 1) behind rho-neg at k = 20
  (`curve25519_algebraic_ecbench_20261102`). The generic baseline is itself 7.8x rho.
  A factor of about 1.5 changes no ordering across the §5a band. **Rho remains ahead.**
