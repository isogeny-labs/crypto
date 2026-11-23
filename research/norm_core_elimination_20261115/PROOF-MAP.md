# Core elimination formalization map / instance gate

The written projection argument is in derivation.md. The original general
subexponential criterion remains open. This map distinguishes generic supporting
laws from a machine-checked equivalence of the entire code/encoding.
PCC-20261115-be28ec:unverified. [pcc:unverified]

1. **Structure above:** Mathlib `CommRing R` for congruence/divisibility and
   powering; `Polynomial K` over a field for monic division, evaluation,
   separability and coprimality. No custom syntax/group structure.
2. **Theorems used as stated:** `sub_dvd_pow_sub_pow`, divisibility add/trans;
   `Polynomial.mul_divByMonic_eq_iff_isRoot`, evaluation homomorphism laws;
   `Polynomial.divByMonic_eq_zero_iff` and
   `Polynomial.modByMonic_eq_self_iff`; existing shared
   `CryptoLean.Polynomial.kummer_norm_charts`.
3. **Objects:** retained sequence V with V0=X and
   Vj^2=Qj U+V(j+1), terminal Vt=c; the norm A^2-f B^2 evaluated at an
   actual target (r,s) with s^2=f(r), A(r)=s B(r).
4. **Assembly probe:** first type-check the target-root/monic-division assembly
   and the pre-wrap quotient/remainder pair from the existing Mathlib laws.
   Then build the missing retained-chain congruence law generically in the
   shared owner. No theorem concludes its own assumed hypothesis.
5. **Missing components:** generic finite retained-chain divisibility law
   belongs in crypto-lean, `CryptoLean.Polynomial.RetainedRemainder`.
   Norm division and first pre-wrap endpoints then apply existing laws. Full
   coefficient graph isomorphism, quotient-degree chart and cost/yield bridge
   remain separate instance obligations if not faithfully assembled.
6. **Inheritance:** ring/polynomial algebra from Mathlib; shared law and norm
   charts from crypto-lean; only actual curve/degree/circuit discharge here.

No new definition is needed. The retained-chain proof opens no composite:
it composes the existing difference-of-powers divisibility law with the
supplied transition equation by induction. No raised resource limits, custom
axioms, `sorry` or native evaluation is permitted. Pins remain Leanv4.35.0,
Mathlibcfc55fc58714a59dc06e6e79edc6d1d95140d3c2, shared parentad93dea.
Status: assembly/build/axiom checks pending; do not infer proof from this map.

## Checked supporting endpoints

Source52bc3fbe8b5f8c92a9f8958319e0762e4414a8ba,
[shared PR7](https://github.com/aburan28/crypto-lean/pull/7):

- `CryptoLean.Polynomial.retained_power_congruence`: actual finite retained
  transitions imply powering congruence over any commutative ring.
- `CryptoLean.Polynomial.target_norm_division`: actual point/target equations
  imply exact reconstruction by monic linear division.
- `CryptoLean.Polynomial.retained_kummer_charts`: actual transitions, terminal,
  nonzero2/c and target separation imply membership and both norm charts.

The assembly probe, focused build, explicit axiom print, module audit and full
library gate pass. The latter checks101 theorems, zero failing and zero trusted
native proofs. New declarations use only propext,Classical.choice,Quot.sound.
Receipts are in checks/. This supersedes the initial pending build status, while
preserving it as the pre-edit map. [pcc:unverified]

Full coefficient graph equivalence, monic degree/first-quotient degree chart,
bounded witness lifting, program/backend correctness and the asymptotic
cost/yield theorem are still outside these declarations. No assumption is
silently promoted to a checked endpoint. [pcc:unverified]
