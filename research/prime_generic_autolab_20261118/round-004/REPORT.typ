#set page(paper: "us-letter", margin: 0.7in)
#set text(font: "Helvetica", size: 9.5pt)
#show heading: set block(above: 10pt, below: 5pt)

= ic_autolab Round 4 (generic track) — Generic Prime-Order E(F_p)

*Claim ID:* `PCC-20261119-8e4c59` (`[pcc:unverified]`) \
*Frozen:* `2026-11-19T08:08:20Z` at git `94bedd024d60216a091669f1ed83a8561637d607` · *Route:* `local` (counted GAE across §5a band `w in {0, 0.1, 1}`, L0, no wall-time claim).

== Holdout Decision Summary

#table(
  columns: (auto, auto, auto, auto, auto, 1fr, auto),
  inset: 4pt,
  align: (right, left, left, left, left, left, left),
  [*k*], [*Class*], [*Selected*], [*sel/inc w=0 (95% CI)*], [*§5a Band*], [*Mean S (w=0 / 0.1 / 1)*], [*sel / rho*],
    [20], [*improvement*], [`tw2l-x20-j16-n2`], [0.213 [0.193, 0.236]], [sel \< inc], [2.123 / 2.305 / 3.943 (was 9.957 / 11.321 / 23.595)], [1.6x / 1.6x / 1.6x],
  [24], [*improvement*], [`wwalkGg-x09-sr-n2`], [0.141 [0.131, 0.151]], [sel \< inc], [1.946 / 2.128 / 3.771 (was 13.847 / 15.906 / 34.433)], [2.0x / 2.0x / 2.0x],
  [28], [*improvement*], [`tw2l-x20-j16-n2`], [0.084 [0.075, 0.093]], [sel \< inc], [1.726 / 1.897 / 3.432 (was 20.656 / 23.768 / 51.777)], [1.8x / 1.8x / 1.9x],
  [32], [*improvement*], [`tw2l-x20-j16-n2`], [0.061 [0.056, 0.066]], [sel \< inc], [1.833 / 2.015 / 3.659 (was 30.241 / 34.745 / 75.281)], [1.8x / 1.8x / 1.8x],
)

== Recorded Train Screen (`round.svg`)
#image("round.svg", width: 100%)

== Cumulative Track Ratchet (`ratchet-generic.svg`)
#image("../ratchet-generic.svg", width: 88%)
