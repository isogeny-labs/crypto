# Torsion translates of the M61 root factor base

Run from the repository root:

`cargo run --release --offline --manifest-path research/m61_root_relation_20261126/native/Cargo.toml --example torsion_translate`

The Rust program lifts the 4950th-root x basis on `E:y²=x³−3x+40` over `p=2^61−1` and obtains **B=2447** canonical points. It takes the known full-order point `R=(0,313662643971843112)` and computes `T=[r]R=(2062051146300954192,1913788083764509161)`, where `r=329406144237565159`. It checks `T≠O`, `[7]T=O`, and exact order 7. The established point-order certificate for R and the Hasse bound certify `#E=7r`; the script directly verifies the torsion arithmetic.

For every base point P and every `j=1,...,6`, it computes `P+[j]T`, tests whether its x coordinate is among the 2447 root-base x values, and checks every hit against the canonical point or its negation. It reports directed hits, cross-index hits, self hits, unique unordered cross-index pairs, and distinct source/endpoint indices.

| Field | h | B | Root directed hits | Root cross-index hits | Random-x controls | Translation group-law calls |
|---:|---:|---:|---:|---:|---|---:|
| M61 | 7 | 2447 | 0 | 0 | one equal-size control: 0 | 14,688 per base |
| 131071 | 37 | 54 | 3 | 2 (one unordered pair) | 16 controls: mean 1.00, range 0–4 | 1,980 per base |
| 524287 | 32 | 79 | 0 | 0 | 16 controls: mean 0.75, range 0–2 | 2,480 per base |

At p131071, the three exact root hits are a self hit at base index 16 (a cofactor-torsion point) and the reciprocal cross-index pair 27↔49, with translates j=36 and j=1. The program prints their affine result coordinates and checks the signed-base point equality. The 16 random controls show comparable counts. In the p524287 control, paired directed hits likewise account for nonzero counts.

The M61 screening costs exactly `2447×6=14,682` group additions plus six additions to form `[j]T`, or 14,688 group-law calls. Computing `T=[r]R` costs 84 more group-law calls in the implemented double-and-add ladder (additions and doublings both counted). Base x generation, square roots, hash lookup, and exact-order verification are excluded from those counts.

## Normalized Vélu image check

The same Rust source also computes the normalized Vélu map with kernel `<T>` for every canonical M61 base point:

`x'(P)=x(P)+Σ_{j=1}^6 (x(P+[j]T)−x([j]T))`, and likewise for `y'`. The example also checks the paired-kernel rational x-formula against this direct sum for every base column.

It derives the quotient model `y'²=x'³+a'x'+b'` with `a'=1940049852484514886`, `b'=1624958032410326836`, and checks **all 2447 image points** against that equation. All 2447 image x coordinates are distinct; **zero lie in μ_4950**. An equal-size deterministic random-x control also has 2447 distinct images and zero μ_4950 hits. Each complete image pass costs 14,682 P-plus-kernel group additions and six kernel-multiple additions, plus 14,682 coordinate corrections and 2447 root-order exponentiations. This is a separate pass from the translation-hit scan; the two could share P-plus-kernel additions in a combined implementation.

The root condition on `x'` depends on the chosen normalized quotient coordinate model. The Rust example therefore also computes the coset label `(x')^4950` for every nonzero image. All 2,447 images are nonzero and have **distinct labels**. For any rescaling `x''=c x'` with `c∈F_p*`, an image lies in `μ_4950` exactly when its original label equals `c^(-4950)`; hence **at most one** of these images can lie in the root subgroup after any such rescaling. This includes every `F_p`-isomorphism between short Weierstrass quotient models, whose x-change is a scaling. It does not cover arbitrary nonlinear coordinate changes or a different isogeny. A uniform-coordinate heuristic expects only `2447·4950/p≈5.25×10⁻¹²` hits in the normalized model, so zero there is unsurprising; the distinct-label census is the stronger selected-instance result.

For a uniformly scattered signed base, expected directed hits are approximately `B(h−1)·2B/#E`. This is only `3.12×10⁻¹¹` at M61, so zero hits in the root and random bases is the ordinary prediction, not strong evidence against hidden structure. The exact result does falsify this particular torsion-translate source of free relations on the selected M61 root base. It says nothing about relations through higher-degree isogenies or an implicit algebraic representation.
