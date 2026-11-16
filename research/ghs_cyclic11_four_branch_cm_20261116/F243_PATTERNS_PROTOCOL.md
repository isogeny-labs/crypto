# Frozen opposite-pair F243 trace census

16 November 2026. Research screen after the exact `(1,1,10,10)` carrier
gave a supersingular quotient trace. Enumerate all 270 ordered nonzero
inertia rows `(a,b,c,d)` summing to zero modulo 11 whose multiset splits
into two opposite pairs. Rational branch positions are fixed at
`0,1,2,infinity` over F3; the cover is
`y^11=x^a(x-1)^b(x-2)^c`.

For each nonbranch `x in F243=F3[z]/(z^5+2z+1)`, compute the order-11
character of `x`, `x-1`, and `x-2`. Count `N` where their exponent-weighted
sum is zero. Cross-check the `(1,1,10,10)` row against the frozen exact
single-curve result `N=0`. Group rows by sorted inertia multiset; report
whether every ordered realization of a multiset agrees. Derive the
candidate degree-five trace `t=(240-11N)/10`; mark nonintegrality,
Hasse failure, ordinary status and whether the trace is a fifth-power
trace of an elliptic class over F3. List all distinct candidate traces
and sorted inertia patterns.

This census alone does not prove the quotient model or a subgroup map.
Keep source, compiler, hash, stdout and stderr; retain failures. Freeze
protocol and native Rust source in a commit before execution. No parameter
changes after the run without a versioned amendment.
