# Exact row-space statement (producer proof, not a signed theorem)

Over a field F, write the original polynomial list as U,R, where U consists
of rows supported only on the chosen input variables. Let P forget input-only
monomials and A=P(R). The map from coefficient vectors c to cR is linear.
Since P(cR)=cA, the input-only part of span(U,R) is exactly

    span(U) + { cR : c in ker(A^transpose) }.

Here A has rows corresponding to R; the implementation stores its LEFT kernel,
so equivalently cA=0 for a row vector c. This convention is explicit in data.

Each recorded transformed row i is a weighted combination of source rows
0..i with a nonzero coefficient on i. The resulting square origin transform
is invertible lower triangular. Nonzero projected rows have a fresh unit
pivot and zero coefficients at every earlier pivot,so they are independent.
Zero projected rows therefore form a complete kernel basis through their
origin vectors. Recomposition of their full original rows gives all images.
Separate instances of the same rank-certificate contract certify span(U) and
span(U,images). Subtracting these exact ranks gives the new-consequence space
dimension. The checker reconstructs rather than trusting the producer count.

This is classical finite-dimensional linear algebra,not a new algorithm.
Courtois/Klimov/Patarin/Shamir XL Definition3.01 steps2-3 describe linearization
and univariate extraction. We test only constant original-row weights at the
original degree,not multiplied rows or any XL generic-rank hypothesis.
Primary author full text: https://www.nicolascourtois.com/papers/xlfull.pdf .
KB unavailable in this environment (MCP absent; local wrapper not materialized).
RFC9380 AppendixI was consulted for the proposed finite-field extractor,which
is NOT implemented here. No claim is made from search snippets or inaccessible
Diem full text. The frozen route criteria V3 were read at cryptanalysis commit
2dee266203bedb19e302d2af64240815c679f085.
