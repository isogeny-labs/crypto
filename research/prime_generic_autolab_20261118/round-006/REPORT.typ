#set page(paper: "us-letter", margin: 0.7in)
#set text(font: "Helvetica", size: 9.5pt)
#show heading: set block(above: 10pt, below: 5pt)

= ic_autolab Round 6 (multibase track) — Generic Prime-Order E(F_p)

*Claim ID:* `PCC-20261119-a5013d` (`[pcc:unverified]`) \
*Frozen:* `2026-11-19T08:43:46Z` at git `94bedd024d60216a091669f1ed83a8561637d607` · *Route:* `local` (counted GAE across §5a band `w in {0, 0.1, 1}`, L0, no wall-time claim).

== Holdout Decision Summary

#table(
  columns: (auto, auto, auto, auto, auto, 1fr, auto),
  inset: 4pt,
  align: (right, left, left, left, left, left, left),
  [*k*], [*Class*], [*Selected*], [*sel/inc w=0 (95% CI)*], [*§5a Band*], [*Mean S (w=0 / 0.1 / 1)*], [*sel / rho*],
    [28], [no\_challenger], [-], [-], [-], [3.081 / 3.466 / 6.936], [-],
  [32], [*improvement*], [`wsum-b28-n3`], [0.739 [0.665, 0.822]], [sel \< inc], [2.966 / 3.396 / 7.269 (was 4.013 / 4.749 / 11.381)], [2.7x / 2.8x / 3.3x],
)

== Recorded Train Screen (`round.svg`)
#image("round.svg", width: 100%)

== Cumulative Track Ratchet (`ratchet-multibase.svg`)
#image("../ratchet-multibase.svg", width: 88%)
