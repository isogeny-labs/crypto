# Integration of the F4 speed-up leads  [pcc:unverified]

Repository: /home/user/crypto, branch `claude/funny-brown-735uvs`, base HEAD a53cc49711.
All statements below are [pcc:unverified] assertions of this integration step. No
wall-clock effect is claimed: the host was shared and no serial A/A stage was run.

## Leads

| lead | status in the input | integrated | files |
|---|---|---|---|
| f4-parallel | candidate | yes | src/cryptanalysis/f4_fp.rs |
| f4-memory | candidate | yes (reconciled with f4-parallel, see below) | src/cryptanalysis/f4_fp.rs |
| gf2-m4ri | candidate | yes (plus a test-race fix, see below) | src/cryptanalysis/koblitz_groebner.rs |
| crossbred-m3 | not in the candidate list | no | (its patch only adds examples/crossbred_m3_bench.rs) |

crossbred-m3 was named in the apply order but was not among the candidate results: its
NOTES.md conclusion is "measurement_only", no library code changed, and the patch is only a
measurement harness example. Nothing to integrate as a speed-up; not applied.

## Step 1: checkout
`git status --porcelain src examples` was empty before applying.

## Step 2: application and reconciliation
`git apply --3way` of f4-parallel, f4-memory, gf2-m4ri: all three applied textually clean.

Semantic conflict (f4-parallel vs f4-memory): f4-memory dispatched at the top of
`reduce_pairs_fl` and `reduce_pairs_blocked` into its copies `reduce_pairs_fl_mem` /
`reduce_pairs_blocked_mem`, which carry the OLD thread dispatch. Applied as is, the default
path would have silently bypassed every f4-parallel change in the B' build and the blocked
pair stage. Reconciliation (all in src/cryptanalysis/f4_fp.rs):

* `reduce_pairs_fl`: dispatches to `reduce_pairs_fl_mem` only when `mem_site(1) && par_old()`
  (so `F4_FP_PAR_OLD=1` gives exactly the f4-memory lead's verified function). Otherwise the
  f4-parallel body runs and, when `mem_site(1)` (`fuse`), applies the fused `MemGroup`
  updates at the same places the f4-memory lead did: `against_finished` and the row-wise
  in-block finish (copied from `reduce_pairs_fl_mem`), plus the two places that only exist in
  f4-parallel: the column-tiled B' build (`bprime_tiled`, default at >1 thread) and the
  column-tiled in-block finish (`finish_block_tiled`). Those use the same flush rule
  (normalise before a flush that would pass 256 pending updates) over tile-local slices.
  With `F4_FP_MEM_OLD=1` or `F4_FP_MEM_B=0`, `fuse` is false and the f4-parallel code runs
  unchanged.
* `reduce_pairs_blocked` / `reduce_pairs_blocked_mem`: the f4-parallel dispatch (spread gate,
  rows per block, PAR_OLD fallback) moved into one helper `pairs_blocked_dispatch` used by
  both kernels, so the fused site C runs with the new dispatch by default and with the old
  one under `F4_FP_PAR_OLD=1`.
* `rref32_deferred` / `reduce_lazy_by`: no conflict (f4-parallel adds profiling counters,
  f4-memory dispatches inside `reduce_lazy_by`).

Toggle semantics after integration:
* `F4_FP_PAR_OLD=1`: old thread dispatch everywhere (with f4-memory fusion still on unless
  turned off). `F4_FP_PAR_OLD=1 F4_FP_MEM_OLD=1` = the baseline code path.
* `F4_FP_MEM_OLD=1`: f4-parallel code with one-update kernels (f4-parallel's own path).
  `F4_FP_MEM_A=0 / _B=0 / _C=0`: one site each.
* `F4_FP_PAR_FL_MODE=tiles|rows`, `F4_FP_PAR_SYM_SEEDS`, `F4_FP_PAR_SYM_FRONTIER`,
  `F4_FP_PAR_FL_WORK`, `F4_FP_PAR_FL_INBLOCK`, `F4_FP_PAR_FL_TILES`; diagnostics
  `F4_FP_PAR_PROF=1`, `F4_FP_MEM_STATS=1`.
* `KIC_M4RI_OLD=1`: gf2-m4ri old sorted Macaulay builder.

Test fix (gf2-m4ri): the lead's new test `parity_rows_match_sorted_rows_and_tally` compared
the process-wide layout hit/miss atomics, which other tests in the same harness reset and
increment concurrently. With the default parallel test runner it failed 4 out of 4 runs
("case 1 layout stats: (0, 1) vs (0, 0)"); it passed alone and with `--test-threads=1`.
Fix (not a weakening): the six counter increments go through `f4_layout_count(hit)`, which
bumps the same global atomics and, under `#[cfg(test)]` only, a thread-local mirror
(reset by `f4_layout_stats_reset`); the test compares the calling thread's counts
(`f4_layout_stats_thread`). The layout cache is itself thread-local, so the check is the same
equality, now isolated from other tests. Production behaviour and the public
`f4_layout_stats` are unchanged.

## Step 3: build
`flock repo.lock cargo build --release --example f4_fp_bench --example crossbred_bench
--example koblitz_f4_kernel_bench`, no warnings. No lead added a new example (crossbred-m3's
example was not integrated). Binaries: scratchpad/integrated/bin/ (rebuilt after the test fix;
sha256 in BINARIES.sha256 next to this file).

## Step 4: correctness matrix
Script: scratchpad/integrated/matrix.sh, outputs in scratchpad/integrated/matrix/, log
matrix.log (first build) and matrix2.log (final build). Reference: scratchpad/baseline/bin.

f4_fp_bench `1 large` (13 cases; compared basis_fp, solve_fp, field_ops, solve_field_ops,
steps, max_rows, max_cols, basis_len, solutions), each arm at RAYON_NUM_THREADS=1 and 4:
default; F4_FP_PAR_OLD=1; F4_FP_MEM_OLD=1; F4_FP_MEM_A=0; F4_FP_MEM_B=0; F4_FP_MEM_C=0;
F4_FP_PAR_FL_MODE=tiles; F4_FP_PAR_FL_MODE=rows; KIC_M4RI_OLD=1; all toggles
(F4_FP_PAR_OLD=1 F4_FP_MEM_OLD=1 KIC_M4RI_OLD=1); F4_FP_MEM_OLD=1 + FL_MODE=tiles;
F4_FP_FL_BLOCKED=0 (vs baseline with the same flag). Also default, tiles, MEM_OLD at 2
threads. Result: every arm identical to baseline on every case, no counter changed
(e.g. quad_n9 5a5d7dea3744f69d / 399,396,830 field_ops).

Fused path coverage (F4_FP_MEM_STATS=1, 4 threads, quad_n9): default (tiled B') bprime site
115,070,238 ops, 15,747,883 accumulator words, 310,552 flushes; PAR_OLD=1 (lead's _mem
function) 115,070,238 / 16,389,025 / 37,183; FL_MODE=rows (tiled in-block finish)
115,070,238 / 16,389,025 / 64,960; deferred 69,287,216 ops and pairs 214,305,456 ops in all
three. So the new fused tiled code is exercised by the matrix and output-identical.

crossbred_bench 9 11 13 (wall columns stripped) and koblitz_f4_kernel_bench 5 x / 5 sym
(median_ms/samples_ms dropped; rows, cols, rank, word_ops kept) at 1 and 4 threads under
default, KIC_M4RI_OLD=1, all toggles, F4_FP_PAR_OLD=1, F4_FP_MEM_OLD=1: identical to baseline
(verdicts yes; x 1023x4369 rank 1022 word_ops 4,335,210; sym rank 992 word_ops 3,628,128).

Mismatches: none. Final build (matrix2.log): 57 of 57 comparisons SAME.

## Step 5: tests
* `cargo test --release --lib f4_fp`: 97 passed (default env); also 97 passed with
  F4_FP_MEM_OLD=1 and with F4_FP_PAR_OLD=1.
* `cargo test --release --lib koblitz_groebner`: before the fix 94 passed, 1 failed (race
  above); after: 95 passed, 0 failed, 5 ignored, in 4 parallel runs, and with KIC_M4RI_OLD=1.
* `inherited_f4`: 18 passed, 2 ignored. crossbred.rs unchanged, not run separately.

## Step 6: commit
Commit 929c01c601 on claude/funny-brown-735uvs (not pushed); hooks did not refuse it.
Before committing, rustfmt reformatted 3 short hunks (line wrapping only: MemTimer::drop,
MemGroup::flush, f4_layout_count); the binaries and tests above are from the source just
before that whitespace-only change.

## Not done / open
* Wall clock not measured; needs the serial A/A stage (isolab-eks) with bench records.
* The fused tiled B' variants (bprime_tiled, finish_block_tiled) are new integration code,
  not measured by either lead; verified only for output identity and counters.
* `reduce_pairs_fl_mem` now only runs under F4_FP_PAR_OLD=1; F4_FP_PAR_PROF counters for the
  B' build are not collected on that path.
