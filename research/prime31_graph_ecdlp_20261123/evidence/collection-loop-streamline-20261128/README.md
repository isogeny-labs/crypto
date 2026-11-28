# Remaining exact collection-loop optimizations, M31 graph control (2026-11-28)

**Change.** Three more switchable levers in
`../../native/mitm_four_fast.rs`; the default is now `1234567`:

5. **Streamlined probe loop** (needs 1). The backward batch pass visits
   each complement directly, so no 24-byte output buffer is written,
   zeroed or re-read. The `num`/`den`/`prefix` buffers are reused across
   queries without zeroing. Special cases are encoded as `den = 0`.
   Negation is a branch instead of `% p`. The passes iterate instead of
   indexing.
6. **Lazy Mersenne reduction** (needs 2 and 5). Each product gets one fold
   and one conditional subtraction, with values kept in [0, p] and p
   standing for 0. Only the final coordinates are made canonical; the
   inversion's argument is canonical, as before.
7. **Chained pair index.** One map entry per distinct sum plus a `next`
   array over pair ids, instead of one `Vec` allocation per key. The order
   in which a key's ids are visited changes. Matches go into a set and the
   relations are sorted afterwards, so the output cannot depend on it.

The match handling is the baseline's inner loop, moved into
`record_matches`. Measurements were made in the `577d6c2c` export, as for
#5311, because the `main` harness pin is broken; shared 4-vCPU container,
rustc 1.97.0. [pcc:unverified]

## Exactness

The baseline and 13 lever combinations each give the same 707 relation
rows, per-query classes and ranks, probe count and recovered scalar
`987654321` after 11 queries. The combinations are 0, 1234, 15, 125, 1256,
12345, 123456, 1234567, 12347, 123457, 7, 37 and 157. One normalised
SHA-256 covers all of them (`EXACTNESS.txt`).

## Cost

All timing files carry round IDs. Arm order is fixed within each round,
and each run is a separate cold process. Medians are of 7 runs.

**Against the merged binary** (`timing-vs-merged.json`; PR #5311's binary,
`BUILD_SHA256SUMS`):

| | merged `1234` | new `1234567` |
|---|---:|---:|
| cold wall | 965 ms | 781 ms |
| collection | 694 ms | 635 ms |
| pair table | 203 ms | 116 ms |
| instructions per query (`CALLGRIND.txt`) | 252.8 M (276 per probe) | 218.5 M (238 per probe) |
| fixed part: fixture, table, setup | 620 M | 287 M |
| peak RSS (`peak-rss.json`) | 193.5 MB | 116.4–117.1 MB |

**End to end** (`timing-end-to-end.json`): baseline `mitm_four.rs`
5,424 ms; new build `1234` 979 ms; `1234567` 785 ms. That is about 6.9x
against the baseline.

**Per lever** (`timing-levers.json`, new build):

| arm | cold wall | collection | table | per query | fixed part | peak RSS |
|---|---:|---:|---:|---:|---:|---:|
| 1234 | 968 ms | 719 ms | 200 ms | 268.2 M | 623 M | 193.5 MB |
| 12345 | 934 ms | 688 ms | 210 ms | 243.2 M | 641 M | 172.8 MB |
| 123456 | 841 ms | 600 ms | 202 ms | 218.4 M | 641 M | 172.8 MB |
| 12347 | 858 ms | 708 ms | 114 ms | 268.3 M | 269 M | 137.8 MB |
| 123457 | 820 ms | 660 ms | 108 ms | — | — | — |
| 1234567 | 763 ms | 612 ms | 107 ms | 218.5 M | 287 M | 117.0 MB |

**Reading the lever table:**
- Lever 5 removes 25 M instructions per query and 21 MB, the output buffer.
- Lever 6 removes another 25 M per query.
- Lever 7 more than halves the fixed part and the table time, and removes 56 MB of per-key `Vec` allocations. Peak RSS is now below the baseline's 130 MB.
- Single runs suggested lever 6 was slower; the interleaved medians show it is quicker.

**Regression note.** In the new build the `1234` path itself is 15.4 M
instructions per query costlier than in the merged binary. Lookups now
dispatch through the `PairIndex` enum and a closure. This affects only
arms without lever 7's chain. The `1234567` figures above already include
that cost.

## Limits

- **One fixed target and IC seed.** Std randomized hashing remains in the de-duplication set and in the baseline. Arm order was fixed with no A/A arm, on a shared host. No target panel or counterbalancing was run.
- **Rho is still ahead.** Matched-target untuned Brent rho takes 43–45 ms (#5311), about 18x less than `1234567`. These levers make this solver cheaper without changing its output; they are not a route past rho.
