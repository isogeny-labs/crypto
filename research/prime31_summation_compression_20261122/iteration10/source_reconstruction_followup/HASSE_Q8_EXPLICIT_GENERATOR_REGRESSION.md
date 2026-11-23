# Explicit F8 Mumford generator for the inherited order-BSGS regression

Date: 2026-11-23. Research lane: [PR #4977](https://github.com/isogeny-labs/crypto/pull/4977). Input head: `e9d788b0ea603db0fdcc1935b3f6858389ddcdd8`. This is an isolated mathematical producer computation and original-root **test specification**, not compiled Rust, independent signed validation, or a production change. **[pcc:unverified]**

## Concrete class, no existential generator left to find

Work in `F8 = F2[z]/(z^3+z+1)`. Encode `a0+a1*z+a2*z^2` by the integer `a0+2*a1+4*a2`. In particular `z=2`, `z²+z=6`. The genus-two curve is

```
C: y²+y=x⁵+x+1
m=3; irreducible polynomial low terms [0,1]; h(x)=1
f coefficients low to high = [1,1,0,0,0,1]
```

The affine point `P=(z,z²+z)=(2,6)` lies on `C(F8)`. Its reduced degree-one Mumford divisor

`D=[P-∞]=(u=x+2,v=6)`

is an **explicit generator of order 117**, not just a class whose existence follows from the Jacobian structure. The group-cardinality input `#Jac(C)(F8)=117` is established in the preceding [cyclic BSGS note](HASSE_Q8_CYCLIC_117_BSGS_FAILURE.md), where `#C(F8)=13` and `#C(F64)=81` are derived by trace counting.

## Exact scalar-multiple oracle values

All coefficients are canonical `F8` integer encodings, in ascending polynomial degree. `u` is monic and `deg(v)<deg(u)`; `u=[1],v=[]` denotes the identity.

| scalar k | u coefficients | v coefficients |
| ---: | --- | --- |
| 0 | `[1]` | `[]` |
| 1 | `[2,1]` | `[6]` |
| 2 | `[4,0,1]` | `[3,7]` |
| 3 | `[1,7,1]` | `[6,5]` |
| 4 | `[5,1]` | `[1]` |
| 8 | `[7,0,1]` | `[0,2]` |
| 9 | `[5,1,1]` | `[4,5]` |
| 13 | `[4,1,1]` | `[6,5]` |
| 39 | `[1,1,1]` | `[]` |
| 78 | `[1,1,1]` | `[1]` |
| 117 | `[1]` | `[]` |

Thus `[9]D != O` proves that the order contains a factor 13, while `[39]D != O` proves that it contains the full factor 9. Since the Jacobian has order `9·13=117`, Lagrange forces **ord(D)=117**. The explicit `[39]D` and `[78]D` are inverse nonzero 3-torsion classes; their `u=x²+x+1` divides `f=x⁵+x+1`, furnishing a simple extra consistency check.

## Reproducible mathematical group-law specification

For two reduced classes `(u1,v1),(u2,v2)` satisfying `u_i | (v_i²+v_i+f)` in characteristic two, compute an extended gcd certificate
`d = a*u1 + b*u2 + c*(v1+v2+1) = gcd(u1,u2,v1+v2+1)`. Then form

```
u = (u1*u2)/d²
v = (a*u1*v2 + b*u2*v1 + c*(v1*v2+f))/d  mod u
while deg(u)>2:
    u_new = monic((f+v+v²)/u)
    v_new = (v+1) mod u_new
    (u,v)=(u_new,v_new)
u=monic(u)
```

All operations are in `F8[x]`. For each displayed nonidentity pair, check `u | (v²+v+f)`. The same-coordinator bounded mathematical computation produced 117 distinct reduced classes `[k]D` for `0<=k<117`, returned the identity at `[117]D`, agreed between repeated addition and double-and-add for all k, and checked 289 sampled pair-addition results. Those are *producer-side abstract finite-field arithmetic checks*, not original-Rust execution or independent oracle signatures. No timing or cryptographic performance is implied.

## Exact frozen-source Rust regression contract

Use the existing `F2mElement`, `F2mPoly`, `HyperellipticCurve::try_new`, `MumfordDivisor` and `scalar_mul_fast` from the original repository. For the `m=3` irreducible polynomial `z³+z+1`, `h=[1]` and `f=[1,1,0,0,0,1]`, make `D` with the existing `divisor_for_point(curve,&fe(3,2),&fe(3,6))`. Check on-curve, equality of the displayed 9D,13D,39D,78D and 117D reduced representations, the inverse relation, and `scalar_mul_fast(117)==O`. Then call the unchanged `divisor_order_bsgs(curve,&D,4096)` **before any repair**: its interval `[1,81]` and searched candidate multiples `1..=110` exclude the true order 117, predicting `None`. After a separately reviewed fix to the odd-m upper endpoint, require `Some(117)` on the same exact input. If any prior native result disagrees, retain the failure and investigate reduction normalization/group arithmetic rather than modifying this oracle retroactively.

Do not treat a standalone substituted module as the original-root validation gate. Preserve the tests and implementations from **both** parents of merge `77d388738a48eb5a6e2abd80636c2e31cceab254`, inspect the published complete-union candidate blob `616422524711c1dd0e54b6bb6c557348b999e262`, and run normal full-root checks only after source freeze and budget admission. The independent validator must reconstruct at least the `9D` and `39D` classes using separately reviewed arithmetic, rather than trusting the table. No production source, shared workflow, frozen evidence, sibling PR #4979, Space or claim-ledger mutation is authorized by this note.

## Coordinator review and next step

Algebra deliverable: a specific class and complete numerical Mumford witnesses strengthen the previous existential cyclicity/loop argument. Validation deliverable: deterministic small-field, sign, representation, exact-order and old/fixed-BSGS checks on a frozen source. Rust implementer: **deferred**; the lane's original module is not yet reconciled, native Rust execution is unavailable here, and this result is a correctness control rather than a selected new Prime31 relation-finder primitive. Native child-agent capacity was unavailable; this is one coordinator's work, not an independent-worker agreement. No extra automation, benchmark or paid compute was initiated. The separate sublinear empty-inclusive Prime31 S3 query objective remains open. **[pcc:unverified]**
