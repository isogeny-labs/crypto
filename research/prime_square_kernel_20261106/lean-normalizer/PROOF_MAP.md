# Declaration-level proof map

All declarations are in namespace CauchyNormalizer in CauchyNormalizer.lean.

| Declaration | Exact statement/scope |
| --- | --- |
| choose_all_ones | For all natural a,k with k<2^a, choose(2^a-1,k) casts to1 in ZMod2 |
| choose_unit | Same natural inputs; rational2-adic valuation of that binomial coefficient is0 |
| weight_ne_zero | The actual factorial product k!*(2^a-1-k)! is nonzero |
| weight_valuation | Under k<2^a, that factorial product has valuation equal to (2^a-1)! |
| signedWeight_ne_zero | The explicitly signed factorial product is nonzero |
| signedWeight_valuation | Its sign does not change the preceding valuation equality |
| signedWeight_ratio_unit | For k,m<2^a, the actual signed-factorial quotient has valuation0 |
| puncture_zero | For every n, actual punctured product at root0 equals (-1)^n*n! |
| puncture_shift | For every n,k, P(n+1,k+1)=(k+1)*P(n,k) |
| puncture_formula | For every k<=n, actual P(n,k)=(-1)^(n-k)*k!*(n-k)! |
| puncture_eq_signedWeight | For k<2^a, actual P(2^a-1,k) equals the defined signed factorial product |
| puncture_normalizer_unit | For k,m<2^a, actual punctured-product quotient has valuation0 |
| shiftedPuncture_eq | For all n,k and rational shifts, every shifted difference cancels to the actual unshifted product |
| center_lt | For every a, center index2^a is less than grid size2^(a+1) |
| cauchyRho_ne_zero | For k<2^(a+1), the actual shifted-grid quotient relative to center is nonzero |
| cauchyRho_unit | Same inputs; the actual shifted-grid quotient has valuation0 |
| normalized_square_valuation | Same inputs and ANY nonzero rational w; v2(w*w/rho)=2v2(w), with no assumed unit hypothesis |

The following additional declarations are in KernelNodes.lean in the same
namespace, importing the normalizer module:

| Declaration | Exact statement/scope |
| --- | --- |
| baseKernel_node | Every valid source node m has value rho_k if m=k and0 otherwise in the explicitly defined original kernel |
| squareKernel_node_preserved | For all valid k,m and every dyadic size, actual normalized-square kernel equals original kernel at source node m |
| squareKernel_valuation | For any rational t where the actual original kernel is nonzero, the actual square kernel's valuation is doubled |
| cauchyRho_center | The actual center normalizer equals1 |
| baseKernel_origin | Original kernel at0 equals1 at the center-difference index,0 otherwise |
| squareKernel_origin | The same origin statement for the actual normalized square |
| matrixEntry_origin | For all x,y<2^a, actual matrix entry at0 is1 if x=y,0 otherwise |

`cauchyRho` is explicitly defined using shifted products on2^(a+1) nodes.
Its center is k=2^a, and physical coordinate is delta=k-2^a. It is not an
uninterpreted parameter. The bridge from factorials to these actual products
is proved by puncture_formula and shiftedPuncture_eq. [pcc:unverified]

Not formalized: equality to a polynomial derivative API, a Polynomial-typed
degree bound, coefficientwise chart-content valuation, global
moment/Smith count bounds, recovery, recursion, ECDLP correctness or cost.
The native derivative helper directly computes the same omitted-root product;
code-to-Lean execution equivalence has not been certified. [pcc:unverified]

Source build and actual transitive axiom gate pass at the scope above. Claims
bd8ff1 and82eac6 await a trusted signed verdict. Their local checks do not
establish independent execution verification. [pcc:unverified]

Node/kernel claims7288ef,088017,e12f46 have the same pending standing. The
final gate checks24 named theorems and two generated declarations, with0
failing and0 native-trusted. [pcc:unverified]
