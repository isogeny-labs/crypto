# N6 formal proof-boundary report

**Status:** partial mechanization
**Generated:** 2026-10-25
**Schema:** `projective-law-formal-report/v1`

## Recorded result

Lean 4.34.1 with Mathlib revision
`d13f23b723b8a846827a245b89c10fc7d3f11612` kernel-checks three
branch-selection lemmas, three literal formula identities, totality of a
manual 17-row reachable-branch mapping, and application of an explicitly
caller-supplied divisor-class bridge. The formula identities include exact
factorizations for both generic 2+2 interpolation congruences.

The packet does not establish Rust-to-Lean semantic correspondence, a smooth
genus-2 curve or Mumford representation in Lean, the principal-divisor and
`Pic^0` bridge, the complete exceptional-branch composition, or independent
mathematical review. The conditional theorem `formula_to_divisor_class`
constructs no bridge and assumes no bridge globally.

![Formal proof boundary](formal-status.svg)

The graph counts named obligations in this packet. Counts are an inventory;
they are not measures of proof effort, importance, or universal coverage.

## Standardized source table

| Evidence | Recorded status | Scope or limit |
|---|---|---|
| `equal_or_opposite_of_sq_eq_sq` | kernel checked | Abstract field; assumes equality of squares. |
| `same_u_zero_slope_delta_is_inverse` | kernel checked | Abstract field; assumes the two coefficient identities and characteristic not two. |
| `shared_zero_q_delta_unreachable` | kernel checked | Abstract field; assumes distinct coefficient pair and `B=0`. |
| `generic12_output_v_clearing` | kernel checked | Literal commutative-ring identity; no divisor argument. |
| `generic22_interpolates_first` | kernel checked | Exact factorization by explicit `u1=X^2+q1*X+r1`. |
| `generic22_interpolates_second` | kernel checked | Exact factorization by explicit `u2=X^2+q2*X+r2`. |
| `runtime_certificate_mapping_total` | kernel checked, manual scope | Total only over the manually transcribed Lean inventory; the Rust source is hash-bound, not parsed by Lean. |
| `formula_to_divisor_class` | kernel checked, conditional | Applies a caller-supplied implication; supplies no geometric bridge. |
| `source-bindings.json` | integrity checked | SHA-256 binds runtime source, written audit, and eight native certificate sources/results. |
| `branch-coverage.json` | integrity checked | 17 mapped labels and five rejected or unresolved labels; semantic correspondence remains unproved. |

## Validation

- `MATHLIB_NO_CACHE_ON_UPDATE=1 lake build` completed 740 jobs.
- The axiom audit found only subsets of `propext`, `Classical.choice`, and
  `Quot.sound`; it found no unexpected dependency.
- `python3 verify.py` verified source bindings, result schemas, status, branch
  counts, and retained build/axiom receipts.
- `sha256sum -c SHA256SUMS` verifies the packet files listed there.

The retained build and axiom outputs are `build.log` and `axiom-audit.log`.
The exact formal environment is pinned by `lean-toolchain`, `lakefile.toml`,
and `lake-manifest.json`.

## Reviewer boundary

The next proof obligation is a faithful formal model joining the hashed Rust
case split to reduced Mumford divisor classes on the stated smooth quintic
family, including exceptional compositions and degree drops. Until that is
supplied and independently reviewed, this packet records algebraic sublemmas
and a manual inventory rather than a universal group-law proof.
