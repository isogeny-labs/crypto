= Closure degree at m \= 4, (n, ℓ) \= (13, 5) and (13, 6) (lead L1, 2026-10-18)
Lead L1 of #link("../ecdlp_exponent_assessment_20261018/LEADS.md")[`../ecdlp_exponent_assessment_20261018/LEADS.md`]. Class: #strong[measurement] (degree observations on frozen draws); no cost row, no improvement claim, no ECC2K-130 inference.

#image("closure_m4.png")

== What ran
`run.sh gen` drew `m = 4` Koblitz chain systems with `examples/dreg_ladder` (`--cells n:ℓ:3 --m 4 --unsat 4 --ffd-max 3 --seed 20261016`): 67 draws at `(13, 5)` and 441 at `(13, 6)`, four unsatisfiable each (`draws/*.jsonl`, `draws/*.log`). `run.sh measure` ran the dense MutantXL closure (`examples/dreg_closure --to 4 --max-cols 200000`) on each unsatisfiable draw, one at a time, under a 13 GB address-space cap and a 3 h wall, on a 15.7 GB, 4-core host (`runs/*.jsonl`, `runs/*.log`, `runs/queue.log`). Two interruptions are logged in `queue.log` (the session\'s background-task limit, and a pause while the L4 msolve run held 5–9 GB); each interrupted draw was discarded and rerun from the start. `plot.py` draws the figure from `runs/` only.

== Results
#align(center)[#table(
  columns: 8,
  align: (col, row) => (left,right,right,left,left,right,right,right,).at(col),
  inset: 6pt,
  [cell (n, ℓ, m)], [vars], [draw], [degree 3: columns, closure rank, XL rank], [degree 4: columns, closure rank, XL rank], [closure degree], [peak RSS], [degree-4 time],
  [(13, 5, 4)],
  [46],
  [18],
  [7 051, 2 019, 636],
  [179 447, #strong[1], 15 143],
  [4],
  [8.5 GB],
  [841 s],
  [(13, 5, 4)],
  [46],
  [19],
  [6 831, 2 019, 636],
  [179 397, #strong[1], 15 143],
  [4],
  [8.1 GB],
  [563 s],
  [(13, 5, 4)],
  [46],
  [43],
  [7 807, 2 018, 636],
  [179 447, #strong[1], 15 143],
  [4],
  [10.0 GB],
  [767 s],
  [(13, 5, 4)],
  [46],
  [66],
  [7 965, 2 018, 636],
  [179 423, #strong[1], 15 143],
  [4],
  [8.6 GB],
  [511 s],
  [(13, 6, 4)],
  [50],
  [258, 352, 362, 440],
  [8 507–8 538, 2 244, 688],
  [not built],
  [≥ 4 (not refuted at 3)],
  [0.01 GB],
  [–],
)
]

Every `(13, 5)` unsatisfiable draw is refuted by the closure at degree 4 (rank 1) and by neither engine at degree 3; XL does not refute at degree 4 on any draw (its rank is 15 143 of about 179 400 columns), as in every earlier `m = 4` cell. The `(13, 6)` draws stopped at degree 3 because the degree-4 matrix has 251 176 columns, above the 200 000-column cap; at the dense closure\'s measured memory (0.59 × `3·cols²/8`, `research/dreg_closure_m4l6_20261018`) it needs about 14 GB, which this host cannot hold under its cap. They are recorded as not measured at degree 4.

== Reading
- `(m, ℓ) = (4, 5)` at `n = 13` (`m·ℓ = 20`, `N = 46`) holds at degree 4 on 4 of 4 unsatisfiable draws. This extends the \"flat at 4 through `N = 42`\" line of `ic_leads_review_20261018` §2.1 item 3 to `N = 46`.
- `(m, ℓ) = (4, 6)` is answered on `main` at `n = 15` (`research/dreg_closure_m4l6_20261018`, 256 GB host): refuted at degree 4 on 2 unsatisfiable draws with the satisfiable control not refuted, so \"`m·ℓ ≥ 21` raises the degree\" is refuted and \"`ℓ = 7`\" survives. The `(13, 6)` cell here would have added two more unsatisfiable draws to that reading; it needs a host with about 20 GB free.
- L1\'s question (\"is the closure degree set by `m·ℓ`?\") is therefore answered no for `m·ℓ` up to 24 at `m = 4`; what remains open is `m = 3, ℓ = 7` at degree 5 (C1 of the `(4, 6)` protocol, about 250 GB).

== Distance to rho
None stated: these are degree observations without a cost row. The ECC2K-130 `m = 4` bracket that rests on them is unchanged (`research/dreg_closure_m4l6_20261018/RESULTS.md`, \"Consequence\").

== Next
Run the four `(13, 6)` draws on a host with at least 20 GB free (`run.sh measure` resumes where it stopped; about 15 min each), or fold them into the `(4, 6)` protocol\'s A2 draws.
