#set page(paper: "us-letter", margin: 0.7in)
#set text(font: "Helvetica", size: 9.5pt)
#show heading: set block(above: 10pt, below: 5pt)

= ic_autolab Round 3 (multibase track) — Generic Prime-Order E(F_p)

*Claim ID:* `PCC-20261119-e03ae8` (`[pcc:unverified]`) \
*Frozen:* `2026-11-19T07:35:17Z` at git `94bedd024d60216a091669f1ed83a8561637d607` · *Route:* `local` (counted GAE across §5a band `w in {0, 0.1, 1}`, L0, no wall-time claim).

== Holdout Decision Summary

#table(
  columns: (auto, auto, auto, auto, auto, 1fr, auto),
  inset: 4pt,
  align: (right, left, left, left, left, left, left),
  [*k*], [*Class*], [*Selected*], [*sel/inc w=0 (95% CI)*], [*§5a Band*], [*Mean S (w=0 / 0.1 / 1)*], [*sel / rho*],
    [20], [no\_challenger], [-], [-], [-], [4.479 / 5.010 / 9.787], [-],
  [24], [no\_challenger], [-], [-], [-], [3.060 / 3.422 / 6.678], [-],
  [28], [no\_challenger], [-], [-], [-], [3.081 / 3.466 / 6.936], [-],
  [32], [no\_challenger], [-], [-], [-], [4.078 / 4.828 / 11.577], [-],
)

== Recorded Train Screen (`round.svg`)
#image("round.svg", width: 100%)

== Cumulative Track Ratchet (`ratchet-multibase.svg`)
#image("../ratchet-multibase.svg", width: 88%)
