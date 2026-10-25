# The `ic.pipeline` oracle set-up: a cache-blocked pair table, |F| = 1,024 to 16,384

Measured 2026-10-20 under [`PROTOCOL.md`](PROTOCOL.md), frozen in `e59331c96`
with the code, before any recorded session. PDF: [`REPORT.pdf`](REPORT.pdf). Run
reports: [`reports/run-20261020-A/`](reports/run-20261020-A/report.pdf) (session A),
[`reports/run-20261020-B/`](reports/run-20261020-B/report.pdf) (session B).

`research/ic_linalg_pipeline_20261019` found the oracle's |F|² pair-table set-up
to be 84–89% of an `ic.pipeline` run's wall time at |F| ≥ 8,192. This study adds a
third builder for that table, `mitm-prime-blocked`, and measures it against the two
that already existed.

## Result

**For `S`, nothing, by construction.** All three builders make the same table:
the same keys, the same witness for every key, the same probe, and the same counted
operations (one abstract addition per folded pair, about |F|²). On every target,
round and size the three arms recovered the same logarithm with the same counts in
every phase: **456 of 456 executions identical**, warm-ups included (measure 1).
`S` is identical at w = 0, 0.1 and 1 (`ecbench band`: unresolved, i.e. tied, inside
the band). This is a component speed-up, not an advance (AGENTS.md §3).

**For the set-up's wall time, 2–4× against the batched builder and 4–8× against
the scalar one:**

| N | set-up scalar | set-up batched | set-up blocked | blocked / batched, Σ [pair range] | blocked / scalar, Σ [pair range] | whole run blocked / scalar | peak RSS batched → blocked |
|--:|--:|--:|--:|:--|:--|--:|:--|
| 1,024 | 216 ms | 117 ms | 55 ms | 0.47 [0.32, 0.79] | 0.25 [0.15, 0.43] | 0.91 | 0.09 → 0.08 GiB |
| 2,048 | 1.44 s | 739 ms | 238 ms | 0.32 [0.19, 0.44] | 0.17 [0.13, 0.22] | 0.38 | 0.19 → 0.12 GiB |
| 4,096 | 8.40 s | 3.82 s | 1.04 s | 0.27 [0.23, 0.32] | 0.12 [0.10, 0.15] | 0.16 | 0.63 → 0.34 GiB |
| 8,192 | 43.6 s | 20.5 s | 5.29 s | 0.26 [0.24, 0.31] | 0.12 [0.11, 0.15] | 0.13 | 2.37 → 1.25 GiB |
| 16,384 | 224 s | 117 s | 28.2 s | 0.24 [0.22, 0.26] | 0.13 [0.12, 0.14] | 0.13 | 9.15 → 4.65 GiB |

Wall times are means per run; ratios are Σ blocked / Σ other over (target, round)
pairs, 40 per N (8 at 16,384), with the range of the per-pair ratios. Peak RSS is
the whole process's maximum over runs, not isolated table memory.
A/A control (`blocked-4096-aa` against `blocked-4096`): identical counts, set-up
wall ratio 1.02, per-pair range 0.70–1.46.

- **Where the time went.** The batched arithmetic alone costs about 20 ns per sum
  on this host (diagnostic `profile_prime_pair_table_build_layouts`). The batched
  builder spent another 90–415 ns per sum inserting into one hash map that grows
  past the caches (2.4 GB at 8,192). The blocked builder packs an entry into one
  word (p < 2³², at most 2¹⁶ signed points) and fills cache-sized open-addressed
  regions one at a time from their gathered sums: 53–105 ns per table addition
  across the sizes, against 112–434 ns batched and 206–835 ns scalar
  (`figures/setup_ns_per_entry`).
- **From N = 2,048 up**, every per-pair set-up ratio lies below the A/A range,
  against both other builders. At 1,024 the blocked/batched range (0.32–0.79)
  overlaps it.
- **Every wall figure is descriptive.** Most IC runs earned L1, not L2 (hypervisor
  steal on the run CPU, as in the previous study): 1–11 of 40 runs per arm reached
  L2 at N ≤ 4,096 and none above. No comparison had the 5 admitted (both-L2) pairs
  ecbench needs, so every wall comparison is `descriptive` (`comparisons/*.json`).
- **End to end**, the whole run is 7.5× faster than scalar at 8,192 and 16,384,
  because set-up was most of it. At N = 1,024 the relations phase (about 0.8 s) is
  most of a run, and the whole-run ratio is 0.91 against scalar and 1.03 against
  batched.

## Distance remaining to rho

`ops` ratio `S_blocked / S_rho` (ecbench cluster 95% intervals); every builder's
is identical:

| N | 1,024 | 2,048 | 4,096 | 8,192 | 16,384 |
|:--|--:|--:|--:|--:|--:|
| w = 0 | 33.0 [27.7, 40.6] | 74.3 [63.7, 90.3] | 269 [231, 326] | 1,062 [912, 1,287] | 5,110 [3,374, 9,867] |
| w = 1 | 33.0 | 45.4 | 139 | 535 | 2,567 |

The smallest gap is at N = 1,024, where the table is half of `S` (1.05M of 2.09M
GAE). It has not moved: this study changes no count.

These gaps are smaller than the previous study's (best 149× at 2,048). That is not
this change. The previous study used the full-rank stop rule, which needed about
4.75 relations per column; this one uses `incremental-gauss`, the default
target-pinned rule that the leaderboard's prime ladder already uses, which stopped
at about 0.5 per column (533 relations at N = 1,024). It is the baseline, not an
improvement over it.

## Predictions (PROTOCOL.md)

- P1 (every equality holds; `S` identical across the band): **held**, 456/456.
- P2 (blocked/batched set-up ≤ 0.5 at N ≥ 4,096): **held, descriptively**: 0.27,
  0.26, 0.24.
- P3 (blocked/scalar set-up ≤ 0.25 at N ≥ 4,096): **held, descriptively**: 0.12,
  0.12, 0.13.
- P4 (smaller gain at 1,024, blocked/batched ≤ 1.0): **held**: 0.47.
- P5 (blocked peak memory ≤ batched at N ≥ 4,096): **held**: about half.
- P6 (no counted change; smallest gap at 1,024): **held**.

## Classification and decision

Engineering, a component (phase) speed-up (AGENTS.md §3). The pair table builds
2–4× faster than with the batched builder and 4–8× faster than with the scalar one,
at half the memory, on identical output. `S` and the gap to rho do not move. The
set-up's counted cost is already at its floor for this method: one charged addition
per table key, with the negation fold, and at the best N it is half of `S`. A
smaller `S` needs fewer charged operations (fewer table entries or fewer
relations per column), not a faster table.

- `mitm-prime-blocked:m=2` is the fastest prime-field `ic.pipeline` oracle to date
  for wall time. The existing oracles, and every recorded session, are unchanged.
- Leaderboard: a pair-table set-up section outside tables A–C
  (`scripts/build_ic_leaderboard.py`, `pair_table_component`); no row of tables
  A–C moves. The same commit restores the linear-algebra component section, whose
  rendering the #1874 squash merge had dropped.

## Deviations from the protocol

- **Session B was interrupted and re-run.** The first session B
  (`sessions/B-16k`, 27 records) was killed by SIGTERM at run 27 of 48: its
  background job hit the operator's tool time limit (an operator error). Run 27
  (`scalar-16384`) is recorded as `crashed` by the signal; the 26 runs before it
  verified. It is kept unchanged and is not analysed. Session B was then re-run
  from the same frozen spec into `sessions/B-16k-r2` (48 records, 48 verified),
  from the same binary (`c1378033…41c5`). It recorded a later repository head
  (`855c0bc1f`); the commits after `e59331c96` change only research files and the
  leaderboard script, not the Rust source.
- The protocol did not fix a replay count. The audits replayed 24 of session A's
  672 records and 6 of session B-r2's 48 (`--replay`, not `--replay-all`).

## Evidence

- Specs: [`specs/A-1k-8k.json`](specs/A-1k-8k.json), [`specs/B-16k.json`](specs/B-16k.json).
- Sessions: `sessions/A-1k-8k` (`ECBS1h9e263e582e51`, 672 records, 672 verified)
  and `sessions/B-16k-r2` (`ECBS1h90cd7a0dc213`, 48 records, 48 verified);
  the interrupted `sessions/B-16k` (27 records). One binary for all three
  (`c1378033…41c5`, built from `e59331c96`, clean tree); each session records the
  repository head at its start (`e59331c96`, `855c0bc1f`, and `28faa7ca1` for
  the interrupted one), and no Rust source changed between them.
- Audits: `sessions/A-1k-8k.audit.json` (OK, 672 records, 24 replays reproduced,
  SHA-256 `9423e18737808afb4ae10d2275447b30ee8cd0f019e9883a1bd2841e114c250c`),
  `sessions/B-16k-r2.audit.json` (OK, 48 records, 6 replays reproduced, SHA-256
  `4663d45496f42b3cfeeeda4e8485d2549d36d77b156e8882a55f162006a44568`), and
  `sessions/B-16k.audit.json` (`--allow-interrupted`, integrity only: OK, 27
  records, 26 verified, 0 problems).
- Comparisons: `sessions/*/comparisons/` (ecbench `compare --save`). Tables:
  `analysis/table-A.md`, `analysis/table-B.md` (ecbench `table`); pricing band:
  `analysis/band-*.{txt,json}` (ecbench `band`).
- Derived figures: `analysis/A-1k-8k.json` and `analysis/B-16k.json` from
  `scripts/analyze.jq`, and `RESULT.json` from `scripts/result.jq` (jq over the
  records; no Python computation). Figures: `figures/*.svg` and `.png`, from
  `scripts/plots.py` (layout only).
- Code: `PairTable::build_negation_folded_prime_blocked`, `PackedPairTable` and
  `for_each_folded_prime_sum` in `src/cryptanalysis/ic_boundary.rs`; the oracle in
  `src/cryptanalysis/ic_framework/plugins.rs`. Test:
  `prime_blocked_table_matches_batch_table_across_blocks_and_parts` (both layouts,
  every key of a 20-bit field, held or not).
- Host: 4-vCPU Intel Xeon @ 2.80 GHz VM (AVX-512), Linux 6.18, 15 GiB, one NUMA
  node; `ecbench run --cpus auto`, run CPU 3 reserved.
