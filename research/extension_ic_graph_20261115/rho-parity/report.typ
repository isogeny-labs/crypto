#set page(paper: "a4", margin: 16mm)
#set text(size: 9.4pt)
#set heading(numbering: none)
= Exact Nagao unit elimination
Rho-parity continuation · 17 November 2026 · producer development evidence

The opt-in relation finder removes one variable before F4 and recovers every
original solution. Six fixed native fixtures use *27.6879% fewer observed
primitive units in aggregate*, with identical solutions and materialized rows.
These incomplete component counts have unverified PCC standing; they are not
an admitted clock or a whole-pipeline result. [pcc:unverified]

#image("flow.svg", width: 100%)

== Exact substitution
For monic degree-seven H and degree-three u, the Nagao quotient satisfies
`Q5 = H6 + 2*mu - lambda1^2 - u2`.
Writing `lambda1 = a1 + t*b1`, `mu = c0 + t*d0`, solve
`d0 = a1*b1 + (Im(u2)-Im(H6))/2`.
The constant denominator is a unit in an odd prime field. Substitution in the
other five equations has degree at most four and defines a bijection on
solutions. The code checks the actual pivot, bounds expansion, preserves
polynomial scalars, lifts each root and replays all original equations.
The checked materializer retains full-row group checks and partial-row
norm/Mumford checks; final group replay is included in the diagnostic.

== Native diagnostic
Public p/seed pairs: (53,2), (101,409), (1009,389). Each has G and a planted
six-point sum. Six/unit/six/unit order, fresh solver seed947, degree24,
staircase64, 30-second solve cap. All24 records complete, complete coordinate
sets and cofactor geometry agree, repeats have exact tallies, and foreign-thread
hooks are zero. No cap or failure is treated as an empty set. [pcc:unverified]

#table(columns: (0.6fr, 1.6fr, 1.5fr, 1.5fr, 1fr), inset: 4pt,
  [*p*], [*Residual*], [*Six variables*], [*Unit elimination*], [*Unit / six*],
  [53], [G], [390320222], [354082244], [0.907158],
  [53], [Planted six], [393984098], [357905368], [0.908426],
  [101], [G], [516267201], [480730918], [0.931167],
  [101], [Planted six], [524285687], [462874792], [0.882867],
  [1009], [G], [998834924], [899422406], [0.900472],
  [1009], [Planted six], [1977157172], [916578068], [0.463584],
)

The aggregate is 3471593796 / 4800849304 = 0.7231207597, weighting each
fixture once. It estimates no descent-stream distribution. Matrices fall from
231 rows to 162–175. The fixed cohort contains three planted full rows and
one empty coordinate set. Separate correctness coverage finds a positive
quadratic partial at [10]G on p101/seed409 and checks identical geometry and
its group identity; it is not added to the cost cohort. [pcc:unverified]

== Scope and reproducibility
Frozen measurement source: `e4af92fbc62ac21b6ae53d45d961de50ed60275f`.
Unmodified library: `232925556a6e39d4589932c60e75a1bf24f9e049`.
Rust1.100.0, native release build; one Rayon worker owns all counters.
Raw JSONL, Rust analyzer, binary/library manifests, tests and commands are
beside this report. Setup of the public fixture is outside the component.
Container/control work, some inherited arithmetic, PRNG internals and
serialization remain unpriced. Complete cost and rho-parity fields are null.
No paid launch or current-main global-build assertion is made.

Next: production descent-stream integration and an admitted cold recovery
comparison with complete costs and the strongest matched rho. Paid execution
requires the original budget's remaining balance, currently unknown.

Source: Joux–Vitse, #emph[Cover and Decomposition Index Calculus on Elliptic
Curves Made Practical], sections3.1–3.3; author PDF SHA256
`7ca88c54ebfcb6fac8d05d313bfe8c26d3cb133081c55c4ab6ea4c1b45729b38`.
See PLAN.md and README.md. PCC-20261117-817df2 remains unverified.
