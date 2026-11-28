# Rational 7-torsion versus the root-of-unity factor base

Research note, 2026-11-26. Selected curve `E/F_p: y²=x³−3x+40`, `p=2^61−1`, `#E=7r`. The point

`T=[r]R=(2062051146300954192,1913788083764509161)`

has order 7. This tests whether translation by its rational kernel, or the associated degree-7 isogeny, gives a new relation shortcut for the 2,447 lift-valid x-coordinates in `H=μ_4950`. It goes beyond the earlier cofactor-class count by checking **actual incidence of H with torsion translates**.

## Exact x-translation and isogeny map

Write `f(x)=x³−3x+40`, `T_j=[j]T=(t_j,v_j)` for `j=1,2,3`, and `P=(x,y)`. Away from `x=t_j`, the chord law gives

`x(P+T_j)=U_j(x)−2yv_j/(x−t_j)²`,

`x(P−T_j)=U_j(x)+2yv_j/(x−t_j)²`,

where `U_j(x)=(f(x)+f(t_j))/(x−t_j)²−x−t_j`. The two branches are the roots of `F₃(x,t_j,X)`. Since `v_j≠0`, their difference is a nonzero multiple of `y`; **translation by `T_j` does not induce a single rational function of x alone**. Changing the sign of `P` swaps the branches.

Adding both branches cancels `y`. Vélu's quotient by `K=⟨T⟩` therefore has the explicit rational x-map

`x(φ(P))=V(x)=x+2 Σ_(j=1)^3 (U_j(x)−t_j)`.

The denominator divides `∏_(j=1)^3(x−t_j)²` (degree 6); the numerator has degree 7 because `V(x)=x+O(1/x)` at infinity. This is a genuine low-degree coordinate map, but it intertwines rather than simplifies the large-prime DLP: `φ(Q)=[d]φ(G)` iff `Q=[d]G` on the r-subgroup, since `K` has order 7 and is disjoint from that subgroup.

## Exhaustive incidence check on this instance

The native [torsion probe](torsion_translate.md) enumerates the 2,447 canonical lifts and all six nonzero translations `P+[j]T`, using **14,682 screening group additions**. This also covers the negative lifts because `x(−P+[j]T)=x(P−[j]T)` and `j` runs through all six nonzero residues. It found:

| Tested property | Exact count |
|---|---:|
| Translated point with x still in all 4,950 roots `H` | 0 |
| Translated point in the lift-valid signed base | 0 |
| Translation to infinity | 0 |

Thus the signed root base intersects each `K`-orbit in at most one point. In particular, no rational 7-torsion translate identifies two existing factor-base columns under `φ` or under `[7]`. The same Rust example compares the rational Vélu formula above with the direct six-kernel-point sum on all 2,447 base columns, checks every quotient image against the derived quotient curve, and finds **2,447 distinct image x-values**, none in `H`. All 2,447 are nonzero and occupy **different multiplicative cosets of `H`**: their `(x')^4950` labels are distinct. Thus multiplying every quotient x-coordinate by any fixed nonzero field element can put at most one image into `H`. This covers all isomorphic short-Weierstrass quotient models over `F_p`, where the x-change is a scaling; other coordinate constructions remain open.

## Cost and rank implications

Computing `T_j` costs `O(log r)` group operations. Evaluating `V(x)` at all `B` roots costs `O(B)` field inversions (or one batched inversion pass); the image still has `B` distinct x-columns and loses the sparse constraint `x^4950=1`. Building its pair table and direct three/three or three/four searches still costs `Θ(B²)`, `Θ(B³)`, or `Θ(B⁴)` respectively. The isogeny supplies no collision compression of the root base.

One can force a torsion-orbit-closed base by adjoining every `P+[j]T`. The check above implies exactly `7B=17,129` distinct x-columns in that enlarged base. But `[7](P+[j]T)=[7]P`, so its seven columns per original column have the **same r-subgroup logarithm**. The six extra columns per orbit add projection-equivalent rows, not independent prime-subgroup information. Since `r mod 7=2`, translation changes cofactor class by `2j mod 7` and visits every class; it can normalize the cofactor of a point, but projected relation verification already removes that cofactor without enlarging the base. Any extra raw decomposition multiplicity is a bounded factor in this degree-7 construction, not an exponent change.

This is an exact obstruction to the proposed H-preserving torsion-translation or 7-isogeny compression **for this selected curve and base**. It is not a lower bound on other factor bases, isogenies, or implicit coordinate solvers.

Rational torsion as an index-calculus symmetry has prior art, including Renault, *On Using Torsion Points in the Elliptic Curve Index Calculus* (ECC 2014, https://www.imsc.res.in/~ecc14/slides/guenael.pdf). The repository also studies torsion and isogeny factor-base transports. This note gives a selected-instance incidence and rank check, not a new torsion technique.
