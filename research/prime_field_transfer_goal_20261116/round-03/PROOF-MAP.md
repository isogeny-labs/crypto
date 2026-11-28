# Round03 instance gate and proof boundary

1. **Structure above:** Mathlib `Zsqrtd (-3)` with `norm`, `norm_def`,
   `norm_mul`; `Polynomial ℤ` with its commutative ring instance;
   concrete `ZMod 19` and `ZMod 79` rings. No new CM structure is invented.
2. **Theorems used as stated:** `Zsqrtd.norm_def`, `Zsqrtd.norm_mul`,
   existing ring normalization and finite-ring arithmetic. These certify
   norms of scaled CM parameters, polynomial parameter identities and the
   literal small-fixture differential/eigenvalue arithmetic only.
3. **Objects as instances:** `2*pi=1+b*sqrt(-3)` and
   `alpha=2-b*sqrt(-3)`, represented by existing `Zsqrtd` pairs. Use b=5
   and b=1146303377458733895550677674601920593921 (m112). The ring
   Z[sqrt(-3)] contains these scaled quantities; pi itself belongs to the
   larger Eisenstein order and is NOT defined here by integer division.
   The polynomial variable is Mathlib `Polynomial.X` over ℤ.
4. **Assembly probe:** apply `norm_def` to an arbitrary existing `Zsqrtd`
   value, `norm_mul` to two such values, and their literal small instances.
   Type-check this assembly before adding the concrete endpoint module.
5. **Missing components:** CM endomorphisms over Q(omega), the chosen sextic
   twists' Frobenius identification, elliptic primality/Hasse certificates,
   the étale kernel's global Galois representation, oriented common-prime
   embeddings, native Smart arithmetic and source-to-target evaluation are
   not formalized. No hypothesis asserting an evaluator is counted as its
   construction. Scalar-preserving homomorphism laws already exist in
   Mathlib; the missing input is a computable homomorphism for these points.
6. **Inheritance table:** below. No general law is duplicated in this
   study; all exported statements are concrete instance obligations or
   existing-law endpoints. No heartbeat/recursion increase, native_decide,
   custom axiom or study-local ring/group law.

| Operation | Inherited from |
| --- | --- |
| Pair value and norm | `Zsqrtd`, `Zsqrtd.norm_def` |
| Norm of a product | `Zsqrtd.norm_mul`, applied as stated |
| Integer polynomial operations | `Polynomial ℤ` commutative ring instance |
| Polynomial identities | kernel-checked `ring` normalization |
| Literal integer calculations | kernel-checked `norm_num` |
| Small residue rings | existing `ZMod` ring instances; kernel reduction |

The proof chain for each norm endpoint is norm of the literal pair =
re²+3*im² (`norm_def`) = the recorded integer (`norm_num`). The product
endpoint first uses `norm_mul`, then the two literal norm endpoints. The
polynomial equalities are finite symbolic instance certificates, accompanied
by IDC1 records; they do not establish prime density or a transfer algorithm.

| Statement | Kind / owner | Exact scope |
| --- | --- | --- |
| `small_twice_pi_norm` | endpoint / CMTargetBoundary | norm(1+5√−3)=76=4*19 |
| `small_alpha_norm` | endpoint / CMTargetBoundary | norm(2−5√−3)=79 |
| `small_product_norm` | endpoint / CMTargetBoundary | multiplicative norm at those two values |
| `large_twice_pi_norm` | endpoint / CMTargetBoundary | norm of the actual m112 scaled parameter is4r |
| `large_alpha_norm` | endpoint / CMTargetBoundary | norm of the actual m112 alpha is p |
| `target_orientation` | instance / CMTargetBoundary | roots7/11 and pi values0/1 modulo19 |
| `source_orientation` | instance / CMTargetBoundary | root55 and alpha value0 modulo79 |
| `parameter_norm_identity` | instance / CMTargetBoundary | exact integer polynomial identity |
| `parameter_factor_identity` | instance / CMTargetBoundary | exact integer polynomial factor identity |
| Global degree r−1 | written CM implication only | auxiliary scalar order + classical CM interpretation |
| Arbitrary source Q → compatible target residue | OPEN | no evaluator, no formal map |
