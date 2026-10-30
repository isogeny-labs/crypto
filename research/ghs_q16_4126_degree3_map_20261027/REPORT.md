# Order-4126 cubic transfer: degree-three cover exclusion for listed hyperelliptic models

27 October 2026. Conductor T-5502. The [one-source exhaustive protocol](PROTOCOL.md),
[one-source certificate](certificate.json), [all-source protocol](ALL_SOURCES_PROTOCOL.md),
[all-source certificate](all_sources_certificate.json), and
[second-source exhaustive replay](replay_378_certificate.json) preserve
the exact scope and inputs.

## Question and prior work

The preceding [GF(16) Hess-closure audit](../ghs_cubic_hess_closure_q16_20261027/REPORT.md)
isolated all 108 normalized ordinary elliptic equations of order
`4126=2·2063` outside Hess's cubic genus-three isogeny closure. Their
Weil-restriction class over `k=GF(16)` contains a genus-three Jacobian:
[LMFDB class 3.16.a_a_bd](https://www.lmfdb.org/Variety/Abelian/Fq/3/16/a_a_bd)
lists 140 hyperelliptic models, and the earlier screen verified a
separate smooth plane quartic. A matching Weil polynomial gives an
isogeny-existence statement, not its coordinates.

Generalized binary GHS and isogeny walks are already in
[Hess (2004)](https://doi.org/10.1112/S146115700000108X) and
[Menezes--Teske (2004)](https://cacr.uwaterloo.ca/techreports/2004/corr2004-25.pdf).
[Tian--Li--Wang--Yu](https://doi.org/10.1007/s10623-018-0457-6)
studied cofactor-two degree-three covers in odd characteristic;
[Tian's cubic isogeny construction](https://arxiv.org/pdf/2012.07173)
and [Frey--Kani correspondences](https://doi.org/10.1007/978-3-642-25261-7_1)
are additional known transfer routes. This screen asks whether a
**separable degree-three curve map** from any one of the 140 listed
hyperelliptic curves to any one of the 108 exact source equations can
supply an immediately evaluable transfer.

## Complete finite map criterion

Take `K=GF(4096)` in the same Conway basis, a source
`E: v²+uv=u³+a2*u²+a6`, and an archived target
`C: y²+h(x)y=f(x)` over `k`. All 140 listed `h` polynomials have four
distinct roots over `K`, with nonzero branch residues. Put `z=y/h(x)`.
The [preceding exact Artin--Schreier proof](../ghs_automorphism_screen_q16_20261026/REPORT.md)
shows that a separable degree-three map `C_K→E` exists exactly when
there are a degree-three rational function `R∈K(x)` and `t∈K(x)` with

```text
t²+t = f/h² + R + a2 + a6/R².
```

The resulting map is `u=R(x)`, `v=R(x)(z+t(x))`. Four simple branch
points admit exactly two degree-three zero/pole patterns: 24
four-support assignments, or `4·2·4093=32,744` five-support
assignments with one unbranched double point. A constructed map of
each pattern is recovered by the code.

The five-support screen avoids iterating all 4,093 possible hidden
points. Fix the lone simple branch point `s`, let the other three be
`r_j`, let `L_s,L_j` be the Artin--Schreier branch residues, and put
`b=√a6`, `p=∏_j(s−r_j)`, and
`A_j=(r_j−s)/∏_(l≠j)(r_j−r_l)`. For either fiber orientation, eliminating
the uniquely determined scalar gives

```text
c_j := L_j L_s / (b p A_j)
    = (r_j−h)²/(s−h)² = (r_j²+H)/(s²+H),  H=h².
```

For a finite hidden point, `c_1=1` is impossible because `r_1≠s`.
Otherwise its **only** possible value is
`H=(r_1²+c_1*s²)/(c_1+1)`. The code checks that its square root is
outside the four branch points, tests the other two residue equations,
then tests the full displayed Artin--Schreier equation for both
orientations. The hidden point at infinity is checked separately by
`c_1=c_2=c_3=1`. Thus every five-support assignment is either ruled
out by a necessary identity or checked as an exact map candidate.
The four-support pattern similarly filters by the necessary residue at
its simple pole before the full equation.

## Measured finite census

The exact source list and every `a6` code are retained in the
[all-source certificate](all_sources_certificate.json). All 108 sources
have `a2=32` and order 4126. Every one of the 140 cataloged targets
passes the genus-three and four-branch gates.

| Gate | Exact result |
| :--- | ---: |
| Exact source/model pairs | **15,120** |
| Four-support assignments checked | **362,880** |
| Four-support assignments passing necessary residue | **96** |
| Five-support assignments covered algebraically | **495,089,280** |
| Finite hidden-point candidates computed | **60,444** |
| Five-support assignments passing all branch residues | **0** |
| Degree-three maps passing the full equation | **0** |

The number 495,089,280 counts the finite and infinity assignments
covered by the proven elimination, **not** that many explicit
rational-function evaluations. The 96 four-support residue candidates
were tested against the full Artin--Schreier equation and failed.

The original hidden-point enumerator explicitly tested all 4,584,160
five-support assignments for source `a6=63` against all 140 models;
it agreed with the fast result. A second exhaustive replay did the
same for source `a6=378`, where two four-support candidates survive
the quick residue filter; again no map was found. Both constructed
positive controls pass the original and fast enumerators. The frozen
certificates hash the prior catalog, source census, and exact scripts.

The earlier [degree-two screen](../ghs_automorphism_screen_q16_20261026/REPORT.md)
already excludes separable degree-two elliptic quotients over `K` for
all 140 listed models (four sparse and 136 nonsparse). Combining the
two exact screens yields this **route-specific lower bound**:

> For each of the 15,120 pairs consisting of one of the 140
> catalog-listed hyperelliptic curves in class `3.16.a_a_bd` and one
> of the 108 normalized ordinary elliptic curves of order 4126,
> every separable curve map over `K` from the former to the latter
> has degree at least four.

This does not exclude a map from the separately verified plane
quartic, an unlisted hyperelliptic polarization, an isogeny-only
correspondence, or a higher-degree cover. In particular, it does not
show that the order-4126 subgroup is vulnerable or secure under all
transfer attacks. The order-4126 source-to-Jacobian map has not been
evaluated, and its full GHS cost versus rho is unknown.

## Novelty labels and checked proof scope

The 108 exact elliptic equations and 140 cataloged target equations
are all specified by the certificates. Their LMFDB isogeny class and
the underlying degree-three-cover method are prior work; a bounded
literature check cannot establish publication novelty of every exact
equation. The new result here is the **complete finite degree-three
exclusion for this specified 15,120-pair cohort**, not a newly
vulnerable family. [CoverCounts.lean](CoverCounts.lean) checks the
integer partitions. Lean does not formalize the function-field
classification, residue elimination, or the Sage computations.
