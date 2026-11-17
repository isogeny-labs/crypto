# Local validation record

These are producer-run development checks, not signed independent verification
[pcc:unverified]. No experiment or benchmark was launched.

Source baseline: `e472218c8b14a770c38a1198bc2073edbc03d480`.
Toolchain: Rust 1.99.1, rustfmt from the same stable toolchain.

## Commands

```
rustfmt --edition 2021 --check src/cryptanalysis/koblitz_index_calculus.rs
git diff --check
CARGO_BUILD_JOBS=2 cargo test --release --lib decomposition_trace --no-default-features
```

Formatting and whitespace checks pass locally [pcc:unverified]. The first
optimized focused command completed with four passing tests and one failed
baseline comparison. The exact target diagnostic and final corrected results
are recorded below. The whole-library run did not complete.

Earlier failed attempts stopped before compilation because sparse checkout
omitted crates/ecdl-tensor-ic and research/pencil_large_prime_20261029.
Those paths were materialized with the repository sparse-checkout helper.
The first formatting command also needed the rustfmt toolchain component;
it was installed before rerunning the check.

## Publication and experiment gates

- `scripts/checkpoint.sh gate`: rejected an experiment launch because no
  pushed task-specific checkpoint exists. A tracked resume note belongs to
  another workstream and was preserved. No configured upload token or local
  AWS credentials are available.
- `gh api .../actions/variables`: HTTP 403. Workflow-read and source-push
  rights are distinct from experiment/checkpoint access.
- `python3 tools/benchrec.py gate`: fails on historical raw/report paths
  excluded from this sparse checkout and a pre-existing open record,
  benchmarks/records/20261108T2223Z-pkm-fd-pfd-c.json. No record in
  benchmarks/records is changed by this patch. Its diagnosis is retained;
  no gate bypass is used.
- Scoped PCC prose lint: passes on the study README, pending protocol and
  access problem record [pcc:unverified].
- `python3 scripts/kb_usage_report.py --since 2026-11-05`: completed;
  KB itself is unavailable (no MCP tools and wrapper exits 127).
- Typst report rendered and both pages inspected. No measured timing chart
  exists; flow.svg is a source-analysis diagram.

## Reproduction

Use a checkout with the repository's native path dependencies and compile-time
fixtures materialized. The checked source contains the native tests named
`decomposition_trace_*`, plus the pre-existing
`trace_bit_is_a_group_homomorphism_including_torsion` test. Test outputs will
be retained beside this record once available. No smaller-curve arithmetic
check is represented as an admitted degree-83 or ECC2K-130 DLP measurement.

## Final scoped outcome (2026-11-09 02:52 UTC)

The final command was:

```
CARGO_BUILD_JOBS=3 cargo test --release --lib decomposition_trace \
  --no-default-features --config 'profile.release.package.crypto.opt-level=0'
```

Result: five passed, zero failed [pcc:unverified]. See checks/focused-final.txt.
This is a correctness diagnostic with an explicit root-package optimization
override. The unmodified optimization build's earlier failure is retained in
checks/focused-initial-optimized.txt and the exact target/witness in
checks/degenerate-target-diagnostic.txt. The final source has not been rerun
with ordinary root-package optimization.

Six existing native regressions passed using the final test binary; filters
and output are in checks/regressions.txt. These cover the original group
homomorphism, native SAT domain filtering, Crossbred against enumeration,
F6 closure and exact misses, and unsupported result admission [pcc:unverified].

The full-library run on the earlier binary was interrupted after about 13
minutes, already carrying failures. Its status is incomplete/failed, not passed.
checks/broad-suite-interrupted.txt lists observed failures and SIGINT; compact
isolated failures identify missing GPU source and a missing historical session
record. Other failures are not classified. The full local raw log is
/workspace/scratch/trace-full-tests.log, pinned in checks/broad-local-log.sha256;
it has not been uploaded to durable external storage. There is no claimed
whole-suite or independent verification receipt.
