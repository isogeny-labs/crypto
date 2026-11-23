# Same-team source review — 2026-11-23

**Status: unsigned same-team source review, not signed verification or a PCC verdict.**
The reviewer authored the separate identity-record binary and performed its
previously recorded fresh-seed replay. This review reads source and retained
results; it performs no new build, solver execution, or test. [pcc:unverified]

Reviewed the corrected [example](../../../examples/dickson_s3_quadratic.rs),
the eleven added lines in [ic_boundary.rs](../../../src/cryptanalysis/ic_boundary.rs),
the relevant production polynomial/visitor/sign-lift routines, and the
`correctness-02`, `example-tests-02`, and preceding failure records.
[build-03-inputs.sha256](build-03-inputs.sha256) records these source digests:

| Source | SHA-256 |
|---|---|
| Example | `8393e95f937b2d50821d4445ba44a771ba65313448885b03c895ac77d9a4e05e` |
| Production boundary | `9abe0bdcc7bb7940515bad9cc72418a779272002b5a84e23de87c2674793ab7c` |

## Construction and domain findings

- `collect_terms` now sums coefficients through production `Fp32`, removes
  zeros, and sorts with the production grevlex comparator. It does not divide
  by a leading coefficient. This directly repairs the earlier misuse of
  `f4_fp::normalise`, whose documented monic scaling changes intermediate
  expressions. No remaining expression-node rescaling was found.
- The quartic arm implements the ordinary expanded S3 formula. Q implements
  `r=xy`, `e2=r+z(x+y)`, `e3=rz`, and
  `(e2-A)^2=4(x+y+z)(e3+B)`. L implements `r=xy`,
  `lambda^2=x+y+z`, `r+z(x+y)=A-2*lambda*nu`, and
  `rz+B=nu^2`. The signs agree with coefficient matching. Q/L equations and
  leaf ladders have total degree at most two; the constructor checks this.
- The depth-ell selector correctly ends with `d_last^2-2=0`, representing
  `D_(2^ell)(x,1)=0`. All arms use identical leaf equations. Every domain
  supplied to `solve_visit` is `None`, meaning the full coefficient field;
  neither intermediates nor auxiliaries inherit the selector-root domain.
- Q has unique invariant auxiliaries. L retains the horizontal `e1=0` branch
  without division and can reject isolated twist-only triples. Its simultaneous
  line-sign witnesses have separate multiplicity from projected tuples;
  the exceptional `lambda=nu=0` lift is single. Repeated coordinates encode
  tangency/intersection multiplicity. The finite affine chain explicitly
  excludes a prefix equal to the identity.

No blocking equation or domain defect was found in the corrected example
within that stated scope. Known-prime fixtures are supplied internally; this
is not a general primality or curve-order validator. [pcc:unverified]

## Extraction and evidence limits

The tiny visitor rechecks every returned polynomial assignment, uses
production `lift_abscissae`, and literally replays the full signed point sum
against the target before inserting an ordered leaf tuple into a set.
The independent two-by-two selector-tuple control is not supplied as solver
domains. Completeness/timeout/early-stop guards and fallback counters remain
visible. [correctness-02.stdout](correctness-02.stdout) records the same two
ordered leaf tuples for all arms, with raw solution counts 2/2/4; these are
not three separate useful-relation counts or performance comparisons.

`recover_signed` enumerates sign choices and returns one valid choice. Its
minimal factor-base view supplies curve points and x indices, not subgroup
certification or relation-column metadata. It validates a leaf relation;
it does not enumerate every signed relation or reconcile every supplied
local line witness. Only the tiny m=2 system is actually solved. M31 m=2/3/4
checks construct public planted witnesses, including repeated points; they
are not unknown-target extraction.

The retained [initial tests](example-tests-01.stdout) report one pass and four
failures. The [corrected tests](example-tests-02.stdout) report five passes;
[correctness-02-post-call.json](correctness-02-post-call.json) records exit 0
and an unchanged frozen binary. The corrected correctness output covers
468 local triples over F5/F7, 4096 distinct M31 selector roots at depth 12,
horizontal/twist controls, planted signed assignments, and the tiny complete
solve. These records support that qualification scope. The separate IDC
records are evaluated identities, not certificates for the complete program,
general Dickson ladders, production extraction, or a speedup. [pcc:unverified]

## Production boundary repair and unresolved gate

The eleven additions restore method/guard delimiters, the documented `None`
defaults for `abscissa_key_bound` and `x_sibling_pair`, and the local binary
field binding. Existing arithmetic formulas and accounting statements are
unchanged. No unintended behavior change was identified by inspection;
the malformed original has no executable baseline for a runtime-equivalence
claim. The scoped build includes the complete `CountedGroup` trait and
`PrimeCurve` implementation, but not the `BinaryGroup` implementation, as
shown in [source-manifest-02.json](source-manifest-02.json).

**The full-root gate remains unresolved.** The reviewed
[root-build-02 result](root-build-02-result.json) exits 101 before tests because
an offline dependency was unavailable. No successful full-root build or
BinaryGroup test verdict was reviewed. A further preexisting source risk is
that `BinaryGroup::sibling_pair` initializes `SiblingPair` without its
`y_plus`/`y_minus` fields; this is outside the eleven-line repair and the
scoped qualifier. It requires the actual root gate, not inference from the
qualifier's successful build. [pcc:unverified]
