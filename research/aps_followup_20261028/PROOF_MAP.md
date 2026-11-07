# Proof map and implementation obligations

All new declarations are in `lean/Line.lean`, namespace `APSFollowup`, with
Lean `v4.34.1`, core only. They use `Lean.Grind.CommRing`, so the coefficient
identities specialize to every prime field without characteristic-specific
division. The proof gate checks the actual file and transitive axioms.

Both the repository gate and the shared CryptoLean gate at
`148f5dc389b341401e06d76abe7783c4ccf42f2e` were executed successfully.
Their exact gate/auditor blobs are pinned in
[`lean/proof-dependencies.json`](lean/proof-dependencies.json). This uses
the shared checking tool; no CryptoLean/Mathlib module is imported or built.

| Statement | Lean declaration | Exact scope |
| --- | --- | --- |
| Substituting y=tX+c gives the monic intersection cubic | `line_norm` | Every commutative ring |
| A root of that cubic gives a point satisfying the curve equation | `line_root_on_curve` | Every commutative ring, assuming the displayed root equation |
| Global sign reversal of the line keeps its abscissae | `signed_line_same_cubic` | Every commutative ring |
| Propagated orbit coefficients sum to zero | `orbit_label_sum` | Every commutative ring, assuming lambda^2+lambda+1=0; applied separately to u and v |
| The horizontal j=0 intersection is X^3+b-c^2 | `horizontal_cubic` | Every commutative ring |
| Saturation t*z-1=0 excludes t=0 | `saturation_excludes_zero` | Explicit nontriviality hypothesis 0 != 1 |
| Every supplied inverse survives the saturation equation | `saturation_keeps_inverse` | Assumes t*z=1; a field supplies such an inverse for nonzero t |

All seven declarations passed the pinned gate with zero trusted
declarations. Their transitive axioms are only `propext`, `Classical.choice`,
and `Quot.sound`. There is no `sorry`, custom axiom or native-decide bypass.

## Readable completeness argument

Let V be a set of distinct rational abscissae on a nonsingular short
Weierstrass curve over F_p, p>3. Its vanishing polynomial F_V is monic and
squarefree. If the monic cubic H(t,c) divides F_V, it splits into three
distinct roots in V. The line identity gives points (x,tx+c) on the curve.
The chord law says their sum is O. Conversely, a signed zero-sum triple
with distinct abscissae lies on its chord, with finite slope, so its monic
intersection polynomial is H(t,c) and divides F_V. Vertical-line and repeated
root cases do not satisfy the distinct-abscissa hypothesis. Horizontal
lines use t=0 directly, without division.

For j=0 and nontrivial beta of order three, any horizontal intersection
with distinct roots is {x,beta*x,beta^2*x}; x cannot be zero. On an orbit
with labels propagated by a checked lambda, the signed sums of both labels
vanish by `orbit_label_sum`. In the declared prime subgroup of order n>3,
the other sign patterns cannot sum to O: subtracting them from the full
orbit sum would give twice a nonzero point equal to O. Thus these removed
components contain no informative relation. The equation t*z-1=0 retains
every nonhorizontal line with its unique inverse z. The native builder
checks orbit closure, eigenvalue action on G and Q, labels and sign flips;
it refuses independent relabeling even when the geometry is identical.

## Formalization boundary

The full squarefree-divisor/root-count implication, chord law, signed
subgroup argument, and translation of the Rust expression builder to Lean
are not formalized here. They have a readable argument above and exact
finite-instance tests. These seven lemmas are a partial formalization of
the complete encoding/pruning theorem, not a full Lean proof of it.

The identity certificate is an additional native replayable coefficient
check over the certificate format's field 2^61-1. It is not a replacement
for the Lean statement or a formal proof of elliptic-curve completeness.
