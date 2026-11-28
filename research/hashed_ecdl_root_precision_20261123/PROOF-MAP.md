# Root precision: design and instance gate

Status: proposed finite arithmetic refinement, before proof execution.
Existing conditional34 and finite-history81 statements stay byte-identical.

## Six fields

1. Structure: existing Std `Nat` order, addition, multiplication, truncated
   subtraction, floor division and powers. Existing source expression is
   `CryptoLeanCore.TaperedPrecision.childPrecision`, core3e47fb77; its
   `childPrecision_formula` is used as stated.
2. Missing law: a finite root envelope from `p<=M`, `32*u<=M`, `i<u`,
   `a=2*u`. Candidate conclusion is
   `childPrecision p M u (2*u) i*u <= 35*M*2^(2*u-i)`.
   Build it generically in crypto-lean before study controls. Bounded KB/GitHub
   and exact Std/Mathlib Nat source searches found no matching declaration;
   KB wrapper is unavailable127, so no novelty/absence clearance follows.
3. Instance: root shape from `Setup.parameters(_,0)`: exponent `a=2*u`,
   length `2^(2*u)`, array `64^(4*u)` and guard `length*array<=M`.
   Original Boolean edges E yield subdivided M=2E, initial p=E+1<=M for E>0.
   Successful checked Rust arithmetic and actual graph/path extraction remain
   separate. This file does not identify an unbounded integer with usize.
4. Assembly: type-check existing Nat laws and composition of three proposed
   component bounds before editing the shared law. Include a concrete numeric
   instance of the arithmetic contracts, explicitly not native graph admission.
5. Missing components: polynomial-versus-power inequality below; root margin
   from the array guard; root precision law. The study only applies these laws
   and checks the proposed counterexample family/translated scalar guards.
6. Inheritance: scalar formula via `childPrecision_formula`; ceiling via
   `Nat.div_mul_le_self`; powers via `Nat.pow_add`, `Nat.pow_mul`,
   `Nat.pow_le_pow_right`; order via existing Nat monotonicity/cancellation.
   The short polynomial inequality uses Nat induction and distributivity,
   not a new evaluator, arithmetic type, group or curve structure.

## Dependency and statement order

Shared crypto-lean `CryptoLeanCore.TaperedPrecisionRoot`:
polynomial evaluation envelope, `4*u<=2^(u+1)`, root guard margin, root35.
Only then add study-specific normalized counterexample and premise controls.
No existing source definition, theorem, dependency pin, proof gate or trust
list changes. Toolchain remains Lean4.35.0. Std-only import closure is planned.

## Written proof chain (to be checked)

Put D=2^(2u-i); i<u gives 2^(u+1)<=D.
For u>0, the proposed exact elementary inequalities are
`4u<=2^(u+1)` and `16u(2u+1)<=21*2^(u+1)`.
The second is checked for u=1,2,3; for u>=3 its polynomial grows by at most
two on succession, so Nat induction and `pow_succ` extend it.

Let h=(3M+u-1)/u. Existing floor law yields h*u<=3M+u.
Separate component replacements, without assuming the conclusion:

- p<=M and 4u<=D give `4*(p*u)<=M*D`.
- h*u<=3M+u and 32u<=M give
  `4*(8*h*D*u)<=96*M*D+32*u*D<=97*M*D`.
- the polynomial bound gives `4*(8*M*(2u+1)*u)<=42*M*D`.
- Expand the existing formula, distribute multiplication, add the three
  bounds: `4*(childPrecision*u)<=140*M*D`; cancel positive4.

Array margin: `2^(2u)*64^(4u)=2^(26u)`; for positiveu,
`26u>=u+5` and `u<=2^u`, hence `32u<=2^(26u)<=M`.

## Source-model counterexample prediction

With M=2E,p=E+1,u=2,a=4,i=1, the predicted exact identity is
`2*childPrecision=546E+2`, while `34*M*2^(a-i)=544E`.
Thus positiveE should exclude34 as a universal constant over this root
arithmetic domain. The conditional34 theorem retains its additional premises.

A translated unbounded Setup witness will use M=2^262150, E=M/2,
lambda=262150, rootu=2, floor(sqrt(lambda))=512 and delta=2^235935;
delta^10=M^9 exactly. Check every scalar guard, initial normalization,
valuation and both finite envelopes in native Rust BigUint. This constructs
no graph and represents no admitted production usize call or ECDLP solve.

## Controls, acceptance and limits

Accept only after assembly, shared theorem compilation, all reported axiom
queries and unchanged native proof gates pass. Retain every failed attempt.
Check the normalized34 counterexample and35 positive; deleting initial
precision, margin or active-valuation premises must have explicit failing
examples. Predicted wrong exponent/scale witness controls must reject.

One compiler worker under the existing outer lock,600s per phase and sampled
whole-cgroup stop15,032,385,536bytes. No raised elaboration limits, custom
axiom, sorry or native_decide in reported proofs. No new solver/timing panel,
S3 or external spending. The classical ECDLP goal remains active.
