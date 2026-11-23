# SKEPTIC: dulmage-mendelsohn  [pcc:unverified]
Reproduced: rebuilt example, reran 16 seeds; raw_output.jsonl byte-identical (cmp) and summary.txt identical (diff).
Raw recount: 128/128 slack 0 (IG and echelon), coarse-DM, minimal, block-pin correct (IG, echelon) 128/128; all library arms correct. Headline numbers match raw.
Issues:
1. Uncounted native work: the DM arm charges only DFS adjacency probes (271 at |F|=128 m=2). It does not charge the per-row O(|F|) vis_col reset or the dense O(|F|) scan that builds each row's support (~|F| x 82 rows = ~10.6k elementary steps vs the 599-op "saving"). Baselines are charged by multiply-subtract row_ops only. A sparse implementation would avoid this, but it is unmeasured, so the w=0.1/w=1 savings are an upper bound and the sign is not established.
2. "S-i never pins" is exhaustive only for |S|<48; sampled (about 24 rows) in 56/128 streams, yet the headline says 128/128. Minimality (matching) is exhaustive.
3. Slack 0 is close to structural (pin iff B dependent, generic values); agreement is expected, not a surprising finding.
4. Headline omits that DM total loses to echelon in many m=3 streams (3/16 at |F|=64, 7/16 at |F|=128); the claim is only against incremental-gauss. Planted uniform columns only.
Lever label native_only is correct; w=0 gain is 0; beats_baseline=false is consistent.
