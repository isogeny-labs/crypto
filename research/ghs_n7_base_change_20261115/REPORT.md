# Coprime base-change carrier line from the ±9 W0/W1 pair

15 November 2026. [Theorem and proof](THEOREM.md), [frozen protocol and amendment](PROTOCOL.md), [q4 target raw output](reports/run-20261115T0918Z/result.txt), [q4 source/W1 raw output](reports/run-20261115T0918Z/source.txt), and [run receipt](reports/run-20261115T0918Z/RUN.md). The geometric claim is awaiting independent PCC review. [pcc:unverified]

## Result and novelty boundary

The earlier [q2 census](../ghs_n7_prym_trace_20261115/HYPERELLIPTIC_CLASSIFICATION.md) found a genus-seven hyperelliptic curve Y/F2 whose Jacobian has the same Weil polynomial as the restriction of scalars of two elliptic curves E/F128 with traces ±9: the minus source is W1 and the plus source is W0. For each positive s coprime to seven, base-change E to F_(2^(7s)) and Y to F_(2^s). Finite-field linear disjointness and base change of Weil restriction imply `J(Y_s) ~ Res(E_s)` over F_(2^s). The cubic-kernel coefficient presentations persist, possibly swapping arms. Absolute `Tr(a)` changes under even-degree base change, so the minus source is W1 for odd s and W0 for even s; the plus source is W0 throughout. This constructs an **infinite isogeny-class carrier line inside published W0/W1 families**. It does not construct a new GHS transfer map or establish a lower-cost DLP method.

![Base-change tower and open map](base-change-tower.svg){width=85%}

The source pair is `E_-(a,b)=(1,8)` with t=-9 and `E_+(a,b)=(0,8)` with t=9 over `F2[z]/(z⁷+z+1)`. The target pair is `Y_-(v,u)=(0x8f,0xc07a)` and `Y_+(v,u)=(0x8f,0x802f)` over F2, where bit i is the coefficient of x^i in `y²+v(x)y=u(x)`. These are exact curves. A fixed F2-isogeny exists by Tate because the Weil polynomials match; its degree and rational formulas have not been found. For every prime ℓ outside that fixed degree, its base change injects the ℓ-primary source group into the target Jacobian mathematically, but there is no executable subgroup map in this study.

An [exact genus-floor theorem](GENUS_FLOOR.md) shows that the base-changed Weil restriction is simple over `F_(2^s)` precisely when `7∤s`. Its proof uses the quadratic field `Q(√−431)`: the source Frobenius has norm `2⁷` but cannot be a seventh power because that field has no algebraic integer of norm two. Capelli's criterion then makes the degree-14 Weil polynomial irreducible for every `s` coprime to seven. Any nonzero algebraic map from the restriction to a Jacobian must therefore have target genus at least seven. The displayed `Y_s` meets this **geometric** lower bound at the isogeny-class level. SageMath 10.10.rc0 checked both signs through s=10, including the expected split at s=7; the failed first validation assertion and corrected run are preserved in `reports/simplicity-20261115/`. The theorem is paper-level and awaits independent review. [pcc:unverified]

## q=4 independent check

At s=2, the trace recurrence from t_1=±9 and q2 extension size 128 gives `t_2=t_1²−256=−175`. Both source twists therefore have predicted order `4⁷+1−t_2=16560`. The new native source code independently embedded F128 into `F2[z]/(0x4021)`, found `w=0xd2e` satisfying `w⁷+w+1=0`, mapped b=8 to `w³=0x1dee`, and counted **16,560** points on each E. It found the W1 coefficient presentation with old and new cubic arm 2; at s=2 both twists have absolute `Tr(a)=0`, so this does not make either curve W1: `r=0x2549`, `v=0xd30` in the q4 field.

| m | Target extension | Predicted and measured N_m for both Y twists | Check |
|---:|:---|---:|:---|
| 1 | F4 | 5 | direct `(x,y)` and trace |
| 2 | F16 | 17 | direct `(x,y)` and trace |
| 3 | F64 | 65 | direct `(x,y)` and trace |
| 4 | F256 | 257 | direct `(x,y)` and trace |
| 5 | F1024 | 1025 | direct `(x,y)` and trace |
| 6 | F4096 | 4097 | direct `(x,y)` and trace |
| 7 | F16384 | 17610 | Artin–Schreier trace |

The seven counts determine the full genus-seven Weil polynomial `1+175T⁷+4⁷T¹⁴`, which matches the source restriction-of-scalars polynomial. This replay checks one nontrivial tower instance; the all-s statement follows from the algebraic argument in `THEOREM.md`.

## Prior work and proof status

The W0/W1 source families are in [Menezes–Teske, Sections 5.1–5.2](https://cacr.uwaterloo.ca/techreports/2004/corr2004-25.pdf). The hyperelliptic enumeration method is in [Borodin–May, Section 4](https://arxiv.org/pdf/2508.16886). An earlier internal `research/ic_next10_20261014/10-torelli-toy/README.md` already produced the ±13 genus-seven carriers; its explicit `v=0xb9,u=0x8051` is the +13 class in the new q2 census. That is a rediscovery. The +9 source is a direct published W0 genus-seven case. The predecessor did not report the −9 W1 hyperelliptic carrier, though its all-model search may have traversed that equation. Base change of Weil restriction is standard. The −9 isogeny-class match remains a bounded geometry lead, subject to wider source and data review; it is not a new eligible family. [pcc:unverified]

**Evidence status.** The q2 ±9 target class has a finite census and separate direct replay; its geometric reduction awaits review. The q4 source and target arithmetic were checked by two native programs; the q4 coefficient presentation is not W1 membership because absolute `Tr(a)=0`. `CoprimeBaseChangeTrace.lean` passed the Lean 4.35.0 proof gate with three theorems and zero trusted exceptions. The all-s carrier and genus-floor arguments are paper proofs; their geometric statements are not formalized in Lean. Explicit isogeny formulas, subgroup evaluation, and complete GHS/rho costs remain open.

The q2 group orders are 138 and 120; their q4 base changes both have order 16,560. These first two rungs are geometry and membership controls. Any security claim at larger s would require a large chosen source subgroup, an explicit nonzero map, full target recovery, and a matched total-cost benchmark against rho. The source family label follows absolute `Tr(a)`, and a Jacobian in the same isogeny class need not be the endpoint of the published GHS cover map.

For every `s>1`, these exact source equations still have coefficients in the proper subfield `F128⊂F_(2^(7s))`. This fixed-coefficient base-change line should not be represented as a family of generic full-field curves. When `gcd(s,7)=1`, `F128∩F_(2^s)=F2`, so the coefficient `b=z³` does not lie in the base field `F_(2^s)`; the relative degree-seven orbit remains nontrivial. The smaller field of definition is nevertheless part of the workload identity and must be charged in any matched rho or endomorphism baseline.
