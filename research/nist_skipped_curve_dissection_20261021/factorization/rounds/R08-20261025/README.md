# R08: two-hour local continuation from the original checkpoint

The native process and two lattice-sieving workers were running at
**2026-10-25 06:07:37 UTC**. The [dated observation](OBSERVATION.json)
records 614,231 new raw relations for this allocation, excluding the
inherited 4,592,399 lines. It does not assert future process state.

R07's filter found insufficient singleton survivors for a matrix. This
new allocation uses R06's unmodified original state, not the copy with
appended free relations. Fresh baseline certification, whole-target replay
and all predecessor checkpoint checksums pass. Fourteen runtime source
blobs match immutable commit 5cb553810e5bbfbd8691ec614ffcaacefc625757.
The native binary matches its passing timeout-control summary.

The [frozen protocol](PROTOCOL.md) allows one local native 7200-second
allocation, outer 7260 seconds plus 30-second kill grace, two workers,
4 GiB per native process and 1 GiB per output file. The configured native
relation request is not a completion-time estimate or matrix guarantee.
No paid/remote host, lease or task is created. The distributed backlog
card remains unqueued.

See the [interim report](reports/run-20261025T0551Z/REPORT.md) and
[inspected PDF](reports/run-20261025T0551Z/report.pdf).
Final native status, complete raw archive, post-run replay and completion
report remain **pending until this worker terminates**. The mathematical
result remains eight of nine integers complete, with the 444-bit residual
still unresolved. Progress metrics currently record completed checkpoints
and filtering results; this live observation remains separately dated.
