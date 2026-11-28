# 1D-1 rational-discriminant fast path: wall-clock A/B (Prime31, B = 64)

Status: **indicative CPU wall time on a contended cloud VM. Not a validated benchmark row.** [pcc:unverified]

## Comparison used
The generic path was NOT removed from merged main (`ee44af4289`): `PairProjection::build_generic`, `decompose_three_generic`, `decompose_four_generic` still exist next to `build_fast`, `decompose_three_fast_path`, `decompose_four_fast_path` / `decompose_four_first`. So the comparison is **fast vs generic inside one binary on current main**; no parent-commit checkout was needed. Existing tests (`fast_path_matches_generic_small`, `prime31_opcount_old_vs_new`) cover correctness; the harness re-asserts found/none equality and `compatible_keys` equality on all 100 targets.

## Workload
p = 2^31-1, y^2 = x^3+2x+3, B = 64 base points (2080 pairs incl. diagonal), 100 point targets (18 decompose_three hits, 1 decompose_four hit: most four-queries are exhaustive "none" scans), 21 interleaved repeats (order alternated per repeat), 3 warmup builds plus a full equivalence pass, pinned to one core (`taskset`), an A/A arm (generic vs generic) for the noise floor.

## Results (ms, median; range = min..max of 21)
| step | generic (old) | A/A (old again) | fast (new) | median ratio | min/min ratio |
|:--|:--|:--|:--|:--|:--|
| build, B=64 | 6.02 (5.33..14.7) | 9.57 (5.46..24.9) | 1.20 (0.98..5.74) | 5.0x | 5.4x |
| decompose_three x100 | 20.4 (12.3..45.8) | 17.2 (12.3..43.1) | 1.11 (1.09..5.13) | 18.5x | 11.4x |
| decompose_four x100 (fast_path) | 1398 (1153..1932) | 1432 (1165..1848) | 95.2 (66.8..141) | 14.7x | 17.3x |
| decompose_four x100 (first) | | | 83.7 (58.3..191) | 16.7x | |

Noise: host load average was about 14 on 4 cores (sibling builds). The A/A build medians differ by 1.6x, so the **build ratio is only good to roughly a factor 1.5; query ratios to about 20%**. The old and new ranges are nearly disjoint for every step (build: new max 5.74 vs old min 5.33), so the direction and the order of magnitude are real; the third digit is not.

## Cycles / IPC
No hardware PMU: no `perf` binary, no `cpu` event source in `/sys/bus/event_source/devices`, `perf_event_paranoid=2`, hypervisor flag set. Wall time only; no cycles or IPC are claimed.

## Relation to op counts
Existing op counts (build invm 4096 -> 1, mulm 12.5x) are unchanged and are not re-derived here (counters are `cfg(test)`). Build wall ratio (~5x) is smaller than the mulm ratio (12.5x), plausible since `build_fast` still pays B sqrtm and the Montgomery batch is cheaper per mul than `u128 %`; this is an observation, not a decomposition (no profile was taken).

## Caveats (read before citing)
- Built through a shim crate (`shim/README.md`): the full lib release build was OOM-killed twice; semaev_projection.rs itself is unmodified, imports are verbatim extracts except a stub `FieldElement` that the benchmark never calls.
- Single contended VM, not isolab; one binary, one host; no CPU-frequency control.
- Kernel microbenchmark of a sub-step. No complete DLP was solved, so there is **no S, no GAE, no matched rho** and this is not a speed claim against IC or rho.

## Needs a lab run (isolab-eks + S3), not produced here
1. Rerun on isolab-eks with CPU isolation, PMU (cycles, instructions, IPC), same harness against the real `crypto_lib`.
2. S3 upload of raw data (`benchrec.py upload`, needs AWS credentials; absent here) and a CI-closed `bench.record/v1` (needs a recorded command via `benchrec.py run`, a report PDF, S3 `uploaded`).
3. Independent replay certificate, signed (pcc claim).
4. Validator report recomputed independently from the raw data.
5. Matched-rho reference with the 0 / 0.1 / 1 GAE band: only meaningful for an end-to-end solve that uses this path.
6. Leaderboard / benchmarks README regeneration (gate forbids a row without 1-3).

## Files
`raw_wallclock.json` (all 21 samples per arm), `fig_old_vs_new_wallclock.svg`, `bench.record.scaffold.json` (open `bench.record/v1`, 3 metrics, raw hashed, not uploaded; kept out of `benchmarks/records/` because an open record fails CI), `VALIDATION.json` (skeleton, invalid by design), `shim/` (harness).
