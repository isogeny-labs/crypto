# Frozen F3125 opposite-pair quotient trace screen

16 November 2026. Complete the rational-base characteristic-five arm of
the cyclic-11 four-branch opposite-pair screen. Enumerate all 270 ordered
nonzero inertia rows `(a,b,c,d)` summing to zero modulo 11 and splitting
into two opposite pairs. For each `lambda=2,3,4` in F5, use the exact
carrier `y^11=x^a(x-1)^b(x-lambda)^c`.

Select a monic irreducible `z^5+c1*z+c0` over F5 by checking no monic
factor of degree one or two; verify all 3124 nonzero elements have order
dividing 3124. Precompute order-11 character logs for each nonbranch
`x` and count `N` where the exponent-weighted log sum is zero. This gives
`#C(F3125)=4+11N`. For an F5-rational involution quotient Q, the
Frobenius-orbit formula predicts `#Q=(#C+3126)/2`, so the putative
elliptic trace is `t=(3122-11N)/10`. Check integrality and Hasse bound.
Classify each trace as ordinary (`5` does not divide `t`) or
nonordinary, and check whether it is a fifth-degree trace of an F5
elliptic class (`s in -4..4`). Report every distinct row class and its
exact `(lambda,a,b,c,d)`. Do not equate a trace hit with a subgroup map.

The quotient point-count relation is an external paper argument pending
an independent smooth quotient replay. Freeze this protocol and native
Rust source in a commit before execution. Keep compiler, source hash,
stdout and stderr, including failures. Any correction is an amendment.
