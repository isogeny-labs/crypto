# Confluent principal relations: construction and cancellation

Prior art: D'Andrea/Krick/Szanto, https://arxiv.org/html/1211.6895v3 , sections2and4,theorem4.2. Rational Hermite/osculatory interpolation, confluent matrices, Euclidean reconstruction and denominator nonvanishing are established. This application does not imply originality or speed. Previous distinct-anchor/rational/primitive studies in ../hec_anchor_relations_20261114are baseline; the repeated-base theta-divisor correction is a different construction.

For a smooth odd-degree hyperelliptic C:y^2=f(x), characteristicodd, nonbranch pointP=(xi,yi), use t=x-xi. The local seriesY(t) is determined byY0=yiandY^2=f(xi+t):Yn=(fn-sum_(j=1..n-1)YjY_(n-j))/(2yi). These are ordinary power/Hasse coefficients, with no factorial division, so small odd characteristic is allowed. At a branchpoint use t=y, compute X(t) from f(X)=t^2 withX0=xi using f'(xi)!=0. ThenordP(x-xi)=2. Repeated branch conditions cannot be treated as ordinary xderivatives.

Choose supportpointswithmultiplicitiesmi,totalM. For d0:M=g+1,maxdega=g,maxdegb=0; for d>=1:M=g+2d,maxdega=g+d,maxdegb=d-1. There areM+1unknowncoefficients. ImposeMhomogeneous local conditionsa(X(t))-b(X(t))*Y(t)=0modt^mi, with signedYatnonbranchpoints. Hence a nonzero solution always exists. The normN=a^2-fb^2is nonzero, has degree<=M+g, and is divisible byA=product(x-xi)^mi: at nonbranch, the conjugate local factor supplies the norm; at a branchpoint valuation in x of the norm equals the local zero order. Exact division and orientations must be checked, not inferred from matrix nullity.

## Primitive cancellation

Let h=gcd(a,b), a=h*a0,b=h*b0. At a NONBRANCHsupportpoint, hi=ord_(x-xi)h removes hi orders from the chosen local factor, so the remaining guaranteed multiplicity is max(mi-hi,0). At a BRANCHpoint it removes2hi local orders, so remaining multiplicity is max(mi-2hi,0). The norm loses2deghdegrees. The total removed guaranteed multiplicity is at most2degh, so primitive residualQ=(a0^2-fb0^2)/product(x-xi)^remaining has degree<=g. At nonbranch-only supports the sharper loss bound<=degh applies. Never keep an invalidated interpolation condition after cancellation. hiatnon-support roots still lowers normdegree. For primitivepairs, b0is nonzero at every normroot (elsea0alsozero); whenb0=0, primitivenormisconstant andthereareno residualroots. At a branchpoint primitive a0-b0y can vanish at most once; multiplicity>=2forces a vertical common factor.

IfQsplits, each residualrootrhas point(r,a0(r)/b0(r)), withnormmultiplicity. Combined with surviving signed supportmultiplicities, theirdegreezero sumisprincipal andmustverifyaszero in the original Jacobian. Branchpoints useone orientation. Normleading cancellation/degreedrop and empty/vertical rows remain explicit.

## Independent nonbranch construction

Build Hermite polynomialHmodAby incremental CRT, matching local Taylor coefficients, then Euclidean rational reconstruction ofa-bH=0modA at the same degreebounds. The first Euclidean remainder ofdegree<=maxdega has denominator degree<=maxdegb. For anytwo solutions, a1*b2-a2*b1is divisiblebyAandhasdegree<=M-1, so itiszero; their primitive rationalpair is unique up to scalar even when the raw nullspace hasdimension>1. Compare canonical primitivepairs, not arbitrary common multiples. Branchmultiplicity>1usesyuniformizerandisnotcoveredbythisunivariateCRTcomparison; it has separate norm/divisor/forcedvertical controls.

These statements establish algebra only. Multiplicity expands profile capacity but may produce mostly common factors, duplicate functions or dependent rows. A subsequent rank/cost gate must count all construction, vertical degeneration and duplicates before any tenfold inference.
