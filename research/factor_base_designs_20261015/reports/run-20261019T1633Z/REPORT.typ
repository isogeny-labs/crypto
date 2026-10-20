#set document(title: "Factor-base designs: Amendment 5 run", author: "fb_designs_slope")
#set page(paper: "a4", margin: (x: 1.8cm, y: 1.8cm), numbering: "1")
#set text(size: 9.5pt)
#set par(justify: true)
#show raw: set text(size: 8pt)

= Factor-base designs: m = 4 longer-budget cells L1 and L2 (run-20261019T1633Z)

*Kind:* solver-axis stage diagnostic (AGENTS.md §2). Observations only. Nothing here prices a pipeline, reports S or shows a speedup. The protocol is #raw("PREREGISTRATION.md") Amendment 5, registered before any L1 or L2 row (commit #raw("1551b4dc0")). The analysis is #raw("runs/analysis-amendment5.txt") (native #raw("analyze --amendment5")). Every number below is copied from it or from the row files.

== Run

- *Commands* (#raw("B=target/release/examples/fb_designs_slope"), #raw("R=research/factor_base_designs_20261015/runs"), one system at a time, L1 before L2): \
  #raw("$B run --arms A0 --m 4 --t-min 4 --t-max 4 --budget 14400 --grace 600 --vmem-kb 14500000 --out-dir $R --file 'L1-{arm}-m4-t4'") \
  #raw("$B run --arms A11 --m 4 --t-min 5 --t-max 5 --budget 14400 --grace 600 --vmem-kb 14500000 --out-dir $R --file 'L2-{arm}-m4-t5'") \
  #raw("$B analyze $R --amendment5 > $R/analysis-amendment5.txt")
- *Binaries:* L1 ran on #raw("81577f519"). That is the Amendment 5 harness plus one recorded field, #raw("peak_rss_kb"): the child is reaped with #raw("wait4"), and its #raw("ru_maxrss") is recorded as descriptive only. L2 ran on #raw("4aec76a5a"). Between the two commits, only #raw("group_of") and the new #raw("analyze --amendment5") changed, not the run path. Both harness versions reproduce the committed #raw("analysis-amendment2.txt") and #raw("analysis-amendment4.txt") byte for byte. A smoke run at A0 m = 2, t = 2..3 reproduced the committed #raw("field_ops") (2131, 40982).
- *Host:* "vm", Intel Xeon @ 2.80 GHz, 4 cores, a Claude Code cloud container. #raw("MemTotal") is 16 479 424 kB, which is at least 15 GB, so #raw("--vmem-kb") stayed at 14 500 000 and no Amendment 6 was needed. The host was otherwise idle while each system ran. Note that C2 ran on a 2.10 GHz host, which affects wall time only.
- *Window (UTC, 2026-10-19):* L1 16:33:42–16:51:18. L2 16:52:33–16:55:58.
- *Limits as registered:* budget 14 400 s, grace 600 s (hard kill at 15 000 s), #raw("ulimit -v 14500000").
- *Records:* #raw("runs/L1-A0-m4-t4.{jsonl,log}"), #raw("runs/L2-A11-m4-t5.{jsonl,log}"), #raw("runs/L2-A11-m4-t5.dmesg.txt") (kernel log lines copied verbatim), #raw("runs/analysis-amendment5.txt").

== L1: A0, m = 4, t = 4 (b = 4, |V| = 16, 18 variables, 19 equations)

#table(columns: 7, align: (left, left, right, right, right, right, left), stroke: 0.4pt + gray,
  [*system*], [*status*], [*time (s)*], [*peak RSS (kB)*], [*D*], [*field_ops*], [*limit*],
  [target 0], [aborted], [516.7], [12 835 532], [—], [—], [malloc failed under #raw("ulimit -v 14500000") (SIGABRT)],
  [target 1], [aborted], [539.7], [12 891 360], [—], [—], [malloc failed under #raw("ulimit -v 14500000") (SIGABRT)],
)

- *Identity with C2:* both L1 systems match C2's A0 systems on cell seed (18358879315), target seeds (18358879571, 18358879827), #raw("n_vars") = 18, #raw("n_eqs") = 19, planted x, x#sub[R], curve and set (2 of 2).
- C2 ran the same systems under #raw("ulimit -v 13000000"), and they aborted at 536.9 s and 512.8 s. With 1.5 GB more address space, L1 aborted at about the same wall time (516.7 s, 539.7 s). No partial F4 values exist for a SIGABRT. The only partial value recorded for this cell is F4's earlier 300 s timeout: degree 7 reached, #raw("max_cols") 102 516 (open marker in the figure).

== L2: A11, m = 4, t = 5 (b = 5, |V| = 32, 42 variables, 43 equations)

#table(columns: 7, align: (left, left, right, right, right, right, left), stroke: 0.4pt + gray,
  [*system*], [*status*], [*time (s)*], [*peak RSS (kB)*], [*D*], [*field_ops*], [*limit*],
  [target 0], [aborted], [97.3], [13 974 324], [—], [—], [SIGKILL: kernel memory-cgroup OOM],
  [target 1], [aborted], [108.0], [13 974 336], [—], [—], [SIGKILL: kernel memory-cgroup OOM],
)

- The harness recorded #raw("child exited: signal: 9 (SIGKILL)"). The harness sends SIGKILL only at its 15 000 s hard limit, so it did not send this one. The kernel log (#raw("runs/L2-A11-m4-t5.dmesg.txt")) shows the memory-cgroup OOM killer on the container's cgroup, at #raw("memory: usage 14007508kB, limit 14007508kB"). Kill times (boot + 2639.98 s and 2748.17 s, i.e. 16:54:10Z and 16:55:57Z) and pids (10420, 10700) match the two systems.
- *Deviation:* the binding limit for L2 was this cgroup limit, 14 007 508 kB of charged memory. That is below the registered #raw("ulimit -v 14500000"). The registered #raw("MemTotal") check (16 479 424 kB) did not reveal it. L1 is not affected: its systems hit #raw("ulimit -v") with peak RSS 12.8 GB, below the cgroup limit.

#figure(image("m4_vs_b.svg", width: 100%),
  caption: [log#sub[2] #raw("field_ops") (left) and #raw("max_cols") (right, log#sub[2] scale) against b at m = 4, p#sub[1], for A0 and A11. Points are recorded systems (2 per cell), and lines join the cell means of completed cells. × marks a cell that hit a limit, with no value recorded (L1, L2). The open circle is the #raw("max_cols") reached by A0 at t = 4 before F4's 300 s timeout (#raw("runs/F4-A0-m4.jsonl")), a partial value, not the cell's maximum. Rows: #raw("runs/C1-rescore.jsonl") (F4 replays, trace matched), #raw("runs/C2-*"), #raw("runs/F4-*"), #raw("runs/L1-*"), #raw("runs/L2-*"). Run run-20261019T1633Z, #raw("plot.py").])

== σ and κ at m = 4 (completed cells only)

#table(columns: 6, align: (left, center, center, center, center, left), stroke: 0.4pt + gray,
  [*series*], [*completed cells*], [*κ*], [*m − 1*], [*σ*], [*verdict*],
  [A0 m=4 p#sub[1] (F4 via C1; L1 t = 4 not completed)], [2 (t = 2, 3)], [10.773], [3], [1.0000], [inconclusive (fewer than 3 cells)],
  [A11 m=4 p#sub[1] (F4 via C1, C2 t = 4; L2 t = 5 not completed)], [3 (t = 2, 3, 4)], [14.886], [3], [1.5000], [κ: cost-unfavourable; σ: no middle ground (σ ≥ 0.5)],
)

Per-system values in the completed cells (#raw("field_ops")): A0 t = 2: 63 703 685, 63 704 352. A0 t = 3: 111 465 267 887, 111 465 240 139. A11 t = 2: 941, 941. A11 t = 3: 33 417 634, 33 402 245. A11 t = 4 (C2): 863 032 546 892, 863 026 754 688.

== Decision rules (Amendment 5)

- *R1, does A11's constant factor survive at m = 4, t = 4?* L1 did not complete. Outcome: *A0 not measurable at t = 4 within 14.5 GB and 4 h* (limit: #raw("ulimit -v") at 517 s and 540 s). No cost comparison is made.
- *R2, A11 at m = 4, t = 5.* L2 did not complete (limit: kernel memory-cgroup OOM at 14 007 508 kB, about 100 s per system). A11's 3-cell verdicts stand: κ = 14.886 (cost-unfavourable, κ ≥ m − 1 = 3), σ = 1.5000.
- A0's m = 4 series gained no third cell, so its σ and κ verdicts remain undecided.

== Predictions scorecard

#table(columns: 3, align: (left, left, left), stroke: 0.4pt + gray,
  [*prediction*], [*observed*], [*outcome*],
  [L1: A0 aborts for memory again], [both systems aborted for memory (#raw("ulimit -v 14500000"))], [held],
  [L2: A11 does not complete within 14.5 GB], [did not complete; the kernel memory cgroup killed both systems at 14 007 508 kB, below the registered 14.5 GB], [not decided as registered],
)

== Limits and what is missing

- No #raw("field_ops"), D or #raw("max_cols") exist for A0 at m = 4, t = 4, or for A11 at m = 4, t = 5. The only partial values are wall time and peak RSS. A memory abort leaves no F4 counters.
- L2 never had the registered 14.5 GB. Whether A11 at t = 5 completes between 14.0 and 14.5 GB, or within 4 h on a larger host, is not measured here. The independent lab runner is still unavailable from this container (Amendment 4, C2).
- Two systems per cell, one host, no isolation receipt. Wall times are descriptive only.
