#set page(width: 21cm, height: 29.7cm, margin: 2cm)
#set text(size: 10pt, font: ("Liberation Sans", "DejaVu Sans"))
#set par(justify: true, leading: 0.62em)
#show raw: set text(font: ("Liberation Sans", "DejaVu Sans"), size: 9pt)
#show heading: set text(fill: rgb("#1a3a5a"))

#align(center)[
  #text(size: 14pt, weight: "bold")[Kaarelson v2 Theorem 3 coverage, reproduced engine-free (m = 2)]
  #linebreak()
  #text(size: 9pt, fill: rgb("#555"))[2026-11-18 UTC - base `ad2a014c00` - PR #4745 - follows PR #4672]
]

#block(fill: rgb("#f3f3f3"), inset: 7pt, radius: 4pt, width: 100%)[
  Exact integer counts over F_p, native C++20, no Python in the measurement.
  This checks only the engine-free coverage statement, at small instantiable
  primes. Not a solver, timing, or deployed-curve claim. `[pcc:unverified]`
]

*Measured.* For E over F_p and each dilation u, factor base F^(u) = points with
x in [1, X], X = floor(p^(1/2)); coverage(u) = fraction of R in E(F_p) that are a
generic 2-sum of two factor-base points; additive energy E_2/X^2; averaged over
all dilations u. At p in {1009, 4099, 10007}, m = 2, on prime-order curves.

#figure(image("coverage.svg", width: 100%),
  caption: [Measured mean coverage and E_2/X^2 against the paper's values.])

#table(columns: 7, align: (left, left, right, right, right, right, right),
  stroke: 0.4pt + rgb("#bbb"), inset: 4pt,
  [*p*], [*arm*], [*n*], [*X*], [*coverage (SD)*], [*E_2/X^2*], [*cov(u=1)*],
  [1009], [generic], [1013], [31], [0.367 (0.099)], [3.87], [0.274],
  [1009], [j0], [967], [31], [0.364 (0.109)], [3.65], [0.314],
  [4099], [generic], [4049], [64], [0.385 (0.076)], [3.89], [0.551],
  [10007], [generic], [9851], [100], [0.385 (0.062)], [3.86], [0.385],
)

Paper (text after Theorem 3): mean coverage 0.39-0.42 and E_2/X^2 about 4, and
"for y^2 = x^3 + B the numbers are the same". j0 is unavailable at prime order
for p = 4099 (no prime among a = 0 sextic twists) and p = 10007 (supersingular).

*Findings.*
- E_2/X^2 = 3.86-3.89 reproduces the paper's about 4 (within ~3%); the excess over
  the ~1 random baseline is the +/-P sign symmetry of the factor base.
- Mean coverage 0.367 -> 0.385 -> 0.385 is in the same regime, just under the
  paper's 0.39-0.42 on one curve per prime (consistent with the paper averaging
  the band over curves and/or a minor definitional difference).
- j = 0 at p = 1009 matches the generic curve, corroborating "same numbers".
- The plain box u = 1 (0.274, 0.551, 0.385) is not the per-curve dilation mean;
  it is one sample within the spread. The paper's "u = 1 equals the average over
  u" is about the average over curves and needs several curves per prime to test.

*Checks.* An independent implementation (`verify_one.cpp`, direct point
enumeration + hash set, no discrete-log representation) reproduces (p = 1009,
generic, u = 1) to all digits: coverage 0.274432, E_2/X^2 2.378772. The energy
identity sum r = f^2 and E_2 >= f^2 hold; each n is prime and within Hasse; the
run is ~8 s and byte-reproducible.

*Next step.* Average coverage over several curves per prime (same tool, cheap) to
test the paper's exact 0.39-0.42 band and the u = 1 ~ mean claim. The Lean engine
`subexponentialRate_all` audit remains a separate open obligation for Theorem 1.
