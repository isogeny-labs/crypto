# The IC CPU gap at m = 14-15 (REVIEW-3, new blocking issue 2)

Question: in `unit-price-f2m4-m14-17-rerun.json` IC cost 11.1-16.2 ns per
full-priced FME and rho 4.4-13.6 ns per counted op, so IC looked at least
2.5x dearer per priced unit at m = 14-15, about 1.2x at m = 17, and about
1.17x on the complete solves at m = 10-12. Is some IC phase under-priced at
m >= 14, or is the partial run unrepresentative?

## Conclusion

**Neither. The gap was the measurement mode, not the pricing and not the
length of the run.** The m = 14-17 re-run called `ic.pipeline_f2m4_full`
through the global rayon pool with one worker. The main thread is not a member
of that pool, so each of the solver's 387 parallel regions per trial is
handed to the worker thread and the main thread blocks. On one pinned core
that costs 5-8 us per region: 2.0-2.6 s of system time and 2-4 s of
user-side spinning per 2000 trials, about the same at every m. The complete solves at
m = 10-12 (`closure/m1?.json`) used `on_one_thread`, which runs every region
inline; the same code run both ways differs by the hand-off and nothing else.

With every region inline the IC CPU per priced FME is 3.9 / 4.0 / 4.6 / 5.8 /
9.0 ns at m = 10 / 12 / 14 / 15 / 17, rising with m about as rho's does (the
FME unit drifts with the cache, REVIEW-3 non-blocking note). IC over rho is
then 1.2-1.6 at m = 10-12 (median 1.32) and 0.7-1.7 at m = 14-17 (median
1.21 at m = 14-15, 1.06 at m = 17) with **no trend that the run-to-run noise
does not cover**, so there is no m-dependent shape left to flatten IC's
slope. No price is
changed: no IC phase is under-priced enough to matter (table 3), and the
hand-off is a cost of the scheduler on one core that no operation count
should carry. The unit-price harness's plain IC block is now labelled
(`pool_mode`) and every `--phases` run reports both modes.

What remains is a **level**, not a shape: over all 36 inline comparisons in
table 2 the median IC/rho CPU-per-FME ratio is 1.26 (range 0.70-1.70), and
the m = 10-12 complete solves gave 1.04-1.45 (`closure/m1?.json`, IC 1.61-1.66
and rho 1.12-1.59 user over priced). It does not move with m, and it is in
IC's favour: IC's priced cost sits about a quarter below its CPU, relative to
rho. The partial
runs cannot size it better than +-25 %: the host is not isolated, the local
F_q multiplication time is bimodal (1.8-4.2 ns between runs at one m), and
rho's per-FME cost depends on the length of the walk (table 2).

## Method

* `phase_cpu.rs`: exclusive per-phase timers on the calling thread's CPU
  clock (`CLOCK_THREAD_CPUTIME_ID`, user plus system, ns) with exact minor
  fault counts. The kernel's user/system split is tick-sampled and too coarse
  per phase, so user and system are reported for the whole window
  (`getrusage`) and per phase only CPU and faults. Off unless
  `unit-price-f2m4 --phases`; counts are unchanged (test
  `the_phase_timers_change_no_count`).
* Phases: curve build (outside the window, unpriced), factor base, its second
  build for the report, symmetrised precomputation, per trial the Weil system
  build, Macaulay build, echelon, normal forms, charpoly, roots, eigenvectors,
  split and lift, oracle bookkeeping, the relation loop, the relation matrix.
* Priced side: counted F_q multiplications per phase (`OracleNative::
  solve_muls`) plus the solver's native counters per stage
  (`stage_native`: element fills, scans, skips, parallel regions, monomial
  hash operations), each at the rate the record's own `by_counter` gives it;
  `phase_priced` and `groups` in the JSON. The groups add up to the window's
  priced total (`sum_priced_fme`, test `the_phase_groups_add_up_to_the_priced_total`).
* `rho_full` is populated in the same units: a fixed-length partial walk of
  `*_full` rho (batched, 256 walks, 100,000 steps; unbatched `rho.negation`,
  25,000), whole and marginal (long minus quarter-length), as CPU ns and wall
  ns per full-priced FME. `ns_per_fq_mul_local` is recorded at every m.
* Runs: one thread (`RAYON_NUM_THREADS=1`), `taskset -c 3`, seed 1, 2000 IC
  trials, 3 repeats (the plain block takes the median of 3; the phased block
  reports the median run and every run's headline), same host class as the
  re-run (4-core KVM Xeon, `in_isolcpus=false`). The plain block runs first,
  before anything makes the main thread a member of the one-thread pool
  (after that every solve is inline). One process per m. Extras: 10,000 trials
  at m = 14; rho at 10x the steps (`rho1M`); m = 10 and 12 for the same
  table.

Files (all in this directory): `unit-price-f2m4-cpu-gap-m{10,12,14,15,17}.json`,
`...-m14-10000trials.json`, `...-rho1M-m{14,15,17}.json`.

## Table 1: IC CPU per full-priced FME, ns (user + system)

| m | IC plain (hand-off) | IC inline | hand-off, us per region | system s (plain) | priced FME / trial |
|---|---|---|---|---|---|
| 10 | 7.05 | 3.88 | 5.3 | 2.25 | 651k |
| 12 | 6.85 | 3.97 | 4.9 | 2.04 | 657k |
| 14 | 8.53 | 4.58 | 5.9 | 2.14 | 574k |
| 15 | 11.31 | 5.75 | 7.1 | 2.32 | 496k |
| 17 | 15.30 | 8.96 | 8.0 | 2.15 | 486k |
| 14, 10,000 trials | 8.22 | 4.81 | 5.0 | 10.54 | 564k |

Counts and priced FME are identical in the two modes (same seed, same
trials); only the CPU differs. The hand-off is proportional to the number of
trials (10,000 trials: 5.0 us per region, 10.5 s of system time), so a longer
run does not amortise it: the short-run hypothesis is refuted, not
supported. Inline, 10,000 trials cost 4.81 ns per FME against 4.58 at 2000,
and the one-off set-up (factor base, precompute: 0.10 s at m = 14, 2 % of a
2000-trial run, 0.4 % of 10,000) is not what moves the number. The complete
m = 14 solve was killed at the 25-minute cap earlier (log in `../logs`), so
no complete solve is claimed.

## Table 2: IC (inline) against rho, CPU ns per full-priced FME

Rho is the `*_full` arm on the same curve, one thread, a partial walk priced
at the same unit (`whole` includes its set-up, `marg` is long minus quarter).
Ratio = IC inline / rho.

| run | IC inline | rho batched whole / marg | rho unbatched whole / marg | IC/rho batched (whole, marg) | IC/rho unbatched (whole, marg) |
|---|---|---|---|---|---|
| m10 | 3.88 | 3.15 / 2.85 | 3.18 / 2.47 | 1.23, 1.36 | 1.22, 1.57 |
| m12 | 3.97 | 3.10 / 2.80 | 3.31 / 2.56 | 1.28, 1.42 | 1.20, 1.55 |
| m14 | 4.58 | 5.53 / 5.81 | 3.89 / 3.01 | 0.83, 0.79 | 1.18, 1.52 |
| m15 | 5.75 | 5.79 / 4.20 | 4.33 / 3.38 | 0.99, 1.37 | 1.33, 1.70 |
| m17 | 8.96 | 9.18 / 8.69 | 7.71 / 6.53 | 0.98, 1.03 | 1.16, 1.37 |
| m14, rho 1M steps | 4.77 | 3.84 / 3.86 | 3.83 / 3.75 | 1.24, 1.24 | 1.24, 1.27 |
| m15, rho 1M steps | 5.59 | 4.20 / 4.15 | 3.66 / 3.39 | 1.33, 1.35 | 1.53, 1.65 |
| m17, rho 1M steps | 9.38 | 13.43 / 13.48 | 8.57 / 7.72 | 0.70, 0.70 | 1.09, 1.21 |
| m14, 10,000 IC trials | 4.81 | 3.76 / 3.27 | 3.89 / 3.08 | 1.28, 1.47 | 1.24, 1.56 |

The plain (hand-off) IC against the same rho: 1.5-2.7 (batched) and 2.1-3.4
(unbatched) at m = 10-15, 1.3-2.3 at m = 17. That is the re-run's pattern:
the hand-off is a fixed ~2 s of system time plus 2-4 s of user spin per 2000
trials, and the ratio closes at m = 17 because rho's cost per op rises
faster with m than the fixed hand-off does (the re-run compared rho per
counted op, 13.6 ns at m = 17). Two cautions on the inline ratios: rho batched
at m = 14 is 5.5 in one run and 3.8 in two others (host noise, +-35 % on a
0.3 s walk), and at m = 17 rho batched goes from 9.2 (100k steps) to 13.4 (1M
steps) as its tables outgrow the cache, so a partial rho walk understates the
complete one at the top of the range. Both blur the level; neither makes a
trend in m.

## Table 3: CPU share / priced share, % (cpu ns per priced FME), per phase

Inline, 2000 trials, the median run of 3. Solver phases are each phase's
counted multiplications plus its own native counters; "solver" also carries
table inversions, systems built, witness lookups and lifts, which have no
stage (0.5-0.7 % of the priced total).

| phase | m10 | m12 | m14 | m15 | m17 |
|---|---|---|---|---|---|
| weil_build | 1.5 / 1.1 (5.2) | 1.4 / 1.1 (5.4) | 1.4 / 1.0 (6.6) | 1.3 / 0.9 (8.2) | 1.1 / 0.8 (12.9) |
| macaulay_build | 19.3 / 22.9 (3.3) | 18.9 / 22.2 (3.4) | 18.2 / 19.5 (4.3) | 17.5 / 17.6 (5.7) | 12.5 / 13.7 (8.2) |
| echelon | 40.8 / 46.0 (3.4) | 40.1 / 44.5 (3.6) | 39.1 / 41.6 (4.3) | 38.4 / 38.1 (5.8) | 29.7 / 34.3 (7.8) |
| normal_form | 8.8 / 7.6 (4.5) | 8.7 / 7.5 (4.6) | 8.5 / 6.6 (5.9) | 8.4 / 5.8 (8.4) | 7.9 / 7.6 (9.2) |
| charpoly | 18.5 / 13.6 (5.3) | 18.7 / 13.4 (5.5) | 18.3 / 15.4 (5.5) | 19.0 / 17.8 (6.1) | 25.5 / 18.1 (12.6) |
| roots | 5.4 / 6.0 (3.5) | 7.4 / 8.4 (3.5) | 9.1 / 12.5 (3.3) | 9.8 / 16.1 (3.5) | 11.8 / 19.9 (5.3) |
| eigen | 2.2 / 0.3 (29.7) | 2.2 / 0.3 (30.0) | 2.2 / 0.3 (29.2) | 2.1 / 0.4 (31.7) | 2.0 / 0.4 (45.9) |
| split_lift | 0.3 / 0.1 (16.1) | 0.3 / 0.1 (15.7) | 0.4 / 0.1 (15.3) | 0.4 / 0.1 (15.4) | 0.3 / 0.1 (20.7) |
| **solver, all** | 97.1 / 98.2 (3.8) | 98.0 / 98.2 (4.0) | 97.4 / 97.7 (4.6) | 97.1 / 97.4 (5.7) | 91.2 / 95.5 (8.6) |
| setup (fb, precompute) | 2.5 / 1.7 (5.7) | 1.5 / 1.7 (3.5) | 2.0 / 2.2 (4.3) | 2.3 / 2.5 (5.3) | 8.0 / 4.4 (16.3) |
| relation loop | 0.4 / 0.1 (16.1) | 0.5 / 0.1 (17.9) | 0.5 / 0.1 (16.3) | 0.5 / 0.2 (19.7) | 0.9 / 0.2 (50.7) |
| relation matrix | 0 / 0 | 0 / 0 | 0 / 0 | 0 / 0 | 0 / 0 |

Reading it:

* **The solver is priced within 0.2-1.1 points of its CPU share at
  m = 10-15** (97-98 % of the CPU) and 4.3 points over at m = 17 (91.2 %
  of the CPU against 95.5 % of the price). Nothing in the solver is mispriced by a factor that depends on m:
  the CPU per priced FME of the whole solver follows the overall figure
  (3.8 to 8.6).
* Phases whose CPU share exceeds their priced share (points of the total):
  `charpoly` +4.9 / +5.3 / +2.9 / +1.2 / +7.4 at m = 10 / 12 / 14 / 15 / 17
  (det A at 100+ points is dominated by matrix fill and the determinant's
  loop, which a multiplication count absorbs), `eigen` +1.9 at every m (30
  ns per priced FME against ~4: its counted multiplications are 0.3 % of the
  price but the chain eigenvectors and candidate checks are 2 % of the CPU),
  `normal_form` +1.2 to +2.6 at m = 10-15. Phases whose price exceeds their
  CPU: `roots` -0.6 / -1.0 / -3.4 / -6.3 / -8.1 (its multiplication count
  grows with m, its CPU per multiplication does not follow), `echelon`
  -5.2 / -4.4 / -2.5 / +0.3 / -4.6 and `macaulay_build` -3.6 / -3.3 / -1.3 /
  -0.1 / -1.2. They largely cancel, which is why the solver total is
  balanced. A share error of 7-8 points would move IC's total by that many
  percent if it alone were repriced; the two largest, `charpoly` (+7.4) and
  `roots` (-8.1) at m = 17, point in opposite directions, and no phase's
  error grows from m = 17 down to m = 14-15 in a way that could explain a
  2.5x.
* The only m-dependent item is **set-up at m = 17**: factor base, its second
  build and precompute are 8.0 % of a 2000-trial run's CPU against 4.4 % of its
  price. That is the short run: 0.69 s of fixed set-up, not repeated per
  trial. A complete solve needs about columns / hit rate trials (4,122 /
  0.7 % = 6e5 at m = 14, 32,881 / 0.6 % = 5e6 at m = 17), a few hundred to
  a few thousand times more, so the set-up is below 0.1 % there. It is not IC
  work a complete solve is under-charged for.
* Relation matrix: no weight in a partial run (10-15 rows). A complete solve
  carries the matrix; its price (the mod-r row operations at the one-pass
  units) is the only part of the IC total this partial run does not test, and
  the closure runs at m = 10-12 (user over priced 1.61-1.66 for IC) already
  include it.

## What changed in the tools

* `phase_cpu.rs` (timers), hooks in `gaudry_quartic_bin.rs`, `ic_boundary.rs`
  and `ic_wide.rs` (inert unless started), `OracleNative::{solve_muls,
  stage_native, unsolved}` (counted work and native counters by stage, read
  only by the report), `methods::phase_priced_block` (added to `detail` only
  while the timers run), `methods::F2M4_RHO_STEP_CAP` (a step cap for the
  F_(2^m)^4 rho arms, zero = off = the claim's cap), `unit_price --phases`
  and `ecbench unit-price-f2m4 --phases`.
* Counts of every method id are unchanged: the timers read clocks, the stage
  counters are written beside the old ones, and the cap hook is inert at its
  default. Test: `the_phase_timers_change_no_count`.

## Not done

* No complete solve at m >= 14. The earlier m = 14 attempt hit the 25-minute
  cap (`../logs/unit-price-m14-complete.log`) in hand-off mode. Inline an IC
  trial is 2.7 ms at m = 14, so about 6e5 trials is ~27 minutes plus the
  matrix, and the attempt in hand-off mode (4.6 ms per trial) would have
  needed ~45; m = 15 would take ~1 hour, m = 17 about 6 hours on one core.
  Not attempted here; the 10,000-trial run is the longer run.
* The re-run's absolute numbers (`unit-price-f2m4-m14-17-rerun.json`) stay as
  committed and are superseded for the CPU comparison by the files above.
  The `*_ref*` records are still those of the earlier divisor.
* Host not isolated (`in_isolcpus=false`); the +-25 % level uncertainty above
  wants isolab, as REVIEW-3 already says for the one-pass table.

## Reproduce

```sh
CARGO_TARGET_DIR=target-repl cargo build --release -j2 --bin ecbench
# one process per m (the plain block must run before any one-thread solve)
RAYON_NUM_THREADS=1 taskset -c 3 target-repl/release/ecbench unit-price-f2m4 \
  --m 14 --reps 3 --ic-trials 2000 --rho-steps 100000 --seed 1 --phases \
  --out research/gaudry_k4_char2_replication_20261026/unit-price/unit-price-f2m4-cpu-gap-m14.json
```

`--rho-steps 1000000` gives the `rho1M` files; `--ic-trials 10000 --reps 1`
the 10,000-trial file. Curve build at m = 12, 14 and 17 takes 1.5-3 minutes of
CPU before the window (unpriced).
