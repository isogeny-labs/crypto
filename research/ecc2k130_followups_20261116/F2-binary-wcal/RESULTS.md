# F2: GF(2^n) w_cal instrument — RESULTS (2026-11-16)

Every number here is `[pcc:unverified]`: a local, shared-VM, best-of-N / median-of-rounds
timing, labelled **provisional**. No claim is signed, replayed on a second host, or run on
isolab-eks. Frozen protocol: `PROTOCOL.md` (sha256 `38cab9bf…` in `raw/PROTOCOL.frozen.sha256`,
frozen before any measured run; Amendment 1 appended after the data, description only).
Full tables: `tables.md`. Machine-readable: `results.json`. Figures: `fig_prices_vs_n.svg`,
`fig_l2_breakeven.svg`.

## Summary

- **Instrument.** `binwcal` (Rust, lane crate in `src/`) prices one native op against one
  affine group addition (1 GAE) on the same binary Koblitz curve, in the same process. The GA is
  the one ecbench's binary rows count: `FastCurve::add` for n ≤ 62 and `WideStrongRho::add` for
  64 ≤ n ≤ 127. At n = 131, where ecbench has no instance, it is
  `certicom131_symlp::BinaryW::add`. Rounds are paired and interleaved, with 3 processes per
  cell, and every run used core 3 only. [pcc:unverified]
- **Controls.** All 11 (n, impl) cells pass A/A, both positive controls and the negative
  control. The drift control passes and the count-exactness check passes (0/1800 mismatches
  against L2's committed counts). [pcc:unverified]
- **Prices (n = 11 … 47, ecbench `small`).** A field multiplication costs 0.057 → 0.021 GAE
  (latency) or 0.018 → 0.0066 GAE (throughput). A squaring costs about the same. An inversion
  costs 0.74 → 0.90 GAE: the GA is inversion-bound. A 64-bit word XOR in a row XOR costs
  3.1e-3 → 1.2e-3 GAE at W = 64 words, and 5e-2 → 1.9e-2 GAE at W = 1. Every price falls
  as n^-0.69, because the GA's time grows as n^0.69 while the cost of a mul or an XOR does not
  depend on n in one word. [pcc:unverified]
- **n = 83 (ecbench `wide`).** mul 0.055 GAE, word XOR 4.6e-4 GAE (W = 64).
  `WideGf2::inv` costs 4.8 GAE, because the GA inverts by a faster private normal-basis
  routine. [pcc:unverified]
- **n = 131 (`w3`).** mul 7.2e-3 and word XOR 3.0e-5 GAE (W = 64), against a GA that
  `Gf2W::inv` dominates (0.96 GAE). The same `w3` GA is 13.4× slower than ecbench's `wide` GA
  at n = 83. Rescaled by that factor, a labelled extrapolation gives mul about 0.096 GAE and a
  word XOR about 4.0e-4 GAE against an ecbench-quality addition. [pcc:unverified]
- **L2's m = 2 trade is settled on the losing side.** Every m = 2 cell loses at the measured
  word-op price, from 37× above break-even (n = 11, D = 2) to 8.7e4× (n = 19, D = 4). The §5a
  flag "ranked at calibrated price" is still false, because the filter wins at w = 0.
  [pcc:unverified]
- **L4's G-vs-I order stays unresolved by the frozen rule.** In the m = 2 regime cells the
  measured range still straddles 1. The point estimate (latency mul, W = 64) gives I/G = 0.51–0.56,
  so I is cheaper by about 2×. At throughput mul the same cells read 1.46–1.63, so G is cheaper.
  The order now turns on whether G's multiplications run latency-bound or throughput-bound, not
  on the P1/P4/P2n band. Resolved, as in L4: G is cheaper at 13:5:3 and 19:6:3, and I is cheaper
  at 29:6, 29:8 and 29:10. [pcc:unverified]

## 1. Prior art (grep audit; research-kb and the lead atlas were not queried; no novelty claim)

`ecbench price-units` (`src/cryptanalysis/ecbench/full_price.rs`, `full_prices.json`, measured
2026-10-14 on another 4-vCPU Xeon) already priced `field_mul`, `field_sqr`, `field_inv`,
`word_xor` and `double` against an affine addition for Koblitz n = 13 … 41. L7's statement
"`ecbench wcal` covers prime fields only" is true of `ecbench wcal`, but the binary *table*
existed. What this lane adds: intervals, A/A, positive and negative controls, n = 11, 47, 83
and 131, row-width dependence of the word XOR, latency vs throughput, L2's own kernel priced
all-in, and the application to L2 and L4. Agreement with `full_prices.json`: our mul, sqr, inv
and double prices are 0.96–1.11× theirs at n = 13 … 41; our `xor_w4096` price is 0.67–0.74× theirs.
That difference is a host/codegen difference in a streaming XOR. [pcc:unverified]

## 2. Prices (point; 95% interval is the union over 3 processes; see `tables.md` Table 1)

| n | impl | GA ns | mul lat | mul tp | sqr lat | inv | word XOR W=1 | W=16 | W=64 | W=4096 | null (floor) |
|--:|--|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 11 | small | 73.3 | 5.73e-2 | 1.78e-2 | 6.06e-2 | 0.736 | 5.03e-2 | 5.58e-3 | 3.14e-3 | 2.60e-3 | 1.79e-3 |
| 13 | small | 79.1 | 5.31e-2 | 1.65e-2 | 5.60e-2 | 0.756 | 4.67e-2 | 5.17e-3 | 2.93e-3 | 2.41e-3 | 1.66e-3 |
| 19 | small | 102.2 | 4.09e-2 | 1.28e-2 | 4.33e-2 | 0.809 | 3.61e-2 | 3.99e-3 | 2.24e-3 | 1.86e-3 | 1.29e-3 |
| 23 | small | 119.2 | 3.51e-2 | 1.09e-2 | 3.71e-2 | 0.834 | 3.08e-2 | 3.42e-3 | 1.93e-3 | 1.60e-3 | 1.10e-3 |
| 29 | small | 136.4 | 3.07e-2 | 9.57e-3 | 3.25e-2 | 0.859 | 2.70e-2 | 3.00e-3 | 1.69e-3 | 1.40e-3 | 9.61e-4 |
| 37 | small | 160.1 | 2.62e-2 | 8.21e-3 | 2.77e-2 | 0.877 | 2.35e-2 | 2.56e-3 | 1.45e-3 | 1.23e-3 | 8.30e-4 |
| 41 | small | 171.7 | 2.45e-2 | 7.63e-3 | 2.59e-2 | 0.885 | 2.22e-2 | 2.39e-3 | 1.35e-3 | 1.13e-3 | 7.85e-4 |
| 47 | small | 199.6 | 2.11e-2 | 6.56e-3 | 2.22e-2 | 0.902 | 1.87e-2 | 2.06e-3 | 1.16e-3 | 9.70e-4 | 7.40e-4 |
| 83 | wide | 509.2 | 5.46e-2 | 5.19e-2 | 5.71e-2 | 4.85 † | 7.28e-3 | 8.09e-4 | 4.59e-4 | 3.76e-4 | 2.85e-4 |
| 83 | w3 | 6818.9 | 1.08e-2 | 1.17e-2 | 1.10e-2 | 0.953 | 5.36e-4 | 5.97e-5 | 3.37e-5 | 2.80e-5 | 2.02e-5 |
| 131 | w3 | 7721.9 | 7.19e-3 | 9.22e-3 | 7.30e-3 | 0.965 | 4.80e-4 | 5.28e-5 | 3.01e-5 | 2.47e-5 | 1.72e-5 |
| 131 | w3 × 13.39 (extrapolation) | — | 9.6e-2 | 1.2e-1 | 9.8e-2 | — | 6.4e-3 | 7.1e-4 | 4.0e-4 | 3.3e-4 | 2.3e-4 |

† `WideGf2::inv`, not the GA's own inversion (PROTOCOL §4 limits).

The 95% intervals are narrow: within ±2% of the point for every op at n ≤ 47, and within ±5%
for the `wide` and `w3` rows. The between-process spread is the dominant term
(`tables.md` Table 1). Best-of-N is up to 26% below the median reading on short-row XORs (W = 2 … 16, worst W = 3), and within a few percent elsewhere; the median reading is the frozen point.
[pcc:unverified]

**Floor flag (frozen rule §5).** By the frozen rule, a price within 2× of the `null` loop is
"at the floor". The word XOR at W ≥ 64 is at the floor at every n: for example 1.69e-3 against
9.6e-4 at n = 29. The `fadd` op is not. For such ops the instrument resolves only an upper
bound near the cost of one loop iteration (about 0.13 ns). They are not lower bounds: nothing
is subtracted. [pcc:unverified]

**n-scaling.** In the one-word family the GA's time grows from 73 ns to 200 ns between n = 11
and 47, a log-log slope of 0.69. The `mul_lat` (4.2 ns) and `xor_w64` (0.23 ns per word) times
are flat in n, so their prices fall as n^-0.69. Beyond one word the scaling depends on the
implementation, not on n: at n = 83 the ecbench GA takes 509 ns, while the `w3` GA takes
6.8 µs at n = 83 and 7.7 µs at n = 131. [pcc:unverified]

## 3. Controls (`tables.md` Table 2)

| control | rule | result |
|---|---|---|
| A/A `ga_aa/ga` | union of 95% intervals inside [0.95, 1.05] | 11/11 pass; widest [0.989, 1.017]; relative noise floor ≤ 1.7% |
| PC1 `ga4/ga` | point in [3.6, 4.4] | 11/11 pass (3.99–4.02) |
| PC2 `mul8_lat/mul_lat` | point in [7.2, 8.8] | 11/11 pass (7.97–8.37) |
| NC `null/ga` | upper < 0.01 | 11/11 pass (max 2.1e-3) |
| Drift, ns/word at W = 64 | max/min ≤ 1.5 over 33 processes | 1.06, pass |
| linz count exactness | equal to L2's committed counts | 1800/1800 equal |
| External (full_prices.json, other host) | descriptive | mul/sqr/inv/double 0.96–1.11×; xor 0.67–0.74× |

All results are [pcc:unverified].

## 4. Application 1: L2's m = 2 linearisation filter (provisional local calibration)

This applies a provisional local calibration to L2's committed counts. L2's break-even per
cell is net GAE at price 0 divided by mean word ops. The measured price interval is the union of
the pure row-XOR price at the cell's row width and the price of L2's own `macaulay` kernel
timed all-in on the same 200 targets (counts identical). The table is `tables.md` Table 3 and
the figure is `fig_l2_breakeven.svg`.

| cell n/d/D | break-even (GAE/word op) | measured price low end | ratio |
|---|--:|--:|--:|
| 11/5/2 | 1.33e-3 | 4.9e-2 | 37× |
| 13/6/2 | 4.51e-4 | 2.9e-2 | 63× |
| 11/5/3 | 1.73e-4 | 2.4e-2 | 138× |
| 13/6/3 | 1.16e-5 | 1.3e-2 | 1.1e3× |
| 11/5/4 | 5.13e-6 | 1.2e-2 | 2.4e3× |
| 13/6/4 | 1.18e-6 | 6.5e-3 | 5.5e3× |
| 11/6/4 | 4.85e-7 | 7.0e-3 | 1.4e4× |
| 13/7/4 | 1.17e-7 | 4.4e-3 | 3.7e4× |
| 19/9/4 | 2.57e-8 | 2.2e-3 | 8.7e4× |

The other nine m = 2 cells refute nothing (net 0) and lose at every positive price.
[pcc:unverified]

- **Reading:** the filter loses on every m = 2 cell at the measured price. L2's lane left this
  as an INCONCLUSIVE trade. At this provisional price it resolves to a loss, with a margin of at
  least 37×, far outside the timing's ±5% interval. [pcc:unverified]
- **§5a:** "ranked at calibrated price" is false. The filter is ahead at w = 0, behind at
  `w_cal` and behind at `10·w_cal`, so the three orders differ. The band verdict (L2's) is
  unchanged. The new statement is narrower: the break-even lies 37× to 8.7e4× below the
  measured price. [pcc:unverified]
- **The kernel all-in price is higher still.** Per counted word op, L2's `macaulay` costs
  0.56–0.65 ns at D = 4 and n = 19, against 0.23 ns for a bare row XOR. At D = 2 the fixed setup
  dominates (69–500 ns per counted word op, 0.7–6.9 GAE). [pcc:unverified]
- **n = 131 (L2's unmeasured assumption, "≥ 1e-3 GAE per word op").** At W ≥ 64 the measured
  price is 3.0e-5 GAE in the `w3` unit, or about 4e-4 GAE extrapolated to an ecbench-quality GA.
  The assumption as stated does not hold for wide rows. L2's conclusion would only flip at
  n = 131 if the D = 4 break-even rose from ≤ 5e-6 (toy n, falling with n) to above about
  3e-5–4e-4. That would take a 6× to 80× increase against the observed trend. This is an
  extrapolation; no word-op counts exist at n = 131. [pcc:unverified]

## 5. Application 2: L4's G (gcd with L_V) vs I (`inherited-f4`) (provisional local calibration)

G's field operations are priced in word XORs at the measured ratios at each n, with I's
native word XORs as given. The table is `tables.md` Table 4, and Table 4b gives the corners.

| cell n:l:m | L4 band I/G (P1/P4/P2n) | measured point (lat, W64) | tp, W64 | frozen range | frozen reading |
|---|---|--:|--:|---|---|
| 11:6:2 | 6.48 / 2.40 / 0.55 | 0.51 | 1.46 | [0.35, 5.48] | unresolved |
| 13:7:2 | 7.00 / 2.60 / 0.51 | 0.56 | 1.63 | [0.38, 6.01] | unresolved |
| 19:10:2 | 6.69 / 2.49 / 0.34 | 0.53 | 1.57 | [0.36, 5.81] | unresolved |
| 29:15:2 | 5.90 / 2.20 / 0.20 | 0.46 | 1.38 | [0.32, 5.11] | unresolved |
| 29:12:2, 29:14:2 | ≤ 3.24 | 0.18, 0.26 | 0.52, 0.76 | straddles 1 | unresolved |
| 29:6, 29:8, 29:10 | < 1 | 0.002–0.075 | ≤ 0.22 | < 1 | I cheaper (as in L4) |
| 11:4:3, 13:4:3 | 12.6 / 4.6 / 0.98; 12.3 / 4.4 / 0.81 | 1.07, 1.04 | 3.14, 3.06 | straddles 1 | unresolved |
| 13:5:3 | 75.5 / 27.5 / 5.0 | 6.4 | 19.0 | [4.3, 67] | G cheaper (as in L4) |
| 19:6:3 | I at budget | ≥ 12.8 | ≥ 37.8 | ≥ 8.5 | G cheaper (I a lower bound) |

All entries are [pcc:unverified].

- The measured multiplication price is 18.2–18.3 word XORs per latency-bound mul at W = 64,
  and 5.7 per throughput-bound mul. Both lie between L4's P4 (4) and P2n (2n = 22–58).
  [pcc:unverified]
- **Reading:** the frozen rule resolves no cell that L4 left open. What changed is the
  reason. The order of the m = 2 regime cells flips between the latency and throughput
  variants (I/G about 0.5 against about 1.5). The row width of I's XORs (W64 against W4096)
  moves it by only 17%. The open question is therefore how G's multiplications pipeline.
  Settling it takes a timed G next to a timed I, or L4's open thread: G ported into
  `ic_framework` on the shared `bitop/v1` tally. [pcc:unverified]
- Against the IC baseline's own oracle (L4's S), G stays 7–14× (m = 2) and 300–1500× (m = 3)
  more expensive. No price in this lane changes L4's verdict "no improvement of the IC
  baseline". [pcc:unverified]

## 6. Deviations

- **HEAD does not compile.** Merge `16aa66283b` left `lift_x_packed` returning a `Vec` in
  `src/cryptanalysis/koblitz_index_calculus.rs`. The lane built against a scratch copy of HEAD
  `c3655407dc` with the two-line fix in `crypto-head-build-fix.patch`. The shared file was not
  edited. The main session should land this fix separately.
- K_0 has no usable prime-order subgroup in ecbench's constructor at n = 11 and 29, so a = 1
  was used there (fixed before the freeze).
- L2's raw linz files record seeds `20261115000 + …`, not the `20261114000 + …` in L2's
  `run_linz.sh`. The recorded seeds were used, and the count check confirms them.
- L2's harness is vendored as `src/l2v/l2_main_verbatim.rs`. It is identical to L2's
  `src/main.rs` except that lines 1–7 have `//!` changed to `//`, which `include!` requires.
  `analyze.rs` is byte-identical.
- Amendment 1 (after the data) adds Table 4b only. The analysis binary was rebuilt for it
  (`raw/ANALYZE_BINARY.sha256`); the measurement binary (`raw/BINARY.sha256`) is unchanged.
- File edits to source text were made with small shell or python one-off text substitutions.
  All experiment, harness and result-generation code is Rust.
- No PDF: no TeX or typst engine is installed in this container.

## 7. What this does not establish, and the authoritative task card

This is a provisional calibration on a shared 4-vCPU VM. Core 3 is reserved by lane convention
only: there is no isolcpus, no governor control and no hardware counters, and cores 0–2 ran
other lanes' work. The load average was 2.7–6.2 during the run. The A/A floor (≤ 1.7%) and the
drift control (1.06) say the ratios were stable here. They say nothing about another
microarchitecture. The W = 1 to W = 4096 spread (about 16×) and the latency/throughput spread
(about 3.2×) are properties of the op definition, not noise. A consumer must pick the variant
that matches its kernel.

**Task card for the authoritative w_cal (isolab-eks; not launched: no recorded live budget
on 2026-11-15/16):**

- Route: `isolab-eks`, x86 CPU, L2 admission (native L2 / replay / A/A), on-demand or Spot
  with no resume: each process runs about 5 s.
- Source: this lane's `binwcal` at the commit that lands it, built against a HEAD that compiles
  (or with `crypto-head-build-fix.patch`), with the image digest and binary sha256 recorded.
- Command: `run.sh` unchanged (11 (n, impl) cells × 3 reps, `--rounds 101 --warmup 10
  --seed 20261116`, plus the 6 linz cells) on one isolated, frequency-pinned core, under
  `perf stat -e cycles,instructions` per process (the `perf-ipc` skill). The run is repeated
  on a second instance type for the cross-host spread.
- Outputs: `raw/` with SHA256SUMS to S3, a `bench.record/v1` record, `binwcal analyze`, and
  the band input `WCal { w_cal, method: "perf-ipc" or "best-of-N timing", host }` per counter
  class.
- Decision use: the same §7 readings. A row reaches `ecbench band` as a `w_cal` only with
  method and host recorded (AGENTS.md §5a, 2026-11-12).
- Open design item: ecbench has no n = 131 instance, so the `w3` unit (`Gf2W::inv`-bound) is
  13.4× slower than ecbench's `wide` GA at n = 83. A 3-word ecbench `Instance` with the
  normal-basis inversion would remove the extrapolation.

## Files

`PROTOCOL.md`, `RESULTS.md`, `tables.md`, `results.json`, `fig_prices_vs_n.svg`,
`fig_l2_breakeven.svg`, `run.sh`, `Cargo.toml`, `Cargo.lock`, `src/main.rs`, `src/analyze.rs`,
`src/l2v/` (vendored L2 harness), `crypto-head-build-fix.patch`, and `raw/` (`measure/*.json`
with 33 processes, `linz/*.json` with 6 cells, logs, `PROTOCOL.frozen.*`, `BINARY.sha256`,
`ANALYZE_BINARY.sha256`, `SOURCE.sha256`, `RUN_START/END`, `run.log`, `SHA256SUMS`).
Binaries (not in the repo): `$SP/lanes/F2/binwcal.frozen` and `$SP/lanes/F2/binwcal.analyze`;
the scratch library copy is `$SP/lanes/F2/crypto-src`.
