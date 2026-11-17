# Prospective native accumulator refinement

This is the next implementation design, not an executed arm. Frozen v1 remains
unchanged; adding this algorithm requires a prospective amended protocol.
Sources checked: actual StreamPrecision declarations, ResidueEncoding kernel,
pinned manuscript D.9/D.10 and its rational-arithmetic combination paragraph.
KB unavailable; no priority claim for odd-part modular arithmetic.

## Algorithm to refine

Let m=p+K, K=8h*2^a+8M(a+1). Process one active node t at a time.
For every nonzero integer z, write z=2^v(z)*odd(z), with odd(z) signed odd.
Construct, using actual orders and excluding0,t,

    V_t = sum_s q_s*(v2(s)-v2(t-s)),
    U_t = product_s [odd(-s)/odd(t-s)]^q_s mod2^m.

All inversions in U_t are odd-unit inversions. The actual valuation theorem
bounds V_t below by-6h*2^(a-v2(t)). Put b_t=a+1-v2(t)>0 and w_t=odd(-t).
For every j<q_t, use

    e_(t,j)=K+V_t-j*b_t,
    g_(t,j)=2^e_(t,j)*U_t*w_t^j mod2^m.

The established multiplier/evaluation bounds make every exponent nonnegative.
If e>=m the modular coefficient is zero, so avoid constructing an enormous
power. Otherwise compute the unit part modulo2^(m-e), then shift. This formula
is the residue of the actual scaled rational coefficient, not a heuristic
unit-cancellation rule [pcc:unverified].

Read or construct the node input/basis buffers at its original p_t; form the
finite inverse and quotient as in frozen v1. Add g_(t,j)*y_(t,j) to one
accumulator modulo2^m, then release that node's buffers. Existing precision
stability makes allowed errors in y vanish in this larger ring after scaling.
The final canonical accumulator must be divisible by2^K. Shift right by K to
obtain the target modulo2^p. Prove the canonical extraction step explicitly;
crossPrecisionCancellation alone is not a native implementation of the shift
[pcc:unverified].

## Proof obligations before an implementation result

1. Bind the signed odd-part product and V_t to actual E_t(0), including negative
   differences and all active levels. No zero valuation convention is used.
2. Bind each modular coefficient to scaledCoefficient and establish the
   exponent/truncation branches, including e0, e=m and e>m.
3. Bind a finite-ring fold to scaledFunctional; show original short p_t input
   residues suffice even though the accumulator has m bits.
4. Derive divisibility of the canonical integer accumulator and correctness
   of its right shift using the actual residue maps, not division by2 in a
   fixed quotient ring.
5. Charge coefficient generation, odd-unit inversion, quotient construction,
   node input delivery, discarded/recomputed state and final extraction.

## Prospective empirical contrast

Keep the exact rational sum as the reference and compare all frozen v1 cells
against a new modular accumulator arm under a new protocol version. Add
boundary controls for nonnegative exponent, truncation at m, negative odd
parts and canonical-shift divisibility. First collect exact outputs/counters;
a timing comparison needs matched isolation and the fastest external component
reference. No timing or memory-saving conclusion is currently established.

A straightforward coefficient-generation inventory is O(L^2) node pairs plus
modular exponentiations and O(sum q_t) coefficient accumulation. Input-jet
production can dominate and is not hidden in that inventory. Peak retained
state and recomputation need an explicit implementation and proof; the dense
all-node B lower bound does not apply automatically to a one-node fold.
