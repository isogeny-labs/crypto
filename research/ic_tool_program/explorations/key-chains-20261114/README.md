# Collection's key and filter stages: split min chains (A/B, not established) and a two-level filter (dropped)

- **Status:** measured, 2026-11-14, on this session's container. Nothing
  was established, and the default code path is unchanged.

## 1. Two-level presence filter: dropped at micro-benchmark ([`microbench/two_level_filter.rs`](microbench/two_level_filter.rs), [output](microbench/two_level_filter.txt))

- **The idea:** an L2-sized first-level filter (1–3 bits a key) in front of
  today's filter (8 MB at 2^47.2), so most absent keys never reach L3.
- **Result:** with branch-free compaction and a prefetched first level, two
  levels cost 5.2–5.4 ns a query against 4.9–5.0 ns for one.
- **Why:** a probe's floor is about 2.3 ns even with an L1-resident filter,
  so at most about 3 ns a probe is memory, and the extra pass costs about
  that much.
- **Side effect:** two levels also admit fewer false positives, which
  changes the counted bucket reads; `ecbench`'s methods would have to opt
  out.

## 2. Split min chains in the least-rotation kernel (A/B: [`ab/PROTOCOL.md`](ab/PROTOCOL.md), [`ab/report/report.pdf`](ab/report/report.pdf))

- **The idea:** the funnel kernel folds `n` rotations through one chain of
  dependent minima a vector; odd and even rotations in two chains cut a
  key from 3.89 to 3.48 ns at n = 61 in isolation
  ([`microbench/lr.rs`](microbench/lr.rs)).
- **A/B:** 42 rounds, same binary, a same-session A/A. Collection one/split:
  - k0n53: 1.020 [1.006, 1.034] against an A/A up to 1.030;
  - k0n61: 1.019 [1.004, 1.035] against an A/A up to 1.009;
  - not established at either size;
  - set-up 1.029 and 1.016, also not established;
  - zero identity mismatches.
- **Decision (frozen rule):** the default stays one chain. The split kernel
  is kept as an option, `KIC_KEY_CHAINS=2`, with the same words by test.
