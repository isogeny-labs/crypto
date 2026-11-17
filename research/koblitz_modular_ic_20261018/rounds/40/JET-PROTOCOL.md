# R40 jet implementation, frozen before JET40 data

Exact native Rust computation, 4GiB virtual/1200s build and run caps.
No statistical, comparative or speed claim. All 60 R39 canonical degree-7
supports are fixed; no adaptive point selection or coefficient box search.
Use original S5/U8 polynomials and lambda/nu derivatives from accepted ENC39,
literal original/descended functions from ENC40, and fixed R33 product pairs.
Donor hashes are verified against sealed manifests before copying.

Build F32/F128 with all field products checked against independent binary
reduction, inverses, associativity/distributivity and quadratic extension.
Construct u=u0+z, the plane v by the truncated AS sum and rational b,c.
Solve the actual five source equations for descent lambda/nu through a
lexicographically first constant invertible two-component pair. Newton stages
2,4,8,16,17 evaluate only at current precision. Require every source equation
after each correction, and all previous coefficients unchanged. Export six
coordinate series, eight unit/inverse series and all selected section values.

For every support: check all seven coefficient conjugates and closure,
recompute all source/unit inverse identities, and reject nu+z16. Independently
evaluate all 68 original functions in both T orientations and compare their
descended combination with all 68 literal twist functions. Compute the
196 quadratics and 324 cubics with fixed pairs; no new RR basis selection.

Positive regression: native F32 solver at actual R35 Q, compare ALL 588
selected section/quadratic/cubic series through precision 17 against the
sealed precision-154 output. This catches coordinate/denominator/descent and
field-encoding errors beyond merely passing a source residual.

Pack all 60 degree-seven local algebras, dimension 7140 over F2; compute ranks
of fixed 68/196/324 columns with original-column recombination of every pivot.
Require those ranks exactly. These are section/evaluation ranks, not IC
relation ranks. The supports/local/proper bridge is R39 LOCAL39 and R34
LOCAL_GEOMETRY; no chosen target image or complete degree460 divisor follows.
Exit zero, exact marker, all controls and all 60 outputs are required.
Retain build, failure, timeout and native records. No automatic retries.
