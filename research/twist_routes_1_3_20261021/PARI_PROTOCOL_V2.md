# Equal-original-height lift control, version 2

Keep V1 intact. Repeat its exact 254 integer-twist parameters, rank effort zero,
source models, point signs, triple-relation enumeration and finite-field replay
in a fresh directory, now with naive **original** x height at most 16.

For the standard twist Y²=X³+d²AX+d³B, enumerate standard-model x height at most
16|d| and filter by max(|numerator(X/d)|,denominator(X/d))<=16. This enclosing
search is sufficient because x=X/d and height(d*x)<=|d|height(x). Retain both
the enclosing standard-model bound and the actual original-model bound. No
comparison may silently interchange the two height conventions.

Use the same native PARI 2.17.2, 15-second hard limit per GP process, 4 GiB
enclosing process limit and 90-minute outer limit as V1. Independent selected
cover characteristic polynomials are repeated. The executable is the separate
`twist-route-supplement` binary, so the running solver-panel executable is not
replaced. Its source, this protocol and input identities are frozen before run.

Use the identical projected signed base for the ordinary Fp relation/rank
control. Rank bounds still do not certify a Mordell-Weil basis. General target
lifting, scalable height search, discovery cost and a full IC improvement remain
open even if the equal-height search finds more points or relations.
