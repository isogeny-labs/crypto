# Complete normalized degree-seven exclusion for one trace +22 carrier

## Exact objects and conclusion

Let `K=F_3[a]/(a^5+2a+1)` and let the fixed genus-five curve over `F_3` be

`Y: y²=f(x)=x^11+x^9+x^8+2x^7+2x^6+2x^4+x³+x²+2x`.

Over `K`, `f=x Q_1Q_2Q_3Q_4Q_5`, where each `Q_i` is monic irreducible of
degree two. Let

- `E_68: v²=u³+u²+(2+a+a²+2a³)`, and
- `E_84: v²=u³+u²+(a+a⁴)`.

Both curves have 222 `K`-points and trace `+22`. Their eight other exact
source models in the recorded class census are the two Frobenius orbits
checked in `../ghs_trace22_carrier_variants_20261116/Trace22Orbit.lean`.

**Computer-assisted fixed-instance theorem.** There is no degree-seven
`K`-morphism `Y_K→E_68` or `Y_K→E_84`. Applying `a↦a³` gives the same
conclusion for the other eight source equations in those two orbits.
This is a finite-model statement on this `Y`, not a classification of all
genus-five carriers or all Jacobian correspondences. The computational
elimination has one Sage implementation and structural receipt audit; it has
not been independently reimplemented in another algebra system.
[pcc:unverified]

## Why the denominator list is complete

Translate any proposed map so that the rational point at infinity on `Y`
goes to the elliptic origin. With that base point, the hyperelliptic
involution acts as elliptic negation. Thus `u=A(x)/B(x)` for coprime
`A,B∈K[x]`, and `v=y C(x)/B(x)²`. The map degree is the degree of the rational
function `A/B` on the `x`-lines, namely seven. Infinity maps to infinity;
its local index `e∞` is odd because the involutions act by sign on both
local parameters. Hence `deg A=7` and `deg B=7-e∞∈{0,2,4,6}`. Multiplying
`A,B` by one scalar makes `B` monic.

Write `T=A³+A²B+bB³`. The exact equation is `f C²=B T`. At an irreducible
factor `P|B`, coprimality gives `T≡A³ (mod P)≠0`. Comparing valuations in
the equation shows `v_P(B)≡v_P(f) (mod 2)`. Thus an odd exponent in `B`
is allowed only at one of the branch factors `x,Q_1,…,Q_5`; every nonbranch
factor occurs with even exponent. Since `deg B` is even and `deg Q_i=2`,
the linear factor `x` cannot occur to odd exponent. It also cannot occur to
even exponent because `v_x(f)=1`. Therefore

`B=(product of s distinct Q_i) H²`, with `2s+2deg H≤6`,

apart from the same formula when `H` shares an already selected `Q_i`.
That overlap can occur only for `s=1`, `deg H=2`, yielding `B=Q_i³`.
If `H` shares any other branch factor, the parity equation rejects it.
The possible `(s,deg H)` pairs are exactly

`(0,0),(0,1),(0,2),(0,3),(1,0),(1,1),(1,2),(2,0),(2,1),(3,0)`.

The arithmetic list, its ten-entry count, and the candidate-count formulas
are checked by [DegreeSevenShapeArithmetic.lean](DegreeSevenShapeArithmetic.lean).
The geometric normalization and valuation argument are written proofs here,
not Lean-formalized geometry.

At every unpoled branch factor, the map lands on an elliptic 2-torsion
point. The elliptic cubic has one `K`-root and splits over the quadratic
extension, so CRT supplies every Galois-compatible assignment used by the
searches. The numerator has the unique residue `A≡Bh (mod M)` at the
unpoled branch product `M`; when `deg M≤7`, every allowed high-degree
multiple of `M` is enumerated. Every surviving tuple is checked in the full
polynomial identity, so the point and coefficient filters only reduce work.

## Exhaustive normalized degree-seven receipt partition

| denominator class | retained search | masks 68/84 exact hits |
|---|---|---:|
| `1, Q_i, Q_i³, Q_iQ_j` | [sparse](SPARSE_LINEAR_RESULT.md) | 0 / 0 |
| `(x-c)^2,(x-c)^4,(x-c)^6,Q_i(x-c)^2,Q_i(x-c)^4` | [linear square](SPARSE_LINEAR_RESULT.md) | 0 / 0 |
| `H²,Q_iH²`, monic quadratic `H` coprime to `f` | [prior quadratic screen](../ghs_trace22_cover_screen_20261116/RESULT.md) | 0 / 0 |
| `H²`, monic cubic `H` coprime to `f` | [cubic square](CUBIC_SQUARE_RESULT.md) | 0 / 0 |
| `Q_iQ_j(x-c)²` | [two poles plus linear square](TWO_POLES_LINEAR_RESULT.md) | 0 / 0 |
| `Q_iQ_jQ_k` | [three poles](THREE_POLES_RESULT.md) | 0 / 0 |

There are twelve source-bound receipts across these six arms and two source
representatives. [AUDIT.json](AUDIT.json), built by
[audit_receipts.py](audit_receipts.py), checks each receipt's source SHA256,
mask, group order, expected coverage counters, absence of censored cubic
systems, and zero-hit field. This is a structural audit, not an independent
implementation of every elimination. The sparse and linear arms reuse the
previously frozen trace-`+26` algorithms with exact trace-`+22` parameters;
the other three new arms are separately frozen and committed before runs.

## Bound, subgroup implication, and novelty boundary

For these curves, [Howe, Theorem 4.17 and Remark 4.18](https://arxiv.org/pdf/2110.04221)
give a `K`-morphism to each elliptic target of some degree at most 26. The
published theorem uses the ordinary factor, the full endomorphism order
`Z[π]` (discriminant `22²−4·243=−488`), and `γ_10^10<5669`.
The numerical inequality for ceiling 26 is checked in the new Lean file.
The [earlier geometric lower bound](../ghs_odd5_carrier_20261115/THEOREM.md)
is four. Thus the presently proved
degree window on this fixed carrier is **`{4,5,6,8,…,26}`**: degree seven is
excluded, while the other listed degrees are possibilities rather than
constructed covers.

Because 37 divides `#E(K)=222` and every Howe-supplied degree is below 37,
push-pull implies that such a cover's pullback is injective on the selected
order-37 subgroup of `E(K)`. This is an abstract subgroup-preserving map
existence statement. It does not provide evaluable coordinate formulas or a
descent into `J(Y)(F_3)`, and it has no complete GHS/rho cost measurement.

Prior work checked: Howe's original theorem and the repository's primary
source audit of [Iijima–Momose–Chao 2015](https://eprint.iacr.org/2015/805.pdf),
whose specified direct genus-five Case 13 has full rational 2-torsion;
these order-222 sources have only one rational 2-torsion point. The bounded
search and paper audit do not establish publication priority for this
finite exclusion. It is a new local obstruction receipt in this worktree,
not a newly demonstrated vulnerable family or attack method.
