#set page(paper: "a4", margin: 1.8cm)
#set text(size: 9.5pt)
#set table(stroke: 0.4pt)

= The faster `f4_fp` in the end-to-end index calculus: 5.0× cheaper, same relations

2026-10-18. Run report: `reports/run-20261018T1548Z/report.pdf`. Engine change: PR \#1717 (`research/f4_fp_msolve_parity_20261011`).

== Question

\#1717 made `f4_fp` level with msolve on generic $FF_p$ systems. Which index-calculus pipelines call it, and what does the change do to their end-to-end cost?

== Where `f4_fp` runs

The Koblitz binary-field pipeline does not use `f4_fp`; it runs the Boolean engines (inherited F4, F6-IC, `gf2_elim`). `f4_fp` is the decomposition solver of the prime-field and cover pipelines: `jv_cover`, `jv_quartic`, `jv_quintic`, `jv_sieve`, `glv_gaudry`, `coordinate_descent`, `sig_fp_tower`, and the P-256 factor-base binaries. This study measures `jv_cover`, an index calculus on $E(FF_(p^6))$ through a genus-2 cover with Nagao decompositions, end to end.

== Result

#figure(image("dlp.svg", width: 100%), caption: [End-to-end $S_"IC" slash S_"rho"$, before and after \#1717, run-20261018T1548Z.])

#table(columns: 5,
  [p, seed], [$S$ before ÷ after], [$S slash S_"rho"$ before], [$S slash S_"rho"$ after], [answer],
  [53, 1], [5.05], [4297], [850], [correct, same relations],
  [53, 2], [4.99], [1843], [370], [correct, same relations],
  [101, 1], [5.06], [981], [194], [correct, same relations],
  [101, 2], [5.08], [1897], [373], [correct, same relations],
)
Ratios derived from the recorded $S$; the class is *engineering* (same algorithm, cheaper engine).

#figure(image("ccov.svg", width: 100%), caption: [Cost of one decomposition test by stage.])

Per decomposition test, the F4 stage costs 7.7× fewer field multiplications (5.7e6 → 7.4e5) and $C_"cov"$ 5.0× fewer (6.2e6 → 1.2e6); the reduction is the same at p = 53, 101 and 251. Gebauer–Möller drops the degree-9 and degree-10 steps that only reduced redundant pairs: F4 now stops at degree 8, with 231 rows instead of 767. The linear stage after F4 is unchanged (4.8e5) and is now 40 % of a test, so it is the next lever in this pipeline.

== Distance to rho, scope and limits

- *Distance to rho.* After the change this pipeline still costs 194–850× the matched Pollard rho at these sizes (4 rho runs each, so the reference is noisy). That is the distance remaining.
- *Lower-bound comparison.* Neither arm emits counts for its unpriced native work, so the §5a band cannot be applied: $S slash S_"rho"$ is a lower-bound comparison, not a ranking.
- *What is measured.* Counted cost, deterministic and identical across hosts. Wall time (912 s → 370 s for the whole dlp invocation, rho runs included) is shown as an observation only; the host is shared (isolation L0).
- *Scope.* p = 53 and 101 end to end, p up to 251 per test, one run each. The engine change is the only difference between the arms.
- *Leaderboard.* Updated in this PR with its own component section, outside tables A–C, read from `summary.json`.
