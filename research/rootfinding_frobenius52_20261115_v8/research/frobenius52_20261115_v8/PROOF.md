# Exact pure-Frobenius U-coordinate reduction

Scope: regular doubled-source norm component, not original all-roots or rank4
coverage. Previous source/row/support proofs remain unchanged. For the fixed
binary curve, target and regular anchor, F=F0+kF1, G=hX+k and
D=(X+z)(X+a)². AtX0, N=t(k)²+B k², D(0)=z a²!=0. Hence
U(k)=[t0²+(t1²+B)k²]/(z a²): betaU=0 IDENTICALLY. Independently the old
six-scalar formula gives r0=z+h(zr1+s1), forcing betaU=r0+hs1+z(1+hr1)=0.
The worker verifies both literal constant expressions for each actual cell.

If alphaU!=0, k=k0+eta sqrt(U), eta=1/sqrt(alphaU), k0=eta sqrt(gammaU).
Frobenius is a field bijection. Using a fixed independent W3 basis, its square
roots are independent and parameterize exactly all k with U(k) inW3.
ActualW3 has dimension52. Only T(k) inW2 remains:48 complementary bit rows
on52 coordinates, versus the strong83-k formulation's atmost79 independent
constraints. Gaussian gives complete coordinate particular/kernel, explicitly
mapped to k. Mutual span and coset replay prove full family equality; coordinate
basis arrays are not falsely equated. Actual rank/nullity are measured.

If alphaU0 and gammaU outsideW3, the coefficient family is empty by that
constant witness. If alphaU0 and gammaUinsideW3, retain the full83-k formulation;
no dimensionbound or division is assumed. Enumerationcap means UNKNOWN, not
empty. SyntheticGF4/8/16 controls exhaust every alpha,gamma,k and both branches,
with independent membership sets. Tiny originalnorm cells additionally compare
allk, literalquotients, supportedproducts, repeatedroots andevery sourceordering.

Compare against the current strongest cached explicit83-k support implementation.
Both arms parse the fullsameactualfixture ONCE per4cell batch, select the same
source-only anchors, construct domainspans/lambdas, solve allfamilies, literally
verify each candidate and pay support/extraction/serialization. The newarm pays
sqrt(W3basis) ONCE perfullbatch and eta/k0 peranchor; their costs are included.
Three alternating SAMEPROCESS repetitions are diagnostic, not cold genuineGB
or qualified isolation comparisons. Stage ratios are not multiplied.

The firsttwo cells are a1 ordinates0/1 (knownorder4 regression). The newanchor
is the smallest rational x in2..255 with4P!=O, selected by field/curve arithmetic
BEFORE any support or comparative cost. Odd-degree halftrace produces one
canonical ordinate; include its inverse y+x as the fourthcell. Doubling gives
x(2P) nonzero iff4P!=O here. Every finitefieldpoint is torsion; this is explicitly
outside the ORDER-FOUR subgroup, not 'nontorsion'. The full scan decisions and
doubling coordinates are retained. Newanchor support positivity is unobserved
until execution, never a selection condition. Allfour cells remain in botharms.

Candidate cap256 perfullbatch, nullity12/cell, tuplecap1000 with120tuple preflight;
any cap/factor failure makes the whole comparison incomplete and ratiosnull.
Full candidate k/Q/R/R4/recovery records are stored once bycontenthash and arm/
rep/cell aliases. Both coordinate representations retain coefficients, coordinate
particular/kernel/rank, kmap and mappedkfamily; deterministic rows can be exactly
rederived fromthese for independent verification, avoiding large repeated pivots.
Raw512KiB/compressed32KiB/stdout16KiB caps protect limited disk. Missingraw data
clears comparisonclaims. Exact artifact: research/frobenius52_20261115_v8/raw.json.gz.
Worker90s/4GiB/onecore, work80s/reportdemote84s/print88s plusparentlifetime90s.
The author ran ASTparse only. All outercharts, exceptional/original auxiliary
branches, odd fields and the requested8x genuineF4/F5/F6 gate remain open.
