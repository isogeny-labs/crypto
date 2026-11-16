# Exact field-aware polynomial squaring

For a=sum a_i X^i over a commutative field, a^2=sum a_i^2 X^(2i)+sum_(i<j)2*a_i*a_j X^(i+j). In characteristic2 the second sum vanishes. Doubling uses field addition, not an integer encoding assumption for extension fields. Reducing the exact polynomial by monic H preserves equality in K[X]/H even for repeated or zero-root moduli. Empty/sparse inputs remain canonical after the old remainder routine.

For z nonzero coefficients, raw generic convolution uses z^2 scalar multiplications. The specialized raw odd-characteristic product uses z scalar squares and z(z-1)/2 multiplications; the binary product uses z scalar squares. Remainder work is unchanged and counted separately. Fewer operations do not establish runtime or end-to-end gain.

The copied C78 power kernel replaces only its three modular self-products; generic fallback and inverse guards remain frozen. Independent convolution checks arbitrary inputs/moduli; Taylor/Hasse coefficients check repeated prime-field roots; constructed factor products and original conditional solve outputs check full-root integration. Odd extensions, characteristic3 and GF(2^83) are included. No timing promotion is attempted while C79 A/A remains unqualified.
