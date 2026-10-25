#set page(margin: 1.8cm)
#set text(size: 10pt)
#set par(justify: true)
// one body font for code spans too: fewer embedded fonts, a smaller PDF
#show raw: set text(font: "Libertinus Serif", size: 9pt)

= Native checkpoints for the dense closure: stop, kill, resume

Date: 2026-10-24. Code: #raw("src/cryptanalysis/closure_checkpoint.rs"), #raw("inherited_f4.rs") (#raw("close_with"), #raw("ClosureSink")), #raw("gf2_elim.rs") (#raw("rref_resumable")), #raw("examples/dreg_closure.rs") (#raw("--checkpoint-dir")). Driver: #raw("../../ckpt-validate.sh"). Graph source: #raw("plot.py"). Raw outputs: #raw("data/").

This report records observations. It reaches no verdict about any hypothesis, and it makes no timing claim: runs that were stopped and resumed are not cold measurements.

== What ran

Every run is #raw("dreg_closure --to 4 --max-cols 1100000 --draw K FILE"), the dense closure scan at degrees 3 and 4 on a committed draw of #raw("research/dreg_closure_scale_20261016/draws/"), on one 4-core, 15 GB container.

*Cell m=4 (12,3), draw 1* (#raw("data/m4-12-3-d1/"), #raw("ckpt-validate.sh"), nothing else running):

#table(columns: (2.6fr,1fr,1fr,1fr,1fr,1fr,1fr), inset: 4pt, stroke: 0.4pt,
 [run], [segments], [saves], [GB written], [s in saves], [wall s (first / last segment)], [9 fields equal plain],
 [plain (no checkpoint)], [1], [0], [0], [0], [44], [(reference)],
 [checkpoint every 5 s, never stopped], [1], [14], [15.9], [58], [103 / 103], [yes],
 [SIGTERM at 25 s, resumed], [2], [16], [19.4], [67], [35 / 81], [yes],
 [SIGKILL at 25 s, resumed], [2], [15], [17.6], [61], [25 / 96], [yes],
)

Compared per degree: #raw("degree, xl_refuted, closure_refuted, saturated, xl_rank, closure_rank, columns, closure_products, closure_word_ops"). #raw("secs") is excluded: it is the compute time summed over segments. The checkpoint interval was 5 s, chosen to force many saves on a 44 s run; the largest state is 2.5 GB (303,065 rows × 1,038 words).

*Cell m=4 (13,4), draw 1* (#raw("data/m4-13-4-d1/")): segment 1 was stopped by #raw("timeout 280"), which sends SIGTERM, while round 5's elimination was at word 180 of 1,943 with 11,520 of 287,231 rows pivoted. It wrote generation 6 (4.47 GB, 27.2 s) and exited 143. Segment 2 restored generation 6 and finished. Result: closure degree 4, 124,314 columns, 700,747 products, 717,930,505,315 word operations, in both the resumed and the plain run (all nine fields equal). These runs shared the 4 cores with a compile, so their wall times (plain 415 s; segments 308 s + 132 s) are indicative only (#raw("data/m4-13-4-d1/walls.txt")).

#figure(image("ckpt-validation.svg", width: 100%), caption: [Stop, kill, resume: wall seconds, and the cost of one save. Data in `data/`, source `plot.py`.])

== Observations

- Measured: the three checkpointed runs on (12,3) and the stopped-and-resumed run on (13,4) have the same value as the plain run in every compared field, including the word-operation counter.
- Measured: with a 5 s interval the uninterrupted checkpointed run took 103 s against 44 s plain; 58 s of that is inside 14 saves of up to 2.5 GB each. At the workflow's 3600 s interval the same save cost is paid once an hour. Derived: one save writes, hashes and fsyncs about 0.3–0.4 GB/s here (2.5 GB in 6–9 s; panel c), so a 200 GB state would take about 8–11 minutes on this disk. The cluster's disk is not measured.
- Measured: a SIGKILL 25 s into the run left the generations committed so far; the next process restored the newest valid one and finished. Loss was the work since that save.
- Measured: a first attempt at this validation failed with #raw("No space left on device"): committing a generation before pruning needed three states on disk at the peak. Pruning to one committed generation before writing the next makes the peak two states; the failed save was not recoverable then and a failed periodic save is now logged and retried instead of ending the run. The numbers above are from the corrected build.
- Derived, from the unit tests (see #raw("../../CHECKPOINTS.md")): stopping at every word boundary of the elimination kernel and resuming reproduces the matrix, rank and charged word operations exactly; a scan stopped after every round, and inside eliminations, equals the plain scan on 9 chained systems; a broken restore of the pending falls fails the round-resume and fallback tests, and a broken restore position of the elimination makes the second never finish (the mutated build did not terminate).

== Limits

- One draw per cell; two small cells. Not measured: any state above 4.5 GB, S3 transfer, a node loss.
- Not covered by a checkpoint: the plain (XL) elimination and the Macaulay matrix build and first elimination of the closure (before round 0). A loss there repeats that step.
- Wall times are those of a shared 4-core container and say nothing about the cluster.
