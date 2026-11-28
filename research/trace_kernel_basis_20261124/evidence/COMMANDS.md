# Development qualification commands

Working directory: repository root. Toolchain: Rust 1.100.0 stable,
x86_64-unknown-linux-gnu, edition2021, no external crate dependencies.
The scratch installation lived outside the repository. These are ordinary
local development checks, not the protocol's research panel or an isolated
performance measurement.

```sh
cargo test --release --locked --manifest-path research/trace_kernel_basis_20261124/native/Cargo.toml --all-targets
cargo clippy --locked --manifest-path research/trace_kernel_basis_20261124/native/Cargo.toml --all-targets -- -D warnings
cargo fmt --manifest-path research/trace_kernel_basis_20261124/native/Cargo.toml -- --check
```

Each command has separate `.stdout` and `.stderr` files. Exit statuses are
in `development-status.json`; source and log identities are in `SHA256SUMS`.
The source test executable reports seven library tests and zero binary unit
tests. No CLI inspection workload ran. [pcc:unverified]

Initial qualification had one unused-parameter warning; the parameter was
renamed before the retained qualification. Rustfmt and Clippy were initially
absent from the minimal scratch toolchain; they were installed, then the
checks were executed. No failing control or timed-out scientific run was
discarded.

Repository integration: the separate `root-library` log records the actual
whole-root offline build attempt, not a substitute success. No production
root-library change is made by this component.

Admission preflight: `scripts/checkpoint.sh init` created a template; after
all required sections were completed, `scripts/checkpoint.sh push` returned
1 because no configured AWS upload profile or checkpoint token was present.
The branch's local resume copy was retained outside Git; the pre-existing
tracked `.resume/RESUME.md` was restored, not replaced in this PR. No S3
checkpoint, EKS submission or paid run is asserted.

Formalization preflight: neither `lean` nor `lake` was found, and
`lake --version` returned127. Current IDC1 v1 supports a fixed odd prime,
not the characteristic-two identities used here. The report records these
proof dependencies rather than asserting a kernel-checked theorem.
