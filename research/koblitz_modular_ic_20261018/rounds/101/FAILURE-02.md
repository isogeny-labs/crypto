# R101 attempt 02: actual derivative degeneration rejected by protocol

The source-bound `fiber101-02` preparation passed the donor hash, additive
exponent grid, and `f51(1,y)=y^7+y^3`, then exited 1 with
`ValueError: lambda derivative vanished identically`. This was a protocol
assumption, not a malformed source: the l-linear coefficient of the actual
`G51` is zero after specialization at x=1. No Singular process launched.
Amendment 03 requires this vanishing and checks the specialized derivative
in Singular before factoring the two base-point l-polynomials.
