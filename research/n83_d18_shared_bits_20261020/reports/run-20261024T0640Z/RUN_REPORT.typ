#set page(paper: "a4", margin: 16mm)
#set text(font: "New Computer Modern", size: 10pt)
= N83 D18: independent construction-policy audit
_Run 2026-10-24 06:40:42–06:42:50 UTC · revision 5e2dc2583_

The native freezer and archive auditor now independently enforce the actual
construction container's successful exit, 4 GiB memory, zero swap, one CPU,
no network, read-only root, process/capability limits, pinned image and exact
capped command. The preceding version checked the receipt hash and relied on
the shell preparer to enforce the resource policy. The frozen policy is unchanged.

#table(columns: (2fr, 3fr), inset: 6pt,
  [*Gate*], [*Observed result*],
  [Full release library], [4,478 passed; zero failed; 119 ignored],
  [Eight affected examples], [29 passed; zero failed],
  [Updated native auditor], [Release build exit 0])

One new test independently changes memory, swap, CPU, write access, network,
running state, OOM state and exit status. Each altered receipt is rejected;
an empty container list is also rejected.

#v(4mm)
#image("gate.svg", width: 100%)
#v(4mm)

Host: shared Apple M4 Pro / arm64; Rust 1.98.0 selected explicitly. The native
busy lock serializes work; it does not qualify timing. The worker, general-field
replayer and construction executable are unchanged. The six-model reconstruction
remains in the earlier versioned run. This run performs no SAT query.

The 17-file validation-v4 archive was extracted, every file hash checked and
repacked identically. SHA-256:
#text(size: 8pt)[aca521b35b0523e0e3732f1dcd2b2cbcec29c58e895717740bb40070f63ba7d6].

The Linux construction/card and current-head PR CI remain open gates. Natural
useful rank, complete cold/reused IC cost and matched rho remain open. The
queued worker source is `4e1d311d8`; the updated auditor independently checks
that source's construction and worker receipts. Canonical runtime charts and
benchmark entries stay unchanged because this run adds no timing comparison.

Sources: `RUN_REPORT.md`, `validation-v4-files.sha256`,
`n83_d18_shared_card_audit.rs`, `PROTOCOL.md` and the earlier validation-v3 report.
