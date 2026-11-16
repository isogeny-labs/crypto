# Low-product recursion and source integration

Let L_n(A) be A modulo X^n. Terms of either operand with degree at least n cannot
contribute, so L_n(AB)=L_n(L_n(A)L_n(B)). Normalize coefficients and remove trailing
zeros before choosing a split. If n covers the full output, use the original product.
If the shorter operand has length<=12, sum only terms with i+j<n.

For the recursive case write A=A0+X^k A1, B=B0+X^k B1, where k is the existing
half-length split. Define U=A0B0, V=A1B1 and W=(A0+A1)(B0+B1). Then

    AB = U + X^k(W-U-V) + X^(2k)V.

For n>k it suffices to compute U modulo X^n, and W,V modulo X^(n-k).
Only U modulo X^(n-k) enters the cross subtraction. The final shifted V term needs
just n-2k coefficients when positive; these are already available. In particular,
one must not discard V merely because 2k>=n: its subtraction still contributes to
the cross term. If n<=k only L_n(A0B0) contributes. Recursive sizes decrease;
the result is assembled into n slots and canonically trimmed.

This identity preserves Karatsuba's three-product split; truncation changes the
requested coefficient ranges. It establishes correctness, not an automatic
complete-cost improvement: normalization, allocation, addition/subtraction and
recursion overhead remain. No wall-time, operation-total or memory gain is inferred.

Newton update V <- V(2-AV) modulo X^k uses only low-k products. Reversed polynomial
division uses only the first n coefficients of reverse(A)/reverse(B). The monic
remainder path needs only low-d coefficients of QB. These are the five replaced
full-product/truncate callsites (two Newton, two reversed quotient, one remainder).
Full product-tree polynomials are still computed by the original multiplier.

Prior art: truncated products and their role in Newton arithmetic are established;
FLINT documents nmod_poly_mullow and coefficient-range multiplication at
https://flintlib.org/doc/nmod_poly/mul.html (retrieved2026-11-03). This implementation
adapts the repository's existing canonical-vector/Karatsuba engine, not FLINT source.

Evidence:936 finite low-product comparisons against two references; sparse and
cancellation cases; full66-method Rust invocation including8 fresh verified recovery
receipts. This finite validation accompanies the algebraic argument, not a proof
of every machine execution or a fresh full-cohort run.
