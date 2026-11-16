# Lane sort-merge-join (all figures [pcc:unverified])

Question: does batching the relation-phase lookup (radix sort + merge-join against the sorted pair-sum key array) change counted GAE or the w=0.1/1 native band for the m=2 prime-abscissa pipeline on p24?

## Method
`examples/ic_alt_sort_merge_join.rs` (std only; a self-contained model of the lookup, NOT the ecbench ic.pipeline).
- p24: p=16777199, a=16777196, b=31, r=16782299, G=(2,5412647); asserts G on curve and r*G=O.
- Base: first 128 points with prime abscissa. Pair-sum table: x(Pi+Pj), x(Pi-Pj) (i<j), x(2Pi): 16384 group ops (= |F|^2), 16384 raw entries, 16360 unique keys.
- Trials: random walk R += S[rng] (32 precomputed uG+vQ steps, Q=1234567*G), 1 group add per trial, tracking (u,v). Setup 2218 group ops (identical in all arms).
- Arm (a) hash: open-addressing, 32768 slots x 8 B, one probe sequence per trial.
- Arm (b) merge-B: collect B trials, LSD radix sort on 24-bit key (3 passes x 8 bit), plain two-pointer merge against sorted unique key array. Also an extra arm "gallop-B" (exponential+binary search merge).
- Counting conventions (my definitions): key_cmps = key equality/order comparisons; word_ops = 1 hash + 1 per slot read (hash); radix 5n+256 per pass; merge 2 per loop step (galloping: 1 per compared word); line_touch = distinct-line changes (64 B lines), cold-cache upper bound, radix/batch-array lines are an estimate (3 streams per pass).
- Wall: median over reps, ns/trial, label wall L0 uncertified, shared 3-core container with other lanes running (noisy).

## Commands
```
cargo build --release --example ic_alt_sort_merge_join
target/release/examples/ic_alt_sort_merge_join 48550 21      # raw_n48550.txt  (protocol trial count of incumbent)
target/release/examples/ic_alt_sort_merge_join 4194304 3     # raw_n4194304.txt (scaled, to see lookup effects)
```

## Results (N=48550, seed 0xC0FFEE; raw in raw_n48550.txt)
| arm | group ops (walk) | hits | key cmps | word ops | ratio vs hash | identical |
|---|---|---|---|---|---|---|
| hash | 48550 | 93 | 70882 | 167889 | 1.00 | base |
| merge256 | 48550 | 93 | 3144586 | 7163342 | 42.7 | yes |
| merge4096 | 48550 | 93 | 244821 | 1227108 | 7.3 | yes |
| merge65536 | 48550 | 93 | 64907 | 858832 | 5.1 | yes |
| gallop4096 | 48550 | 93 | 257957 | 1043973 | 6.2 | yes |

Group ops, setup ops, hit lists (trial index, table key) and recovered (u,v) are identical across all arms (ALL_IDENTICAL=true). 93 hits = 93 table-entry relations, all 93 verified by scalar multiplication (uncounted). At N=4194304: 8183 hits, 8196 entry relations, again identical; word-op ratios 42.7/7.3/5.1.
Cache lines: hash 57546, merge256 254289, merge4096 72965, merge65536 61713 (N=48550); the merge never beats hash on lines.
Wall L0 (ns/trial, total walk+lookup, N=48550): hash 138, merge256 175, merge4096 119, merge65536 138, gallop65536 117; N=4194304: hash 137, merge4096 125, merge65536 116. Differences at B>=4096 are 10-20 ns on a ~100 ns/trial walk, comparable to the run-to-run noise (an earlier run with per-trial timers in the hash arm gave a spurious 201 ns, discarded). Not a certified speedup.

## Conclusions
- Counted GAE at w=0: unchanged. Group ops are identical by construction (lookup is native work); table cost 16384 and relation-phase trials are untouched. No change at w=0, as expected.
- Native work at w=0.1/1: batching increases native word ops 5-43x (the table is small, 16k keys, so a merge pass over it costs far more than ~1.4 probes per trial; the hash arm uses only 1.46 probes/trial). The native-op ratio is >1, so the band cannot move in the favourable direction; at w>0 the batched arm is worse or equal. The only possible gain is wall time (cache/branch effects), uncertified and within noise.
- Memory: sorted key array 16360 x 4 B = 64 KB vs hash 256 KB, but this is only relevant where memory caps |F| (N83 frontier); not tested here at memory-bound sizes (|F|^2 table of 16k keys fits L2). Memory-only lever at best.
- Negative result: lever = none for counted S.
