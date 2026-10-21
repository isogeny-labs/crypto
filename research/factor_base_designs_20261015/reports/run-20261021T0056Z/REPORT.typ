#set document(title: "Factor-base designs: Amendment 7 run", author: "fb_designs_slope")
#set page(paper: "a4", margin: (x: 1.8cm, y: 1.8cm), numbering: "1")
#set text(size: 9.5pt)
#set par(justify: true)
#show raw: set text(size: 8pt)

= Factor-base designs: the two m = 4 cells on a 64 GiB host, X1 and X2 (run-20261021T0056Z)

*Kind:* solver-axis stage diagnostic (AGENTS.md §2). Observations only. Nothing here prices a pipeline, reports S or shows a speedup. The protocol is #raw("PREREGISTRATION.md") Amendment 7, registered before any X1 or X2 row (commit #raw("fab81e175")). The analysis is #raw("runs/analysis-amendment7.txt") (native #raw("analyze --amendment7")). Every number below is copied from it, from the row files, or from the pre-flight and kernel-log files beside them. The earlier attempts at the same systems are in #raw("reports/run-20261019T1633Z") (Amendment 5) and #raw("reports/run-20261019T1740Z") (Amendment 6).

== Run

- *Hosts:* two AWS EC2 #raw("r7i.2xlarge") instances in us-west-2, Ubuntu 24.04, kernel #raw("7.0.0-1014-aws"), Intel Xeon Platinum 8488C, 8 vCPU. Each was otherwise idle and ran one cell:
  - X1 on #raw("i-0970999926c7aa48a")
  - X2 on #raw("i-02620d9dd532a4c0d")

  Each instance fetched its inputs and returned its outputs through presigned S3 URLs, so it held no credentials. Each terminated itself when its cell finished, with a 9 h hard cap that never triggered. The raw outputs are kept at #raw("s3://crypto-autoresearcher/lab-results/factor-base-designs-a7/fab81e175/").
- *Binary:* a release build of #raw("examples/fb_designs_slope.rs") at #raw("fab81e175"), built in the session container (glibc 2.39, as on the AMI). It is stamped through #raw("FB_DESIGNS_COMMIT"), and its sha256 #raw("19a5d11b…9bd72c") was checked on each host before the run. The run path is unchanged from L1, L2 and L2r. #raw("fab81e175") adds only the commit stamp.
- *Pre-flight* (#raw("runs/X{1,2}-*.preflight.txt")):
  - #raw("MemTotal") 64 774 300 kB (X1) and 64 774 296 kB (X2), #raw("nproc") 8.
  - Memory cgroup #raw("/system.slice/cloud-final.service"), with #raw("memory.max") = #raw("max"), so no cgroup limit.
  - V = #raw("MemTotal") − 524 288 = *64 250 012 kB* (X1) and *64 250 008 kB* (X2). That is 4.8 × L2r's 13 483 220 kB and 4.4 × L1's 14 500 000 kB.
- *Command* (#raw("scripts/run_amendment7.sh CELL BINARY OUT")), one system at a time: \
  #raw("fb run --arms A0 --m 4 --t-min 4 --t-max 4 --budget 14400 --grace 600 --vmem-kb 64250012 --file 'X1-{arm}-m4-t4'") \
  #raw("fb run --arms A11 --m 4 --t-min 5 --t-max 5 --budget 14400 --grace 600 --vmem-kb 64250008 --file 'X2-{arm}-m4-t5'")
- *Window:* both runs started at 00:56Z on 2026-10-21. X1 ended at 01:59:13Z and X2 at 04:16:44Z. The instances ran about 4.5 instance-hours in all, about \$2.35 at on-demand prices.
- *Analysis:* #raw("analyze --amendment7") adds the #raw("X1") and #raw("X2") row groups. With the X rows present, the base analysis and #raw("--amendment2"), #raw("--amendment4"), #raw("--amendment5") and #raw("--amendment6") reproduce their committed outputs byte for byte.

== X1: A0, m = 4, t = 4 (b = 4, |V| = 16, 18 variables, 19 equations)

#table(columns: 6, align: (left, left, right, right, left, left), stroke: 0.4pt + gray,
  [*system*], [*status*], [*time (s)*], [*peak RSS (kB)*], [*stop*], [*limit*],
  [target 0], [aborted], [1915.9], [60 667 848], [malloc failed under #raw("ulimit -v 64250012") (SIGABRT)], [attributable],
  [target 1], [aborted], [1873.7], [60 713 840], [malloc failed under #raw("ulimit -v 64250012") (SIGABRT)], [attributable],
)

- *Kernel log:* no OOM lines (#raw("runs/X1-A0-m4-t4.dmesg.txt")).
- *Identity:* 2 of 2 systems match C2's and L1's A0 systems on cell seed, target seed, #raw("n_vars"), #raw("n_eqs"), planted x, x#sub[R], curve and set.
- *Earlier attempts:* C2, under #raw("ulimit -v 13000000"), aborted at 536.9 s and 512.8 s. L1, under 14 500 000, aborted at 516.7 s and 539.7 s, with peak RSS 12 835 532 and 12 891 360 kB.

== X2: A11, m = 4, t = 5 (b = 5, |V| = 32, 42 variables, 43 equations)

#table(columns: 6, align: (left, left, right, right, left, left), stroke: 0.4pt + gray,
  [*system*], [*status*], [*time (s)*], [*peak RSS (kB)*], [*stop*], [*limit*],
  [target 0], [aborted], [6011.1], [62 919 616], [malloc failed under #raw("ulimit -v 64250008") (SIGABRT)], [attributable],
  [target 1], [aborted], [6024.0], [62 849 712], [malloc failed under #raw("ulimit -v 64250008") (SIGABRT)], [attributable],
)

- *Kernel log:* no OOM lines (#raw("runs/X2-A11-m4-t5.dmesg.txt")).
- *Identity:* 2 of 2 systems match L2's and L2r's.
- *Earlier attempts:* L2 was killed by the 14.0 GB memory cgroup at 97.3 s and 108.0 s. L2r aborted under #raw("ulimit -v 13483220") at 130.0 s and 131.8 s, with peak RSS 13.3 GB.
- No #raw("field_ops"), D or #raw("max_cols") exist for any X system, because a memory abort leaves no F4 counters.

#figure(image("m4_memory_hosts.svg", width: 100%),
  caption: [Each system's peak RSS against its wall time to the stop, for every attempt at the two open m = 4 cells. Dashed lines show each attempt's #raw("ulimit -v"), and the dotted line is the 4 h budget. Wall times are descriptive only, because the 15 GB containers and the EC2 hosts differ. Rows: #raw("runs/{L1,X1}-A0-m4-t4.jsonl") and #raw("runs/{L2,L2r,X2}-A11-m4-t5.jsonl"). Run run-20261021T0056Z, #raw("plot.py").])

== Decision rules

- *R1:* *A0 not measurable at t = 4 within 64 250 012 kB and 4 h.* The binding limit is memory, in both systems, and both limits are attributable. No cost comparison is drawn, so whether A11's constant factor survives at m = 4, t ≥ 4 is still undecided. A0's m = 4 series keeps 2 completed cells: κ = 10.773 and σ = 1.0000, both inconclusive.
- *R2:* *A11 not measurable at m = 4, t = 5 within 64 250 008 kB and 4 h.* The binding limit is memory, in both systems, and both limits are attributable. A11's 3-cell m = 4 verdicts stand:

#table(columns: 5, align: (left, center, center, center, left), stroke: 0.4pt + gray,
  [*series*], [*completed cells*], [*κ*], [*σ*], [*verdict*],
  [A11 m=4 p#sub[1] (F4 via C1, C2 t = 4; X2 t = 5 not completed)], [3 (t = 2, 3, 4)], [14.886], [1.5000], [κ ≥ m − 1 = 3: cost-unfavourable; σ ≥ 0.5: no middle ground],
)

*No recorded verdict changed.* Neither cell finished within 4 h. In all four systems, memory, not time, was what stopped them: each ran out of its 64 GB well inside the 4 h budget.

== Prediction scorecard (Amendment 7)

#table(columns: 3, align: (left, left, left), stroke: 0.4pt + gray,
  [*prediction*], [*observed*], [*outcome*],
  [X1: A0 does not complete; binding limit *time* (fits in 64 GiB)], [did not complete; memory abort at 1874–1916 s, peak RSS 60.7 GB], [*did not hold*],
  [X2: A11 does not complete; binding limit *memory*], [did not complete; memory abort at 6011–6024 s, peak RSS 62.9 GB], [*held*],
)

The X1 prediction assumed that A0's working set at t = 4 would level off below 64 GiB. The basis was that L1 had reached 12.9 GB by 540 s, but that observation could not bound the working set. A0 at t = 4 needs more than 64 GB. The X2 prediction held, but its stated reason was wrong. It assumed resident memory would keep growing at L2's early rate, about 14 GB in 100 s, which would pass 64 GiB within minutes. Instead the growth slowed: X2 took about 100 minutes to exhaust 64 GB.

== Limits and what is missing

- *A memory abort leaves no counters.* The partial values are wall time and peak RSS, and both are descriptive. Wall time depends on the host. Peak RSS is bounded by V and so does not measure the working set. A per-degree progress record, which would leave partial #raw("max_cols") and #raw("field_ops"), would change the harness's output and needs its own amendment.
- *The working sets are unmeasured.* Neither cell's working set is known, only that each exceeds 64 GB. Settling R1 or R2 needs a host whose memory exceeds those working sets, which are unknown. Even then, extrapolation says the 4 h budget may not suffice: about 2·10#super[14] field ops for A0 at t = 4 and about 2·10#super[16] for A11 at t = 5, from Amendment 7's predictions.
- *Single run per cell.* Each cell has one run of two systems on one host type, so there is no replication across host types.
- The independent lab runner (isolab) is still not reachable from these sessions, and no GitHub larger runner is configured for the repository.
