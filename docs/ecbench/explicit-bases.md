# Explicit binary factor bases

`ecbench` already accepts an explicit linear basis through the built-in
`binary-subspace` factor-base builder. No additional plug-in or separate binary
is required. This interface is present in `crypto` `main` at
`24def3ae4dfe94bbbee5f30245847b372d02fa70`; it was introduced in
`90e5d2fdd7dbf1f329101e65c6d54c3bdcfbb708`.

## Input contract

```text
binary-subspace:dimension=5,basis=0x1;0x2;0x4;0x8;0x20
```

`basis` is a semicolon-separated list of decimal or `0x` hexadecimal field
words in the curve's polynomial basis. Quote the whole argument in a shell.
Commas separate parameters, not basis vectors. Exactly `dimension` nonzero
words are required; each must fit the field degree and the vectors must be
linearly independent over F₂. The dimension must lie in `1..=30` and is
validated before narrowing to a machine index. The existing ecbench narrow
binary-curve limit applies; this does not enable n = 83 or other wide curves.

The builder enumerates nonzero elements of the supplied span and includes all
rational curve points above them, with one column per abscissa and both signs.
The zero abscissa is excluded. It preserves the supplied ordered basis in
`FactorBase::subspace_basis` for algebraic descent. It does not project points
into the prime-order subgroup or close the base under Frobenius.

Omitting `basis` retains the existing polynomial-prefix behavior. An explicit
basis cannot be combined with `select=strict4e` or `select=gated-rank`.
This interface describes binary linear bases; arbitrary prime-field
x-coordinate lists and external point manifests are separate interfaces.

## Dump and run use the same recipe

After a binary has been built in a suitable environment:

```sh
ecbench fb \
  --curve '{"kind":"koblitz","a":1,"n":7}' \
  --factor-base 'binary-subspace:dimension=5,basis=0x1;0x2;0x4;0x8;0x20'
```

Use the identical string in an arm's method parameters:

```json
{
  "name": "explicit",
  "role": "candidate",
  "method": {
    "id": "ic.pipeline_counted",
    "params": {
      "factor_base": "binary-subspace:dimension=5,basis=0x1;0x2;0x4;0x8;0x20",
      "oracle": "mitm:m=2",
      "max_trials": "2000"
    }
  }
}
```

The complete recipe parameters and sorted point digest enter the `FB1`
identity. Decimal versus hexadecimal spelling and reordered or equivalent
bases may have different identities, even when their point sets agree. Freeze
and reuse the exact recipe. The regression test compares the entire recorded
factor-base metadata between `ecbench fb` and the pipeline, then requests a
fresh child replay of every run.

Base lifting and indexing retain the existing `binary-subspace` accounting.
This patch adds no cost calibration, measurement, or performance claim.

## Validation and delivery boundary

This is a source-only change prepared for opening as a PR. The available host
was not used to compile the plug-in, run Cargo tests, run Clippy, build an image,
or submit EKS jobs. The user specified insufficient local build disk and an
EKS image built from `crypto` `main`.

The change rejects oversized dimensions such as `4294967299` before a `u32`
cast could turn them into dimension 3. Unit cases cover this wraparound,
malformed words, zero and repeated vectors, field overflow, wrong list lengths,
and incompatible selection modes. CLI tests add a non-prefix basis,
dump/run identity parity, verified tiny synthetic answers, full replay, and
invalid-input diagnostics. These are test sources, not passing-test evidence.

Noncompiling checks performed for this patch: Rust formatting/parser checks,
`git diff --check`, and the existing curve-name checker's changed-line check.
The full-tree curve-name command was attempted and failed on an unmaterialized
tracked file (`.agents/skills/audit-runs/SKILL.md`) in this sparse checkout;
the changed-line result does not replace that gate.
Compilation and runtime verification remain pending.
On a builder with sufficient resources, complete the repository's required
checks, including:

```sh
cargo test --release --lib
cargo test --release --test ecbench
cargo clippy --release --lib --bin ecbench --test ecbench -- -D warnings
```

For focused diagnosis, the filters are `explicit_subspace_tests` in the library
and `explicit_binary_basis` in the ecbench integration target. Full required
checks and exact-head CI still govern delivery. After review and merge, build
the EKS image from the resulting `crypto` `main` commit and record that commit,
the image digest, and the ecbench binary hash. Confirm the image contains this
interface before any authorized experiment; this source handoff is not an
image-build or EKS-run receipt.

## Source review and session handoff, 2026-10-29

### What was done

Fetched `main`, inspected `BinarySubspaceBase`, `Params::parse_spec`,
`binary_subspace_factor_base`, the ecbench dispatch and factor-base identity
functions, and the existing explicit-basis regression. Consulted
`.agents/skills/ecbench-extend/SKILL.md`, the ecbench README sections 7 and 11,
and the [Rust integer parsing documentation](https://doc.rust-lang.org/std/primitive.u64.html#method.from_str_radix).
Reused the existing construction and prepared the validation fix, regression
coverage, and this input contract in a separate sparse worktree.

### Decisions made

No duplicate plug-in: current `main` already implements explicit binary bases.
Keep valid inputs and existing point ordering unchanged. Stop at a local commit
and a prepared PR description, per the user's requested boundary.

### Open threads

Compilation, runtime tests, Clippy, PR publication, exact-head CI/review, merge,
and the subsequent image build from `main` have not been performed by this
lane. No performance or mathematical result follows from these source changes.

### Errors

The initial long-lived checkout was behind `main` and lacked the explicit
basis parameter; fetching and inspecting the current source corrected that
assessment before implementation. Conductor reported the shared daily session
log held by T-827, so this scoped handoff retains the session entry without
editing the other lane's file. Scope checks passed for the implementation,
tests, and documentation; work was tracked by T-830 and T-831.
The full-tree curve-name lint cannot traverse this sparse checkout; its missing
file error remains an open validation limitation, while the changed-line check
reported zero problems.
