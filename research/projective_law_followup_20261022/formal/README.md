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
- `branch-coverage.json` gives the standardized runtime-label, Lean
  constructor, certificate-schema mapping and lists the five rejected or
  exceptional labels separately.

Build from this directory:

```sh
MATHLIB_NO_CACHE_ON_UPDATE=1 lake update
lake build
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

The first five items are algebraic theorems over abstract fields or
commutative rings.  They are not finite-field samples.  The runtime mapping is
a manually transcribed finite inventory; `source-bindings.json` pins the Rust
source it was read from, but Lean does not parse or compile that Rust source.

`AxiomAudit.lean` rejects dependencies beyond `propext`,
`Classical.choice`, and `Quot.sound`.  The actual audit found only subsets of
those accepted Lean/Mathlib foundations.  The mapping theorem and the explicit
bridge implication use no axioms.

## Remaining theorem bridge

The packet does **not** define the smooth hyperelliptic curve, Mumford
reduction, principal divisors, or `Pic^0`.  The structure
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
Rust implementation.  The full remaining acceptance condition is tracked in
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
