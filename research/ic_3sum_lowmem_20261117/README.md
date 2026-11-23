# Does a lower-memory 3-sum oracle beat the O(B^2) pair table? (prime-field IC, m = 3)

**Date.** 2026-11-17. **Status: NULL / partial.** A validated lower-memory oracle
exists and recovers the identical relations, but it does not beat the pair
table in time, and it does not enable a useful larger `B`. A Schroeppel-Shamir
(sorted-list + priority-queue) merge is structurally inapplicable here. All
numbers are producer-side, single host (4 shared threads, other builds running,
not CPU-isolated), public synthetic known-answer instances. `[pcc:unverified]`.
No security claim, no vs-rho claim.

## Question

The m=3 path (`collect_relations` with `PairTable`, `ic_solve(summands=3)`)
decides `R = ±F_i ± F_j ± F_k` by sweeping `T = R ∓ F_i` (O(B) per target) and
looking `x(T)` up in a `B^2`-entry table of `x(F_j ± F_k)` (16 B/entry). Can an
`O(B)`-memory merge (Schroeppel-Shamir style) replace the table and allow a
larger `B` under a memory budget?

## Findings

1. **Schroeppel-Shamir does not apply.** Its heap merge needs a key `κ` with
   `κ(a+b)` monotone in `a` for fixed `b` (integers: the value; the only such
   key on a group is the discrete log, which is the unknown). Sorting the
   factor base by `x` and adding a fixed `F_b` gives an order-preserving
   adjacent pair in **0.50%** of cases (`monotone.jsonl`: 0.5023 / 0.4993 /
   0.4993 for B = 60 / 240 / 960): no structure to merge on. So the O(B)-memory
   heap algorithm cannot be built; this is a structural null, not a tuning
   failure.
2. **Two valid low-memory oracles were built and validated** (`examples/ic_3sum_lowmem.rs`):
   - `nested` (O(B) memory): per `(i, eps)` solve the 2-sum `T = ±F_j ± F_k`
     by an O(B) sweep. O(B^2) work per target.
   - `stream` (chunked meet-in-the-middle, O(chunk·B) memory): hold the sweep
     candidates `x(R_t ∓ F_i)` of a chunk of targets, stream every
     `x(F_j ± F_k)` once per chunk (never stored), probe.
3. **Validation (before any timing).** On a fixed target list the per-target set
   of merged, group-verified relation rows from `nested` and from `stream`
   (chunk sizes 1, 7, 64, 1000, N) equals the pair-table oracle exactly:
   24-bit B=60 (3000 targets), B=120 (3000), 28-bit B=240 (20000 targets,
   1363 rows; stream only, nested capped at B<=160): all `true`
   (`validate_*.json`). My copy of the table logic also reproduces the crate's
   adaptive `collect_relations` rows (`copy_vs_crate_ok`). Full solves with the
   stream oracle recover the planted log (`solve.jsonl`, `log_ok` true on all 6).
4. **`nested` (true O(B) memory) loses by a factor growing like B.** Same 600/
   300/100 targets, 28-bit: B=60 0.129 s vs table 0.0032 s (40x); B=120 0.197
   vs 0.0035 (56x); B=240 0.232 vs 0.0045 (52x) (min of 2). Matches
   O(B^2) vs O(B)/target.
5. **`stream` has a memory/time trade-off, and the table is on the frontier.**
   Time = (#chunks)·(B^2 pair sums) + the same O(B)/target sweep. 28-bit, the
   number of targets `n` is what the adaptive crate path needed (min of 3):

   | B | n | table s (struct MB) | stream chunk=n s (MB) | chunk=n/4 s (MB) | chunk=n/16 s (MB) |
   |--:|--:|--:|--:|--:|--:|
   | 480  | 900 | 0.0505 (3.7)   | 0.0646 (31.7) | 0.0693 (7.9) | 0.1557 (2.0) |
   | 960  | 220 | 0.0673 (14.7)  | 0.0767 (15.7) | 0.1564 (3.9) | 0.5921 (1.0) |
   | 1920 | 55  | 0.2347 (59.0)  | 0.1663 (7.8)  | 0.5259 (2.0) | 1.8368 (0.5, chunk 4) |
   | 3840 | 14  | 0.9452 (235.9) | 0.5668 (3.95) | 2.156 (1.05, chunk 4) | 7.163 (0.26, chunk 1) |

   (Stream "MB" = candidates + bucketed copy; at B=480 chunk=n it is larger
   than the table because there `n·2B > B^2`.) Halving memory costs about
   2-4x time (a full pair stream per chunk): `time ≈ p/T_c + p/B`,
   memory `T_c·B`; minimising at fixed memory M gives `p/√M`, i.e. the same as
   the table (`M + p/√M`). There is no asymptotic gain, only constants.
   With `O(B)` memory total time is `Ω(p)`.
6. **Where stream wins on memory it does not win on the solve.** For `B >
   ~p^{1/3}` the number of targets needed is so small (every target yields
   `~B^3/p` rows) that one stream pass uses 7.5x (B=1920) to 60x (B=3840) less
   structure memory than the table at equal or lower collect time (0.166 vs
   0.235 s; 0.567 vs 0.945 s). But the end-to-end solve is dominated by the
   dense `O(B^3)` linear algebra (`solve.jsonl`, `baseline_28.jsonl`):

   | B | crate table total s (VmHWM MB) | stream total s (VmHWM MB) | LA s |
   |--:|--:|--:|--:|
   | 960  | 0.099 (33.3)  | 0.104 (25.8) | 0.040 |
   | 1920 | 0.495 (126.0) | 0.524 (39.8) | 0.33 |
   | 3840 | 3.304 (496.2) | 2.616 (122.8)| 2.0-2.3 |

   Total time is minimal near the table optimum B≈960 (0.099 s) and grows with
   B; a larger `B` made affordable by lower memory makes the solve slower
   (B=3840: 26x slower than B=960). The dense LA matrix itself is
   `(B+16)^2·8` bytes (123 MB at 3840), so peak memory stays `Θ(B^2)` anyway:
   the oracle is not the only `B^2` term. Memory binds only far beyond
   anything here (table 16 B·B^2: ~30 GB at B≈44k, where dense LA is already
   ~1 h at this host's rate), and sparse LA, not a low-memory oracle, is what
   would move that wall.
7. **Gap to rho (not re-measured).** README of
   `prime_ic_collection_frontier_20261117` records 1-thread rho at 28-bit =
   0.0041 s; the m=3 table solve here is 0.099 s at B=960, ≈24x slower. The
   stream oracle is no closer (0.104 s). No sub-rho claim.

## Decision

No new default ported; `src/` unchanged except the build repair below. The
pair table remains the best oracle at the optimum `B≈p^{1/3}`; `stream` is a
valid memory-for-time trade (kept only in this example) with no measured
advantage in solve time.

## Caveats

- Wall times from a shared 4-thread host; ratios >=1.5x are likely real, smaller
  differences (e.g. 0.0767 vs 0.0673 s at B=960) are within noise.
- `stream` and `table` are compared on identical fixed target lists; the crate's
  adaptive walk (fresh `(a,b)` after a productive target) differs slightly from
  the fixed stream, so `solve` rows use slightly different target counts.
- Single seed/curve per bit size; field-op counts not instrumented (wall time and
  pair-sum counts, in `bench.jsonl`, are the cost measures).

## Build repair

`origin/main` did not compile: `f4_fp.rs` had both the real fall-degree fields /
`recording_degree_falls` and the earlier no-op shim (E0124/E0592/E0062). The
duplicate shim was removed (18 lines); real fields kept.

## Reproduce

```bash
cargo build --release --example ic_3sum_lowmem
E=target/release/examples/ic_3sum_lowmem
$E --mode validate --bits 28 --b 240 --n 20000
$E --mode baseline --bits 28 --b 960
$E --mode solve --bits 28 --b 960 --chunk 256
research/ic_3sum_lowmem_20261117/run_bench.sh
```
