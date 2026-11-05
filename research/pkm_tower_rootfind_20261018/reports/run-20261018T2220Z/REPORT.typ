#set page(paper: "a4", margin: 1.8cm, numbering: "1")
#set text(size: 9.5pt)
#set heading(numbering: "1.")
#show table: set text(size: 8pt)

#align(center)[
  #text(size: 15pt, weight: "bold")[Root finders against tower F4, signature F5 and the F6 analogue on prime curves] \
  #text(size: 10pt)[Run report, `ecbench`, 2026-10-18 22:20 UTC onward (run `run-20261018T2220Z`)]
]

= What ran

- *Protocol.* `research/pkm_tower_rootfind_20261018/PROTOCOL.md`, committed
  before any measured run.
- *Amendment 1.* It adds the AGENTS.md §5a pricing band and records the
  P8 rerun.
- *Code.* Commit `ebed0cc51`, with one `ecbench` binary in every session.
- *Host.* A cloud VM, isolation L1 or L2 as recorded.
- *P8.* A worker restart killed the first P8 after 20 of 36 runs. It is
  kept in `interrupted/`, and P8 was rerun in full.
- *Base.* Every IC arm used `prime-kummer:t=T,g=G` with `G` = 37, 684, 7047
  and 2551. That is the 2026-10-11 round's base point for point
  (`points_sha256`).
- *Targets.* The same target seed, 202610112301, so the same targets.
- *Arms.*
  - Gröbner: `f4` (tower F4), `sig-f5` (signature F5) and `f6` (the F6
    analogue), all through `pkm-tower`.
  - No-algebra control: `enum`.
  - Root finders: `rootfind-subtract:m=2` and `rootfind-mitm:m=2`.
  - Context: `rootfind-mitm:m=3`, `rootfind-pairs:m=3` and `rho.negation`.

#table(
  columns: 6,
  align: (left, left, left, right, right, left),
  table.header([spec], [session], [curve], [targets], [verified], [replay]),
  [P5], [`ECBS1h2306d857d0af`], [`icv1-fp9-t25-33fa1ca2`], [8], [72/72], [ok],
  [P6], [`ECBS1hf6c3a80d1387`], [`icv1-fp11-t43-4d73e7f7`], [8], [72/72], [ok],
  [P7], [`ECBS1hede9211000ad`], [`icv1-fp13-tm41-756f569a`], [8], [72/72], [ok],
  [P8], [`ECBS1h3c465cff7dc5`], [`icv1-fp15-t181-50ff72d2`], [4], [36/36], [ok],
)

- *Instrument check.* `f4`, `sig-f5` and `f6` reproduce the 2026-10-11
  round's counts on 28 of 28 targets.
- *Sources.* Raw records are in `sessions/<spec>/records.jsonl`. Derived
  values are in `ANALYSIS.json`, written by
  `examples/pkm_rootfind_analyze.rs`.

= Measured values (medians over 8 targets, 4 at P8)

*Units.*
- `S`: charged GAE over √r.
- `M`: `F_p` multiplications, derived as 3 × additions + 7 × doublings
  plus the solver's multiply-adds.
- Inversions: one per addition or doubling, plus native batched
  inversions.

#table(
  columns: 9,
  align: (left,) + (right,) * 8,
  table.header([cell], [F4 S], [F5 S], [subtract S], [mitm S], [F4 M], [F5 M], [subtract M], [mitm M]),
  [P5], [25.84], [25.84], [29.02], [41.53], [8.05e7], [1.82e7], [2,190], [2,762],
  [P6], [15.55], [15.55], [36.03], [35.74], [2.35e9], [6.51e8], [5,200], [5,192],
  [P7], [9.36], [9.36], [35.72], [58.08], [4.07e10], [1.25e10], [11,039], [16,882],
  [P8], [7.48], [7.48], [49.97], [126.25], [4.86e11], [1.59e11], [22,029], [52,947],
)

#figure(image("s_per_arm.svg", width: 100%),
  caption: [`S` per arm and cell, median and range, sessions P5–P8. The root
  finders charge every addition. The tower arms' `S` omits their multiply-adds.])

#figure(image("fp_multiplications.svg", width: 100%),
  caption: [Derived `F_p` multiplications per run. F4 and F5 are dominated by
  solver multiply-adds; `enum` and the root finders have none.])

#figure(image("inversions.svg", width: 100%),
  caption: [Derived `F_p` inversions per run. The root finders do 1.15–17 times
  F4's, one per affine addition.])

= The protocol's questions against F4 and F5 (derived; rules as written)

- *Q1, `S(root)/S(F4)`* (identical against F5).
  - `rootfind-subtract`: 1.16, 2.23, 3.53, 6.70.
  - `rootfind-mitm`: 1.62, 2.29, 6.24, 16.9.
  - Rule: *not decided by S* in every cell. F4's `S` is a lower bound.
- *Q2, `M(root)/M(F4)`.*
  - `rootfind-subtract`: 2.8e-5, 2.3e-6, 3.3e-7, 4.6e-8.
  - Against F5: 1.2e-4, 8.3e-6, 1.1e-6, 1.4e-7.
  - The root finders have more inversions on every target, so the rule
    reads *mixed*.
  - For F4 to be cheaper, an inversion would have to cost 1.3e6 (P5) to
    8.4e7 (P8) multiplications.
- *§5a band* (`S_w`, w = 0, 0.1, 1 GAE per unpriced operation).
  - At w = 0 the root finders are above F4/F5. At 0.1 and 1 they are below
    by 4–8 orders of magnitude.
  - Verdict: *unresolved inside the band*, in every cell.
  - Derived break-even weight, `rootfind-subtract` against F4: 8.5e-7,
    3.6e-7, 6.9e-8, 1.2e-8 GAE.
- *Relation-phase wall, root / F4:* 1e-4 (P5) down to 1e-6 (P8).
- *Context.*
  - `S`/rho: `rootfind-subtract` 5.4–29.2, F4 3.9–5.4 (lower bound).
  - The `m = 3` root finders cost 1.29–1.90 times `rootfind-mitm` at
    `m = 2`.

= Limits

- *Derived quantities.* `M` and the inversions use the curve's fixed
  per-operation costs.
- *Practicality notes.* Wall time comes from mixed L1/L2 runs.
- *Scope.* No arm is below rho. There is no exponent, from four sizes up
  to 15 bits. The head-to-head is at `m = 2` only.
- *Interpretation.* It is in the study's `RESULT.md`.
