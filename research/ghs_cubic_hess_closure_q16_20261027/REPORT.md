# Cubic Hess genus-three closure over GF(16): exact residual classes

27 October 2026. Conductor T-5491. The [frozen protocol](PROTOCOL.md),
[executable census](audit.py), and [complete certificate](certificate.json)
give the exact model and order lists. This is a structural transfer lead,
not an attack-cost or vulnerability result.

## Complete criterion within Hess's construction

Let `k=GF(16)`, `K=GF(4096)`, `q=16`, and
`E_(a,b): y²+xy=x³+a*x²+b` with `a` of absolute trace zero or one and
`b∈K*`. Put `δ=√b`, `U=ker Tr_(K/k)`, and `P=U* U*` (products of two
nonzero elements of `U`). Hess's Artin–Schreier representation has
`δ=γβ`. The only possible genus-three Frobenius-order pairs for
`γ,β` in a cubic extension are `(1,2)` and `(2,2)`: their Hess genera
are respectively `8−4−2+1=3` and `4−1−1+1=3`. A nonzero order-one
factor lies in `k*`, while a nonzero order-two factor lies in `U*`.
Thus the `(1,2)` products are exactly `U*`; the `(2,2)` products are
exactly `P`, with `U*⊂P`.

[Hess's Theorems 11 and 12](https://doi.org/10.1112/S146115700000108X)
give the genus formula and Frobenius descent condition. With absolute
trace `Tr_(K/F2)(a)=1`, two trace-zero factors fail the latter condition;
an order-one factor has nonzero relative trace and passes it. With
absolute trace zero, either pair passes. Consequently the **if and only
if criterion for a genus-three curve from this Hess construction** is

```text
Tr_(K/F2)(a)=0: δ ∈ P;
Tr_(K/F2)(a)=1: δ ∈ U*.
```

The earlier [product-family proof](../ghs_hess_c_parameter_audit_20261026/EXACT_CUBIC_FAMILY.md)
gives `|U*|=q²−1=255` and `|P|=(q−1)(q+1)(q+2)/2=2295`.
The executable audit constructs both sets and checks these sizes. The
criterion does **not** bound all genus-three covers, all Jacobian
isogenies, or all GHS-style transfer constructions.

## Exhaustive isogeny closure

All 8,190 normalized ordinary models, `a∈{0,32}` and
`b∈{1,…,4095}` in Sage's Conway polynomial basis, were counted
individually. They occupy 128 distinct elliptic isogeny classes.
Over a finite field, two elliptic curves have the same point count
exactly when they are isogenous over that field; hence an order class
is inside the **Hess genus-three isogeny closure** if and only if one
of its normalized models passes the criterion above. This tests the
entire isogeny class, without truncating an isogeny walk.

| Exact partition | Models | Orders |
| :--- | ---: | ---: |
| Direct Hess genus-three models | 2,550 | — |
| Models outside direct Hess criterion | 5,640 | — |
| In Hess isogeny closure | 5,315 | 79 |
| Outside Hess isogeny closure | 2,875 | 49 |
| Outside closure, cataloged genus-three Jacobian isogeny class | 2,827 | 47 |
| Outside closure, catalog status not positive | 48 | 2 |
| Outside closure, prior verified smooth plane-quartic witness | 258 | 4 |

The two catalog-status exceptions are orders 3972 and 4222, with 24
source models each. Their status is **unresolved**, not a proof that a
genus-three Jacobian cannot exist. Every one of the 2,827
catalog-positive residual source models passes the preceding
[simple-root subgroup-survival gate](../ghs_cubic_tate_bridge_20261026/REPORT.md):
for its largest prime `r`, `gcd(r,3(4096−1))=1`. Tate's theorem and
the integral Hom argument therefore imply existence of a subgroup-
surviving isogeny to some Jacobian in the cataloged class. They do not
provide coefficients, an evaluable map, or its computation cost.

The complete certificate records each model's `a,b`, source order,
Hess predicate, catalog label and status, and selected prime. Its
47 positive residual order rows provide an exact per-candidate catalog
lookup. The [earlier LMFDB snapshot and independent catalog audit](../ghs_cubic_catalog_q16_20261026/REPORT.md)
are hashed into this certificate. A cataloged isogeny class is prior
work; the snapshot does not establish whether any exact elliptic equation
or quartic equation is unpublished.

## High-prime residual family and exact witness

Thirteen catalog-positive residual order classes have cofactor two
and selected prime `r>1900`; their orders and primes are
`3974/1987`, `3986/1993`, `3998/1999`, `4006/2003`, `4022/2011`,
`4034/2017`, `4054/2027`, `4078/2039`, `4126/2063`, `4166/2083`,
`4174/2087`, `4178/2089`, and `4198/2099`. Only order 4126 among
these has a previously verified smooth plane-quartic equation in the
current screen. This is a **new residual search cohort relative to the
complete Hess genus-three isogeny closure**, not a newly proved
cryptographic weakness.

For order 4126, use
`K=GF(2)[w]/(w^12+w^7+w^6+w^5+w^3+w+1)`, Sage polynomial-basis
integer encoding, and

```text
E: y²+xy=x³+K.from_integer(32)*x²+K.from_integer(63)
#E(K)=4126=2·2063
G=(62,1499), Q=(2633,1798)=[7]G, ord(G)=2063.
```

All **108** normalized models of order 4126 fail the direct Hess
genus-three criterion, so this entire isogeny class is outside that
closure. The [verified quartic witness](../ghs_binary_cubic_isogeny_q16_20261026/REPORT.md)
over `k` has the coefficient vector
`[1,13,13,0,10,7,11,10,5,10,14,5,15,3,11]` in the monomial order
specified there; it is geometrically smooth and its Jacobian has the
same Weil polynomial as `Res_(K/k)(E)`. Its LMFDB isogeny class is
[3.16.a_a_bd](https://www.lmfdb.org/Variety/Abelian/Fq/3/16/a_a_bd).
The other previously verified quartic orders outside the closure are
3978 (30 source models), 4030 (48), and 4094 (72), giving the 258-model
row above.

The formerly highlighted class-C cohort partitions more sharply now:

| Source order | Models | Direct Hess genus three | In Hess isogeny closure |
| ---: | ---: | ---: | :--- |
| 4036 | 96 | 48 | yes |
| 4124 | 96 | 84 | yes |
| 4126 | 108 | 0 | no |

Thus 4036 and 4124 are **rediscoveries at the family level** through
Hess plus isogeny; 4126 is a structurally distinct transfer candidate
from that specific construction. Hess and [Menezes–Teske](https://cacr.uwaterloo.ca/techreports/2004/corr2004-25.pdf)
already describe generalized GHS and isogeny transfer. The quartic
isogeny class was already cataloged. A bounded exact-equation search
has not established global publication novelty.

## Proof and attack gates

`ClosureCounts.lean` checks the arithmetic partitions and the abstract
meaning of an isogeny closure label. Lean does **not** formalize finite
field arithmetic, Hess's theorem, the 8,190 Sage cardinalities, Tate's
isogeny theorem, or a concrete source-to-Jacobian map. Those dependencies
are explicit in the proof chain.

The 4126 quartic gives an isogeny **existence** certificate. An attack
needs an evaluable map `Φ:E(K)[2063]→Jac(C)(k)[2063]` with
`Φ(G)≠0` and `Φ(Q)=[7]Φ(G)`, plus complete relation collection,
linear algebra, target descent, and a same-source rho comparison that
charges construction and transfer. These have not run for this class,
so its full GHS cost and ratio to rho are unknown. The earlier GF(8)
end-to-end controls had total attack times 0.720 s and 0.870 s versus
0.0003125 s mean rho on their respective fixed prime subgroups;
those times cannot be transferred to GF(16).

### Novelty labels for the order-4126 lead

| Label | Current assessment |
| :--- | :--- |
| New exact equation | Unresolved outside a bounded catalog and exact-string search |
| New eligible family | New as a residual classification in this audit; global publication novelty unresolved |
| New attack method | No: the isogeny-transfer idea is prior work |
| Measured vulnerability | Not established: map and full attack cost missing |
