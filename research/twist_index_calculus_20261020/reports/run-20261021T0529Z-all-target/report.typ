#set page(width: 210mm, height: 297mm, margin: 17mm)

#set text(font: "DejaVu Sans", size: 10pt)

= Exhaustive all-target cost run

The all-target run rebuilt each cold factor base and solved every nonzero public target for six curves and three base sizes. It emitted 49,248 verified IC solves and 12,312 matched rho solves. The independent Python audit checked all 61,579 records, target points, recovered scalars, phase sums, and complete target coverage.

#image("figure.svg", width: 100%)

Figure: recorded all-target complete-time medians and nearest-rank 5th–95th percentiles, L0. The raw SHA-256 is cf209579f9f1a84b7416e2a6fa5b495e1b6e81fede87b8e0b759de2b3468acce. This run is descriptive; counted and isolated comparison is reported separately.
