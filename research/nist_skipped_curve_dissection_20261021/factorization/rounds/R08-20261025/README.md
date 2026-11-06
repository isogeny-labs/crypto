# R08: two-hour local continuation from the original checkpoint

**Completed:** native sieving stopped at **2026-10-25 07:52:38 UTC**,
with exit 0 after its configured time budget. It added **5,848,718 raw
relations**; the unfiltered collection checkpoint now has **10,441,117
lines** (4,592,399 inherited + 5,848,718 new). No proper factor was found.
Fresh whole-target product and PARI certificate replay pass; the same
444-bit composite remains. No collection process remains running.

Read the [explanation of the numbers](reports/completion-20261026T2122Z/REPORT.md),
[visual](reports/completion-20261026T2122Z/counts.svg), and
[PDF](reports/completion-20261026T2122Z/report.pdf), or inspect the
[machine-readable summary](summary.json), [native receipt](native-receipt.json),
[retained native log](evidence/native-sieve.log), and
[checkpoint checksums](final-checkpoint-files.sha256).
The [archive manifest](archive-manifest.json) records local preservation
and the still-missing required object-store publication.

The [registered follow-up](../../../../../backlog/XP-20261026-nist-p256-checkpoint-filter/README.md)
replays the old filtering result, then filters a fresh copy of this larger
checkpoint. It is submitted as a blocked backlog card and a draft expspec,
with no task, lease, paid host or new experiment launched. Its protocol
applies prospectively to filtering; it does not retrospectively preregister R08.

## Preserved interim observation

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
The preceding live observation remains unchanged and separately dated.
The final status, post-run replay and completion report above supersede
its pending-work description. Bulk publication remains pending. The
mathematical result remains eight of nine integers complete, with the
444-bit residual unresolved.
