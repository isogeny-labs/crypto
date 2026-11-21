# Macaulay certificate instance gate and formalization map

The native checker is producer-local and unverified under PCC. No theorem
about its parser/program or the full norm solver is asserted. This map begins
with an actual small source certificate; the norm cases remain unknown.
[pcc:unverified]

1. Structure above: Mathlib `Polynomial (ZMod 5)`, its commutative ring and
   characteristic5 instance, and `Ideal`/`Ideal.span`. General native inputs
   are multivariate polynomials, not a newly defined arithmetic syntax.
2. Existing laws used as stated: `Ideal.eq_top_iff_one`, `Ideal.subset_span`,
   `Ideal.add_mem`, `Ideal.mul_mem_left`, `CharP.cast_eq_zero`. The exact unit
   identity is an instance computation, not a new generic algebra law.
3. Object: actual unit.ms input x,x-1; the first native certificate row is
   1*x+4*(x-1)=1 over F5. Source rows have multiplier1. A1 also retains the
   original generators as extra candidate rows; the unit row is unchanged.
4. Assembly probe: abstract ring/ideal closure with that numeric identity
   supplied as the component contract, closed by the existing Ideal laws.
   Concrete module then discharges the identity in the declared polynomial
   ring. It does not assume ideal membership or the unit-ideal conclusion.
5. Missing components: full file/parser-to-semantic-polynomial bridge;
   soundness of every native row identity/normal-form trace; actual grevlex
   order binding and Buchberger endpoint on arbitrary native inputs. Native
   exact checking is not represented as a checked general Lean program.
6. Inheritance: ring/ideal laws from Mathlib; only this actual unit-certificate
   instance and its coefficient equality live in the study. No shared law is
   re-derived or copied, no custom syntax, native evaluator or raised limit.

Pins: CryptoLean52bc3fbe8b5f8c92a9f8958319e0762e4414a8ba,
Mathlibcfc55fc58714a59dc06e6e79edc6d1d95140d3c2,
Leanleanprover/lean4:v4.35.0. The existing Mathlib Groebner module was found;
its exact order/criterion interface is a further dependency to inspect rather
than replace with a study-local claim. No Lean edit preceded this map.

## Completed exact unit instance

`MacaulayCertificateInstances.unit_certificate_identity` and
`MacaulayCertificateInstances.unit_ideal_top` compile with the declared pins.
The project proof gate checks2 theorems, zero failing/trusted-native proofs;
explicit axiom printing shows only propext,Classical.choice,Quot.sound.
Source/build/axiom/gate receipts are in checks/. No resource limit was raised.
The first attempts needed the existing polynomial CharP instance import and
an explicit Nat.cast_ofNat numeral conversion; both failure receipts remain.
[pcc:unverified]

This proves the declared F5 polynomial identity and unit ideal. It does not
prove that a native text parser reads those polynomials, verify the whole
C++ program, or establish any norm-target certificate. Those bridges remain
explicitly unformalized. The concrete first native unit row has coefficients
1,4 and source multipliers1, matching the semantic statement by inspection.
[pcc:unverified]

The checked Mathlib Groebner.lean module supplies division with respect to
monomial orders, not a Buchberger completion theorem found in these paths.
An actual native grevlex/criterion soundness bridge is still missing; no
module filename or division law is reported as that completed formalization.
