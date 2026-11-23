# A concrete sufficient initial-elimination question

2026-10-30 UTC. Proposed proof obligation, not a solved lemma or new algorithm
claim. The chosen bet remains norm-function coefficients with shared radical
quotient/remainder membership. Capped Macaulay elimination is a standard
primitive; a structural cost/yield theorem for these systems is missing.

The implicit auxiliary circuit has v=O(mt) variables, polynomially many
quadratic coefficient equations and polynomial-bit coefficients. At balanced
m near sqrt(n)/2 and t=log2 d, v=O(n) and m+t=O(sqrt(n)+log n).
Enough positive targets on each simultaneous good base have quotient degree
at most L=128m32^m, by the reviewed moment argument.

A concrete sufficient lemma would give an initial Groebner basis, for a
specified degree-compatible order, within the Macaulay row space of
certificate degree D=O(m+t) on a proved sufficiently large subset of those
positive targets. Certificate degree bounds the multiplied input rows
needed, with D>=2. It is not final printed degree, first fall degree, or the
last productive degree before timeout. The subset's yield and the constant
must have effective bounds; recognizing the subset in advance is not assumed.
For example a proved subset of probability at least rho0/4 would suffice.

The monomial count through degree D is

    M=binom(v+D,D), log M=O(sqrt(n)log n).

All multiplied quadratic input rows through D can be generated and reduced
in M^O(1) field work. If this row space contains a complete basis, row
reduction and Buchberger verification identify it with charged work of the
same sub-exponential class; a failed check returns unknown. Rows retain
input-ideal provenance and include all original generators, preventing a
partial row space from becoming a false refutation. Verification products
stay within another constant multiple of D and the same exponent class.

After certifying the basis, enumerate at most L+1 standard monomials and
return unknown if the quotient exceeds L. On the small-quotient targets,
subsequent polynomial-in-L conversion and finite-field extraction are
sub-exponential too. Cap every phase, verify returned relations and use the
selective contract. A proved inverse yield exp(o(n)) gives an exp(o(n))
total. To retain the sharper exp(O(sqrt(n)log n)) total, the inverse yield
must obey that sharper class as well; rho0/4 does. Fast refutation on all
targets is not required.

No such lemma is proved. A single-point radical ideal can still require
expensive elimination. Finite etaleness and small L do not bound the work.
The unbound F17 library check's final printed degree3 is not a measurement
of this certificate degree. Historical productive degree8 work and contended
timeouts remain the scoped evidence in the initial audit; they neither
prove nor refute a growing-arity O(m+t) theorem.

The next discriminating work is an invariant establishing the row-space
bound, or a complete natural-positive counterexample to a precise proposed
bound. Isolated empirical profiles can guide that search and reject wrong
conjectures; they do not prove the missing complexity theorem.
