# Primitive degree-eleven base-change isogeny screen

Frozen before the reproducible run on 16 November 2026 UTC. The earlier interactive `isogenies_prime_degree(23)` call was discovery only: it reported 24 isogenies, of which 22 had codomain `j` outside `F243`. The JSON producer below is the independently replayable receipt.

## Fixed instance and checks

Take `K=F3[a]/(a^5+2a+1)`, `L=F_(3^55)` using Sage's fixed modulus and the first `K -> L` embedding, and `E/L:y²=x³+2x²+a`. Enumerate all Sage-rational cyclic 23-isogenies of E. Select the first whose codomain `j` is not fixed by 243-power Frobenius. Record the exact field modulus, embedding image of `a`, the source and codomain Weierstrass coefficients, codomain `j`, the kernel polynomial, and the dual codomain.

The trace recurrence `t_0=2`, `t_1=26`, `t_s=26t_(s-1)-243t_(s-2)` gives the exact source order at `s=11` as `174449211009094048378706606 = 2*109*2399*333566377062870325133`. Check the 69-bit factor is prime and coprime to 23. From a deterministic sampled source point P, multiply by the cofactor to obtain an order-r point, then verify `f(P)` is also order r and `f^dual(f(P))=[23]P`. The selected dual is an evaluable, subgroup-preserving map from the primitive-j source `E'` back to E. These checks do not construct the `Res E -> J(Y)` map.

The screen's limited novelty claim is an exact primitive-j isogeny neighbor in the same known transfer-eligible isogeny class. It does not establish a new attack method or a faster DLP. Prior work on isogeny-assisted GHS remains applicable. A full claim still requires an evaluable small-field Jacobian image, decomposition and matched rho cost.
