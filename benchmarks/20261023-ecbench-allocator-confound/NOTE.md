# ecbench allocator confound (PR #2455): benchmarks copy

Copies of the study's report PDFs and graphs; the source of truth is
`research/ecbench_allocator_confound_20261023/` (regenerate there, never edit a copy).

**No row in `benchmarks/README.md`, and no `VALIDATION.json` here.** The validation gate
(docs/VALIDATION_GATE.md) needs an independent replay certificate and an independent
validator report. This study was run, replayed and summarised on one host by one agent, so
`independent: true` cannot honestly be asserted. A candidate `VALIDATION.json`, replay
certificate and validator report (`independent: false`; the matched-rho reference and the
section 5a band are complete) are in `research/ecbench_allocator_confound_20261023/validation/`;
`python3 scripts/validation_gate.py evidence` on that directory refuses it on those two
fields only. A replay from a host of another ecbench class, or by a validator who did not run
the study, would complete it; then add the row and copy `VALIDATION.json` and the two pinned
files here.

Headline (measured, descriptive wall): counts identical across the merge-base and PR
binaries (10 session pairs, 0 differences); smoke-prime solve wall 18.4 to 38.8 times shorter
on the PR binary for the eight rho/BSGS/kangaroo arms (n = 12 per arm per binary); IC online
window on claim-koblitz-r10 not distinguished (ratio 1.067, n = 40 per binary).
