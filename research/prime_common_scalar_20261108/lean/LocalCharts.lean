module
public import LocalIntegral
public section
namespace LocalCharts
open CauchyNormalizer LocalIntegral Polynomial

/-- Generic finite puncture identity; the nonzero factors are explicit. -/
theorem normalized_puncture {ι : Type*} [Fintype ι] [DecidableEq ι]
    (c : ι → ℚ) (m s : ι) (v d : ℚ) (hd : d ≠ 0)
    (hm : c m = 0) (hc : ∀ j, j ≠ m → c j ≠ 0) :
    (∏ j ∈ Finset.univ.erase s, (c j+v)) / d =
      ((∏ j ∈ Finset.univ.erase m, c j)/d) *
      (if s=m then 1 else v/c s) *
      ∏ j ∈ (Finset.univ.erase m).erase s, (1+v/c j) := by
  have hprod (A : Finset ι) (hA : ∀ j ∈ A, j ≠ m) :
      (∏ j ∈ A, (c j+v)) = (∏ j ∈ A, c j) * ∏ j ∈ A, (1+v/c j) := by
    rw [← Finset.prod_mul_distrib]
    apply Finset.prod_congr rfl
    intro j hj
    field_simp [hc j (hA j hj)]
  by_cases hs : s=m
  · subst s
    simp only [ite_true, mul_one, Finset.erase_idem]
    rw [hprod _ (by intro j hj; exact Finset.ne_of_mem_erase hj)]
    ring
  · simp only [hs, ite_false]
    have hms : m ∈ Finset.univ.erase s := Finset.mem_erase.mpr ⟨Ne.symm hs, Finset.mem_univ _⟩
    have hsm : s ∈ Finset.univ.erase m := Finset.mem_erase.mpr ⟨hs, Finset.mem_univ _⟩
    rw [← Finset.mul_prod_erase _ (fun j => c j+v) hms, hm, zero_add]
    have he : (Finset.univ.erase s).erase m = (Finset.univ.erase m).erase s := by ext j; simp; tauto
    rw [he]
    rw [hprod _ (by intro j hj; exact Finset.ne_of_mem_erase (Finset.mem_of_mem_erase hj))]
    rw [← Finset.mul_prod_erase _ c hsm]
    field_simp [hd, hc s hs]

@[expose] def centerDen (a : ℕ) : ℚ :=
  shiftedPuncture (2^(a+1)-1) (2^a) (2^a)

theorem centerDen_ne_zero (a : ℕ) : centerDen a ≠ 0 := by
  unfold centerDen
  rw [shiftedPuncture_eq, puncture_eq_signedWeight (a+1) (2^a) (center_lt a)]
  exact signedWeight_ne_zero _ _

theorem base_as_fin_product (a : ℕ) (s : Fin (2^(a+1))) (T : ℚ) :
    baseKernel a s T = (∏ j ∈ Finset.univ.erase s, (T-((j:ℚ)-(2^a:ℕ)))) / centerDen a := by
  rw [baseKernel_formula]
  unfold centerDen
  congr 1
  rw [← Fin.prod_univ_eq_prod_range (fun j => if j=s.val then (1:ℚ) else T-((j:ℚ)-(2^a:ℕ)))]
  have heq : (fun j : Fin (2^(a+1)) => if j.val=s.val then (1:ℚ) else T-((j:ℚ)-(2^a:ℕ))) =
      (fun j : Fin (2^(a+1)) => if j=s then (1:ℚ) else T-((j:ℚ)-(2^a:ℕ))) := by
    funext j; simp only [Fin.ext_iff]
  rw [heq, ← Finset.prod_erase_mul _ _ (Finset.mem_univ s)]
  simp only [ite_true, mul_one]
  apply Finset.prod_congr rfl
  intro j hj
  rw [ite_eq_right (Finset.ne_of_mem_erase hj)]

/-- Actual shifted base kernel, not a conditional polynomial surrogate. -/
theorem actual_chart_formula (a : ℕ) (m s : Fin (2^(a+1))) (z : ℚ) :
    baseKernel a s ((m:ℚ)-(2^a:ℕ)+((2^(a+1):ℕ):ℚ)*z) =
      cauchyRho a m * (if s=m then 1 else (((2^(a+1):ℕ):ℚ)/((m:ℚ)-(s:ℚ)))*z) *
      ∏ j ∈ (Finset.univ.erase m).erase s,
        (1+((((2^(a+1):ℕ):ℚ)/((m:ℚ)-(j:ℚ)))*z)) := by
  have hc : ∀ j : Fin (2^(a+1)), j ≠ m → (m:ℚ)-(j:ℚ) ≠ 0 := by
    intro j hj hzero
    have : m.val=j.val := by exact_mod_cast (sub_eq_zero.mp hzero)
    exact hj (Fin.ext this.symm)
  have hr := baseKernel_node a m m m.isLt
  simp only [ite_true] at hr
  rw [base_as_fin_product] at hr
  have hr' : (∏ j ∈ Finset.univ.erase m, ((m:ℚ)-(j:ℚ))) / centerDen a = cauchyRho a m := by
    convert hr using 2
    congr 1
    funext j; ring
  rw [base_as_fin_product]
  have h := normalized_puncture (fun j : Fin (2^(a+1)) => (m:ℚ)-(j:ℚ)) m s
    (((2^(a+1):ℕ):ℚ)*z) (centerDen a) (centerDen_ne_zero a) (by ring) hc
  convert h using 1
  · congr 1
    apply Finset.prod_congr rfl
    intro j hj; ring
  · rw [hr']
    congr 1
    · split_ifs <;> ring
    · apply Finset.prod_congr rfl
      intro j hj; ring

@[expose] noncomputable def localBase (a : ℕ) (m s : Fin (2^(a+1))) : Polynomial integralRing :=
  C (rhoUnit a m : integralRing) *
    (if s=m then 1 else C (chartRatio a m s) * X) *
    ∏ j ∈ (Finset.univ.erase m).erase s, (1+C (chartRatio a m j)*X)

theorem localBase_eval (a : ℕ) (m s : Fin (2^(a+1))) (z : ℚ) :
    (localBase a m s).eval₂ (integralRing.subtype) z =
      baseKernel a s ((m:ℚ)-(2^a:ℕ)+((2^(a+1):ℕ):ℚ)*z) := by
  rw [actual_chart_formula]
  unfold localBase
  split_ifs <;> simp [eval₂_finsetProd, chartRatio, rhoUnit]

theorem localBase_constant (a : ℕ) (m s : Fin (2^(a+1))) :
    (localBase a m s).coeff 0 = if s=m then (rhoUnit a m : integralRing) else 0 := by
  unfold localBase
  split_ifs <;> simp [coeff_zero_prod]

@[expose] noncomputable def localDenominator (a : ℕ) (m : Fin (2^(a+1))) : Polynomial integralRing :=
  ∑ s, localBase a m s

theorem localDenominator_eval (a : ℕ) (m : Fin (2^(a+1))) (z : ℚ) :
    (localDenominator a m).eval₂ integralRing.subtype z =
      RationalScalar.denominator a ((m:ℚ)-(2^a:ℕ)+((2^(a+1):ℕ):ℚ)*z) := by
  unfold localDenominator RationalScalar.denominator
  rw [eval₂_finsetSum]
  simp_rw [localBase_eval]
  exact Fin.sum_univ_eq_sum_range (fun j => baseKernel a j
    ((m:ℚ)-(2^a:ℕ)+((2^(a+1):ℕ):ℚ)*z)) (2^(a+1))

theorem localDenominator_constant (a : ℕ) (m : Fin (2^(a+1))) :
    (localDenominator a m).coeff 0 = (rhoUnit a m : integralRing) := by
  simp [localDenominator, localBase_constant]

@[expose] noncomputable def denominatorSeries (a : ℕ) (m : Fin (2^(a+1))) : PowerSeries integralRing :=
  (localDenominator a m).toPowerSeries

theorem series_constant (a : ℕ) (m : Fin (2^(a+1))) :
    PowerSeries.constantCoeff (denominatorSeries a m) = (rhoUnit a m : integralRing) := by
  change (localDenominator a m).coeff 0 = _
  exact localDenominator_constant a m

@[expose] noncomputable def inverseSeries (a : ℕ) (m : Fin (2^(a+1))) : PowerSeries integralRing :=
  PowerSeries.invOfUnit (denominatorSeries a m) (rhoUnit a m)

theorem denominator_mul_inverse (a : ℕ) (m : Fin (2^(a+1))) :
    denominatorSeries a m * inverseSeries a m = 1 := by
  exact PowerSeries.mul_invOfUnit _ _ (series_constant a m)

theorem inverse_mul_denominator (a : ℕ) (m : Fin (2^(a+1))) :
    inverseSeries a m * denominatorSeries a m = 1 := by
  rw [mul_comm, denominator_mul_inverse]

/-- Every coefficient of the actual constructed reciprocal is integral. -/
theorem inverse_coefficient_integral (a : ℕ) (m : Fin (2^(a+1))) (n : ℕ) :
    padicNorm 2 ((PowerSeries.coeff n (inverseSeries a m) : integralRing) : ℚ) ≤ 1 :=
  (PowerSeries.coeff n (inverseSeries a m)).property

/-- Actual-object existence statement: the polynomial represents the source
denominator for every rational z and its inverse is over the concrete local ring. -/
theorem actual_integral_denominator_inverse (a : ℕ) (m : Fin (2^(a+1))) :
    ∃ (P : Polynomial integralRing) (Q : PowerSeries integralRing),
      (∀ z : ℚ, P.eval₂ integralRing.subtype z =
        RationalScalar.denominator a ((m:ℚ)-(2^a:ℕ)+((2^(a+1):ℕ):ℚ)*z)) ∧
      (P.toPowerSeries * Q = 1) := by
  exact ⟨localDenominator a m, inverseSeries a m, localDenominator_eval a m, denominator_mul_inverse a m⟩

@[expose] noncomputable def localKernelSeries (a : ℕ) (m s : Fin (2^(a+1))) : PowerSeries integralRing :=
  (localBase a m s).toPowerSeries ^ 2 * inverseSeries a m

/-- This is the defining local rational-kernel identity over the concrete
integral coefficient ring, with no assumed denominator integrality. -/
theorem localKernel_identity (a : ℕ) (m s : Fin (2^(a+1))) :
    denominatorSeries a m * localKernelSeries a m s =
      (localBase a m s).toPowerSeries ^ 2 := by
  unfold localKernelSeries
  calc
    _ = (localBase a m s).toPowerSeries ^ 2 * (denominatorSeries a m * inverseSeries a m) := by ring
    _ = _ := by rw [denominator_mul_inverse, mul_one]

theorem kernel_coefficient_integral (a : ℕ) (m s : Fin (2^(a+1))) (n : ℕ) :
    padicNorm 2 ((PowerSeries.coeff n (localKernelSeries a m s) : integralRing) : ℚ) ≤ 1 :=
  (PowerSeries.coeff n (localKernelSeries a m s)).property

end LocalCharts
#print axioms LocalCharts.normalized_puncture
#print axioms LocalCharts.actual_chart_formula
#print axioms LocalCharts.localBase_eval
#print axioms LocalCharts.localDenominator_eval
#print axioms LocalCharts.localDenominator_constant
#print axioms LocalCharts.denominator_mul_inverse
#print axioms LocalCharts.actual_integral_denominator_inverse
#print axioms LocalCharts.localKernel_identity
#print axioms LocalCharts.kernel_coefficient_integral
