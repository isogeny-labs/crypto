# Dual projected-base transport proof map

## Readable argument

Let `φ:E→E′` be the normalized degree-three Vélu map with kernel `{O,±(3,1)}` over `F_(211²)`, and let `ψ:E′→E` be its dual. The dual-composition identities `φψ=[3]` and `ψφ=[3]` are standard; see the [Sage isogeny reference, `dual()`](https://doc.sagemath.org/html/en/reference/arithmetic_curves/sage/schemes/elliptic_curves/ell_curve_isogeny.html#sage.schemes.elliptic_curves.ell_curve_isogeny.EllipticCurveIsogeny.dual). The concrete map and scaling in this study are checked independently on every rational point.

For every point `Y` of order dividing 229 on `E′`, define `X=[153]ψ(Y)`. Since `3·153=459=2·229+1`, homomorphism and `[229]Y=O` yield

`φ(X)=φ([153]ψ(Y))=[153]φψ(Y)=[459]Y=Y`.

Moreover, `[229]X=O` because `ψ` is a homomorphism. Thus `X` is an order-229 source-subgroup preimage of `Y`. If two order-229 source points map to the same destination point, their difference lies in the degree-three kernel and has order dividing 229, so it is the identity. The preimage is unique within this subgroup. This is a transport identity and uniqueness statement; it does not increase the relation support of a fixed projected base.

## Formal and executable status

| Statement | Lean declaration | Exact hypotheses | Status |
| --- | --- | --- | --- |
| `153·3≡1 mod 229` | `DualProjected229.inverseThree` | scalar ring `Fin 229` | checked in `LEAN_BUILD_V4.stdout` |
| `[153]([3]y)=y` | `DualProjected229.inverseThreeAction` | `y:Fin 229` | checked in `LEAN_BUILD_V4.stdout` |
| conditional transport | `DualProjected229.projectedTransport` | two functions on `Fin 229`, `forward(dual y)=3y`, and `forward(n x)=n forward(x)` | checked in `LEAN_BUILD_V4.stdout` |
| concrete `φψ=[3]` and `ψφ=[3]` on the stated curves | none | verified formulas and exceptional cases for all `E(F_(211²))`, `E′(F_(211²))` | finite native and independent checker gates in `RUN_V1/DUAL_IDENTITY*.json`, not a Lean curve-map theorem |
| subgroup uniqueness | none | `ker φ` has order three and target subgroup has order 229 | readable proof above; Lean curve-group bridge open |

The Lean project pins `leanprover/lean4:v4.34.1` and has no external package dependency. `lake build DualProjected229` passed in V4. `#print axioms` reported only standard `propext` for all three declarations, with no `sorryAx` or custom research axiom. Failed V1–V3 build attempts are retained. `lean4checker --fresh` is unavailable on this host, so a fresh independent Lean kernel recheck remains open. The formal theorem is conditional at the scalar-coordinate level; it does not claim to formalize the concrete elliptic curves or the Vélu map.
