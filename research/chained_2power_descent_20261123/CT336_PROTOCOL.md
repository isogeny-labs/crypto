# CT336 preregistration — bielliptic positive-control maps from CT335 degenerate kernel

2026-11-23 UTC. **Plan frozen before CT336 arithmetic execution**. [pcc:unverified]

**Input:** the *same* CT334 fully split genus-two curve over p=2147483647, with roots 1..6, and the CT335 determinant-zero pairing (1,6)|(2,5)|(3,4). CT335's full 15-row JSON, JS source, protocol, report and all previous evidence remain unchanged.

**Mathematical construction (not a claim of computational novelty):** take t=x-7/2, u=t², a=1/4, b=9/4, c=25/4 in Fp. Propose two degree-two quotient maps from C:y²=prod_{r=1}^6(x-r):
- pi_plus: (x,y) -> (u, v=y) into the cubic genus-one curve v²=(u-a)(u-b)(u-c).
- pi_minus: (x,y) -> (u, w=t*y) into the quartic genus-one curve w²=u(u-a)(u-b)(u-c).
Check both defining polynomial identities as polynomials in Fp[t], the squarefree/integral genus-one equations, the explicit deck involutions (t,y)->(-t,y) and (-t,-y), and the two linearly independent pulled-back invariant differentials (2t dx/y and 2 dx/y). These certify *curve maps and a separable product-Jacobian quotient* but **not** a measured whole ECDLP transfer or Cantor point map.

**Independent checks:** a source-only integer modular-polynomial computation and a separate SymPy symbolic finite-field polynomial computation of the same two identities, plus three effective mutant rejections (wrong half shift, wrong quadratic root, omitted t in the second map). Record exact residues, equations, source/run identity and scope; no experiment outside this fixed model.

**Hard boundaries:** This is a deliberately bielliptic/decomposable positive-control genus-two curve; it cannot certify a breakthrough on ordinary prime-field elliptic curves or a non-weak benchmark. The still-unproduced original source E->Jac transfer, authenticated 695/701 norm-reuse fixture, native original Cantor/elliptic DLP recovery, physical C169/C13a/C13b/N2_actual/D and strongest cold optimized rho/BSGS/IC comparisons stay with existing owners. Coordinate branch #5007 already owns F3 actual Richelot/elliptic pullback; **do not duplicate its native or E7 work**. No paid compute, CI rerun, protected merge or signed PCC.
