# Skeptic review, lane sort-merge-join [pcc:unverified]
Reproduced: rebuilt release example, ran `ic_alt_sort_merge_join 48550 21`. Every deterministic number matches raw_n48550.txt exactly (group ops 48550/2218/16384, hits 93, word ops 167889/7163342/1227108/858832, ratios 42.7/7.3/5.1, lines 57546/72965/61713, ALL_IDENTICAL=true).
Wall (L0, noisy): hash 137.1, merge4096 118.0, merge65536 116.8 vs claimed 138/119/138. merge65536 moved 138 -> 117 between runs, as large as the claimed batch-vs-hash gaps, so "within noise" holds.
Findings (none refute the negative result):
- Word-op unit is self-defined and asymmetric (hash: 1 per probe; merge: 2 per step + 5n+256 per radix pass); the 5-43x ratio depends on it. merge256 rescans the full 16k-key table per 256 trials, a strawman batch size. Direction (merge >= hash) is plausible but the magnitude is convention-dependent.
- Batch arm also appends (u,v) per trial (uvall), hash arm only on hits: wall bias against batch; the 10-20 ns "gain" is therefore not a clean measure either way.
- Table build 16384 ops is |F|^2 counted in both arms; hash-slot construction is uncharged in both. Lookup is native, not group ops, so no GAE lever: lever=none is correct.
- Model only, not ic.pipeline; hits 93 is table hits, not solved relations.
