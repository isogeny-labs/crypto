#set page(width: 8.5in, height: 11in, margin: 0.7in)
#set text(font: "New Computer Modern", size: 10pt)
#set heading(numbering: none)
#align(center)[#text(17pt, weight: "bold")[EXP-PFDR-ada71b break-even growth ladder v2]]
#align(center)[Run report (unsupervised rehearsal) --- 2026-11-01]
#v(0.4em)
#table(columns: (1.5in, 4.9in),
  [Experiment / hypothesis / goal], [EXP-PFDR-ada71b (approved DEC-20261101-cb3277) / H-PFDR-ed7ccf / GOAL-PFDR-fd09e0],
  [Run IDs], [RUN-PFDR-a4895b selftest, 4fa30b pilot, efce3d census-main, 2d655b analyze-main, b5f267 census-rep, fe2d35 analyze-final],
  [Status of these records], [#text(weight: "bold")[Rehearsal, not an admitted run.] The supervisor requires the task claim to be visible in origin refs; this session cannot push (see Limits). Same plan and sources, run by hand in a scratch tree; no claim, no execution receipts; manifests record commit null.],
  [Frozen inputs], [specification sha256 51753c34e9...; trial-plan 6750eea391...; run.py, check.py, paircount.c, curve.py hashes equal trial-plan source_sha256 (verified before each run)],
  [Recorded reading], [#text(weight: "bold")[O-SUB-BE] (analysis.json /reading, analyze-main and analyze-final)])

== What ran
Six trials in plan order, each followed by its `check.py` (all `CHECK OK`). Selftest passed 126 kernel-vs-brute checks, 9 near-2^32 key checks, 6 `plant_internal` checks and 12 order certifications. Pilot: 32 curves per (rung, theta), 960 jobs, 30 cells, 40 plant entries, none infeasible, 20,151 curves planned. Census-main: 20,151 jobs, 0 failed_infrastructure. Census-rep: empty (no ALIVE line). Wall time on this 4-core Linux container: pilot 153 s, census 402 s. Pilot and census were re-run once more from scratch; `design.json` and `main-jobs.jsonl` are identical modulo `seconds` and record order.

== Recorded values
#figure(image("graph.svg", width: 100%), caption: [A: recorded kappa per rung with the bootstrap 95% interval (RUN-PFDR-efce3d, RUN-PFDR-fe2d35). B: recorded slope and interval of every line; crosses mark the plants' design slopes.])
#table(columns: (0.55in, 0.35in, 0.9in, 0.8in, 1.3in, 1.1in), table.header([theta],[d],[class],[delta_hat],[95% interval],[kappa(32) / M(16)]),
  [0.30],[1],[SUB-BE],[+0.0047],[-0.0041, +0.0136],[0.985 / 3836],
  [0.30],[2],[SUB-BE],[-0.0044],[-0.0123, +0.0039],[0.974 / 3836],
  [0.30],[3],[SUB-BE],[+0.0047],[-0.0044, +0.0131],[1.027 / 3836],
  [0.35],[1],[SUB-BE],[+0.0070],[-0.0004, +0.0144],[0.995 / 4163],
  [0.35],[2],[SUB-BE],[-0.0001],[-0.0082, +0.0076],[1.014 / 4163],
  [0.35],[3],[SUB-BE],[+0.0066],[-0.0011, +0.0138],[1.014 / 4163])
Largest recorded upper95(delta) over the six valid lines: 0.0144, against delta\* = 1/3 (derived: 0.3333/0.0144 = 23.1). Recorded per-rung kappa range 0.916 to 1.058; largest recorded upper95 kappa at 32 bits 1.105. Minimum M over every rung of every line (structured, plant, Z/N, A/A) is 3670 (floor 2500).

== Recorded controls
#table(columns: (0.5in, 0.6in, 5.0in), table.header([control],[pass],[recorded]),
  [C1],[yes],[selftest as above],
  [C3],[yes],[plants recover design slope: |error| 0.0005 to 0.0128 (tolerance 0.03); classes gamma 0 SUB-BE, 1/6 SUB-BE-GROWING, 1/2 ALIVE at both thetas],
  [C4],[yes],[Z/N delta 0.6992 (expected 0.70) and 0.6452 (expected 0.65), both ALIVE],
  [C5],[yes],[six distinct A/A lines, all intervals contain 0, none ALIVE; delta_hat from -0.0062 to +0.0046],
  [C6],[yes],[30 distinct z_cal: mean -0.243 (band 0.40), sd 0.899 (band 0.70 to 1.30)])

== Limits and missing data
- Provenance: scratch-tree rehearsal; no supervisor receipts, claim, commit binding, isolation tier or independent replay. A supervised run of the same plan should reproduce every count.
- Raw records are in `rehearsal-records/` (gzipped; large files are not meant for Git and are covered by the session checkpoint). The graph plots recorded values only; `delta_hat` and intervals are the frozen analysis' own output.
- This report states no review verdict. Reviews: `analysis/redteam_report.md` (adversarial read), `NOTE-diagnosis-and-verification.md`.
