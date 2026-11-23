# Fixed small-prime Frobenius/kernel-admission screen

Five unchanged qualified models:g2(101,2,7481),(251,18,67931),
(601,7,411491);g3(113,3,1247429),(421,2,77152909).
Use existing binomial Frobenius polynomial P(sigma) from character formula,
same canonical orientation. No new source/target or timing experiment.

Construct integer multiplication matrix MofP(zeta_n) on cyclotomic basis,
n5/7,d=n-1. Check M*Mbar=qI. Checked-i128 Faddeev-LeVerrier computes
charpoly with exact trace divisions and final Cayley-Hamiltonzero. Require
reciprocity,constantq^g,ordinarymiddlecoefficientnonzero modq,F(1)=existingell,
andtrace agrees with direct #C(Fq) from canonical point catalogue. This adds
checks, not independent proof of full Frobenius formula; its previous stated
CM/character hypotheses remain in force.

Powerbasis columns1,pi,...,pi^(d-1) form integerK. CheckedBareissdet gives
nonzero index I=[Z[zeta_n]:Z[pi]]. For all15primes l<=47,75rows total,
record v_l(I),rankKmodl,gcd(F,F')degree,distinct irreducible factor degrees,
and scalarMmodl flag. Exact finite-field arithmetic includesl2 without using
odd-prime-nativeFp. Factor degrees via gcd(F,X^(l^k)-X),k1..d,subtract proper
divisor contributions; no root/split heuristic.

Require repeated factors iff l|Ior l=n (cyclotomic discriminant support).
Rank deficiency iff l|I. Away from ramifiedn,rank equals degree of distinct
factors since multiplication in Z[zeta]/l is semisimple. Scalar flags require
M=sI ands²=qmodl. Reproduce priorl2dimension results:g2obstructed,g3not.

Dimension obstruction:gcd of distinct irreducible degrees must divideg for
any invariant g-dimensional subspace. If not, no rational(l,...,l)kernel
can exist. Passing is necessary only, not isotropy, quotient construction,
newpolarizedisomorphismclass,nonhyperelliptic transport or speed evidence.
Rational kernel means Frobenius-stable, not pointwise rational torsion.
Rankdeficiency highlights primes permitting non-cyclotomic stable modules;
it is not a complete list of transport degrees. Regular primes are NOT
declared allselfisogenies. Ramifiednretained separately,notexcluded.

Primary context: Smith genus3isogenies https://arxiv.org/html/0806.2995 ;
Costa etal ordinary graphs/polarizations sections1-2and7
https://arxiv.org/html/2601.20979v1 . KBsearchreturned prior rational-isogeny
notes,not independent computation or absence/priority evidence. This gate is
an algebraic admission screen with no novelty/10x/genus4solverclaim.

Freeze source/protocol/native/import/order-evidence/binary hashes,push before
one120CPU-second invocation. Gitraw/exit/hash/receipt,no overwrite,
scientificretry/S3/cloud. No field/kernel substitution after outcomes.
