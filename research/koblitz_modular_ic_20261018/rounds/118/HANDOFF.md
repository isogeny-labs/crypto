# R118 handoff

R118 is a frozen, local exact-computation continuation of R117. Its committed protocol and source precede result generation. `unit118.py` reads SHA-pinned R51, R98, R105, R116 and R117 inputs; `verify118.py` uses independent schoolbook arithmetic. `generate_lean118.py` binds four finite Newton-edge claims to `lean/GNewton118.lean`; `publication/render.py` produces the one-page PDF.

All four coefficient rows are exact at order 640; the two new resolved leading coefficients are y0/e3 lambda^0 order `-112`, lambda^1 order `-82`, and y1/e2 lambda^0 order `0`. R117's prior unknowns were conservative precision bounds, not erroneous coefficient values. The next construction is an F4 lambda lift at each repeated initial root, followed by normalized branch valuation and N51 pullback.

The first local producer run failed at a monotonicity assertion because denominator exponent zero needs no unit precision. The corrected source passes all checks. The initial plotting dependency was absent; the committed renderer uses only Python's standard library.
