# G3 implementation and proof map

2026-11-13. This map records the scope of the actual source statements and the
written bridges. All local acceptance below has unsigned standing; it is not a
signed main-branch PCC verdict. [pcc:unverified]

## Exact formal statements

The original Lean4.35 proof gate accepts `Descent.lean` in attempt02d9f9f6:
nine named lemmas and three generated repeatAdd helpers, twelve audited
declarations, zero trusted declarations. Failed attempt01 and its tactic error
remain retained. The same gate accepts the two lemmas in
`ModelTransport.lean`,5cbc69. The original PCC verifier locally accepts all
eleven exact declaration bindings in `evidence/genus3-lean-bindings-01/claims.json`.
Their complete original verdicts and statement digests are retained there.
[pcc:unverified]

| Obligation | Actual statement and claim | Scope |
|---|---|---|
| Completed quartic square | completed_square_quartic,3fc2f3 | Identity in any CommRing |
| Forward finite chart | quartic_to_weierstrass,3ba1c4 | Cleared curve and coordinate identities |
| Inverse finite chart | inverse_finite_branch,b8507f | Requires an inverse of 2x |
| Two x=0 ordinates | zero_abscissa_ordinate_factor,aac37c | Factorization; an integral-domain argument is still needed to split roots |
| Actual finite x=0 branch | actual_finite_zero_branch,97840b | Correct finite t and y give x=0,Y=alpha |
| Unique finite parameter | unique_finite_zero_parameter,557fdf | Requires an inverse of alpha |
| Trace/norm composite | conditional_trace_pullback_composition,15ab8f | Assumes additive norm, diagonal [2], cross terms zero |
| Odd-subgroup injection | conditional_odd_subgroup_injectivity,88dd89 | Generic ell and Bezout 2u+ell*v=1, with composite [2] |
| DLP scalar transport | conditional_dlp_scalar_transfer,ca977c | Any zero-preserving additive map; explicit repeatAdd induction |
| Primitive shift | primitive_shift_square,26cccb | Exact polynomial identity |
| Moore coordinate inverse | triangular_moore_transport_inverse,101c39 | Exact triangular coordinate identity |

The formal files define neither curves nor Picard functors. In particular the
composition hypotheses are not extra empirical assumptions that complete
geometric formalization; the geometric identifications below remain written
mathematics, checked separately against actual source and native fixtures.

## Actual geometry behind the conditional group result

Over an algebraic closure of odd characteristic, the checked smooth Ciani
quartic has sign-change group V4. Quotienting by all V4 gives the smooth
genus-zero conic in the square coordinates; each order-two quotient has
genus one. Write p_i for quotient pullback and n_i for divisor pushforward on
Picard groups. Finite flat degree-two quotient maps give
n_i p_i=[2] and p_i n_i=1+tau_i. These are identities of group morphisms,
not scalar labels used to define the runtime. [pcc:unverified]

For i unequal to j, tau_i induces the nontrivial double-cover involution on
the j-th genus-one quotient. Its quotient has genus zero, so it acts as -1
on Pic^0(E_j), independently of the origin used to write its point model.
Invariance on the i-th quotient gives n_i tau_i=n_i, while
tau_i p_j=-p_j. Thus [2] n_i p_j=0. A homomorphism between abelian varieties
has no integer torsion: its image is connected, and an image killed by [2]
is contained in a finite group scheme; in odd characteristic it must be zero.
Hence n_i p_j=0. No assertion about coprime orders of conjugate elliptic
groups is used. [pcc:unverified]

The full quotient has Pic^0=0, so 1+tau0+tau1+tau2=0 on Jac(C).
Consequently the product pullback P=sum p_i and product norm N=(n_i)
satisfy NP=[2] and PN=[2]. Both are isogenies. Quotient pullback and norm
are adjoint for the canonical principal polarizations, so deg P=deg N;
their composite has degree2^6 and each therefore has degree8. The cyclic
Galois action permutes the three conjugate elliptic factors, descending P
from Res(K/Fq)E0 to Jac(C). [pcc:unverified]

For a source point P0, the descended class is the sum of its three conjugate
completed fibre divisors minus the same sum for O. Applying n0 gives [2]P0;
cross terms vanish as above. The map is additive and injective on each
odd-order subgroup by Bezout. Thus Q=kG transports with the same scalar k
modulo that subgroup order. The finite even-part kernel is allowed and is
reported by the native panel. This does not construct an inverse on the
entire Jacobian. [pcc:unverified]

## Runtime identification and remaining formal bridges

`G3_DESCENT_DERIVATION.md` supplies the completed finite, u0=0 and two
infinity fibre ideals, Cartier product and coefficient-trace descent. The
origin is computed from its actual fibre with multiplicities3,1,1,1.
`G3_MODEL_TRANSPORT_ADDENDUM.md` supplies the explicit historical primitive
basis shift and requires a full quartic polynomial identity before using the
unchanged canonical Picard reducer. `G3_RAMIFICATION_COVERAGE_ADDENDUM.md`
separates rational from geometric branch images and adds doubled-point jets.
These documents preserve the rejected scalar-only model assumption and the
false positive-per-field ramification expectation. [pcc:unverified]

The finite native checks compare the complete source domains to independent
FpR3/FpR6 arithmetic, geometric points and local jets; they exercise the
unchanged original rank, completion and canonical reduction. They do not
formalize ideal sheaf saturation, Riemann--Roch bounds, reduction uniqueness,
norm/pullback functoriality or Galois descent. Those bridges are explicit
remaining formal work. An executable norm on an arbitrary Jacobian class and
an executable destination index-calculus pipeline remain separate tasks.

The old C0/T20 graphs and cost tables are unchanged: this proof map supplies
no timing, solver result or charged pipeline comparison. Source and literature
receipts are in `evidence/genus3-primary-01/sources.json`; the additional
historical builder receipt is in `evidence/genus3-model-correction-01/`.
KB unavailability is recorded in those receipts; no novelty claim is made.
