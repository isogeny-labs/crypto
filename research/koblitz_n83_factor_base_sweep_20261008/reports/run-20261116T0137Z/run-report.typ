#set page(paper: "us-letter", margin: (x: 0.75in, y: 0.72in))
#set text(font: "Helvetica", size: 10pt)
#set par(leading: 0.7em)
= N83 current-head Linux release-library validation

*Run:* `run-20261116T0137Z`  ·  *Source:* `6a8d673562ae`  ·  *Status:* full validation gate failed

The source-bound Linux ARM64 `cargo test --release --lib --locked -j 1` ran from 2026-11-16 01:37:37 to 02:29:51 UTC, a derived whole interval of 3,134 seconds. The main suite reported *6,395 passed, 45 failed, 138 ignored* in 1,047.78 test seconds. Child and Docker exit codes were 101; Docker reported `OOMKilled=false`. The checkout remained clean, and outer container cleanup passed. The hard cgroup was 12 GiB memory, zero swap, four CPUs and 512 PIDs.

#image("failure-sets.svg", width: 100%)

All 45 Linux failed names were also in the earlier 63-failure macOS set; 18 names were macOS-only and none Linux-only. These are different source revisions and hosts, so the count difference is descriptive, not a validated improvement. Failure excerpts show tracked files absent from the sparse checkout and at least one missing `records.jsonl` with an S3 artifact pointer. Full materialization and hash-verified artifact restoration are separate requirements before a clean rerun.

The raw log is `lib-test.log.gz`: 119,992 compressed bytes, SHA-256 `3d9f11ba463f7c32732ae484dea2980e78b264a50d1776a7d3f53c7f4d98f850`; it expands to 640,511 bytes, SHA-256 `9879654194ce14336a9979e9b5414ee1a0a45aff6a05a8cb70b554eba39b2f51`. The child/outer receipts, Docker inspect, source commit and three set-comparison lists are beside this PDF. The artifact bot may archive the compressed blob to S3; its exact pointer must be verified.

This was validation, not IC performance. The release-library gate remains red, and the separate charged build stopped at its Rust stage cap. No relation yield, rank, descent, matched rho or total cold one-target IC cost was measured. No canonical IC graph changes.
