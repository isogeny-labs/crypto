module
public import BooleanContraction
public section
namespace PackedContraction
open CauchyNormalizer GlobalPolynomials BooleanContraction Polynomial

@[expose] def slotIndex (a : ℕ) (i j : Fin (2^a)) : Fin (2^(a+1)) :=
  ⟨j.val+2^a-i.val, by have hi:=i.isLt; have hj:=j.isLt; rw [pow_succ]; omega⟩

@[expose] noncomputable def packedBasePolynomial {κ : Type*} [DecidableEq κ] {M : ℕ}
    (a : ℕ) (place : Fin M → κ × Fin (2^a)) (i j : Fin M) : Polynomial ℚ :=
  if (place i).1=(place j).1 then basePolynomial a (slotIndex a (place i).2 (place j).2) else 0

@[expose] def packedBaseValue {κ : Type*} [DecidableEq κ] {M : ℕ}
    (a : ℕ) (place : Fin M → κ × Fin (2^a)) (T : ℚ) (i j : Fin M) : ℚ :=
  if (place i).1=(place j).1 then baseKernel a (slotIndex a (place i).2 (place j).2) T else 0

@[expose] def packedKernelValue {κ : Type*} [DecidableEq κ] {M : ℕ}
    (a : ℕ) (place : Fin M → κ × Fin (2^a)) (T : ℚ) (i j : Fin M) : ℚ :=
  if (place i).1=(place j).1 then RationalScalar.kernel a (slotIndex a (place i).2 (place j).2) T else 0

@[expose] noncomputable def denominatorPolynomial (a : ℕ) : Polynomial ℚ :=
  ∑ s : Fin (2^(a+1)), basePolynomial a s

@[expose] noncomputable def packedNumeratorPolynomial {κ : Type*} [DecidableEq κ] {M : ℕ}
    (a : ℕ) (place : Fin M → κ × Fin (2^a)) (i j : Fin M) : Polynomial ℚ :=
  (packedBasePolynomial a place i j)^2

theorem packedBaseEval {κ : Type*} [DecidableEq κ] {M : ℕ}
    (a : ℕ) (place : Fin M → κ × Fin (2^a)) (T : ℚ) (i j : Fin M) :
    (packedBasePolynomial a place i j).eval T = packedBaseValue a place T i j := by
  unfold packedBasePolynomial packedBaseValue
  split_ifs <;> simp [eval_basePolynomial]

theorem packedKernelFormula {κ : Type*} [DecidableEq κ] {M : ℕ}
    (a : ℕ) (place : Fin M → κ × Fin (2^a)) (T : ℚ) (i j : Fin M) :
    packedKernelValue a place T i j = (packedBaseValue a place T i j)^2 / RationalScalar.denominator a T := by
  unfold packedKernelValue packedBaseValue
  split_ifs <;> simp [RationalScalar.rationalKernelFormula,pow_two]

theorem packedNumeratorEval {κ : Type*} [DecidableEq κ] {M : ℕ}
    (a : ℕ) (place : Fin M → κ × Fin (2^a)) (T : ℚ) (i j : Fin M) :
    (packedNumeratorPolynomial a place i j).eval T = (packedBaseValue a place T i j)^2 := by
  simp only [packedNumeratorPolynomial,eval_pow,packedBaseEval]

theorem denominatorPolynomialEval (a : ℕ) (T : ℚ) :
    (denominatorPolynomial a).eval T = RationalScalar.denominator a T := by
  unfold denominatorPolynomial RationalScalar.denominator
  rw [eval_finsetSum]
  simp_rw [eval_basePolynomial]
  exact Fin.sum_univ_eq_sum_range (fun s => baseKernel a s T) _

theorem denominatorPolynomialDegree (a : ℕ) :
    (denominatorPolynomial a).natDegree ≤ 2^(a+1)-1 := by
  unfold denominatorPolynomial
  exact natDegree_sum_le_of_forall_le _ _ (fun s hs => basePolynomial_degree a s)

theorem packedBaseDegree {κ : Type*} [DecidableEq κ] {M : ℕ}
    (a : ℕ) (place : Fin M → κ × Fin (2^a)) (i j : Fin M) :
    (packedBasePolynomial a place i j).natDegree ≤ 2^(a+1)-1 := by
  unfold packedBasePolynomial
  split_ifs
  · exact basePolynomial_degree _ _
  · simp

theorem packedNumeratorDegree {κ : Type*} [DecidableEq κ] {M : ℕ}
    (a : ℕ) (place : Fin M → κ × Fin (2^a)) (i j : Fin M) :
    (packedNumeratorPolynomial a place i j).natDegree ≤ 2*(2^(a+1)-1) := by
  exact natDegree_pow_le_of_le 2 (packedBaseDegree a place i j)

/-- Source D.8 with the ACTUAL packed Cauchy rational matrix. The affine
zero-output factors contribute D rather than being treated as homogeneous. -/
theorem packedPointClearing {κ : Type*} [DecidableEq κ] {M : ℕ}
    (a : ℕ) (place : Fin M → κ × Fin (2^a)) (A B : (Fin M → Bool) → ℚ)
    (T : ℚ) (hD : RationalScalar.denominator a T ≠ 0) :
    (RationalScalar.denominator a T)^M * sourceContraction A B (packedKernelValue a place T) =
      (contractionPolynomial A B (denominatorPolynomial a) (packedNumeratorPolynomial a place)).eval T := by
  rw [contractionPolynomialEval,denominatorPolynomialEval]
  have hN : (fun i j => (packedNumeratorPolynomial a place i j).eval T) =
      (fun i j => (packedBaseValue a place T i j)^2) := by funext i j; exact packedNumeratorEval _ _ _ _ _
  rw [hN]
  have hW : packedKernelValue a place T =
      (fun i j => (packedBaseValue a place T i j)^2 * (RationalScalar.denominator a T)⁻¹) := by
    funext i j
    rw [packedKernelFormula,div_eq_mul_inv]
  rw [hW]
  simpa only [Units.val_mk0,Units.val_inv_eq_inv_val] using
    sourceContractionClearing A B (fun i j => (packedBaseValue a place T i j)^2)
      (Units.mk0 (RationalScalar.denominator a T) hD)

/-- The cleared polynomial over any Q-algebra coefficient ring, including
nilpotent coefficient algebras used by the source. A,B are immutable in T. -/
@[expose] noncomputable def packedContractionPolynomial {R κ : Type*} [CommRing R] [Algebra ℚ R]
    [DecidableEq κ] {M : ℕ} (a : ℕ) (place : Fin M → κ × Fin (2^a))
    (A B : (Fin M → Bool) → R) : Polynomial R :=
  contractionPolynomial A B ((denominatorPolynomial a).map (algebraMap ℚ R))
    (fun i j => (packedNumeratorPolynomial a place i j).map (algebraMap ℚ R))

theorem actualPackedContractionDegree {R κ : Type*} [CommRing R] [Algebra ℚ R]
    [DecidableEq κ] {M : ℕ} (a : ℕ) (place : Fin M → κ × Fin (2^a))
    (A B : (Fin M → Bool) → R) :
    (packedContractionPolynomial a place A B).natDegree ≤ M*(2*(2^(a+1)-1)) := by
  unfold packedContractionPolynomial
  apply contractionPolynomialDegree
  · calc
      _ ≤ (denominatorPolynomial a).natDegree := natDegree_map_le
      _ ≤ 2^(a+1)-1 := denominatorPolynomialDegree a
      _ ≤ 2*(2^(a+1)-1) := by omega
  · intro i j
    exact natDegree_map_le.trans (packedNumeratorDegree a place i j)

theorem packedKernelOriginPlaces {κ : Type*} [DecidableEq κ] {M : ℕ}
    (a : ℕ) (place : Fin M → κ × Fin (2^a)) (i j : Fin M) :
    packedKernelValue a place 0 i j = if place i=place j then 1 else 0 := by
  by_cases hp : (place i).1=(place j).1
  · have hs : (slotIndex a (place i).2 (place j).2).val=2^a ↔ (place i).2=(place j).2 := by
      rw [Fin.ext_iff]
      change (place j).2.val+2^a-(place i).2.val=2^a ↔ (place i).2.val=(place j).2.val
      have hi := (place i).2.isLt
      have hj := (place j).2.isLt
      omega
    simp only [packedKernelValue,hp,ite_true,RationalScalar.kernel_origin,hs,Prod.ext_iff,true_and]
  · have he : place i ≠ place j := by intro h; exact hp (congrArg Prod.fst h)
    simp [packedKernelValue,hp,he]

theorem actualPackedOriginContraction {κ : Type*} [DecidableEq κ] {M : ℕ}
    (a : ℕ) (place : Fin M → κ × Fin (2^a)) (hinj : Function.Injective place)
    (A B : (Fin M → Bool) → ℚ) :
    sourceContraction A B (packedKernelValue a place 0) = ∑ x, A x*B x := by
  have hW : packedKernelValue a place 0 = (fun i j => if i=j then (1:ℚ) else 0) := by
    funext i j
    rw [packedKernelOriginPlaces]
    simp only [hinj.eq_iff]
  rw [hW]
  exact sourceIdentityContraction A B

/-- The unit is constructed from the actual integral chart and its proved inverse. -/
@[expose] noncomputable def localDenominatorUnit (a : ℕ) (m : Fin (2^(a+1))) :
    (PowerSeries LocalIntegral.integralRing)ˣ where
  val := LocalCharts.denominatorSeries a m
  inv := LocalCharts.inverseSeries a m
  val_inv := LocalCharts.denominator_mul_inverse a m
  inv_val := LocalCharts.inverse_mul_denominator a m

@[expose] noncomputable def localPackedNumerator {κ : Type*} [DecidableEq κ] {M : ℕ}
    (a : ℕ) (m : Fin (2^(a+1))) (place : Fin M → κ × Fin (2^a))
    (i j : Fin M) : PowerSeries LocalIntegral.integralRing :=
  if (place i).1=(place j).1 then
    (LocalCharts.localBase a m (slotIndex a (place i).2 (place j).2)).toPowerSeries ^ 2 else 0

@[expose] noncomputable def localPackedKernel {κ : Type*} [DecidableEq κ] {M : ℕ}
    (a : ℕ) (m : Fin (2^(a+1))) (place : Fin M → κ × Fin (2^a))
    (i j : Fin M) : PowerSeries LocalIntegral.integralRing :=
  if (place i).1=(place j).1 then
    LocalCharts.localKernelSeries a m (slotIndex a (place i).2 (place j).2) else 0

theorem localPackedNormalization {κ : Type*} [DecidableEq κ] {M : ℕ}
    (a : ℕ) (m : Fin (2^(a+1))) (place : Fin M → κ × Fin (2^a)) :
    localPackedKernel a m place =
      fun i j => localPackedNumerator a m place i j *
        (↑(localDenominatorUnit a m)⁻¹ : PowerSeries LocalIntegral.integralRing) := by
  funext i j
  unfold localPackedKernel localPackedNumerator
  split_ifs
  · rfl
  · rw [zero_mul]

/-- Actual local chart, without assumed integrality or an assumed reciprocal. -/
theorem actualLocalContractionClearing {κ : Type*} [DecidableEq κ] {M : ℕ}
    (a : ℕ) (m : Fin (2^(a+1))) (place : Fin M → κ × Fin (2^a))
    (A B : (Fin M → Bool) → PowerSeries LocalIntegral.integralRing) :
    (LocalCharts.denominatorSeries a m)^M *
        sourceContraction A B (localPackedKernel a m place) =
      clearedContraction A B (LocalCharts.denominatorSeries a m)
        (localPackedNumerator a m place) := by
  rw [localPackedNormalization]
  exact sourceContractionClearing A B _ (localDenominatorUnit a m)

/-- In particular f may be reduction modulo the actual two-generator jet ideal.
No domain assumption is made on its target. -/
theorem mappedLocalContractionClearing {R κ : Type*} [CommRing R] [DecidableEq κ] {M : ℕ}
    (f : PowerSeries LocalIntegral.integralRing →+* R)
    (a : ℕ) (m : Fin (2^(a+1))) (place : Fin M → κ × Fin (2^a))
    (A B : (Fin M → Bool) → PowerSeries LocalIntegral.integralRing) :
    (f (LocalCharts.denominatorSeries a m))^M *
        sourceContraction (fun y => f (A y)) (fun x => f (B x))
          (fun i j => f (localPackedKernel a m place i j)) =
      clearedContraction (fun y => f (A y)) (fun x => f (B x))
        (f (LocalCharts.denominatorSeries a m)) (fun i j => f (localPackedNumerator a m place i j)) := by
  simpa only [map_mul,map_pow,mapSourceContraction,mapClearedContraction] using
    congrArg f (actualLocalContractionClearing a m place A B)

end PackedContraction
#print axioms PackedContraction.packedPointClearing
#print axioms PackedContraction.actualPackedContractionDegree
#print axioms PackedContraction.actualPackedOriginContraction
#print axioms PackedContraction.actualLocalContractionClearing
#print axioms PackedContraction.mappedLocalContractionClearing
