# Projective-law formal proof boundary

This directory pins and builds a Lean packet for algebraic obligations used by
the weighted genus-2 projective-law implementation.  It is a proof-boundary
artifact.  It does not claim a complete formal proof of Jacobian addition.

## Pinned environment

- Lean `v4.34.1`, pinned by `lean-toolchain`.
- Mathlib commit `d13f23b723b8a846827a245b89c10fc7d3f11612`, pinned by
  `lakefile.toml` and `lake-manifest.json`.
- Runtime source and native certificate inputs are bound by SHA-256 in
  `source-bindings.json`.
- Four exact Rust function fragments are brace-scanned and bound separately in
  `runtime-bindings.json`; `verify.py` recomputes those fragment hashes.
- The [coefficient-slice packet](COEFFICIENT_SLICE.md) derives seven common
  2+2 coefficient expressions from the pinned Rust and Lean source, compares
  their expanded integer polynomials, and rejects operator mutations on both
  sides. Its [certificate](coefficient-slice.json), [CSV](coefficient-slice.csv),
  [graph](coefficient-slice.svg), and [PDF](COEFFICIENT_SLICE.pdf) are reviewable
  separately. This check covers only those seven expressions.
- The [cross-scaling packet](CROSS_SCALE22.md) compares the eight power
  intermediates, eight weighted common-branch input expressions, and seven
  composed coefficients with their Lean definitions. Its kernel-checked theorems prove the common
  `Z=z1*z2`, `W=w1*w2` weights when the weighted numerator equations hold,
  derive `B=Z⁸B_aff`, `A=Z⁹WA_aff`, `C=Z⁷WC_aff`, and preserve the nonzero
  `B,C` gates for nonzero weights.
  The packet includes a [certificate](cross-scale22.json),
  [CSV](cross-scale22.csv), [graph](cross-scale22.svg), and
  [PDF](CROSS_SCALE22.pdf) and a [coefficient-weight table](cross-scale22-weights.csv);
  it does not prove the returned Jacobian class.
- The [common residual packet](COMMON22_RESIDUAL.md) proves the exact cleared
  ordinate congruence in Lean and checks five source-to-theorem output
  obligations. Its [certificate](common22-residual.json),
  [summary CSV](common22-residual-summary.csv), [graph](common22-residual.svg),
  accompany a [PDF](COMMON22_RESIDUAL.pdf) and term-level CSV in the artifact
  store. Pull both from the repository root with:

  ```sh
  python3 tools/artifacts/artifacts.py pull research/projective_law_followup_20261022/formal
  ```
- The [residual-support head packet](COMMON22_SUPPORT_HEAD.md) proves the
  unit-weight common branch's `X⁶`, `X⁵`, and `X⁴` coefficients are `0`,
  `B²`, and `0`. Its source checker binds `B,A,C,Q,Rbar` to Rust and rejects
  both-side `Q` sign mutations. The [certificate](common22-support-head.json),
  [CSV](common22-support-head.csv), [graph](common22-support-head.svg), and
  artifact-stored [PDF](COMMON22_SUPPORT_HEAD.pdf) retain the bounded result.
- The [coprime-support uniqueness packet](COMMON22_UNIQUENESS.md) proves that
  nonzero source-style determinant `B` makes the two quadratic supports
  coprime by an explicit Bézout identity; the supports' degree-two and
  nonzero properties are also checked. Conditional residue lemmas derive
  the two support divisibilities from interpolation and valid-input curve
  congruences. Separate theorems derive the strict degree bound from a
  sextic upper bound with zero sixth coefficient, matching quintic heads,
  and nonzero quadratic supports; the bound then
  forces the cleared residual difference to vanish. The runtime interpolation
  link remains open. Four generic polynomial
  theorems now expand `N²-u₁u₂U` and read its degree-six, five, and four
  coefficients. Four source-style theorems substitute the proof-side
  coefficient formulas and recover `0,B²,0`; they also equate the canonical
  cubic with its interpolation form and prove its second-support
  factorization. The proof-side residual is at most
  sextic, and its difference from a monic quintic with zero fourth
  coefficient has degree below the support product. A further theorem
  combines these source-style ingredients into the cleared curve equation
  under valid-input assumptions. Runtime semantic correspondence remains
  open. The [CSV](common22-uniqueness.csv),
  [JSON](common22-uniqueness.json), [graph](common22-uniqueness.svg), and
  [PDF](COMMON22_UNIQUENESS.pdf) distinguish the kernel-checked implication
  from source and geometric premises still to establish.
- `branch-coverage.json` gives the standardized runtime-label, Lean
  constructor, certificate-schema mapping and lists the five rejected or
  exceptional labels separately.

Build from this directory:

```sh
MATHLIB_NO_CACHE_ON_UPDATE=1 lake update
lake build
```

For the dependent source-head file, build its two local imports before the
read-only kernel check:

```sh
lake env lean -o .lake/build/lib/lean/Common22SupportHead.olean Common22SupportHead.lean
lake env lean -o .lake/build/lib/lean/Common22PolynomialHead.olean Common22PolynomialHead.lean
lake env lean -o .lake/build/lib/lean/Common22Coprime.olean Common22Coprime.lean
lake env lean -o .lake/build/lib/lean/Common22Degree.olean Common22Degree.lean
lake env lean Common22SourceHead.lean
```

`MATHLIB_NO_CACHE_ON_UPDATE=1` avoids unpacking the complete Mathlib binary
cache.  It does not alter the pinned dependency graph or the proof checked by
Lean.  The successful build output is retained in `build.log`; the axiom-audit
lines are copied verbatim to `axiom-audit.log`.

## Kernel-checked scope

`ProjectiveLawBoundary.lean` proves:

1. equal squared ordinates over a field are equal or opposite;
2. the same-`u`, zero-slope-delta mismatch collapses to the inverse branch,
   with characteristic two explicitly excluded;
3. `B=0` and `dq=0` force equal quadratic coefficients, excluding the
   `shared_zero_q_delta` label for distinct inputs;
4. the cleared output-ordinate identity used by the generic 1+2 formula;
5. both exact factorization identities expressing that the generic 2+2 cubic
   interpolant agrees with the two input ordinates modulo `u1` and `u2`;
6. a total, manual mapping from the reachable runtime label inventory to the
   eight native certificate schemas or the definitional inverse case.

`RuntimeSemantics.lean` adds a manual executable abstraction of the decision
order in `add12_weighted_complete` and selected proof-side intermediates
associated with the generic 1+2 and common 2+2 calculations. Its five
kernel-checked theorems are conditional on their supplied hypotheses:

1. a valid equal-square ordinate pair cannot select the 1+2 mismatch result;
2. the selected 1+2 abstraction satisfies the proved cleared ordinate identity
   when its two defining intermediate equations are supplied;
3. the proof-side 2+2 polynomial witness has the stated exact remainders modulo
   both input support polynomials; and
4. through an explicit principal-divisor interface, a residual divisor has
   the inverse class of the input whenever their sum is principal.

The algebraic items are theorems over abstract fields or
commutative rings.  They are not finite-field samples.  The runtime mapping is
a manually transcribed finite inventory; `source-bindings.json` pins the Rust
source it was read from. The fragment bindings identify four exact function
bodies associated with the manual abstractions, but Lean does not parse or
compile Rust and no Rust-to-Lean derivation is proved. In particular, the 2+2
`N` in `generic22Runtime` is a proof-side congruence witness; the Rust function
does not expose that variable.

`AxiomAudit.lean` rejects dependencies beyond `propext`,
`Classical.choice`, and `Quot.sound`.  The actual audit found only subsets of
those accepted Lean/Mathlib foundations.  The mapping theorem and the explicit
bridge implication use no axioms.

`verify.py` also runs all four source-derived checkers in read-only mode; a stale
or altered certificate fails verification. It requires exactly four distinct Rust functions and their exact Lean
declaration sets, parses declarations after removing nested comments and
strings, and cross-checks the status count, schema, binding kind, and audited
theorem set. Every verifier run also confirms rejection of a duplicate row, a
hash-valid unexpected Rust function, an unexpected but real Lean declaration,
and a name present only in a Lean comment.

## Remaining theorem bridge

The packet does **not** define the smooth hyperelliptic curve, Mumford
reduction, concrete principal divisors, or `Pic^0`.  The structure
`DivisorClassBridge` makes the missing implication visible:

```text
literal formula identities
  + valid reduced Mumford inputs
  + smooth odd-characteristic quintic
  + branch composition and degree-drop arguments
  => returned representation is the sum in Pic^0
```

`formula_to_divisor_class` only applies a supplied bridge.  It does not
construct or assume one globally.  Therefore this packet strengthens the
mechanized algebraic boundary but does not close universal correctness of the
Rust implementation. `PrincipalClassInterface` and
`residual_represents_input` additionally prove the final abstract group step,
but construction of that interface for the intended smooth curve remains
open. The full remaining acceptance condition is tracked in
`problems/genus2-projective-transfer-backend.md`.

The standardized reviewer summary is available as [REPORT.md](REPORT.md),
with the same proof boundary rendered in [formal-status.svg](formal-status.svg)
and [REPORT.pdf](REPORT.pdf).

## Failed setup attempt retained

The first environment setup invoked a full `lake exe cache get`.  Extraction
failed with `No space left on device` while unpacking the complete Mathlib
cache.  No theorem was checked in that attempt.  Only that attempt's untracked
`.lake` directory was removed, and the build was restarted with Mathlib's
documented no-auto-cache environment setting and targeted imports.  The
failure and recovery are recorded here because they change how the successful
build was produced; they do not change its theorem scope.
