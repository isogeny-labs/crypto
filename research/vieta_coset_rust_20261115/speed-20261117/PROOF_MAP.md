# Proof instance gate and current obligations

## Checked status — 17 November

The shared generic law is published at crypto-lean
23569e7f1da1abb865601f8f0dacc16fa11810e3, PR26. The study's pinned Lake project
builds and the shared proof gate passes21 theorems with0 trusted exceptions.
The gate checks transitive axioms, not just successful elaboration. Source
files, exact toolchain/dependency commits and gate receipts are retained.

| Declaration(s) | Scope | Status |
|---|---|---|
| CryptoLean.Polynomial.normalForm_constructible_bound | Generic actual-degree monomial/support induction under explicit NF contracts | kernel checked |
| VietaSpeedAudit.lastFallDegree_le_of_normalForm | Application to existing V/lastFallDegree, NF contracts still hypotheses | kernel checked |
| partition, coordinate10x/y, coordinate12x/y, coordinate2x/y, coordinate9x/y | Literal non-cartesian F17 projector and coordinate identities | kernel checked |
| coprime_witness, upper_fiber_exact, lower11_fiber_exact, lower14_fiber_exact, fiber_roots_distinct, leaf_denominators_nonzero | Exact finite full-splitting/coprimality/pole conditions | kernel checked |
| nonsplit_control, repeated_tower_fiber | Distinguish forbidden tower fibers from allowed local slope-algebra cases | kernel checked |
| nilpotent_root_square_zero, nilpotent_root_nonzero | Actual AdjoinRoot(X^2), nonzero nilpotent retained using a map to DualNumber | kernel checked |
| Arbitrary-depth tower NF/finite-product instance | Instantiate all contracts and degree budgets for every allowed tower | unfinished |

The integer certificate generator is native Rust and is not trusted: Lean's
kernel checks every emitted multiple-of17 identity. Finite `decide` is kernel
reduction, not native_decide. The listed theorem axioms are confined to
propext, Classical.choice and Quot.sound. The full tower bound is not promoted.

The bundled lean4lean alternate-checker probe failed on incompatible object
headers, including Std.olean, so no independent recheck is claimed from that
executable. Its help invocation unexpectedly attempted a library scan; only
the observed diagnostic is retained, not an invented successful checker exit.
Shared PR26 cannot enable auto-merge under that repository's configuration.
Independent signed standing and the general tower instantiation remain open.

Shared library source: aburan28/crypto-lean at
01750b4044989583e70e4886b4ab5a79fe83f128. Lean4.35.0, Mathlib
cfc55fc58714a59dc06e6e79edc6d1d95140d3c2. Original constructible-space and
last-fall definitions: crypto-autoresearcher `formal/CryptoResearch/FallDegree/LastFall.lean`
at the recorded sibling checkout revision; import the exact source rather than
introducing a second definition. Its generic representation-degree upper bound
does not automatically establish the tower's bound: constructible closure can
use actual-degree falls, whereas a fixed representation may have larger terms.

## Instance gate

1. Structure above: Mathlib `MvPolynomial sigma k`, its total degree and
   `Submodule k (MvPolynomial sigma k)`. The semantic objects are existing
   polynomials/subspaces, not a new expression syntax.
2. Missing generic law: degree-controlled linear normal-form certificates imply
   `f-NF(f)` lies in the constructible space at `max(T,deg f)`. This belongs in
   crypto-lean `CryptoLean/Polynomial/ConstructibleNormalForm.lean` first.
3. Instance: the existing autoresearcher `V F n` family, with `V_mono` and
   `mul_mem_V`; the finite product of retained slope algebras supplies NF.
4. Assembly probe: type-check the generic law, then apply it to those existing
   closure laws and the original `lastFallDegree` definition. Expose every NF
   contract as a hypothesis; no contract states the desired ideal-degree bound.
5. Missing components: the generic monomial induction; then the *actual tower*
   projector/finite-product construction satisfying the NF hypotheses. The
   second is a separate obligation, not assumed solved by the first.
6. Inheritance: polynomial arithmetic/degree from Mathlib; monotone closure
   and degree-bounded multiplication from existing V; linearity from LinearMap;
   coordinate actions/compatibility and strict degree bound from the tower
   product-algebra construction, still to instantiate generically.

## Proof chain

For constants use scalar multiplication of `1-NF(1)`. For a monomial p and a
variable X, multiply `p-NF(p)` by X using its **actual** degree bounded by
`max(deg p,T-1)`; add the low-degree NF coordinate-action certificate.
The degree of nonzero pX is deg p+1. Sum over the actual support of f, whose
monomial degrees are bounded by deg f, rather than inducting over arbitrary
polynomial sums with cancellations. If NF kills the ideal, the original last
fall definition then gives the upper bound. No finite-rank computation or
kernel check of this abstract bridge alone supplies all tower hypotheses.

The readable tower audit must separately retain squarefree full fibers,
coprime N/D, degD<degN, the empty-composition convention, and nilpotent/nonsplit
slope blocks. Repeated *tower fibers* violate the proposed theorem's hypothesis;
repeated *abscissae/slope roots* require scheme algebra and remain supported.
