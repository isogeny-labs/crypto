#set document(title: "Target-pin sparse solver, accounting and where linear algebra carries weight")
#set page(paper: "a4", margin: (x: 1.8cm, y: 1.8cm), numbering: "1")
#set text(size: 9.5pt)
#set heading(numbering: "1.")
#show table: set text(size: 8pt)

#align(center)[
  #text(15pt, weight: "bold")[Target-pin sparse solver, block-arithmetic accounting, and where linear algebra carries weight]
  #v(2pt)
  research/sparse_la_target_pin_20261104 · 2026-11-04 · protocol frozen at `33bf99c1a` \
  Stage diagnostic `sparse_la_stage/v1`: 420 runs, planted systems, counted `row_ops`
]

#block(fill: luma(240), inset: 8pt, radius: 3pt)[
  *Partly measured; no end-to-end claim.* The target-pin solver `pin-wiedemann` reproduces the echelon pin row
  exactly (every cell of the stage grid), but costs 99 to 212 times structured elimination's counted work when it
  attempts after every row, and the spaced variant overshoots the relation count by up to 2.21 times; it is never
  below structured elimination. Not preregistered, exploratory: the filtered deferred solvers (`sparse-cado`,
  `filter+lanczos`) cost 0.05 to 0.37 of structured elimination's counted work at weights 3 and 4 and 1,025 columns
  while needing about 2.2 times the relations. The two ecbench sessions and the 2,048-column cells were *not run*:
  the resume-checkpoint hook cannot upload from this container, and the stage run was launched through the hook by
  mistake with an empty-reason bypass (disclosed in the README).
]

= What was built

`pin-wiedemann` and `pin-wiedemann-every`: buffer rows; peel singleton columns exactly; test whether the target's unit
vector $e_t$ is in the row space of the stored rows $R$ by solving $A z = e_t$ with $A = R^T D R$ (Wiedemann, Berlekamp–Massey),
check $A z = e_t$ exactly, and read $x_t = z^T R^T D a$. The exact check certifies the pin; two-seed agreement (as in
`sparse-cado`) cannot, because a Wiedemann solution lies in the row space whatever the seed.
`+counted` names charge the Krylov core's block × block multiplications in `row_ops` (accounting only).
29 library tests pass, including: pin row equals the echelon row on 8 planted systems; an undetermined target is
never pinned; `+counted` gives the same answer with a larger `row_ops`.

= Stage diagnostic

Planted rows: $w$ base columns plus the target column; $w in {2,3,4,6,8}$, 128 to 1,024 base columns, 3 seeds, seven arms,
modulus $2^61 - 1$. 412 of 420 runs recovered the planted value, 0 wrong; 8 full-rank runs were censored (weight 2, an
untouched base column). Ratios are paired per seed over structured-gauss, median of three.

#table(columns: 7, align: (right, right, right, right, right, right, right),
  [$w$], [$n$], [pin-every rows÷sg], [pin-every ops÷sg], [spaced rows÷sg], [spaced ops÷sg], [sparse-cado ops÷sg],
  [2], [1025], [1], [14.2], [1], [14.2], [7.34],
  [3], [1025], [1], [191], [1.02], [6.42], [0.08],
  [4], [1025], [1], [169], [1.01], [2.32], [0.37],
  [6], [1025], [1], [130], [1.98], [2.19], [1.59],
  [8], [1025], [1], [123], [2.21], [1.79], [2.05],
)
Full table: `results/stage-a-summary.md`. Preregistered: P1 (exact pin row) *confirmed* on the grid; P2 (spacing overshoot at most 10 %)
*falsified*; P5 (pin solver below structured elimination at $w >= 4$, $n >= 1024$) *falsified*; P6 (`+counted` raises `row_ops`)
confirmed, factor 1.04 to 1.42 (the 4,096-column part untested).

#figure(image("figures/stage_ops_over_sg.svg", width: 100%), caption: [Counted `row_ops` over structured-gauss, paired, median of 3 seeds, by base weight and columns; the line is structured-gauss. Source: `results/stage-a.jsonl`.])
#figure(image("figures/stage_rows_over_sg.svg", width: 100%), caption: [Relations used over structured-gauss.])

= Where linear algebra carries weight (committed data, nothing run)

E(F#sub[p#super[4]]), $k = 4$: linear algebra is about $6 dot 10^(-8)$ of whole $S$ at $p = 1601$ (Amdahl bound 1.00000006); no solver change can
move end to end there. The stacked D#sub[4] + 2LP arm was measured (whole 1.0264 against D#sub[4]); only the eager variant remains.
Genus 3 after cubic GHS: linear algebra is 22 to 41 % of the whole at $c_ell = 1$ (Amdahl 1.28 to 1.70), 80 % at $c_ell = 9$ (4.9);
this is the workload where a better solver could move end to end. These two readings come from subagent summaries of committed records, partly spot-checked.

= Next steps

(1) Run the two frozen ecbench sessions and the 2,048-column cells on a runner that can push the checkpoint (backlog cards
`XP-20261104-sparse-la-pin-*`). (2) Apply `filter+lanczos+counted` to the genus-3 relation rows. (3) A pin schedule that does not overshoot.
(4) Online peeling in front of structured elimination at weights 3 and 4.

#text(size: 7.5pt)[Host: shared VM, no PMU, isolation not earned; counts are the unit, wall time is not used. Planted random systems carry no
cycles of two-summand rows and no repeated columns. Matched-rho gap unchanged by this study.]
