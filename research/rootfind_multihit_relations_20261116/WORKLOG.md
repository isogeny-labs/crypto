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
