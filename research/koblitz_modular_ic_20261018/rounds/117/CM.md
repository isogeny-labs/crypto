# R117: actual G51 Laurent coefficients on all four x=0 base places

All 29 terms of the pinned G51 equation were evaluated on each R116 base series. Each x^e denominator was factored into its exact local s-order and a unit, inverted only to a recorded cutoff, and combined by lambda exponent before taking orders. A separate schoolbook rational evaluator reproduced every coefficient support and cutoff.

The two e=1 places give exact coefficient orders at lambda exponents 0,1,4,8,16:

- y0/e1: `[-160,-127,-97,-80,0]`, with cutoffs `[320,386,446,480,640]`. Its certified Newton edge joins `(0,-160)` to `(16,0)`, so lambda roots on this edge have local s-order -10. The on-edge exponents are 0,8,16.
- y1/e1: `[0,23,23,0,0]`, with cutoffs `[318,384,444,478,640]`. Its certified Newton edge is horizontal from `(0,0)` to `(16,0)`, so lambda roots have local s-order 0. Its on-edge exponents are again 0,8,16.

In both cases the initial edge polynomial is `1+T^8+T^16=(1+T+T^2)^8` over F2. Its repeated F4 roots require a further lambda lift; the edge valuations do not identify normalized lambda places.

The other two places expose precise precision gaps:

- y0/e3: coefficient orders for lambda^0 and lambda^1 remain unknown above lower bounds -323 and -125. The lambda^4, lambda^8 and lambda^16 orders are -61, -56 and 0.
- y1/e2: the lambda^0 coefficient order remains unknown above lower bound -4. The lambda^1, lambda^4, lambda^8 and lambda^16 orders are 23,23,0,0.

All 29 rational numerators were visible at base precision 640; the gaps arise from cancellation past the termwise rational precision cutoff, not a hidden numerator. No Newton hull is asserted for either incomplete place. A precision extension to at least about 1024 is the next contained test for y0/e3, with the same explicit cutoff logic; choose a larger margin if cancellation persists.

The source/protocol were committed before data. Fresh replay was byte-identical, changed R116 input failed the hash gate, and independent schoolbook inversion reproduced every row. Lean 4.34.1 checked both exact edge inequalities, on-edge index sets, and the repeated initial polynomial; generator readback binds its finite constants to the checked JSON. The Lean file does not formalize all rational substitutions or Newton's theorem. No producer, replay, verifier or Lean failure.

Result SHA-256: `b1786c05cca9509a8e9c0dd21c781c1110ced4b0a545b15c08120edbb467f393`. Raw archive SHA-256: `82427e80dd28742853e934d7f34cde66bd8e95c4b1d4acc9c1063a3af89eab15` (3,580 bytes). Replay: `python3 research/koblitz_modular_ic_20261018/rounds/117/coeff117.py <output-directory>`; independent check: `python3 research/koblitz_modular_ic_20261018/rounds/117/verify117.py`.

Sources: R51 donor, R98 denominator audit, R105 parser, R116 base series, [Newton polygon theorem](https://www.math.columbia.edu/~gyujinoh/ANT_revised.pdf).
