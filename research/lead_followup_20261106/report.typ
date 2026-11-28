#set page(paper: "a4", margin: 17mm)
#set text(size: 9pt, font: "DejaVu Sans")
= Conjugate-pair repair and radix-grading follow-up
November 6, 2026 · Development report

Native chart selection/resume support and chart/radix panel readouts are implemented. Scientific qualification panels remain blocked and have not launched. Development observations are producer assertions under PCC-20261106-ded5b6, standing unverified.

#image("workflow.svg", width: 100%)

== Conjugate-pair continuation
The instrument accepts independent chart offsets and exact target filters, stores the actual matrix and solver seed, preserves the historical seed-zero stream, and resumes interrupted pairs without overwriting earlier output. Tests cover same-file append, separate-file history, malformed/truncated input, duplicate identities and seed/degree separation. The prospective panel uses two original targets, three chart offsets and plain/transformed controls. Correctness and solver completion are separate outputs; an empty set is not a successful solve merely because it matches an empty oracle.

== Radix continuation
The native comparator checks equal curves, tower parameters, targets, construction seeds, budgets and grading weights. Missing draws are refused; capped runs retain unknown ratios. The prospective panel compares radix 2/4/8 at depths 10/11/12 with tower/null controls, two draws and four threads. Weighted degree values are not compared as costs. The existing depth-10 data provides a compatibility check, not a new trial.

== Execution dependency
The mandatory S3 checkpoint gate has no upload transport in this runtime. An authenticated command-capable EKS route, exact image/source/binary binding and live budget/capacity admission remain required. The ordinary GitHub experiment dispatch accepts ecbench specifications and is not a substitute for this example payload. No scientific computation is running in the background.

== Evidence and next step
The complete populations, controls, deadlines, accounting exclusions and stop conditions are in PROTOCOL.md. The blocked backlog cards are XP-20261106-conjugate-chart-followup and XP-20261106-radix-depth-followup. The worker script is a payload, not a submitter. Development commands and actual outcomes are in validation/checks.md and validation/native-tests.txt. A preparation PR does not complete either experiment.

== Sources checked and limits
KB unavailable: no tools or executable/catalog; the repository wrapper exits 127. Direct sources: the two existing conjugate-pair studies; pkm_radix_grading_20261104; gaudry_quartic.rs; Petit–Kosters–Messeng, PKC 2016, sections 2.4, 3, 4.2 and 5, https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf. The primary Gaudry PDF at eprint 2004/027 returned HTTP 403. No new theorem, novelty claim or measured gain is asserted. Historical scientific plots and dashboards remain unchanged because no new scientific value exists.
