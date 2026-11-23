# Frobenius small-prime screen qualified

Pushed freeze1604877628b, single invocation exits0. All75prime/model checks
and5integer norm/character/Cayley-Hamilton/reciprocity/order/pointcountchecks
pass. Whole Frobenius formula remains conditional on existing character
lemma; these checks are not a wholly independent proof of that formula.

Powerbasis indices [Z[zeta]:Z[pi]]:

|g|p|c|index|
|---|---|---|---|
|2|101|2|7171|
|2|251|18|9151711|
|2|601|7|4582625|
|3|113|3|575889585954787|
|3|421|2|469521548813867549|

Among primes<=47only three index-degeneracy rows occur:

|g|p|l|index valuation|Frobenius algebra rank|distinct factor degrees|dimension obstruction|
|---|---|---|---|---|---|---|
|2|251|19|2|2|2|none from degree gcd|
|2|601|5|3|2|1|none from degree gcd|
|3|113|13|2|4|2,2|3-dimensional kernel impossible|

No scalar Frobenius modlflag occurs. Passing genus2cases do not construct
kernels, identify isotropy/quotient class or establish a speedup. They are
admission points for separate polarized-kernel analysis, degree19²and5².

For each original genus3model, only l=2,7,11,23,29,37,43pass degree-gcd
screen. At2/11/23/37factor degrees are3,3,algebra rank6. At29/43sixdistinct
linear factors,rank6. At ramified7,rank6andindexvaluation0despite repeated
characteristic polynomial; pi generates the whole truncated cyclotomic
algebra, so repeated polynomial alone does not produce extra kernels.
Everyotherprime<=47has factor-degree gcd notdividing3.

Next: explicit norm-lendomorphisms may exhaust genus3kernels in those seven
cases. Prove polarized quotient isomorphism, not merely abelian isogeny.
Regular primes are not globally declared selfisogenies by this screen;
bounded exhaustion requires separate witnesses/count argument. Genus2
admission,genus4 and original10xobjective remain open. No new targets,
transport formulas, novelty or runtime gain established here.
