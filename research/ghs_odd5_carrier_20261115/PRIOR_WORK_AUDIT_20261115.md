# Prior-work audit for the odd-five carrier lead

16 November 2026 UTC. This audit records the scope of the sources actually checked. An unavailable full text or an unsuccessful exact-equation search is not evidence of publication priority.

| Proposed differentiator | Checked prior work | Consequence for this project |
|---|---|---|
| Odd-characteristic prime-degree direct `(2,...,2)` covers | [Iijima--Momose--Chao 2013](https://eprint.iacr.org/2013/487.pdf); [2015 indexed abstract](https://eprint.iacr.org/2015/805.pdf) states complete elliptic classification for degrees 2, 3, 5, 7; full 2015 PDF returned HTTP 403 | Direct-cover eligibility is a published problem. The local 100-class result concerns a different isogeny-class carrier and does not supersede the direct-cover classification. |
| Cofactor-two elliptic source, degree-three hyperelliptic cover | [Tian--Li--Wang--Yu 2018](https://doi.org/10.1007/s10623-018-0457-6); indexed abstract and the discussion in [Tian 2020, introduction](https://arxiv.org/pdf/2012.07173) checked; 2018 full text unavailable here | Cofactor two is already a known attack setting. It does not identify the ten degree-five curves as vulnerable. |
| Weil restriction to Jacobian by an explicit isogeny without a curve cover | [Tian 2020](https://arxiv.org/pdf/2012.07173), full paper checked, cubic extension and genus-three Jacobian with image recovery | The transfer mechanism is prior art. A degree-five formula, if found, would require a separate comparison of construction and performance. |
| Prime-order source with hyperelliptic carrier and decomposition | [Fan--Fan--Song--Wang 2022](https://doi.org/10.1155/2022/9833393), full open-access article checked, quadratic extension and genus-two carrier | Prime order and hyperellipticity alone do not define a new family or method. |
| Exact `F3/F243` eight-trace genus-five Jacobian identification | Local LMFDB snapshot and connected mirror as detailed in `THEOREM.md`; targeted exact-equation searches | Six Weil-polynomial classes were catalog-known Jacobian classes; two had uncertain flags. The two explicit identifications are new relative to those catalog flags. Worldwide priority is unresolved. |
| Ten trace-`+26` targets and the fixed genus-five carrier | `TRACE26_CLASS_THEOREM.md` and its independent degree-four, five, six replays | A computer-assisted **cover obstruction** is established for K-defined degrees at most six, with a Howe existence ceiling twenty. This does not by itself identify a vulnerable family. |

## Distinct claims and the next construction gate

1. **New exact curve:** The two catalog-uncertain Jacobian identifications and the ten exact source equations may be new as explicit records. Exact-equation publication priority has not been established.
2. **New eligible family:** The 100 ordinary classes over `F243` share a genus-five Jacobian *isogeny class* carrier and a direct-cover genus-floor separation. This is a finite eligible stratum, not yet a demonstrated attack family. The all-`s` coprime-to-five base changes supply an infinite sequence of carrier instances, but each source is defined over the proper subfield `F243`; that matters for rho and for any novelty claim.
3. **New attack method:** None is established. Cubic explicit isogeny transfer, cofactor-two covers, and quadratic prime-order hyperelliptic covers are prior art.

For a `K=F243` cover `phi:Y_K -> E_b`, the push-pull identity proves `phi^*` is injective on the order-109 source subgroup when `deg(phi)<109`. An attack using the **small-field** Jacobian still needs a nonzero `F3`-rational image. A concrete route is to evaluate the Galois trace

`T(phi^*(P)) = sum_(i=0)^4 Frob_3^i(phi^*(P)) in J(Y)(F3)[109]`.

This trace could vanish even when `phi^*(P)` is nonzero: the pullback is only K-rational. Thus each proposed cover or correspondence must certify its action on a chosen source generator, the nonzero small-field image, and scalar preservation. A zero trace falsifies that particular transfer route, although a different correspondence may work. Then benchmark construction, map evaluation, relation/decomposition, linear algebra, target descent and final verification against a Frobenius-aware matched rho run on the same source subgroup.

The next targeted search is degrees 7 through 20 for the fixed `Y` and ten `E_b`, or an evaluable F3-rational isogeny `Res E_b -> J(Y)`. The former is bounded by the verified finite theorem; the latter requires an explicit correspondence, not just a Weil-polynomial match. No practical-vulnerability or globally-new-family label is supported yet.

## New exact primitive source after this audit

`PRIMITIVE_S11_PROTOCOL.md` and `primitive_isogeny_s11.json` record a full-degree-55 `j` source reached by an evaluable 23-isogeny at `s=11`. Its 69-bit subgroup survives the saved reverse map, independently reconstructed in `verify_primitive_s11.py`. This removes the proper-subfield model for that **exact source**, while retaining its membership in the known isogeny-assisted transfer setting. A matched rho comparison can also use the 23-isogeny to move back to the descended curve and exploit its Frobenius action. The Jacobian map and full cost comparison remain the decisive open gates. [pcc:unverified]
