# Independent theory review — 2026-11-15 v10

Verdict: APPROVE the packet as a fixed, exposed next-experiment design and hand-derived conditional norm positive. No scientific algorithm/oracle was executed by this reviewer. This is not admission of a global original-root solver, novelty, hardness, or a measured speedup. The entire THEORY.md, PRIMARY_SOURCES.md, NEXT_EXPERIMENT.json and SOURCE_LOG.json were read before pinning.

## Independent GF9 arithmetic and signs

Work in GF3[t]/(t²+1); t²=2. The polynomial is irreducible because 2 is not a square in GF3. C(2)=1, C(1+t)=2 and C(1)=2, so A=(2,1), B=(1+t,t), T=(1,-t) are curve points. C'=1 makes Y²=X³+X nonsingular in characteristic3. Target x=1 is outside {2,1+t}.

For a=1-t,b=2+t, one has a²=t, ab=2t,b²=t. Thus P²-CG² has descending coefficients [1,t,2+t,1+t,t,2+t,t]. Independently, (X-2)³=X³+1 and (X-(1+t))²=X²+(1+t)X+2t. Multiplying the latter by X-1 gives X³+tX²+(2+t)X+t; multiplying by X³+1 reproduces exactly those seven norm coefficients. This verifies the claimed norm factorization by hand, without a search.

At A,B,T respectively, G takes values 2+2t,1+t,2+t, all nonzero. P=-Gy holds at each selected ordinate, while P-Gy=-2Gy is nonzero. Since all selected y are nonzero, x-x(point) is a local parameter and the norm multiplicities transfer to the selected zeros of P+GY: 3 at A,2 at B,1 at T. No opposite-ordinate zero is silently included in this selected function. The cubic polynomial P has pole order6 at infinity, exceeding the order5 of GY, so no leading-pole cancellation occurs.

The regular derivative check uses N'=2PP'-C'G²-2CGh. At a=2,ya=1, G(a)!=0, the condition reduces to 2ya P'(a)+C'(a)G(a)+2ya²h=0. Here P'(a)=2+t and the sum (1+2t)+(2+2t)+2t is zero. The chart is regular; the complementary G(a)=0 or ya=0 branches remain separate.

The source-sign convention is crucial and is correct in the final packet: doubling gives 2A=(0,0), 2B=(t,0), 3A=-A, and 3A+2B=(1,t)=-T. Hence T is the norm divisor zero and Q=-T is the source-sum target. There are ten distinct abscissa orders of the bag a³b². That ten is not an unrestricted ordinate/auxiliary root count. Indeed the same Q also permits A signs all positive or one positive/two negative, and B signs both positive or both negative; these give eight rational point-sign choices per abscissa ordering. This is a hand sign analysis only, not an emitted original-root set. No global original count should be inferred.

The tangent-line construction is consistent: l1,l2 are tangent at A,B; l3 passes through A,T. If their non-Y parts are alpha,beta,gamma, then alpha*beta*gamma=(t-1)C and alpha+beta+gamma=tX-t. Reducing their product divided by Y therefore cancels the denominator in the function field, giving the stated polynomial P+(tX-1)Y rather than an unresolved pole at Y=0.

## Independent GF25 translation check

In GF5[t]/(t²+2), t²=3 is nonsquare over GF5. Substitution u=X-t gives C(u)=X³+2tX²+(1+t) and P(u)=X³+(4+2t)X²+(1+2t)X, as claimed. The old base norm is independently confirmed by expansion: P²-4C=X6+3X5+3X3+3X2+3X=X(X-3)³(X-4)². Translating preserves the displayed new factorization and the target/source separation. At the selected old source3, y=4 is on the curve and P(3)+2*4=0; translation preserves this. The nonzero C2 coefficient must remain an admitted model feature. Encodings C=[6,0,10,1], r=14,s=11,constant0 and domain[8,9] agree with base5 digits.

## Scope and literature

The binary AS identity, the distinction between pth Frobenius and odd squaring, absence of proper nonzero Fp-subspaces of Fp, and guarded affine coordinate substitution are correct field-specific boundaries. An invertible coordinate change preserves the surviving relaxation; it does not supply additional pruning. Constant-functional counterexamples do not refute parameter-dependent polynomial multipliers, but sample consistency also never proves a global identity.

This reviewer directly opened the four cited primary sources. Wu–Liu introduction equations1–2 and matrix discussion support the established linearized-map machinery. Petit–Kosters–Messeng introduction supports composed rational-map factor bases and the motivation for dedicated solving; it expressly does not establish current cryptographic practicality. Hader et al. §1 states the inspected2024 implementation's prime-order restriction, not a current-software limitation. Berlekamp's full short author page describes Frobenius fixed-space/gcd factoring. The packet's conservative overlap assessment is appropriate; this is not an exhaustive priority search.

The proposed GF9/GF25 control must compare full regular parameter sets under literal general-cubic norms and arbitrary two-point domains, preserving exceptional branches as UNKNOWN. R³=0 modulo a cubic is an exact support test for a squarefree domain polynomial, including repeated roots; it is not universally an Fp-linear computation. The <=20000 raw-state cap accommodates 9³+25³=16354 declared states, but setup, support, permutations, verification and output must also fit the independent90-second/4GiB/one-core budget. These fixtures are exposed constructive training controls; no hardness or timing comparison follows.
