# Prime31: factor-base windowing converts B8 algebraic relation access into B31 rank and complete logs

**2026-11-18** · [Existing draft RR9 lane #4728](https://github.com/isogeny-labs/crypto/pull/4728) · [Prospective registered protocol](W8_WINDOWED_B8_RANK_PROTOCOL_20261118.md) · [Reused original Rust example](../native/examples/prime31_w8_windowed_rank.rs) · [independent Python BigInt artifact replay](../native/verify_prime31_w8_windowed_rank.py) · **[pcc:unverified]**.

## Structural contribution

A single RR9/F4 unknown-x solver need not expose all 31 factor-base points in **each of its eight slot supports**. Instead, cover the 31 global unknown logs by four fixed 8-point windows:

```
W0:  0..7   (8 new factor logs)
W1:  8..15  (8 new factor logs)
W2: 16..23  (8 new factor logs)
W3: 23..30  (7 new factor logs; one shared with W2)
```

For each window, the eight signed weighted slots offer 16 possible points per slot. A natural public `R=[a]G` decomposition gives an equation `Σ ε_j w_j log_G(P_i)=a (mod n)`. Eight independent equations in each 8-point window span the window's log space; the first three windows contribute 24 independent columns and the last eight-dimensional window, sharing one known column, contributes seven new columns. Thus their union **can** reach full B31 rank 31 without ever solving a B31 unknown-chart polynomial system. This is a linear-algebra existence/assembly observation, **not** a new efficient relation-extraction algorithm.

For any query point `Q`, a decomposition of shifted `R=Q+[a]G` in any of the four windows can then recover `log_G(Q)=Σ ε_j w_j log_G(P_i)-a (mod n)` once the 31 logs are known.

## Exact finite-group witness expectations (combinatorial, not experiments)

For any fixed sets of signed slot points, the total **ordered** eight-slot witness count, summed over every `R∈E(F_p)`, equals `(2B)^8`. On the original prime-order group `n=2147446831` the uniform-target average is

| Per-slot base size B | Ordered words | Mean eight-slot witnesses / uniformly random R |
|---:|---:|---:|
| 4 | 16,777,216 | 0.007812634 |
| **8** | **4,294,967,296** | **2.000034289** |
| 16 | 1,099,511,627,776 | 512.008778 |
| 31 | 218,340,105,584,896 | 101,674.277767 |

These are unconditional first moments, not hit probabilities. They quantify a practical high-degree/multiple-root obstacle: B31 yields roughly 101,674 ordered witnesses per uniformly random residual, whereas B8 offers an expectation near two. An algebraic solver that only needs one root could still exploit higher B; these first moments do **not** prove an F4 complexity bound or preclude any attack.

## Actual local original-curve preflight (independent Python implementation, not native CI)

I implemented a separately coded BigInt elliptic-curve 4+4 grading pilot over the **unchanged** original curve `p=2147483647, a=847514846, b=2087444724, n=2147446831, G=(625037715,2100627251)`. It generated the original 31 canonical first-x points, four signed-weighted B8 windows and exact training residuals `[a]G` (scalar a not sent to the oracle).

| Frozen window | Natural training attempts | Successful public relations used | New independent log columns | Cumulative rank |
|---|---:|---:|---:|---:|
| W0, 0–7 | 8 | 8 | 8 | 8 |
| W1, 8–15 | 8 | 8 | 8 | 16 |
| W2, 16–23 | 10 | 8 | 8 | 24 |
| W3, 23–30 | 11 | 7 | 7 | **31** |
| **Total** | **37** | **31** | **31** | **31/31** |

Every recovered factor-base logarithm independently verified `[log_G(P_i)]G=P_i`. A separate local test then recovered **32/32** newly generated unknown public logarithms using shifted residuals, with no BSGS/rho fallback. The locally measured end-to-end *Python diagnostic* duration was ~20.17s for four-window table setup, rank, verification and 32 public targets; it is **not** a Rust benchmark and this diagnostic **does** employ generic 4+4 tables. These local results are producer-only and are not a scientific competitiveness claim.

The four measured right-half support sizes were each **64,737** points, agreeing with the earlier single-window exact grader. However, the Rust producer deliberately does **not** assume these values for every possible window; unexpected group collisions are counted, not treated as a theorem.

## Native exact-source reproduction underway

A GitHub Actions workflow [`prime31-w8-windowed-rank.yml`](../../../.github/workflows/prime31-w8-windowed-rank.yml) is committed to run 40 existing original-crate Rust library tests, focused original-curve 32-target controls and a separate **full 1,024-target** public unknown-log recovery using original Rust source pinned at `1df714645aa683cf975bc76fcacb9842cb4780fd`, Rust 1.100.0. A separately authored Python checker independently reconstructs all base points, 31 factor logarithms, natural training witnesses, and 1,024 target identities. The experiment is explicitly marked `generic_four_plus_four_grader_used=true`.

**Do not claim native 1,024-log success, timing, or source-specific reproducibility until the actual workflow is complete.** Source-specific run IDs, errors, failed checks and raw evidence should be added below, after observation. No signed PCC certificate exists.

## Prior leads and next expensive discriminant

- [#4692](https://github.com/isogeny-labs/crypto/pull/4692): the *genuine* public-only nonenumerative RR9/F4 unknown-x/rootfinding extractor remains unimplemented at the required end-to-end quality; this new work supplies a B8-to-B31 algebraic assembly scaffold, **not** a competing extractor lane.
- [#4728](https://github.com/isogeny-labs/crypto/pull/4728): existing source-verified signed B8 grader (46/64 natural public hits) and given-x determinant components, reused rather than replaced.
- [#4714](https://github.com/isogeny-labs/crypto/pull/4714): existing complete M6 generic-equivalent Rust solver loses about 21× to strongest same-runner shared BSGS. The historical BSGS 144.049ms / 1,024 targets remains a severe **fully charged** budget; no new matched BSGS run has been performed here.
- [#4722](https://github.com/isogeny-labs/crypto/pull/4722): six-weight formal collision/yield ceilings rule out major improvements from simple M6 retuning.
- [#4746](https://github.com/isogeny-labs/crypto/pull/4746): earlier favorable PKM exponent was withdrawn after correction; do not promote any asymptotic speedup without proof.
- Published prior art: Petit–Kosters–Messeng, PKC 2016, DOI `10.1007/978-3-662-49387-8_1`; Amadori–Pintore–Sala, FFA 2018, DOI `10.1016/j.ffa.2018.01.009`. GitHub code-index searches in `crypto`, `cryptanalysis`, `crypto-autoresearcher` found related factor-base designs and many failed/specialized algebraic lanes but no identical complete windowed B8 corpus. Research-KB server/wrapper inaccessible; this is no novelty claim.

**Decisive next experiment:** instantiate the **same four-window public-R interface** with an RR9/F4 unknown-index extractor that receives no generic sum tables, no planted witnesses and no hidden target scalar. Test each window's natural hit/miss oracle equivalence and exact 31-rank/1024-target recovery. Count every failed F4 call and memory allocation, then AB/BA benchmark the full system against the strongest reused Rust index-calculus control and same-runner shared BSGS and optimized rho. Given ~1,000+ algebraic calls for 1,024 logs, the historical 144.049ms shared-BSGS result leaves at most on the order of 100 microseconds per call **before setup/rank costs**; this is an arithmetic budget, not a measured constraint-solver runtime.

**Standing:** an actionable structural reduction to small B8 solves with original-curve local rank evidence and check-in, but **not yet** a cheaper complete non-generic prime-field solver. Independent signed validation pending.
