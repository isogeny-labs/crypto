#set document(title: "Factor-base designs: V16, in-step snapshots at the scale of A0 m = 4, t = 4", author: "factor_base_designs")
#set page(paper: "a4", margin: (x: 1.8cm, y: 1.8cm), numbering: "1")
#set text(size: 9.5pt)
#set par(justify: true)
#show raw: set text(size: 8pt)

= Factor-base designs: V16, snapshots inside a step at the scale of A0 `m = 4`, `t = 4` (run-20261027T1710Z)

*Kind:* a tooling validation run registered in PREREGISTRATION.md, Amendment 16 ("Validation run V16, registered before its rows"). It adds no cell to Amendment 14 and changes no row or verdict. Numbers are copied from #raw("runs/V16-A0-m4-t4.jsonl"), #raw("runs/V16-A0-m4-t4.log"), #raw("runs/V16-instance.log") and #raw("runs/R256-A0-m4-t4.jsonl"); a few differences and quotients are marked as derived. Nothing here is a speedup, an S figure or a claim about the factor-base designs.

== What was run

The system of R256-A0 target 0 (A0, m = 4, t = 4, p#sub[1]; cell seed 18358879315, target seed 18358879571) on an AWS r7i.8xlarge Spot host (Xeon Platinum 8488C, 256 GiB, 800 GiB gp3 at 1000 MiB/s), with `--checkpoint-every 600 --checkpoint-in-step-every 900`, after the host's self-test (now including the in-step kill-and-resume chain) passed. Once the in-step snapshot file reached 20 GB the host waited 60 s and sent SIGKILL to the study process; the run script was called again. The binary was built on the host from the tree of commit #raw("ffcfcd8f1") with the Amendment 15 and 16 files (main did not compile at the time, as for R256); #raw("git_commit cd2f036fa"), sha256 #raw("9c398819234f264ee2a5a2e7605632022c227dafd98fff16d299f9332b8aa7fa"). Host 14:19Z--16:56Z; one SIGKILL (14:46:42Z), no AWS reclaim.

== Result

#table(columns: 9, align: (left, center, center, center, center, center, center, center, center), stroke: 0.4pt + gray,
  [*row*], [*status*], [*D*], [*steps*], [*max rows*], [*max cols*], [*basis*], [*field_ops*], [*F4 time (s)*],
  [R256-A0 target 0], [ok], [7], [60], [159 661], [167 726], [53], [32 354 122 772 019], [7 650],
  [V16-A0 (killed once, resumed)], [ok], [7], [60], [159 661], [167 726], [53], [32 354 122 772 019], [9 016],
)

*Pass criterion met:* every counted field of the resumed row equals R256-A0 target 0's, `field_ops` to the last digit (the registered criterion; the two rows are the same planted system). The V16 row has 2 segments; the second resumed at step 49 with 432.5 s of the budget already spent, found the in-step snapshot (phase 0, column 29 394, 23 611 pivots, 52 983 rows, 1 359.7 s spent) and continued from it. Peak RSS 128.7 GB (R256: 121.2 GB).

== Snapshot sizes and times, at scale

#figure(image("snapshots.svg", width: 90%),
  caption: [Size (bars) and write time (labels) of every checkpoint and snapshot logged in V16, in log order. "Step" is the step being run or just finished as the log names it. Source #raw("runs/V16-A0-m4-t4.log"), #raw("plot.py").])

- In-step snapshots (dense matrix with pivots): 28.9 GB in 34.5 s (step 50, forward phase, column 29 394 of the dense reduction), 20.5 GB in 27.5 s (step 50, back-substitution), 44.3 GB in 47.6 s (step 54, forward phase, column 29 190).
- Step-top checkpoints: 18.2 GB (27 s), 40.0 GB (49 s), 45.7 GB (54 s), 49.3 GB (59 s, 57 s).
- The first in-step snapshot of a step also writes that step's top checkpoint (18.2 GB, 27 s), which a resumed run wrote again at the same step although it had just been read; a 27 s waste, not a correctness issue.
- Read and re-derivation time on resume was *not logged*. The second segment's whole process time is 7 680 s.

== Predictions (Amendment 16, written before the rows)

- *Counted fields equal:* held.
- *Dense snapshot 40--90 GB, 40--100 s:* did not hold for two of three snapshots. They were 28.9, 20.5 and 44.3 GB in 34.5, 27.5 and 47.6 s; only the last is in either range. The snapshots seen were of steps 50 and 54, not the largest steps (in R256 the last steps (55 to 60) took about two minutes together, and no snapshot fell in them here).
- *Overhead over R256-A0's F4 time 10--30 %:* held. V16's F4 time is 9 016 s against R256-A0 target 0's 7 650 s: +1 366 s, +17.9 % (derived; +16.9 % against target 1's 7 714 s). The logged writes sum to 383 s. The remaining about 980 s is not decomposed: it includes the work lost after the last snapshot and before the kill, reading about 47 GB on resume, re-deriving the step's matrix, and the run-to-run spread (R256's two targets differ by 64 s). R256 also wrote checkpoints, so this is the cost of the in-step option and the kill and resume together, not against an uncheckpointed run (none exists at this scale).
- *Completes within 14 400 s:* held (9 016 s).

== What this verifies, and what it does not

- *Verified:* the study binary with the in-step option builds and runs on the host; in-step resume at the scale of A0 m = 4 reproduces a full-scale row exactly after a kill in the middle of a step's dense reduction.
- *Not verified:* the largest steps' snapshots (the sizes above are for steps 50 and 54); resume from the backward phase at this scale (one forward-phase resume only; both phases are covered by the unit test at small scale); a kill during symbolic preprocessing or matrix build (redone, not covered); the narrow arithmetic, weights, the tower engine; A11's steps (dense matrices several times larger: checkpoints of 55 and 101 GB were seen there, so snapshots are larger still).
- The cost of the option for a run that uses it is about 18 % of F4 time at these intervals, on this system, with one kill. A different interval or system will differ; a run that uses the option must register its interval and report its own sizes and times.

== Cost and data

About 2.6 instance-hours of Spot (about \$3, an estimate). Rows and logs in #raw("runs/V16-*"); S3: #raw("s3://crypto-autoresearcher/lab-results/factor-base-designs-a14/cd2f036fa-ckpt/state/out/") (the 20--49 GB checkpoint files were deleted after the run).
