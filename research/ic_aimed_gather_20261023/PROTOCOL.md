# IC collection: gather the aimed subset once per unit (protocol)

Written on 2026-10-23, **before any measurement of the change**. Parent:
`d34b9b390` (`origin/main`).

## Where the time is

A native `perf record -e cpu-clock --call-graph dwarf` sample of `ic price`
on rule v3's `k0n53/T01.params.json` (main plus #2411's rho change, one run;
`/tmp/claude-0/icprof/`) puts 5.6% of samples in two `Vec::extend_trusted`
closures inside `PairSumTable::witnesses_fast_scan`:
- 3.05%: the `Scan::Indices` gather of `gather_x` and `gather_y`;
- 2.51%: the key-hash pass.

These params use aimed collection (`collection_aim: true`, window 874). Each
trial's scan is a window of `Targets::Subset`'s fixed, sorted subset at a
rotating offset, so the same coordinates are gathered again on every trial.

## Change (no value changes)

`aimed()` gathers the subset's negated coordinates once per unit into
`doubled_x` and `doubled_y`, in the same order as `doubled`. A trial's scan
becomes `Scan::Gathered { idxs, xs, ys }`, whose window slices it hands to
`add_many_lazy_soa` directly. Every other path is unchanged:
- the point list (`!fused`);
- the index-to-witness mapping;
- other `Scan::Indices` callers.

The same summands go in, in the same order, so every relation, counter and
recovered logarithm must be identical.

## Identity gate

- **Unit tests:** the new test `a_pre_gathered_scan_hands_the_sink_what_the_index_list_does`
  (folded table, n = 19 and 31, sink order compared exactly) and the existing
  scan tests pass.
- **`ic price` outputs:** every `ic price` output on the measured params is
  identical between arms once timing, binary-hash and host fields are
  removed. That covers recovered logarithms, relation counts and every
  counter.

## Measurements (all three, as the owner asked)

1. **Callgrind instructions.** Total instructions for one `ic price`
   process at n53 and n61 (rule v3 `T01` params), per arm.
   - *Caveat, stated now:* Valgrind runs no AVX-512, so this counts the
     scalar fallback of the key kernels.
   - The fused SoA path that this change touches does run under Valgrind:
     it is gated on `KIC_SCAN_SOA` and the fold, not on AVX-512.
   - Reported as the instruction difference and ratio.
2. **perf sampling share.** Native `perf record -e cpu-clock` at 4 kHz,
   n53, 3 runs per arm. Reported as the share of samples in the
   `witnesses_fast_scan` gather closure, and in `witnesses_fast_scan` plus
   its closures, per arm. This is descriptive and not a timing claim.
3. **Many-run native timing.**
   - Setup: `ic price` at n53, rule v3 `T01` params, under
     `tools/isolated_bench.py --cpus 3`, `RAYON_NUM_THREADS=1`.
   - Schedule: an A/A of the baseline × 10, then 30 interleaved paired A/B
     rounds, with the arm order alternating each round.
   - Metric: the collect-phase time (`phases_ns.collect`, median over the
     price's repetitions) and the whole set-up (`total_ns`).
   - Statistic: the median of the 30 per-round paired ratios
     (baseline/candidate), with a 95% bootstrap interval over the rounds.
   - Rule: the native effect is **established** only if the interval's lower
     end is above 1 **and** above the A/A's own paired-ratio interval upper
     end. Otherwise it is reported and not claimed.
   - Contended runs are excluded and counted.

No IPC: the host has no hardware PMU.
