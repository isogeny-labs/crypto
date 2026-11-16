# Block-filled filter tests in the aimed scan

- **Status:** measured and kept, 2026-11-15, on this session's container.

## Why

A probe of current main ([`microbench/profile-main-*.json`](microbench/))
puts collection at 64–84% of set-up. Its largest stage is the filter test,
at 5.3–5.7 ns a summand. A probe costs about 2.3 ns even with an
L1-resident filter, so much of the stage was instructions: two
multiplies, the word and three bit positions computed twice (once to
prefetch, once to test), a counter and a branch, per key.

## The change (`src/cryptanalysis/koblitz_index_calculus.rs`)

- **`FilterGeom::fill_block`:** fills a block's hashes, filter words and
  bits eight lanes at a time with AVX-512F/DQ, with a scalar tail and a
  fallback.
- **The scan's test loop** prefetches and tests from these arrays. The
  probe count is added once per block, the same total.
- **Control:** `KIC_SCAN_VFILTER=0`.
- **Tests:** `a_filled_block_is_the_per_key_hash_and_slot` checks every
  lane against the scalar hash and slot, under both layouts, both probe
  counts, every width and every tail length. All 217 Koblitz tests pass,
  and the scan tests also pass on the control path.

## A/B ([`ab/PROTOCOL.md`](ab/PROTOCOL.md), frozen before any row; [`ab/report/report.pdf`](ab/report/report.pdf))

42 rounds, one binary, a same-session A/A. Ratios are per-key over block.

| row | collect | set-up | build |
|:--|:--|:--|:--|
| k0n53 | **1.086** [1.071, 1.121], established | 1.049 [1.027, 1.074], not established | 0.997, no regression |
| k0n61 | **1.097** [1.076, 1.115], established | **1.086** [1.056, 1.102], established | 0.996, no regression |

- **Mechanism:** filter stage 5.19 → 4.08 and 5.55 → 4.37 ns a summand.
- **Identity:** zero mismatches.
- **Scope:** a component result on this host class. It makes no claim
  against rho and adds no board row (validation gate).

Micro-benchmark: [`microbench/vf.rs`](microbench/vf.rs).
