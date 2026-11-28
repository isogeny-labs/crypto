#set page(paper: "a4", flipped: true, margin: (x: 15mm, y: 11mm))
#set text(size: 9.5pt)

= Conditional survivor tracking
#text(fill: luma(90))[2026-11-18 · Run 20261118T0409Z · F₁₀₁ · Shared target $(3,2)$]

#image("survivor_retention.svg", width: 100%)

Each cell compares how many distinct pairs of the other two variables remain feasible after the pin with the number feasible at the same stage before the pin. Pins use every value shared by the three bases. For the centered base, the sum stage with $x=2$ retains all four individual values of $y$ and $z$ while retaining only four of ten feasible pairs. At the mixed stage, $y=0$ eliminates all survivors in every base. [pcc:unverified]

Use the joint support alongside the individual value counts, direct coupling trace, and degree of regularity. The summary records every pin, including dead ends. The support CSV records the actual values and their multiplicities.
