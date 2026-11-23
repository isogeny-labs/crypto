# Actual CM19 source: hypotheses for an abstract polarized bridge

Task tracked by ACTIVE-onlyf6d3fbc1 and draft5019 before this new source.
The actual smooth plane quartic C7 and its counts are frozen at source
0b9b506b/result89d7b471, with complete source-qualification report2ce0740c.
This note distinguishes a literature-backed abstract existence argument
from an evaluated coordinate isomorphism. All claims unsigned.
[pcc:unverified]

## Primary dependencies and hypotheses checked independently

Ritzenthaler2010, DOI10.1112/S1461157009000576, Section3.2,
Propositions3.5/3.6, Table2 and Section4.4 explicitly use an ordinary
elliptic curve with End(E)=Z[pi]=O_K and O_K principal. Under those
hypotheses, an abelian variety isogenous to E^g is isomorphic to E^g;
principal polarizations correspond to positive unimodular Hermitian forms,
and absolute indecomposability agrees with lattice indecomposability.
Section4.4 cites the classification: the O_-19 rank3 indecomposable
unimodular class is unique. These general equivalence/classification
theorems are explicit literature dependencies, not independently re-proved
or compiled Mathlib results in this session. [pcc:unverified]

For E7:y^2=x^3+2x+6, independently counted #E(F7)=11 gives trace-3.
It is ordinary since7 does not divide3. Its Frobenius polynomial is
T^2+3T+7, with fundamental discriminant-19. Thus Z[pi] is already
the maximal order, so no larger elliptic endomorphism order is possible.
Set omega=pi+2; then omega^2-omega+5=0, trace1 and norm5.
Every element of O=Z[omega]=Z[pi] is an F7-defined endomorphism.
[pcc:unverified]

The class-number hypothesis has an elementary check. Minkowski's imaginary
quadratic bound is(2/pi)*sqrt(19)<3. Every ideal class therefore has an
integral ideal of norm1 or2. But T^2-T+5 reduces to irreducible
T^2+T+1 modulo2, so2 is inert and no ideal has norm2. Every ideal
class is principal. This is a written argument using the standard
Minkowski theorem, not a native class-number oracle. [pcc:unverified]

The independently qualified smooth C7 has Weil polynomial PE7^3.
The finite-field isogeny theorem and the cited principal-order equivalence
therefore give an abstract F7 isomorphism J(C7)~E7^3. Its canonical
principal polarization is absolutely indecomposable because C7 is a smooth
connected curve. Using the cited uniqueness classification, some O-basis
identifies that polarization with

```text
H = [[ 2, 1, -1 ],
     [ 1, 3, omega-2 ],
     [ -1, conjugate(omega)-2, 3 ]].
```

No coordinate maps for this basis/isomorphism alpha have been constructed.
The cited classification is not silently replaced by point counts. The
new native unit will independently check H's Hermitian property, determinant,
positive minors, inverse/trace Gram matrix and absence of norm1 vectors,
but it does not establish uniqueness of all Hermitian classes.
[pcc:unverified]

## What an integral four-similitude would imply

For an actual integral S satisfying conjugate(S)^T H S=4H, all matrix
entries are F7-defined endomorphisms of E7. The equality states that its
pullback polarization is4H. Its degree is Norm(det S)=4^3=64 and
its kernel is separable because7 does not divide64. Under the abstract
polarized alpha above, alpha^-1 S alpha is an actual Jacobian endomorphism
in an existence statement. Efficiently evaluating it on actual divisors
still requires constructing alpha or an independent correspondence.
An existential matrix is not yet a computational DLP transfer.
[pcc:unverified]

The six-dimensional integral multiplication matrix will be classified by
the gcd of every k-by-k minor, k=1..6, independently of a mod2 rank.
The sought Smith factors[1,1,2,2,4,4] give kernel(4,4,2,2), while
[2,2,2,2,2,2] is the multiplication[2] kernel. Keep all scalar/kernel
outcomes; do not predeclare a mixed example. [pcc:unverified]

For any separable degree64 isogeny, restriction to11-torsion is injective.
The existing degree2 curve cover's Jacobian pullback is also injective on
11-torsion because pushforward-pullback=[2]. These are mathematical
subgroup statements. No explicit composed map, cryptographic solve or
advantage over optimized ECDLP has been implemented. On rational E7^3,
omega=pi+2 acts as3 modulo11; evaluate accepted matrices at omega=3,
and at the conjugate root9, to retain concrete invertibility checks.
[pcc:unverified]

The source's J[2] characteristic polynomial Phi3^3 has only even-degree
simple factors. Every Frobenius-stable subspace has even dimension;
there is no stable dimension3 Lagrangian. Thus no rational(2,2,2)
isogeny can be the first step from this source, even if a mixed4-map
exists. This is a written module argument, not a universal impossibility
of other transfers, extension-field chains or computational reductions.
[pcc:unverified]

Sources checked (KB): KB/Conductor unavailable; locked-vault fallback not
entered. Exact primary hypotheses/table reviewed and prior native source
records reused without executing their population. No novelty clearance.
Generic Mathlib, actual J4/correspondence/target descent, independent replay,
durable original ELF custody and complete strongest-reference cold costs
remain open. Existing paid/vault/CI/merge requirements persist.
[pcc:unverified]
