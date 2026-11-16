# Producer software-check receipt

Claim PCC-20261115-907a94 has standing **unverified**. This document is not
an independent validator report or a mathematical proof. [pcc:unverified]

- Host: Linux x86_64; Rust stable 1.100.0, rustc 0ef1bc386 (2026-11-09).
- Source initially read/tested: d60f19656854fdfa1b37d4976652e614db7fcc91.
  Publication rebased the unpublished working branch onto
  5966c6a98c79ced2856e043a3d979a90c79c8c44; the affected native source was
  unchanged. Both ordered session-log additions were preserved. No official
  claim or ledger event was edited, reordered or re-chained.
- `cargo test --release --locked --manifest-path crates/hashed-ecdl-tensor/Cargo.toml --lib`:
  exit 0, 104 passed, 0 failed, 0 ignored; see checks/cargo-test-release-lib.log.
- `cargo fmt --manifest-path crates/hashed-ecdl-tensor/Cargo.toml --check`:
  exit 0.
- `cargo clippy --release --locked --manifest-path crates/hashed-ecdl-tensor/Cargo.toml --lib -- -D warnings`:
  exit 0; see checks/library-clippy.log.
- Strict all-target clippy: exit 101 on existing src/treewidth.rs:848
  manual_range_contains. Reproduced with the unchanged base admission and
  recursive modules; all remaining native source was unchanged. The isolated
  base copy initially lacked include_str fixtures; those were restored before
  the final base check in checks/base-clippy.log. No lint was suppressed.
- PCC canary: exit 0, 62/62 expected outcomes. Official ledger append: one
  statement after all 1,057 old events. `ledger verify --base origin/main`:
  exit 0, 1,058 clean events. `standing`: unverified. `verify` with the claim
  file path: exit 2, not_covered, statement has no proof kind and cannot be
  verified. No signed theorem or software-execution verdict is claimed.
- README PCC prose lint and source SHA256 checks: exit 0.
- Diagram rendered with Graphviz; report rendered with Typst. Final two PDF
  pages were rasterized and visually inspected. An initial layout spilled to
  three pages; resized the diagram and inspected the final two-page output.
- `tools/benchrec.py gate`: fails in the sparse checkout because no benchmark
  records are materialized and generated progress is absent. No benchmark
  files were changed or regenerated; no bypass was added. Full-checkout CI
  owns that repository-wide check.
  [pcc:unverified]

The utility's scalar premises do not certify actual graph dimensions. There
is no Lean build, identity certificate, bit-operation bound, byte upper bound,
scientific benchmark, timed comparison or cloud experiment in this change.

Infrastructure notes: research-kb backend and conductor are unavailable.
Used the explicit direct-source fallback. An archive-based base copy stalled
on lazy blob retrieval and was stopped; copied the needed materialized native
files and restored the three include_str fixtures instead. A checkpoint init
unexpectedly touched the tracked shared resume file; restored it exactly from
current main. No S3 checkpoint upload or experiment launch occurred. The task's
resume state is in this report, the session log, and its committed PR.
