#set page(paper: "us-letter", margin: 0.7in)
#set text(font: "Helvetica", size: 9.5pt)
#show heading: set block(above: 10pt, below: 5pt)

= ic_autolab Round 5 (generic track) — Generic Prime-Order E(F_p)

*Claim ID:* `PCC-20261119-7478b3` (`[pcc:unverified]`) \
*Frozen:* `2026-11-19T08:20:20Z` at git `94bedd024d60216a091669f1ed83a8561637d607` · *Route:* `local` (counted GAE across §5a band `w in {0, 0.1, 1}`, L0, no wall-time claim).

== Holdout Decision Summary

#table(
  columns: (auto, auto, auto, auto, auto, 1fr, auto),
  inset: 4pt,
  align: (right, left, left, left, left, left, left),
  [*k*], [*Class*], [*Selected*], [*sel/inc w=0 (95% CI)*], [*§5a Band*], [*Mean S (w=0 / 0.1 / 1)*], [*sel / rho*],
    [20], [*null*], [`tw2l-x15-j16-n2`], [0.950 [0.855, 1.055]], [sel \< inc], [2.072 / 2.249 / 3.846 (was 2.181 / 2.369 / 4.058)], [1.5x / 1.5x / 1.5x],
  [24], [*null*], [`tw2l-x15-j16-n2`], [0.984 [0.883, 1.096]], [sel \< inc], [1.878 / 2.058 / 3.686 (was 1.909 / 2.088 / 3.695)], [1.9x / 1.9x / 1.9x],
  [28], [no\_challenger], [-], [-], [-], [1.726 / 1.897 / 3.432], [-],
  [32], [*regression*], [`tw2l-x15-j16-n2`], [1.088 [0.940, 1.258]], [inc \< sel], [1.797 / 1.976 / 3.588 (was 1.652 / 1.817 / 3.298)], [2.0x / 2.0x / 2.0x],
)

== Recorded Train Screen (`round.svg`)
#image("round.svg", width: 100%)

== Cumulative Track Ratchet (`ratchet-generic.svg`)
#image("../ratchet-generic.svg", width: 88%)
