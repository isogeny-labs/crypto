#set page(paper: "a4", margin: 18mm)
#set text(size: 11pt)
= Half-GCD projected-resultant batches
Exact lifted transition factors for the prime-field source

#image("flow.svg", width: 100%)

== Construction
Recursively compute a forward polynomial matrix from high coefficients, apply it to the full input pair, and compose the next batch. Store each transition's dividend/divisor/remainder degrees and leading divisor coefficient. Shift stable degrees on lifting and replace the final remainder degree with its actual lifted degree before accumulating resultant factors.

== Exact checks
960 cases match a separate schoolbook Euclidean trace, including every transition, returned pair, matrix action/determinant and the projected resultant. The panel exercises 1679 recursive calls and 17 changed-final-degree lifts. Geometry exercises 174 recursive calls; all 2500 cases retain agreement and exactly one projection per valid case. Eleven selected native methods pass. Generator replay is exact.

== Cost boundary
The implementation applies polynomial matrices rather than replaying full-size divisions. Matrix products, allocations and division work remain. The degree-eight cutoff is not tuned; no elapsed-time advantage has been measured against either earlier backend. Geometry point-list and decoder-construction costs remain.

== Evidence and next step
Native logs, compiler warnings, binary/dependency hashes and commands are preserved. Checks are producer-authored. Next: complete polynomial-work accounting against both references, then implicit orbit products and compact nonlinear decoder construction. No cloud or recovery comparison ran.

Source: FLINT half-GCD lifting/resultant documentation, linked with the derivation in DERIVATION.md. KB unavailable. Previous evidence and original cohort unchanged.
