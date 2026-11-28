# Vitse / Prime31 compatibility: mathematical review

Date: 2026-11-23. Parent task: crypto PR #5016, `lane/vitse-prime31-compatibility`.
Source revision reviewed: `7d61e3b45de6e93d235345a0061dae08db1f8b44`.

Status: written derivations and source inspection only; not Lean-kernel or signed-PCC verified. This note makes no performance, novelty, or complete-solver claim. [pcc:unverified]

## 1. Sources and exact scope

Sources checked (KB): KB unavailable in this child runtime (no exposed `kb_status`, `search_knowledge`, or theorem tools); the parent already recorded the wrapper/access fallback. Used the supplied primary talk, accepted parent pointers, and the following current files rather than treating the old local checkout as current.

- Vanessa Vitse, *Summation polynomials and symmetries for the ECDLP over extension fields*, DLP 2014: printed slides 7-15, 20-30, and 36. The upload contains overlays; printed slide numbers are the stable references.
- [`semaev_selector.rs`](https://github.com/isogeny-labs/crypto/blob/7d61e3b45de6e93d235345a0061dae08db1f8b44/src/cryptanalysis/semaev_selector.rs): `selector_coordinate_pairs`, `lift_selector_pairs`, split `X^d-1`, and the distinction between coordinate candidates and signed relations.
- [`dickson_s3_quadratic.rs`](https://github.com/isogeny-labs/crypto/blob/7d61e3b45de6e93d235345a0061dae08db1f8b44/examples/dickson_s3_quadratic.rs): `System::new`, `assignment_for_points`, `block_equations`, the fixed target, equal leaf ladders, and finite prefix chart.
- [`dickson_s3_quadratic/accounting.rs`](https://github.com/isogeny-labs/crypto/blob/7d61e3b45de6e93d235345a0061dae08db1f8b44/examples/dickson_s3_quadratic/accounting.rs): `Request::admit`, `build_base`, signed decoding, row and multiset deduplication.
- [`pkm_smooth_tower.rs`](https://github.com/isogeny-labs/crypto/blob/7d61e3b45de6e93d235345a0061dae08db1f8b44/src/cryptanalysis/pkm_smooth_tower.rs): `SmoothTower`, `TowerKind`, `isogeny_tower_on`, `relation_system_polys`.
- [`ic_framework/pkm_semaev.rs`](https://github.com/isogeny-labs/crypto/blob/7d61e3b45de6e93d235345a0061dae08db1f8b44/src/cryptanalysis/ic_framework/pkm_semaev.rs): `T0Quotient`, `dickson_abscissae`, `new_dickson`, `PkmMode::Sym`, `t0_lift`.

The inherited fixture is

\[
p=2147483647,\qquad E:y^2=x^3+847514846x+2087444724,
\]

with the parent's pinned group order `2147446831` and cofactor one. This review does not independently recertify the point count. The current accounting admission independently requires a nonzero point annihilated by an odd prime `r` and a Hasse upper bound below `2r`, which is a sufficient order-admission pattern when its arithmetic checks succeed. [pcc:unverified]

The questions below concern five different objects: a selector's raw coordinate roots, the roots retained after rational-point/subgroup filtering, the summation equation, the fixed target, and the finite chart used by the current solver. Compatibility at one layer does not establish compatibility at the others.

## 2. A sufficient compatibility contract for translations and a common sign

Let `k` be a finite field, `E(k)` its abelian point group, `F_i` the allowed point set of leaf `i`, and let

\[
\mathcal R_R=\{(P_1,\ldots,P_m):P_i\in F_i,\;\sum_i P_i=R\}.
\]

For a permutation `pi`, a COMMON sign `epsilon in {+1,-1}`, and fixed rational points `T_i`, consider

\[
P'_i=\epsilon P_{\pi(i)}+T_i.
\]

Then commutativity gives the exact identity

\[
\sum_iP'_i=\epsilon R+\sum_iT_i.
\]

Consequently this candidate acts bijectively on the full fixed-target relation set if:

1. `epsilon F_{pi(i)} + T_i = F_i` for every leaf;
2. `sum_i T_i = (1-epsilon)R`;
3. each point map and its inverse are defined on the declared domain.

To act on a particular solver chart, it must additionally carry that chart onto itself, including auxiliary-variable reconstruction and excluded points. This is a sufficient contract for the displayed class of actions, not a classification of all possible symmetries. Independent per-leaf signs do not satisfy the displayed sum identity in general and must not be accepted by this rule. [pcc:unverified]

For torsion translations in a subgroup `K` of order coprime to the target subgroup order `r`, with nonzero `R` in an odd-prime-order subgroup `H`, the common-minus case would require `sum T_i=2R`. Since `K intersect H={O}`, that is impossible. Thus the generic fixed signed target admits the common-plus condition `sum T_i=O`, but global point negation maps the workload for `R` to the workload for `-R`.

Pinning only `x(R)` is different: `x(R)=x(-R)`, so a coordinate solver can forget this distinction. Its signed decoder must restore the actual requested target. The current selector lifting routine does exactly a full-point target check; the current oriented Dickson arm pins both target coordinates. [pcc:unverified]

## 3. Point negation is not coordinate negation

On the current short-Weierstrass model,

\[
-(x,y)=(x,-y),\qquad x(-P)=x(P).
\]

Therefore every factor base selected only by `x` and stable under the usual subgroup admission is closed under point negation, regardless of whether the scalar selector set is closed under `x -> -x`. A canonical representative and coefficient `+1` or `-1 mod r` can share one linear-algebra column. The current `accounting::build_base` already does this: it uses a canonical min-y point and its negative, mapping both to one column with opposite coefficients. [pcc:unverified]

By contrast `x -> -x` is a map on the field coordinate. It generally does not even take a rational point of the current curve to a rational point of that curve. Its raw-selector closure must be distinguished from closure of the retained x-support.

An especially direct algebraic check is available. With `S3` in the normalization used by the source,

\[
S_3(-X,-Y,t)-S_3(X,Y,t)
=4(X+Y)\bigl(t(XY+a)+2b\bigr).
\]

This follows by substituting in

\[
S_3(X,Y,t)=(X-Y)^2t^2
-2((X+Y)(XY+a)+2b)t+(XY-a)^2-4b(X+Y).
\]

For odd characteristic and `b != 0`, the difference is not the zero polynomial for any `t`: if `t != 0` its cubic coefficient is nonzero, and if `t=0` it is `8b(X+Y)`. Thus the current target curve does not inherit the Vitse even-coordinate-sign polynomial identity merely because a selector is even. This is an identity-level statement; an accidental symmetry of one very small constrained solution set is a different question and should be checked by an actual witness or exact finite-domain comparison. [pcc:unverified]

The intended negative development control is therefore an original valid coordinate/point relation together with its transformed coordinates, showing the transformed tuple fails the actual S3 equation or point/selector admission. A nonzero difference on an arbitrary tuple is only a polynomial-identity counterexample, not necessarily a valid-relation counterexample.

## 4. Permutations: equal selectors, unequal selectors, and the prefix chart

For the uncharted relation `sum P_i=R`, a permutation preserves the fixed target because addition is commutative. It preserves the labeled leaf domain precisely when it exchanges equal point sets. Equal selector root sets are sufficient for this membership condition only if the point, subgroup, sign, and chart restrictions are also the same.

For nonempty finite coordinate sets `V_i`, the Cartesian product is invariant under a leaf permutation iff `V_i=V_{pi(i)}` for every `i`. If any leaf set is empty, both Cartesian products are empty, but this is vacuous and should not grant a useful reduction. Different selector polynomial expressions can define the same root set, so equality of source strings is not the correct mathematical criterion. Conversely, swapping the selector metadata together with the variables creates a different labeled instance; it does not establish a symmetry of the original instance. [pcc:unverified]

The current FASS two-coordinate system uses identical `X^d-1` and `Y^d-1`, so swapping the two leaves preserves the selector constraints and `S3(X,Y,t)` while fixing `t`. Its returned ordered pairs can be paired by the swap, including fixed diagonal pairs. The signed relation row from a point pair is unchanged by swapping its order.

The current Dickson builder uses the same `D_(2^depth)(x_i,1)=0` selector on every leaf. This supplies full leaf exchangeability for the underlying uncharted group relation, but its represented auxiliary system is a particular prefix chain. It explicitly excludes an infinity partial sum. An arbitrary permutation can change whether a prefix is infinity.

For example, with nonzero points `P,Q`, `P != +/-Q`, and nonzero target `Q`, the tuple `(P,-P,Q)` has its first two-point sum at infinity and is excluded by the current finite prefix chart, while `(P,Q,-P)` has finite first prefix and the same final sum `Q`. Thus full `S_m` quotienting of the current chart cannot be inferred from equal leaf selectors alone. Sorting every solution into a single order can lose the only represented ordering of an orbit unless the missing charts are handled. [pcc:unverified]

The first-two-leaf swap is an immediately safe prefix-chart symmetry when their leaf domains match: the first sum is unchanged, and all later prefixes are unchanged. The current quartic, invariant-Q, line-L and oriented-O first-block equations are symmetric in their two child x-coordinates; their common line/slope and parent are unchanged by child exchange. Larger permutations require a separate chart reconstruction/preservation argument. Final relation-row or signed-multiset deduplication can still be valid even when a pre-extraction chart quotient is unavailable.

For the current library `PkmMode::Sym`, the source already builds ordinary elementary symmetric definitions and specializes a target-parameter symmetric polynomial. This is relevant existing support; it should not be relabeled as the full E[2] rational invariant-field construction in Vitse slides 21-23. [pcc:unverified]

## 5. What the raw selectors actually permit

All entries in this table describe selector roots only. Point filtering and the elliptic relation remain separate.

| Selector | Coordinate action | Exact root-set condition |
|---|---|---|
| `V={x:x^d=1}`, `d | p-1` | `x -> 1/x` | Always closed; all roots are nonzero. |
| Same | `x -> -x` | In odd characteristic, closed iff `d` is even. |
| `V={x:x^d=c}`, nonzero `c`, split with `d` distinct roots | `x -> zeta*x` | Closed iff `zeta^d=1`. |
| Same | `x -> kappa/x`, nonzero `kappa` | Closed iff `c^2=kappa^d`. |
| `D_N(x-lo,a)=c`, even `N` | `x -> 2lo-x` | Closed, because `D_N(-u,a)=D_N(u,a)`. |
| Same | `x -> 1/x` | No general closure rule; check the actual root set and poles. |

Thus the parent's `d=1302` FASS selector supports raw coordinate negation and reciprocal closure. This alone supplies neither action on the retained point support nor a symmetry of the target S3 equation. [pcc:unverified]

The current Dickson routine is the special tower `D_(2^t)(x-lo,1)=c_top`, with all `2^t` roots checked as distinct field elements during construction. In the new quadratic example `lo=0` and `c_top=0`, so raw `x -> -x` closure follows. Its torus identity `D_N(u+u^-1,1)=u^N+u^-N` must not be misread: the deck operation `u -> u^-1` fixes `x=u+u^-1`; it is not the coordinate reciprocal `x -> 1/x`. Already `D_2(x,1)=x^2-2` has roots whose reciprocals satisfy `x^2=1/2`, a different set in the Prime31 field. [pcc:unverified]

## 6. Torsion actions and fixed targets

Vitse's odd-characteristic full-2-torsion coordinate is a specially chosen degree-2 map, not the current ordinary x-coordinate. Under its stated twisted-Legendre and square hypotheses, the three nonzero order-2 translations act as `z -> -z`, `z -> 1/z`, and `z -> -1/z`. The corresponding factor base must be stable under those actions, and torsion shifts across a decomposition must sum to zero to preserve a fixed signed target.

The following are necessary eligibility checks:

- A nonzero rational point of order `ell` requires `ell | #E(k)`. If `#E(k)=h*r` and `gcd(ell,r)=1`, then `ell | h` is necessary. It is not sufficient for every desired higher-order or rank-two torsion structure.
- Full rational `E[2]` in odd characteristic requires all three nonzero 2-torsion points, equivalently splitting of the cubic, and forces `4 | #E(k)`. A cyclic order-4 point alone does not establish full `E[2]`.
- On the inherited cofactor-one odd-prime-order group, every nontrivial small rational torsion translation fails this necessary order condition. This excludes that Vitse mechanism on this curve under the pinned group-order premise; it says nothing about all other algebraic strategies.
- A base-field isogeny preserves the rational point count. It cannot manufacture small rational torsion in an isogenous representative of a prime-cardinality curve.
- The coordinate map, its field of definition, and the induced action must be established. A selector's Mobius symmetry is not a witness of such a curve action.

If one instead starts from the homogeneous sum-zero formulation with an extra `-R` slot, any permitted permutation or torsion shift involving that slot must lie in the stabilizer of the pinned target. One cannot quotient by the whole `(m+1)`-slot action after specializing the last coordinate. For the explicit field actions, finite target-coordinate stabilizers can be checked directly: `-z` fixes only zero (and projective infinity); `1/z` fixes `z=+/-1`; `-1/z` fixes `z^2=-1`. These are coordinate conditions only; signed point and chart conditions are still needed. [pcc:unverified]

## 7. Auxiliary isogenies, invariant maps, and equivariant maps

The current `pkm_smooth_tower::isogeny_tower_on` constructs an auxiliary curve with a smooth cyclic subgroup, a coset `R'+<Q>`, its x-coordinate set `V`, and a chain of quotient-isogeny x-maps. The target curve then uses the resulting field elements as allowable abscissae in its own S3 equations. A translation symmetry on the auxiliary curve does not thereby become a translation or automorphism of the target curve. The two group laws are distinct unless an explicit compatible transfer is supplied. [pcc:unverified]

This must be separated from the library's existing `T0Quotient` on a curve that already has rational `T0=(s,0)`. There, with `z=x-s` and `d=3s^2+a`, translation sends `z -> d/z`, while `w=z+d/z` is invariant. The source explicitly constructs S3 on the quotient curve and later lifts, trying the torsion parity needed for the full target. It is an existing implementation of the invariant/quotient route that Vitse slide 14 distinguishes from strict equivariance. [pcc:unverified]

For any isogeny `psi`, a quotient relation only gives

\[
\sum_i\psi(P_i)=\psi(R)
\quad\Longleftrightarrow\quad
\sum_iP_i-R\in\ker\psi.
\]

If all points lie in a subgroup on which `psi` is injective, this implies the original exact relation. Otherwise the kernel discrepancy must be resolved in the lift. Strict equivariance retains a nontrivial coordinate action `phi(P+T)=M_T(phi(P))`; invariance `phi(P+T)=phi(P)` instead factors through the quotient. Either can be useful in its proper role, but they are different obligations and should not share an eligibility flag.

## 8. The reusable guard and its allowed conclusion

The parent's new `compatibility.rs` has an appropriate bounded integration seam:

- `audit_domain(values, Mobius)` checks finite coordinate-set closure, records every pole and escaping image, and distinguishes empty closure from a nonempty bijection.
- `audit_selector_support(raw, retained, Mobius)` checks both supplied raw roots and the actual retained coordinate support, plus the containment of retained support in raw roots.
- `audit_leaf_permutation(domains, permutation)` checks equal source/destination leaf sets and distinguishes vacuous empty Cartesian products.

The mathematical justification of the finite Mobius test is simple: a nonsingular projective-linear map is injective on the projective line; on a finite affine set with no pole, containment implies equality and a bijection. The constructor's nonzero determinant is therefore a substantive prerequisite. The implementation's explicit `FiniteCoordinateSetsOnly` scope is correct and should remain part of its interface. [pcc:unverified]

This component does not certify completeness of an upstream selector root list, correct rational-point filtering, a curve action, a nonradical ideal automorphism, a fixed-target relation action, or finite-chain chart preservation. Its report should be consumed as a necessary domain-admission check; callers retain the existing original-curve signed replay. Do not turn `both_nonvacuously_closed` into permission to quotient the solver's equations.

The presently justified positive development controls are ordinary point-negation column handling, same-selector first-leaf swap, and identity-coordinate closure. Useful negative controls include a raw selector that closes while retained support fails, a pole, unequal leaf selectors, and an actual valid S3 witness whose coordinate-negated or reciprocal tuple fails. These discriminate the specific missing bridges without constructing a new solver or duplicating existing arithmetic.

## 9. Formalization boundary

No `lean` or `lake` executable was found by `command -v` in this runtime. No Lean file was edited or proof build claimed. Repository search found existing uses of `Finset.sum_add_distrib` and `Equiv.sum_comp`, including `research/twist_routes_1_3_20261021/lean-v4/TwistShiftOverlap.lean`; those inherited finite-sum laws are the natural starting point for formalizing section 2 in an `AddCommGroup`.

The useful first theorem would state the displayed common-sign/translation sum identity for a finite index type and a permutation, with explicit domain and target hypotheses. An elliptic-curve specialization still needs the actual group structure, selector/point-map bridge, and finite-chart predicate. A generic group identity alone would not certify the current solver's equivariance. Sections 3-7 remain written derivations in this phase, not kernel-checked theorems. [pcc:unverified]
