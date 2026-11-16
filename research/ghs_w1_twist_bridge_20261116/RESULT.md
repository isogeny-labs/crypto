# Trace -9 W1 genus-seven transfer: twist obstruction and a prime-to-23 existence certificate

16 November 2026. The exact binary field is `K=F2[z]/(z⁷+z+1)`. Put

* `E_+: y²+xy=x³+z³`, trace +9 and order 120, in the published W0 direct genus-seven family;
* `E_-: y²+xy=x³+x²+z³`, trace -9 and order `138=6·23`, in W1;
* `Y_+/F2: y²+v(x)y=u_+(x)` and `Y_-/F2: y²+v(x)y=u_-(x)`, where bit masks are `v=0x8f`, `u_+=0x802f`, `u_-=0xc07a`.

The bit-polynomial check gives `u_-+u_+=v²=0x4055`, so `Y_-` is the constant quadratic twist of `Y_+`. The source pair is also a constant quadratic twist. This exact coincidence motivated checking whether the already evaluated W0 genus-seven map could twist into an evaluated W1 genus-seven map.

## The direct twist bridge fails for the recorded W0 target

The saved W0 map lands in a **plane** genus-seven curve `C_+`, not the separate hyperelliptic `Y_+`. A fresh `WeilDescent(E_+,F2,z)` generated another plane model. Magma 2.30-1 returned `IsHyperelliptic=false` and `IsGeometricallyHyperelliptic=false` for both the fresh output and the **exact homogeneous equation from the saved map receipt**. The latter equation was copied verbatim and source-bound; [`audit.py`](audit.py) compares its normalized terms with the earlier W0 receipt. The complete responses are [`check_w0_geometry.json`](check_w0_geometry.json) and [`check_previous_target.json`](check_previous_target.json). [pcc:unverified]

There is a geometric reason this matters. If an involution `i` on a genus-at-least-two curve induced `[-1]` on its Jacobian, then `[i(P)-i(P₀)]=-[P-P₀]` would make the degree-two divisors `P+i(P)` linearly equivalent as `P` varies. This gives a degree-two map to `P¹`, so the curve is hyperelliptic. Thus the recorded nonhyperelliptic `C_+` has no curve involution implementing the sign twist on its Jacobian. Twisting the W0 abelian-variety homomorphism by sign is possible abstractly, but it does not produce a curve twist of this recorded `C_+`. This falsifies the proposed *direct* curve-twist shortcut; it does not exclude a separate isogeny from the twisted abelian variety to `J(Y_-)`, another W0 cover model, or other W1 constructions.

## An order-23 subgroup-preserving isogeny exists abstractly

An independent Sage replay recounts `#E_-(K)=138` and the seven carrier point counts `[3,5,9,17,33,65,192]`. Newton identities and the genus-seven functional equation give the full Frobenius polynomial

`P(X)=X¹⁴+9X⁷+128`

for `J(Y_-)`, equal to that of `A=Res_(K/F2) E_-`. The same run checked that `P` is irreducible modulo 13, that `P(1)=138`, and that

`disc(P)=−2⁴²·7¹⁴·431⁷`,

which is coprime to 23; equivalently `P mod 23` is squarefree. Its SHA-bound receipt is [`prime23_inputs.json`](prime23_inputs.json). The first run's output-serialization error and the output-only amendment are retained in [`AMENDMENT.md`](AMENDMENT.md). [`W1Prime23Arithmetic.lean`](W1Prime23Arithmetic.lean) passed the repository proof gate with `9 thm, 0 trusted` and checks the twist-mask, order, discriminant-factor and selected extension arithmetic. Lean does not formalize the zeta calculation or the isogeny theorem. [pcc:unverified]

Here is the paper-level isogeny argument. Since `P mod 23` is squarefree, `Z₂₃[X]/(P)` is an etale algebra. Each 23-adic Tate module of `A` and `J(Y_-)` is rank one over this algebra: both have characteristic polynomial `P` with each residual factor appearing once. Thus the Tate modules are Frobenius-equivariantly isomorphic over `Z₂₃`. [Tate's integral homomorphism theorem](https://pazuki.perso.math.cnrs.fr/index_fichiers/Tate66.pdf) identifies such homomorphisms with the 23-adic completion of `Hom_F2(A,J(Y_-))`; reducing modulo 23 and choosing an integral representative yields an F2-isogeny whose degree is **prime to 23**. It is therefore injective on the exact order-23 subgroup `E_-(K)[23]≅A(F2)[23]`. The published W1 direct-cover genus is fourteen; this theorem certifies a subgroup-preserving **genus-seven Jacobian route by existence** for this exact source. It does not provide the isogeny's coordinates, evaluated images, target recovery or an attack-cost comparison.

The [existing coprime-base-change theorem](../ghs_n7_base_change_20261115/THEOREM.md) extends the isogeny-class match through `F_(2^s)` for `gcd(s,7)=1`; base change of the prime-to-23 isogeny preserves the inherited order-23 subgroup. For odd `s` these sources retain W1 type. More generally, let `t_0=2`, `t_1=-9`, `t_s=-9t_(s-1)-128t_(s-2)`, and `N_s=2^(7s)+1-t_s`. **For `gcd(s,7)=1` and any prime `r|N_s` with `r∤2·7·(t_s²−4·2^(7s))`, the same etale Tate-module argument yields an `F_(2^s)`-isogeny of degree prime to `r`**, hence an abstract subgroup-preserving genus-seven route. This is a criterion on the known W1 base-change line, not an algorithm for finding the isogeny.

The frozen [extension screen](extension_primes.json) checked seven odd coprime values of `s`, exact order factorizations, primality and the discriminant condition. Its factors and recurrence were independently replayed by Python in [`AUDIT.json`](AUDIT.json); Lean checked the `s=11` recurrence, order factorization and coprimality.

| `s` | largest selected prime `r` | bits | discriminant gcd |
|---:|---:|---:|---:|
| 1 | 23 | 5 | 1 |
| 3 | 5,059 | 13 | 1 |
| 5 | 22,635,091 | 25 | 1 |
| 9 | 194,607,397 | 28 | 1 |
| 11 | 5,506,013,662,841,737 | 53 | 1 |
| 13 | 6,343,759,259,552,287 | 53 | 1 |
| 15 | 1,275,204,272,659,381 | 51 | 1 |

At `s=11`, `N_s=151115727452588594488986=2·3·23²·8647·5506013662841737`. The 53-bit prime is a concrete construction target. The source coefficients remain in the proper subfield `F128`, so a later same-target rho comparison must account for its source-field automorphisms. The selected-prime table is **arithmetic and abstract transfer existence**, not a measured GHS attack or vulnerability verdict.

## Prior work and novelty boundary

[Hess 2004](https://www.cambridge.org/core/services/aop-cambridge-core/content/view/315278965D8D277A85812D9498236A88/S146115700000108Xa.pdf/generalising-the-ghs-attack-on-the-elliptic-curve-discrete-logarithm-problem.pdf) supplies the W0/W1 descent and norm-conorm framework; [Menezes–Teske](https://cacr.uwaterloo.ca/techreports/2004/corr2004-25.pdf) define the W0 and W1 source loci. The separate hyperelliptic genus-seven `Y_-` and its isogeny-class match were already recorded in this repository's [finite census](../ghs_n7_prym_trace_20261115/HYPERELLIPTIC_CLASSIFICATION.md). The prime-to-23 conclusion is a new **exact subgroup-existence certificate in this project**, derived from [Tate 1966](https://pazuki.perso.math.cnrs.fr/index_fichiers/Tate66.pdf); it is neither a newly identified source family nor a new attack method. Targeted exact-equation searches did not locate a publication of this particular certificate, but that does not establish publication priority. Constructing and evaluating the isogeny remains the decisive gate before a GHS-versus-rho measurement.
