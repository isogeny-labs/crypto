# Protocol: the line-search sieve route against a matched negation rho, same instances, all three primes (frozen 2026-11-03)

Frozen before any new run. `results/FROZEN.sha` holds the SHA-256 of this file, `run.sh`, the binary and the analysis
tool (`fit/src/main.rs`, `fit/Cargo.toml`) with a UTC timestamp from the system clock (no git). If a threshold proves
unsuitable the outcome is reported as failed, not changed. Item 3 of "Next round" in
`research/jv_cover_next_leads_20261102/LEADS.md`.

**Scope.** The weak class only: full rational 2-torsion, `#E = 4l`, the genus-3 cover over `F_{p^2}`; never prime
order. **Class of claim:** a measured, paired comparison, per instance, of two algorithm settings of one existing
sieve route and a negation-map Pollard rho, in one unit, on the same instances. Accounting and distance to rho, not
a new improvement. **Labels:** *measured* = read from a record; *derived* = computed from records in Rust;
*model* = extrapolation (none is made).

## Question

LEADS.md multiplied two ratios measured on different runs (sieve-with-line-search / plain sieve from
`jv_sieve_fbline_20261102`, plain sieve / rho from `jv_sieve_negrho_20261102`) and got S/rho about 0.0042 (arm H, U1)
and S+/rho about 0.034 (arm U, U2) at p = 1511. This study measures the cover arms and the rho on the same
instances at p = 1009, 1511 and 1823 and reports cover / rho per instance.

## Producer

One frozen binary, `jv_cover_round3_frozen` (built at main aa1c89ab2a), SHA-256 in `results/FROZEN.sha`. No cargo
build of the main crate is made. It is the only producer of cover and rho records; the p = 1511 rho records are
study B's (binary `jv_cover_rho_frozen`, c2b49d7682 code), reused after a bit-for-bit replay (control R1).

## Instances

`p in {1009, 1511, 1823}`, seeds 17-24: 8 per prime, 24 instances. The cover and the rho derive `l` and `c_add_e`
from the seed; control I1 checks they agree per seed.

## Arms and runs

* **Cover** (`--exp sieve --stop 64 --sizes p --first-seed 17 --seeds 8 --rho-runs 0`, `JV_LIN=sparse`, V2's and
  study A's options): **P** plain (`JV_FB_LINE` unset), **H** `JV_FB_LINE=4096 JV_FB_LINE_MODE=horner`,
  **U** `JV_FB_LINE=32768 JV_FB_LINE_MODE=table`. 72 runs. T (exhaustive table) is not re-run (study A found it
  dominated by U in S+ and the band).
* **Rho** (matched): `--exp rhostrong --neg 1 --batch 16 --r 256 --dp-bits 16 --sizes p --first-seed s --seeds 1 --run k`,
  one process per run, single thread. p = 1009: seeds 17-24, runs k = 0, 1 (16 runs). p = 1823: seeds 17-24, run
  k = 0 (8 runs). p = 1511: study B's 16 runs (seeds 17-24, k = 0, 1), copied unchanged with SHA-256s into
  `results/inputs/` and `results/rho/`.
* **Reference population #3278:** its 126 runs (`research/jv_cover_negrho_rescore_20261101/results/{inputs/old_rhostrong,new}`,
  p in {1301, 1451, 1601}, same settings), read in place.

## Units and per-run metrics

Per rho run: `s_harness` (U1, group additions) and `s_tuned` (U2, textbook F_p mults) as emitted, both
`/(c_add_e sqrt l)`; `steps/sqrt(l)`; native operations `(canon_cmp + hash + dp_check + scalar_ops + table_ops +
cycle_cmp)/sqrt(l)`, `fp_adds/sqrt(l)`. Per cover run: `S = total_muls/(331 sqrt l)` (search multiplications are in
`total_muls`), `S+ = total_plus/(331 sqrt l)` (V2's definition plus the search's emitted adds and lookups).
`c_add_e = 331` everywhere (checked).

## Estimand and statistics (frozen)

* **Per-instance ratio.** For cover instance i (prime p, seed s) and arm a: `R_i = X_i / ref_p`, `X` one of `S`,
  `S+`; `ref_p` the reference mean of the unit (U1 `s_harness`, U2 `s_tuned`) for prime p. Primary: **geometric
  mean of R_i over the 24 instances (pooled) and over the 8 of each prime.** Also reported: `S+` over the rho
  priced like-for-like (`S+/rhoS+`: rho native + fp_adds counted one per mult), and, as information, the ratio to
  each instance's own 1-2 rho runs (noisy).
* **Reference.** *Pooled*: mean over the #3278 126 runs plus all 40 new/reused runs (one value for all primes).
  *Per-prime*: mean of that prime's matched runs (16, 16, 8). Both are always computed; the decision below
  picks the primary.
* **Homogeneity (frozen decision, metric steps/sqrt(l)).** Four permutation tests (20000 permutations, group sizes
  kept): (a) permutation one-way ANOVA statistic over the three primes' matched groups; (b) Kruskal-Wallis H over
  the same; (c) absolute difference of means, all 40 matched runs vs #3278's 126; (d) two-sample KS D, same pair.
  **Homogeneous iff all four p >= 0.05** (no multiplicity correction: it favours the per-prime reference, which is
  always valid). **Primary reference: pooled if homogeneous and the replays pass, else per-prime.** Printed for
  information: each prime vs the 126, and the same tests on `s_harness` and `s_tuned`.
* **Positive control (per prime).** Matched mean of steps/sqrt(l) within 3 se of `sqrt(pi/4)/sqrt(1-1/256) = 0.88796`
  (se = sd/sqrt(n)). A prime that fails is flagged and so is every verdict at that prime.
* **Interval.** 95% percentile bootstrap, **20000 resamples**, jointly: cover seeds resampled with replacement
  within each prime (8 draws, the same draw shared by arms, references, units and w), and rho runs resampled with
  replacement within each instance group `(p, seed)` (group sizes kept; pooled mode: 9 #3278 groups + 24 new;
  per-prime mode: that prime's 8 groups). splitmix64, fixed seed in the tool. Verdict on `[lo, hi]`: *below* rho if
  `hi < 1`, *above* if `lo > 1`, else *not resolved*. "How far below" = `1/point` with the interval inverted.
* **Section 5a band.** Native operations priced at w = 0, 0.1, 1 GAE each (1 GAE = 331 F_p mults) on both arms:
  `ratio_w = geomean over instances of (S + w nat_cover/sqrt l) / (rho_unit + w nat_rho/sqrt l)`.
  **Layer B (as emitted, the band claim):** cover native = `sieve_lookups + search lookups`; rho native = the six
  #3278 counters. **Layer A (V2's upper bound):** cover native = `sieve_adds + sieve_lookups + enum_adds + search
  adds + search lookups + (total_muls - sieve_muls - enum_muls)`; rho native = six counters + `fp_adds`. The order
  "cover below rho" holds across the band for a cell iff the verdict is the same at all three w.
* **Comparison with the product estimate** (p = 1511, primary reference): arm H S/rho U1 vs 0.0042 and arm U
  S+/rho U2 vs 0.034 (also H U2 vs 0.010, U S+ U1 vs 0.014); reported as measured/product and whether the product
  lies inside the interval. Seeds 17-24 only, whereas LEADS.md's factors used all 16 seeds.
* **Phase shares** of `total_muls` (enumeration, sieve, descent, linear algebra, search, other) per arm and prime,
  and the cover-only arm/P ratios (point), reported.

## Controls (all must pass or the dependent decisions are withheld and the failure reported)

| id | requirement |
|:--|:--|
| C1 | every re-run cover record (P, H, U; 3 primes; seeds 17-24) equals study A's record for that arm, prime and seed in every field but `wall_ms` |
| C2 | every cover run solved, correct, 0 failed verifications, not exhausted, `lin_mode` sparse, `fb_line` absent (P) or mode and candidates as declared (H, U) |
| R1 | study B's record `p=1511 seed=17 run=0` re-run with the frozen binary is identical in every field but `wall_ms`; run before any new rho run (`run.sh` blocks the rho queue until it passes) |
| R2 | #3278's record `p=1451 seed=1 run=0` (single seed row) re-run with the frozen binary: same test, same blocking (instrument control) |
| I1 | `l` and `c_add_e` of every rho run (new and reused) equal the cover P record of the same prime and seed |
| R3 | every rho run correct, uncapped, negation, batch 16, r 256, dp-bits 16; an excluded run is listed, never replaced |
| PC | positive control per prime (above) |

If R1 or R2 fails the #3278 runs are excluded from the reference and the primary is per-prime.

## Budget and cut (frozen)

At most 4 h wall on the shared 4-core box (counted metrics are host-independent; wall times are an observation).
`run.sh` runs the cover arms sequentially and the rho queue four-wide concurrently. Rho queue order: p = 1823 seeds
17-20, then all p = 1009 jobs, then p = 1823 seeds 21-24. **No rho job launches after T0 + 2.5 h; a job over 1 h is
killed and recorded.** **Cut rule:** if the p = 1823 seeds 21-24 jobs cannot run, p = 1823 has 4 rho runs (seeds
17-20); the tool flags it, the analysis proceeds, and the cut is reported. A cut removes rho runs, never a cover seed.

## Deliverables

`PROTOCOL.md`, `run.sh`, `results/` (records, logs, `walls.txt`, `FROZEN.sha`, `versions.txt`), `fit/` (Rust),
`fit.txt` / `fit.json` / `table.md`, `graph.svg` / `graph.png`, `README.md`, `REPORT.pdf`. No Python computes a
number. No leaderboard or `benchmarks/` row: there is no `VALIDATION.json`.

## Limits declared in advance

* Local run on the 4-core box, not isolab-eks: instructed by the task owner; the quantities are counted and
  host-independent for the deterministic cover and rho records.
* Cover seeds 17-24 are the same instances on which arm H/U were selected (study A) and on which the n rule's
  exponent was set; the n values were frozen in study A before data and are not re-tuned here.
* 8 instances per prime and 8-16 heavy-tailed rho runs (sd about 0.45-0.48 of a 0.89 mean) per prime give wide
  per-prime intervals; the pooled interval is the main result.
* The distances are to a negation-map rho at these primes (l about 2^55-2^64); no extrapolation is made.
