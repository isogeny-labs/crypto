# Structured M31 Semaev/F4 solver

This native component extracts signed witnesses for the **three-summand** constrained point problem

\[
P_1+P_2+P_3=Q,\qquad L_i(x(P_i))=0,
\]

on the caller's supplied nonsingular curve over `p = 2147483647`. It retains the
three supplied monic selectors exactly, supports different selectors per leaf,
and returns signed rational points. It uses the repository's original Rust F4
visitor, root recovery, field arithmetic, and curve arithmetic through the
existing source-bound native dependency. The separate RR5 component owns the
four-summand construction.

## Implemented formulations

| Selection | Equations | Exceptional case |
| --- | --- | --- |
| `expanded` | One expanded `S4(x1,x2,x3,xQ)` and all three selectors | Resultant roots are filtered by actual signed point addition |
| `chain_left` | `S3(x1,x2,u) = S3(u,x3,xQ) = 0` and selectors | A second chart imposes `x1=x2`, `x3=xQ` for an infinite intermediate |
| `chain_right` | `S3(x2,x3,u) = S3(u,x1,xQ) = 0` and selectors | A second chart imposes `x2=x3`, `x1=xQ` |

The monolithic polynomial is constructed by the exact quadratic resultant.
Writing the two quadratics as `a U²+b U+c` and `d U²+e U+f`, its expression is
`(af-cd)²-(ae-bd)(bf-ce)`. Leading-coefficient degeneracies remain in the input;
the constructor does not divide by an expression that might vanish.

Semaev polynomial zeros are algebraic coordinate candidates. Every accepted
output has rational curve lifts, lies in the caller's leaf selectors, and sums
to the full supplied point `Q`, including its y-coordinate. The finite chart
can contain extra algebraic candidates. An empty signed-lift result is a
rejection of that candidate, not an extraction error or an accepted relation.
The additional chart covers a canceling first pair, whose sum has no affine
x-coordinate. No claim of equality between the unsaturated chained ideal and
the resultant ideal is made.

## Certified selector preprocessing

With `require_selector_quotient`, each nonselector equation `f` is replaced by
its normal form under the unchanged monic univariate selectors. The original
selectors remain in the F4 input. Each replacement carries explicit cofactors:

\[
f-\operatorname{NF}(f)=q_1L_1(x_1)+q_2L_2(x_2)+q_3L_3(x_3).
\]

Consequently, the original and prepared equations generate the same ideal
**within each chart**. The replay function takes the original equations from
its caller and checks the identity by direct term accumulation, without
calling F4 or the producer's reduction function. It also checks that all
selector equations and the complete set of replacement rows were retained.
This is an explicit checkable algebraic certificate, not an independently
signed project verdict or a new formal Lean theorem.

`try_selector_quotient` attempts the same transformation within finite limits.
If construction exceeds a cap, that entire chart retains its original
equations and reports the fallback. All raw charts are constructed before any
optional preprocessing, so a failed preprocessing attempt cannot consume the
budget needed to construct the cancellation chart. Exhausting the shared
deadline still produces an incomplete solve.

This implementation performs selector reduction before calling the existing
F4 engine. It does not claim to replace F4's internal symbolic-preprocessing
algorithm, or to infer F4 solving degree from the degrees of the input rows.
The returned visitor diagnostics describe the actual bounded calls.

## Extraction and stopping contract

The F4 visitor receives no coordinate domain list. It may branch only on
algebraically recovered univariate roots. Its trace callback rejects both
full-field branching and Cartesian-domain fallback before specialization.
The finite base oracle in `tests.rs` is used only for correctness controls.
Algebraic root splitting can still visit many branches, including a product
of selector roots when coupling has not sufficiently constrained them. The
branch guard is an execution restriction, not a proof of subenumerative
complexity or a cost advantage.

The returned report retains chart inputs, transformation certificates,
algebraic assignments, signed witnesses, matrix/degree diagnostics, and
incomplete outcomes. `status=complete` means every chart's algebraic traversal
and signed lifting finished. Only a complete report with no witnesses
establishes absence for the supplied constrained point problem. A time cap,
allocation refusal, assignment cap, witness cap, or rejected fallback reports
`incomplete`. Partial signed witnesses remain available and individually
replayable.

The API has finite ceilings for source term counts, symbolic products, native
pairs, symbolic support, matrix entries, assignments, witnesses, and deadline.
Native allocation admission is a conservative modeled workspace limit; it is
not a process RSS measurement. `point_work` covers signed lifting, not a
complete cold-solving cost. No timing, asymptotic cost advantage, or end-to-end
ECDLP gain is asserted by this implementation.

## Build and use

```sh
cargo test --locked --manifest-path research/m31_integrated_build_20261123/structured/Cargo.toml
cargo build --locked --manifest-path research/m31_integrated_build_20261123/structured/Cargo.toml
```

The binary reads one JSON request on standard input. For example:

```json
{
  "problem": {
    "p": 2147483647,
    "a": 847514846,
    "b": 2087444724,
    "target": [625037715, 2100627251],
    "selectors": [[1522445932, 1], [1522445932, 1], [1522445932, 1]]
  },
  "formulation": "chain_left",
  "preprocessing": "require_selector_quotient"
}
```

`limits` is optional and otherwise uses the finite `Limits::default()` values.
The binary exits `0` after complete extraction, `3` for an incomplete report,
and `2` for malformed inputs or a construction refusal. The library's
`replay_witness` checks a single signed witness against the full problem.

## Validation and provenance

The M31 correctness controls cover the standard ordinary `PRIME31` curve,
expanded versus both chain orders, raw versus selector-reduced equations,
the missing infinite-intermediate case, complete emptiness, independent
finite-base signed enumeration, cofactor/source/selector corruption,
allocation and output limits, and exact fallback preservation. The oracle
uses separate test-only modular and point formulas. Test logs and exits live
in `evidence/`; they are development observations pending the repository's
independent validation and PCC controls.

One additional exceptional-point correctness control uses the same M31 field
with `a=2`, `b=3`, `T=(2147483646,0)`, and `Q=(3,6)`, testing `T+T+Q=Q`.
It checks a y-coordinate of zero and an infinite intermediate. It is neither
a benchmark instance nor a replacement for the ordinary `PRIME31` curve.

The dependency `rr5-original-f4-closure` builds the production F4 module and
checks its source provenance. Its source-range pins are maintained by the
combined integration task. This crate does not rewrite those pins or copy a
private F4 implementation. `Cargo.lock` records the resolved dependency set.

Sources checked: `src/cryptanalysis/f4_fp.rs`,
`src/cryptanalysis/f4_fp_tower.rs`,
`research/prime31_ic_20261115/native/lib.rs`, and the RR5 native dependency
closure. Research-KB tools were unavailable and `scripts/research-kb status`
returned exit 127, so this is a source-based review. Mathematical background:
[Semaev, 2004/031](https://eprint.iacr.org/2004/031) and
[Faugère, 1999, sections 2–3](https://wstein.org/129-05/refs/faugere_f4.pdf).
The work applies those standard constructions without a novelty claim.

## Auxiliary identity record

The generic quadratic resultant formula also has the repository-required
`identity.certificate/v1` record **`IDC1hd6568d78855aef4c`** in
`evidence/quadratic-resultant.identity.json`. The small native binary
`m31-structured-identity-certificate` constructs the closed formula and all 24
Leibniz terms of the 4x4 Sylvester determinant independently in formal variables
`a,b,c,d,e,f`. It serializes and rechecks the certificate through the existing
repository module, with 32 certificate points and 16 domain-separated extra
points. Adding one to the closed formula is refused by the issuer and rejected
by checker evaluation; both counterexamples are in
`evidence/quadratic-resultant.check.json`.

The v1 format fixes its evaluation field to **M61, 2^61-1**. This is a generic
auxiliary algebraic record, not an elliptic-curve fixture, a replacement for
the M31 tests, or a formal proof. The record has no proof pointer and no signed
verification claim. Issuance and checking use the same native program; the
native probability-bound diagnostic is not a formal identity proof. The exact
bounded invocation and exit are in `evidence/identity-certificate.command.txt`,
`.log`, and `.exit`. Existing solver arithmetic and tests are unchanged.
