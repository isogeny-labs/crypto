#set page(paper: "us-letter", margin: 0.7in)
#set text(font: "Helvetica", size: 9.5pt)
#show heading: set block(above: 10pt, below: 5pt)

= ic_autolab Round 2 (multibase track) — Generic Prime-Order E(F_p)

*Claim ID:* `PCC-20261119-946051` (`[pcc:unverified]`) \
*Frozen:* `2026-11-19T07:33:20Z` at git `94bedd024d60216a091669f1ed83a8561637d607` · *Route:* `local` (counted GAE across §5a band `w in {0, 0.1, 1}`, L0, no wall-time claim).

== Holdout Decision Summary

#table(
  columns: (auto, auto, auto, auto, auto, 1fr, auto),
  inset: 4pt,
  align: (right, left, left, left, left, left, left),
  [*k*], [*Class*], [*Selected*], [*sel/inc w=0 (95% CI)*], [*§5a Band*], [*Mean S (w=0 / 0.1 / 1)*], [*sel / rho*],
    [20], [no\_challenger], [-], [-], [-], [4.479 / 5.010 / 9.787], [-],
  [24], [*improvement*], [`wsum-b12-n3`], [0.218 [0.206, 0.231]], [sel \< inc], [3.060 / 3.422 / 6.678 (was 14.028 / 16.122 / 34.971)], [3.4x / 3.4x / 3.9x],
  [28], [*improvement*], [`wsum-b20-n3`], [0.156 [0.148, 0.166]], [sel \< inc], [3.081 / 3.466 / 6.936 (was 19.700 / 22.895 / 51.648)], [3.3x / 3.4x / 3.7x],
  [32], [*improvement*], [`wsum-b20-n3`], [0.134 [0.120, 0.150]], [sel \< inc], [4.078 / 4.828 / 11.577 (was 30.364 / 35.321 / 79.931)], [4.2x / 4.6x / 6.0x],
)

== Recorded Train Screen (`round.svg`)
#image("round.svg", width: 100%)

== Cumulative Track Ratchet (`ratchet-multibase.svg`)
#image("../ratchet-multibase.svg", width: 88%)
