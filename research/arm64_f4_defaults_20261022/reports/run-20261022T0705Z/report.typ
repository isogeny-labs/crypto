#set page(paper: "a4", margin: 1.6cm)
#set text(size: 8.5pt)
= Arm64 F4/F5 defaults — run report
Run `run-20261022T0705Z` · workflow `arm64-f4-defaults.yml`, GitHub Actions run 39732230961, attempt 1 · PR merge commit `d6b94c91f` (branch head `c034f1639`) · 2026-10-22

== What ran
One binary per host and tool, arms by environment only, arm order rotated inside every round (`scripts/run_inh.sh`, `scripts/run_gf2.sh`). Decision rule: `scripts/DECISION_RULE.md`, committed before any Arm data.

- *Inherited F4 / F6-IC* (`f6_ic_probe --engines f4,f6p --reps 3`), nine cells as in `research/inherited_f4_eager_runs_20261016`, 5 rounds. Arms: *on* (defaults: eager rewrite through shift runs, gate-first F6-IC order), *noeager* (`KIC_F4_EAGER=0`: lazy rewrite with the bit scatter, what Arm hosts ran before \#2043), *nogate* (`KIC_F6_GATE_FIRST=0`). CPU = probe `cpu_ms` of the arm total.
- *GF(2)* (`f4_f2_bench`), 3 rounds: F5 cases ($n <= 24$, Reduced output) at 1 thread and at all cores; F4 cases ($n <= 20$) at 1 thread. Arms: *t4* (default, 4 Gray-code tables), *t2*, *t3* (`KIC_GF2_TABLES`), *reuse* (`KIC_GF2_REUSE_TABLE=1`). Time = bench `wall_ms`.

Hosts (`*/host.txt`):
- `ubuntu-24.04-arm`: Arm Neoverse-N2, 4 vCPU, L2 1 MiB per core, L3 128 MiB, Linux 6.17 (Azure), rustc 1.99.1; load 5.1 at start, 1.0–1.2 later. Shared, unpinned (L0).
- `macos-latest`: Apple M1 (virtual), 3 vCPU, L2 12 MiB, macOS kernel 25.6, rustc 1.99.1; the VM reported load average 36–50 at the start of the inherited arms and 1.8–14 at the end of the GF(2) arms. Shared, unpinned (L0); its timings carry much wider round-to-round spread.

Runs: inherited 135 per host, GF(2) 36 per host; every run exited 0 on both hosts.

== Exactness
On both hosts all 80 targets have the same decision tuple in every arm and round; each arm's word count is identical across rounds; the GF(2) fingerprints (basis / rows / row space, terms, rank, word XORs) agree across all arms and rounds for all 21 case keys. The word counts are identical on the two hosts, and *on* / *noeager* charge exactly what the x86 run `research/inherited_f4_eager_runs_20261016` recorded for *runs* = *bmi2* / *scatter*.

== Inherited F4 / F6-IC
Ratios are medians over 5 rounds (arm ÷ default); the bracket is the worst–best round pairing. Units: per-arm total `word_xors`.
=== `ubuntu-24.04-arm`
#table(columns: 8, align: (left, left, right, right, right, right, right, right),
  [cell], [engine], [CPU on ms], [noeager/on], [nogate/on], [units on], [units noeager], [units nogate],
  [`K1_17`], [f4], [455.1], [1.12 (1.11–1.12)], [1.00 (0.99–1.00)], [18019804], [15109049], [18019804],
  [`K1_17`], [f6p], [165.2], [1.11 (1.10–1.12)], [1.85 (1.83–1.87)], [10830979], [8865230], [15514870],
  [`K1_23`], [f4], [867.8], [1.28 (1.27–1.28)], [1.00 (0.99–1.00)], [62213299], [52572523], [62213299],
  [`K1_23`], [f6p], [357.6], [1.21 (1.20–1.22)], [2.10 (2.09–2.11)], [38122485], [32069197], [59881009],
  [`K0_23`], [f4], [2516.8], [1.30 (1.29–1.31)], [1.00 (0.99–1.01)], [167017420], [148425559], [167017420],
  [`K0_23`], [f6p], [819.3], [1.36 (1.35–1.37)], [2.01 (1.99–2.02)], [81022008], [67544699], [132377695],
  [`K0_31`], [f4], [2593.1], [1.61 (1.61–1.63)], [1.00 (0.99–1.01)], [290412310], [264887288], [290412310],
  [`K0_31`], [f6p], [1188.7], [1.57 (1.56–1.58)], [2.01 (2.00–2.03)], [164736149], [146257576], [285541596],
  [`K0_19`], [f4], [334.5], [1.13 (1.13–1.14)], [1.00 (1.00–1.00)], [17250812], [15282173], [17250812],
  [`K0_19`], [f6p], [119.6], [1.12 (1.11–1.13)], [1.69 (1.68–1.70)], [10226773], [8837211], [13660081],
  [`K1_19`], [f4], [581.2], [1.20 (1.20–1.21)], [1.00 (0.99–1.01)], [30159678], [25782851], [30159678],
  [`K1_19`], [f6p], [236.2], [1.15 (1.14–1.15)], [1.92 (1.91–1.93)], [19117122], [16026687], [28309592],
  [`K1_17d7`], [f4], [528.0], [1.08 (1.08–1.09)], [1.00 (1.00–1.00)], [22448248], [19473310], [22448248],
  [`K1_17d7`], [f6p], [157.5], [1.09 (1.08–1.10)], [1.82 (1.81–1.83)], [10867029], [8987079], [15297452],
  [`K1_29`], [f4], [1862.8], [1.52 (1.51–1.52)], [1.00 (1.00–1.00)], [185919270], [165390163], [185919270],
  [`K1_29`], [f6p], [870.0], [1.45 (1.45–1.46)], [1.96 (1.95–1.97)], [112675706], [98192767], [182738266],
  [`K0_37`], [f4], [4579.1], [1.69 (1.68–1.71)], [1.00 (1.00–1.00)], [652414757], [604262254], [652414757],
  [`K0_37`], [f6p], [2138.3], [1.62 (1.61–1.65)], [2.02 (2.01–2.03)], [376487242], [342788148], [644803622],
)
=== `macos-latest`
#table(columns: 8, align: (left, left, right, right, right, right, right, right),
  [cell], [engine], [CPU on ms], [noeager/on], [nogate/on], [units on], [units noeager], [units nogate],
  [`K1_17`], [f4], [604.8], [1.18 (0.81–1.76)], [1.04 (0.71–1.53)], [18019804], [15109049], [18019804],
  [`K1_17`], [f6p], [222.6], [1.11 (0.94–1.59)], [1.71 (1.41–2.54)], [10830979], [8865230], [15514870],
  [`K1_23`], [f4], [1161.5], [1.33 (1.20–1.45)], [0.98 (0.82–1.06)], [62213299], [52572523], [62213299],
  [`K1_23`], [f6p], [492.4], [1.27 (1.02–1.53)], [2.05 (1.55–2.30)], [38122485], [32069197], [59881009],
  [`K0_23`], [f4], [3348.0], [1.33 (1.03–1.57)], [1.01 (0.84–1.10)], [167017420], [148425559], [167017420],
  [`K0_23`], [f6p], [1020.6], [1.61 (1.06–2.00)], [2.15 (1.57–2.67)], [81022008], [67544699], [132377695],
  [`K0_31`], [f4], [3841.7], [1.42 (1.20–2.18)], [0.93 (0.82–1.32)], [290412310], [264887288], [290412310],
  [`K0_31`], [f6p], [1751.9], [1.53 (1.26–2.01)], [1.90 (1.74–2.66)], [164736149], [146257576], [285541596],
  [`K0_19`], [f4], [502.7], [1.17 (0.83–1.40)], [0.98 (0.74–1.20)], [17250812], [15282173], [17250812],
  [`K0_19`], [f6p], [173.2], [1.19 (0.84–1.67)], [1.76 (1.52–2.31)], [10226773], [8837211], [13660081],
  [`K1_19`], [f4], [883.8], [1.15 (0.87–1.66)], [1.06 (0.70–1.21)], [30159678], [25782851], [30159678],
  [`K1_19`], [f6p], [401.5], [0.97 (0.71–1.54)], [1.75 (1.21–2.27)], [19117122], [16026687], [28309592],
  [`K1_17d7`], [f4], [821.4], [1.09 (0.86–1.44)], [0.99 (0.78–1.28)], [22448248], [19473310], [22448248],
  [`K1_17d7`], [f6p], [246.0], [1.07 (0.79–1.43)], [1.62 (1.44–2.56)], [10867029], [8987079], [15297452],
  [`K1_29`], [f4], [2565.2], [1.44 (1.18–2.02)], [0.96 (0.79–1.32)], [185919270], [165390163], [185919270],
  [`K1_29`], [f6p], [1091.7], [1.52 (1.13–2.00)], [2.13 (1.53–2.67)], [112675706], [98192767], [182738266],
  [`K0_37`], [f4], [5871.8], [1.87 (1.47–2.13)], [1.01 (0.83–1.27)], [652414757], [604262254], [652414757],
  [`K0_37`], [f6p], [2648.5], [1.78 (1.57–2.30)], [2.10 (1.63–2.57)], [376487242], [342788148], [644803622],
)

#figure(image("fig_arm64.svg", width: 100%), caption: [Left: CPU ratio of each switched-off arm over the default per cell and engine (5 rounds; bars: worst and best round pairing). Right: GF(2) wall ratio arm / t4 per bench case (median of 3 rounds). run-20261022T0705Z.])

== GF(2) tables
Geometric mean, minimum and maximum over the cases of the per-case median ratio arm ÷ *t4* (below 1: the arm is faster).
=== `ubuntu-24.04-arm`
#table(columns: 7, align: (left, right, left, right, right, right, right),
  [tool], [threads], [arm], [geomean arm/t4], [min], [max], [cases],
  [f4], [1], [reuse], [1.004], [0.996], [1.008], [7],
  [f4], [1], [t2], [0.999], [0.995], [1.004], [7],
  [f4], [1], [t3], [1.001], [0.998], [1.008], [7],
  [f5], [1], [reuse], [1.001], [0.988], [1.025], [7],
  [f5], [1], [t2], [1.067], [0.986], [1.229], [7],
  [f5], [1], [t3], [1.031], [0.989], [1.091], [7],
  [f5], [4], [reuse], [1.014], [0.984], [1.081], [7],
  [f5], [4], [t2], [1.076], [1.001], [1.217], [7],
  [f5], [4], [t3], [1.033], [1.000], [1.085], [7],
)
=== `macos-latest`
#table(columns: 7, align: (left, right, left, right, right, right, right),
  [tool], [threads], [arm], [geomean arm/t4], [min], [max], [cases],
  [f4], [1], [reuse], [0.993], [0.921], [1.027], [7],
  [f4], [1], [t2], [0.998], [0.914], [1.133], [7],
  [f4], [1], [t3], [0.997], [0.911], [1.062], [7],
  [f5], [1], [reuse], [0.975], [0.816], [1.160], [7],
  [f5], [1], [t2], [0.906], [0.747], [1.017], [7],
  [f5], [1], [t3], [0.917], [0.847], [1.010], [7],
  [f5], [3], [reuse], [0.990], [0.922], [1.045], [7],
  [f5], [3], [t2], [1.065], [0.962], [1.260], [7],
  [f5], [3], [t3], [1.013], [0.887], [1.109], [7],
)

== The pre-registered rule applied
+ *Eager rewrite through shift runs* (keep if `noeager/on` ≥ 1 on ≥ 6 of 9 cells per engine per host): Linux 9/9 (f4) and 9/9 (f6p), 1.08–1.69×; macOS 9/9 (f4) and 8/9 (f6p), 0.97–1.87×. *Stays the default.*
+ *Gate-first F6-IC* (keep if `nogate/on` ≥ 1 on ≥ 7 of 9 cells for f6p per host): Linux 9/9, 1.69–2.10×; macOS 9/9, 1.62–2.15×. *Stays the default.* The f4 engine (no node oracle) reads 1.00 on every Linux cell and 0.93–1.06 on macOS, the A/A spread of each host.
+ *GF(2) tables / reuse* (an Arm default only if one arm is ≥ 5 % faster in the geometric mean at 1 thread and at all cores on both hosts, no case below 0.97×): Linux — no arm is faster than *t4* in any group (geomeans 0.999–1.076). macOS — *t2* and *t3* are 1.10× and 1.09× faster at 1 thread on F5, but 0.94× and 0.99× at 3 threads. No arm meets the rule; *the x86-tuned defaults stand.*

== Observations and limits
- These are CPU or wall times of shared, unpinned CI virtual machines; the macOS VM ran under heavy reported load during the inherited arms, so its brackets span 1 on many cells even where the median is far from it. The Linux runner's brackets lie within about ±2 % of the median.
- One run per host; no second attempt and no isolated host. The ratios describe these runners, not Arm hardware in general.
- Nothing here is an index-calculus total or a comparison against rho; it is a check that two solver defaults are right on another architecture, and that one constant needs no Arm-specific value.

Raw records: `ubuntu-24.04-arm/{inh,gf2}/raw/`, `macos-latest/{inh,gf2}/raw/`; tables `*/inh.tsv`, `*/gf2.tsv`; `summary.txt` (from `summarize.py`); figure from `plot.py`.
