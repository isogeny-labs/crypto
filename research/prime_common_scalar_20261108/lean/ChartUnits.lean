module
public import StreamPrecision
public import Mathlib.RingTheory.Nilpotent.Basic
public section
namespace ChartUnits
open LocalIntegral LocalCharts ResidueEncoding Polynomial CauchyNormalizer

/-- The actual alpha/d ratio has a factor of two, not just integrality. -/
theorem halfIntegerRatioIntegral (a : ℕ) (d : ℤ) (hd : d≠0)
    (hbound : d.natAbs<2^(a+1)) :
    padicNorm 2 (((2^a:ℕ):ℚ)/d)≤1 := by
  have hv := bounded_valuation a d.natAbs (Int.natAbs_ne_zero.mpr hd) hbound
  have hq : (d:ℚ)≠0 := by exact_mod_cast hd
  have hp : (((2^a:ℕ):ℚ))≠0 := by positivity
  have hn : padicValRat 2 (((2^a:ℕ):ℚ))=(a:ℕ) := by
    rw [Nat.cast_pow,padicValRat.pow,padicValRat.self (by decide)]
    simp
  have hval : 0≤padicValRat 2 (((2^a:ℕ):ℚ)/d) := by
    rw [padicValRat.div hp hq,hn,padicValRat.of_int]
    unfold padicValInt
    omega
  rw [padicNorm.eq_zpow_of_nonzero (div_ne_zero hp hq)]
  exact zpow_le_one_of_nonpos₀ (by norm_num) (by omega)

theorem chartRatioEven (a : ℕ) (m j : Fin (2^(a+1))) :
    (2:integralRing) ∣ chartRatio a m j := by
  by_cases hm : m=j
  · subst j
    refine ⟨0,?_⟩
    apply Subtype.ext
    simp [chartRatio]
  have hd : (m:ℤ)-(j:ℤ)≠0 := by
    intro hz
    have he : m.val=j.val := by omega
    exact hm (Fin.ext he)
  have hb : ((m:ℤ)-(j:ℤ)).natAbs<2^(a+1) := by
    have hm' := m.isLt
    have hj' := j.isLt
    omega
  let half : integralRing := ⟨((2^a:ℕ):ℚ)/((m:ℤ)-(j:ℤ)),by change padicNorm 2 _≤1; simpa using halfIntegerRatioIntegral a _ hd hb⟩
  refine ⟨half,?_⟩
  apply Subtype.ext
  change ((2^(a+1):ℕ):ℚ)/((m:ℚ)-(j:ℚ))=(2:ℚ)*(half:ℚ)
  simp only [half,Int.cast_sub,Int.cast_natCast,pow_succ,Nat.cast_mul,Nat.cast_ofNat]
  ring

/-- Actual integral chart reduces to its node value under every characteristic2 map. -/
theorem localBaseMap {R : Type*} [CommRing R] (π : integralRing→+*R) (h2 : π 2=0)
    (a : ℕ) (m s : Fin (2^(a+1))) :
    (localBase a m s).map π=if s=m then C (π (rhoUnit a m:integralRing)) else 0 := by
  have hr (j : Fin (2^(a+1))) : π (chartRatio a m j)=0 := by
    obtain ⟨r,he⟩ := chartRatioEven a m j
    rw [he,map_mul,h2,zero_mul]
  unfold localBase
  split_ifs <;> simp [Polynomial.map_prod,hr]

theorem localDenominatorMap {R : Type*} [CommRing R] (π : integralRing→+*R)
    (h2 : π 2=0) (a : ℕ) (m : Fin (2^(a+1))) :
    (localDenominator a m).map π=C (π (rhoUnit a m:integralRing)) := by
  simp [localDenominator,Polynomial.map_sum,localBaseMap π h2]


/-- The earlier polynomial scalar, expressed in the same actual integral chart. -/
@[expose] noncomputable def localNormalizer (a : ℕ) (m : Fin (2^(a+1))) :
    Polynomial integralRing :=
  ∑ s, C (((rhoUnit a s)⁻¹:integralRingˣ):integralRing)^2*localBase a m s

theorem localNormalizerEval (a : ℕ) (m : Fin (2^(a+1))) (z : ℚ) :
    (localNormalizer a m).eval₂ integralRing.subtype z=
      CommonScalar.normalizer a ((m:ℚ)-(2^a:ℕ)+((2^(a+1):ℕ):ℚ)*z) := by
  rw [CommonScalar.normalizer_formula]
  unfold localNormalizer
  simp only [eval₂_finsetSum,eval₂_mul,eval₂_pow,eval₂_C,localBase_eval]
  simp only [rhoUnit]
  rw [←Fin.sum_univ_eq_sum_range]
  apply Finset.sum_congr rfl
  intro s hs
  simp [div_eq_mul_inv]
  ring

theorem localNormalizerMap {R : Type*} [CommRing R] (π : integralRing→+*R)
    (h2 : π 2=0) (a : ℕ) (m : Fin (2^(a+1))) :
    (localNormalizer a m).map π=C (π (((rhoUnit a m)⁻¹:integralRingˣ):integralRing)) := by
  classical
  unfold localNormalizer
  simp only [Polynomial.map_sum,Polynomial.map_mul,Polynomial.map_pow,map_C,
    localBaseMap π h2]
  simp only [mul_ite,mul_zero,Finset.sum_ite_eq',Finset.mem_univ,if_true]
  rw [←C_pow,←C_mul,←map_pow,←map_mul]
  congr 2
  simp [pow_two,mul_assoc]

/-- Convert an actual coefficient-residue identity to an even polynomial remainder. -/
theorem evenPolynomialRemainder (P : Polynomial integralRing) (c : integralRing)
    (h : P.map (toResidue 1)=C (toResidue 1 c)) :
    C (2:integralRing) ∣ P-C c := by
  apply (C_dvd_iff_dvd_coeff _ _).mpr
  intro j
  have he : ((P-C c).map (toResidue 1)).coeff j=0 := by
    rw [Polynomial.map_sub,map_C,h,sub_self,coeff_zero]
  rw [coeff_map] at he
  have hz : toResidue 1 ((P-C c).coeff j)=toResidue 1 0 := by simpa using he
  simpa using (StreamPrecision.residueEqIff 1 ((P-C c).coeff j) 0).mp hz

theorem twoResidueZero : toResidue 1 (2:integralRing)=0 := by
  rw [map_ofNat]
  simpa using ZMod.natCast_self 2

theorem denominatorEvenRemainder (a : ℕ) (m : Fin (2^(a+1))) :
    C (2:integralRing) ∣ localDenominator a m-C (rhoUnit a m:integralRing) :=
  evenPolynomialRemainder _ _ (localDenominatorMap _ twoResidueZero a m)

theorem normalizerEvenRemainder (a : ℕ) (m : Fin (2^(a+1))) :
    C (2:integralRing) ∣ localNormalizer a m-
      C (((rhoUnit a m)⁻¹:integralRingˣ):integralRing) :=
  evenPolynomialRemainder _ _ (localNormalizerMap _ twoResidueZero a m)

/-- Unit evaluation for an actual even remainder; valid with nilpotents and
zero divisors in the target. No unspecified normalizer-unit premise. -/
theorem unitEvalOfEvenRemainder {R : Type*} [CommRing R] (π : integralRing→+*R)
    (h2 : IsNilpotent (π 2)) (P : Polynomial integralRing) (c : integralRingˣ)
    (he : C (2:integralRing) ∣ P-C (c:integralRing)) (z : R) :
    IsUnit (P.eval₂ π z) := by
  obtain ⟨Q,hQ⟩ := he
  have hP : P=C (c:integralRing)+C (2:integralRing)*Q := by rw [←hQ]; ring
  rw [hP,eval₂_add,eval₂_mul,eval₂_C,eval₂_C]
  have hn : IsNilpotent (π 2*Q.eval₂ π z) := by
    obtain ⟨n,hn⟩ := h2
    exact ⟨n,by rw [mul_pow,hn,zero_mul]⟩
  exact hn.isUnit_add_left_of_commute (c.isUnit.map π) (Commute.all _ _)

theorem actualDenominatorUnit {R : Type*} [CommRing R] (π : integralRing→+*R)
    (h2 : IsNilpotent (π 2)) (a : ℕ) (m : Fin (2^(a+1))) (z : R) :
    IsUnit ((localDenominator a m).eval₂ π z) :=
  unitEvalOfEvenRemainder π h2 _ (rhoUnit a m) (denominatorEvenRemainder a m) z

theorem actualNormalizerUnit {R : Type*} [CommRing R] (π : integralRing→+*R)
    (h2 : IsNilpotent (π 2)) (a : ℕ) (m : Fin (2^(a+1))) (z : R) :
    IsUnit ((localNormalizer a m).eval₂ π z) :=
  unitEvalOfEvenRemainder π h2 _ (rhoUnit a m)⁻¹ (normalizerEvenRemainder a m) z

theorem actualComparisonFactorUnit {R : Type*} [CommRing R] (π : integralRing→+*R)
    (h2 : IsNilpotent (π 2)) (a : ℕ) (m : Fin (2^(a+1))) (z : R) :
    IsUnit ((localNormalizer a m).eval₂ π z*(localDenominator a m).eval₂ π z) :=
  (actualNormalizerUnit π h2 a m z).mul (actualDenominatorUnit π h2 a m z)


theorem twoResidueNilpotent (p : ℕ) : IsNilpotent (toResidue p (2:integralRing)) := by
  refine ⟨p,?_⟩
  rw [map_ofNat]
  simpa only [Nat.cast_pow,Nat.cast_ofNat] using ZMod.natCast_self (2^p)

theorem actualModularComparisonUnit (p a : ℕ) (m : Fin (2^(a+1))) (z : ZMod (2^p)) :
    IsUnit ((localNormalizer a m).eval₂ (toResidue p) z*
      (localDenominator a m).eval₂ (toResidue p) z) :=
  actualComparisonFactorUnit _ (twoResidueNilpotent p) a m z

@[expose] noncomputable def denominatorUnit {R : Type*} [CommRing R]
    (π : integralRing→+*R) (h2 : IsNilpotent (π 2))
    (a : ℕ) (m : Fin (2^(a+1))) (z : R) : Rˣ :=
  (actualDenominatorUnit π h2 a m z).unit

@[expose] noncomputable def normalizedEntry {R : Type*} [CommRing R]
    (π : integralRing→+*R) (h2 : IsNilpotent (π 2))
    (a : ℕ) (m s : Fin (2^(a+1))) (z : R) : R :=
  ((denominatorUnit π h2 a m z)⁻¹:Rˣ)*(localBase a m s).eval₂ π z^2

/-- The finite rational repair and earlier scalar square differ by one actual
common unit, independent of the entry index. -/
theorem actualEntryComparison {R : Type*} [CommRing R] (π : integralRing→+*R)
    (h2 : IsNilpotent (π 2)) (a : ℕ) (m s : Fin (2^(a+1))) (z : R) :
    ((localNormalizer a m).eval₂ π z*(localDenominator a m).eval₂ π z)*
      normalizedEntry π h2 a m s z=
      (localNormalizer a m).eval₂ π z*(localBase a m s).eval₂ π z^2 := by
  have hd : ((denominatorUnit π h2 a m z):R)=(localDenominator a m).eval₂ π z :=
    (actualDenominatorUnit π h2 a m z).unit_spec
  unfold normalizedEntry
  rw [←hd]
  calc
    _ = (localNormalizer a m).eval₂ π z*
      (((denominatorUnit π h2 a m z):R)*((denominatorUnit π h2 a m z)⁻¹:Rˣ))*
      (localBase a m s).eval₂ π z^2 := by ring
    _ = _ := by simp

theorem unitDvdIff {R : Type*} [CommRing R] (u : Rˣ) (d x : R) :
    d ∣ (u:R)*x ↔ d ∣ x := by
  constructor
  · rintro ⟨r,hr⟩
    refine ⟨((u⁻¹:Rˣ):R)*r,?_⟩
    calc
      x=((u⁻¹:Rˣ):R)*((u:R)*x) := by simp
      _ = ((u⁻¹:Rˣ):R)*(d*r) := by rw [hr]
      _ = d*(((u⁻¹:Rˣ):R)*r) := by ring
  · rintro ⟨r,hr⟩
    refine ⟨(u:R)*r,?_⟩
    rw [hr]
    ring

/-- Every scalar divisibility threshold is preserved in the actual finite
entry comparison. This is not a minimal mode-count theorem. -/
theorem actualEntryDivisibility {R : Type*} [CommRing R] (π : integralRing→+*R)
    (h2 : IsNilpotent (π 2)) (a : ℕ) (m s : Fin (2^(a+1))) (z d : R) :
    d ∣ (localNormalizer a m).eval₂ π z*(localBase a m s).eval₂ π z^2 ↔
      d ∣ normalizedEntry π h2 a m s z := by
  rw [←actualEntryComparison π h2 a m s z]
  obtain ⟨u,hu⟩ := actualComparisonFactorUnit π h2 a m z
  rw [←hu]
  exact unitDvdIff u d _

end ChartUnits
#print axioms ChartUnits.chartRatioEven
#print axioms ChartUnits.localNormalizerEval
#print axioms ChartUnits.actualDenominatorUnit
#print axioms ChartUnits.actualNormalizerUnit
#print axioms ChartUnits.actualModularComparisonUnit
#print axioms ChartUnits.actualEntryComparison
#print axioms ChartUnits.actualEntryDivisibility
