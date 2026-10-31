# A Frobenius-cyclic quartic transfer outside the direct cubic cover census

26 October 2026. This report follows the [frozen geometric panel](PROTOCOL.md)
and the [fixed full-attack trial](FULL_ATTACK_PROTOCOL.md). Every displayed
finite-field coefficient uses Conway generators `a³+a+1=0` in `k=GF(8)`
and `w⁹+w⁴+1=0` in `K=GF(512)`, with `a=w^73`. Scripts, public Magma input
files, complete API responses, Sage certificates, and [the audit](audit.py)
are retained in this directory.

## Exact quartic and the geometric split

The order-554 witness is the smooth plane quartic

```text
C: x^4 + a²x³y + (a²+a)x²y² + axy³ + (a+1)y⁴
   + a²x³z + (a²+a+1)x²yz + (a²+1)y³z
   + (a²+1)xyz² + (a²+a)y²z² + (a²+a+1)xz³
   + a²yz³ + (a²+a+1)z⁴ = 0.
```

The independently checked counts are `#C(k)=9`, `#C(GF(64))=65`,
`#C(K)=636`. Its Weil polynomial is `X⁶+41X³+512` and
`#Jac(C)(k)=554=2·277`. Magma 2.30-1 independently checks smoothness
and the nine `k`-points. Its automorphism group over `k` is trivial;
its geometric group is `C₂×C₂`. All three nonidentity involutions are
defined over `K`, where the base-change group has order four. The
three other panel quartics (orders 482, 524, 556), together with the
separate order-548 witness, have trivial geometric groups, so those
four **exact quartics** have no separable degree-two elliptic map over
any extension. This does not rule out other curves in their Jacobian
isogeny classes.

| Source order | Normalized sources | Geometric group of this quartic | Prior catalog and route check |
| ---: | ---: | :--- | :--- |
| 482 | 9 | trivial | [`3.8.a_a_abf`](https://www.lmfdb.org/Variety/Abelian/Fq/3/8/a_a_abf): Jacobian known, 15 listed hyperelliptic curves; this exact quartic not listed |
| 524 | 18 | trivial | [`3.8.a_a_l`](https://www.lmfdb.org/Variety/Abelian/Fq/3/8/a_a_l): Jacobian known, 15 listed hyperelliptic curves; this exact quartic not listed |
| 548 | 9 | trivial | [`3.8.a_a_bj`](https://www.lmfdb.org/Variety/Abelian/Fq/3/8/a_a_bj): Jacobian status uncertain and no listed curve; the exact quartic proves existence, as reported in the [separate audit](../ghs_q8_548_geometric_cover_20261026/REPORT.md) |
| 554 | 9 | `C₂×C₂` | [`3.8.a_a_bp`](https://www.lmfdb.org/Variety/Abelian/Fq/3/8/a_a_bp): Jacobian known, three listed hyperelliptic curves; this exact quartic and transfer route not listed |
| 556 | 9 | trivial | [`3.8.a_a_br`](https://www.lmfdb.org/Variety/Abelian/Fq/3/8/a_a_br): Jacobian known, three listed hyperelliptic curves; this exact quartic not listed |

All five order classes are outside the previously enumerated direct-B
cubic-cover order closure. Each row is an exact-order and exact-equation
check against the current LMFDB class, not proof that the equation has
never appeared in another publication. The source classes and catalog
receipts are in the [parent structural report](../ghs_quartic_new_classes_q8_20261026/REPORT.md).

Magma's general `CurveQuotient` intrinsic raised an internal assertion
on these characteristic-two involutions; the full failed response is
retained in [554_quotients_receipt.json](554_quotients_receipt.json).
We instead derived each invariant equation from the explicit matrix,
checked the polynomial identity in Sage, and independently constructed
the first quotient as a smooth genus-one intersection of two quadrics
in Magma. All three invariant models have 554 `K`-points.

## Explicit separable degree-two quotient

For the first involution, set

```text
X = x,
Y = w^132 x + w^68 y + w^179 z,
Z = x+z,
U = X²+XY.
```

The regular quotient map is
`π:C_K→D`, `[x:y:z]↦[Y²:YZ:Z²:U]`. With coordinates
`[t0:t1:t2:t3]`, the complete-intersection model is

```text
t1²+t0 t2 = 0,
t3² + (w^161 t0+w^297 t1+w^83 t2)t3
     + w^458 t0²+w^234 t0t1+w^366 t1²
     + w^498 t1t2+w^310 t2² = 0.
```

Substituting the map into both quadrics yields zero modulo the quartic.
The involution fixes `Y,Z,U` and sends `X` to `X+Y`, so the induced
function-field extension has degree exactly two. Magma checks that `D`
is smooth, genus one, has 554 points, and that `π` is regular.
The elliptic model of `D` is isomorphic over `K` to the **exact** source

```text
E_105: v²+uv = u³+u²+b,  b=105 in Conway integer encoding.
```

With the quotient's Magma elliptic coordinates `(u',v')`, the checked
isomorphism from this source model is
`u'=w^418 u+w^424`,
`v'=w^116 v+w^135 u+w^167`. The defining
`IsomorphismData` is `[w^424,w^228,w^167,w^209]` in
[the independent Magma receipt](554_elliptic_quotient_receipt.json).

The `+u²` term matters: `a=1` in the source census is its Weierstrass
`a₂` coefficient. The nine order-554 normalized source models are
`b∈{14,84,105,222,235,408,437,454,493}`. Every one has an evaluated
degree-two isogeny path to `E_105`, as certified in
[554_isogeny_paths_receipt.json](554_isogeny_paths_receipt.json). The
directed edges are

```text
14→493→222→235→105→454→437→408→84→14.
```

Each edge is made with `IsogenyFromKernel(E,x)` and a checked
codomain isomorphism. The longest selected route to `105` has eight
degree-two steps (degree at most 256), all prime to 277. Every source
generator and `Q=[7]G` retains order 277 and the scalar relation after
every step.

## Subgroup transfer and exact point check

Let `u_b:E_b→E_105` be the selected isogeny path, followed by the
explicit isomorphism to `Jac(D)`. The transfer is the homomorphism

```text
Φ_b(P) = Tr_{K/k}(π*(u_b(P))) ∈ Jac(C)(k).
```

Here `π*` is divisor-class pullback and the trace sums the three
`k`-Frobenius conjugates. For an odd prime `r`, the maps from the three
elliptic quotients give a product-to-Jacobian isogeny with 2-primary
kernel. On `r`-torsion it is an isomorphism; Frobenius cycles its three
factors, so tracing one factor yields a nonzero diagonal class. Each
selected `u_b` also has 2-power degree. Thus `Φ_b` is injective on
`E_b(K)[277]`. This is a structural transfer statement, independent of
attack speed.

For `b=105` an exact source pair is

```text
G=(w^478,w^433),    Q=(w^77,w^119)=[7]G,    ord(G)=277.
```

The quotient fibers used for independent evaluation are

```text
π⁻¹(O) = {(w^54:w²:1), (w^75:w^281:1)},
π⁻¹(G) = {(w^164:w²:1), (w^262:w^140:1)},
π⁻¹(Q) = {(w^328:w^211:1), (w^109:1:0)}.
```

The [Sage trace replay](trace_subgroup.sage) forms both pullback
divisor classes from these distinct fibers and sums their three
Frobenius conjugates in the genus-three Jacobian. It checks that both
are nonzero, each is killed by 277, and `Φ_105(Q)=7Φ_105(G)`.
[Counts.lean](Counts.lean) checks the arithmetic and the abstract
push-pull and injective-product nonzero lemmas; it does not formalize the curve geometry or
the finite-field calculations.

## General criterion and finite bounds

Let `k=GF(q)` have characteristic two and `K=GF(q³)`. Suppose a smooth
nonhyperelliptic genus-three `C/k` has geometric `C₂×C₂` automorphisms
whose three nonidentity involutions form a Frobenius 3-cycle. Over `K`
the quotient curves are elliptic; if any one has trace `t` over `K`,
then the quotient decomposition and cyclic Frobenius action imply

```text
P_C(T)=T⁶−tT³+q³,
#Jac(C)(k)=q³+1−t,
#C(k)=q+1,   #C(k²)=q²+1,   #C(K)=q³+1−3t.
```

For any odd prime `r` dividing `q³+1−t`, a source curve over `K` with
an evaluated isogeny of degree prime to `r` into a quotient elliptic
curve has an injective order-`r` transfer `Φ` as above. These are
rigorous **eligibility conditions**, not a speedup bound. Hasse gives
`|t|≤2q^(3/2)`. For this witness `q=8`, `t=−41`, `r=277`.

The complete normalized ordinary source census has 1,022 models.
The established direct-B cubic route has 324 models in its arbitrary
`K`-isogeny order closure. Order 554 is outside that closure and has
exactly nine models, all reached by the maps above. Thus the union of
these **two established route closures** has 333 models, leaving 689
outside both. This is not an upper bound on all possible covers or on
all GHS vulnerabilities. The four trivial-automorphism quartics only
exclude their own degree-two morphisms.

## Prior work and claim boundary

[Nart--Ritzenthaler 2009](https://arxiv.org/pdf/0905.0546) proves the
three elliptic quotients and product decomposition for genus-three
curves with a `C₂×C₂` subgroup after the involutions are defined.
[Their earlier complete classification](https://arxiv.org/pdf/math/0312366)
already covers all characteristic-two smooth plane quartics and their
finite-field descents. This is therefore **not a newly discovered
geometric curve family**. The order-554 isogeny class
[`3.8.a_a_bp`](https://www.lmfdb.org/Variety/Abelian/Fq/3/8/a_a_bp)
was catalog-positive with three hyperelliptic models; this exact plane
quartic was absent from the cataloged curves. Exact-label and equation
searches in the [parent prior-work check](../ghs_quartic_new_classes_q8_20261026/prior_work.json)
did not identify it elsewhere, but absence from that search is not
publication novelty. The direct-B GHS route and its isogeny closure
are in [GHS](https://www.kantmath.de/publications/papers/weil_descent.pdf)
and [Menezes--Teske](https://cacr.uwaterloo.ca/techreports/2004/corr2004-25.pdf).
In §7, Menezes--Teske already place non-subfield curves with group order
`0` or `2` modulo `8` in a larger conjectural isogeny-walk eligibility
set, under their Assumption C. Since `554≡2 (mod 8)`, the arithmetic
source class was **not overlooked by that prior analysis**. Their
endpoint there is a direct hyperelliptic GHS cover; our complete small
`q=8` census finds no such direct-B endpoint of order 554. The exact
quartic quotient and traced subgroup map are a different evaluated
construction at this parameter.

The finding is an **exact previously uncataloged quartic witness and a
verified transfer route for nine source models outside the prior
direct-B closure**. It does not establish a new attack method or a
cryptanalytic vulnerability. The bounded exact-equation search and
full attack measurement below determine the narrower labels.

## Measured attack against rho

The [source rho baseline](rho_baseline.sage) uses the same exact `(G,Q)`
and 32 deterministic successful Floyd trials. Median is 42 elliptic
group operations and 0.0005539165 seconds. The fixed
[full-attack trial](full_attack.sage) completed after 400 Jacobian
additions, retaining 189 collision relations and both transferred target
decompositions. The relation matrix has a one-dimensional right kernel;
its two decomposition scores give scalar `7`, checked on the exact
source pair. [full_attack.json](full_attack.json) retains all relation
rows, the kernel vector, both decomposition vectors, and the stage
times. [audit.py](audit.py) independently checks each relation against
the kernel and recomputes the scalar ratio.

| Stage | Measured seconds |
| :--- | ---: |
| Field, curve, and Jacobian setup | 7.709412 |
| Transfer of both targets | 14.447603 |
| Factor-base construction | 31.386549 |
| Relation walk and target decompositions | 291.752011 |
| Linear algebra | 0.000556 |
| Source verification | 0.034788 |
| **Full one-target attack** | **345.331173** |
| Same-target rho, median of 32 runs | 0.000554 |

The measured full attack is about 623,000 times slower in wall time
than this same-target rho baseline. This ratio describes these concrete
Sage implementations and this small group; it is not an asymptotic
bound or a result for larger parameters. It establishes an end-to-end
transfer and relation attack, but **not** a cryptanalytic vulnerability
for the order-554 source family. The independently evaluated
[subgroup-map replay](trace_subgroup.sage) took 47.368 seconds in its
separate run and checks the complete point transfer.
