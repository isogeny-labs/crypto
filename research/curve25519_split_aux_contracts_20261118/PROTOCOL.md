# Protocol: signed split-auxiliary contracts on full-width Curve25519

**Frozen 2026-11-18 before adding or running the new contract tests.**

## Request and scope

The research request is to use the Isogeny Labs Crypto repository to identify
Curve25519 index-calculus leads involving symmetry, splitting, and auxiliary
variables, establish the current baseline, and set a concrete optimization
target. This round isolates one prerequisite of a split-Semaev relation finder:
the exact information retained and lost when a signed group relation is split
through an auxiliary abscissa.

This is a correctness-only search round on the repository's exact RFC 7748
Curve25519 model. It does not time a solver, estimate an exponent, collect
relations, solve a discrete logarithm, or claim a performance result.

## Fixed model and baseline

- Field: `p = 2^255 - 19`.
- Montgomery model: `v^2 = u^3 + 486662*u^2 + u`.
- Short-Weierstrass adapter, subgroup order, cofactor, and generator: exactly
  `Curve25519IcModel::rfc7748()` in
  `src/cryptanalysis/curve25519_ic.rs`.
- Arithmetic: the repository's generic exact `BigUint` point path.
- Algebraic oracle: `Curve25519IcModel::semaev_s3` on short-Weierstrass
  abscissae.
- Existing baseline check: `exact_s3_vanishes_on_a_zero_sum_triple` checks
  one unsplit three-point witness.

The repository's 2026-11-12 closure note treats split `S3` chains with
intermediate variables as outside its restricted unsplit XL/Macaulay model.
This round therefore checks a representation contract; it does not extend
that model's lower bounds to split systems.

## Frozen witnesses

All scalars multiply the exact RFC base point `G` in the declared prime-order
subgroup.

### A. Finite split join and sign control

- `P = [2]G`, `Q = [3]G`, `S = [5]G`, `T = [7]G`, `R = [12]G`.
- Exact joins: `P + Q = S` and `S + T = R`.
- Algebraic joins:
  `S3(x(P), x(Q), x(S)) = 0` and
  `S3(x(S), x(T), x(R)) = 0`.
- Negative control: replace `P,Q` by `-P,-Q`. Their abscissae are unchanged
  and their sum has the same auxiliary abscissa `x(S)`, but the fixed-signed
  equation `(-P) + (-Q) + T = R` is false.

This control distinguishes the existential sign semantics of an x-only
Semaev chain from an exact fixed-signed relation witness.

### B. Identity auxiliary branch

- `P = [2]G`, `Q = -P`, `S = O`, `T = [7]G`, `R = T`.
- Exact joins: `P + Q = O` and `O + T = R`.
- The auxiliary point has no affine abscissa.

This control records the branch that an affine-only auxiliary variable omits.

## Implementation and commands

Add two tests to the existing `curve25519_ic` test module:

1. `split_join_distinguishes_fixed_signs_from_x_only_witnesses`;
2. `split_join_keeps_point_at_infinity_branch`.

Run, in order:

```text
cargo test --release --lib split_join_distinguishes_fixed_signs_from_x_only_witnesses -- --exact
cargo test --release --lib split_join_keeps_point_at_infinity_branch -- --exact
cargo test --release --lib curve25519_ic
```

If Rust's exact test filter requires the module-qualified name, rerun the same
test with that name and record both commands and exits. No timing from these
commands is research evidence.

## Acceptance and falsifiers

The round passes only if:

1. both exact group joins in witness A hold;
2. both `S3` evaluations in witness A vanish;
3. the negative control keeps the same x-only inputs but fails the chosen
   fixed-signed relation;
4. witness B reaches `Point::Infinity`, has no `x_coord`, and still satisfies
   the exact four-point relation;
5. the complete `curve25519_ic` test subset passes in release mode.

Any failed condition is retained as a counterexample and reported. No witness
or scalar may be changed after a failure without a separately frozen addendum.

## Evidence and reporting

The search round will retain the exact source revision, commands, exit status,
and observed assertions in its README. It will include an editable diagram,
rendered SVG, Typst report source, and inspected PDF. Statements remain
`[pcc:unverified]` unless separately registered and signed by the repository's
claim workflow.

Because this round makes no new general polynomial-identity claim, it does not
issue a new `identity.certificate/v1` record. The report will describe the two
concrete checked witnesses, not promote them to a general theorem.

## Next optimization target (not run in this protocol)

The follow-up target is a projective split-chain relation finder that carries
an explicit infinity branch and enough lift/sign information to verify the
chosen signed relation. Its first measured comparison should use the existing
Curve25519 miniature/F4 harness with matched targets and seeds, charging all
extra variables, branches, verification, and solver multiply-adds. That run
requires its own frozen protocol and records.
