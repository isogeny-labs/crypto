#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set par(justify: true)

= Run report: one invariant guess per link at m = 4, degree 3, k = 10

- *Study:* `research/dreg_m4_link_guess_d3_20261106/`, pre-registered
  before any study run. One amendment: the runner's F4 caps, before any
  outcome was read.
- *Run:* 2026-11-06 16:33 UTC to 17:13 UTC.
- *Class:* stage diagnostic. No `S`, no rho ratio, no speed claim.
- *Standing:* unverified assertion until a pcc verdict exists.

== What ran

- *Chain:* `S₃(x₁, x₂, e₁)`, `S₃(e₁, x₃, e₂)`, `S₃(e₂, x₄, x(R))` at ℓ = 5,
  n = 11 and 12. Every committed unsatisfiable draw: 8 in all.
- *Arm I:* four pair sums on `x₁ + x₂`, three guesses on `e₁ + x₃` and three
  on `e₂ + x₄`. The link coordinates are the three largest k with |T_k| ≥ 2
  on the draw.
- *Arm R:* ten raw guesses in the same places (top coordinates of x₂, x₃,
  x₄).
- All 1024 branches of both arms on every draw: 16,384 runs of
  `dreg_linear_guess --to 3 --check`, plus the identity control N.

== Completion

- Every run left a record; none was killed.
- 32 to 34 variables occur after the guesses. The degree-3 closure took
  0.002–0.51 s per branch.
- The first attempt (runner without the F4 caps) is kept, unread, in
  `runs/attempts/run1-no-f4-env-20261106/`.

== Measured values

#figure(image("branches.svg", width: 100%),
  caption: [Branches refuting at degree 3, per arm and draw. Source:
  `plot.py` (layout only) on `plot-data.json` from
  `dreg_m4_link_score --plot-data`.])

- *Controls:* valid. N matches the committed m = 3 record, every record
  passes the identity and exactness checks with arm I's link guesses
  re-derived from the basis, and all branches are present.
- *Pooled:* I refutes on 1,888 of 8,192 branches, R on 1,312.
- *By draw:* I > R on 4 draws, R > I on 3, equal (both 0) on 1.
- *Registered verdict:* no detected difference (two-sided exact sign test,
  p = 1.0).
