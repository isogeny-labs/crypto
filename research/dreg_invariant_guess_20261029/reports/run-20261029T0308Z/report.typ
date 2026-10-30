#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set par(justify: true)

= Run report: invariant against raw guesses at ℓ = 8

- *Study:* `research/dreg_invariant_guess_20261029/`, pre-registered
  (commit `d0b05eee0`). Amendment 1, made after the data, replaces the
  soundness rule and is disclosed as post hoc.
- *Run:* 2026-10-29, 03:08 to 11:53 UTC.
- *Class:* stage diagnostic. No `S`, no rho ratio, no speed claim.

== What ran

- *Tool:* `examples/dreg_linear_guess.rs` specialises the m = 3 chained S₃
  system. `--fix v=b` fixes a variable; `--sum j=i+c` substitutes
  `x_j := x_i + c`. Then it scans the MutantXL closure (dense, at most 140,000
  columns) at degrees 3 and 4.
- *Exactness:* `--check` evaluates every specialised equation against the
  original at 4,096 random points satisfying the guesses. It is true on every
  scored run, and a deliberately wrong build fails it.
- *Draws:* the committed (22, 8) draws (N = 46). Unsatisfiable draws 0, 3, 5,
  6; satisfiable draw 1.
- *Arms:* three guesses each, 8 branches, 43 occurring variables, 136,698
  degree-4 columns.
  - S3: `x₁₅ := x₇ + c` (the top coordinate of `s = x₁ + x₂`), `x₂₃`,
    `x₂₄`.
  - X3: `x₇`, `x₂₃`, `x₂₄`.
- *Early stop:* an (arm, draw) stops at its first branch that saturates
  degree 4 without refuting.
- *Limits:* `prlimit --as=15e9`, `timeout 7200`, one process at a time, on
  the 4-core, 16 GB container, not isolated. Iso Lab EKS is not reachable
  from it.

== Completion

- 42 processes exited 0, and 5 were killed with exit 137:
  - S3 draw 0 branch 3, twice: an unsatisfiable draw, unmeasured;
  - S3 branches 1, 5 and 6 on satisfiable draw 1, which are the branches
    holding its solutions.
- Two container restarts each killed one run in flight. Their empty outputs
  are in `runs/killed-by-restart/`.
- Peak RSS of the completed runs was 6.1–8.5 GB.

== Measured values

#figure(image("branches.svg", width: 100%),
  caption: [Branches per arm and draw: refuting at degree 4, saturating degree
  4 without refutation, or not run. Source: `plot.py` (layout only) on
  `plot-data.json` from `dreg_linear_guess_score --plot-data`.])

- *Controls:*
  - the instrument identity control matches the committed G3 record on every
    non-timing field;
  - the positive control refutes on 4 of 4;
  - soundness (V3′, post hoc): the 6 solutions of draw 1 lie only in the
    killed branches, and the 5 refuting branches hold none.
  - Valid.
- *S3:* suffices on draws 5 and 6, and fails on 0 and 3.
- *X3:* suffices on draw 6 only.
- *Registered verdict:* mixed (S3 split, X3 fails).
- *Per-run values:* in `score-output.txt` and `runs/`.
