import RationalFibers

namespace GuardedFibers
noncomputable section
open MvPolynomial CubicArithmetic CubicNormalization GridArithmetic StagedResultant
  GuardedReduction RationalFibers GridLower FilteredClosure

variable {K : Type*} [Field K]

def left (a b : K) (w : K) : K := by classical exact if w=a then b else a
def right (a b c : K) (w : K) : K := by classical exact if w=a then c else if w=b then c else b

theorem complement_properties (α β a b c w : K)
    (hab : a ≠ b) (hac : a ≠ c) (hbc : b ≠ c) (hw : w ∈ ({a,b,c} : Set K)) :
    left a b w ∈ ({a,b,c} : Set K) ∧ right a b c w ∈ ({a,b,c} : Set K) ∧
    left a b w ≠ right a b c w ∧
    val (q1A (rootsCoefficients a b c)) w = (left a b w-right a b c w)^2 ∧
    val (q1B (rootsCoefficients a b c) α β) w = -2*middle α β (left a b w) (right a b c w) ∧
    val (q1D (rootsCoefficients a b c) α β) w = constantTerm α β (left a b w) (right a b c w) := by
  classical
  rcases hw with rfl | rfl | rfl
  all_goals
    simp [left,right,hab,hac,hbc,Ne.symm hab,Ne.symm hac,Ne.symm hbc,
      val,q1A,q1B,q1D,rootsCoefficients,middle,constantTerm]
    repeat' constructor
    all_goals ring

theorem q1A_ne_zero (a b c w : K) (hab : a ≠ b) (hac : a ≠ c) (hbc : b ≠ c)
    (hw : w ∈ ({a,b,c} : Set K)) : val (q1A (rootsCoefficients a b c)) w ≠ 0 := by
  obtain ⟨_,_,hxy,hA,_,_⟩ := complement_properties (0 : K) 0 a b c w hab hac hbc hw
  rw [hA]
  exact pow_ne_zero 2 (sub_ne_zero.mpr hxy)

def normalized (a b c : K) (A T : Coeff K) : Coeff K :=
  mulReduced (rootsCoefficients a b c) T (inverseCoeff A a b c)

theorem normalized_value (a b c : K) (A T : Coeff K)
    (hab : a ≠ b) (hac : a ≠ c) (hbc : b ≠ c) (x : K) (hx : x ∈ ({a,b,c} : Set K)) :
    val (normalized a b c A T) x = val T x*(val A x)⁻¹ := by
  rw [normalized,mulReduced_val _ _ _ x (root_in_set a b c x hx)]
  rcases hx with h | h | h
  · rw [h,inverseCoeff_at_first A a b c hab hac]
  · rw [h,inverseCoeff_at_second A a b c hab hbc]
  · rw [h,inverseCoeff_at_third A a b c hac hbc]

def firstLinear (α β a b c : K) : Coeff K :=
  normalized a b c (q1A (rootsCoefficients a b c)) (q1B (rootsCoefficients a b c) α β)
def firstConstant (α β a b c : K) : Coeff K :=
  normalized a b c (q1A (rootsCoefficients a b c)) (q1D (rootsCoefficients a b c) α β)
def secondLinear (α β a b c r : K) : Coeff K := normalized a b c (q2A r) (q2B α β r)
def secondConstant (α β a b c r : K) : Coeff K := normalized a b c (q2A r) (q2D α β r)

theorem first_scalar_identity (α β a b c w u : K)
    (hab : a ≠ b) (hac : a ≠ c) (hbc : b ≠ c) (hw : w ∈ ({a,b,c} : Set K)) :
    PrimeBitLocal.oddS3 α β (left a b w) (right a b c w) u =
      (left a b w-right a b c w)^2*
      (u^2+val (firstLinear α β a b c) w*u+val (firstConstant α β a b c) w) := by
  obtain ⟨_,_,hxy,hA,hB,hD⟩ := complement_properties α β a b c w hab hac hbc hw
  rw [raw_monic α β _ _ u hxy]
  simp only [firstLinear,firstConstant,normalized_value a b c _ _ hab hac hbc w hw,
    hA,hB,hD,div_eq_mul_inv]

theorem second_scalar_identity (α β a b c r z u : K)
    (hab : a ≠ b) (hac : a ≠ c) (hbc : b ≠ c)
    (hz : z ∈ ({a,b,c} : Set K)) (hzr : z ≠ r) :
    PrimeBitLocal.oddS3 α β u z r = (z-r)^2*
      (u^2+val (secondLinear α β a b c r) z*u+val (secondConstant α β a b c r) z) := by
  have hsym : PrimeBitLocal.oddS3 α β u z r = PrimeBitLocal.oddS3 α β z r u := by
    unfold PrimeBitLocal.oddS3
    ring
  rw [hsym,raw_monic α β z r u hzr]
  simp only [secondLinear,secondConstant,normalized_value a b c _ _ hab hac hbc z hz,
    q2A_value,q2B_value,q2D_value,middle,constantTerm,div_eq_mul_inv]

theorem first_splits (α β a b c w : K)
    (hab : a ≠ b) (hac : a ≠ c) (hbc : b ≠ c) (hw : w ∈ ({a,b,c} : Set K))
    (hlift : ∀ x ∈ ({a,b,c} : Set K), ∃ Y : K, Y^2=x^3+α*x+β) :
    ∃ ρ τ : K, val (firstLinear α β a b c) w = -(ρ+τ) ∧
      val (firstConstant α β a b c) w = ρ*τ := by
  obtain ⟨hx,hy,hxy,hA,hB,hD⟩ := complement_properties α β a b c w hab hac hbc hw
  obtain ⟨Y,hY⟩ := hlift _ hx
  obtain ⟨Z,hZ⟩ := hlift _ hy
  obtain ⟨ρ,τ,hs,hp⟩ := split_monic α β _ _ Y Z hxy hY hZ
  refine ⟨ρ,τ,?_,?_⟩
  · simpa only [firstLinear,normalized_value a b c _ _ hab hac hbc w hw,hA,hB,div_eq_mul_inv] using hs
  · simpa only [firstConstant,normalized_value a b c _ _ hab hac hbc w hw,hA,hD,div_eq_mul_inv] using hp

theorem no_common_root (α β a b c r w z u : K)
    (hab : a ≠ b) (hac : a ≠ c) (hbc : b ≠ c)
    (hw : w ∈ ({a,b,c} : Set K)) (hz : z ∈ ({a,b,c} : Set K)) (hr : r ∉ ({a,b,c} : Set K))
    (hns : NoSolution α β a b c r)
    (h1 : u^2+val (firstLinear α β a b c) w*u+val (firstConstant α β a b c) w = 0) :
    u^2+val (secondLinear α β a b c r) z*u+val (secondConstant α β a b c r) z ≠ 0 := by
  intro h2
  obtain ⟨hx,hy,hxy,_,_,_⟩ := complement_properties α β a b c w hab hac hbc hw
  have hzr : z ≠ r := by intro h; apply hr; simpa [← h] using hz
  apply hns
  apply solution_of_common α β a b c r (left a b w) (right a b c w) z u hx hy hz hxy
  · rw [first_scalar_identity α β a b c w u hab hac hbc hw,h1,mul_zero]
  · rw [second_scalar_identity α β a b c r z u hab hac hbc hz hzr,h2,mul_zero]

theorem scalar_resultant_nonzero (α β a b c r w z : K)
    (hab : a ≠ b) (hac : a ≠ c) (hbc : b ≠ c)
    (hw : w ∈ ({a,b,c} : Set K)) (hz : z ∈ ({a,b,c} : Set K))
    (hlift : ∀ x ∈ ({a,b,c} : Set K), ∃ Y : K, Y^2=x^3+α*x+β)
    (hns : NoSolution α β a b c r) :
    quadRes (val (firstLinear α β a b c) w) (val (firstConstant α β a b c) w)
      (val (secondLinear α β a b c r) z) (val (secondConstant α β a b c r) z) ≠ 0 := by
  have hr := target_outside_base α β a b c r hab hlift hns
  obtain ⟨ρ,τ,hA,hB⟩ := first_splits α β a b c w hab hac hbc hw hlift
  have hρ := no_common_root α β a b c r w z ρ hab hac hbc hw hz hr hns (by rw [hA,hB]; ring)
  have hτ := no_common_root α β a b c r w z τ hab hac hbc hw hz hr hns (by rw [hA,hB]; ring)
  rw [hA,hB,quadRes_split]
  exact mul_ne_zero hρ hτ

end
end GuardedFibers

#print axioms GuardedFibers.scalar_resultant_nonzero
