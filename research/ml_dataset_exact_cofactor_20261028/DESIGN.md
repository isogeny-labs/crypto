# Exact prime dataset cofactor context

Design frozen before implementation or new fixtures, 2026-10-28. Base source
`c14d4552e513f6e28bda0e0e045f5e64beb492db` (binary runtime d79). Conductor
T-751 owns only `src/ml_dataset/curves.rs`, `src/ml_dataset/solvers.rs` and this
research directory. This is software coverage work under the existing corpus
goal, not an experiment, new budget, worker launch or mathematical result.

## Concrete problem and intended behavior

The public dataset already stores exact decimal group order, prime-subgroup
order and cofactor. Its construction and reference arithmetic use word or
BigUint scalars. Nevertheless, `curves::generate` rejects every selected prime
curve with cofactor above `u32::MAX`; `prime_curve` repeats that narrowing to
the legacy `ecc::CurveParams.h` field. No mathematical obstruction requires
this rejection. Preserve the chosen curve and all public identities, remove
this representational rejection, and let the existing bounded construction,
membership and solver gates report their actual outcome.

Introduce a dataset-local exact prime-curve context with BigUint field and
subgroup values and exact u64 cofactor. It provides only the existing point
arithmetic helpers needed by this adapter: canonical field elements, a,
generator and curve-membership checks. Point addition/multiplication continue
to use the existing independent textbook `Point` implementation. Construct
`prime_fast::FastCurve` directly from the validated p/a/b/r words; that type
represents arithmetic and subgroup scalars and has no cofactor field.

Do not change the legacy public `ecc::CurveParams` type, introduce a fake h=1,
truncate a cofactor, resample the selected curve, alter a subgroup or reset
frozen seeds. No solver requires the narrow type: all prime branches read
coefficients, field/subgroup values and point helpers, while PKM uses its own
exact u64 cofactor context. Existing F4 p<2^32, PKM p<2^31 and arity caps remain.

## Admission and compatibility

Parse canonical decimal strings and preserve existing field primality, degree,
nonsingularity and coordinate checks. Require positive subgroup/cofactor and
checked `subgroup_order * cofactor == group_order` before admitting the exact
context. This arithmetic identity does not replace the independent group-order
certificate, generator/subgroup checks or actual cyclic-membership tests.
The existing full public verifier remains the authority for those checks.

For n<=61, the existing admitted field and group-order words fit u64; public
construction still verifies its actual order rather than trusting a machine
width. Cases outside the existing word-field limits remain unsupported.

Keep all v1 public/config/artifact schemas, factor-base policies, RNG draws and
small-cofactor results unchanged. No private scalar or subgroup-membership log
becomes a feature or solver input. A large exact cofactor is public structural
metadata; it is not evidence that a method solved the target or improved cost.

## Validation obligations

1. Compare the new context against legacy `CurveParams` on admitted small-
   cofactor fixtures: generator, curve membership, point addition/scalar replay,
   arithmetic field/subgroup parameters and existing public output identity.
2. Freeze one independently checked prime-curve fixture with cofactor above
   2^32 before solver checks. Retain exact order evidence, point/subgroup checks
   and scalar replay; no fake cofactor on an unrelated curve qualifies.
   A bounded deterministic software fixture may use the existing native point
   counter and primality code. Preserve failed setup instead of relabeling it.
3. Check zero/noncanonical/overflowing cofactors and wrong group-order products
   fail closed. Exercise exact large-cofactor construction/reference replay and
   at least one generic solver path without supplying its private witness.
4. Preserve F4/PKM unsupported boundaries and confirm root/static-export
   metadata remains attached to the exact public domain. Run focused dataset
   tests, strict scoped Clippy and the required final full-library gate.
5. Bind final source, test executables and retained logs; publish immutable S3
   packets with fresh GET/hash verification and a regular dependent PR. Neither
   software fixtures nor a merged PR establish cloud campaign completion.

## Sources checked

- Repository `src/ecc/curve.rs` and `point.rs`, `src/cryptanalysis/prime_fast.rs`,
  `src/ml_dataset/{curves,solvers}.rs`, and the exact-cofactor gap in
  `docs/ml-dataset/REMAINING-COVERAGE.md`.
- [SEC1 v2, C.2](https://www.secg.org/sec1-v2.pdf) records the cofactor as an
  integer equal to curve order divided by base-point order. Its standard
  cryptographic domain-selection rules are separate from this research
  population. No cryptographic-strength or novelty claim follows here.

Actual EKS S3 authorization, durable budget admission, original USD100–200
total ceiling, current external owners and deadlines remain unchanged. This
increment does not launch, retry, train or promote a first/last-fall-degree claim.
