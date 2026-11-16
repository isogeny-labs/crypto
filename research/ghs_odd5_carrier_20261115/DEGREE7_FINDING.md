# Five explicit degree-seven covers and prime-subgroup transfers

16 November 2026 UTC. All finite enumeration claims remain [pcc:unverified] until independent PCC admission. The saved equations themselves admit direct replay in Sage and Magma. This note supersedes the earlier statement in `PRIOR_WORK_AUDIT_20261115.md` that the trace-26 slice has no evaluable cover.

## Exact objects and bound

Set `K=F3[a]/(a^5+2a+1)` and `Y/F3: y²=f(x)`, where

`f=x^11+x^9+2x^8+x^7+2x^5+x^3+x^2+x+1`.

For each mask `m`, write `b_m=sum_i floor(m/3^i) mod 3 * a^i` and `E_m/K: v²=u³+2u²+b_m`. The ten trace-`+26` source curves have masks `3,27,30,103,139,167,182,194,230,233`, with `#E_m(K)=218=2*109`. The quadratic-square screen found one explicit cover for each of **30, 103, 139, 194, 233** and none in its specified denominator shapes for the other five. The exact `A,B,C,H` for each hit are in `degree7_quadratic_square_<mask>.json`; the `degree7_evidence_manifest.json` binds each source and receipt hash.

| Mask | Exact `b_m` | Pole quadratic index in the saved factor order | Minimum cover degree for this fixed `Y` |
|---:|---|---:|---:|
| 30 | `a+a³` | 3 | 7 |
| 103 | `1+a+2a²+a⁴` | 1 | 7 |
| 139 | `1+a+2a³+a⁴` | 2 | 7 |
| 194 | `2+a+a³+2a⁴` | 4 | 7 |
| 233 | `2+2a+a²+2a³+2a⁴` | 0 | 7 |

**Orbit correction.** These five K-equations form one Frobenius orbit:

`30 → 194 → 103 → 139 → 233 → 30` under `b↦b³`.

The other five targets form the orbit `3 → 27 → 167 → 182 → 230 → 3`. The five saved positive covers themselves are Frobenius conjugates: their A and B coefficient vectors match those of mask 30 after the indicated field automorphism; C matches up to sign. Thus there are **five exact K-model records but one independent positive Frobenius orbit**, not five unrelated families. `frobenius_orbits.sage`, public Magma 2.30-1 (`frobenius_orbits_magma.json`), `cover_frobenius_orbit.sage`, and two axiom-free Lean modulus-reduction theorems verify this partition and construction relation. The negative orbit has no hit in the specified quadratic-square degree-seven shapes; this is not a complete degree-seven exclusion.

Each positive cover has `u=A/B`, `v=y*C/B²`, `B=Q_i H²`, `deg A=7`, `deg B=6`, and satisfies the exact polynomial identity

`f*C² = B*(A³+2A²B+b_m B³)`.

The five identities, degrees, smoothness and coprimality passed a separate Magma 2.30-1 replay (`degree7_hits_replay.magma`, `degree7_hits_replay_magma.json`), with successful, untruncated output. The earlier complete degree-2-through-6 exclusion for this **fixed Y and ten E_m** makes the minimum `K`-cover degree exactly **7** for the five positive masks. This is a scoped minimum; it does not classify other genus-five carriers or all degree-seven maps to the five remaining curves. The Howe bound still gives a cover of degree at most 20 for every target in this ten-curve slice.

## Evaluated subgroup transfer

For `P in E_m(K)[109]`, let `(U,V)` represent the pullback fiber over `P`, where `U=(A-u(P)B).monic()` and `V=v(P)B²/C mod U`. If `Q_i` is the quadratic branch factor of `B`, the pole fiber relative to `7∞` is the 2-torsion class `(Q_i,0)`. The corrected pullback is `D(P)=(U,V)-(Q_i,0)`. The small-field map is

`T(P)=sum_{j=0}^4 Frob_3^j D(P) in J(Y)(F3)[109]`.

For each of the five masks, `degree7_subgroup_map_<mask>.json` records a source point, its independent `37P` preimage, nonzero order-109 `D(P)` and `T(P)`, Frobenius fixedness of `T(P)`, and both scalar identities `D(37P)=37D(P)` and `T(37P)=37T(P)`. The exact function-field identity plus divisor pullback functoriality supplies the group-homomorphism argument; the point checks certify the selected subgroups. Lean's arithmetic ledger does not formalize that geometric argument.

The same covers base-change to `L=F_(3^55)` over `k_s=F_(3^11)`. For each mask, `degree7_s11_transfer_<mask>.json` records a nonzero image of a source point of prime order `r=333566377062870325133` (69 bits) in `J(Y)(k_s)[r]`, fixed by `Frob_(3^11)`, with independently evaluated scalar preservation at 37. The source and Jacobian orders are `174449211009094048378706606=2*109*2399*r`, hence the Jacobian group at this exact `s=11` is cyclic. These are executable subgroup transfers for five exact curves. A complete measured attack at this 69-bit scale remains open.

The separate `primitive_cover_s11.json` gives an additional exact curve `E'/F_(3^55)` with `j(E')` of full degree 55 over F3. An evaluable 23-isogeny from the mask-30 source produces `E'`; the saved reverse 23-isogeny maps its order-r point back to `23P`, then the same corrected pullback and trace give a nonzero order-r Jacobian image. `verify_primitive_cover_s11.py` independently rebuilt both isogenies from their kernel polynomials, reconstructed the cover from coefficient vectors, and matched the saved image and its 37-multiple. Thus the exact composite transfer is executable for a primitive-j source; it remains in the known isogeny-assisted attack setting. Composing the forward 23-isogeny with `Y_L→E_30` also gives a degree-161 cover `Y_L→E'`, although its minimum degree is not determined here.

## Cost and priority boundary

[Diem 2006](https://arxiv.org/abs/math/0606607) proves expected `O-tilde(q^(2-2/g))` for fixed-genus curves over `F_q` given by bounded-degree plane models when the degree-zero class group is cyclic. At `g=5`, the exponent is `8/5`. Our `s=11` Jacobian satisfies the cyclicity hypothesis, but the theorem is asymptotic and supplies no measured time or concrete constant for this 69-bit instance. A matched source rho comparison must include the degree-five Frobenius action and any isogeny transport. The complete `s=3`, mask-30 pipeline in `S3_FULL_ATTACK_RESULT.md` recovered its target but measured **1.653 s GHS versus 0.144 s matched rho** on the same order-65,827 subgroup, with rho faster by about 11.5 times. That result does not determine the `s=11` outcome. No practical speedup is claimed.

The established mechanisms of odd-characteristic weak coverings, cofactor-two elliptic sources, isogeny-assisted transfer, and small-genus index calculus appear in [Iijima--Momose--Chao 2013](https://eprint.iacr.org/2013/487.pdf), [Tian--Li--Wang--Yu 2018](https://doi.org/10.1007/s10623-018-0457-6), [Tian 2020](https://arxiv.org/abs/2012.07173), and [Fan et al. 2022](https://doi.org/10.1155/2022/9833393). A targeted search for this exact `f`, the characteristic-three degree-seven genus-five construction, and the five exact equations found no matching publication, but that search cannot establish worldwide priority. Label these **new exact cover and transfer records in this project, representing one positive Frobenius orbit**. A globally new eligible family or new attack method is unconfirmed. The five base equations have coefficients in the proper subfield at `s=11`; the separately constructed primitive-j isogeny neighbor has an explicitly composed transfer but remains in the known isogeny-assisted setting.
