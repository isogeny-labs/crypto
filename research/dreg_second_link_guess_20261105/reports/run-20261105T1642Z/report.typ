#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set par(justify: true)

= Run report: an invariant guess on the second link at ℓ = 8

- *Study:* `research/dreg_second_link_guess_20261105/`, pre-registered
  before any L3 run. No amendments.
- *Run:* 2026-11-05 16:42 UTC to 22:33 UTC.
- *Class:* stage diagnostic. No `S`, no rho ratio, no speed claim.
- *Standing:* unverified assertion until a pcc verdict exists.

== What ran

- *Arm L3:* `x₁₅ := x₇ + c`, `x₄₄ := h + Σ_{t∈T₂₀} x_{16+t}` (coordinate 20
  of `e₁ + x₃`), `x₂₄ = g`, with 8 branches, on the 12 committed
  unsatisfiable (22, 8) draws.
  - It runs a new `dreg_linear_guess` build with multi-source `--sum` and
    `--link2`, at `--to 4 --max-cols 140000 --check`.
  - Early stop at the first branch that saturates degree 4 without refuting.
- *Comparator S3:* the committed records of the two earlier studies.
- *Identity control N:* S3 on draw 5 branch 0, re-run with the new build.
- *Limits:* `prlimit --as=15e9`, `timeout 7200`, one process at a time, on
  the 4-core, 16 GB container, not isolated.

== Completion

- 74 processes started; 73 exited 0.
- One run, L3 on draw 11 branch 7, was killed by a container restart with
  empty output. That attempt is kept in `runs/attempts/`, and the retry
  completed.
- Every L3 system has 43 occurring variables. The degree-4 scan took
  147–351 s per run.

== Measured values

#figure(image("branches.svg", width: 100%),
  caption: [Branches per arm and draw: refuting at degree 4, saturating degree
  4 without refutation, or not run. Source: `plot.py` (layout only) on
  `plot-data.json` from `dreg_second_link_score --plot-data`.])

- *Controls:* valid. N matches the committed record, every record passes the
  identity and exactness checks, and every draw is decided.
- *L3* suffices on 6 of 12 draws, and *S3* on 7 of 12.
- *Discordant draws:* L3-only 0, S3-only 1 (draw 27).
- *Registered verdict:* no detected difference (two-sided exact sign test,
  p = 1.0).
- *Where both fail:*
  - L3 fails later on draws 0 (b6 against b4) and 14 (b6 against b2).
  - L3 fails earlier on draw 17 (b4 against b6).
  - L3 fails at the same branch on draws 3 and 25.
