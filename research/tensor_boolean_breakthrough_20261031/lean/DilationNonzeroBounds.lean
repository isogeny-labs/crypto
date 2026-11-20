import DilationSemaevBridge

/-!
# Arithmetic bounds for the dilation nonzeroness argument

This module isolates the finite combinatorics and integer bounds needed by
the Galois-projector route to nonzeroness.  It does not construct the
function-field point relation.
-/

namespace TensorBoundaryBounds

open scoped BigOperators

/-- The number of occurrences in `List.ofFn` is the cardinality of the
corresponding finite fibre. -/
theorem count_ofFn_eq_card_filter {K : Type*} [DecidableEq K] :
    ∀ {n : ℕ} (t : Fin n → K) (v : K),
    (List.ofFn t).count v =
      (Finset.univ.filter fun i => t i = v).card := by
  intro n
  induction n with
  | zero => intro t v; simp
  | succ n ih =>
      intro t v
      rw [List.ofFn_succ, Finset.card_filter, Fin.sum_univ_succ,
        List.count_cons, ih, Finset.card_filter]
      by_cases h : t 0 = v <;> simp [h, Nat.add_comm]

/-- Signed multiplicity of one value in a tuple. -/
def fiberCoefficient {K : Type*} [DecidableEq K]
    {n : ℕ} (t : Fin n → K) (sign : Fin n → ℤ) (v : K) : ℤ :=
  ∑ i with t i = v, sign i

/-- A nontrivial tuple has a value of odd multiplicity. -/
theorem exists_odd_fiber_of_not_triv {K : Type*} [DecidableEq K]
    {n : ℕ} (t : Fin n → K)
    (h : ¬ ECDLP.L40.TrivL (List.ofFn t)) :
    ∃ v, ¬ Even ((Finset.univ.filter fun i => t i = v).card) := by
  unfold ECDLP.L40.TrivL at h
  push Not at h
  obtain ⟨v, hv⟩ := h
  exact ⟨v, by simpa [count_ofFn_eq_card_filter] using hv⟩

/-- An odd fibre cannot cancel when every occurrence has sign `±1`. -/
theorem fiberCoefficient_ne_zero_of_odd
    {K : Type*} [DecidableEq K] {n : ℕ}
    (t : Fin n → K) (sign : Fin n → ℤ)
    (sign_unit : ∀ i, sign i = 1 ∨ sign i = -1) (v : K)
    (odd : ¬ Even ((Finset.univ.filter fun i => t i = v).card)) :
    fiberCoefficient t sign v ≠ 0 := by
  intro coefficientZero
  let fiber := Finset.univ.filter fun i => t i = v
  have eachMod : ∀ i ∈ fiber, sign i ≡ 1 [ZMOD (2 : ℤ)] := by
    intro i hi
    rcases sign_unit i with h | h
    · rw [h]
    · rw [h]
      norm_num [Int.ModEq]
  have sumMod := Int.ModEq.sum eachMod
  have hone : (∑ _i ∈ fiber, (1 : ℤ)) = (fiber.card : ℤ) := by simp
  have hmod : fiberCoefficient t sign v ≡
      (fiber.card : ℤ) [ZMOD (2 : ℤ)] := by
    unfold fiberCoefficient
    rw [← hone]
    exact sumMod
  rw [coefficientZero] at hmod
  have divides : (2 : ℤ) ∣ (fiber.card : ℤ) := by
    rw [Int.modEq_iff_dvd] at hmod
    simpa using hmod
  apply odd
  exact_mod_cast ((even_iff_two_dvd.mpr divides) : Even (fiber.card : ℤ))

/-- The signed multiplicity is bounded by the size of its fibre. -/
theorem fiberCoefficient_natAbs_le_card
    {K : Type*} [DecidableEq K] {n : ℕ}
    (t : Fin n → K) (sign : Fin n → ℤ)
    (sign_unit : ∀ i, sign i = 1 ∨ sign i = -1) (v : K) :
    (fiberCoefficient t sign v).natAbs ≤
      (Finset.univ.filter fun i => t i = v).card := by
  let fiber := Finset.univ.filter fun i => t i = v
  calc
    (fiberCoefficient t sign v).natAbs ≤
        ∑ i ∈ fiber, (sign i).natAbs := by
      exact Int.natAbs_sum_le fiber sign
    _ = ∑ _i ∈ fiber, 1 := by
      apply Finset.sum_congr rfl
      intro i hi
      rcases sign_unit i with h | h <;> simp [h]
    _ = fiber.card := by simp

/-- Every nontrivial signed tuple has a nonzero fibre coefficient of absolute
value at most the arity. -/
theorem exists_nonzero_fiberCoefficient_of_not_triv
    {K : Type*} [DecidableEq K] {n : ℕ}
    (t : Fin n → K) (sign : Fin n → ℤ)
    (sign_unit : ∀ i, sign i = 1 ∨ sign i = -1)
    (h : ¬ ECDLP.L40.TrivL (List.ofFn t)) :
    ∃ v, fiberCoefficient t sign v ≠ 0 ∧
      (fiberCoefficient t sign v).natAbs ≤ n := by
  obtain ⟨v, odd⟩ := exists_odd_fiber_of_not_triv t h
  refine ⟨v, fiberCoefficient_ne_zero_of_odd t sign sign_unit v odd, ?_⟩
  refine (fiberCoefficient_natAbs_le_card t sign sign_unit v).trans ?_
  simpa using
    (Finset.card_le_univ (Finset.univ.filter fun i => t i = v))

/-- A nonzero integer of absolute value below the modulus stays nonzero in
`ZMod modulus`. -/
theorem intCast_zmod_ne_zero_of_natAbs_lt
    (modulus : ℕ) (coefficient : ℤ) (coefficientNonzero : coefficient ≠ 0)
    (small : coefficient.natAbs < modulus) :
    (coefficient : ZMod modulus) ≠ 0 := by
  intro castZero
  have intDivides : (modulus : ℤ) ∣ coefficient :=
    (ZMod.intCast_zmod_eq_zero_iff_dvd coefficient modulus).mp castZero
  have natDivides : modulus ∣ coefficient.natAbs := by
    have h := Int.natAbs_dvd_natAbs.mpr intDivides
    simpa using h
  have positive : 0 < coefficient.natAbs :=
    Int.natAbs_pos.mpr coefficientNonzero
  have lower := Nat.le_of_dvd positive natDivides
  omega

/-- Nonvanishing in `ZMod p` transfers to every field of characteristic `p`.
This is the exact form needed for the nonzero-index premise of a division
polynomial over a rational-function field. -/
theorem intCast_field_ne_zero_of_zmod_ne_zero
    {K : Type*} [Field K] (modulus : ℕ) [CharP K modulus]
    (coefficient : ℤ) (nonzero : (coefficient : ZMod modulus) ≠ 0) :
    (coefficient : K) ≠ 0 := by
  intro castZero
  have mapCoefficient :
      (ZMod.castHom (dvd_refl modulus) K) (coefficient : ZMod modulus) =
        (coefficient : K) := by
    rw [ZMod.castHom_apply, ZMod.cast_intCast (dvd_refl modulus)]
  apply nonzero
  apply ZMod.castHom_injective K
  rw [mapCoefficient, map_zero, castZero]

/-- If the arity is below the characteristic modulus, some signed fibre
coefficient from every nontrivial tuple remains nonzero modulo that modulus. -/
theorem exists_nonzero_fiberCoefficient_cast_of_not_triv
    {K : Type*} [DecidableEq K] {n modulus : ℕ}
    (t : Fin n → K) (sign : Fin n → ℤ)
    (sign_unit : ∀ i, sign i = 1 ∨ sign i = -1)
    (h : ¬ ECDLP.L40.TrivL (List.ofFn t))
    (arityBelow : n < modulus) :
    ∃ v, (fiberCoefficient t sign v : ZMod modulus) ≠ 0 := by
  obtain ⟨v, coefficientNonzero, bound⟩ :=
    exists_nonzero_fiberCoefficient_of_not_triv t sign sign_unit h
  exact ⟨v, intCast_zmod_ne_zero_of_natAbs_lt modulus _ coefficientNonzero
    (bound.trans_lt arityBelow)⟩

/-- Regrouping a signed point sum by tuple value produces exactly the fibre
coefficients above. -/
theorem sum_fiberCoefficient_zsmul
    {K G : Type*} [Fintype K] [DecidableEq K] [AddCommGroup G]
    {n : ℕ} (t : Fin n → K) (sign : Fin n → ℤ) (point : K → G) :
    (∑ v, fiberCoefficient t sign v • point v) =
      ∑ i, sign i • point (t i) := by
  simp only [fiberCoefficient, Finset.sum_smul]
  simp_rw [Finset.sum_filter]
  rw [Finset.sum_comm]
  apply Fintype.sum_congr
  intro i
  simp

/-- Regrouping over the finite image of the tuple.  Unlike
`sum_fiberCoefficient_zsmul`, this version requires no finiteness assumption
on the ambient field, so it applies to rational-function fields and their
algebraic closures. -/
theorem sum_fiberCoefficient_zsmul_image
    {K G : Type*} [DecidableEq K] [AddCommGroup G]
    {n : ℕ} (t : Fin n → K) (sign : Fin n → ℤ) (point : K → G) :
    (∑ v : {v // v ∈ Finset.univ.image t},
        fiberCoefficient t sign v.1 • point v.1) =
      ∑ i, sign i • point (t i) := by
  classical
  calc
    (∑ v : {v // v ∈ Finset.univ.image t},
        fiberCoefficient t sign v.1 • point v.1) =
        ∑ v ∈ Finset.univ.image t,
          fiberCoefficient t sign v • point v := by
      exact (Finset.sum_subtype (Finset.univ.image t)
        (fun _ ↦ Iff.rfl)
        (fun v : K ↦ fiberCoefficient t sign v • point v)).symm
    _ = ∑ i, sign i • point (t i) := by
      simp only [fiberCoefficient, Finset.sum_smul]
      simp_rw [Finset.sum_filter]
      rw [Finset.sum_comm]
      apply Fintype.sum_congr
      intro i
      simp

/-- Finite-image regrouping when the point family itself is indexed only by
the values that occur in the tuple. -/
theorem sum_fiberCoefficient_zsmul_image_subtype
    {K G : Type*} [DecidableEq K] [AddCommGroup G]
    {n : ℕ} (t : Fin n → K) (sign : Fin n → ℤ)
    (point : {v // v ∈ Finset.univ.image t} → G) :
    (∑ v ∈ (Finset.univ.image t).attach,
        fiberCoefficient t sign v.1 • point v) =
      ∑ i, sign i • point ⟨t i, Finset.mem_image.mpr ⟨i, Finset.mem_univ i, rfl⟩⟩ := by
  classical
  simp only [fiberCoefficient, Finset.sum_smul]
  simp_rw [Finset.sum_filter]
  rw [Finset.sum_comm]
  apply Fintype.sum_congr
  intro i
  let q : {v // v ∈ Finset.univ.image t} :=
    ⟨t i, Finset.mem_image.mpr ⟨i, Finset.mem_univ i, rfl⟩⟩
  have qmem : q ∈ (Finset.univ.image t).attach := by simp [q]
  rw [Finset.sum_eq_single q]
  · simp [q]
  · intro b hb bne
    have hne : t i ≠ b.1 := by
      intro equal
      apply bne
      apply Subtype.ext
      exact equal.symm
    simp [hne]
  · exact fun absent ↦ (absent qmem).elim

/-- A nonzero fibre coefficient necessarily belongs to the tuple's finite
image. -/
theorem mem_image_of_fiberCoefficient_ne_zero
    {K : Type*} [DecidableEq K] {n : ℕ}
    (t : Fin n → K) (sign : Fin n → ℤ) (v : K)
    (nonzero : fiberCoefficient t sign v ≠ 0) :
    v ∈ Finset.univ.image t := by
  by_contra absent
  apply nonzero
  unfold fiberCoefficient
  apply Finset.sum_eq_zero
  intro i hi
  exact (absent (Finset.mem_image.mpr
    ⟨i, Finset.mem_univ i, (Finset.mem_filter.mp hi).2⟩)).elim

/-- Exact certificate extracted from a nontrivial signed relation: the
regrouped relation is zero and one coefficient survives with explicit size. -/
theorem nontriv_signed_relation_certificate
    {K G : Type*} [Fintype K] [DecidableEq K] [AddCommGroup G]
    {n : ℕ} (t : Fin n → K) (sign : Fin n → ℤ)
    (sign_unit : ∀ i, sign i = 1 ∨ sign i = -1)
    (point : K → G)
    (relation : ∑ i, sign i • point (t i) = 0)
    (nontriv : ¬ ECDLP.L40.TrivL (List.ofFn t)) :
    (∑ v, fiberCoefficient t sign v • point v = 0) ∧
      ∃ v, fiberCoefficient t sign v ≠ 0 ∧
        (fiberCoefficient t sign v).natAbs ≤ n := by
  constructor
  · rw [sum_fiberCoefficient_zsmul t sign point, relation]
  · exact exists_nonzero_fiberCoefficient_of_not_triv
      t sign sign_unit nontriv

/-- Finite-image form of the signed-relation certificate, suitable when the
ambient x-coordinate field is infinite. -/
theorem nontriv_signed_relation_image_certificate
    {K G : Type*} [DecidableEq K] [AddCommGroup G]
    {n : ℕ} (t : Fin n → K) (sign : Fin n → ℤ)
    (sign_unit : ∀ i, sign i = 1 ∨ sign i = -1)
    (point : K → G)
    (relation : ∑ i, sign i • point (t i) = 0)
    (nontriv : ¬ ECDLP.L40.TrivL (List.ofFn t)) :
    (∑ v : {v // v ∈ Finset.univ.image t},
        fiberCoefficient t sign v.1 • point v.1 = 0) ∧
      ∃ v, fiberCoefficient t sign v ≠ 0 ∧
        (fiberCoefficient t sign v).natAbs ≤ n := by
  constructor
  · rw [sum_fiberCoefficient_zsmul_image t sign point, relation]
  · exact exists_nonzero_fiberCoefficient_of_not_triv
      t sign sign_unit nontriv

/-- Subtype-indexed version of the finite-image certificate.  In addition to
the relation, the surviving coefficient is returned as an actual member of
the finite projector index type. -/
theorem nontriv_signed_relation_image_subtype_certificate
    {K G : Type*} [DecidableEq K] [AddCommGroup G]
    {n : ℕ} (t : Fin n → K) (sign : Fin n → ℤ)
    (sign_unit : ∀ i, sign i = 1 ∨ sign i = -1)
    (point : {v // v ∈ Finset.univ.image t} → G)
    (relation : ∑ i, sign i •
      point ⟨t i, Finset.mem_image.mpr ⟨i, Finset.mem_univ i, rfl⟩⟩ = 0)
    (nontriv : ¬ ECDLP.L40.TrivL (List.ofFn t)) :
    (∑ v ∈ (Finset.univ.image t).attach,
        fiberCoefficient t sign v.1 • point v = 0) ∧
      ∃ v : {v // v ∈ Finset.univ.image t},
        fiberCoefficient t sign v.1 ≠ 0 ∧
        (fiberCoefficient t sign v.1).natAbs ≤ n := by
  constructor
  · rw [sum_fiberCoefficient_zsmul_image_subtype t sign point, relation]
  · obtain ⟨v, nonzero, bound⟩ :=
      exists_nonzero_fiberCoefficient_of_not_triv t sign sign_unit nontriv
    exact ⟨⟨v, mem_image_of_fiberCoefficient_ne_zero t sign v nonzero⟩,
      nonzero, bound⟩

/-- After a Galois projector sums over `Gamma`, a surviving coefficient is
still nonzero modulo `p` under the explicit bound `|Gamma| * n < p`. -/
theorem projectedCoefficient_zmod_ne_zero
    {Gamma : Type*} [Fintype Gamma] [Nonempty Gamma]
    (modulus n : ℕ) (coefficient : ℤ)
    (coefficientNonzero : coefficient ≠ 0)
    (coefficientBound : coefficient.natAbs ≤ n)
    (projectedBelow : Fintype.card Gamma * n < modulus) :
    (((Fintype.card Gamma : ℤ) * coefficient : ℤ) : ZMod modulus) ≠ 0 := by
  apply intCast_zmod_ne_zero_of_natAbs_lt modulus
  · have hcard : (Fintype.card Gamma : ℤ) ≠ 0 := by
      exact_mod_cast (Fintype.card_ne_zero : Fintype.card Gamma ≠ 0)
    exact mul_ne_zero hcard coefficientNonzero
  · rw [Int.natAbs_mul, Int.natAbs_natCast]
    exact (Nat.mul_le_mul_left (Fintype.card Gamma) coefficientBound).trans_lt
      projectedBelow

/-- The projected coefficient is nonzero in any field of the same
characteristic, under the same explicit size bound. -/
theorem projectedCoefficient_field_ne_zero
    {K Gamma : Type*} [Field K] [Fintype Gamma] [Nonempty Gamma]
    (modulus n : ℕ) [CharP K modulus] (coefficient : ℤ)
    (coefficientNonzero : coefficient ≠ 0)
    (coefficientBound : coefficient.natAbs ≤ n)
    (projectedBelow : Fintype.card Gamma * n < modulus) :
    (((Fintype.card Gamma : ℤ) * coefficient : ℤ) : K) ≠ 0 := by
  apply intCast_field_ne_zero_of_zmod_ne_zero modulus
  exact projectedCoefficient_zmod_ne_zero modulus n coefficient
    coefficientNonzero coefficientBound projectedBelow

/-- A projected torsion certificate together with its nonzero-index proof in
the characteristic-`p` coefficient field. -/
theorem projectedTorsion_index_ne_zero
    {K Gamma Point : Type*} (modulus n : ℕ)
    [Field K] [CharP K modulus]
    [Fintype Gamma] [Nonempty Gamma] [AddCommGroup Point]
    (coefficient : ℤ) (point : Point)
    (coefficientNonzero : coefficient ≠ 0)
    (coefficientBound : coefficient.natAbs ≤ n)
    (projectedBelow : Fintype.card Gamma * n < modulus)
    (torsion : ((Fintype.card Gamma : ℤ) * coefficient) • point = 0) :
    (((Fintype.card Gamma : ℤ) * coefficient : ℤ) : K) ≠ 0 ∧
      ((Fintype.card Gamma : ℤ) * coefficient) • point = 0 := by
  exact ⟨projectedCoefficient_field_ne_zero modulus n coefficient
    coefficientNonzero coefficientBound projectedBelow, torsion⟩

/-- Direct projector input extracted from `¬TrivL`: one value has a Galois-
projected coefficient that is nonzero modulo the prime modulus. -/
theorem exists_projectedCoefficient_zmod_ne_zero_of_not_triv
    {K Gamma : Type*} [DecidableEq K] [Fintype Gamma] [Nonempty Gamma]
    {n modulus : ℕ} (t : Fin n → K) (sign : Fin n → ℤ)
    (sign_unit : ∀ i, sign i = 1 ∨ sign i = -1)
    (nontriv : ¬ ECDLP.L40.TrivL (List.ofFn t))
    (projectedBelow : Fintype.card Gamma * n < modulus) :
    ∃ v, ((((Fintype.card Gamma : ℤ) *
      fiberCoefficient t sign v : ℤ)) : ZMod modulus) ≠ 0 := by
  obtain ⟨v, coefficientNonzero, coefficientBound⟩ :=
    exists_nonzero_fiberCoefficient_of_not_triv t sign sign_unit nontriv
  exact ⟨v, projectedCoefficient_zmod_ne_zero modulus n
    (fiberCoefficient t sign v) coefficientNonzero coefficientBound
      projectedBelow⟩

/-- Eisenstein norm appearing in the `A = 0` cubic-CM branch. -/
def eisensteinNorm (a b : ℤ) : ℤ := a ^ 2 - a * b + b ^ 2

theorem eisensteinNorm_pos {a b : ℤ} (nonzero : a ≠ 0 ∨ b ≠ 0) :
    0 < eisensteinNorm a b := by
  unfold eisensteinNorm
  rcases nonzero with ha | hb
  · nlinarith [sq_pos_of_ne_zero ha, sq_nonneg (a - b), sq_nonneg b]
  · nlinarith [sq_nonneg a, sq_nonneg (a - b), sq_pos_of_ne_zero hb]

/-- If both projected coefficients have absolute value at most `M`, their
Eisenstein norm has absolute value at most `3 M²`. -/
theorem eisensteinNorm_natAbs_le {a b : ℤ} {bound : ℕ}
    (ha : a.natAbs ≤ bound) (hb : b.natAbs ≤ bound) :
    (eisensteinNorm a b).natAbs ≤ 3 * bound ^ 2 := by
  unfold eisensteinNorm
  calc
    (a ^ 2 - a * b + b ^ 2).natAbs ≤
        (a ^ 2 - a * b).natAbs + (b ^ 2).natAbs :=
      Int.natAbs_add_le _ _
    _ ≤ ((a ^ 2).natAbs + (a * b).natAbs) + (b ^ 2).natAbs := by
      gcongr
      exact Int.natAbs_sub_le _ _
    _ = a.natAbs ^ 2 + a.natAbs * b.natAbs + b.natAbs ^ 2 := by
      rw [Int.natAbs_pow, Int.natAbs_mul, Int.natAbs_pow]
    _ ≤ bound ^ 2 + bound * bound + bound ^ 2 := by gcongr
    _ = 3 * bound ^ 2 := by ring

/-- Explicit characteristic bound for the CM norm-torsion scalar. -/
theorem eisensteinNorm_zmod_ne_zero
    (modulus bound : ℕ) (a b : ℤ)
    (nonzero : a ≠ 0 ∨ b ≠ 0)
    (ha : a.natAbs ≤ bound) (hb : b.natAbs ≤ bound)
    (normBelow : 3 * bound ^ 2 < modulus) :
    (eisensteinNorm a b : ZMod modulus) ≠ 0 := by
  apply intCast_zmod_ne_zero_of_natAbs_lt modulus
  · exact ne_of_gt (eisensteinNorm_pos nonzero)
  · exact (eisensteinNorm_natAbs_le ha hb).trans_lt normBelow

/-- The exact scalar produced by the two-term cubic-CM projector is nonzero
modulo `p` under `3 * (|Gamma| * n)^2 < p`, provided the selected fibre
coefficient survives. -/
theorem projectedEisensteinNorm_zmod_ne_zero
    {Gamma : Type*} [Fintype Gamma] [Nonempty Gamma]
    (modulus n : ℕ) (chosen partner sign : ℤ)
    (chosenNonzero : chosen ≠ 0)
    (chosenBound : chosen.natAbs ≤ n)
    (partnerBound : partner.natAbs ≤ n)
    (signUnit : sign = 1 ∨ sign = -1)
    (normBelow : 3 * (Fintype.card Gamma * n) ^ 2 < modulus) :
    (eisensteinNorm
      ((Fintype.card Gamma : ℤ) * chosen)
      (((Fintype.card Gamma : ℤ) * partner) * sign) :
        ZMod modulus) ≠ 0 := by
  have hcard : (Fintype.card Gamma : ℤ) ≠ 0 := by
    exact_mod_cast (Fintype.card_ne_zero : Fintype.card Gamma ≠ 0)
  apply eisensteinNorm_zmod_ne_zero modulus (Fintype.card Gamma * n)
  · exact Or.inl (mul_ne_zero hcard chosenNonzero)
  · rw [Int.natAbs_mul, Int.natAbs_natCast]
    exact Nat.mul_le_mul_left (Fintype.card Gamma) chosenBound
  · rw [Int.natAbs_mul, Int.natAbs_mul, Int.natAbs_natCast]
    have hsign : sign.natAbs = 1 := by
      rcases signUnit with rfl | rfl <;> simp
    rw [hsign, Nat.mul_one]
    exact Nat.mul_le_mul_left (Fintype.card Gamma) partnerBound
  · exact normBelow

/-- The projected cubic-CM Eisenstein scalar stays nonzero in every field of
characteristic `p` under the same explicit norm bound. -/
theorem projectedEisensteinNorm_field_ne_zero
    {K Gamma : Type*} (modulus n : ℕ)
    [Field K] [CharP K modulus] [Fintype Gamma] [Nonempty Gamma]
    (chosen partner sign : ℤ)
    (chosenNonzero : chosen ≠ 0)
    (chosenBound : chosen.natAbs ≤ n)
    (partnerBound : partner.natAbs ≤ n)
    (signUnit : sign = 1 ∨ sign = -1)
    (normBelow : 3 * (Fintype.card Gamma * n) ^ 2 < modulus) :
    (eisensteinNorm
      ((Fintype.card Gamma : ℤ) * chosen)
      (((Fintype.card Gamma : ℤ) * partner) * sign) : K) ≠ 0 := by
  apply intCast_field_ne_zero_of_zmod_ne_zero modulus
  exact projectedEisensteinNorm_zmod_ne_zero modulus n chosen partner sign
    chosenNonzero chosenBound partnerBound signUnit normBelow

/-- A cubic-CM norm-torsion certificate together with nonvanishing of its
index in the characteristic-`p` coefficient field. -/
theorem projectedEisensteinNorm_torsion_index_ne_zero
    {K Gamma Point : Type*} (modulus n : ℕ)
    [Field K] [CharP K modulus] [Fintype Gamma] [Nonempty Gamma]
    [AddCommGroup Point]
    (chosen partner sign : ℤ) (point : Point)
    (chosenNonzero : chosen ≠ 0)
    (chosenBound : chosen.natAbs ≤ n)
    (partnerBound : partner.natAbs ≤ n)
    (signUnit : sign = 1 ∨ sign = -1)
    (normBelow : 3 * (Fintype.card Gamma * n) ^ 2 < modulus)
    (torsion : eisensteinNorm
      ((Fintype.card Gamma : ℤ) * chosen)
      (((Fintype.card Gamma : ℤ) * partner) * sign) • point = 0) :
    (eisensteinNorm
      ((Fintype.card Gamma : ℤ) * chosen)
      (((Fintype.card Gamma : ℤ) * partner) * sign) : K) ≠ 0 ∧
    eisensteinNorm
      ((Fintype.card Gamma : ℤ) * chosen)
      (((Fintype.card Gamma : ℤ) * partner) * sign) • point = 0 := by
  exact ⟨projectedEisensteinNorm_field_ne_zero modulus n
    chosen partner sign chosenNonzero chosenBound partnerBound signUnit
    normBelow, torsion⟩

end TensorBoundaryBounds
