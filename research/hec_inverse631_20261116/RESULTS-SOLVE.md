# F631 algebraic inverse parametrizations

The original frozen solve (3ee7fb6c13e5, checkpoint20261116T023638Z) passes the positive and D1 cases. Both have dimension zero and degree12, with squarefree eliminants and invertible denominators. Independent polynomial substitution verifies every one of the twelve original equations. The positive point is recovered exactly. Numerator convention is630.

The original B solve returns exit1 with an empty parametrization and elimination degree11, recommending a different separating form. The collector stops with exit101. This failure is retained in solve-B.* and SOLVE.*; it is not converted into a successful first attempt.

A separately frozen amendment (0bc845562ea9, checkpoint20261116T023756Z) changes only the final linear equation to t=x1+2y1+4x2+8y2+16x3+32y3. SEPARATION-DIFF.txt confirms all other bytes match. B-alt passes at dimension zero and degree12 with exact substitution into all twelve amended equations, squarefree eliminant and invertible denominator. No positive or D1 solve was rerun. The alternative collector's inherited aggregate PASS text mentions three cases, but that invocation executes only B-alt; SOLVE_VERIFIED label=B-alt and the separate first-run logs are authoritative.

This is exact algebraic parametrization evidence, not yet rational target-class recovery or a global map certificate. Parametrizations may describe triples over extension fields. Next reconstruct rational divisor classes, apply the recorded centering and translation offset, and use public adjacent coordinates to resolve signs. Full destination IC, charged comparison and genus2/4 remain open. No timing or tenfold improvement is claimed.
