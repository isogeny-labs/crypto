# Complete Hess-parameter scan and evaluated genus-14 transfer on the −9 W1 source

15 November 2026. [Frozen protocol and amendments](PROTOCOL.md), [full Magma scan response](scan_receipt.json), [validated summary](scan_summary.json), [map response](map14_receipt.json), and [independent kernel input check](kernel_bound.txt). [pcc:unverified]

## Exact source and prior work

The source is `E/F128: y²+xy=x³+x²+z³`, `z⁷+z+1=0`, with 138 points and trace −9. It belongs to the **published** n=7 W1 family. The separate genus-seven hyperelliptic `Y/F2` in `../ghs_n7_base_change_20261115/` has a Jacobian in the same isogeny class; no algebraic map to that Y was built here. [Magma's WeilDescent handbook](https://docs.magma-maths.org/ArithmeticGeometry/EllipticCurvesOverFiniteFields/ec_weil_desc.html) documents the parameterized GHS curve and divisor map. [Hess, *The GHS Attack Revisited*, Theorems 4–7](https://www.iacr.org/archive/eurocrypt2003/26560374/26560374.pdf) supplies the trace condition, genus and kernel statements used below. In §5.4 Hess also explains why changing among degree-two rational subfields by point translation does not create a new genus lever. This closes that apparent extension as a novelty lead.

## Complete parameter screen

Magma reconstructed the exact field polynomial and source group order. The pilot `c=1` had genus 63. Since `K×` has prime order 127 and `z≠1`, `c=z^i`, `i=0,...,126`, enumerates every nonzero parameter. The complete scan returned 127 rows, with 98 eligible parameters and 29 rejected by Magma's descent trace condition. The eligible genus histogram is:

| Genus | 14 | 63 | 105 | 112 | 119 | 120 | 126 | 127 |
|:---|---:|---:|---:|---:|---:|---:|---:|---:|
| Parameters | 2 | 2 | 2 | 12 | 12 | 12 | 38 | 18 |

The minimum is genus **14**, at `c=z^94` and `c=z^98`; no documented `WeilDescent(E,F2,c)` parameter reaches genus seven. This is a complete finite result for this **specific source, base field, and Magma/Hess parameter family**. It does not exclude a different correspondence or a map from a source isogenous to E.

## Evaluated subgroup map

At `c=z^94`, Magma returned a genus-14 plane curve (`CrvPln`) and evaluated its divisor map on the first exact order-23 point, `P=(1:z^105:1)`, and on the identity. The response contains both divisors. Since this is a plane curve, Magma returns divisors (`DivCrvElt`), not reduced Jacobian classes; the printed divisors alone do not establish their class orders.

An independent bit-field computation found `j(E)=z^124∉F2`, `r=sqrt(1/j)=z^65`, `c=z^94`, and `r/c=z^98`. Their Frobenius annihilators are `m_c(T)=T⁴+T²+T+1=(T+1)(T³+T²+1)` and `m_(r/c)(T)=T³+T²+1`; `Tr(c)=1` satisfies Hess's Frobenius-extension condition. The a₂=1 term adds `T+1`, already present, so `deg m_f=4`. As the source is not defined over the proper subfield F2, Hess Theorem 14 (from Theorem 7) confines the kernel of this conorm/norm class map to `2^(m−1)=8` torsion. Therefore its restriction to the order-23 subgroup is injective. This is a **theorem-based subgroup preservation check** for an evaluated, published genus-14 map. It is not a direct Jacobian-class order computation or target-log recovery. [pcc:unverified]

## Validation and limits

The input programs, Python HTTP runners, full responses and parser are in this directory. The first HTTPS attempt failed local certificate validation; a `certifi` CA bundle fixed it without disabling verification. The first scan stopped on an ineligible parameter; a protocol amendment added `try/catch` and retained that failure. The corrected scan response reports success and no truncation, and `summarize_scan.py` checks 127 distinct exponents, the histogram, and the absence of genus seven.

The map is an instance of the known W1 attack family. The genus-seven isogeny-class carrier has a sharp algebraic genus floor from `../ghs_n7_base_change_20261115/GENUS_FLOOR.md`, but the complete Hess parameter scan does not supply its correspondence. No index-calculus relation collection, decomposition, linear algebra, descent, or matched rho run is included here. A vulnerability claim for an overlooked family needs a source outside prior W1/other published classes, an evaluable subgroup map, and a complete same-target cost below rho.
