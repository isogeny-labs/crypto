# Coprime base change of the ±9 genus-seven carrier

Status: conditional on the independently reviewed q2 census witness; the base-change implication is a standard geometric argument written out below. The isogeny and family conditions are not yet formalized in Lean or signed by PCC. [pcc:unverified]

## Theorem

Let `k0=F2`, `K0=F128`, and let `E_-/K0` and `Y_-/k0` be the exact curves in `PROTOCOL.md`. The q2 census gives `J(Y_-) ~ A0 := Res_(K0/k0) E_-`. For every positive integer s with `gcd(s,7)=1`, set `k_s=F_(2^s)` and `K_s=F_(2^(7s))`. Let `E_s` and `Y_s` be the respective base changes. Then

`Res_(K_s/k_s) E_s ~ J(Y_s)` over `k_s`.

The analogous isogeny-class statement holds for the +9 twist pair. Family membership depends on the absolute `Tr_(K_s/F2)(a)`: the minus source has W1 type for odd s and W0 type for even s; the plus source has W0 type for every s. These are published families, not a new eligible family.

## Proof

Because `gcd(s,7)=1`, the two finite fields `F128` and `F_(2^s)` are linearly disjoint over F2. Their tensor product is the field `K_s=F_(2^(7s))`. Weil restriction commutes with arbitrary base change at the level of represented functors: for any `k_s`-algebra R,

`(A0 ×_(k0) k_s)(R) = E_-(R ⊗_(k0) K0) = E_s(R ⊗_(k_s) K_s) = (Res_(K_s/k_s) E_s)(R)`.

Thus `A0 ×_(k0) k_s` is canonically isomorphic to `Res_(K_s/k_s) E_s`. The Jacobian commutes with extension of the ground field, so `J(Y_-) ×_(k0) k_s ≅ J(Y_s)`. Base change of any F2-isogeny `A0→J(Y_-)` gives the claimed Fq-isogeny. An F2-isogeny exists by Tate's theorem because the q2 census identifies the same characteristic polynomial of Frobenius, `X¹⁴+9X⁷+128` for the minus pair (and `X¹⁴−9X⁷+128` for the plus pair).

For the coefficient presentation, let `σ0(x)=x²` on K0 and `σ_s(x)=x^(2^s)` on K_s. Restricted to K0, `σ_s=σ0^s`. On the order-seven Frobenius eigenspaces, multiplication of eigenvalue exponents by s permutes the two cubic cyclotomic orbits `{1,2,4}` and `{3,5,6}` because s is a unit modulo seven. The q2 witness b=8 has a W1 coefficient presentation `b=(r(v+1))²` with `r,v` in one old cubic kernel; this remains valid over the new kernel. Independently, b=8 has a W0 product presentation in the same cubic kernel, as checked in `../ghs_n7_isogenous_w0_20261115/`. The absolute trace of `a=1` from F_(2^(7s)) to F2 is `s mod 2`; that of `a=0` is zero. Thus the W1 coefficient condition alone does not make a curve W1. At even s the two twists are isomorphic over K_s. These facts use the family definitions in Menezes–Teske and do not assert that the resulting Hess descent map reaches Y_s.

Let φ0 be any isogeny `A0→J(Y_-)` of degree d. Its base change φ_s has the same degree. For a prime ℓ not dividing d, φ_s is injective on the ℓ-primary rational subgroup; under `A_s(k_s)=E_s(K_s)` this is a **mathematical subgroup-preserving map**. Neither φ0 nor d has been constructed, so this statement does not supply an evaluable map, target recovery, or a full attack-cost comparison.

## Trace and genus checks

If π and π̄ are the F128 Frobenius eigenvalues of E_-, with `π+π̄=-9` and `ππ̄=128`, the K_s trace is `t_s=π^s+π̄^s` and obeys `t_0=2`, `t_1=-9`, `t_s=-9t_(s-1)-128t_(s-2)`. Over Fq, `J(Y_s)` has Frobenius polynomial `X¹⁴-t_sX⁷+q⁷`, and its first seven curve counts are `N_m=q^m+1` for 1≤m<7 and `N_7=q⁷+1-7t_s`. At s=2 this predicts `t_2=-175` and `N_7=17610`; the native q4 replay in this directory checks the counts directly.

The genus remains seven. This is a fixed base-change line inside the **known W0/W1 families**, not a newly established GHS-vulnerable family. The fixed isogeny whose existence follows from Tate could in principle be reused across s, but its degree, rational formulas, evaluation complexity, and action on the chosen source subgroup are not known. An asymptotic index-calculus exponent alone is insufficient to claim a practical attack.

The separate [simplicity and genus-floor proof](GENUS_FLOOR.md) strengthens this statement: for every `s` coprime to seven, the Weil restriction is simple, so every nonzero algebraic homomorphism from it to a Jacobian has target genus at least seven. The genus-seven carrier meets this lower bound at the isogeny-class level. This is a bound on transfer geometry, not a completed attack.

## Prior-work boundary

The q2 +13 carrier with `v=0xb9,u=0x8051` appears explicitly in the earlier `research/ic_next10_20261014/10-torelli-toy/README.md`; that class is a rediscovery. The +9 source is a published W0 case with a direct Hess genus-seven descent. The −9 W1 isogeny-class match is a bounded geometry lead, but the predecessor's exhaustive model enumeration may have traversed its equation. The base-change theorem is an application of standard Weil-restriction functoriality and finite-field tensor products. No new eligible GHS family or attack method is claimed. [pcc:unverified]

## Formalization boundary

The q2 trace intersection, sample support counts, and witness arithmetic are kernel-checked in `../ghs_n7_prym_trace_20261115/HyperellipticTraceLedger.lean`. This theorem needs Lean definitions for finite-field tensor products, Weil restriction, Jacobians, Tate isogeny, and W1 cubic-kernel presentations. None is supplied by the standalone `Init` environment used in that file, so the full theorem is currently paper-level. A bounded Lean recurrence lemma can check the q4 trace arithmetic, but it would not formalize the geometric isogeny.
