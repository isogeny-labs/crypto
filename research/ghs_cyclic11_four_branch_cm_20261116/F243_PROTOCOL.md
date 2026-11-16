# Frozen F243 quotient trace screen

16 November 2026. Exact curve `C/F3: y^11=x(x-1)/(x+1)` and the proposed
involution quotient `Q` with `u=x+2/x`, `v=y+1/y`.

Use `F243 = F3[z]/(z^5+2z+1)`. Exhaustively verify nonzero inversion
and the multiplicative order 242 for each field element before counting.
For every `x` except 0,1,2, form `f=x(x-1)/(x+1)` and count `N` for which
`f^22=1`, equivalently `f` is an eleventh power in `F243*`.

The algebraic count formula to test is `#C(F243)=4+11N`. For the
involution quotient, the paper Burnside/Frobenius calculation predicts
`#Q(F243)=(#C(F243)+244)/2`, hence elliptic trace
`t=(244-#Q(F243))/5=(240-11N)/10`, if the quotient is the genus-five
curve with the indicated smooth model. Preserve any nonintegral result
as a falsifier. Record whether `3` divides `t`, whether the Hasse bound
holds, and whether `t` is one of the fifth traces from base-field
elliptic traces `-3..3`. These are arithmetic screens, not a subgroup map.

Freeze protocol and native Rust source in a commit before execution.
Keep compiler, source hash, stdout and stderr. No parameter change after
the run; a correction requires an amendment.
