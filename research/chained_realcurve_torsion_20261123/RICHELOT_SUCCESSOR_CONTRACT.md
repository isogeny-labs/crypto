# Actual Richelot descent and subsequent DLP-map boundary

This is a prospective written contract, without a new computation, Lean
theorem or signed verdict. The completed census and genuine J[8] evidence
remain fixed; no new result is inferred from this plan. [pcc:unverified]

## Native source and primary algebra audit

Pinned crypto816f9c3a cd_attack/src/richelot.rs, blob2495980b, implements
delta as the determinant of the descending quadratic coefficients, cyclic
co-Wronskians H_i=G_j*G_k'-G_j'*G_k and f_new=H0*H1*H2/delta. It explicitly
warns against monicizing without a field-defined square-root isomorphism;
otherwise the codomain may be twisted. It supplies a degree2-divisor
resultant pushforward, not a degree1 branch, and assumes G0 is monic
quadratic. Its cancellation path does not assert the numerator remainder.
These are source-audit observations, not newly executed counterexamples or
permission to alter frozen production code. [pcc:unverified]

Earlier Magma Richelot handbook and Bruin--Doerksen/Smith primary audits stay
in force. Native examples/chained_richelot_automorphisms.rs5cd8ced1 concerns
different fixed F101 codomains, so do not replay it as new evidence. Native
transfer_audit/pairing.rs3fed0b29 provides a cost/certificate route and says
generic pairing execution is missing; it does not implement the desired
actual genus-two Weil8 pairing. KB tools remain absent; no novelty/absence
claim beyond these exact sources is made. [pcc:unverified]

## Actual-codomain qualification, before point-group transport

Use only selected C:y^2=x^6+x^5+x^2+1. Work in the genuine branch field
F27=F3[a]/(a^3-a-1), whose cubic has no F3 root. Enumerate all27 elements,
the6 actual branch roots, and all15 pair partitions only to identify the3
Frobenius-stable actual factorizations. No720-permutation or abstract lift
population is repeated. Construct the actual3 quadratic factors, their
determinants and cyclic brackets, preserving every split-product case.
If delta is nonzero, require the entire codomain polynomial to lie in F3
and be squarefree degree5/6. Count F3/F9 points with the correct leading-term
infinities and compare the Weil polynomial with the source; never monicize
an unapproved nonsquare coefficient. [pcc:unverified]

The exact bivariate identity, for this descending determinant convention, is
sum_i G_i(x)*H_i(z) = -delta*(x-z)^2.
Verify every coefficient. On the correspondence
G0(x)*H0(z)+G1(x)*H1(z)=0, the identity implies
f(x)*f_new(z)=[G0(x)*H0(z)*(x-z)]^2.
Thus y*y_new=G0(x)*H0(z)*(x-z) gives an actual curve correspondence, not
just an abstract module action. [pcc:unverified]

For every non-branch affine source point over F27, form its quadratic image
ideal with u(z)=G0(x)*H0(z)+G1(x)*H1(z) and
v(z)=G0(x)*H0(z)*(x-z)/y modulo monic u. Check u divides f_new-v^2.
Record all degree drops and branch/pole exceptions explicitly. Changing the
ordinate rule by omitting x-z must be tested by the same real polynomial
validity check across the entire eligible population, with both accepted
coincidences and rejections retained; require a rejection and restoration.
Changing delta's sign must fail the source Weil-polynomial comparison.
This does not yet implement a total Jacobian homomorphism. [pcc:unverified]

## Subsequent actual odd-prime transfer gate

After codomain qualification, select a rational non-branch source basepoint
P0 and represent a source class D by an effective degree2 divisor in
D+2P0. In the current nonrational Weierstrass chart this can be obtained by
adding2*[P0-W0], then transforming that representative back to original x,y.
The Richelot correspondence sends that effective divisor to4 image points;
subtract2 times the degree2 image of P0. This cancels all basepoint choices
and gives a concrete route to rational divisor-class transport. [pcc:unverified]

Degree drops, poles, zero ordinates, nonexact gcd cancellation and noninvertible
denominators must be handled or declared out of the partial-map domain. Do
not infer a total DLP-preserving map from generic Mumford validity alone.
For a separately frozen tiny qualification, a nonzero rational7-torsion
class in J(F3) has an image of order7 under any genuine degree4 isogeny;
the transfer function must consume only the class, never its scalar label,
and its outputs must match all scalar multiples and Frobenius descent.
Known scalar multiples are controls, not recovered unknown scalars.
[pcc:unverified]

An explicit E->J elliptic-factor map still has to be constructed; an abstract
seven-element lookup is not a geometric ECDLP transfer. Complete cold setup,
maps, destination solver, inversion/replay, memory, field representation and
strongest matched rho/BSGS/IC costs remain mandatory before an advantage claim.
No cost, asymptotic improvement or cryptographic-size result is supplied here.
[pcc:unverified]
