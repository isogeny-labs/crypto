# Same-support zero-slope branch: bounded derivation for review

Status: written derivation and proposed Lean statements, **not kernel checked**. No source edit, build, trial or universal addition theorem. Source references and hashes are retained in `law-source-hashes.json`; this note uses `src/bin/projective_law_gate.rs:896–930`, normalization at `:551–568`, and `formal/RuntimeSemantics.lean:89–153`.

## Domain and actual runtime expressions

Work over a field K. For input i=1,2 let the runtime tuple be `(q_i,r_i,s_i,t_i,z_i,w_i)`, with `z_i≠0`, `w_i≠0`. Interpret it exactly as normalization does:

```
q̄_i = q_i / z_i²          r̄_i = r_i / z_i⁴
s̄_i = s_i / (z_i³ w_i)   t̄_i = t_i / (z_i⁵ w_i)
u_i = X² + q̄_i X + r̄_i
v_i = s̄_i X + t̄_i
```

Assume both inputs are valid for the **same polynomial f**: `u_i ∣ f-v_i²` in K[X]. For this narrowly algebraic implication, smoothness, quintic degree, and odd characteristic are unnecessary. They remain part of the intended group-law domain; odd characteristic is required by the existing classifier theorem we will invoke.

The Rust source computes:

```
Qc1 = q1 z2²            Qc2 = q2 z1²
Rc1 = r1 z2⁴            Rc2 = r2 z1⁴
Sc1 = s1 z2³ w2         Sc2 = s2 z1³ w1
Tc1 = t1 z2⁵ w2         Tc2 = t2 z1⁵ w1
```

The branch under consideration has `Qc1=Qc2`, `Rc1=Rc2`, and `Sc1-Sc2=0`. The source first rejects inverse and equal ordinate pairs before testing that last condition. Let `Z=z1*z2`, `W=w1*w2`; both are nonzero. Exactly:

```
Qci = Z² q̄_i,  Rci = Z⁴ r̄_i,
Sci = Z³ W s̄_i,  Tci = Z⁵ W t̄_i.
```

These identities follow by field cancellation from the source expressions. Therefore the same-support conditions imply `q̄_1=q̄_2` and `r̄_1=r̄_2`; the zero-slope condition implies `s̄_1=s̄_2`. Denote the common monic quadratic by u.

## Derive the coefficient hypotheses, rather than assume them

By subtracting the two valid-input divisibilities, `u ∣ v₂²-v₁²`. Expand:

```
v₂²-v₁² = (s̄₂²-s̄₁²)X²
           + 2(s̄₂t̄₂-s̄₁t̄₁)X
           + (t̄₂²-t̄₁²).
```

Using `s̄₁=s̄₂`, this polynomial becomes

```
D = ((t̄₂-t̄₁)(s̄₁+s̄₂)) X
    + (t̄₂-t̄₁)(t̄₂+t̄₁).
```

It has degree at most one. Since u is monic of degree two and divides D, D is zero. (If using `natDegree`, split D=0 first; for a nonzero D the usual divisor degree inequality contradicts the bound.) Taking coefficients of X and 1 yields:

```
(t̄₂-t̄₁)(s̄₁+s̄₂)=0,
(t̄₂-t̄₁)(t̄₂+t̄₁)=0.
```

Scale these by `Z⁸W²` and `Z¹⁰W²`, respectively. The source identities above give exactly:

```
hx : (Tc2-Tc1)(Sc1+Sc2)=0,
hc : (Tc2-Tc1)(Tc2+Tc1)=0.
```

These are the hypotheses of `ProjectiveLawRuntime.classifySameU22_ne_zeroSlope Sc1 Tc1 Sc2 Tc2 hodd hx hc`. Its conclusion excludes `.sameUMixedZeroSlope` for the existing abstract classifier, including its inverse/equality precedence.

## Proposed Lean statements

The following are **statement drafts**, not declarations claimed to compile. The first is intentionally independent of runtime scaling:

```lean
-- Proposed statement; proof still to be implemented and kernel checked.
theorem same_support_zero_slope_validity_coefficients
    {K : Type*} [Field K]
    (f : Polynomial K) (q r s1 t1 s2 t2 : K)
    (valid1 : (Polynomial.X ^ 2 + Polynomial.C q * Polynomial.X + Polynomial.C r) ∣
      f - (Polynomial.C s1 * Polynomial.X + Polynomial.C t1) ^ 2)
    (valid2 : (Polynomial.X ^ 2 + Polynomial.C q * Polynomial.X + Polynomial.C r) ∣
      f - (Polynomial.C s2 * Polynomial.X + Polynomial.C t2) ^ 2)
    (hs : s1 = s2) :
    (t2 - t1) * (s1 + s2) = 0 ∧ (t2 - t1) * (t2 + t1) = 0
```

Proof plan: subtract the divisibilities; prove the displayed polynomial expansion by `ring` after `hs`; prove the common quadratic has degree two; conclude the degree-at-most-one multiple is zero; apply `congrArg (fun p => p.coeff 1)` and coefficient zero. Precise Mathlib lemma names and imports are not checked here.

The second statement should introduce an explicit `WeightedValid` predicate with the normalization formulas above, then accept nonzero input weights, two `WeightedValid f` hypotheses, and the source same-support/zero-slope conditions. It should return the conjunction hx/hc for the literal six cross-scaled expressions. Proof plan: establish the four scaling identities, cancel nonzero factors to obtain shared normalized q/r/s, invoke the first theorem, and multiply each conclusion by the indicated powers. This separates algebra from the source mapping and makes the imported hypotheses reviewable.

A final short corollary adds `[DecidableEq K]` and `hodd : (2:K)≠0` and invokes the existing classifier theorem. It must not assert compiled Rust totality unless the field-operation/equality interpretation and exact branch-order correspondence are separately justified.

## Why the zero-slope condition matters

Without `s̄₁=s̄₂`, divisibility of the quadratic difference by u does **not** force its degree-one and constant coefficients to vanish: the degree-two coefficient may supply a nonzero multiple of u. Thus dropping the zero-slope hypothesis or treating hx/hc as consequences of arbitrary equal-support inputs would strengthen the statement incorrectly. Likewise zero weights make the runtime normalization interpretation invalid, and validity on different f polynomials gives no common difference divisibility.

No counterexample to the stated, fully qualified implication was found or searched for here. The written field-polynomial argument supplies a concrete candidate proof. It still needs formal checking and independent review.

## What this connection would and would not close

It would discharge the previously supplied validity-coefficient hypotheses for one abstract error-exclusion theorem, under explicit runtime-shaped same-support and zero-slope assumptions. It would not prove all runtime branches preserve validity, that the collector always supplies such valid inputs, correctness of Rust field arithmetic, exact source-to-Lean semantics, correctness of the returned divisor class, or any global principal-divisor/Picard assertion. No cohort or operation-cost result changes.
