# Guarded section coefficient elimination

A separate Rust compiler implements the proposed coefficient-block operation
for a restricted regular chart. It keeps every original point constraint,
adds21 explicit difference inverses and replaces five normalized coefficients
with one guarded minor inverse. Each selected row's sparse Cramer identity
must cancel exactly at compilation; candidates still receive full original
section/index/group and raw-R replay. This is a classical operation, not a
new index-calculus algorithm. [pcc:PCC-20261123-d83c36]

Corrected v2 controls:12 supplied fixtures,10 admitted,2 identity-target
exclusions,60 successful pivot lift/projections. No discovered root or backend
call. At toy chart0, variables353to349, degree4to12 and terms1274to6963.
At original-q/k20633,1085to1081 variables and3458to9147 terms. These are
representation counts, not solver costs. The direct original point system
still has fewer variables. [pcc:PCC-20261123-d83c36]

The failed first call, source/binary and partial streams are frozen. V2 changes
only classification of an already excluded target. Both full raw archives
remain locally retained and pending trusted upload; publication is incomplete.
Metadata/source statement d83c36 is unsigned/not_covered. See report.md,
report.pdf, NEXT.md, PUBLISHING.md and the original PROTOCOL/PROTOCOL_V2.

Reproduction: restore the103-input V2 inventory using the three unchanged
point-context constituents and source-v2-delta. Use the exact crate0.2.0
source and pinned fixture. Final release five tests/fmt/strict all-target
Clippy passed before qualification. Native-v2 contains build/precall/postcall
and source hashes; no scientific source test was rerun for this report.
Do not call all v1 hashes current after the three legitimate v2 input changes.
No new Lean endpoint, full chart/sector proof, cost or exponent claim.

```sh
cargo test --release --manifest-path tools/torus-section-elimination/Cargo.toml
cargo fmt --manifest-path tools/torus-section-elimination/Cargo.toml --check
cargo clippy --release --all-targets --manifest-path tools/torus-section-elimination/Cargo.toml -- -D warnings
```

Use a recorded task target and CARGO_INCREMENTAL=0. A new run/version needs
its own frozen qualification. The PR maintenance checker performs no research.
