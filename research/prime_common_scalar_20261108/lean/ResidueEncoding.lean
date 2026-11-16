module
public import FiniteBuffers
public import Mathlib.NumberTheory.Padics.RingHoms
public section
namespace ResidueEncoding
open LocalIntegral LocalCharts FiniteBuffers

/-- Embed the concrete rational integral subring into genuine two-adic integers. -/
@[expose] noncomputable def toPadic : integralRing →+* ℤ_[2] :=
  ((Rat.castHom ℚ_[2]).comp integralRing.subtype).codRestrict (PadicInt.subring 2) (by
    intro r
    change ‖((r:ℚ) : ℚ_[2])‖ ≤ 1
    rw [Padic.eq_padicNorm]
    exact_mod_cast r.property)

@[simp] theorem coe_toPadic (r : integralRing) : ((toPadic r : ℤ_[2]) : ℚ_[2]) = ((r:ℚ):ℚ_[2]) := rfl

@[expose] noncomputable def toResidue (p : ℕ) : integralRing →+* ZMod (2^p) :=
  (PadicInt.toZModPow p).comp toPadic

/-- The residue map has exactly the intended coefficient-precision kernel. -/
theorem residue_kernel (p : ℕ) : RingHom.ker (toResidue p) = coefficientIdeal p := by
  ext r
  rw [RingHom.mem_ker]
  constructor
  · intro h
    have hh : toPadic r ∈ RingHom.ker (PadicInt.toZModPow p) := h
    rw [PadicInt.ker_toZModPow, Ideal.mem_span_singleton] at hh
    obtain ⟨z,hz⟩ := hh
    have hv : ((r:ℚ):ℚ_[2]) = (2:ℚ_[2])^p*(z:ℚ_[2]) := by
      have hv := congrArg (PadicInt.Coe.ringHom : ℤ_[2] →+* ℚ_[2]) hz
      rw [map_mul,map_pow,map_natCast] at hv
      simpa only [PadicInt.Coe.ringHom_apply,coe_toPadic,Nat.cast_ofNat] using hv
    have hpow : (2:ℚ_[2])^p ≠ 0 := pow_ne_zero _ (by norm_num)
    have heq : (((r:ℚ)/(2:ℚ)^p : ℚ):ℚ_[2]) = (z:ℚ_[2]) := by
      push_cast
      rw [hv]
      field_simp
    have hr : padicNorm 2 ((r:ℚ)/(2:ℚ)^p) ≤ 1 := by
      have hn : ‖(((r:ℚ)/(2:ℚ)^p : ℚ):ℚ_[2])‖ ≤ 1 := by rw [heq]; exact z.property
      rw [Padic.eq_padicNorm] at hn
      exact_mod_cast hn
    rw [coefficientIdeal, Ideal.mem_span_singleton]
    refine ⟨⟨(r:ℚ)/(2:ℚ)^p,hr⟩,?_⟩
    apply Subtype.ext
    change (r:ℚ) = (2:ℚ)^p*((r:ℚ)/(2:ℚ)^p)
    field_simp
  · intro h
    rw [coefficientIdeal, Ideal.mem_span_singleton] at h
    obtain ⟨s,hs⟩ := h
    rw [hs, map_mul, map_pow, map_ofNat]
    have hz : (2 : ZMod (2^p))^p=0 := by
      simpa only [Nat.cast_pow,Nat.cast_ofNat] using ZMod.natCast_self (2^p)
    rw [hz,zero_mul]

/-- Coefficient residue classes are exactly the usual finite modular ring. -/
@[expose] noncomputable def coefficientEquiv (p : ℕ) :
    (integralRing ⧸ coefficientIdeal p) ≃+* ZMod (2^p) :=
  (Ideal.quotEquivOfEq (residue_kernel p).symm).trans
    (RingHom.quotientKerEquivOfSurjective (ZMod.ringHom_surjective (toResidue p)))

theorem coefficient_equiv_exists (p : ℕ) :
    Nonempty ((integralRing ⧸ coefficientIdeal p) ≃+* ZMod (2^p)) :=
  ⟨coefficientEquiv p⟩

theorem coefficient_card (p : ℕ) : Nat.card (integralRing ⧸ coefficientIdeal p) = 2^p := by
  rw [Nat.card_congr (coefficientEquiv p).toEquiv]
  simp

theorem denominator_unit (p : ℕ) (r : integralRing) :
    IsUnit ((r:ℚ).den : ZMod (2^p)) := by
  have hn : ‖((r:ℚ):ℚ_[2])‖ ≤ 1 := by rw [Padic.eq_padicNorm]; exact_mod_cast r.property
  have hu := PadicInt.isUnit_den (r:ℚ) hn
  have h := hu.map (PadicInt.toZModPow p)
  simpa only [map_natCast] using h

/-- The abstract map is the ordinary numerator/denominator formula. -/
theorem residue_formula (p : ℕ) (r : integralRing) :
    toResidue p r = ((r:ℚ).num : ZMod (2^p)) * ((r:ℚ).den : ZMod (2^p))⁻¹ := by
  have hden : ((r:ℚ).den : ℚ) ≠ 0 := by exact_mod_cast (r:ℚ).den_nz
  have hrel : r*((r:ℚ).den : integralRing) = ((r:ℚ).num : integralRing) := by
    apply Subtype.ext
    change (r:ℚ) * ((r:ℚ).den : ℚ) = ((r:ℚ).num : ℚ)
    exact ((div_eq_iff hden).mp (r:ℚ).num_div_den).symm
  have hmul := congrArg (toResidue p) hrel
  rw [map_mul,map_natCast,map_intCast] at hmul
  calc
    _ = toResidue p r * (((r:ℚ).den : ZMod (2^p)) * ((r:ℚ).den : ZMod (2^p))⁻¹) := by
      rw [ZMod.mul_inv_of_unit _ (denominator_unit p r),mul_one]
    _ = (toResidue p r * ((r:ℚ).den : ZMod (2^p))) * ((r:ℚ).den : ZMod (2^p))⁻¹ := by ring
    _ = _ := by rw [hmul]

/-- Finite inverse correctness under any coefficient ring map. -/
theorem invert_mapped_actual {R : Type*} [CommRing R] (π : integralRing →+* R)
    (a : ℕ) (m : Fin (2^(a+1))) (q : ℕ) :
    invertBuffer q (fun n : Fin q => π (PowerSeries.coeff n (denominatorSeries a m)))
      (Units.map π.toMonoidHom (rhoUnit a m)) =
      (fun n : Fin q => π (PowerSeries.coeff n (inverseSeries a m))) := by
  by_cases hq : q=0
  · subst q; funext n; exact Fin.elim0 n
  have hqpos : 0<q := Nat.pos_of_ne_zero hq
  let v : Fin q → R := fun n => π (PowerSeries.coeff n (denominatorSeries a m))
  let u := Units.map π.toMonoidHom (rhoUnit a m)
  let D := PowerSeries.map π (denominatorSeries a m)
  let I := PowerSeries.map π (inverseSeries a m)
  let E := liftBuffer q v
  let F := PowerSeries.invOfUnit E u
  have hDI : D*I=1 := by
    dsimp [D,I]
    rw [← map_mul,denominator_mul_inverse,map_one]
  have hc : PowerSeries.constantCoeff E = (u:R) := by
    rw [← PowerSeries.coeff_zero_eq_constantCoeff_apply]
    have h := liftBuffer_coeff q v (⟨0,hqpos⟩ : Fin q)
    change PowerSeries.coeff 0 E = _
    rw [h]
    change π (PowerSeries.coeff 0 (denominatorSeries a m)) = _
    rw [PowerSeries.coeff_zero_eq_constantCoeff_apply,series_constant]
    rfl
  have hEF : E*F=1 := PowerSeries.mul_invOfUnit E u hc
  have hi : ∀ n<q, PowerSeries.coeff n E = PowerSeries.coeff n D := by
    intro n hn
    simpa only [E,D,PowerSeries.coeff_map,v] using liftBuffer_coeff q v (⟨n,hn⟩ : Fin q)
  funext n
  simpa only [invertBuffer,F,I,PowerSeries.coeff_map,v,u] using
    inverse_low_coefficients q D I E F hDI hEF hi n n.isLt

theorem modular_inverse_correct (a : ℕ) (m : Fin (2^(a+1))) (p q : ℕ) :
    invertBuffer q (fun n : Fin q => toResidue p (PowerSeries.coeff n (denominatorSeries a m)))
      (Units.map (toResidue p).toMonoidHom (rhoUnit a m)) =
      (fun n : Fin q => toResidue p (PowerSeries.coeff n (inverseSeries a m))) :=
  invert_mapped_actual (toResidue p) a m q

theorem modular_buffer_card (p q : ℕ) : Nat.card (Fin q → ZMod (2^p)) = 2^(p*q) := by
  simp [Nat.card_eq_fintype_card,pow_mul]

end ResidueEncoding
#print axioms ResidueEncoding.residue_kernel
#print axioms ResidueEncoding.coefficient_card
#print axioms ResidueEncoding.coefficient_equiv_exists
#print axioms ResidueEncoding.residue_formula
#print axioms ResidueEncoding.modular_inverse_correct
#print axioms ResidueEncoding.modular_buffer_card
