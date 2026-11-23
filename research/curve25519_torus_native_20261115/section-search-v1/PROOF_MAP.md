# Proof map and instance gate, written before Lean edits

All new mathematics is producer unsigned/unverified. [pcc:unverified]

| Report step | Mathematical owner / structure | Existing law and status |
|---|---|---|
| Norm sextic and target divisor | Elliptic function field / Pic^0 | Classical Cheng section2 and frozen section derivation; readable argument only. Shared formal divisor/native binding missing. |
| Rational/curve/membership support tests | Fp[X], degree6, primary multiplicities | Root multiplicity and finite-field laws; no new full Lean proof. |
| Geometric trace residual | Commutative ring polynomial identity | Native evaluated identity only; generic formal law belongs upstream in crypto-lean. |
| Multiplication by a unit preserves sixth-power zero | Mathlib CommRing/IsUnit | IsUnit.mul_right_eq_zero, IsUnit.pow, mul_pow applied as stated in standalone assembly endpoint. |
| Fast interval algorithm over a finite free algebra | Polynomial arithmetic and scalar-unit chirp operations | Bostan--Yurkevich Algorithm2; explicit operation transfer in DERIVATION. Native backend/semantic/cost proof missing. |
| Restriction and extraction | Polynomial gcd, local jets and original raw decoder | Written contracts; no implemented full extractor. |
| Uniform coefficient counting | Finite projective chart and divisor-to-section injection | Written proof. Formal actual divisor map/global-functions theorem missing; no surrogate conditional cardinality theorem claimed as the result. |

Instance-gate six fields:

1. Structure above: Mathlib CommRing, its unit group and multiplication/powers/zero.
2. Theorem used unchanged: IsUnit.mul_right_eq_zero; power closure IsUnit.pow.
3. Object: the element 4v in the finite free quotient algebra. Its unit and
   geometric-product equality are explicit missing bridge obligations, not
   a native or field instance asserted by a producer boolean.
4. Assembly probe: for any commutative ring R, a with IsUnit a, and h,
   (a*h)^6=0 iff h^6=0. Rewrite by mul_pow; apply cancellation for a^6.
5. Missing components: actual monic quotient/free-basis construction; inverse
   of v from its quadratic relation; field characteristic/4 unit; product
   equality and algorithm transfer; divisor/counting and original-row bindings.
6. Inheritance: ring operations/powers from CommRing; unit powers from IsUnit.pow;
   zero equivalence from IsUnit.mul_right_eq_zero; no custom ring, axiom or
   limit override. Generic new laws are upstream candidates, not copied here.

The endpoint verifies only existing-law assembly, with its unit hypothesis.
It is not the norm, product, counting, solver or attack-exponent theorem.
Toolchain and full dependency manifest are pinned separately. Run the maintained
standalone proof gate using the pinned LEAN_PATH, which compiles and audits the
actual file without a new Lake project's unrelated cache hydration. Do not
change the helper, trust manifest, frozen prior project or toolchain.
