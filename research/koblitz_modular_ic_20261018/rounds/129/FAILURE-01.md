# R129 first fixed-point cap failure

The first published source `e044f3da2` used 16 fixed-point updates for a
`u^128` lift. It raised `AssertionError: fixed point did not converge` and
wrote no `result.json`. The local equation contains higher odd powers of s,
so the update need not double precision on every step. Amendment 01 raises
the fixed finite cap to 130, above the retained 128 coefficients. This is a
loop-bound repair; the source equations, input hashes and decision rule are
unchanged. No failed output is treated as accepted data.
