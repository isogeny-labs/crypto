# R102: coefficientwise N51 clearing at x=1

The actual R51 N51 relation has 105 distinct rational coefficients. Exact
F2[x] division by x+1 found every displayed coefficient order negative:
the minimum is -9 (22 terms) and the maximum is -1. Thus (x+1)^9 is the
least coefficientwise clearing power for this relation; (x+1)^8 leaves
22 displayed coefficients with a pole. The attached histogram is generated
from the retained `result.json`.

The source hash, all 105 monomials, a planted malformed x coefficient,
and a separate byte-identical replay passed. An independent
Hasse-coefficient calculation of `p(1+t)` matched all 105 division-based
orders, including the 22 worst terms. Lean 4.34.1 checked the reported
list bound and the failure of power 8. The committed raw archive SHA-256 is
`554e9e6fab5c216b3f4964e9c505970544cac5a9ad0059fc6694c23c79ae38e4`.

The coefficientwise bound is not a normalized function valuation. The next
step is local reduction of (x+1)^9 n at the two R101 source-order points,
followed by integral closure and branch valuation checks.
