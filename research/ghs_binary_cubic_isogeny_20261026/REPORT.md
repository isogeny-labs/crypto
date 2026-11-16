# Binary cubic plane-quartic transfer screen

26 October 2026. Conductor T-5328. Frozen [protocol](PROTOCOL.md), deterministic
[screen](screen.py), [raw result](results.json), independent [replay](verify.py)
and [certificate](verification.json). This record is a structural screen.

## Result and novelty boundary

The fixed search drew 2,048 plane quartics over `k=GF(4)`, of which 55 were
smooth with `#C(k)=5` and `#C(GF(16))=17`. Forty-eight of those had a Weil
polynomial matching an elliptic curve over `K=GF(64)` in the complete 126-model
source cohort. They occupied 12 distinct source-order classes. Eight of the
12 have no directly genus-three class-B source model in that cohort. This
exhaustive absence concerns the 126 normalized elliptic models, not arbitrary
covers or correspondences.

The strongest subgroup shapes in those eight are `#E(K)=58=2·29` and
`62=2·31`. The direct binary GHS criterion gives genus four for their selected
class-C source equations. Smooth plane quartics in their Weil-restriction
isogeny classes were constructed and independently recounted. By Tate's
finite-field theorem, equality of Weil polynomials implies an isogeny of
abelian varieties. It does **not** supply an evaluated source-to-Jacobian map
or establish that a chosen isogeny preserves the order-29 or order-31 group.

The LMFDB prior-work check changed the novelty verdict: **all eight isogeny
classes already have catalogued Jacobian models, including plane quartics**.
This search therefore does not establish a new eligible isogeny class or a
new vulnerable curve. The open hypothesis is an *efficient explicit binary
source-to-quartic transfer* with a complete decomposition attack and a
favorable measured cost. Joux--Vitse already described characteristic-two
hyperelliptic covers, decomposition, and isogeny walks; Tian's direct
Weil-restriction quotient gives another prior approach to genus-three
transfers. Neither a new attack method nor a below-rho cost is claimed here.

## Exact candidates and prior-work checks

Let `w` satisfy `w^6+w^4+w^3+w+1=0` over `GF(2)`, and write integers in
Sage's polynomial-basis encoding. Every source below is
`E: y²+xy=x³+a*x²+b`. Let `a0` satisfy `a0²+a0+1=0` in `k=GF(4)`.
The quartic equations are completely specified by the 15-element
coefficient vectors in [results.json](results.json): exponents are
`(i,j,4-i-j)` for descending `i`, then descending `j`, beginning with `x⁴`.

| `#E` | `a,b` | `r`, cofactor | `N3` | LMFDB class | catalogued Jacobians / hyperelliptic | rational point classes reach `r`? | distinct fully split lines |
| ---: | :--- | :--- | ---: | :--- | :--- | :--- | ---: |
| 58 | `8,2` | `29,2` | 44 | `3.4.a_a_ah` | 10 / 4 | yes | 0 |
| 60 | `0,11` | `5,12` | 50 | `3.4.a_a_af` | 12 / 4 | yes | 0 |
| 62 | `8,8` | `31,2` | 56 | `3.4.a_a_ad` | 10 / 4 | yes | 0 |
| 68 | `0,8` | `17,4` | 74 | `3.4.a_a_d` | 16 / 4 | yes | 0 |
| 70 | `8,11` | `7,10` | 80 | `3.4.a_a_f` | 24 / 4 | yes | 0 |
| 72 | `0,2` | `3,24` | 86 | `3.4.a_a_h` | 18 / 4 | no | 0 |
| 76 | `0,10` | `19,4` | 98 | `3.4.a_a_l` | 20 / 4 | yes | 0 |
| 78 | `8,9` | `13,6` | 104 | `3.4.a_a_n` | 10 / 0 | yes | 0 |

The LMFDB `hyp_count=0` at order 78 says that its current catalog lists no
hyperelliptic model; it is not a proof that none exists. The other seven
classes already have catalogued hyperelliptic models as well as quartics.
The database records were queried by exact degree, field size, full Weil
polynomial and group order on 26 October 2026. Each class can be inspected
at `https://www.lmfdb.org/Variety/Abelian/Fq/3/4/<label-tail>`; for example
[`3.4.a_a_ah`](https://www.lmfdb.org/Variety/Abelian/Fq/3/4/a_a_ah) and
[`3.4.a_a_ad`](https://www.lmfdb.org/Variety/Abelian/Fq/3/4/a_a_ad).
Targeted web searches on those labels and characteristic-two cubic cover
attacks found no *evaluated source map* for these exact elliptic models. That
limited search is not a proof of unpublished novelty.

For the two leading source groups, the independent verifier found:

| source | `G` | `Q=[7]G` | prime-order check | quartic coefficient vector |
| :--- | :--- | :--- | :--- | :--- |
| `a=8,b=2,#E=58` | `(41,55)` | `(24,54)` | `[29]G=[29]Q=O` | `[1,3,2,2,3,0,1,0,2,2,1,0,0,2,3]` |
| `a=8,b=8,#E=62` | `(9,36)` | `(46,53)` | `[31]G=[31]Q=O` | `[1,3,3,0,0,2,0,1,0,0,2,3,0,2,0]` |

These coordinates identify source subgroups and fixed target checks. They
do not constitute an elliptic-to-quartic transfer. The verifier used Sage's
independent projective rational-point enumeration for `N1,N2,N3`, recounted
all 126 elliptic source groups, computed each quartic Jacobian's order with
the Hess model, and checked whether rational point-difference classes reach
the selected prime subgroup. All 12 witness orders passed the source-order,
quartic-count and Jacobian-order checks. Eleven selected quartics passed the
rational-point subgroup-reach check. For order 72, the five rational points
in this selected quartic do not reach
the order-three subgroup, making this particular factor base unsuitable.

## GF(8)/GF(512) extension: stronger structural candidates

The separately frozen [GF(8) protocol](PROTOCOL_Q8.md) drew 1,024 quartics.
Eleven were smooth with `N1=9`, `N2=65`, and a Weil polynomial matching one
of the 1,022 exhaustively enumerated normalized source models; they covered
ten group orders. The [independent replay](verify_q8.py) recounted every
source group and all ten quartics over `GF(8)`, `GF(64)`, and `GF(512)`.
It also computed the Jacobian group order and rational-point prime-subgroup
reach for the four leading rows below. The frozen [raw data](results_q8.json)
and [verification certificate](verification_q8.json) contain every equation,
coefficient vector and source model.

Here `K=GF(2)[w]/(w^9+w^4+1)`, `k=GF(2)[a0]/(a0^3+a0+1)`, integer coordinates
use Sage's polynomial-basis encoding, and each source is
`E: y²+xy=x³+x²+b`. The equations for each smooth plane quartic are the
coefficient vectors in `results_q8.json` with the same fixed monomial order
as above.

| `#E(K)` | `b` | `r`, cofactor | `G`, `Q=[7]G` | `N3` | LMFDB class | catalogued models / hyperelliptic | distinct split lines |
| ---: | ---: | :--- | :--- | ---: | :--- | :--- | ---: |
| 502 | 9 | `251,2` | `(352,425)`, `(508,5)` | 480 | `3.8.a_a_al` | 15 / 15 | 2 |
| 514 | 2 | `257,2` | `(3,97)`, `(50,216)` | 516 | `3.8.a_a_b` | 9 / 9 | 1 |
| 526 | 19 | `263,2` | `(114,490)`, `(215,149)` | 552 | `3.8.a_a_n` | 27 / 27 | 2 |
| 542 | 44 | `271,2` | `(15,127)`, `(219,506)` | 600 | `3.8.a_a_bd` | 15 / 15 | 2 |

The corresponding quartic coefficient vectors, in the monomial order above,
are:

- Order 502: `[1,4,4,5,7,7,5,3,0,4,6,3,4,4,6]`.
- Order 514: `[1,1,5,4,6,5,7,4,3,0,3,2,6,2,6]`.
- Order 526: `[1,4,3,2,2,4,2,4,6,6,7,4,0,4,0]`.
- Order 542: `[1,1,7,6,2,5,1,2,6,5,0,0,3,7,4]`.

All source models at these four orders are class C in the complete cohort:
none has the direct genus-three binary GHS trace condition. For each row,
`[r]G=[r]Q=O`, the selected quartic is geometrically smooth,
`#Jac(C)(k)=#E(K)`, and its nine rational point classes reach the order-`r`
part. These checks establish a useful factor-base gate, not a transfer map.

The LMFDB exact-polynomial check identifies all four **isogeny classes** as
previously catalogued, with only hyperelliptic equations in their currently
stored `curves` arrays. Our plane quartic equations are explicit structural
witnesses absent from those arrays; targeted searches on the four exact
class labels and `plane quartic` returned no source identifying these exact
equations. Neither catalog absence nor that bounded search proves model
novelty. The potentially new result to pursue remains an evaluated binary
source-to-quartic transfer and an improved complete attack on this cohort.
The novelty labels for all four rows are: **new exact curve: unresolved**,
**new eligible family: no**, and **new attack method: no**. The currently
catalogued isogeny class does not establish a vulnerability.
The four exact class records are
[`3.8.a_a_al`](https://www.lmfdb.org/Variety/Abelian/Fq/3/8/a_a_al),
[`3.8.a_a_b`](https://www.lmfdb.org/Variety/Abelian/Fq/3/8/a_a_b),
[`3.8.a_a_n`](https://www.lmfdb.org/Variety/Abelian/Fq/3/8/a_a_n), and
[`3.8.a_a_bd`](https://www.lmfdb.org/Variety/Abelian/Fq/3/8/a_a_bd).

## Attack-cost gate

For a source point `P`, the missing object is a computable geometric map
`Φ : E(K) → Jac(C)(k)` (or a verified chain of such maps) with
`Φ(G) ≠ 0` and `Φ(Q)=[7]Φ(G)` in the two leading cases. Matching zeta
polynomials and equal group orders cannot replace that check. The next run
must account for construction and evaluation of `Φ`, relation collection,
linear algebra, target descent and independent scalar replay, then compare
the complete cold cost with rho on the *same* `(G,Q)`. The current
same-target GHS-versus-rho ratio is **unknown** for every row. The zero
distinct fully split lines for the eight `GF(4)` quartics warns that a plain
four-point line harvest there is inadequate. The four `GF(8)` quartics have
one or two such lines, still far short of a demonstrated relation system;
large-prime relations and target descent need separate measurements.

The repository's existing checked Lean subgroup lemmas are conditional on an
additive nonzero map. This study does not instantiate their geometric
hypotheses for these curves, and no new candidate-specific Lean proof is
claimed.

## Reproduction

From the repository root:

```sh
sage -python research/ghs_binary_cubic_isogeny_20261026/screen.py
sage -python research/ghs_binary_cubic_isogeny_20261026/verify.py
sage -python research/ghs_binary_cubic_isogeny_20261026/screen_q8.py
sage -python research/ghs_binary_cubic_isogeny_20261026/verify_q8.py
```

`results.json` binds the protocol and screen SHA-256 hashes;
`verification.json` binds the raw result and verifier hashes. These files
preserve the fixed-field run, including all 126 source models and the first
quartic witness for each of 12 matched group orders. Elapsed runtime is a
diagnostic only and is not an attack measurement. The corresponding `q8`
files carry the full 1,022-source cohort and the first quartic witness for
ten matched group orders.

## Sources

- [Joux--Vitse, cover and decomposition index calculus](https://www.iacr.org/archive/eurocrypt2012/72370010/72370010.pdf), especially §4.1 and Appendix A.2.
- [Tian, cover attacks for elliptic curves with prime order](https://arxiv.org/abs/2012.07173), direct Weil-restriction isogeny construction.
- [Diem--Thomé, genus-three plane-quartic index calculus](https://members.loria.fr/EThome/files/non-he-genus3.pdf).
- [LMFDB, abelian varieties over finite fields](https://www.lmfdb.org/Variety/Abelian/Fq/).
