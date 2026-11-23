#set page(paper: "a4", margin: 18mm)
#set text(font: "DejaVu Sans", size: 10pt)
#set par(leading: 0.6em)
= SAT CLI acceptance: retained initial evidence
Descriptive run report · 5 November 2026

The recorded acceptance completed all eleven CLI commands with exit code zero. Both fixed ordinary-binary cells retained a checked initial pattern through verification, report/export, local custody creation, restoration and restored verification. The analyzer records PASS; the canonical export validation records zero schema errors and zero semantic findings. This report describes those receipts and does not assign a research verdict.

#image("patterns.svg", width: 100%)
#text(size: 8pt)[Recorded counts from analysis.json and the two cell manifests. Sample: two cells, one per twist; no repeats or uncertainty estimates. Run: sat-cli-custody-acceptance-20261103-v1. The gray bar is each cell's recorded total, not another trial.]

== Recorded outcomes
#table(columns: (1fr,1fr,1fr), inset: 5pt,
[Field], [Cell 0 / plain], [Cell 1 / twist],
[Initial patterns checked], [1 of 2], [1 of 2],
[Native outcome], [partial], [partial],
[Solver completed], [false], [false],
[Scalar recovered without witness], [false], [false],
[Worker exit code], [0], [0],
[Diagnostic elapsed milliseconds], [203.069508], [102.878048])

All values above are recorded. The elapsed values are supervisor diagnostics with clock_valid_for_comparison=false; no speed comparison is made. Whole-worker retired instructions are null. CPU time and a valid whole-worker instruction count are not supplied by these outcome records.

Two cells were requested and attempted; both worker processes completed, with zero recorded failed, timed-out, interrupted or resumed cells. Their algebraic outcomes remain partial. One Generate invocation is recorded. SAT causal training admission remains false. No independent negative CLI generation arm was part of this acceptance; the protocol points to prior exact-source test gates for corruption and refusal controls.

#pagebreak()
= Instrument, inputs and receipt custody
The executed code commit was #raw("93a5ab621ce5ec5aadc830dcd41f3ba186eaa64b"). Source before/after checks and the export report dirty=false. The retained binary SHA-256 was:
#raw("e5a594ce5cfa3dd1701c429d174c885cbfa845584c1458f8a2f90a1105217d26", block: true)

Protocol #raw("sat-cli-initial-custody-acceptance-20261103-v1"), version 2, was frozen before data. Canonical protocol hash:
#raw("b1d47282b1f736ff0a201ed00dfff65c8d36879c663554dd4a622b3075988d36", block: true)
The full protocol file hash is distinct:
#raw("808d166936024767fc62057728a73128cdfa8957feae6833f3c8488b9fdf82ac", block: true)

Fixed inputs: ordinary binary family, n=23, m=3, both twists, legacy factor base size 4, seed 42, one curve seed, generator and key per twist, degree bound 3, work limit 16384. Producer caps were 60 seconds per cell, 256 MiB memory, 128 MiB trace; the driver cap was 1800 seconds. Replay uses its separate registered allowance. Exact command arrays and all receipt fields are retained in data.json; no command was rerun to produce this report.

The eleven commands were plan, preflight, generate, verify, status, report, export, custody create, custody verify, custody restore, and restored verify. Local restore assertions checked original run bytes. The host record reports x86_64, Intel Xeon 6975P-C, 32 available logical CPUs, Linux 6.18.54-121.166.amzn2023.x86_64 and L0 isolation. These are descriptors, not an exclusive allocation claim.

Recorded start: 2026-11-05T15:30:21.797741Z. Recorded finish: 2026-11-05T15:30:38.651101Z. Derived whole-attempt elapsed interval: 16.85336 seconds, calculated as finish minus start. This interval includes driver and verification activity and is not a solver timing measurement. No new cloud launch or remote request is recorded by this local acceptance.

== Evidence and next step
Source root: #raw("/var/tmp/sat-cli-custody-acceptance-20261103-v1"). data.json lists the exact source hashes and embeds the command receipts. The source experiment.export.json SHA-256 is:
#raw("8d15714e7c344e4dc038415f96c1deabde3300586a75f593ecae07188faf2d38", block: true)

Full SAT search, UNSAT certification and SAT causal training projection are not supplied by this initial-prefix acceptance. Next, integrate and validate the original-owner row and propagation extensions against their separately frozen source gates. Publish these derived report artifacts after the immutable run records.
