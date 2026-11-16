# C115: common-prefix translation degree constraints on the w=1branch

Let H(T)=T^5+h4*T^4+h3*T^3+h2*T^2+h1*T+h0 overF2[X], with five roots in L_d={polynomials of degree<d}, multiplicities retained. Put c=X^(d-1). The pinned h4 has leading bit1, so the number k of roots with that leading bit is odd:1,3or5. The top coefficient w=[X^(4(d-1))]h1 equals binomial(k,4)mod2. Hence w=1forces k=5, including repetitions. This statement applies to TRUE prefix-supported roots; it is not inferred for every relaxed parameter.

On this branch translating T->T+c shifts all roots into L_(d-1). In characteristic2the exact translated coefficients are:
 h4'=h4+c;
 h3'=h3;
 h2'=h2+c*h3;
 h1'=h1+c^2*h3+c^4;
 h0'=h0+c*h1+c^2*h2+c^3*h3+c^4*h4+c^5.
Every h_j' must have degree <=(5-j)*(d-2). For d=18the bounds on h0'..h4'are80,64,48,32,16. These are linear conditions on the existing coefficient bits, because c is fixed. Use RAW polynomial h0=HC0+M*q with the SAMEthree old global q bits; reducing moduloM before applying the degree bounds is unsound.

Hypothesis: these degree constraints contract the current w=1chart beyond its67dimensions, permitting a simpler exact conditional search. They do not apply wholesale to w=0or discard other charts. Translation preserves repeated-root multiplicities and is invertible; no positive branch is omitted. They are necessary, not sufficient, for full prefix support.

Next experiments: (1)independent exhaustive small-dfive-root multiset products with repeated/zero roots, checking the k/wimplication, translated polynomial identity and bounds against direct shifted-root multiplication; (2)build all actual linear bit equations with original quotient lifts, intersect only C111chart1, and report exact rank/free dimension plus positive planted controls. Freeze matrix/maps/source and field convention first;90s4096MiB1core. Advancement criterion: at least4additional independent equations on that branch, otherwise preserve the no-cut result and investigate a separately proved coprime leading-stratum/Hensel factorization instead. No new native timeout before useful measured cuts or a changed exact representation.

Translation, degree bounds and Hensel lifting are standard ideas, not a novelty claim. Prime-field interval domains require separate carry/lift proofs; binary polynomial-prefix translation is not imported unchanged into the NIST prime fields. No50xpromise follows from conditional dimension reduction.
