# Frozen exhaustive degree-four pole-shape screen for the +26 witness

15 November 2026. Source curve, target curve and field are exactly those in `DEGREE4_PROTOCOL.md`. This protocol extends the successful 405-assignment partial screen; its `degree4_screen.json` must be preserved, including the zero numerator count. [pcc:unverified]

## Why the denominator list is complete

Translate a hypothetical degree-four map `φ:Y_K→E` so the rational Weierstrass point at infinity maps to `O`. Then `φ∘ι=[−1]∘φ` for the hyperelliptic involution `ι`, so `u=x_E∘φ=A(x)/B(x)` and `v=y_E∘φ=y·R(x)`. The degree of the rational function `A/B` on `P¹_x` is four. If infinity has ramification index `e∞`, the pole order of `u` there is `2e∞`; since `x` has pole order two, `deg A=4` and `deg B=4−e∞`. The local parameter changes sign under both involutions, so `e∞` is odd: one or three. [pcc:unverified]

At a finite branch root of `f`, a pole of `u` has odd multiplicity in `B`; at a nonbranch root it has even multiplicity. Over `K`, `f=(x+1)Q1···Q5`, with five irreducible quadratic factors. Therefore all monic denominator shapes of degree at most three are exactly:

1. `B=(x+1)Qi`, five cases (`e∞=1`), already screened;
2. `B=(x+1)(x−c)²` for each `c∈K` other than the branch root, 242 cases (`e∞=1`);
3. `B=(x+1)³`, one case (`e∞=1`);
4. `B=x+1`, one case (`e∞=3`).

The total is 249 denominators. The rational branch root is always a pole because `deg B` is odd. The quadratic branch factors are either included whole with multiplicity one or excluded. No nonrational nonbranch pole can fit in degree at most three with even multiplicity. [pcc:unverified]

## Necessary branch interpolation

Every remaining finite Weierstrass point maps to a finite `E[2]` point. Each `Qi` pair has three possible Galois-compatible values: the rational 2-torsion root, or one of the two roots of the quadratic 2-torsion factor. For each `B`, impose `A≡B·ej (mod Qj)` on every `Qj` absent from `B`; a unique CRT residue modulo their product follows. A degree-four map in the specified denominator class requires that residue to have degree exactly four and `gcd(A,B)=1`. For `B=(x+1)Qi` there are `3^4=81` assignments; for each of the other 244 denominators there are `3^5=243`. The total is `5·81+244·243=59,697` assignments. A zero degree-four CRT count excludes **every degree-four map from this exact `Y` to this exact `E` over `K`**, since these are necessary conditions. Any surviving numerator must pass the full elliptic curve equation before a map is accepted. [pcc:unverified]

The implementation may precompute all 243 CRT residue functions on the five quadratics and multiply by each denominator modulo `f/(x+1)`. It must emit counts by denominator type, the source SHA-256, and every surviving numerator. Freeze the code in a commit before measuring. Independent replay must solve the corresponding eight- or ten-equation linear systems directly rather than using the CRT residue routine before a universal degree-four exclusion is reported. [pcc:unverified]

The fixed target `E:y²=u³+2u²+a` is valid for the Howe window: its discriminant `26²−4·243=−296` is fundamental, so `End(E)=Z[π]`. The result would not exclude degree-four covers to a *different isogeny class* or covers on other genus-five carriers. [pcc:unverified]
