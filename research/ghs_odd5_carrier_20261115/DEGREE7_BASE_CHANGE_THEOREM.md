# A base-change family with a genus-five transfer and a scoped cost criterion

16 November 2026 UTC. The geometric proof below uses the standard functoriality of Weil restriction, Jacobian pullback and Galois trace; these notions are not formalized in `Odd5Ledger.lean`. The exact cover identity and nonzero subgroup images are checked by the saved Sage/Magma receipts. Finite enumeration claims remain [pcc:unverified] until PCC admission.

## The family

Fix `k=F3`, `K=F3[a]/(a^5+2a+1)`, the genus-five curve `Y/k` of `DEGREE7_FINDING.md`, and the representative `E/K: v²=u³+2u²+a+a³` (mask 30). Let `phi:Y_K→E` be the saved degree-seven cover. Its four other K-model equations are obtained by Frobenius conjugation, as checked in `cover_frobenius_orbit.json`.

For every integer `s≥1` with `gcd(s,5)=1`, put `k_s=F_(3^s)` and `K_s=F_(3^(5s))`. Since K and k_s are linearly disjoint over k, `K_s=K k_s`, and base change gives an explicit separable degree-seven cover `phi_s:Y_(K_s)→E_(K_s)`. The genus of Y stays five. This is an **infinite field-size base-change family of one K-isomorphism orbit**, not a positive-dimensional family of independent curve parameters.

The `F3`-homomorphism

`F: Res_(K/k) E → J(Y),    P ↦ Tr_(K/k)(phi^*(P))`

is defined algebraically by divisor pullback and trace. Its evaluation on the selected order-109 point is nonzero (`degree7_subgroup_map_30.json`), so F is a nonzero homomorphism. Both sides are simple abelian varieties of dimension five with the same irreducible Weil polynomial `X^10−26X^5+3^5` (`THEOREM.md`); hence F is an isogeny. Base change preserves an isogeny, and the linear-disjointness identification gives an isogeny

`F_s: Res_(K_s/k_s) E_(K_s) → J(Y)_(k_s)`

for every `s` coprime to five. Its point formula is the corrected degree-seven pullback followed by the five-term `Frob_(3^s)` trace. For any prime-order subgroup of `E(K_s)` with prime `r` coprime to the fixed integer `deg F`, `F_s` is injective on that subgroup. The fixed-kernel condition is a **rigorous algebraic eligibility criterion**; it is not an assertion that every prime factor of every `#E(K_s)` passes it. The saved evaluations separately verify nonzero images for `r=109` at `s=1`, `r=65827` at `s=3`, and `r=333566377062870325133` at `s=11`.

The same Frobenius-polynomial argument shows `Res_(K_s/k_s) E_(K_s)` and `J(Y)_(k_s)` stay simple of dimension five when `gcd(s,5)=1`. Thus no nonzero algebraic transfer from this Weil restriction into the Jacobian of a curve of genus below **five** exists over k_s. The explicit Y attains this genus floor for the entire coprime-to-five family. At `s=1`, the independently replayed fixed-pair cover search excludes degrees below seven, so this exact Y-to-E cover also attains the **minimum K-cover degree seven**. After enlarging to K_s, a lower cover degree is not excluded by those K-only screens; the proven uniform bound is `degree(phi_s)=7`.

## DLP cost criterion and measured limit

[Diem's theorem](https://arxiv.org/abs/math/0606607) gives expected `O-tilde(q^(8/5))` DLP time for a genus-five Jacobian over `F_q` with bounded-degree plane model when its degree-zero class group is cyclic, and also when its full group structure is known. Our Y has a degree-eleven hyperelliptic plane model. The orders of `J(Y)(F3)`, `J(Y)(F27)`, and `J(Y)(F_(3^11))` are squarefree, so the cyclicity hypothesis holds at `s=1,3,11`. This theorem supplies an asymptotic cost class, not a concrete bound or measured speedup for any fixed q.

For a source prime subgroup of order r, generic rho costs on the order of `sqrt(r/m)` operations when an efficiently evaluable order-m Frobenius quotient is used. For this base-change family m can be as large as s; the exact s=3 subgroup has checked m=3. At s=11, `s11_frobenius_eigenvalue.json` checks an order-11 Frobenius eigenvalue `213515707826203049379` on the saved 69-bit point, so a matched rho can use m=11. Ignoring logarithmic factors and constants, the GHS exponent beats this matched rho exponent only when `r` grows faster than `m q^(16/5)`, with `q=3^s`. This is a **screening formula**, not a sufficiency theorem for concrete runtime. It also requires that the relevant transfer image be nonzero and that Diem's group-structure hypotheses hold. The Lean ledger checks the bare integer exponent comparisons and eigenvalue arithmetic for the selected s=3 and s=11 primes; it does not prove the algorithmic asymptotics or point action.

For the saved s=11 prime, `q=177147`, so the bare leading powers are `q^(8/5)≈2.50×10^8` small-field units and `sqrt(r/11)≈5.51×10^9` source-group units, about 27.9 and 32.4 bits respectively. These units have different costs, and the omitted constants, logarithms, relation graph, linear algebra and target descent can reverse the numerical ordering. This is a **derived exponent screen**, not a measured attack-cost ratio.

At `s=3`, the complete measured pipeline is slower than matched rho by 11.47 times (`S3_FULL_ATTACK_RESULT.md`). At `s=11`, the 69-bit subgroup transfer is checked, and the group is cyclic, but no complete genus-five index-calculus solve or same-target rho measurement has been performed. A globally new practical GHS vulnerability or a universal classification of vulnerable curves therefore remains unestablished.
