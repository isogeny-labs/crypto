# Linear-denominator specialization: frozen exactness gate

Previous turn was progress: normalized single-target solver passes12runs
(6distinct targets under2anchor configurations). Runtime benefit is unmeasured.
Freeze this gate before implementation. No timing claim is licensed by it.

## Source receipt

Primary prior art: D'Andrea/Krick/Szanto arXiv:1211.6895v3, Theorem3.1and
Remark3.2, read directly: https://arxiv.org/html/1211.6895v3 . The proposal below
is a specialization of established rational interpolation, not a novelty claim.
KB revision9141e386254340a9bdba3036f2bb706a, refresh2026-11-14T21:44:48Z,
lexical-only and predating this work. Searches: "rational interpolation"
"linear denominator"; "barycentric" "common factor". Hits do not supply
relevant mechanism prior art or absence evidence; the primary source supplies
the interpolation framework. Native existing source is rational.rs and the
primitive accounting in PRIMITIVE-PROTOCOL.md. Do not relax that accounting.

## Candidate algebra and proof obligations

Specialize d2, M=g+4. For signed weights ui=sigma_i*yi/A'(xi), set

    H = sum_i ui*A/(x-xi), S0=sum_i ui, S1=sum_i ui*xi,
    b=S0*x-S1, a=b*H-S0^2*A.

Require exact coefficient equality with the existing subset/Walsh pair, not
just projective equality. The leading two coefficients cancel, so deg a<=M-2.
At a root r of b, a(r)=-S0^2*A(r). If S0!=0, a,b have a common factor exactly
when r is an anchor, and that factor is x-r. If S0=0,S1!=0 then b is a nonzero
constant; if both are zero the pair is zero. These conditions replace a generic
polynomial GCD on this family, not on arbitrary pairs.

Compute all b(xi) as linear sign forms. Precompute wi*(xi-xj), update with Gray
sign flips using additions/subtractions, and store the canceled-anchor index
at its INTEGER sign-mask position. Preserve integer consumption order exactly.
No sign/tuple/rank scheduling change is permitted in the exactness comparison.

For primitive normalization, divide a by the identified monic linear factor,
set b'=S0 and remove that anchor; otherwise leave a,b unchanged. Compute Q
from only the upper norm coefficients by monic long-division recurrence:

    q[k] = N[L+k] - sum_(j>k) A_keep[L+k-j]*q[j], L=deg A_keep.

The residual bound is g when no factor is removed, g-1 otherwise. The recurrence
is exact only because interpolation already guarantees divisibility; gate it
against full norm construction and division in every fixture. Preserve zero
pairs, constant denominators, branches and degree drops.

## Frozen checks, caps and acceptance

Curve fixtures g2/3/4, p101/127/503, first squarefree thread-family c1..59,
four contiguous FB tuples at offsets0/3/6/9, all integer sign classes, d2.
For each: exact a,b equality; normalized a,b,Q,keep-mask/common-degree/removed
equality versus generic primitive; independently verify predicted gcd and full
norm division through that reference. Require nonzero common-factor coverage
per genus. Check all-zero ordinates and a forced nonzero constant-denominator
case on arbitrary data separately. The latter checks pair/gcd classification,
not a curve norm or principal relation.

Integrate an explicit solve_linear wrapper beside existing rational_solve::solve.
It must preserve all12frozen solver runs' recovered k, requested/effectiveH,
constructed/consumed functions, admissions, common factors, normalized admissions,
pure rows, tuples, unions/pins and descents. Compare both full raw/projected
group gates. Runtime fields may differ. Keep the old generic solve path intact.

Native local correctness tests, <=60s per invocation, <=256MiB target memory,
no cloud spend. Any mismatch fails the gate and its output is retained. A
separate complete cold protocol must precede timing; correctness/count equality
does not establish a runtime improvement or the full mathematical10x objective.
