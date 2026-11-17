#set page(paper:"a4",margin:15mm)
#set text(font:"New Computer Modern",size:9.5pt)
#let data = json("accounting.json")
= N83 finite-query reuse accounting
_2026-10-20 · accounting addition from retained native measurements_

The frozen preparation protocol requests reuse counts 1, 8 and 64.
The earlier report supplied 1 and 8. This native re-derivation completes
all three and separately charges fresh per-query receipt replay and the
worker container boundary. It introduces no new timed query or algorithm.

#align(center,image("reuse.svg",width:96%))

== Accounting correction and scope

The earlier sums used census + export + generic whole-base replay + the
mean worker interval for each projected query. They omitted the fresh
per-query receipt replay and the worker interval outside its internal
timer. Those earlier sums remain a separately labeled view in
`accounting.json`; the figure includes both observed container intervals.
It still is not a complete cold IC budget.

#pagebreak()
== Native reconstruction

The Rust tool verifies the pinned manifest and all #data.paired_files_checked
retained files in `paired-mode-v1`. It requires all 36 expected probes,
their exact finite attempt counts and identical D point/label sets,
all 36 bound fresh replay receipts, successful non-OOM container exits,
and nanosecond start/end timestamps on the pinned run date.

Every cell has nine probe observations: two at public target 0 and one at
each of indices 1 through 7. The arity-2 event scans all 2,760,082 distinct
first summands; the arity-3 event draws 2^20 pairs. The retained finite
events have zero relation rows and rank zero. These counts do not estimate
unrestricted natural yield.

#table(columns:(0.5fr,1fr,1fr,1fr,1fr),inset:4pt,
  [*m*],[*Mode*],[*Worker mean (s)*],[*Worker container (s)*],[*Fresh replay container (s)*],
  ..data.cells.map(c => ([#c.m],[#c.mode],
    [#calc.round(c.worker_mean_seconds,digits:6)],
    [#calc.round(c.worker_container_mean_seconds,digits:6)],
    [#calc.round(c.fresh_replay_container_mean_seconds,digits:6)])).flatten())

Prebuilt setup is #calc.round(data.setup_seconds,digits:9) s, the sum of
the separately pinned census, export and generic full-object replay.
Those one-time phases and the query worker have distinct recorded native
revisions. They ran on the same shared L0 SSH host. The point and coefficient
label sets agree between query preparation modes.

For each mode and m, the new projection is setup + q ×
(mean worker container interval + mean fresh replay container interval).
The older worker-only projection is setup + q × mean worker interval.
On-demand enumeration/closure/indexing are inside every on-demand worker,
so its separate setup term is zero. It is not free preparation.

== Evidence and unresolved charges

The native regression test rejects failed/capped container outcomes,
nonpositive intervals, wrong dates and malformed fractional timestamps.
The accounting run completed with exit zero, after checking the immutable
input digests. CI repeats the native computation and requires byte-identical
JSON. The graph and table read that JSON directly.

Worker peak RSS reached 651,149,312 bytes in the prebuilt cells and
563,486,720 bytes on demand. This is observed worker RSS, not a measured
peak for an unexecuted 64-query panel. All per-arm records are retained.

Excluded charges remain explicit: Docker launch/teardown outside the
container intervals; upload, custody and storage; earlier implementation
and infrastructure retry history; natural rank collection outside these
finite events; linear algebra and individual logarithm verification;
same-point rho and an external-fastest comparison. The card's original
16 min 48 s host interval includes its setup, controls, repeats and
launch/replay operations and is not replaced by a projection.

Candidate, selected total runtime, complete IC seconds and matched-rho
gap remain null. This is an accounting completion, with no measured
algorithm improvement or qualified timing promotion.

== Sources and reproduction

`examples/n83_prebuild_reuse_accounting.rs`; `PAIRED_MODE_PROTOCOL.md`;
`runs/paired-mode-v1.sha256`; the census/export/replay receipts cited by
`accounting.json`; `native-test.log`, `native-run.log` and `native-run.exit`
beside this report. The original result/graph remain under
`reports/result-20261019/`. This addendum and its SVG/150 dpi PNG/PDF have
matching browsing copies under `benchmarks/20261020-n83-prebuild-reuse/`.
