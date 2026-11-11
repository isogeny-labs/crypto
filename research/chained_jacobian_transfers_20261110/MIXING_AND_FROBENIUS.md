# Elliptic-factor obstruction, Jacobian shear control, and Frobenius point-count oracle

**Date: 2026-11-11. Standing: [pcc:unverified].** Companion to PR #3877 and queue #3904. This is a bounded mathematical audit, not a new faster discrete-logarithm algorithm or a second full Cantor implementation. No EKS, cloud, or paid experiment was launched.

## Literature/repository-first scope

- Existing internal treatment: `papers/ecdlp-index-calculus-treatise/survey/transfer.md` (Tate/isogeny-class obstruction), `research/notes/ecdlp-general/RESEARCH_REPRESENTATION_STRUCTURE.md`, and `.agents/skills/ecdlp-surfaces/SKILL.md`. The last already documents `frobenius_filter`; this script is a **separate point-count oracle**, not a new sieve.
- T. Shaska, *Genus two curves covering elliptic curves: a computational approach* (2012), §§1, 5: https://ar5iv.labs.arxiv.org/html/1209.3187 (paired degree-two covers and split Jacobians).
- E. Howe, E. Nart, C. Ritzenthaler, *Jacobians in isogeny classes of abelian surfaces over finite fields* (2009): https://ewhowe.com/papers/paper29.html (Weil polynomials and finite-field isogeny classes).
- E. Howe, *Curves of genus two with maps of every degree to a fixed elliptic curve*, version 6 (Sept. 2026): https://arxiv.org/html/2601.19050 (many cover degrees need not be independent isogeny factors).
- Research-KB MCP and the encrypted cross-repo lead atlas were unavailable here. `conductor` and full repository checkout unavailable in this runtime. Do not assert novelty or an independent PCC standing.

## Theorem 1: the elliptic image of any algebraic transfer chain

Let `E/k` be elliptic and `A/k` an abelian variety. A nonzero homomorphism of abelian varieties `H:E -> A` has a finite kernel and an elliptic abelian subvariety `H(E)` as its image; it is an isogeny `E -> H(E)`.

**Proof.** E is simple. If the connected component of `ker H` had positive dimension, it would be all of E and H would vanish. Hence the kernel is finite. Its connected image is a dimension-one abelian subvariety, so is elliptic and isogenous to E. Over `F_q`, H commutes with Frobenius, and the rational Tate-module factor of its image forces `T²-t_E T+q` to divide the Frobenius characteristic polynomial of A. QED.

A composite `E -> A1 -> ... -> J(C)` is still a homomorphism and subject to exactly the same result. A nonzero continuation ending on another elliptic curve `E'` makes `E'` isogenous to E. **Boundary:** index calculus on the *whole* ambient Jacobian might still help an embedded elliptic subgroup; non-homomorphic reductions, auxiliary data, and cross-characteristic computational constructions are outside this theorem.

## Theorem 2: a nonzero cross-factor chain can be manufactured without a gain

Over characteristic other than 2 and 3, use smooth projective models `C:y²=x⁶+1` and `E:v²=u³+1` with the degree-two maps

`f(x,y)=(x²,y)`; `g(x,y)=(x^(-2),y/x³)`.

Their deck involutions are `(-x,y)` and `(-x,-y)`, respectively. Under the opposite quotient map, points in a generic deck pair map to negatives, giving

`f_*f^*=g_*g^*=[2]`, and `g_*f^*=f_*g^*=0`.

Set `K=g^* f_* : J(C)->J(C)`. Then `K²=g^*(f_*g^*)f_*=0`, so `M=1+K` is an **abelian-variety automorphism**, inverse `1-K`. Direct composition yields

`g_* M f^* = (g_*f^*)+(g_*g^*)(f_*f^*)=[4]`,
`f_* M f^*=[2]`.

Thus the zero natural cross map becomes nonzero following an actual Jacobian-to-Jacobian mixing map. On odd-order torsion `(f_*,g_*)` supplies split-factor coordinates; M has matrix `[[1,0],[2,1]]` and transports `(2P,0)` to `(2P,4P)`. This is **factor rerouting** with no new attack. Require more than nonzero off-diagonal composites to call CT04 useful. This is an elementary application of the known split-Jacobian structure, not a novelty claim.

## Theorem 3: exact necessary condition from N1 and N2

For genus-two `C/F_q`, write `N1=#C(F_q)`, `N2=#C(F_(q²))`, `a1=q+1-N1`, `a2=(a1²-(q²+1-N2))/2`. Then

`chi_J(T)=T⁴-a1*T³+a2*T²-q*a1*T+q²`.

An elliptic `E/F_q` of trace `t=q+1-#E(F_q)` can be a nonzero algebraic image only if

`a2=t*(a1-t)+2q`.

This tests divisibility by `T²-t*T+q`. A passing result is **not** a constructive map, a map-cost certificate, or a DLP advantage.

Local exact point enumeration in `frobenius_screen.py`, over five prime fields:

| p | t_E | C:y²=x⁶+1 (N1, N2, a1, a2) | E-factor | C0:y²=x⁵+1 (N1, N2) | E-factor |
|---:|---:|---|:---:|---|:---:|
| 101 | 0 | 102, 10606, 0, 202 | pass | 98, 9958 | reject |
| 103 | 20 | 64, 10222, 40, 606 | pass | 104, 10610 | reject |
| 107 | 0 | 108, 11878, 0, 214 | pass | 108, 11450 | reject |
| 109 | 2 | 106, 12310, 4, 222 | pass | 110, 12318 | reject |
| 127 | 20 | 88, 15838, 40, 654 | pass | 128, 16130 | reject |

**Concrete shared-subgroup negative control:** `D/F_101 : y²=x⁵+3x+1` has `N1=87`, `N2=10273`, `a1=15`, `a2=148` and `#J(D)(F_101)=8820`. The source `E` has order 102, hence `gcd(102,8820)=6`: both groups have order-three subgroups, yet `148 != 202`, so **no nonzero F_101-algebraic homomorphism E -> J(D) exists**. Its quintic is squarefree because the discriminant `3125+256*(3^5)` is nonzero modulo 101. Sharing a cyclic prime-order subgroup abstractly is not an algebraic transfer.

## Replay receipt and limits

```sh
python3 test_unipotent.py
python3 frobenius_screen.py
python3 -m py_compile test_unipotent.py frobenius_screen.py
```

Local CPython runs passed: `test_unipotent.py` checked 100/62/106/104/86 affine C-points over p=101/103/107/109/127 respectively, 98/60/104/102/84 nonzero-x deck pairs per field (both deck involutions), and all 17 scalar values for a generator of order 17 over F_101. The point-count program tested ten genus-two cases plus the separate r=3 shared-subgroup negative control; its norm-based quadratic character was checked independently against Euler exponentiation on 49 extension-field elements per field. **Producer self-checks only [pcc:unverified].** Neither script evaluates a genus-2 Mumford divisor under M, nor benchmarks a full solver.

**Queue consequence:** CT03 should implement this trace-factor necessity as a fast screen; CT04 should distinguish zero paths, algebraically trivial shears and genuinely useful cross-factor transfers. CT01/CT02 must still independently validate arbitrary divisor maps; CT05 still requires matched complete solving-cost accounting. No merge until the existing PCC/CI/review gates accept the exact head.

### Additional independent numerical oracle

`python3 independent_quad_count.py` independently reproduced all **11** `F_(p²)` counts: the five positive `x⁶+1` cases, five `x⁵+1` negative controls, and `D:y²=x⁵+3x+1` over `F_101`. Instead of quadratic characters or the field-extension powering routine in `frobenius_screen.py`, it enumerates **every** `y²` in the quadratic field into a square-table and evaluates `x⁵`/`x⁶` by binomial expansion. This provides a structurally different point-counting algorithm, but is still same-session producer evidence **[pcc:unverified]**, not a signed independent-verifier result.
