# CT20b: geometric 2-torsion factorization for the degree-seven continuation

**Standing: [pcc:unverified].** This is a conditional algebraic argument and a symbolic branch-pair self-check, **not** an executed genus-2 Jacobian isogeny, a signed PCC proof, or a speedup. Stacked on crypto #3920 / #3877; elliptic maps from cryptanalysis #1483. No frozen PCC ledger or existing arithmetic code changed.

## Prior work and exact hypotheses

Primary source: Howe, Leprévost and Poonen, *Large torsion subgroups of split Jacobians of curves of genus two or three* (Forum Math. 12, 2000), Section 3.2, Propositions 3 and 4 and their proofs: https://math.mit.edu/~poonen/papers/large.pdf . The standard 2-gluing graph and quotient theorem are **prior art**, not a novel transfer. This work reuses CT17's quotient identities, CT19's existing degree-7 maps, and CT20's explicit target models. Checked `CT20_PLAN.md`, `CT20_F101_GLUING_INVENTORY.md`, `QUEUE.md`, source PRs #3877/#3919/#3920 and consumer #1483. Research-KB and locked lead atlas inaccessible; no novelty assertion.

Let `J0=Jac(C0)` over F101, `C0:y²=t⁶+6t⁴+13t²+56`, and `A0=E1×E2`, with rational elliptic orders 109 and 113. Let `N0:J0→A0` be the pair of quotient norms and `P0:A0→J0` the sum of quotient pullbacks. Assume the **geometric** identities `P0 N0=N0 P0=[2]` and `Ker(P0)=G0`, the graph of the specified Galois-equivariant E1[2]→E2[2] matching.

For either CT20 monic target chart (B+A or B+B) let `alpha=phi1×phi2:A0→A1` be the published product of separable degree-7 elliptic maps, and `G1=alpha(G0)`. Assume the **actual** HLP target quotient is `J1=A1/G1`, with `P1:A1→J1` and `N1:J1→A1` obeying `Ker(P1)=G1` and `P1 N1=N1 P1=[2]`. Checked cubic coefficients, zeta and elliptic-isomorphism scaling are necessary but alone do not certify those quotient-map identifications. The direct/reverse elliptic quotient **swap** in #3920 must be respected.

## Conditional theorem and proof (global, not rational halving)

**Claim [pcc:unverified].** `A=P1 alpha N0:J0→J1` factors uniquely as `A=Phi [2]` for a k-rational isogeny `Phi` of degree 49. Its corresponding reverse `PhiHat`, defined with the normalized degree-7 dual elliptic isogenies, satisfies `PhiHat Phi=Phi PhiHat=[7]` on the geometric Jacobians. Therefore `Phi` is injective on every source subgroup of order coprime to 7 (in particular 109).

Work over an algebraic closure, not merely on the odd-order rational group. Write the six branch points of the source as `W_(i,+),W_(i,-)` for three distinct roots `a_i` of `a³+6a²+13a+56`. Their elliptic quotient images are the matched nonzero 2-torsion points `T_i` and `S_i`, with x-coordinates respectively `a_i+2` and `56/a_i+38`. The 15 unordered branch-pair divisor classes represent the 15 nonzero elements of `J0[2]`. Their `N0` images are `(T_i+T_j,S_i+S_j)`. Three opposite pairs (i=j, different signs) map to zero. For all twelve pairs with i≠j, the matching `psi(T_i)=S_i` is F2-linear, hence `S_i+S_j=psi(T_i+T_j)` and the norm image lies in `G0`. We have proved the inclusion `N0(J0[2])⊆G0` on **all sixteen geometric classes**, not just J0(F101).

Since `alpha(G0)=G1=Ker(P1)`, `A` kills geometric `J0[2]`. Characteristic 101 is odd, so `[2]:J0→J0` is the separable quotient by this finite étale kernel. The quotient universal property gives a unique **k-defined** homomorphism `Phi` with `A=Phi[2]`. Its degree is `(4×7²×4)/16=49`.

With `alphaHat alpha=alpha alphaHat=[7]`, the reverse `Ahat=P0 alphaHat N1` also kills `J1[2]`: on 2-torsion `alphaHat(G1)=[7]G0=G0`, so `Ahat=PhiHat[2]`. Thus
`Ahat A=P0 alphaHat (N1P1) alpha N0=[2]P0[7]N0=[28]`,
and symmetrically `A Ahat=[28]`. Since `Ahat A=[4]PhiHat Phi`, cancel `[4]` in the torsion-free group `Hom(J0,J0)` to obtain both global dual identities. The order-109 kernel intersection is trivial because `[7]` is invertible there.

This argument explicitly **depends** on the geometric source/target gluing identifications. It is a proof of what those maps entail, not independent construction of `P1` or execution of `Phi`.

## Small symbolic census with a discriminating negative control

Run `python3 -S research/chained_jacobian_transfers_20261110/ct20_two_torsion_graph.py` (assertions enabled, no third-party modules). It enumerates 15 branch-pair labels under an F2² representation of each elliptic 2-torsion group. Producer local test reports: **3** pairs with norm zero; **4** in each of the three nonzero graph fibers; **15/15** lie in the correctly matched target graph. Under a deliberately wrong three-cycle matching, only **3/15** pass; the other **12** reveal failure of the required kernel inclusion. This tests symbolic incidence and degree bookkeeping, **not** field arithmetic, actual quotient maps, Frobenius compatibility, Cantor arithmetic, or signed independent PCC standing.

## Next technical acceptance

Independently certify actual target `P1/N1` and swapped component isomorphisms in balanced divisor coordinates, including exceptional/infinity/nonsplit classes and Frobenius descent; evaluate `Phi(D)=P1 alpha (N0(D)/2)` on **all 12,317 rational classes** (halves 55 mod 109 and 57 mod 113), with inverse dual identities, additivity and every 109-source scalar. This rational halving is a *formula for points only after the geometric factor theorem*, not its proof. Next use the 64 elliptic product 4-torsion lifts to compute `Phi|J0[2]` and select the transported nonbacktracking Richelot kernel. Freeze a complete solver-cost benchmark only after actual maps, independent checks, S3 admission and budget visibility. Preserve direct elliptic degree-7 and matched rho/index-calculus controls. Existing PCC ledger fork/archive gates remain unresolved.

Reference diagram: `ct20b_torsion_graph.svg`. A visual PDF report was rendered and inspected in the producing workspace; it is not claimed to be part of the GitHub text-only commit. **[pcc:unverified]**
