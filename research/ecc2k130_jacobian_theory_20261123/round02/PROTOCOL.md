# Exact local-order proof check

Question: classify residue collisions of pi=alpha*zeta for the260 embeddings
of K, at the two prime divisors263 and ell=146505763881528721 of U131.
No curve search, timings, DLP solve or cost estimate is part of this check.

Use C++17 exact arithmetic. At263 enumerate both roots of X²+X+2 and every
nontrivial131st root. Atell use the quadratic residue-field construction of
../check.cpp, enumerate alpha*z^j and beta*z^j, j=1..130, and classify equal
values and their ell-Frobenius orbits. Retain all collision indices.

Predictions:263 has129 double values and2 single values; ell has one real
double value,64 quadratic orbits of double values, and one quadratic orbit
of single values. The full polynomial discriminants have only simple
pairwise collisions at either prime, since v_p(U131)=1.

Controls: all260 labelled embeddings included; every z^j distinct; beta
and alpha polynomial identities; Frobenius swaps alpha/beta atell and
inverts z; every collision class has size at most2. Checks are producer
arithmetic evidence only; number-field/order/polarization bridges require
written proof and independent review. Stop the check on any disagreement.

Artifact: local_check.cpp, local_check.txt; fixed no-randomness input.
Scope: arithmetic replay for a proposed proof, no measured performance claim.
