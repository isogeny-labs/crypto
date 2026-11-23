# Frozen protocol: Dickson31 B31 complete 1024-Q rank/control (2026-11-17)

Owner PR: #4623, stacked on #4618 (whose batched-complement oracle is reused unchanged). Producer-development only [pcc:unverified].

## Objective and classification

Build an actually complete index-calculus-*style* 31-column linear-algebra + individual-log recovery control on the **real ordinary cofactor-one prime-field** curve p=2147483647, E: y²=x³+847514846x+2087444724, n=2147446831, G=(625037715,2100627251). The current 8A relation mechanism is explicitly the **generic-equivalent** signed 4A+4A MITM; it is NOT a new non-generic attack. This is a fresh *complete-solver feasibility and cost* control, **not** the target-conditioned sign-independent extractor.

Reuse the original native Rust Arithmetic, Base::build(PRIME31,6), FourSet::build and the existing parent #4618 512-wide Montgomery-batched recover_batched, without changing the 618,885 distinct four-sums, factor-base contents or witness validation. The full recovery program may add modular n rank and a fallback (never supply test-only k to the solver).

## Controls, seeds and limits (fixed before execution)

* Basis-learning input points are public [a]G for uniformly generated nonzero a from SplitMix64 seed 0x4241534533315f31, with a known **public coefficient**, not target secrets. Up to 128 such input points; public shifts 0..8; insert each exact signed eight-leaf relation with RHS a+shift into incremental rank over Z/nZ, with 31 unknown base logarithms. Include all failed scans. Stop when modular rank 31 and independently replay each of the 31 recovered point logs on the original curve.
* Optionally verify 16 further independently drawn public [a]G equations without updating rank; no chosen-witness or planted oracle shortcuts.
* 1,024 heldout caller-only k values: SplitMix64 seed 0x4d49583450333131, each k=1+next%(n-1), enforce distinctness. Input to solving is Q=[k]G only. For each Q try the public adaptive shifts 0..8, recover one exact signed B31 8A relation, and calculate k = sum(token-sign * base-log) - shift (mod n). All failed shifts are counted.
* For targets unresolved after nine shifts, a cold original-arithmetic generic BSGS fallback is mandatory and its full table setup, misses, wall, memory capacity and original-curve replay are charged. This makes the control mathematically total on the prime-order subgroup; a fallback hit MUST NOT be counted as index calculus success. Exhaustive BSGS size m=46341 and giant-step bound ceiling(n/m).
* Every returned k must pass [k]G=Q using original Rust, and after solver return must equal caller-only known k. An unsuccessful rank-31 setup or missing original-curve replay is a failed experiment, never a suppressed trial.

## Measurement/reporting

Report source SHA and pinned Rust 1.100.0; 40 native library tests plus focused test; exact rank and row counts; batch point/field op counters, all public-shift misses, full 1024 solved logs, number via algebraic-index-control versus fallback, distinct public Q, charged curve validation + base/4A setup + known-source relation collection + scalar point checks + 1024 queries + fallback + output verification, and HashMap capacity. Keep caller corpus generation cost separate as benchmark harness overhead and state so. Original 512-wide complement-only result #4618 and historical same-1024-Q BSGS m=2^20=170.113ms vs mixed IC=2221.474ms #4532 are contextual, NOT a fair cross-run speed ratio; stronger same-host matched BSGS/rho comparison is owned by #4596.

## Prior work / prohibitions

Already searched or consulted: #4618 and #4615 signed Dickson31 4A oracle, #4596 complete 1024-Q pair-table IC comparator (different basis/algorithm), #4604 public-only RR9 D9 rank, #4532 1024-Q BSGS, #4581 Fp^128 quotient split and #4612 fixed B8 rarity, #4594 priority audit, autoresearcher #3301/#3305 and cryptanalysis #1536; PKM 2016 DOI 10.1007/978-3-662-49387-8_1 and APS 2018 DOI 10.1016/j.ffa.2018.01.009. Research-KB MCP and repository CLI were inaccessible in this isolated connector runtime, so source coverage is bounded. No copied Rust reimplementation of the group law, no paid computing, no leaderboard edit, no merging, no security claim, no independent PCC promotion.

Decision gates: component completes iff 31 independently verified basis logs and all 1024 Q recovered with fully accounted failures and fallback, otherwise record exact failure and freeze. Scientific goal of a cheaper non-generic solver remains a separate target-conditioned RR9/S9/K961 extractor program.
