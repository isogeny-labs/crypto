#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set par(justify: true)

= Run report: the summand-swap invariant on the chained S₃ descent

- *Study:* `research/dreg_pair_invariant_20261028/`, pre-registered in
  `PREREGISTRATION.md` (commit `d708ad2ee`; Amendment 1 after the first
  secondary run capped, before scoring).
- *Run:* 2026-10-28 (UTC): control 05:51–06:15, primary and soundness
  09:30–09:31, secondary 15:21–17:11 (one sparse run, capped).
- *Class:* stage diagnostic. No `S`, no rho ratio, no speed claim.

== What ran

- *Tool:* `examples/dreg_invariant.rs` builds three arms from the same
  committed draw.
  - B: the plain m = 3 chain.
  - Q: adds the swap invariant `p = x₁x₂` over a basis of `span{v_i v_j}`.
    The first link becomes `s²e₁² + p e₁ + p² + b`, and every equation is
    quadratic.
  - R: the matched control, adding `r = x₁e₁`. It is also quadratic, but
    not invariant.
  - `--check` proves each construction by an exact polynomial identity.
- *Draws:* the committed m = 3 draws at (22, 6) (driver study, draws 0–3)
  and at (21, 7) (scale study: draws 0, 2, 3, 6 unsatisfiable, 1, 4, 5
  satisfiable).
- *Engine:* MutantXL closure (`closure_refutation_scan`, dense, at most
  130,000 columns). The secondary step used the sparse closure at degree 4,
  capped at 10⁹ entries.
- *Limits:* `prlimit --as=15e9`, `timeout 7200`, one process at a time.
- *Host:* the 4-core, 16 GB container, not isolated. Iso Lab EKS is not
  reachable from it (no `aws` CLI or credentials). The outcomes are exact
  algebra; the timings are indicative only.
- *Binary:* `dreg_invariant`, SHA-256 in `runs/binary.sha256`.

== Completion

- 30 measurement processes exited 0.
- One secondary run was started under a 10-minute tool cap and stopped by
  hand; it was relaunched detached.
- One secondary run was stopped by Amendment 1, 39 s after it started.
- Both stopped runs left empty outputs, kept in their own directories.
- Peak RSS: 5.1–5.8 GB (B at degree 4), ≤ 0.13 GB (Q and R at degree 3) and
  12.0 GB (the sparse run).

== Measured values

#figure(image("invariant.svg", width: 100%),
  caption: [Left: the degree-3 closure rank as a share of the degree-3
  columns, one point per draw; no arm refutes at degree 3. Right: columns of
  the degree-4 Macaulay matrix for each arm (draw 0). Source: `plot.py`
  (layout only) on `plot-data.json` from `dreg_invariant_score --plot-data`.])

- *B:* refutes at degree 4, not 3, on 4 of 4 draws at (22, 6). Saturates
  degree 4 without refuting on 4 of 4 at (21, 7). This matches the committed
  records.
- *Q and R:* do not refute at degree 3 on any of the 8 unsatisfiable draws.
  Degree 4 is unmeasured: the sparse run capped, and the dense matrix needs
  523,686–637,393 columns.
- *Validity:* the identity check is true on 22 of 22 systems, the positive
  control is reproduced, and no satisfiable draw is refuted. Valid.
- *Registered verdicts:* H_none at (22, 6), which is measured, and H_none
  at (21, 7), which follows the registered convention for an unmeasured
  degree 4. Q is unmeasured below B's degree on 4 of 4 draws at (21, 7), so
  a lower degree is not excluded there.
- *Per-run values:* in `score-output.txt` and `runs/`.
