#set page(paper: "a4", margin: 19mm)
#set text(size: 10.5pt)
= Shared seeds and reusable preparation
Prime-field native source · 2026-11-03

The native prepared-source API now retains strides and propagates normalization seeds across public-point windows. Calibration supplies the base seed; anchors at O, P, minus P and 2P handle the singular transitions. Only nonanchor positions among the first two require direct phase.

#image("chart.svg", width: 100%)

== Checks
Three selected native methods pass once. All 80 signs in five length-16 F509 windows agree with checker-only Euler symbols. All 78 finite seeds agree with direct phases; 64 saved public shifts match. Reuse of prepared state passes on the three original curves, including exact saved calibration/raw records and empty/invalid cases. F7/r13 and F5/r7 exercise the omitted-singleton and omitted-partial branches. Nineteen earlier tests were filtered.

== Component cost
The shared path records three direct phases, 67 recurrence steps and ten anchor hits. Each recurrence uses four field products and one inversion, hence 268 products and 67 inversions for these five windows. These counts exclude test-only direct references and all other source work; they establish no complete-cost or timing result.

Setup includes doubling the base point and preparing strides. Each query still pays geometry, value division, resultant, inner recurrence and final quotient/calibration costs. Prime-field and prime-order assumptions and inherited word limits apply. No full4563-case cohort or cloud job ran.

== Next step and provenance
Bind the shared source to the repaired native sign-mask decoder, then instrument complete costs. The construction preserves the historical mixed-projective-strides-v2/run.py algorithm, bound by original-inputs.sha256. Exact code/dependency hashes, commands, compiler and logs accompany receipt.json. See DERIVATION.md for the formula, branch coverage and remaining accounting.
