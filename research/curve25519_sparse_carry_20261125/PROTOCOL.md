# Curve25519 sparse differential-addition search: protocol v1

## Question and scope

Find a non-torsion quadruple of actual Curve25519 point abscissae
`(x(P), x(Q), x(P+Q), x(P-Q))` with each coordinate representable by at most
two nonzero signed radix-8 digits and all cyclic carries of the quartic at
most 16 in magnitude. This is a bounded public-curve search, not a discrete
logarithm computation or performance comparison.

## Frozen input and enumeration

- Field: `p = 2^255 - 19`; curve: `y^2 = x^3 + 486662*x^2 + x` over `F_p`.
- Digits: `x = sum(d_i * 8^i) mod p`, `i = 0..84`, `d_i in {-1,0,1}`;
  Hamming weight at most 2. Enumerate by weight, increasing positions, then
  signs `-1,+1`; retain the first encoding of a residue.
- Lift each residue to a curve point with the deterministic square-root
  algorithm for `p = 5 mod 8`; choose the root returned by that algorithm.
- For every ordered pair of distinct liftable coordinates whose first input
  has weight at most 1, compute both output coordinates using the affine
  Montgomery addition formulas. Count sparse-output hits. Require all four
  coordinates to have nonzero `[8]` multiples to exclude torsion.
- For each surviving quadruple, verify the quartic and the cyclic carry
  recurrence exactly over the integers. The pass criterion is at least one
  non-torsion quadruple with `max |c_i| <= 16`. The stop condition is exhaustion
  of the stated ordered-pair population. Retain every hit and failure count.
- No random source, seed, scheduling choice, target scalar, or private input
  enters this version. Enumeration order is fixed above; the prospective
  `experiment-seeds/v1` seed manifest is therefore `random_streams: []`.

## Exact carry convention

For the quartic

`F = (x1-x2)^2*x3*x4 - (x1*x2-1)^2`, form its integer coefficient vector
after substituting the chosen signed-digit polynomials in `t`. Fold every
coefficient at exponent `e` into `h[e mod 85]` with factor
`19^(floor(e/85))`, since `t^85 = 19`. The cyclic carries satisfy

`h0 + 19*c84 = 8*c0` and `hi + c(i-1) = 8*ci` for `i = 1..84`.

The implementation must reject a nonintegral recurrence, a failed closure,
or `F(x1,x2,x3,x4) != 0 mod p`. The bound concerns these 85 cyclic carries,
including `c84 = H(8)/p`, where `H(8) = sum(hi*8^i)`.

## Verification and evidence

- Unit controls: the torsion quadruple `(0,1,1,1)` must satisfy the curve,
  quartic, and zero-carry equations but fail the non-torsion filter; `x=4`
  must pass the non-torsion check. A synthetic `F=p` carry vector must pass
  with nonzero carries.
- A separate verifier should recompute every reported hit from coordinates,
  not infer validity solely from the quartic. Record source commit, Rust
  toolchain, dependency lock hash, command, complete output, and its hash.
- Counts from the prior standalone Python exploration are historical context,
  not a frozen native result or a speed measurement. No timing claim is made.
- If a run is performed, add its raw record, source-linked report, visual,
  and PDF to this directory before promoting any result.

## Prior work and applicability

- [RFC 7748](https://www.rfc-editor.org/rfc/rfc7748) fixes the Curve25519
  field and Montgomery parameter.
- The [Explicit-Formulas Database](https://www.hyperelliptic.org/EFD/g1p/auto-montgom-xz.html)
  gives the Montgomery differential-addition and doubling formulas used for
  the coordinate and torsion checks.
- [Semaev 2004](https://eprint.iacr.org/2004/031.pdf) establishes the
  summation-polynomial setting; this search tests a restricted coordinate
  representation, and makes no novelty or solving-cost claim.
- Repository context: `research/prime_field_summation_polynomials_20261119/README.md`
  and `research/curve25519_wave7_untried_20261122/README.md` cover existing
  Semaev and Curve25519 coordinate studies. The shared research-KB service
  was unavailable in this environment, so no prior-art absence is claimed.
