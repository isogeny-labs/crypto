# Independent pre-run review

Date: 2026-11-23 UTC. PR: #4966. This note records source inspection and the
checker contract before any new producer or reviewer call. It does not report
an execution result or signed verification. [pcc:unverified]

## Sources inspected and current scope

- `src/cryptanalysis/semaev_selector.rs`: the existing division-free quadratic
  selector resultant, degree-drop and zero-polynomial handling, split-selector
  coordinate extraction, and separate signed rational lifting contract.
- `src/cryptanalysis/semaev_prime_poly.rs`: existing producer field arithmetic,
  S3 representation and polynomial helpers.
- `research/fass_prime31_20261122/target_quotient/{REPORT.md,PROOF_MAP.md,PROTOCOL.md}`:
  distinction between the nonreduced right product, rational squarefree
  quotient, canonical coefficient rank, and algebraic zero support.
- `target_quotient/evidence/independent/SELECTOR_TARGET_LEMMA.md`: exact
  selector-target zero row, ordinary linear diagonal factor and nonsingular
  2-torsion constant diagonal factor.
- `research/prime31_summation_compression_20261122/iteration9/PAIR_SPECTRUM_RESULTANT.md`:
  existing resultant/gcd/Bezout query and separate zero-pair/identity branches.
- `research/prime31_summation_compression_20261122/iteration6/MATHEMATICAL_NOTE.md`:
  rational auxiliary guard, projected ghost example and limits of a fixed
  affine chain or balanced tree.

Sources checked (KB): the parent observed no loaded research-KB MCP and
`scripts/research-kb status` reported no executable. These exact repository
records are the fallback; this inspection is not a literature-wide absence
or novelty claim.

The inspected main tree contains the published two-summand selector extractor.
Its ACTIVE record assigns the three-summand signed extractor to a concurrent
lane. No tracked FASS common-root implementation was found under the inspected
FASS paths. Iteration 9 explicitly describes its pair-spectrum oracle as a
written algorithm without native qualification. New code therefore supplies a
generic quotient certificate component without replacing those ownerships.

## Certificate contract and independent checker

Let F and G be monic, squarefree and split over the stated prime field, and
H the canonical remainder modulo (F,G). Every leaf has a nonconstant monic
right modulus q and polynomials D,F1,H1,A,B in (Fp[t]/q)[x], with D monic in x.
The checker requires

    product q = G,                 gcd(q_i,q_j) = 1 for i != j,
    F = D F1,                     H = D H1,
    A F + B H = D.

The cofactor identities establish common divisibility; Bezout establishes
the opposite gcd divisibility. Monicity then identifies the actual fiber gcd.
All equalities are checked coefficientwise modulo q, without running the
producer's dynamic Euclidean search. Counts and emptiness are recomputed as
sum deg(q) deg_x(D). G=1 is a separate empty partition; F=1 requires H=0.

`../../native/independent.rs` contains separately written u128 residue,
polynomial and tensor arithmetic. It calls no producer field, polynomial,
gcd, norm or Euclidean routine. It checks canonical representatives and
degree bounds, monicity, the prime modulus, and the identities above. To
establish splitness and squarefreeness it checks F divides x^p-x and G divides
t^p-t using its own modular powering and division. This is an identity checker,
not an independently implemented candidate solver. [pcc:unverified]

The optional full-grid checker treats supplied root lists as untrusted:
distinctness, canonicality, root equations, and root count equal to degree
must all hold. It checks routing to exactly one right branch and equality
between the zeros of H and D on every grid point.

The S3 input checker also binds this certificate to the declared curve,
target coordinate and selector. It uses the symmetric expanded S3 expression,
interpolates each right quadratic at 0,1,-1, multiplies those factors, and
independently computes the rational squarefree gcd. It checks the selector
root inventory and the actual left liftability restriction. Finally it
checks H against a direct S3 product on every point of the complete quotient
grid. This independently binds the reduced H; no coefficient-rank hypothesis
is used. [pcc:unverified]

Planned mutation checks include false emptiness and support count, nonmonic
F or branch modulus, noncanonical residues, repeated or nonsplit G, a missing
or duplicated/noncoprime branch, changed Bezout multiplier, changed cofactor,
and changed gcd. Their actual outcomes must be retained after execution.

## Edge cases that must stay explicit

1. A zero polynomial has no invertible leading coefficient. Euclidean degree
   trimming and a branch on a zero divisor must precede any inversion.
2. G=1 is an empty right root set, even though its canonical remainder is
   zero. H=0 with nonempty F,G is instead the full grid.
3. At X in the selector the first norm has a forced zero row. This is a
   valid nonunit case, not failed generic-rank qualification.
4. For nonsingular 2-torsion, the diagonal S3 factor is the nonzero constant
   f'(X)^2. Distinct selector coordinates give repeated quadratics. A zero
   diagonal polynomial requires f(X)=f'(X)=0 and hence a singular curve;
   the elliptic input checker rejects singular curves.
5. A raw repeated right product and its squarefree rational part have
   different dimensions. No multiplicity is reintroduced by a leaf count.
6. Algebraic support is not by itself a signed relation. Nonliftable selector
   coordinates and 2-torsion targets require particular care; right rational
   roots can have anti-rational point interpretations. Signed extraction must
   check all actual point lifts, subgroup conditions and the target sum.
7. A target at infinity has no affine X. A triple P,-P,Q can have an identity
   prefix even when Q is finite. A complete relation API needs those branches
   or a separately proved permutation-complete output contract. An affine
   empty-grid certificate cannot silently certify the omitted branches.
8. Every witness inventory retains collided preimages, sign choices and
   repeated leaves at its declared output scope. Polynomial support has no
   implicit backpointer completeness guarantee.
9. A bounded or failed query is indeterminate; neither a timeout nor a
   rejected certificate can be returned as EMPTY.

## Frozen development panel supplied by the coordinator

The intended panel has 95 queries: 88 S3 curve cases and seven synthetic
split-algebra cases. The ordinary F97 curve uses A=2,B=3, selector degrees
1,2,3,4,6,8,12,16 and four fixed target categories, with both the full and
liftable left selector. The original Prime31 coefficients use d=7,21,63,
X=0,1,2 and both left selectors. The F97 curve A=96,B=0,d=2 uses X=0,1,2 and
both left selectors. Synthetic cases exercise zero/one H, nonunit leading
coefficients, varying fiber degree, and F=1/G=1. Exact inputs and execution
limits are governed by the coordinator's frozen PROTOCOL.md, not this review.

Every certificate is checked in memory. The protocol preselects 27 complete
serialized certificates: seven synthetic, eight ordinary F97 d=2, six
rank-one-curve and six Prime31 d=7 cases. Other certificate objects retain
their digest and size with `persisted=false`; they are reconstructed by the
frozen source and input. This is an evidence-size distinction, not a change
to which cases are checked. No native result is asserted in this note.

## Root repository prerequisites observed without building

The four declared local crates `certified-relations`,
`relation-replay-checker`, `hashed-ecdl-tensor` and `ecdl-tensor-ic` have their
Cargo manifests materialized. The tracked manifests at
`research/pencil_large_prime_20261029`, `tools/factor-base`,
`tools/volcano-cli` and `isogeny_algos` are not materialized in this sparse
checkout. They exist in Git. Further transitive prerequisites have not been
fully audited. This is a checkout setup gap, not an upstream source defect.

The observed toolchain is rustc 1.100.0 and cargo 1.100.0. Lean and Lake were
not found on PATH. No root Cargo build, standalone qualifier build, test call,
formal proof check or research run was performed for this pre-run review.
