#set page(paper: "a4", flipped: true, margin: (x: 15mm, y: 13mm))
#set text(size: 10pt)

= Regularity from two survivor fibers
#text(fill: luma(90))[2026-11-18 · Run 20261118T0459Z · F₁₀₁ · Mixed stage of the toy flow]

#image("fiber_regularity.svg", width: 100%)

*Exact rule.* Group surviving points by $y$. The observed degree is the size $M$ of the larger fiber, with one extra degree if both nonempty fibers have size $M$. At degree $M - 1$ the two fiber interpolants must share their top coefficient. An unequal smaller fiber has room to match it. Equal fibers can impose conflicting coefficients and need one more degree.

*Checks.* The formula agrees with the archived Singular degrees for 130 base–target cases and nine pins, and with evaluation ranks for 1,023 additional finite point-set controls. With three fibers of one point each, collinear and noncollinear sets give degrees 3 and 2. The two-fiber rule has no prediction for those cases. [pcc:unverified]

Follow the derivation and replay in #raw("RUN.md"). The CSV contains fiber sizes, predictions, and independent reference degrees.
