# Multihit relation screen worklog

## 2026-11-16T12:12Z — Protocol, source, and first launch refusal

The fixed protocol was committed at `15a7782ef3`, then the builder-control
correction at `ccb82776e7` before any run. Source was committed at
`30ad507c71`. The question is whether later valid triples for one residual
produce useful independent rows, with an explicit scan-cost counter.

The first command was `cargo test --release --test rootfind_multihit_relations
-- --nocapture` from the repository root, with Rust 1.99.1, four Cargo jobs
and the existing shared target directory. Raw stdout, stderr, UTC timestamps,
exit 101 and `/usr/bin/time -l` counters are in `run.*`. It compiled no test:
Cargo also built the Linux-only `icprog` binary, whose `libc::cpu_set_t` and
affinity symbols are unavailable on this macOS host. This is a build-route
failure, not an observation about the relation mechanism. [pcc:unverified]

The corrective route is a library-only Cargo harness with a copied locked
dependency set and a package-name correction. Its source and lockfile will
be committed before a second launch. The first output remains untouched.

## Open threads

- Run the locked library-only harness once, inspect every outcome, derive
  a machine-readable bounded record, and produce diagram/chart/PDF.
- Preserve any correction as a new commit and output; do not rewrite the
  first launch.

## 2026-11-16T12:19Z — Locked library harness result

The corrected harness and lockfile were committed at `4f66bdbda5` and
`aa129e0ab0` before launching. The exact command was `cargo test
--manifest-path research/rootfind_multihit_relations_20261116/harness/Cargo.toml
--release --locked --test rootfind_multihit_relations -- --nocapture` with
Rust 1.99.1 and four Cargo jobs. `run-locked.*` retains full stdout,
stderr, UTC times 12:14:45–12:19:21Z, exit zero and resource counters.
The release build took 4m35s; one test ran for 0.11s and passed.

K13/K19/K23 first-hit reached full rank after 26/115/740 distinct target
queries and 418/8,741/152,582 pair lookups. All-hit reached it after
16/92/599 queries and 528/12,788/164,725 lookups. The later hits added
3/18/19 independent rows to the all-hit reduction. Both arms recovered
the planted scalar 641 on each cell. These are scan-model counts, not
complete-call timings. `derive_results.py` binds the committed source,
test binary, raw hashes, fixed counts and arithmetic to `RESULTS.json`.
[pcc:unverified]

The diagram, tradeoff chart, report and PDF carry this bounded result.
The next screen should stop after a newly independent row or a frozen
probe cap; it cannot be treated as a benefit until complete-call work
and F4/F5/F6 controls are measured on the same targets.

After the run, prose lint found two missing `[pcc:unverified]` tags in
the original protocol. `AMENDMENT-2.md` records a nonsemantic editorial
annotation and preserves the frozen text by commit and SHA-256. No
execution parameter or interpretation rule changed.
