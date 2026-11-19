#set page(paper: "us-letter", margin: 0.7in)
#set text(font: "Helvetica", size: 9.5pt)
#show heading: set block(above: 10pt, below: 5pt)

= ic_autolab Round 1 (multibase track) — Generic Prime-Order E(F_p)

*Claim ID:* `PCC-20261119-95e1b8` (`[pcc:unverified]`) \
*Frozen:* `2026-11-19T07:18:34Z` at git `94bedd024d60216a091669f1ed83a8561637d607` · *Route:* `local` (counted GAE across §5a band `w in {0, 0.1, 1}`, L0, no wall-time claim).

== Holdout Decision Summary

#table(
  columns: (auto, auto, auto, auto, auto, 1fr, auto),
  inset: 4pt,
  align: (right, left, left, left, left, left, left),
  [*k*], [*Class*], [*Selected*], [*sel/inc w=0 (95% CI)*], [*§5a Band*], [*Mean S (w=0 / 0.1 / 1)*], [*sel / rho*],
    [20], [*improvement*], [`wsum-b5-n3`], [0.445 [0.407, 0.487]], [sel \< inc], [4.479 / 5.010 / 9.787 (was 10.067 / 11.449 / 23.886)], [3.2x / 3.3x / 3.8x],
  [24], [*null*], [`wsum-b5-n3`], [0.876 [0.745, 1.029]], [unresolved inside the band], [11.973 / 14.245 / 34.690 (was 13.671 / 15.695 / 33.907)], [10.3x / 11.2x / 15.5x],
  [28], [*improvement*], [`mitm2-n342`], [0.950 [0.922, 0.980]], [sel \< inc], [19.309 / 22.426 / 50.475 (was 20.316 / 23.360 / 50.754)], [19.4x / 20.5x / 25.7x],
  [32], [*improvement*], [`mitm2-n810`], [0.936 [0.915, 0.958]], [sel \< inc], [29.239 / 33.970 / 76.548 (was 31.222 / 35.924 / 78.237)], [30.4x / 32.2x / 40.0x],
)

== Recorded Train Screen (`round.svg`)
#image("round.svg", width: 100%)

== Cumulative Track Ratchet (`ratchet-multibase.svg`)
#image("../ratchet-multibase.svg", width: 88%)
