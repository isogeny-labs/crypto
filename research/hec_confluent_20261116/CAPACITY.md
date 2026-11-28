# Exact signed-profile capacity, derived after the rank screen

Assume H distinct NONBRANCH rational anchor abscissae, no support contains both conjugate orientations of the same point, and total input multiplicity M>0. A profile has support size r, with positive multiplicities summing to M. Choose support in C(H,r)ways, multiplicities in C(M-1,r-1)ways, and orientations modulo simultaneous hyperelliptic conjugation in2^(r-1)ways. Therefore

    F_all(H,M) = sum_(r=1..min(H,M)) C(H,r) C(M-1,r-1) 2^(r-1),
    F_distinct(H,M) = C(H,M) 2^(M-1)  (H>=M).

Equivalently, F_all is half the coefficient of z^M in ((1+z)/(1-z))^H. This counts signed interpolation INPUTPROFILES, not distinct primitive functions or independent principal relations. One primitive function can satisfy multiple profiles, common factors can erase conditions, and independent functions can still give dependent rows. Dedup/rank checks cannot be replaced by this formula. Branchanchorshaveoneorientationandrequire a modified generatingfunction; they are excludedfromthiscountpanel, withseparatealgebracontrols.

The fully exhausted low-pole families provide exact consistency checks: H8,M3has F_distinct=224andF_all=344; H9,M4has1008and2241. These equal the recorded constructed counts. They are post-run combinatorial checks, not newly registered selection gates or speed ratios.

ForfixedM andH→infinity, ther=Mtermisthedistinctfamily. Ther=M-1termrelative to it is M(M-1)/(2(H-M+1)); all lower r contributeO(H^-2). Thus

    F_all/F_distinct = 1 + M(M-1)/(2H) + O(H^-2).

Merely expanding the full bounded-multiplicity family therefore changes profilecapacityby a vanishing relativefraction at largeH; it does not byitselfsupply a newleadingexponent. This is NOTa lowerbound on all repeated-support algorithms: a deliberately selected repeated-support subfamily could have a different useful-relation distribution, and a different growingMregimehasdifferentconstruction/polecosts. Such an algorithm would need a constructive yield/cost proof. The current finite rank screen gives no tenfold premise for the tested indiscriminate mixed-multiset policy.
