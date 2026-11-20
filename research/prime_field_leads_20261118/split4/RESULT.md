# Reusable Split4 support

The packed high-bit x table returned the exact signed decomposition count on all 576 of its online calls. At B=256 it used 589,828 bytes of left-table payload and index and a median 406.4 ms for a complete 24-target pass. The x-hash control used about 3.19 MB and 598.4 ms. The packed, globally sorted x table occupied 524,288 bytes and needed 1,309.0 ms per pass.

The experiment ran on the prior prime31 curve with two disjoint-base laws, four sizes, and three repetitions. All 24 planned shared-right builds, 96 planned left builds, and 2,304 planned target calls ran and passed. The independent affine group-law oracle found 1,655 exact signed decompositions across 98 of the 192 fixture slots. Every target matched its oracle in all four layouts and all repetitions, including the infinity-complement boundary, coordinate collisions, and the 48 B=8 targets checked by brute signed enumeration. A separate full invocation reproduced all 2,424 non-timing records exactly.

## Pass timing and retained storage

Each median includes one right build, one left build and 24 target calls, over six passes with two base laws. The target panel and retained tables are as frozen in [the protocol](PROTOCOL.md). Timings are local diagnostics on a shared Intel Xeon Platinum 8573C host with GCC 14.2.0, Debian build 14.2.0-19. The armed loop rotates order. No CPU or RSS counters were collected. Payload and index byte counts omit allocator and batch scratch overhead.

| B | Point hash | x hash | Packed sorted x | Packed x buckets |
|---:|---:|---:|---:|---:|
| 8 | 0.441 ms | 0.355 ms | 0.609 ms | 0.397 ms |
| 32 | 6.917 ms | 6.004 ms | 12.929 ms | 5.904 ms |
| 128 | 160.528 ms | 116.181 ms | 281.621 ms | 99.678 ms |
| 256 | 816.900 ms | 598.407 ms | 1,308.965 ms | 406.430 ms |

At B=256 the paired x-hash/bucket wall ratios ranged from 1.42 to 1.51, with both charged the same reused right build. Component medians at that size follow. Independent per-phase medians need not add to the median total.

| Left layout | Left build | 24-target online | Left payload plus index | Field multiplications online |
|---|---:|---:|---:|---:|
| Point hash | 40.074 ms | 772.281 ms | 6,412,072–6,412,136 B | 28,686,393 |
| x hash | 16.289 ms | 574.141 ms | 3,189,848–3,189,880 B | 22,394,939 |
| Packed sorted x | 14.806 ms | 1,291.180 ms | 524,288 B | 22,394,939 |
| Packed x buckets | 15.082 ms | 386.012 ms | 589,828 B | 22,394,939 |

The shared right table adds 2,097,152 payload bytes, 605,440 field multiplies and a median 2.977 ms. The left build costs 605,440 field multiplies for full point keys and 474,368 for each x mode. All arms made 6,291,456 complement visits per 24-target pass. One infinite complement bypassed lookup. Packed x buckets made 50,321,150–50,331,480 key comparisons per pass, against 113,246,985–113,247,081 for globally sorted lookup. The raw hash-layout comparison field is zero because unordered-map internals are uninstrumented.

## Applicability

Packing keeps duplicate x roots as duplicate 32-bit entries, so counting preserves multiplicities. All packed arms still enumerate 2B² x roots and store quadratic support. The smaller and faster observed layout is a count-only component. The production Rust PairTable already has high-bit x buckets and a globally sorted option, and it retains witness coordinates of the factor-base pair as indices. Measuring a packed witness-preserving layout at the production call boundary is a separate experiment. This evidence covers no factor-base extraction, individual logarithms, or production solver timing. [pcc:unverified]

## Replay and provenance

From this directory:

```sh
sha256sum -c inputs.sha256
./run.sh > replay.csv
cut -d, -f1-18 replay.csv > replay-stable.csv
sha256sum -c stable.sha256
./summarize.sh replay.csv
```

The code reuses the checked-in prior Split4 arithmetic and small brute-force validation. It appends the standalone experiment to those common functions before building using the flags in [the runner](run.sh). The preregistered plan and source inputs were hashed before launch. The deterministic digest excludes the elapsed-time column. The original timed file digest is in `results.sha256`. The original raw data and replay are available in the experiment workspace as `/workspace/scratch/split4-20261118-measurements.csv` and `/workspace/scratch/split4-20261118-replay.csv`.
