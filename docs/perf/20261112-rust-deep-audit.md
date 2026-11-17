# Rust deep audit, round 2: profile first, then WALL fixes and two new ids

Second round after [`20261110-wall-only-rust-fixes.md`](20261110-wall-only-rust-fixes.md)
(PR #3850). Branch `claude/nice-babbage-bn0fn0-r2`, cut from
`claude/nice-babbage-bn0fn0` at `08394720b6`. Rules: AGENTS.md §10 and
"Implementation language: no Python"; owner authorisation for this task:
WALL changes keep every value, counted unit, RNG draw and output; a COUNTER
or METHOD change ships under a new method/oracle id and never changes an
existing id.

Every wall figure below is **local, unisolated, [pcc:unverified]**: one
shared 4-vCPU VM, `--cpus none` (no `tools/isolated_bench.py`, no EKS),
other agents' builds running beside it, the 1-minute load average recorded
with each timing. Callgrind instruction counts are deterministic but were
not filed as pcc claims either: **[pcc:unverified]**. Identity (records,
replays) is the result of the Phase B items; times are indicative only.

## Host and binaries

- Intel Xeon @ 2.10 GHz (Sapphire Rapids flags: popcnt, bmi2, adx, avx2,
  avx512f, avx512ifma, avx512_fp16, amx_tile, pclmulqdq, vpclmulqdq),
  4 vCPU, 15 GB, Linux 6.18.44 x86_64, rustc 1.97.0 (2d8144b78 2026-07-07),
  `[profile.release]` unchanged. No PMU (no `perf`); valgrind 3.22 Callgrind.
- Binaries, `cargo build --release --bin ecbench` at each commit
  ([`binaries.sha256`](20261112-rust-deep-audit/binaries.sha256)):
  `r2-base` = `08394720b6` (the round-2 baseline), `b1` = `03d9587118`,
  `b2` = `703ae75e42`, `c` = `5e242a3128` (the tree of both Phase C
  commits).

## Phase A: profile before editing

Method. `ecbench profile-input --spec S --seq 0` exports the exact
measured-child input of a one-arm spec
([`profile-specs/`](20261112-rust-deep-audit/profile-specs)); the child
`ecbench exec < input` ran under Callgrind with `RAYON_NUM_THREADS=1`.
Two obstacles, both handled:

- The IC phase marks call a Callgrind `DUMP_STATS_AT` client request on
  every phase change (`ic_measurement::dump`, ungated): a 80 ms
  `ic.pipeline` child produced 371,831 part files and filled the disk
  (cleaned up; see Errors in the session log). Profiles were therefore
  taken with a **copy** of each binary whose `dump` entry byte is patched to
  `ret` ([`nodump.sh`](20261112-rust-deep-audit/nodump.sh)); its child
  report (wall fields aside) is identical to the unpatched binary's (checked on the 24-bit IC input).
  The measured binaries were never patched.
- `callgrind_annotate` mis-attributes multi-part files with compressed
  names, so [`cgagg.awk`](20261112-rust-deep-audit/cgagg.awk) /
  [`cg-top.sh`](20261112-rust-deep-audit/cg-top.sh) sum every part:
  exclusive Ir per function, and "inclusive" as self plus direct call cost
  (recursion can double count; used only to rank).

Workloads (one target each): `rho.negation`, `kangaroo.vow`,
`bsgs.textbook` on the 32-bit `prime_search` curve (`prime_search` stops
at 32 bits); `ic.pipeline` `mitm:negation_folded=1` + incremental-gauss on
24-bit (size 32) and 32-bit (size 256) prime-abscissa bases; `rootfind-mitm:m=3`
+ structured-gauss on the 24-bit prime curve (size 32) and on Koblitz
`a=0, n=19` (`koblitz-standard-subspace:dimension=6`); `descent-algebraic:m=2`
+ `f4-f2` on Koblitz `a=0, n=13`. `pollard_collab` has no ecbench path or
small CLI; it was not profiled (see "Not done").

Full exclusive/inclusive top-12 tables per arm:
[`profile-tables.md`](20261112-rust-deep-audit/profile-tables.md). The
heads:

| arm | total Ir | top exclusive (share) |
|---|---:|---|
| rho.negation 32-bit | 57.1 M | `TunedWalker::walk` 47.4% (inlines one `add_raw` copy, the look-ahead, the 16-key window scan, hashing), `PrimeCurve::add_raw` 41.2%, `remember` 6.4% |
| kangaroo.vow 32-bit | 58.9 M | `add_raw` 67.7%, `kangaroo_with` 27.2% |
| bsgs.textbook 32-bit | 73.9 M | `add_raw` 43.7%, `bsgs_textbook` 38.3% (inlined string-keyed `BTreeMap` counter lookups), `memcmp` 14.3% (the same lookups' key compares) |
| ic mitm + igauss 32-bit, size 256 | 4,191 M | `add_raw` 30.9%, relation loop 25.5%, guard-set `HashMap::insert` 8.0% + `reserve_rehash` 7.3%, `Trace::boundary` 7.8% + `clock_gettime` 2.4%, `PairTable::get` 5.4%; **`IncrementalGauss::add_row` 0.04% inclusive** |
| rootfind-mitm prime 24-bit | 311.9 M | `add_raw` 54.9%, `row_points` 11.5%, `PairTable::get` 10.9%, `probe` 8.3% |
| rootfind-mitm Koblitz n=19 | 18.5 M | `add_many_clmul` 14.6%, `PairTable::get` 9.1%, `batch_inv_clmul` 8.8%, `inv_clmul` 8.6% (already batched) |
| descent f4-f2 Koblitz n=13 | 191.2 M | `echelon_block_tables` 11.4%, `_int_malloc` 8.0%, `_int_free` 6.5%, `sort_masks_descending` 6.0%, `memset` 4.7%, `malloc`/`free` 6.3% |

Assembly of the hot functions (`objdump -d` of `r2-base`;
[`asm-audit.md`](20261112-rust-deep-audit/asm-audit.md)): no 128-bit
division calls (`__udivti3`/`__umodti3`/`__divti3`) and no bounds-check
panic calls in any of them; `add_raw` has 13 hardware `div`s (the 3
`mulmod_u32` and the unrolled 32-bit Euclid of the inversion, about 19
divisions an inversion at 32 bits), the walker 14 (an inlined `add_raw`
copy), `kangaroo_with` 7 (one per step: the jump index `h % jumps`),
`IncrementalGauss::add_row` 17; `bsgs_textbook` makes 5 out-of-line
`BTreeMap<String, u64>` calls (per step) and `kangaroo_with` 9 (per
distinguished point only); no SipHash in any hot loop (the DP and guard
maps are `FastMap`); allocation in the hot loops happens only in callees
(the guard set's growth; F4's per-call vectors).

What the profile says about the planned candidates:

- **R6-A** (`CountedFp` primitives): the inversion dominates the step.
  A microbenchmark of the exact chord step on a 32-bit prime
  ([`microbench/mb.rs`](20261112-rust-deep-audit/microbench/mb.rs), `rustc -C opt-level=3`, 3·10⁶ steps, five rounds) gave 101–107 ns/step
  with the current `div` primitives and 95–100 ns/step with a Barrett
  `mulmod` (identical outputs on 2·10⁷ random and edge operands): about 5%,
  since the Euclid inversion is about 85 ns of the step and no
  division-free single inversion beats it on this host (round 1 measured
  binary GCD at 0.63 s against 0.54 s wall in single samples [pcc:unverified]; Fermat with Montgomery-32 needs about 46 dependent
  products). A stored Barrett constant needs a new `PrimeCurve` field
  (128 literal constructors, and the struct is serialised) or a 64-bit
  division per addition to derive it, which cancels the gain. **Rejected**
  on measurement; the batched inversion of Phase C is where the step cost
  goes.
- **R4** (`IncrementalGauss` kernel): 0.04% of the 32-bit size-256 IC solve.
  **Not done**: no wall to gain at the sizes ecbench runs; it becomes
  relevant only for bases in the thousands of columns.
- **R13** (BSGS counters): 14.3% `memcmp` plus inlined lookups: **done (B1)**.
- **kangaroo `h % jumps`**: one 64-bit `div` per step: **done (B2)**.
- IC guard-set growth (`reserve_rehash` 7.3–7.7%) and per-phase timing
  (`Trace::boundary` + `clock_gettime` about 10%): not changed. The first
  needs a capacity guess (memory trade, no safe bound from `max_trials`);
  the second is the measurement instrument itself (`online.phases_ns`).
- F4 allocator churn (descent arm, about 25% of Ir in malloc/free/memset):
  spread over many call sites in `pq_f4_f2`; not attempted in this round.

## Phase B: WALL-only fixes

Verification for each commit: the three round-1 gate specs
([`gate-prime.json`](20261110-wall-only-rust-fixes/gate-prime.json),
[`gate-koblitz.json`](20261110-wall-only-rust-fixes/gate-koblitz.json),
[`gate-tensor.json`](20261110-wall-only-rust-fixes/gate-tensor.json)) run
with `r2-base` and the commit's binary, `records.jsonl` compared after
[`norm-records.sh`](20261110-wall-only-rust-fixes/norm-records.sh), and
`ecbench verify --replay-all` on the candidate session
([`gate.sh`](20261112-rust-deep-audit/gate.sh)). An A/A rerun of `r2-base`
reproduced its own normalised hashes, so the gate is deterministic.

| binary | gate-prime (36) | gate-koblitz (16) | gate-tensor (4) | replays |
|---|---|---|---|---|
| r2-base | `f81b9eb603435717` | `18257ae507917e7d` | `54a61ad4915a6b19` | — |
| r2-base A/A | identical | identical | identical | — |
| b1 (`03d9587118`) | identical | identical | identical | 36/36, 16/16, 4/4, `ok: true` |
| b2 (`703ae75e42`) | identical | identical | identical | 36/36, 16/16, 4/4, `ok: true` |
| c (`5e242a3128`, Phase C) | identical | identical | identical | 36/36, 16/16, 4/4, `ok: true` |

Timings: [`solveab.sh`](20261112-rust-deep-audit/solveab.sh) alternates
A and B for 5 rounds; a round sums the child's `report.solve_wall_ns` over
20 `ecbench exec` runs of the same input, `RAYON_NUM_THREADS=1`
([`wall-solve-abab.txt`](20261112-rust-deep-audit/wall-solve-abab.txt);
whole-process times in
[`wall-process-abab.txt`](20261112-rust-deep-audit/wall-process-abab.txt),
where process start-up of about 13 ms a run hides the 32-bit solve calls).

| # | commit | change | Callgrind Ir (child, before → after) | solve wall min/median ms, A → B (5 rounds × 20 runs) | load (1-min) | A/A on the same input |
|---|---|---|---|---|---|---|
| B1 (R13) | `03d9587118` | `bsgs_textbook`, `bsgs_interleaved`, `bsgs_negation_table`: per-step counters in a `StepCounters` struct, written to the map once (only non-zero keys, so the map is the one the per-step calls built) | bsgs.textbook 32-bit: 73,895,870 → 46,296,620 (−37.3%) | 304/318 → 283/293 | 3.3 | 310/322 vs 319/339 |
| B2 | `703ae75e42` | `kangaroo_with`: jump index by `FastMod` (Lemire–Kaser–Kurz 64-bit remainder with a 128-bit constant; unit test against `%` on moduli 1..64 and wide ones) instead of a 64-bit `div` per step | kangaroo.vow 32-bit: 58,900,601 → 60,005,058 (+1.9%; a `div` counts 1 Ir, its replacement 3 multiplies) | 324/351 → 328/343 | 3.4 | 333/355 vs 340/351 |

B1 is outside the A/A spread by a small margin; B2 shows **no measurable
wall difference** (inside the A/A spread). B2 is kept because it is exact,
tested and removes the only per-step division outside the group law, but
a reviewer who prefers no change without a measured gain can drop
`703ae75e42` alone. For a division removal Ir is the wrong measure (a
`div` is one instruction); wall is the figure that would show it, and here
it does not.

Whole `gate-prime` session (all 9 arms on existing ids, 36 records),
`r2-base` vs `c`, ABAB ×5 process wall: 6033/6304 → 5916/5992 ms
(load 3.0), not distinguishable from noise at this size.

Library tests on the final tree (`cargo test --release --lib --no-run`,
then the harness with filters, 4 threads): `ecbench::generic::` 7,
`ecbench::methods::` 71 (includes `registry_audit_tests`, 7, and the new
`rho_lanes_tests`, 2), `ic_framework::rootfind` 2, `ecbench::optimization` 4,
`ecbench::work::` 15, `ic_boundary::tests` 60 (+4 ignored): all pass.
`cargo clippy --release --lib --tests`: no warning in the changed code (one
pre-existing crate-level `unknown lint: clippy::chunks_exact_to_as_chunks`).
`rustfmt --edition 2021 --check` clean on every changed file. Not run:
`cargo test --release --test ecbench` and the debug-profile
`cargo clippy --all-targets -- -D warnings` of CI (disk: about 2 GB free).

## Phase C: two new ids (COUNTER/METHOD), parents unchanged

### `rho.negation_lanes_v1` (`5ab901b1a5`)

`ic_boundary::rho_walk_lanes`: `rho.negation`'s set-up, draw for draw
(`G + Q`, the jump table, the start stride), and its walk logic step for
step (canonical representatives, Wiener–Zuccherato look-ahead, 16-point
cycle window, escape by doubling the least-key member, distinguished
points, walk cap), run in `lanes` walks that advance together. Each round
every active lane requests one addition `X + M_j`; the round goes through
the prime curve's `native_lane_walk_adder(batched = true)` (prime_fast
Montgomery form, one shared inversion: Montgomery's trick). Finished lanes
take the next start `[k]T` in lane order. Registered `PrimeOnly`, params
`cap_multiple` (64) and `lanes` (no default; 1..=4096); optimization policy
`ImplementationAb` / `DiagnosticOnly`; registry audit case (`lanes=8`);
`lanes` and `lane_rounds` are bitop shape names.

**Counted unit.** `S` is charged as for `rho.negation`: one group operation
per walk step, per look-ahead rejection and per escape doubling, plus set-up,
starts and the final `[d]G` check; additions computed in the round in which
the solve ends are charged (they were performed). The field tally is the
batched one (per round with `k` ordinary additions: one inversion and
`3k + 1` multiplications; `2M + 1S` an addition; `2M` per point converted
into Montgomery form); conversions back are counted as
`montgomery_conversions_uncharged`, so they enter `unpriced` and the §5a band.

**Correctness.** With `lanes = 1` the walk is `rho.negation`'s exactly (unit
test over 8 seeds on a 22-bit curve: same `S`, steps, walks, distinguished
points and counters, plus the three new ones; same inversion count). Every
record of the smoke session ([`smoke-phase-c.json`](20261112-rust-deep-audit/smoke-phase-c.json),
30 records) and of the comparison session (384 records) is `verified` (the
runner checks the recovered log externally), and `verify --replay-all` on
the smoke session reproduced 30/30.

**Paired comparison** ([`cmp-lanes.json`](20261112-rust-deep-audit/cmp-lanes.json):
prime_search 26, 28, 30, 32 bits, 8 targets each, 3 rounds, warm-up 1;
session `ECBS1h462792189a53`; `ecbench compare --a rho-neg --b lanesN`),
**[pcc:unverified]**:

| arm | mean S | ops ratio to rho.negation [95%] | §5a band (w = 0 / 0.1 / 1) | wall ns/GAE ratio [95%] | end-to-end wall ratio [95%] | mean solve wall |
|---|---:|---|---|---|---|---:|
| rho.negation | 0.951 | 1 | — | 1 | 1 | 3.73 ms |
| lanes = 8 | 0.992 | 1.043 [1.028, 1.058] | 1.04 / 1.23 / 2.06 | 0.731 [0.708, 0.754] | 0.761 [0.735, 0.788] | 2.84 ms |
| lanes = 32 | 1.092 | 1.149 [1.100, 1.200] | 1.15 / 1.35 / 2.27 | 0.558 [0.545, 0.572] | 0.639 [0.608, 0.670] | 2.38 ms |

(Comparisons `ECC1hd4bc545469fc`, `ECC1h07cb952349be`; measurement quality
0/100, L0, no admitted A/A: these are not admissible wall results.) The
lanes cost more group operations at these sizes: when the solving collision
arrives, the other lanes' unfinished walks (about `lanes × 2^dp` steps,
`dp = ⌊bits(r)/4⌋`) are already paid, a share that falls as `r` grows. The
band ranks `rho.negation` below both at every weight, because each lane step
adds two Montgomery conversions to the unpriced work. The wall per group
operation is 0.56–0.73× [pcc:unverified]. Single 32-bit target, solve wall
ABAB ×5 × 20 runs: 240/257 → 128/132 ms (load 3.4) [pcc:unverified].
Callgrind on that input: 57.1 M Ir (83,675 GAE) → 64.6 M Ir (80,612 GAE);
the walk logic is now the largest share (`rho_walk_lanes` 42%, the adder
closure 26%, `add_many_batched` 16%, `batch_inv` 11%), so the next gain is in
the lane bookkeeping and in keeping points in Montgomery form across steps.

### `rootfind-mitm-prime-batch` (`5e242a3128`)

`PairedAdd::add_many_batched` (default: `add_many`) is overridden for
`PrimeCurve` with `prime_fast::FastCurve::add_many_batched` (one inversion
per block of up to 32 row additions, `BatchAddScratch` reused through a
thread-local); `RootfindMitmOracle::new_prime_batch` routes `row_points`
through it, and `solve_ic_prime` accepts `rootfind-mitm-prime-batch:m=2|3`.
`rootfind-mitm`, `rootfind-pairs` and every binary path still call
`add_many` (the gates above are identical with binary `c`). Unit test (18-bit
prime-abscissa base, 96 targets): identical witnesses, charged additions,
doublings and lookups; fewer than a quarter of the inversions.

`ic.pipeline` records do not carry a field tally, so the new oracle's
records equal its parent's in every counted unit; only the method id and
wall differ. Paired comparison
([`cmp-rfbatch.json`](20261112-rust-deep-audit/cmp-rfbatch.json): prime_search
24, 26, 28 bits, `prime-abscissa:size=48`, structured-gauss, 4 targets, 3
rounds; session `ECBS1hec91af1dff11`, comparison `ECC1h12f9902c1e5e`),
**[pcc:unverified]**: ops ratio 1.0000 [1.000, 1.000] (36 pairs, every `S`
equal); wall ns/GAE ratio 0.486 [0.468, 0.504]; mean solve wall 342.9 →
166.5 ms. Child process wall, 24-bit size 32, ABAB ×5: 76/84 → 39/50 ms
(load 3.6). Callgrind on that input: 311.9 M → 371.7 M Ir (+19%) while the
wall halves: the removed work is hardware divisions, which Callgrind counts
as one instruction each.

## Not done, and why

- **R4** IncrementalGauss kernel: 0.04% of the profiled IC solve (above).
- **R6-A** division-free `CountedFp`: measured about 5% at best and needs a
  `PrimeCurve` layout change (above). `rho.frozen_reference` untouched.
- **R9** `pollard_collab` walk on fixed limbs: no ecbench path and no small
  CLI to profile it in this session; a bit-identical rewrite of
  `walk::step`/`step_batch` (BigUint `Point`/`FieldElement` with the
  modulus cloned per element) is an M–L change that needs its own test
  against shared DP records. Left for a dedicated round.
- **R10** Wiedemann transpose-gather: reachable only for group orders
  ≥ 2^64; no workload here exercises it, so a reordering could not be
  compared against the old code.
- **R12** `prime_fast::rho_parallel` DP store: not on any profiled ecbench
  path (the assessment's estimate was under 5%).
- IC guard-set pre-sizing and the per-phase clock: discussed in Phase A.

## Reproduce

```bash
cargo build --release --bin ecbench          # at each commit; keep the binary
S=/path/to/work docs/perf/20261112-rust-deep-audit/gate.sh BIN LABEL   # after a base-* run of r2-base
ecbench profile-input --spec docs/perf/20261112-rust-deep-audit/profile-specs/X.json --seq 0 > X.in
docs/perf/20261112-rust-deep-audit/nodump.sh BIN BIN.nodump            # profiling copy only
valgrind --tool=callgrind --combine-dumps=yes --callgrind-out-file=cg.X BIN.nodump exec < X.in
docs/perf/20261112-rust-deep-audit/cg-top.sh cg.X 12
ecbench run --spec docs/perf/20261112-rust-deep-audit/cmp-lanes.json --out D --cpus none --allow-busy
ecbench compare --dir D --a rho-neg --b lanes32
```
