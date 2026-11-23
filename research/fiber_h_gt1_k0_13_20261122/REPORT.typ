#set page(paper: "a4", margin: 2cm)
#set text(font: "New Computer Modern", size: 11pt)
#set heading(numbering: "1.")

= K0-13 counted vs fiber-closed (planner re-price)

Producer capsule for `research/fiber_h_gt1_k0_13_20261122`. Session
`ECBS1hbaa236332ada`, isolation L0. Numbers are harness-recorded; this PDF
does not recompute them. Tag: `[pcc:unverified]`.

== Outcome

Matched panel on K0-13 (`icv1-f2m13-t181-515ee569`, $h = 4$): *128/128*
verified. Factor base `compact-orbit-scan:columns=1,raw_x_cap=100000000`,
targets `stride-renew`, method `ic.pipeline_full`.

#table(
  columns: 4,
  align: (left, right, left, right),
  [*Arm*], [*Mean cold S*], [*95% interval*], [*Verified*],
  [rho-frob], [2.083], [[1.774, 2.446]], [24/24],
  [ic-counted], [5.966], [[5.837, 6.098]], [24/24],
  [ic-fiber-closed], [14.483], [[14.353, 14.613]], [24/24],
  [rho-frob-aa], [2.083], [[1.774, 2.446]], [24/24],
)

Paired ops ratio fiber/counted $= 2.427$ $[2.396, 2.459]$. Class: accounting
(planner support Measured). Not an S-column win.

#figure(
  image("figures/mean_s.svg", width: 90%),
  caption: [Mean cold $S$ by arm on the matched K0-13 panel.],
)

== Decision

Promote planner $h > 1$ oracle stage to `Support::Measured`. Keep
fiber-closed as the default; do not claim it beats counted in $S$.
See `docs/ic/FIBER_H_GT1_VALIDATION.md`.
