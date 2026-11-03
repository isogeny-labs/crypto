# R100: exact factor support of the marked-23 lambda norm

The accepted R99 norm numerator has 74 terms: 73 positive-exponent terms
and constant 1. Every exponent is divisible by 16. R100 source-bound
preparation reconstructed its degree-123 Frobenius root and Singular checked
`root^16 = numerator` over F2[x]. Singular factored the root as

`(x^11+x^7+x^6+x^5+x^4+x^2+1)`
`*(x^22+x^19+x^17+x^14+x^13+x^12+x^11+x^9+x^8+x^7+x^6+x+1)^3`
`*(x+1)^46`.

The complete product and total degree 123 passed; a changed-product control
was rejected. Native exit was zero with no `?` errors and exact pass marker.
A separate fresh Singular replay produced byte-identical factor output.
The committed `verify100.py` independently checked the source hashes,
sixteenth-power identity, GF(2) product, and Rabin irreducibility of all three
factors. Lean 4.34.1 checks the degree arithmetic only; it does not claim
the polynomial identity or irreducibility. The raw archive includes the
failed preparation, accepted run, and fresh replay, SHA-256
`aff76c57c1a8213306d16bb6ea238f90704a576756eda54405e1419d7f20486b`.

Attempt 01 was retained: its preparation expected 73 terms because the
R99 note had omitted the constant from its count. No Singular factorization
ran in that attempt. Amendment 02 and the R99 correction were committed
before attempt 02. See `FAILURE-01.md`.

On `D(x)`, the finite zero support of the displayed lambda monogenic
discriminant norm is `x=1`, the degree-11 point cut out by the first factor,
and the degree-22 point cut out by the second. Combined with R97's base-order
support, the finite set requiring local normalization is `x=0`, `x=1`,
the degree-7 base-discriminant point, and these degree-11/22 points. Infinity
must also be checked. The factor exponents include the sixteenth-power
contribution and are not normalized ramification indices.

Next: compute local integral closures/branch valuations at these five finite
supports and infinity, then derive the proper degree-112 cover's divisor
geometry for transfer searches. Sources: R51 donor; R63, R97--R99 notes and
exact outputs; [Singular manual](https://www.singular.uni-kl.de/index.php/singular.pdf)
for factorization; [Stacks 53.2](https://stacks.math.columbia.edu/tag/0BXX)
for normal projective models.
