# Rust memory pass (round 3): allocation profile first, then WALL fixes

Third round after [`20261110-wall-only-rust-fixes.md`](20261110-wall-only-rust-fixes.md)
and [`20261112-rust-deep-audit.md`](20261112-rust-deep-audit.md), with the
emphasis on memory handling. Branch `claude/nice-babbage-bn0fn0`, baseline
`e9d5dbdccd`. Rules: AGENTS.md §10 and "Implementation language: no Python".
Only WALL changes are allowed: every value, counted unit, RNG draw, output
and replay stays the same, no method id changes, and nothing new is added
behind an option. Every commit below is WALL only.

Every wall figure below is **local, unisolated, [pcc:unverified]**. The host
is one shared 4-vCPU VM. Runs used `--cpus none`, with no
`tools/isolated_bench.py` and no EKS. Other agents' builds, `cppcheck` and
`clang-tidy` ran beside them, and the 1-minute load average is recorded with
each timing. Valgrind counts (dhat, massif, Callgrind) are deterministic for
a given input, but they were not filed as pcc claims either:
**[pcc:unverified]**.

## Host and binaries

- Intel Xeon @ 2.10 GHz (Sapphire Rapids flags, as in round 2), 4 vCPU,
  15 GB, Linux 6.18.44, rustc 1.97.0 (2d8144b78 2026-07-07), valgrind
  3.22.0, `[profile.release]` unchanged. There is no PMU and no
  `/usr/bin/time`. Peak RSS is ecbench's own `time.max_rss_kib`: the
  measured child's `wait4` rusage `ru_maxrss`.
- Binaries were built with `cargo build --release --bin ecbench` at each
  commit ([`binaries.sha256`](20261112-rust-memory-pass/binaries.sha256)):
  `base` = `e9d5dbdccd`, built before the first edit. The others are
  `f1` = `d8b75f5928`, `t1` = `c30ecbd77e`, `f2` = `86e1510ab9`,
  `t2` = `4c6edcf6ad` and `t3` = `e827d07cf1`. Each binary holds every
  earlier commit. Another agent's `gpu/` commits are interleaved on the
  branch; they do not enter the `ecbench` binary.

## Phase A: memory profile before editing

Method. `ecbench profile-input --spec S --seq 0` exports the measured-child
input of a one-arm spec. The child `ecbench exec < input` ran with
`RAYON_NUM_THREADS=1` under `valgrind --tool=dhat` and `--tool=massif`
([`run-prof.sh`](20261112-rust-memory-pass/run-prof.sh)).
[`dhsum.sh`](20261112-rust-memory-pass/dhsum.sh) (jq) sums the dhat JSON:
- totals;
- *short-lived* blocks, whose site's mean lifetime is under 500 instructions
  (dhat's own threshold) or under 100k instructions;
- allocation sites by count and by bytes, where a site is the first frame
  outside the allocator, `alloc::`, `hashbrown::` and `core::`.

Callgrind ran on a copy of each binary with `ic_measurement::dump` patched
to `ret` (round 2's [`nodump.sh`](20261112-rust-deep-audit/nodump.sh); the
measured binaries were never patched).
[`cg.sh`](20261112-rust-memory-pass/cg.sh) sums the parts.

The specs are round 2's [`profile-specs/`](20261112-rust-deep-audit/profile-specs),
plus [`rho-neg24.json`](20261112-rust-memory-pass/specs/rho-neg24.json) and
[`tensor12.json`](20261112-rust-memory-pass/specs/tensor12.json)
(`ic.tensor_contraction_v1`, `m=2`, `prime_search` 12-bit). Each has one
target, `target_seed` 20261112.

Every process pays the CLI and spec parse: about 1,950 blocks and 0.77 MB
allocated, with a heap peak of 348,349 B. That is `clap` command
construction, `serde_json` and `/proc` reads. Arms whose peak equals that
figure allocate less than it during the solve.

| arm (spec) | solve wall (native) | peak heap, dhat t-gmax | massif peak | total allocated | blocks | short-lived < 500 instr (blocks) | peak RSS `max_rss_kib` | minor faults |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| rho.negation 24-bit (`rho-neg24`) | 0.45 ms | 348,349 B (startup) | 345,664 B | 773,492 B | 2,010 | 668 | 14,116 | 421 |
| rho.negation 32-bit (`rho-neg32`) | 12.9 ms | 348,349 B (startup) | 345,664 B | 774,456 B | 2,013 | 667 | 13,844 | 415 |
| kangaroo.vow 32-bit (`kangaroo32`) | 21.4 ms | 348,349 B (startup) | 345,664 B | 813,242 B | 1,969 | 665 | 14,012 | 424 |
| bsgs.textbook 32-bit (`bsgs32`) | 16.0 ms | 2,238,883 B | 2,238,883 B | 2,938,786 B | 1,950 | 667 | 14,980 | 948 |
| ic.pipeline mitm + incremental-gauss, 32-bit, size 256 (`ic-igauss32`) | 2,688 ms | **115,867,255 B** | 115,867,255 B | 154,488,145 B | 4,000 | 846 | **129,084** | 37,911 |
| rootfind-mitm prime 24-bit, size 32 (`rf-mitm-prime`) | 72.6 ms | 348,349 B (startup) | 345,664 B | 1,089,011 B | 3,201 | 707 | 16,952 | 499 |
| rootfind-mitm Koblitz `a=0, n=19` (`rf-mitm-k19`) | 3.1 ms | 348,349 B (startup) | 345,664 B | 1,040,729 B | 3,414 | 536 | 18,768 | 491 |
| descent-algebraic f4-f2 Koblitz `n=13` (`da-f4-k13`) | 28.9 ms | 459,482 B | 459,482 B | 38,736,742 B | **205,752** | 35,593 | 20,792 | 647 |
| ic.tensor_contraction_v1 12-bit (`tensor12`) | 3,679 ms | 1,556,423 B | 1,547,101 B | **2,129,766,847 B** | **32,757,506** | **6,394,432** | 14,960 | 52,468 |

Top allocation sites in the solve (all sites:
[`dhat-summaries.txt`](20261112-rust-memory-pass/dhat-summaries.txt)):

| arm | by count (blocks; mean life in instructions) | by bytes |
|---|---|---|
| rho.negation 24/32 | `rho_search_from`: 2 blocks | `rho_search_from` 51,323 B (the distinguished-point table) |
| kangaroo.vow | `kangaroo_with`: 14 | 100,338 B (DP table) |
| bsgs.textbook | `bsgs_textbook`: 1 | 2,228,240 B (the baby-step `FastMap<u64>`, presized to `m = ⌈√r⌉`) |
| ic mitm + igauss 32 | `prime_factor_base` 311; `collect_and_solve_relations_scaled_touching` 240 | `collect_and_solve…`: **150,847,648 B in 10 blocks, 113,246,240 B live at the peak**. These are the guard set's `reserve_rehash` (stack below), plus 2,228,240 B of `PairTable::build_negation_folded`. |
| rootfind-mitm prime / Koblitz | `F2mElement::to_biguint` 198 (life 174), `flt_inverse` 166 (Koblitz setup) | `collect_and_solve…` 160,981 / 73,744 B |
| descent f4-f2 n=13 | `F2BoolPoly::mul_mono` 65,932; **`sort_masks_descending` 34,874 (mean life 537)**; `Columns::pack` 33,830; `groebner_basis_f4` 21,580; `Columns::unpack` 10,703 (life 584); `State::insert_core` 9,816 | `groebner_basis_f4` 12.1 MB, `insert_core` 5.5 MB, `mul_mono` 4.3 MB, `Columns::from_monomials` 4.3 MB (261 dense column indices of 16 KiB) |
| tensor 12-bit | **`SparseContractor::count` 20,687,875** (row keys, 1.83 GB); **`opcount::bump` 6,694,543** (a `format!` key per tally bump); `Curve::encode_point` 3,331,808; **`relation_network` closure#3 1,965,414 (life 172)**, the target predicate building a key per row; `Vertex::edges` 41,330 (life 493) | `count` 1.83 GB, `bump` 152 MB, `encode_point` 83 MB |

The IC peak, by dhat stack: `malloc ← RawTableInner::fallible_with_capacity ←
RawTable<(u64, ())>::reserve_rehash ← HashMap<u64, (), MixHasher>::insert ←
collect_and_solve_relations_scaled_touching`. The run made 4,554,363 guard
probes, so the set ends at 2^23 buckets (9 B each, 75.5 MB). Its peak is
the last doubling, with the old table (37.7 MB) and the new one (75.5 MB)
live together.

Callgrind at baseline, for the two churn arms
([`callgrind-tops.md`](20261112-rust-memory-pass/callgrind-tops.md)):
- descent n=13: 190.8 M Ir. Of that, 49.6 M (26 %) is in `malloc`, `free`,
  `realloc` and the glibc internals behind them, and 9.1 M in `memset`.
  Round 2 measured the same split.
- tensor 12-bit: 43.6 G Ir. Of that, 7.8 G (17.9 %) is the malloc family
  and 16.2 G (37 %) is SipHash (`DefaultHasher::write` 18.9 %,
  `RandomState::hash_one<&usize>` 11.0 %, `hash_one<&Vec<u8>>` 4.7 %). The
  SipHash comes from the contractor's `HashMap<Vec<u8>, u64>` rows, its
  `pos` map and the `fixed: &HashMap<usize, bool>` lookups.

Structural scan (rg over `pq_f4_f2`, `pq_groebner_f2`, `ic_boundary`,
`ic_framework`, `ecbench/generic.rs`, `crates/ecdl-tensor-ic`):
- **`collect` only to iterate, and key buffers.** `sort_masks_descending`
  allocates a key `Vec` on every call; `mul_mono` allocates a product
  buffer plus an output buffer; `from_monos` a mask buffer plus an output
  buffer.
- **`Vec::new()` + push with a known length.** `Columns::unpack` (the count
  `set` is computed first).
- **Per-step `Vec`/`FxSet` in a loop.** `groebner_basis_f4` (the pair
  partition, `seen_rows`, `half_keys`, `field_keys`, `lcm_columns`,
  `active`) and `State::insert_core` (`active`).
- **`.clone()` of keys in a loop.** The tensor contractor clones every row
  key that carries into the next table (`key.clone()` + `extend` = two
  allocations per row).
- **`String` formatting in a hot path.** `ecdl_tensor_ic::opcount::bump`.
- **`HashMap` growth without reserve.** The IC guard (no known bound; see
  below) and the tensor row tables (output size not known before the join).
- **Dense where sparse would do.** No hot case found.
  `Columns::from_monomials` builds a dense `u32` index over `2^span` masks
  (16 KiB at 12 variables), which is already capped at 22 variables.
- **Element widths.** The BSGS table's value `j < 2^32` could be `u32`, but
  `(u64, u32)` pads to 16 B like `(u64, u64)`, so nothing is gained. The IC
  guard keys reach 34 bits at 32-bit `p` (`((x+1)<<1)|sign`), so they do not
  fit `u32`. `PairTable` already has packed `u32`/`u16` forms.
- **Unbounded caches.** None on these paths. `Box<dyn …>` is created once
  per solve (walk adders), never per step.

## Reported memory fields

These ecbench record fields measure memory:
- `time.max_rss_kib` and `time.minor_faults`. They are measurements, and the
  normaliser [`norm-records.sh`](20261110-wall-only-rust-fixes/norm-records.sh)
  already drops the whole `time` object. They differ run to run even with
  the same binary (the tensor arm's minor faults ranged from 36,354 to
  107,559 across six binaries,
  [`rss-minflt.tsv`](20261112-rust-memory-pass/rss-minflt.tsv)). No commit
  needed a new normalisation.
- `solver.peak_bytes` (F4's `peak_matrix_bytes`, packed words) and
  `solver.extra.pair_dense_scratch_bytes_max`. Both are logical sizes, not
  allocator readings. They stayed **identical** in every record, because the
  normalised records are identical; F2 keeps its new buffer outside the
  dense scratch for this reason.

Peak RSS before and after, single runs from
[`rss-minflt.tsv`](20261112-rust-memory-pass/rss-minflt.tsv): `ic-igauss32`
129,084 → 128,932 KiB, `da-f4-k13` 20,792 → 20,804 KiB, `tensor12`
14,960 → 13,956 KiB. Every arm stayed within ±2 MiB, which is the run-to-run
spread.

## Phase B: WALL-only fixes

Verification for each commit:
- the three round-1 gate specs ([`gate-prime.json`](20261110-wall-only-rust-fixes/gate-prime.json)
  36 records, [`gate-koblitz.json`](20261110-wall-only-rust-fixes/gate-koblitz.json)
  16, [`gate-tensor.json`](20261110-wall-only-rust-fixes/gate-tensor.json)
  4), base against the candidate, `records.jsonl` compared after
  `norm-records.sh`, then `ecbench verify --replay-all`
  ([`gate.sh`](20261112-rust-deep-audit/gate.sh));
- the same comparison on all nine Phase A specs
  ([`ident.sh`](20261112-rust-memory-pass/ident.sh));
- the touched modules' tests;
- `rustfmt --edition 2021 --check` on every changed file.

Results: [`gate-results.txt`](20261112-rust-memory-pass/gate-results.txt).
The base hashes are `f81b9eb603435717`, `18257ae507917e7d` and
`54a61ad4915a6b19`, the same as round 2's.

| binary | gate-prime (36) | gate-koblitz (16) | gate-tensor (4) | replays | 9 Phase A specs |
|---|---|---|---|---|---|
| f1 `d8b75f5928` | identical | identical | identical | 36/36, 16/16, 4/4, `ok: true` | 9/9 identical, replays ok |
| t1 `c30ecbd77e` | identical | identical | identical | 36/36, 16/16, 4/4, `ok: true` | 9/9 identical, replays ok |
| f2 `86e1510ab9` | identical | identical | identical | 36/36, 16/16, 4/4, `ok: true` | 9/9 identical, replays ok |
| t2 `4c6edcf6ad` | identical | identical | identical | 36/36, 16/16, 4/4, `ok: true` | 9/9 identical, replays ok |
| t3 `e827d07cf1` | identical | identical | identical | 36/36, 16/16, 4/4, `ok: true` | 9/9 identical, replays ok |

Tests:
- `cargo test --release --lib` filters on the F1 tree: `pq_groebner_f2` 21,
  `pq_f4_f2` 13, `inherited_f4` 16 (+2 ignored), `partial_enlargement` 4,
  `ic_framework::solvers` 13, `koblitz_groebner` 93 (+5 ignored). All pass.
- On the final tree (covers F2): `pq_f4_f2` 13, `pq_groebner_f2` 21,
  `ic_framework::solvers` 13, `inherited_f4` 16 (+2 ignored). All pass.
- `cargo test --release -p ecdl-tensor-ic`, on the T1+T2 tree, the T2 tree
  and the T3 tree: 63 tests, all pass.

### Evidence

Wall figures come from [`solveab.sh`](20261112-rust-deep-audit/solveab.sh):
5 alternating rounds of A and B, where a round sums the child's
`report.solve_wall_ns` over N runs. N = 20 for the descent arm and N = 2 for
the tensor arm. Raw output with load averages:
[`wall-abab.txt`](20261112-rust-memory-pass/wall-abab.txt).

The A/A noise floor (same binary as both A and B):
- descent: 481/500 against 488/534 ms;
- tensor: 7,119/7,384 against 7,014/7,321 ms.

| # | commit | change | dhat blocks / total bytes / peak heap (before → after) | Callgrind Ir (before → after) | wall min/median, A → B | load (1-min) |
|---|---|---|---|---|---|---|
| F1 | `d8b75f5928` | `pq_groebner_f2::sort_masks_descending` keys masks in place: the packed key is injective, fits the mask's slot and keeps its order under `wide-vars`. `mul_mono` and `from_monos` sort with a new `sort_monos_descending` and cancel pairs in place (`cancel_adjacent_pairs`), in one buffer. `Columns::unpack` reserves its exact term count. | descent n=13: 205,752 → **125,616 (−39 %)**; 38.7 → 31.5 MB; peak 459,482 → 459,482 B. Short-lived (< 500 instr) blocks 35,593 → 8,283. | 190.8 M → 175.7 M (−7.9 %); malloc family 49.6 M → 36.6 M | 478/501 → 449/486 ms (inside the A/A spread) | 3.1–3.3 |
| T1 | `c30ecbd77e` | `ecdl_tensor_ic::opcount::bump` builds `<phase>.<name>` in a scratch `String` kept in the tally and clones it only for a new entry. The map's keys and counts are unchanged. | tensor 12-bit: 32,757,506 → **26,062,977 (−20 %)**; 2.130 → 1.978 GB; peak 1,556,423 → 1,556,396 B | 43.62 G → 40.27 G (−7.7 %) | 6,889/7,224 → 6,299/6,755 ms | 3.4–4.0 |
| F2 | `86e1510ab9` | `groebner_basis_f4` partitions the pair queue in place (`retain` into a reused buffer; both sides keep their order). It keeps `seen_rows`, `half_keys`, `field_keys`, `lcm_columns` and `active` across steps, cleared each step. `insert_core` keeps its active list in `PairSelectScratch`. `lcm_columns` is read by membership and through `examined`, whose values `Columns` sorts, so iteration order cannot matter. | descent n=13: 125,616 → **115,774 (−7.8 %)**; 31.5 → 28.9 MB; peak 459,482 → **475,466 B (+16 KB, the retained per-step buffers)** | 175.7 M → 170.8 M (−2.8 %); malloc family 36.6 M → 33.4 M | 468/472 → 459/492 ms (**no measurable difference**) | 3.3 |
| T2 | `4c6edcf6ad` | The `SparseContractor` table operations consume their input table. A row key that carries over is moved and extended in place (`join_support` clones only for fan-out beyond the last match). `sum_out` clones a projected key only when it starts a new row. The target predicate compares bits with the stored key (`bits_match_key`) instead of building a key per row. `count` walks vertex edges without a copy (`Vertex::edge_iter`). | tensor 12-bit: 26,062,977 → **12,795,049 (−51 %)**; 1.978 → 1.548 GB; peak 1,556,396 → **1,247,454 B (−20 %)**. Short-lived blocks 1,991,143 → 1,479. | 40.27 G → 37.56 G (−6.7 %); malloc family 6.34 G → 4.12 G | 6,719/6,889 → 6,341/6,627 ms | 2.9–3.4 |
| T3 | `e827d07cf1` | The contractor's private tables (rows, edge positions, support groups) hash with an FxHash word mix and a murmur3 `fmix64` finish (`RowHasher`) instead of SipHash with a random key. `count` reads fixed bits from a dense per-edge `Vec<Option<bool>>`, built once per call, instead of one `HashMap` lookup per row and edge. The `Counter` API is unchanged. | tensor 12-bit: 12,795,049 → 12,803,132 (+8,083: one `fixed_at` vector per `count` call); bytes and peak unchanged (1,247,529 B) | 37.56 G → **21.91 G (−41.7 %)**; SipHash 17.6 G → 0.10 G | 6,411/6,759 → **4,798/4,987 ms** | 1.7–2.9 |

Cumulative, base → t3, local and unisolated, [pcc:unverified]:
- tensor 12-bit: 32.76 M → 12.80 M allocations (−61 %), 2.13 → 1.55 GB
  allocated, heap peak 1.56 → 1.25 MB, 43.6 G → 21.9 G Ir. Solve wall
  7,055/7,624 → 4,596/4,930 ms (two solves a round, load 1.8–2.2).
- descent n=13: 205,752 → 115,774 allocations (−44 %), 38.7 → 28.9 MB,
  190.8 M → 170.8 M Ir. Solve wall 485/527 → 434/479 ms (20 solves a
  round, load 2.1), a difference the same size as the A/A spread.

The T2 row also shows SipHash Ir going up (16.2 G → 17.6 G). `sum_out` now
hashes a new projected key twice: once in `get_mut`, once in `insert`. T3
removes that cost along with the rest of SipHash.

Why the tensor changes cannot change a value: `SparseContractor::count`
returns a wrapping sum of row counts. Its only other outputs are
`contraction_rows_uncharged` (`out.rows.len()` per operation) and
`peak_rows`. All three are independent of the order in which a `HashMap`
yields rows, and that order was already random per process under
`RandomState`. The crate's doc comment in `opcount.rs` states the same
invariant. Moving a key instead of cloning it leaves every table's contents
unchanged.

## Not done, and why

- **IC guard-set pre-sizing or growth policy.** Checked first: no output
  depends on the guard's iteration order. It is used only through
  `RelationGuard::insert_duplicate`; the loop is generic over the trait and
  never iterates the set. So a growth-policy change would be legal. There
  is still no safe size bound:
  - `max_trials` is 10^8 by default;
  - the number of distinct keys is bounded only by the group;
  - the relation yield per trial is not known before the run.

  Sizing to a guess would raise the peak of every run that ends early.
  Growing ×4 instead of ×2 would halve the rehash work, but it could leave
  the table up to 4× oversized. A denser exact set does not pay off either:
  `GuardLineSet` at 3/4 load is about 10.7 B/key, against hashbrown's
  about 10.3 B/key at 4.55 M keys. It would cut the transient peak from
  113 MB to about 96 MB, at the cost of switching the default path. Left
  unchanged; the numbers above are the record.
- **`Trace::boundary` cost.** `mark` already skips same-phase boundaries.
  What remains is two clock reads per trial (`TargetQuery` ↔ `TargetPdp`),
  and those are the `online.phases_ns` measurement itself.
- **F4 rows and products.** `Columns::pack` makes 33,830 allocations, one
  `Vec<u64>` per matrix row; `mul_mono` makes 32,966, one per product
  polynomial. Removing them needs a contiguous row arena through `echelon`,
  `echelon_block_tables` and the M4RI paths, or inline small-vector
  storage (no such dependency in the tree). That refactor is too wide for
  a WALL round. The three `MonomialSeen` bitmaps per step were left too:
  their clearing is charged to `bitop/v1` at construction, and reusing them
  would put that charge in a second place.
- **Tensor `Curve::encode_point`.** It makes 3.3 M allocations, one per row
  of a function vertex. The cause is the public `FuncFn` signature
  (`Fn(&[bool]) -> Option<Vec<bool>>`), so removing them is an API change.
- **Tensor row tables `reserve`.** This is 5.7 % of Ir after T3. The output
  size is bounded by the input for `apply_func` and `apply_pred`, but a
  predicate keeps a small fraction of the rows. Reserving the bound would
  raise the heap peak in a memory pass.
- **Allocator swap** (mimalloc or jemalloc). It would need a new dependency
  and changes every arm's RSS; this round adds no dependencies.
- **R6-A.** Round 2 measured about 5 % at best and rejected it. The rho
  arms allocate 2 blocks in the solve, so the memory profile gives no new
  reason to revisit it.
- **R9 `pollard_collab` BigUint walk.** It still has no ecbench path, so
  this profile does not see it.
- **R12 `prime_fast::rho_parallel` DP store.** It is not on any profiled
  ecbench path.

## Reproduce

```bash
S=/path/to/work; cd $S; mkdir m-prof m-rss m-gate m-specs
cargo build --release --bin ecbench                      # at each commit; keep the binary as $S/m-LABEL-ecbench
ecbench profile-input --spec SPEC --seq 0 > m-prof/NAME.in
(cd m-prof && docs/perf/20261112-rust-memory-pass/run-prof.sh BIN LABEL NAME...)   # dhat + massif
docs/perf/20261112-rust-memory-pass/dhsum.sh m-prof/dh.LABEL.NAME.json 8
docs/perf/20261112-rust-deep-audit/nodump.sh BIN $S/m-LABEL-nodump && S=$S docs/perf/20261112-rust-memory-pass/cg.sh LABEL NAME
S=$S/m-gate docs/perf/20261112-rust-deep-audit/gate.sh BIN LABEL           # after base-* runs of the base binary
S=$S docs/perf/20261112-rust-memory-pass/ident.sh BIN LABEL                # after base-* runs over the Phase A specs
docs/perf/20261112-rust-deep-audit/solveab.sh LABEL BIN_A IN BIN_B IN N
```
