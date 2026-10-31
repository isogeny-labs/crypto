#set page(paper: "a4", margin: 1.8cm)
#set text(size: 9.5pt)
#set table(stroke: 0.4pt)

= Run report: `f4_fp` before and after \#1717 in the Jacobian-cover index calculus, run-20261018T1548Z

*Observations only.* Interpretation: `../../REPORT.pdf`.

== What ran

- *Runner:* `research/f4_fp_ic_effect_20261018/run.sh <jv_cover_main> <jv_cover_new> <this dir>`.
- *Arms:* release builds of `examples/jv_cover.rs` with `src/cryptanalysis/f4_fp.rs` as on `main` before \#1717 (`4b6e09b78`, "before") and as merged in \#1717 (branch `f4-fp-msolve-gap` at `3f5b08ff7`, "after"). SHA-256 in `raw/versions.txt`. Nothing else differs.
- *Pipeline:* Nagao decompositions on a genus-2 cover of $E(FF_(p^6))$; every decomposition test is one `f4_fp` solve (degree bound 24, budget 300 s).
- *Experiments* (same seeds in both arms):
  - `--exp ccov --sizes 53,101,251 --seeds 1 --residuals 100 --constructed 20` measures the cost of one decomposition test, $C_"cov"$: 100 random residuals plus 20 planted decomposable ones per prime, checked against a meet-in-the-middle oracle where the oracle is cheap.
  - `--exp dlp --sizes 53,101 --seeds 2 --rho-runs 4 --check-every 64` runs the DLP end to end: relations, linear algebra and verified logarithm, against 4 Pollard-rho runs on the same curve.
- *Order:* ccov before, ccov after, then dlp before, dlp after. The counted costs are deterministic; wall times are listed but not compared.
- *Host:* Claude Code cloud container, `Intel Xeon @ 2.80GHz`, 4 vCPU, shared, isolation L0.
- *Requested / completed:* 4 of 4 invocations completed, exit 0.

== One decomposition test (`ccov`, means over 120 tests per prime)

#figure(image("ccov.svg", width: 100%), caption: [$C_"cov"$ split by stage, run-20261018T1548Z.])

#table(columns: 7,
  [p], [planted found], [$C_"cov"$ before], [$C_"cov"$ after], [F4 mults before → after], [F4 degree], [rows],
  [53], [20/20, 20/20], [6.158e6], [1.224e6], [5.671e6 → 7.380e5], [10.0 → 8.0], [769 → 235],
  [101], [20/20, 20/20], [6.172e6], [1.223e6], [5.686e6 → 7.369e5], [10.0 → 8.0], [767 → 231],
  [251], [20/20, 20/20], [6.253e6], [1.284e6], [5.713e6 → 7.447e5], [9.9 → 8.0], [767 → 231],
)
The linear stage is identical in both arms (4.82e5, 4.82e5, 5.35e5 mults). At p = 53 the oracle checked 120 tests per arm with 0 mismatches.

== End to end (`dlp`)

#figure(image("dlp.svg", width: 100%), caption: [$S_"IC" slash S_"rho"$ per prime and seed, run-20261018T1548Z.])

#table(columns: 8,
  [p], [seed], [relations], [solved, correct], [$S$ before], [$S$ after], [$S slash S_"rho"$ before], [$S slash S_"rho"$ after],
  [53], [1], [31], [yes, yes], [4734], [936.9], [4297], [850],
  [53], [2], [33], [yes, yes], [2121], [425.2], [1843], [370],
  [101], [1], [53], [yes, yes], [1455], [287.5], [981], [194],
  [101], [2], [50], [yes, yes], [2370], [466.4], [1897], [373],
)
Both arms see the same residual streams and find the same relations; every logarithm is correct. $S_"rho"$ is the mean of 4 runs (1.10 ± 0.20, 1.15 ± 0.69, 1.48 ± 0.52, 1.25 ± 0.58). Wall time of the whole dlp invocation: 912 s before, 370 s after, including the identical rho runs.

== Records

`raw/{ccov,dlp}_{main,new}.{json,log}`, `raw/walls.txt`, `raw/versions.txt`; `../../summary.json` is the frozen summary the leaderboard reads; graphs from `../../plot.py`, which reads `raw/` only.
