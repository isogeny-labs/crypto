# M31 graph control: 8-lane AVX2 probe kernel (2026-11-28)

**Change.** In `../../native/mitm_four_fast.rs`, the lever-8 probe kernel
(`avx2::probe_blocks`) now uses one 8 × u32 vector for its eight
batch-inversion chains. The previous kernel used two 4 × u64 vectors.

- **Lanes.** Two 32-byte loads of four `[x, y]` couples each are
  deinterleaved with a shift and a blend into X = [x0, x4, x1, x5, x2, x6,
  x3, x7] and Y alike. 32-bit lane j therefore holds couple
  `(j & 1) * 4 + j / 2` of the block (`LANE_COUPLE`).
- **Multiplication.** `mul8` uses `vpmuludq` on the even lanes and on the
  odd lanes after a 32-bit shift. Each half gets one Mersenne fold, the
  halves are recombined with a blend, and one shared `min_epu32(t, t − p)`
  does the conditional subtraction.
- **Everything else** covers all eight lanes in one op: subtraction,
  addition, the denominator fix-up, the x = x_R test, and the Bloom filter.
  The filter now uses 32-bit words, `vpmulld` for the hash and a
  `vpgatherdd` of eight words.

The x-index's blocked Bloom filter moves from 64-bit to 32-bit words. It
is still 1 MB with three bits per key, now taken from hash bits 0..14. The
scalar `head()` uses the same layout.

The per-thread probe buffers collapse to one `Vec<u32>`. The default
lever set is still `1234567`, and lever 8 is unchanged as an interface.
[pcc:unverified]

## Exactness

Every row of `EXACTNESS.txt` gives the normalised hash `a3f1e8a9…`, with
scalar `987654321` and 915116 distinct pair sums. The rows are the new
kernel at 1, 2, 3, 4 and 8 threads, lever sets `124568` and `1234567`, and
the 4-lane baseline.

The charged work (group additions, multiplications, inversions) is
identical to the 4-lane kernel at the same thread count. Both kernels run
eight chains, with one inversion per chain.

A new unit test, `avx2_couple_kernel_matches_scalar_kernel`, compares the
kernel with the scalar couple kernel on a 40-point table, for three
queries. They give the same relation set, the same group additions and the
same special-path inversions. One query hits the x(L) = x(r) path, because
r = P₁ + P₂ is itself a couple sum. `cargo test --release` passes all
three tests in the binary.

## Cost

`timing-e2e.jsonl` was made with `../../native/bench_e2e.sh`, 11 rounds.
Each arm is a separate cold process, interleaved within each round. Both
kernels were built in the same `577d6c2c` export tree. The 4-lane binary
is `main`'s source at `c1f7a4ee` (`BUILD_SHA256SUMS`).

| arm | median process wall | median user + sys | median collection |
|---|---:|---:|---:|
| 4-lane, 1 thread | 92 ms | 84 ms | 55.3 ms |
| 8-lane, 1 thread | 75 ms | 68 ms | 38.1 ms |
| 4-lane, 4 threads | 40 ms | 110 ms | 21.7 ms¹ |
| 8-lane, 4 threads | 35 ms | 94 ms | 17.2 ms¹ |
| #5319 `1234567`, 1 thread | 842 ms | 760 ms | |

¹ Collection overlapped with the graph stage.

Paired per-round ratios, 8-lane against 4-lane, as median (range):

| measure | 1 thread | 4 threads |
|---|---:|---:|
| process wall | 1.25× (0.98–1.66) | 1.20× (0.83–1.91) |
| CPU time | 1.24× (1.17–1.41) | — |
| collection stage | 1.47× (0.96–2.13) | — |

Against #5319, the 8-lane kernel is 11.25× by process wall at 1 thread
(range 9.3–12.5), 11.06× by CPU time, and 24.1× by process wall at 4
threads (range 17.8–28.2). Peak RSS is unchanged at 16.4 MB (1 thread) and
17.3 MB (4 threads). [pcc:unverified]

**Instructions** (`CALLGRIND.txt`, 1 thread, whole process; these are
deterministic, unlike the timings):

| | total | `probe_blocks` | per couple |
|---|---:|---:|---:|
| 4-lane | 423.1 M | 228.1 M | 45.3 |
| 8-lane | 342.6 M | 147.6 M | 29.3 |

The kernel's instructions fall by 35% and the whole process's by 19%.

## Caveats

- **Host.** This is one shared 4-vCPU Firecracker container. Single
  rounds range from 0.83× to 2.1×; the medians and the instruction counts
  are the figures to rely on.
- **Still short of 15×.** The single-thread path is now 11× against #5319,
  not 15×. The next levers are vectorising the table build, the x-index
  build (about 13 ms single-threaded) and huge pages.
- **Not yet done.** No independent-host replay or validator, and no
  `benchmarks/` row, bench record or leaderboard row.
