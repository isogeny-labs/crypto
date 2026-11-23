# R112: first blowup of the repeated x=0, y=1 edge

The exact substitution `y=1+x(1+w)` in the pinned R51 base equation is divisible by x^3. Its quotient has 24 nonzero F2 monomials and special fiber `w^3`. The lower Newton hull in `(w exponent,x exponent)` coordinates is `(0,7),(1,2),(3,0),(7,4)`. It has two positive edges:

- Slope `v_x(w)=5`, edge residual `1+w`, a simple root.
- Slope `v_x(w)=1`, edge residual `1+w^2=(1+w)^2`, a repeated root.

The first edge can be Hensel lifted directly. The second calls for `w=x(1+w1)` and another Newton step. These are exact support facts; the normalized y=1 branch count and G/N51 valuations are not yet asserted.

The source/protocol were committed before data. Fresh replay reproduced the JSON byte-for-byte; an altered donor was rejected. An independent evaluator built `y=1+x+xw` by direct bivariate F2 polynomial multiplication, reproduced all 24 terms, and checked every hull supporting line and edge support. Lean 4.34.1 checked the finite residual factorization and transformed-fiber bits. No producer/replay/verifier failure.

Result SHA-256: `0ca85dd551bb1e288aea9f3d2175061750a19b58e9f94d1dbc58f4eb61171fab`. Raw archive SHA-256: `7048bfae8468b1223442297a3fbbdb09ec5ed2968484ad29a6f67dcfe5932a43` (864 bytes). Replay: `python3 research/koblitz_modular_ic_20261018/rounds/112/blowup112.py <output-directory>`; independent check: `python3 research/koblitz_modular_ic_20261018/rounds/112/verify112.py`.

Sources: R51 donor, R105 parser, R111 repeated edge, [Stacks integral closure](https://stacks.math.columbia.edu/tag/00GH), [Stacks Henselian lifting](https://stacks.math.columbia.edu/tag/04GE).
