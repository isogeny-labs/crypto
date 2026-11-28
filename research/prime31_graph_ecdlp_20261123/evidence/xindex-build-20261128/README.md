# M31 graph control: x-index build (2026-11-28)

**Change.** `XIndex::build` in `../../native/mitm_four_fast.rs` (lever 8)
builds the same index: the same bucket layout with couples ascending in
each bucket, the same Bloom filter and the same distinct count. It gets
there in fewer instructions and with fewer cache misses. [pcc:unverified]

1. **Two-pass radix scatter** in place of one random scatter into the
   3.6 MB entry array.
   - **Pass A** streams each build thread's couple range into 256 coarse
     regions, using the top 8 hash bits.
   - **Pass B** copies each region (about 14 KB) into an L1 scratch buffer
     and writes it back stably into its 256 buckets.

   Threads are taken in couple order, so each bucket stays q-ascending.
2. **Filter bits and distinct count fused into pass B.** Equal x means equal
   hash, so a key whose three Bloom bits are not all set yet is the first
   with its x. Only the remaining keys, about 3%, are compared against the
   entries already placed in their bucket.

   The count is 2 per entry, recomputed exactly for the rare buckets with a
   shared x. The previous build compared every pair of entries in every
   bucket.
3. **Write-first touch of the filter.** Each pass-B thread writes its slice
   of the freshly mapped filter (`fill(0)`) before its read-then-set loop.

   Without it, the first read maps the kernel's shared zero page, and the
   following write takes a second, copy-on-write fault that flushes every
   thread's TLB. That cost is negligible at 1 thread, but made the 4-thread
   build slower than `main` (7.3 against 5.7 ms median) until it was fixed.
4. **Loop bounds.** The pass A and pass B loops index fixed 256-entry
   arrays with a hash byte, so they have no bounds checks.

## Exactness

All 10 rows of `EXACTNESS.txt` give the normalised hash `a3f1e8a9…`, with
scalar `987654321` and 915116 distinct pair sums. They are the new build at
1, 2, 3, 4 and 8 threads, lever sets `124568` and `1234567`, and `main`.

The charged work is identical to `main` at the same thread count, and
`cargo test --release` passes all 3 tests.

## Cost

`timing-e2e.jsonl` was made with `../../native/bench_e2e.sh`, 15 rounds.
Each arm is a separate cold process, interleaved within each round. Both
binaries were built in the same `577d6c2c` export tree (`BUILD_SHA256SUMS`).

| arm | median x-index build (c2) | median process wall | median user + sys | peak RSS |
|---|---:|---:|---:|---:|
| `main`, 1 thread | 12.7 ms | 87 ms | 73 ms | 16.4 MB |
| new, 1 thread | 5.9 ms | 78 ms | 67 ms | 16.4 MB |
| `main`, 4 threads | 5.6 ms | 38 ms | 96 ms | 17.3 MB |
| new, 4 threads | 2.9 ms | 37 ms | 88 ms | 17.3 MB |

Paired per-round ratios, `main` over new, as median (range):

| | 1 thread | 4 threads |
|---|---:|---:|
| x-index build | 2.25× (1.52–5.31) | 1.96× (0.34–4.13) |
| process wall | 1.07× (0.92–1.45) | 1.03× (0.59–1.47) |
| CPU time | 1.07× (1.01–1.15) | 1.08× (0.87–1.19) |

**Instructions** (`CALLGRIND.txt`, 1 thread): the x-index build goes from
58.5 M to 25.7 M (−56%), and the whole process from 342.6 M to 314.4 M.
The remaining c2 time is mostly first-touch page faults on the 4.6 MB of
fresh index memory (about 1.7 µs per 4 KB page on this host).

## Tried and dropped

The data for these is in the session scratch, not in this directory; each
was timed interleaved against the kept version.

| attempt | what happened | decision |
|---|---|---|
| Transparent huge pages for the entry array and filter, via a raw `mmap` and `madvise(MADV_HUGEPAGE)` | Fewer minor faults (3557 against 4067 per run), but a worse c2 median (8.7 against 7.8 ms) and equal wall time, over 15 rounds | Dropped, along with the inline-asm syscalls |
| Pass B writing to a separate output array with several threads | c2 at 4 threads 8.5 ms, against 7.6 for the in-place version; the extra 3.6 MB of fresh pages cost more than the coherence traffic saved | Dropped |
| Two strategies chosen by thread count, with `main`'s one-pass scatter for several threads | The fault behind both slowdowns was the read-first zero page (item 3), not the strategy; after the fix the single radix strategy is the fastest at every thread count | Dropped |
| Write-first zeroing of the table build's per-thread bucket histogram | No measurable change in c1 (7.3 against 7.4 ms median at 1 thread, 3.5 against 3.4 at 4) | Dropped |

## Caveats

- **Host.** This is one shared 4-vCPU Firecracker container with large
  run-to-run noise; single rounds go as low as 0.34× for c2 at 4 threads.
  Medians and instruction counts are the figures to rely on.
- **Still short of 15×.** The single-thread path is about 11× against
  #5319, as measured in #5352, not 15×. What remains single-threaded is
  mostly collection (about 38 ms), the graph stage (about 8 ms) and the
  table build (about 7 ms).
- **Not yet done.** No independent-host replay or validator, and no
  `benchmarks/` row or bench record.
