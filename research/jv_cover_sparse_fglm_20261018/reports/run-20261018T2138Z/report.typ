#set page(paper: "a4", margin: 1.8cm)
#set text(size: 9.5pt)
#set table(stroke: 0.4pt)

= Run report: sparse-FGLM linear stage of `jv_cover`, run-20261018T2138Z

*Observations only.* Interpretation: `../../REPORT.pdf`.

== What ran

- *Runner:* `research/jv_cover_sparse_fglm_20261018/run.sh <jv_cover> <this dir>`; one release binary (sha256 in `raw/versions.txt`), arm chosen by `JV_LIN=dense|l1a|sparse`.
- *Experiments (same seeds in all arms):* `--exp ccov --sizes 53,101,251 --seeds 1 --residuals 100 --constructed 20 --max-degree 24 --budget 300`; `--exp dlp --sizes 53,101 --seeds 2 --rho-runs 4 --check-every 64`.
- *Host:* Intel Xeon 2.80GHz, 4 vCPU shared, isolation L0.
- *Completed:* 6 of 6 invocations, exit 0 (`raw/walls.txt`; wall times 43--49 s for ccov, 1081--1695 s for dlp, under varying contention: observation only).

== ccov (means over 120 tests per prime)

#figure(image("ccov.svg", width: 100%), caption: [$C_"cov"$ by stage.])

#table(columns: 6,
  [p], [arm], [lin mults], [$C_"cov"$], [fallbacks / nonsep.], [planted, mism., unverified, incomplete],
  [53], [dense], [487480], [1229631], [0 / 3], [20/20, 0, 0, 0],
  [53], [l1a], [249036], [991187], [4 / 0], [20/20, 0, 0, 0],
  [53], [sparse], [229647], [971798], [4 / 0], [20/20, 0, 0, 0],
  [101], [dense], [486265], [1227378], [0 / 2], [20/20, 0, 0, 0],
  [101], [l1a], [240879], [981992], [0 / 0], [20/20, 0, 0, 0],
  [101], [sparse], [199145], [940258], [0 / 0], [20/20, 0, 0, 0],
  [251], [dense], [534731], [1284369], [0 / 0], [20/20, 0, 0, 0],
  [251], [l1a], [268955], [1018593], [1 / 0], [20/20, 0, 0, 0],
  [251], [sparse], [230360], [979998], [1 / 0], [20/20, 0, 0, 0],
)

== dlp

#figure(image("dlp.svg", width: 100%), caption: [$S slash S_"rho"$ by arm.])

#table(columns: 6,
  [p, seed], [residuals], [relations], [solved, correct], [$S$ dense / l1a / sparse], [$S slash S_"rho"$ dense / l1a / sparse],
  [53, 1], [19020], [31], [yes, yes (all arms)], [937.2 / 752.8 / 731.6], [851 / 683 / 664],
  [53, 2], [8486], [33], [yes, yes], [424.5 / 337.2 / 328.8], [369 / 293 / 286],
  [101, 1], [40100], [53], [yes, yes], [287.8 / 232.9 / 224.4], [194 / 157 / 151],
  [101, 2], [65422], [50], [yes, yes], [466.3 / 379.5 / 365.5], [373 / 304 / 293],
)
Records: `raw/{ccov,dlp}_{dense,l1a,sparse}.{json,log}`, `raw/walls.txt`, `raw/versions.txt`; graphs from `../../plot.py`.
