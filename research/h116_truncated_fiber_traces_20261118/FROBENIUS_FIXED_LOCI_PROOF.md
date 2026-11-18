# H116 algebraic-point Frobenius exhaustion proof

## Outcome

The pinned Lean module now proves that every affine elliptic point over an
algebraic extension of `ZMod p` is fixed by some positive power of the actual
coordinate Frobenius endomorphism. Consequently, if the characteristic
expression vanishes on every positive Frobenius-fixed locus, it vanishes as an
endomorphism on all algebraic points and yields the exact pointwise identity
`V-1=[t-1]-pi`. [pcc:unverified]

The source SHA-256 is
`c7e24fef4e4b1a4d53194dd1b1a55af9185505d8b3820dcd25901f3ae38c1360`.
The pinned 3,157-job build passes. The exact repository auditor reports 119
theorems, zero failures and zero trusted native decisions. Build-receipt
SHA-256 is
`4165925670b0621960cfbf9f0361276bf60b7b3c134653a847aeb2377ca9f2fb`;
audit-receipt SHA-256 is
`b04c0644adf3fbb0fe15e4cbe36456b2bec88be2f226f29e2c6376066161068a`.
[pcc:unverified]

## Proof chain

1. `zmodAlgebraicPair_frobeniusPeriodic` adjoins both affine coordinates to
   the prime field. Algebraicity makes this intermediate field finite
   dimensional, hence finite. Its cardinality is `p^n`, and finite-field
   exponentiation fixes both coordinates. [pcc:unverified]
2. `zmodAffineFrobeniusPointEnd_pow_some` proves by induction that the `n`th
   power of Mathlib's actual point Frobenius raises both coordinates to
   `p^n`. [pcc:unverified]
3. `zmodAffineFrobeniusPointEnd_periodic` combines those facts, including the
   point at infinity, to show that every algebraic point lies in a positive
   fixed locus. [pcc:unverified]
4. `endomorphism_eq_zero_of_periodic_fixedLoci` and
   `endomorphism_characteristic_of_periodic_fixedLoci` promote vanishing on
   all fixed loci to global endomorphism equality. [pcc:unverified]
5. `zmodAffineVerschiebung_sub_one_eq_of_fixedLoci` composes the exhaustion
   theorem with the already proved norm, cancellation and orientation chain.
   [pcc:unverified]

All seven new theorem declarations report only `propext`, `Classical.choice`
and `Quot.sound`; the two generic promotion lemmas use only `propext` and
`Quot.sound`. [pcc:unverified]

## Formal control

`endomorphism_single_fixedLocus_control` uses negation as `pi` and the identity
as a mutation on `ZMod 3`. At period one, the fixed locus contains only zero,
so the identity mutation vanishes on every checked point. At period two, one
is fixed and exposes the nonzero mutation. This proves that one fixed locus
cannot replace the theorem's quantifier over all positive periods.
[pcc:unverified]

## Prospective finite test

Protocol `H116-FROBENIUS-FIXED-LOCI-001`, SHA-256
`0191f93cc986b025224fcad313e8be85842f1599a223f92aa2e94aefb2a2772d`,
is frozen before new producer or verifier implementation. It will measure
least-period strata, every fixed-locus count, full-period coverage and
characteristic failures on the unchanged eight-cell panel. It has no result
rows yet. [pcc:unverified]

## Boundary

The proof replaces an unstructured quantifier over algebraic points by an
exact directed family of finite Frobenius-fixed loci. It does not prove the
characteristic equation on those loci. It also does not identify pointwise
`V-1` with a rational or scheme morphism, transport it to a completed local
series, solve ECDLP, or establish subexponential runtime. [pcc:unverified]
