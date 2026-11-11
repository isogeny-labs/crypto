# CT09 / CT03: sextic-twist genus-2 factors and a genuine Fp cross-factor 3-isogeny

**Date:** 2026-11-11. **Scope:** independently derived family-level mathematical follow-up to #3877 and #3907; this note is not a signed PCC verification verdict and not a claim of algorithmic improvement. [pcc:unverified]

## 1. Parameterized two-quotient family

For any field k of characteristic different from 2,3 and any nonzero `b ∈ k`, put

- `C_b: y² = x⁶ + b` (smooth genus 2);
- `E_b: v² = u³ + b`;
- `E_{b²}: V² = U³ + b²`.

There are two degree-2 morphisms on smooth projective models:

`f_b(x,y)=(x²,y): C_b -> E_b`,

`g_b(x,y)=(b/x², b*y/x³): C_b -> E_{b²}`.

For the latter, substitution gives `(b*y/x³)²=(b/x²)³+b²`. The apparent poles at `x=0` extend to infinity in `E_{b²}`; the formula on the dense open determines the projective morphism.

Their deck involutions are `σ(x,y)=(-x,y)` for f and `τ(x,y)=(-x,-y)` for g. Because `g∘σ=-g` and `f∘τ=-f`, the norm/pullback identities on Jacobians are

`f_*f^*=[2] on E_b; g_*g^*=[2] on E_{b²}; g_*f^*=0; f_*g^*=0`.

The map `E_b × E_{b²} -> J(C_b)`, `(P,Q)↦f^*P+g^*Q`, is an isogeny (its dual composition is `[2]×[2]`). Thus the zeta numerator is the product of the two elliptic factors' numerators. These are standard (2,2)-split Jacobian facts, **not new mathematics**.

**Crucial typing constraint on #3907's shear:** the expression `g^*f_*` is not even defined when the two elliptic codomains differ. A cross-factor morphism `η:E_b→E_{b²}` is needed first. Identical j-invariants do **not** supply an Fp-defined map.

## 2. Explicit nonisomorphic F101 factor pair with a rational degree-three isogeny

For every `b ≠ 0` and `char(k) ≠ 2,3`, the standard separable degree-3 isogeny with geometric kernel `{O,(0,√b),(0,-√b)}` is

`h_b(x,y)=(x+4b/x², y(1-8b/x³)) : E_b -> E_{-27b}`,

extended at x=0 and infinity. Direct polynomial identity:

`(x³+b)(x³-8b)² - (x³+4b)³ + 27b*x⁶ = 0`.

If `u∈k*` satisfies `u⁶ = -b/27`, scaling `(X,Y)↦(u² X,u³ Y)` maps `E_{-27b}` onto `E_{b²}`. Hence

`η_b(x,y)=(u²(x+4b/x²), u³*y(1-8b/x³)): E_b -> E_{b²}`

is a degree-3 **k-rational isogeny**.

When k=Fp with `p≡5 (mod 6)` and b a nonsquare, such u always exists: sixth powers in Fp* equal squares, while `-3` and b are nonsquares; therefore `-b/27` is a square. The two factors are nonetheless **not Fp-isomorphic**, since `E_{b²}/E_b` has sextic twist ratio b, a nonsixth-power. They are geometrically isomorphic over an extension, and Fp-isogenous by η_b.

Concrete exact fixture: p=101, b=2, u=5, `5⁶*(-27*2)=4 (mod 101)`. Both elliptic groups have 102 rational points, but E_2 and E_4 are not F101-isomorphic. Their order-17 source generator `P=(28,21)` has `η_2(P)=(52,11)` and the eventual mixed endpoint is `η_2([4]P)=(78,82)`.

Define `K=g^*∘η_b∘f_*` on J(C_b). The cross-norm identity implies `K²=0`; hence `M=1+K` is an algebraic-group automorphism with inverse `1-K`. Its new cross-factor composite is

`g_* M f^* = g_*g^* η_b f_*f^* = [4]∘η_b ≠ 0.`

For an odd prime r other than 3 (and r not dividing 4), this is injective on any E_b[r]-subgroup where η_b is nonzero; in particular on r=17. **Unlike b=1**, these two Fp elliptic factors are nonisomorphic: the shear is now well-typed only because the explicit degree-3 isogeny was supplied. However the full chain **collapses to an ordinary degree-3 elliptic isogeny plus [4]**. It does not bypass an isogeny barrier or demonstrate any DLP speedup.

Moreover, K≠0 and End(J) is torsion-free, so `M^n=1+nK` are distinct for all integers n. M has infinite order as an unpolarized Jacobian automorphism. Automorphisms preserving a principal polarization form a finite group; in particular M cannot be induced by a genus-2 curve automorphism. This blocks an unjustified interpretation of the shear as an inexpensive curve-coordinate permutation.

## 3. In-family negative control: non-isogenous F103 elliptic factors

For `p=103, b=2`: direct brute point counts give

| object | rational cardinality |
|---|---:|
| E_2(F103) | 117 (trace -13) |
| E_4(F103) | 111 (trace -7) |
| C_2(F103) | 124 |
| C_2(F103²) | 10804 |
| J(C_2)(F103) | 12987 = 117×111 |

The L-polynomial `(1+13T+103T²)(1+7T+103T²) = 1+20T+297T²+2060T³+10609T⁴` matches the independently counted C-points over F103 and F103². Both elliptic groups have rational 3-torsion (`(0,38)` on E_2 and `(0,2)` on E_4) and gcd of group orders 3, but their **different F103 traces** forbid nonzero Hom over F103. In particular `u⁶=-2/27=19 mod 103` has no solution in F103; the F101 mixing construction does not generalize indiscriminately. These same-j elliptic curves **are isomorphic over an algebraic extension**: the obstruction is over the base field, not over the algebraic closure.

## 4. Reproduction record and limits

Executed **locally**, with Python stdlib arithmetic as an *independent scratch checker*, rather than as new repository research runtime:

- p=101, b=2: exhaustively enumerated all 102 source and destination elliptic points; verified η's target equation and homomorphism for **all 102×102 ordered pairs including infinity**, and checked **all 17** scalar relations for `η∘[4]`. `P=(28,21)`; all passed.
- p=101, b=2: independently checked both quotient equations on all 100 affine points of `C_2(F101)`; `#C=102` including two points at infinity.
- p=103, b=2: 122 affine C points, 60 nonramified split f-fibers; verified g target equation and the identity `g(P)+g(σP)=O` on every such fiber.
- p=103, b=2: counted `#C(F103²)=10804` by an explicit square-table on `F103[t]/(t²-3)`, not by assuming the isogeny splitting. All checks passed.

**Not performed:** evaluation of M on arbitrary Mumford divisor classes, an independent Rust/native reproduction, performance pricing against the direct η shortcut, compiler/CI tests for this note, a signed PCC/Lean geometry certificate, or a paid benchmark. The fixture vectors are in `sextic_twist_3isogeny_vectors.json`. Do not promote this to a solver-speedup result.

## 5. Impact on CT09/CT05

Native parameterization should accept distinct **typed** elliptic factors `E_b` and `E_{b²}`, and an **optional explicit cross-factor isogeny** (degree, formulas, field of definition, rational kernel); reject a bare same-j claim. Add p=101,b=2 as the positive nonisomorphic 3-isogeny control and p=103,b=2 as the no-Fp-Hom negative control. A valid cost comparison must include `η([4]Q)` as the collapsed-arm reference, plus raw source rho and full Jacobian route; expect the Jacobian route to lose against this shortcut unless it exposes a new relation algorithm.

## Prior art and coordination

Primary context: Shaska, *Genus two curves covering elliptic curves* (arXiv:1209.3187), degree-two elliptic subcovers and (2,2)-split Jacobians; Bruin–Doerksen, *The Arithmetic of Genus Two Curves with (4,4)-Split Jacobians* (arXiv:0902.3480); standard Vélu isogeny formulas. Explicit twist arithmetic here is a specialization, not claimed novel. Research-KB and Conductor control-plane were unavailable in this invocation (`conductor: command not found`); therefore changes are isolated to a new subfolder and standalone feature branch based on #3907, with **no modification to #3877/native, signed evidence, existing ledgers, or active task cards**. Queue #3904 and original resume state are authoritative.
