# Cyclic trigonal carrier obstruction in eight ordinary odd-five trace classes

**Theorem (application of established p-rank formulas).** Let C be a smooth genus-five curve in characteristic three whose L-polynomial is `1-tT^5+243T^10`, where `t∈{-26,-22,-14,-10,10,14,22,26}`. Then C has no geometrically cyclic degree-three morphism to a genus-zero curve. In particular, no nonhyperelliptic genus-five carrier in these isogeny classes can be obtained as a cyclic cubic Artin–Schreier cover of P¹. This eliminates one broad carrier construction route; it does not exclude non-Galois trigonal or nontrigonal carriers.

The middle coefficient is `-t`, never divisible by 3. The standard ordinarity criterion for a genus-five L-polynomial therefore gives 3-rank five. A geometric cyclic cubic cover of P¹ in characteristic three is Artin–Schreier after base extension. The Deuring–Shafarevich formula gives its 3-rank as `2(s-1)`, where s is the number of geometric poles in reduced Artin–Schreier form. That rank is even, so it cannot be five. The general formula and the fact that an ordinary Artin–Schreier locus need not exist are published in [Bucur–David–Feigon–Lalín, *Statistics for ordinary Artin–Schreier covers and other p-rank strata*](https://arxiv.org/pdf/1304.7876), Section 1. The theorem is a project-specific application of known results, not a claim that the p-rank criterion is new.

A second finite check uses point counts. In every such L-polynomial class, `#C(F3)=4` and `#C(F9)=10`, hence both counts are 1 modulo 3. A reduced genus-five Artin–Schreier equation has pole budget `Σ degree(P)·(order(P)+1)=7`. The only closed-pole partitions are:

| Pole orders and degrees | Rational poles over F3 | Pole points over F9 | Count obstruction |
|---|---:|---:|---|
| `(1,deg1)+(4,deg1)` | 2 | 2 | F3 count would be 2 mod 3 |
| `(2,deg1)+(1,deg2)` | 1 | 3 | F9 count would be 0 mod 3 |
| `(1,deg1)+(1,deg1)+(2,deg1)` | 3 | 3 | F3 count would be 0 mod 3 |

At each nonpole base point, the equation `y³-y=f(x)` has either zero or three solutions; at each pole it has one totally ramified point. Thus the curve's point count modulo three is the number of pole points rational over that field. Each of the three partitions contradicts a required count. `ArtinSchreierPoleArithmetic.lean` compiles with Lean 4.35.0 and checks the finite pole-type list, the candidate partitions, both residue conditions, `14 mod 3`, and the even-rank contradiction. The geometric genus, ordinarity, fibre-count, and Deuring–Shafarevich theorems are cited mathematical inputs external to Lean.

The read-only LMFDB `av_fq_isog` rows for the eight displayed polynomials all report 3-rank five. The row for `t=-14`, labelled `5.3.a_a_a_a_o`, is independently supported by the [point-count replay](../ghs_trace_minus14_nonhyperelliptic_lead_20261116/RESULT.md) on two explicit hyperelliptic carriers. The obstruction is at the isogeny-class level and survives every finite base change. It says nothing about a genus-greater-than-five carrier or a non-cyclic degree-three map. No newly vulnerable family or complete GHS/rho cost follows from it.
