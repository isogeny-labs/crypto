module
public import Mathlib.FieldTheory.Finite.Trace
public import Mathlib.FieldTheory.Finite.Extension
public import Mathlib.FieldTheory.Finite.Polynomial
public import Mathlib.FieldTheory.Galois.NormalBasis
public import Mathlib.RingTheory.LittleWedderburn
public import Mathlib.RingTheory.Norm.Transitivity
public import Mathlib.LinearAlgebra.Lagrange

public section

/-!
# Source-linked finite-field reference library

Source: Mullen–Panario (eds.), Handbook of Finite Fields, 2013.
The source IDs below identify book items, not new mathematical discoveries.
`rules.json` records which subclaims each declaration covers. All structures
are Mathlib objects; none of the book's assertions is installed as an axiom.
The corrected and refuted statements at the end expose errors explicitly.
-/

namespace FiniteFieldHandbook
open scoped BigOperators Polynomial
open Polynomial

/-- HFF2013:2.1.1: laws of a ring without a required identity. -/
theorem ring_laws (R : Type*) [NonUnitalRing R] (a b c : R) :
    a + b = b + a ∧ a * (b * c) = (a * b) * c ∧
    a * (b + c) = a * b + a * c ∧ (a + b) * c = a * c + b * c :=
  ⟨add_comm a b, (mul_assoc a b c).symm, mul_add a b c, add_mul a b c⟩

/-- HFF2013:2.1.2, part 3. -/
theorem domain_no_zero_divisors (R : Type*) [CommRing R] [IsDomain R] (a b : R) :
    a * b = 0 ↔ a = 0 ∨ b = 0 := mul_eq_zero

/-- HFF2013:2.1.5, Wedderburn's theorem. -/
theorem finite_division_ring_is_field (D : Type*) [DivisionRing D] [Finite D] : IsField D :=
  Field.toIsField D

/-- HFF2013:2.1.7; the positive characteristic and nontriviality are explicit. -/
theorem domain_characteristic_prime (R : Type*) [Ring R] [NoZeroDivisors R]
    [Nontrivial R] (p : ℕ) [CharP R p] (hp : p ≠ 0) : Nat.Prime p :=
  (CharP.char_is_prime_or_zero R p).resolve_right hp

/-- HFF2013:2.1.8. -/
theorem finite_field_characteristic_prime (K : Type*) [Field K] [Finite K] :
    Nat.Prime (ringChar K) := CharP.char_is_prime K (ringChar K)

/-- HFF2013:2.1.9, binary form with the prime characteristic exposed. -/
theorem frobenius_add (R : Type*) [CommRing R] (p n : ℕ) [Fact p.Prime]
    [CharP R p] (a b : R) : (a + b) ^ (p ^ n) = a ^ (p ^ n) + b ^ (p ^ n) :=
  add_pow_char_pow a b p n

/-- HFF2013:2.1.9, arbitrary finite sum (also allows n = 0). -/
theorem frobenius_sum (R : Type*) [CommRing R] (p n : ℕ) [Fact p.Prime]
    [CharP R p] {ι : Type*} (s : Finset ι) (a : ι → R) :
    (∑ i ∈ s, a i) ^ (p ^ n) = ∑ i ∈ s, (a i) ^ (p ^ n) :=
  sum_pow_char_pow p n s a

section FiniteField
variable (K : Type*) [Field K] [Fintype K]

/-- HFF2013:2.1.10. The vector-space structure is the Algebra/Module instance. -/
theorem extension_cardinality (L : Type*) [Field L] [Fintype L] [Algebra K L] :
    Fintype.card L = Fintype.card K ^ Module.finrank K L :=
  Module.card_eq_pow_finrank

/-- HFF2013:2.1.12, prime-power conclusion; dimension is covered separately. -/
theorem field_cardinality_prime_power (p : ℕ) [CharP K p] :
    ∃ n : ℕ+, Nat.Prime p ∧ Fintype.card K = p ^ (n : ℕ) := FiniteField.card K p

/-- HFF2013:2.1.16, nonzero elements. -/
theorem fermat (a : K) (ha : a ≠ 0) : a ^ (Fintype.card K - 1) = 1 :=
  FiniteField.pow_card_sub_one_eq_one a ha

/-- HFF2013:2.1.16, all elements. -/
theorem power_card (a : K) : a ^ Fintype.card K = a := FiniteField.pow_card a

/-- A reusable iteration of HFF2013:2.1.16. -/
theorem power_card_iterate (a : K) (n : ℕ) : a ^ (Fintype.card K ^ n) = a :=
  FiniteField.pow_card_pow n a

/-- HFF2013:2.1.17; includes the two-element field. -/
theorem inverse_power (a : K) (ha : a ≠ 0) : a ^ (Fintype.card K - 2) = a⁻¹ := by
  have hq : 2 ≤ Fintype.card K := Fintype.one_lt_card
  apply (mul_right_cancel₀ ha)
  rw [inv_mul_cancel₀ ha, ← pow_succ]
  rw [show Fintype.card K - 2 + 1 = Fintype.card K - 1 by omega]
  exact FiniteField.pow_card_sub_one_eq_one a ha

/-- HFF2013:2.1.18, the stated exceptional cardinality is excluded. -/
theorem sum_elements (hq : 2 < Fintype.card K) : ∑ a : K, a = 0 := by
  simpa using FiniteField.sum_pow_lt_card_sub_one K 1 (by omega)

/-- HFF2013:2.1.26. -/
theorem polynomial_frobenius (f : K[X]) :
    Polynomial.expand K (Fintype.card K) f = f ^ Fintype.card K :=
  FiniteField.Polynomial.expand_card f

/-- HFF2013:2.1.27, equality of the roots as a multiset. -/
theorem field_polynomial_roots :
    (X ^ Fintype.card K - X : K[X]).roots = Finset.univ.val :=
  FiniteField.roots_X_pow_card_sub_X K

/-- HFF2013:2.1.32, the finite-field splitting-polynomial assertion. -/
theorem field_polynomial_splits : (X ^ Fintype.card K - X : K[X]).Splits :=
  by simpa only [Fintype.card_eq_nat_card] using (Polynomial.splits_X_pow_nat_card_sub_X (K := K))

/-- HFF2013:2.1.27, the displayed polynomial factorization. -/
theorem field_polynomial_factorization :
    (X ^ Fintype.card K - X : K[X]) = ∏ a : K, (X - C a) := by
  classical
  have hm : (X ^ Fintype.card K - X : K[X]).Monic :=
    Polynomial.monic_X_pow_sub (by simpa using (Fintype.one_lt_card : 1 < Fintype.card K))
  rw [(field_polynomial_splits K).eq_prod_roots_of_monic hm, field_polynomial_roots K]
  rfl

/-- HFF2013:2.1.37. -/
theorem multiplicative_group_cyclic : IsCyclic Kˣ := inferInstance

/-- HFF2013:2.1.38: the unit group has the required order. -/
theorem multiplicative_group_cardinality [Fintype Kˣ] :
    Fintype.card Kˣ = Fintype.card K - 1 := by
  classical
  simpa only [Fintype.card_eq_nat_card] using Fintype.card_units K

/-- HFF2013:2.1.131, existence; the degree restriction is a separate obligation. -/
theorem polynomial_function_exists (f : K → K) : ∃ g : K[X], ∀ a : K, g.eval a = f a := by
  apply (Polynomial.exists_eval_eq_iff (fun a : K => a) f).mpr
  intro i j h
  exact congrArg f h

/-- HFF2013:2.1.131, existence together with the degree bound. -/
theorem polynomial_function_bounded (f : K → K) :
    ∃ g : K[X], g.degree < Fintype.card K ∧ ∀ a : K, g.eval a = f a := by
  classical
  refine ⟨Lagrange.interpolate Finset.univ (fun a : K => a) f, ?_, ?_⟩
  · exact Lagrange.degree_interpolate_lt _ (Function.injective_id.injOn)
  · intro a
    exact Lagrange.eval_interpolate_at_node _ Function.injective_id.injOn (Finset.mem_univ a)

/-- HFF2013:2.1.131, uniqueness with the book's strict degree bound. -/
theorem polynomial_function_unique (f g : K[X])
    (hf : f.degree < Fintype.card K) (hg : g.degree < Fintype.card K)
    (h : ∀ a : K, f.eval a = g.eval a) : f = g := by
  apply Polynomial.eq_of_degrees_lt_of_eval_finset_eq (s := Finset.univ) hf hg
  intro a _
  exact h a

/-- HFF2013:2.1.29, the full divisibility criterion for an irreducible polynomial. -/
theorem irreducible_divides_field_polynomial (f : K[X]) (hf : Irreducible f) (m : ℕ) :
    f.natDegree ∣ m ↔ f ∣ X ^ (Fintype.card K ^ m) - X := by
  simpa only [Fintype.card_eq_nat_card] using hf.natDegree_dvd_iff_dvd_X_pow_card_pow_sub_X (n := m)

/-- HFF2013:8.1.1 (definition): a function on a finite field permutes iff it is injective. -/
theorem permutation_iff_injective (f : K → K) :
    Function.Bijective f ↔ Function.Injective f :=
  ⟨fun h => h.1, fun h => ⟨h, (Finite.injective_iff_surjective).mp h⟩⟩

end FiniteField

section Extensions
variable (K L E : Type*) [Field K] [Field L] [Field E]
variable [Algebra K L]

/-- HFF2013:2.1.58: algebraicity is necessary for a nonzero minimal polynomial. -/
theorem minimal_polynomial_irreducible (a : L) (ha : IsIntegral K a) :
    Irreducible (minpoly K a) := minpoly.irreducible ha

/-- HFF2013:2.1.60. -/
theorem degree_tower [Algebra L E] [Algebra K E] [IsScalarTower K L E]
    [FiniteDimensional K L] [FiniteDimensional L E] :
    Module.finrank K E = Module.finrank K L * Module.finrank L E :=
  (Module.finrank_mul_finrank K L E).symm

/-- HFF2013:2.1.62. -/
theorem finite_extension_algebraic [FiniteDimensional K L] (a : L) :
    IsIntegral K a := Algebra.IsIntegral.isIntegral a

/-- HFF2013:2.1.80, the Frobenius-sum definition equals the standard algebraic trace. -/
theorem trace_formula [Finite L] (a : L) :
    algebraMap K L (Algebra.trace K L a) =
      ∑ i ∈ Finset.range (Module.finrank K L), a ^ (Nat.card K ^ i) :=
  FiniteField.algebraMap_trace_eq_sum_pow K L a

/-- HFF2013:2.1.83, part 2. -/
theorem trace_add (a b : L) :
    Algebra.trace K L (a + b) = Algebra.trace K L a + Algebra.trace K L b :=
  map_add (Algebra.trace K L) a b

/-- HFF2013:2.1.83, part 3. -/
theorem trace_smul (c : K) (a : L) :
    Algebra.trace K L (c • a) = c * Algebra.trace K L a := by
  simpa using (Algebra.trace K L).map_smul c a

/-- HFF2013:2.1.83, part 4, with standard finite-field instances inferred. -/
theorem trace_surjective [Finite L] : Function.Surjective (Algebra.trace K L) := by
  have : Finite K := Finite.of_injective _ (algebraMap K L).injective
  exact Algebra.trace_surjective K L

/-- HFF2013:2.1.83, part 5. -/
theorem trace_base [FiniteDimensional K L] (a : K) :
    Algebra.trace K L (algebraMap K L a) = (Module.finrank K L : K) * a := by
  simpa [Algebra.smul_def] using Algebra.trace_algebraMap (R := K) (S := L) a

/-- HFF2013:2.1.83, part 8. -/
theorem trace_tower [Algebra L E] [Algebra K E] [IsScalarTower K L E]
    [FiniteDimensional K L] [FiniteDimensional L E] (a : E) :
    Algebra.trace K L (Algebra.trace L E a) = Algebra.trace K E a :=
  Algebra.trace_trace a

/-- HFF2013:2.1.84, the separating property of the trace pairing. -/
theorem trace_pairing_nondegenerate [Finite L] {a : L} (ha : a ≠ 0) :
    ∃ b : L, Algebra.trace K L (a * b) ≠ 0 := by
  have : Finite K := Finite.of_injective _ (algebraMap K L).injective
  by_contra! h
  exact ha ((traceForm_nondegenerate K L).1 a h)

/-- HFF2013:2.1.86, product expression. -/
theorem norm_formula [Finite L] (a : L) :
    algebraMap K L (Algebra.norm K a) =
      ∏ i ∈ Finset.range (Module.finrank K L), a ^ (Nat.card K ^ i) :=
  FiniteField.algebraMap_norm_eq_prod_pow K L a

/-- HFF2013:2.1.86, quotient exponent expression. -/
theorem norm_power [Finite L] (a : L) :
    algebraMap K L (Algebra.norm K a) =
      a ^ ((Nat.card L - 1) / (Nat.card K - 1)) :=
  FiniteField.algebraMap_norm_eq_pow

/-- HFF2013:2.1.88, part 2. -/
theorem norm_mul (a b : L) :
    Algebra.norm K (a * b) = Algebra.norm K a * Algebra.norm K b :=
  map_mul (Algebra.norm K) a b

/-- HFF2013:2.1.88, part 3, including zero. -/
theorem norm_surjective [Finite L] : Function.Surjective (Algebra.norm K (S := L)) :=
  FiniteField.norm_surjective K L

/-- HFF2013:2.1.88, part 3, restricted to nonzero elements as units. -/
theorem norm_units_surjective [Finite L] :
    Function.Surjective (Units.map (Algebra.norm K (S := L))) :=
  FiniteField.unitsMap_norm_surjective K L

/-- HFF2013:2.1.88, part 4. -/
theorem norm_base [FiniteDimensional K L] (a : K) :
    Algebra.norm K (algebraMap K L a) = a ^ Module.finrank K L :=
  Algebra.norm_algebraMap a

/-- HFF2013:2.1.88, part 6. -/
theorem norm_tower [Algebra L E] [Algebra K E] [IsScalarTower K L E]
    [FiniteDimensional K L] [FiniteDimensional L E] (a : E) :
    Algebra.norm K (Algebra.norm L a) = Algebra.norm K a :=
  Algebra.norm_norm

/-- HFF2013:2.1.102 (normal-basis existence only; primitive normality is separate). -/
theorem normal_basis_exists [Finite L] :
    ∃ a : L, LinearIndependent K (fun σ : L ≃ₐ[K] L => σ a) := by
  have : Finite K := Finite.of_injective _ (algebraMap K L).injective
  exact exists_linearIndependent_algEquiv_apply K L

/-- HFF2013:2.1.98 and 2.1.102: the orbit is a spanning basis, not just independent. -/
theorem normal_basis_orbit [Finite L] :
    ∃ b : Module.Basis (L ≃ₐ[K] L) K L, ∀ σ, b σ = σ (b 1) := by
  have : Finite K := Finite.of_injective _ (algebraMap K L).injective
  exact ⟨IsGalois.normalBasis K L, fun σ => IsGalois.normalBasis_apply σ⟩

end Extensions

section Frobenius
variable (K L : Type*) [Field K] [Field L] [Fintype K] [Finite L] [Algebra K L]

/-- HFF2013:2.1.76, order of the generator of the automorphism group. -/
theorem frobenius_order :
    orderOf (FiniteField.frobeniusAlgEquivOfAlgebraic K L) = Module.finrank K L :=
  FiniteField.orderOf_frobeniusAlgEquivOfAlgebraic K L

/-- HFF2013:2.1.77 with the editors' corrected extension field. -/
theorem galois_group_cyclic : IsCyclic (L ≃ₐ[K] L) := inferInstance

/-- HFF2013:2.1.76, the Frobenius powers enumerate the distinct automorphisms. -/
theorem frobenius_enumerates : Function.Bijective
    (fun i : Fin (Module.finrank K L) =>
      FiniteField.frobeniusAlgEquivOfAlgebraic K L ^ (i : ℕ)) :=
  FiniteField.bijective_frobeniusAlgEquivOfAlgebraic_pow K L

/-- HFF2013:2.1.83, part 6. -/
theorem trace_frobenius (a : L) :
    Algebra.trace K L (a ^ Fintype.card K) = Algebra.trace K L a := by
  exact Algebra.trace_eq_of_algEquiv (FiniteField.frobeniusAlgEquivOfAlgebraic K L) a

/-- HFF2013:2.1.88, part 5. -/
theorem norm_frobenius (a : L) :
    Algebra.norm K (a ^ Fintype.card K) = Algebra.norm K a := by
  exact Algebra.norm_eq_of_algEquiv (FiniteField.frobeniusAlgEquivOfAlgebraic K L) a

/-- HFF2013:2.1.105, addition for an arbitrary finite linear combination of Frobenius maps. -/
theorem linearized_add (s : Finset ℕ) (c : ℕ → L) (a b : L) :
    (∑ i ∈ s, c i * (a + b) ^ (Fintype.card K ^ i)) =
    (∑ i ∈ s, c i * a ^ (Fintype.card K ^ i)) +
    (∑ i ∈ s, c i * b ^ (Fintype.card K ^ i)) := by
  have bridge (i : ℕ) (x : L) :
      (FiniteField.frobeniusAlgHom K L)^[i] x = x ^ (Fintype.card K ^ i) := by
    rw [FiniteField.coe_frobeniusAlgHom, pow_iterate]
  simp only [← bridge]
  simp only [← AlgHom.coe_pow, map_add, mul_add, Finset.sum_add_distrib]

/-- HFF2013:2.1.105, scalar linearity in a homomorphism representation. -/
theorem linearized_smul (s : Finset ℕ) (c : ℕ → L) (k : K) (a : L) :
    (∑ i ∈ s, c i * (k • a) ^ (Fintype.card K ^ i)) =
    k • (∑ i ∈ s, c i * a ^ (Fintype.card K ^ i)) := by
  have bridge (i : ℕ) (x : L) :
      (FiniteField.frobeniusAlgHom K L)^[i] x = x ^ (Fintype.card K ^ i) := by
    rw [FiniteField.coe_frobeniusAlgHom, pow_iterate]
  simp only [← bridge]
  simp only [← AlgHom.coe_pow, map_smul, Finset.smul_sum, mul_smul_comm]

end Frobenius

/-- HFF2013:2.1.32, existence of a field of each prescribed prime-power order. -/
theorem finite_field_exists (p n : ℕ) [Fact p.Prime] (hn : n ≠ 0) :
    Nat.card (GaloisField p n) = p ^ n := GaloisField.card p n hn

/-- HFF2013:2.1.12, the dimension over the prime field. -/
theorem cardinality_prime_field_dimension (K : Type*) [Field K] [Fintype K]
    (p : ℕ) [Fact p.Prime] [CharP K p] [Algebra (ZMod p) K] :
    p ^ Module.finrank (ZMod p) K = Fintype.card K :=
  FiniteField.pow_finrank_eq_card p K

/-- HFF2013:2.1.44. -/
theorem totient_multiplicative (m n : ℕ) (h : Nat.Coprime m n) :
    Nat.totient (m * n) = Nat.totient m * Nat.totient n := Nat.totient_mul h

/-- HFF2013:2.1.42, element-order calculation underlying the primitive power test. -/
theorem power_order (G : Type*) [Group G] [Finite G] (g : G) (t : ℕ) :
    orderOf (g ^ t) = orderOf g / Nat.gcd (orderOf g) t := orderOf_pow g

/-- HFF2013:2.1.118 with the editors' required irreducibility assumption. -/
theorem irreducible_splits_degree_extension (K : Type*) [Field K] [Finite K]
    (p n : ℕ) [Fact p.Prime] [CharP K p] [NeZero n]
    (f : K[X]) (hf : Irreducible f) (hn : f.natDegree = n) :
    (f.map (algebraMap K (FiniteField.Extension K p n))).Splits := by
  have hdvd : f ∣ X ^ (Nat.card K ^ n) - X :=
    hf.natDegree_dvd_iff_dvd_X_pow_card_pow_sub_X.mp (hn ▸ dvd_refl n)
  apply Splits.of_dvd (IsSplittingField.splits (FiniteField.Extension K p n)
    (X ^ (Nat.card K ^ n) - X : K[X]))
  · exact Polynomial.map_ne_zero (FiniteField.X_pow_card_pow_sub_X_ne_zero K
      (NeZero.ne n) Finite.one_lt_card)
  · exact Polynomial.map_dvd (algebraMap K (FiniteField.Extension K p n)) hdvd

/-- HFF2013:2.1.35, existence of an embedding with a given divisibility of degrees. -/
theorem extension_embedding_iff (F K L : Type*) [Field F] [Field K] [Field L]
    [Fintype F] [Fintype K] [Fintype L] [Algebra F K] [Algebra F L] :
    Nonempty (K →ₐ[F] L) ↔ Module.finrank F K ∣ Module.finrank F L :=
  FiniteField.nonempty_algHom_iff_finrank_dvd

/-- HFF2013:2.1.9 needs prime characteristic: characteristic four is a counterexample. -/
theorem composite_characteristic_frobenius_fails :
    ((1 + 1 : ZMod 4) ^ 4) ≠ (1 : ZMod 4) ^ 4 + (1 : ZMod 4) ^ 4 := by decide

end FiniteFieldHandbook
