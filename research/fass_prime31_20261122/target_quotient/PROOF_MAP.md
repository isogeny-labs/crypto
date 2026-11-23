# Proof map: target-dependent quotient rank

Tracking: crypto PR #4952. Mathematical source: [THEORY.md](THEORY.md), frozen
under unverified claim PCC-20261123-01ad65. The frozen theory is not modified
by this map. Lean and Lake are unavailable in this environment. No declaration
has been compiled, and no universal theorem is replaced by an axiom or an
unchecked proof stub. [pcc:unverified]

The [protocol](PROTOCOL.md) separates universal derivations from bounded
native fixtures and evaluated certificates. Execution outcomes and their
receipts belong in REPORT.md; this table does not infer a completed run from
a prospective check. All mathematical support described here remains at its
declared written or finite-fixture scope. [pcc:unverified]

## Universal obligations and their evidence boundaries

| Obligation | Exact domain and statement | Written support | Remaining formal obligation / finite-check limit |
| --- | --- | --- | --- |
| Canonical first norm | Characteristic p>2d; L=x^d-1; H=Res_z(L(z),S3(x,z,t)) mod L(x); deg_x H<d and deg_t H<=2d-1 | Prior [coupled report](../coupled/REPORT.md), THEORY section 1 | Formal resultant product, monic remainder, and cancellation of the top t coefficient; fixture evaluations do not discharge all d |
| Right-message normalization | Fixed nonempty W subset mu_d; k=|W|; leading coefficient lambda_W(X)=product_(z in W)(X-z)^2; X not in W gives monic R_W of degree 2k | THEORY section 1 | Formal leading-coefficient computation and localization at lambda_W; no normalization on a vanishing leading coefficient |
| Remainder denominator bound | Entries of K_W=H mod R_W have denominator dividing lambda_W^(2d-2k); d-by-2k coefficient matrix | THEORY section 2, monic long-division argument | Formal degree descent and denominator induction; a single nonzero minor only after denominators are cleared |
| Specialized rectangular minor | p>2d; r=min(d,2k); rows {0,d-r+1,...,d-1}, columns 0,...,r-1; value at A=B=X=0 is (-1)^(r-1) product_(j<r) binom(2d,j) | THEORY equations (4)-(7), inherited cusp coefficient identity | Formal specialization, truncation modulo t^(2k), row permutation, sign and nonzero binomial factors; upper-left r rows are generally the wrong minor |
| Generic fixed-W rank | Rank of the canonical K_W coefficient matrix over F(A,B,X) is min(d,2k) | Nonzero cleared minor plus dimension bound, THEORY proposition 1 | Formal matrix rank specialization and rational-function nonvanishing; a fixed evaluation certificate is not this universal statement |
| Full right message changes nothing | W=V; X^d!=1; deg R_V=2d>deg_t H, so H mod R_V=H | THEORY equation (8) | Formal canonical-remainder uniqueness; no assertion about the smaller rational squarefree quotient follows |
| Target incidence restriction | Substitute B=Y^2-X^3-AX; the map F[A,B,X] to F[A,X,Y] is injective | Highest-even-Y-power argument, THEORY section 3 | Formal polynomial substitution injectivity and localization; generic nonvanishing is not a fixed-curve rational-target guarantee |
| Nonsingular and squarefree generic locus | p>3; nonzero rank minor can coexist with nonsingularity, Y!=0, X not in W, and squarefree R_W | THEORY sections 3 and 5 | Formal nonempty open intersection; the singular nonreduced witness is only a nonzero-minor certificate |
| Quadratic discriminant | disc_t S3(t,z,X)=16(X^3+AX+B)(z^3+Az+B) | Explicit coefficient identity, THEORY equation (10) | Universal symbolic identity, including specialization; finite evaluations cannot certify all parameters without a stated proof method |
| Rational-part bridge | Nonsingular E/F_p, p>3, finite Q=(X,Y), Y!=0, X not in V, no selector 2-torsion; gcd(P_V,t^p-t)=rad(R_(W_E)) | Discriminant square/nonsquare classification, THEORY section 4 | Formal ground-field root criterion, radical/gcd equality and monic normalization; collision-free equality with R_(W_E) needs the additional hypothesis |
| Fiber-collision exceptions | Distinct selector coordinates z,w collide only through 2Q=-epsilon P_z-eta P_w, after excluding infinities and 2-torsion | x-coordinate equality up to sign and doubling formula, THEORY section 5 | Formal group-law branches and finite exceptional set; no scalar-recovery algorithm is supplied |
| Selector-target zero evaluation row | X in V; H(X,t)=P_V(t); G_X divides P_V, hence (H mod G_X)(X,t)=0; for e=deg G_X, coefficient/value rank <=min(d-1,e) | Separate audit: [SELECTOR_TARGET_LEMMA.md](evidence/independent/SELECTOR_TARGET_LEMMA.md) | Formal evaluation through x remainder, S3 symmetry, divisor remainder and the nonzero evaluation functional; this is an evaluation row, not necessarily a monomial coefficient row |
| Selector-target bound on liftable rows | If the target coordinate is retained among B liftable left rows, their evaluation rank <=min(B-1,e) | Same zero-row argument | Formal row restriction and target retention; B counts rows, not signed points |
| Coefficient rank equals value-grid rank | L and the selected right modulus are squarefree and split; use all their distinct roots and canonical degree bounds | Invertible Vandermonde maps, THEORY equation (15) | Formal coefficient/evaluation matrix factorization and invertibility; does not hold for a confluent nonreduced evaluation grid without modification |
| Zero-support rank is a different observable | Form Z_(alpha,tau)=1 when K(alpha,tau)=0, otherwise 0; rank over F_p | Entrywise nonlinear indicator definition in the protocol | No universal equality with coefficient rank is asserted; each reported support rank requires its own retained fixture output |

## What a finite fixture or certificate can support

A native qualifier can establish exact equality for its recorded fields,
curves, selector degrees, targets and polynomial representations. In
particular, it can compare monic division with a separate reconstruction,
the rational gcd with a union of quadratic roots, the coefficient rank with
the evaluation-grid rank, and the chosen rectangular minor with its expected
specialization. A low observed rank is retained rather than rejected merely
because the generic bound is larger. [pcc:unverified]

An evaluated identity certificate is bound to its displayed expression,
field, degree, source and evaluation points. A record with `proof:null`, even
with fresh-point replay and tamper rejection, does not prove proposition 1
for every d or W. A same-host replay is separate-role reproduction; it is
not a signed execution verdict or a Lean kernel result. [pcc:unverified]

The singular specialization R_W=t^(2k) and its radical t must remain distinct.
Their quotient dimensions differ. Rank observed after replacing the former
by its radical cannot be substituted into the nonreduced generic-minor
certificate. Likewise, a Boolean support matrix is produced by an entrywise
zero test, not by an invertible basis change.

## Formalization order

1. Import or build the prior quadratic norm and cusp coefficient identities
   in the intended polynomial-ring and finite-characteristic setting.
2. Formalize monic normalization, the localized remainder map and its
   denominator bound.
3. Establish the rectangular specialized minor with the exact row set, then
   the generic fixed-W rank statement.
4. Prove the target-incidence injection and generic-locus qualifications.
5. Formalize the discriminant, ground-field gcd bridge, and exceptional
   group-law branches separately.
6. Prove the selector-target zero-row lemma and the two row-count bounds.
7. Formalize the split evaluation-basis isomorphisms. Keep zero-support
   indicators outside the linear rank-preservation statement.

Before promoting any item, record the exact Lean toolchain, mathlib and
crypto-lean revisions, complete build output, declared theorem statement and
full axiom dependencies. Unavailable executables are a formalization blocker,
not evidence that the mathematical statements are false or formally proved.

## Excluded conclusions

- No universal rank claim for every fixed curve and rational target.
- No sampling bound conditioned on a liftability pattern, fixed A, ordinary
  curves, group order or prime-order subgroup selection.
- No generic min(d,e) formula for every collided rational squarefree quotient.
- No rank equality between canonical coefficients and Boolean zero support.
- No runtime lower bound from a dense coefficient dimension or full rank.
- No signed relation, independent relation yield, or complete ECDLP recovery
  claim from a rank-only qualifier.

The report must cite actual retained receipts for its execution classification.
This map supplies the universal proof obligations and the boundaries of the
finite evidence. [pcc:unverified]
