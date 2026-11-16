# F2: a GF(2^n) w_cal instrument (PROTOCOL, frozen before any measured run)

Lane F2 of `research/ecc2k130_followups_20261116/` (rules: `../COMMON.md`). Every claim in this
lane is `[pcc:unverified]`. Date: 2026-11-16.

## 1. Question

What does one unpriced native operation of a binary index-calculus arm cost in GAE, where
1 GAE is one affine group addition on the same binary Koblitz curve, host and process? The
operations: a 64-bit word XOR in a row XOR (L2's and `inherited-f4`'s word op), a field
multiplication, squaring, inversion and addition in GF(2^n), and a word op of L2's own Macaulay
kernel counted all-in. The answer can replace or bracket the §5a band (0, 0.1, 1 GAE per
operation) for these counters. It is applied to two decisions the 2026-11-14 sweep left open
(L2's m = 2 trade, L4's G-vs-I order).

## 2. Prior art (audit by repository grep; research-kb and the lead atlas were not queried)

- `crypto:src/cryptanalysis/ecbench/wcal.rs` (`ecbench wcal`): prime fields only; best-of-N
  timing of a mulmod against an affine addition with its inversion; label `best-of-N timing`.
  AGENTS.md §5a (2026-11-12 bullet) defines `w_cal` = time of one native op / time of one group
  addition on the same curve and host, and asks for the order at `w = 0`, `w_cal` and `10·w_cal`.
- `crypto:src/cryptanalysis/ecbench/full_price.rs` with `full_prices.json` (`ecbench
  price-units`): a single-host table, measured 2026-10-14, of ratios to one affine addition of
  `field_mul`, `field_sqr`, `field_inv`, `word_xor`, `row_op`, `double` and others, for Koblitz
  n = 13, 17, 19, 23, 29, 31, 37, 41 (one-word field only). It is the closest prior instrument.
  It has no interval, no A/A, no positive or negative control, no n = 11, 47, 83 or 131, one row
  width for `word_xor` (4096 words), and it was not applied to L2 or L4. This lane extends it and
  uses it as an external cross-check (different host).
- `crypto:src/cryptanalysis/ecbench/wordram_price.rs`: written-down word-RAM budgets, not a
  measurement.
- `crypto:research/band_wcal_20261113/NOTE.md`: how `w_cal` enters `ecbench band`.
- Sweep evidence re-read here (not re-run): `research/ecc2k130_open7_20261114/L2-refutation-filters/results.json`
  and `raw/linz_*.json`; `research/ecc2k130_open7_20261114/L4-baselines/results.json` and `raw/g-K-*.jsonl`.

No novelty claim is made.

## 3. Instrument

A lane crate `binwcal` (Rust, `src/`), depending on the repository library (`crypto_lib`) built
from a scratch copy of HEAD `c3655407dc`. HEAD does not compile (a merge left
`lift_x_packed` returning a `Vec`); `crypto-head-build-fix.patch` is the two-line fix applied to
the scratch copy only. No shared file is edited.

GAE unit, per n, the group addition the binary rows count:
- n ≤ 62: ecbench `Instance::Binary` (`CurveSpec::Koblitz { a, n }`), `FastCurve::add`, field
  `semaev_decomp::Gf2` (pclmulqdq fold). Impl label `small`.
- 64 ≤ n ≤ 127: ecbench `Instance::Wide`, `WideStrongRho::add`, field `WideGf2`. Label `wide`.
- n = 131, and n = 83 as a cross-implementation check: `certicom131_symlp::BinaryW::add` on
  `Gf2W` (three words), curve y² + xy = x³ + a x² + 1 with the registry modulus
  (131: z^131 + z^13 + z^2 + z + 1). Label `w3`. ecbench has no n = 131 instance; this is the
  repository's own 131-bit binary group law.

Ops timed per process (`binwcal measure`):
`ga` (affine addition, chained over 64 fixed points as `ic_boundary::calibrate_group` does),
`ga_aa` (an identical copy, A/A), `ga4` (4 chained additions per op, positive control), `dbl`,
`mul_lat` (dependent chain), `mul8_lat` (8 chained muls per op, positive control), `mul_tp`
(4 independent chains), `sqr_lat`, `sqr_tp`, `inv_lat`, `fadd` (field addition), `null`
(empty loop with `black_box`, negative control), and `xor_wW` = row XOR `a ^= b` of W words
written as L2's `bxor`, over a pool of rows, for W ∈ {1, 2, 3, 4, 5, 7, 8, 13, 16, 22, 23, 64,
97, 256, 4096} (L2's Macaulay row widths plus 256 and 4096); priced per word.

`binwcal linz`: L2's harness is included verbatim (`src/l2v/l2_main_verbatim.rs`, sha256
`f16bda7a…` = L2's committed `src/main.rs`). It rebuilds L2's m = 2 target streams with the seed
each raw file records, builds each descent system outside the timer, and times L2's own
`macaulay` at D = 2, 3, 4. The price per word op is then total ns / total counted word ops on
those calls. This "all-in" price includes the row construction, pivot scans and allocation
that L2's word-op counter leaves out.

## 4. Design

- n ∈ {11, 13, 19, 23, 29, 37, 41, 47} (`small`; a = 0 except n = 11, 29 and 47, where a = 1:
  ecbench's constructor finds no usable prime-order subgroup for K_0 at n = 11 and 29, and n = 47
  has only K_1 registered), 83 (`wide` a = 0, and `w3` a = 0), 131 (`w3` a = 0). The curve
  coefficient a changes one field addition in the group law.
- Per process: every op is sized so one sample takes about 1 ms; 10 warm-up rounds are
  discarded; then 101 measured rounds. Within each round every op runs once, in a fresh
  seeded random order (interleaved arms). A per-round paired ratio `t_op / t_ga` is formed.
- 3 processes (reps 0, 1, 2) per (n, impl). Process order: rep-major; within a rep the 11
  (n, impl) cells run in a seeded shuffled order. Seeds: `--seed 20261116`, the process RNG is
  derived from (seed, n, rep, a).
- Every process runs under `taskset -c 3`, `prlimit --as=2000000000` and `timeout`. Nothing
  else of this lane runs while a measurement runs.
- linz: L2 cells (n, d) ∈ {(11,5), (11,6), (13,6), (13,7), (19,9), (19,10)}, m = 2, all 200
  targets of each stream, 3 repetitions, on core 3, after all `measure` processes.
- Before measuring, each process runs a self-check (field identities x·x⁻¹ = 1, x² = x·x;
  points on the curve; P + Q − Q = P). A failed self-check aborts the process.

## 5. Controls

- **A/A** (noise floor, negative control for spurious differences): `ga_aa / ga`. Pass: the
  union of the per-process bootstrap 95% intervals lies inside [0.95, 1.05]. The relative noise
  floor of a (n, impl) is max(1 − lo, hi − 1).
- **Positive control 1** (a price known by construction): `ga4 / ga`. Pass: point in [3.6, 4.4].
- **Positive control 2**: `mul8_lat / mul_lat`. Pass: point in [7.2, 8.8].
- **Negative control** (no work): `null / ga`. Pass: upper bound < 0.01 GAE. Its value is the
  absolute floor: an op priced within 2× of `null` is reported as "at the floor".
- **Drift control**: ns per word of `xor_w64` (independent of the field) across all processes.
  Pass: max/min ≤ 1.5.
- **Count exactness** (linz): every timed call's word-op count and refutation flag equal L2's
  committed per-target values in `raw/linz_n*_m2_d*.json`. Any mismatch voids the linz price.
- **External cross-check** (descriptive, no pass/fail): our `mul_lat`, `sqr_lat`, `inv_lat`,
  `dbl`, `xor_w4096` prices against `full_prices.json` (other host, 2026-10-14).

A failed control is reported next to every price it touches; a failed A/A or positive control
makes that (n, impl) row INCONCLUSIVE for the re-reads.

### Known limits of the ops, fixed before data (from the smoke run, which is not data)

- `fadd` is timed as a dependent chain through `black_box`, so it carries a store/reload
  (about 1.9 ns against 0.2 ns per word of `xor_w64` in the smoke). It is an upper bound on a
  field addition. Where a price in word-XOR units is needed (L4), the field addition is
  bracketed: low end 1 word op by construction (an addition in GF(2^n), n ≤ 64, is one 64-bit
  XOR; 2 words for 64 < n ≤ 128; 3 at 131), high end the measured `fadd` interval. Point: the
  low end.
- `inv_lat` is the field type's public inversion (`Gf2::inv`, `WideGf2::inv`, `Gf2W::inv`).
  The `wide` group addition inverts with a private normal-basis Itoh–Tsujii, so at n = 83
  `inv_lat` may exceed 1 GAE: it prices `WideGf2::inv`, not the addition's own inversion.
- The `w3` addition is dominated by `Gf2W::inv`. At n = 83 its GA is compared with the ecbench
  `wide` GA. The n = 131 prices are reported in the `w3` unit, as measured. As a labelled
  extrapolation they are also rescaled by the n = 83 ratio GA_w3 / GA_wide, an estimate of the
  price in an ecbench-quality addition at n = 131.

## 6. Reading rule

For op `o` over denominator `u` (u = `ga` for GAE, or `xor_wW` for word-op units), at one
(n, impl):
- point = median over the 3 processes of the per-process median of the per-round paired ratio;
- interval = union of the 3 per-process percentile-bootstrap 95% intervals of the median
  (2000 resamples);
- best-of-N = min over all rounds of `t_o` / min over all rounds of `t_u` (the ecbench `wcal`
  convention), reported alongside.
The price is labelled `best-of-N timing / median-of-rounds, local shared VM, provisional`.

## 7. Applications (re-reads, no rerun of L2 or L4)

**(a) L2's m = 2 trade.** For each m = 2 cell (n, d, D) of L2's Table C, with L2's net saving
at price 0 (`net0`), its mean word ops `wo` and break-even `b = net0 / wo`: the measured price
interval is [lo, hi] = the union of the pure row-XOR price at the cell's row width (smallest
measured W at or above ⌈cols/64⌉) and the all-in linz price. Reading: filter LOSES if lo > b;
WINS if hi < b; otherwise unresolved. Cells with net0 = 0 lose at every positive price. The §5a
flag "ranked at calibrated price" is false wherever net0 > 0, because the filter wins at w = 0.

**(b) L4's G vs I.** For each L4 cell, per target: G = mul·P_mul + sqr·P_sqr + inv·P_inv +
add·P_add with the measured prices in word-XOR units at that n (L4's `g` counts; inversion at
the measured `inv_lat` price, not Itoh–Tsujii); I = `inherited-f4` native `ops` (min over
repetitions). I/G is the geometric mean over targets. Point: the latency variants at W = 64.
Range: every price at its interval's low or high end, over both the latency and throughput
variants and every W from 4 to 4096 (G at all-high gives the low I/G and vice versa). The row
width of `inherited-f4`'s word XORs is not recorded (it XORs partial row tails), hence the wide
W bracket. A secondary range over W ≥ 16 only is reported as description and decides nothing.
Reading:
G cheaper if the whole range is > 1, I cheaper if < 1, unresolved otherwise. A cell where I hit
its budget has I as a lower bound.

## 8. Predictions (pre-registered)

- P1: as in `full_prices.json`, the `small` GA is dominated by its inversion (`inv_lat` 0.7–0.95
  GAE), `mul_lat` 0.02–0.08 GAE, `xor_w4096` 1e-3–5e-3 GAE at n ≤ 47, falling with n.
- P2: at n = 83 and 131, `mul_lat` is 0.01–0.05 GAE and a word XOR is below 1e-3 GAE.
- P3: L2: every D = 3 and D = 4 cell LOSES (break-evens ≤ 2e-4, measured ≥ 5e-4); the D = 2
  cells at n = 11 and 13 (break-even 1.3e-3 and 4.5e-4) are the only ones that could flip.
- P4: L4: the measured mul price lies between P4 (4 word ops) and P2n (2n), so the m = 2 regime
  cells (I/G 2.2–2.6 at P4, 0.2–0.55 at P2n) may resolve either way; no direction predicted.

Falsifier of the instrument: a failed A/A or positive control at most (n, impl) cells, or a
count mismatch in linz.

## 9. What would make it authoritative

A local shared-VM timing is provisional: no isolcpus, no frequency pinning, no hardware
counters (the `perf-ipc` skill refuses on this VM), and noisy neighbours on cores 0–2. The
authoritative w_cal for binary rows needs an isolated host (`isolab-eks`); the task card is in
RESULTS.md.

## 10. Before the freeze

A smoke run (seeds 999 and 1, 5 rounds, scratchpad only, not data) checked the code paths and
found the limits listed under §4 (fadd, wide inversion, w3 addition, K_0 unavailable at n = 11
and 29, L2's recorded seeds differing from `run_linz.sh`). No smoke number enters RESULTS.md.
The run script is `run.sh`; the binary's sha256 is recorded in `raw/BINARY.sha256` at run time.

Amendments, if any, are appended below with a date.
