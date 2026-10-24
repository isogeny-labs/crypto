#set page(paper: "a4", margin: 16mm)
#set text(font: "New Computer Modern", size: 10pt)
= N83 D18: Linux validation and gated continuation
_2026-10-24 UTC · native worker revision 4e1d311d8 · shared x86-64 host, L0_

The Linux full-library gate began at 06:50:23 UTC using Rust 1.99.1 and
exited 101: *4,473 passed, 15 failed, 119 ignored*. No affected-example,
release-worker, bounded-construction or solver-card stage followed it.

Nine failures report runtime fixtures absent from the sparse checkout. The
exact committed evidence paths, SOURCE42 rows and chain-set fixtures are now
materialized; source status remains clean. Six solver assertions also failed:
two explicitly report timeouts, three zero qualifying trials and one a missing
planted solution. The earlier description of all six as confirmed deadline
failures was too broad. A full serial rerun will distinguish deadline/contention
effects from other causes. No test or existing limit has been weakened.

An unrelated benchmark container was active when the failure was inspected.
The shared lock therefore does not establish an uncontended timing interval.

#align(center, image("gates.svg", width: 94%))

The replacement pipeline was queued at 07:24:42 UTC, PID 1841122, with a
four-hour admission limit. Native notification integration is checked first;
then the full library runs with one test thread, followed by the affected
examples, native builds and the original six-model bounded construction.
The actual host card is independently checked locally and committed/pushed
before admission. The benchmark lock is released during publication and
reacquired before the unchanged frozen schedule starts. Invalid controls stop
natural work; completed or failed receipts are collected automatically.

The local Apple arm64 validation at 07:17:32–07:19:57 UTC again passed all
4,478 library tests and 29 affected-example tests (119 library tests ignored).
Workflow/shell checks and one notification-state test pass. Linux file-event
and timeout integration remain queued. Platform-specific test populations
differ; these counts are not a speed comparison.

The 39-file validation-v5 archive preserves the failed Linux run, passing
local run, sources and continuation commands. Every file hash was checked
after extraction, and an identical archive was repacked. SHA-256: \
#text(size: 8pt)[0307e0c981f459e4d6b42ffbefa51806f337b670519b8cbbc1f2b580ecb67f56].

The host card, 22 controls, 22 independent replays and 18 natural queries
remain pending in this record. Final independent audit, result reports and PR
merge remain gates. Useful rank, complete cold/reused cost, matched rho and
best-base selection stay open. No new solver timing is reported; canonical
runtime charts and benchmark rows are unchanged.

Sources: RUN_REPORT.md, validation-v5-files.sha256, PROTOCOL.md and admission/.
