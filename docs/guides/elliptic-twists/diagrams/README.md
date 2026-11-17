# Elliptic curve twist diagrams

All counts include the identity O. Affine grids show the remaining points.
Symmetry polygons represent automorphism groups, not drawings of the curves.

## A hidden bridge

Actual affine points over F7 for y^2=x^3+1 and y^2=x^3+6. The identity O is included in the stated counts. They become isomorphic over F49.

[PNG](01_hidden_bridge.png) · [SVG](01_hidden_bridge.svg)

## A finite-field curve is a set of points

A complete F7 grid for y^2=x^3+1, including an example modular check and the identity point in the total.

[PNG](02_finite_curve_points.png) · [SVG](02_finite_curve_points.svg)

## Extending the coordinate toolbox

The field F49 is obtained by adjoining a square root of 3 to F7. It is a field of pairs, not integers modulo 49.

[PNG](03_field_toolbox.png) · [SVG](03_field_toolbox.svg)

## A point crossing the hidden bridge

An explicit isomorphism from y^2=x^3+1 to y^2=x^3+6 over F49. A base-field point can map to a point with extension-field coordinates.

[PNG](04_explicit_hidden_map.png) · [SVG](04_explicit_hidden_map.svg)

## Quadratic twisting flips square choices

The complete fiber-by-fiber counts for y^2=x^3+1 and 3v^2=x^3+1 over F7. Each x contributes two points across the two models.

[PNG](05_quadratic_square_flip.png) · [SVG](05_quadratic_square_flip.svg)

## Why 2, 3, 4 and 6 appear

The Weierstrass coordinate weights force the generic square-root case and permit fourth or sixth roots when a coefficient vanishes. Cubic twists are a subfamily of sextic twists.

[PNG](06_coordinate_weight_tree.png) · [SVG](06_coordinate_weight_tree.svg)

## More symmetry permits more twists

Symbolic diagrams of the origin-preserving geometric automorphism groups C2, C4 and C6. These polygons are not drawings of the curves.

[PNG](07_symmetry_groups.png) · [SVG](07_symmetry_groups.svg)

## The cubic-twist triangle

The three cubic twists of y^2=x^3+1 over F7 have coefficients 1, 2 and 4, and point counts 12, 9 and 3. Arrows represent multiplication of B by 2, a square, with a cube-root coordinate change.

[PNG](08_cubic_family.png) · [SVG](08_cubic_family.svg)

## The quartic-twist family

The four quartic twist classes over F13 of y^2=x^3+x, with coefficients 1,2,4,8 and exact extension degrees 1,4,2,4.

[PNG](09_quartic_family.png) · [SVG](09_quartic_family.svg)

## All six sextic-twist classes

Six F7-isomorphism classes with j=0. Each node gives B, the actual point count, and the minimal isomorphism-extension degree relative to B=1. Dashed purple lines identify its cubic subfamily.

[PNG](10_sextic_family_wheel.png) · [SVG](10_sextic_family_wheel.svg)

## The field changes the number of twists

For j=0 and characteristic greater than 3, the sextic classes are multiplicative cosets modulo sixth powers. This gives six classes over F7 and two over F5.

[PNG](11_field_changes_class_count.png) · [SVG](11_field_changes_class_count.svg)

## Frobenius is the visibility test

Base-field rational points are fixed by Frobenius. Under a quadratic untwist, rational points on the twist correspond to anti-fixed points pi(P)=-P, which are fixed by pi squared. Two-torsion can satisfy both conditions.

[PNG](12_frobenius_visibility.png) · [SVG](12_frobenius_visibility.svg)

## Why the quadratic-twist order matters

A Montgomery single-coordinate ladder can process inputs associated with either the curve or its quadratic twist. Each group needs the order properties required by the protocol. A nonzero right-hand side is assumed in the branch labels.

[PNG](13_twist_security.png) · [SVG](13_twist_security.svg)

## A sextic twist makes pairing points cheaper

BLS12-381 represents G2 on its M-type sextic twist over Fp2. The untwist embeds the selected subgroup in E(Fp12). Pairing values live in a multiplicative subgroup of Fp12, a different object from elliptic-curve points.

[PNG](14_pairing_field_tower.png) · [SVG](14_pairing_field_tower.svg)
