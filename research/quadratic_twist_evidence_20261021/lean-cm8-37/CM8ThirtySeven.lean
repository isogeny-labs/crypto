import CM8Dual

/-! Actual finite F37 coordinate maps. No generic isogeny classification is assumed. -/
namespace CM8ThirtySeven
set_option maxRecDepth 1000000
set_option maxHeartbeats 0
open WeierstrassCurve.Affine
abbrev k := ZMod 37
instance : Fact (Nat.Prime 37) := ⟨by decide⟩
abbrev E : WeierstrassCurve.Affine k := TwistEmbedding.short 7 19
abbrev T : WeierstrassCurve.Affine k := TwistEmbedding.short 28 33
instance : E.IsElliptic := by
  constructor
  exact isUnit_iff_ne_zero.mpr (by decide)
instance : T.IsElliptic := by
  constructor
  exact isUnit_iff_ne_zero.mpr (by decide)
def ep (x y : k) (h : y^2=x^3+7*x+19) : E.Point :=
  Point.some x y ((equation_iff_nonsingular_of_Δ_ne_zero
    (WeierstrassCurve.isUnit_Δ E).ne_zero).mp (by
      simpa [equation_iff,E,TwistEmbedding.short] using h))
def tp (x y : k) (h : y^2=x^3+28*x+33) : T.Point :=
  Point.some x y ((equation_iff_nonsingular_of_Δ_ne_zero
    (WeierstrassCurve.isUnit_Δ T).ne_zero).mp (by
      simpa [equation_iff,T,TwistEmbedding.short] using h))
def fx (x : k) : k := x+18/(x-4)
def fy (x y : k) : k := y*(1-18/(x-4)^2)
def dx (x : k) : k := 28*(x+35/(x-29))
def dy (x y : k) : k := 14*y*(1-35/(x-29)^2)
theorem forward_equation : ∀ x y : k, y^2=x^3+7*x+19 → x≠4 →
    (fy x y)^2=(fx x)^3+28*fx x+33 := by decide +kernel
theorem dual_equation : ∀ x y : k, y^2=x^3+28*x+33 → x≠29 →
    (dy x y)^2=(dx x)^3+7*dx x+19 := by decide +kernel
def phi : E.Point → T.Point
  | .zero => 0
  | .some x y h => if hx : x=4 then 0 else
      tp (fx x) (fy x y) (forward_equation x y (by
        simpa [equation_iff,E,TwistEmbedding.short] using h.left) hx)
def dual : T.Point → E.Point
  | .zero => 0
  | .some x y h => if hx : x=29 then 0 else
      ep (dx x) (dy x y) (dual_equation x y (by
        simpa [equation_iff,T,TwistEmbedding.short] using h.left) hx)
def P : E.Point := ep 1 8 (by decide)
def U : T.Point := tp 32 29 (by decide)
def K : E.Point := ep 4 0 (by decide)
def TK : T.Point := tp 29 0 (by decide)
def inverse19 (R : T.Point) : E.Point := (10:ℕ) • dual R
theorem twist_coefficients : (35:k)^2*7=28 ∧ (35:k)^3*19=33 := by decide
theorem twist_nonsquare : ∀ u : k, u^2≠35 := by decide +kernel
theorem generator_image : phi P=U := by decide +kernel
theorem source_order : (19:ℕ) • P=0 ∧ P≠0 := by decide +kernel
theorem target_order : (19:ℕ) • U=0 ∧ U≠0 := by decide +kernel
theorem scalar_transport : ∀ n : Fin 19, phi (n.val • P)=n.val • U := by decide +kernel
theorem source_scalar_injective : ∀ i j : Fin 19,
    i.val • P=j.val • P ↔ i=j := by decide +kernel
theorem target_scalar_injective : ∀ i j : Fin 19,
    i.val • U=j.val • U ↔ i=j := by decide +kernel
theorem subgroup_additive : ∀ i j : Fin 19,
    phi (i.val • P+j.val • P)=phi (i.val • P)+phi (j.val • P) := by decide +kernel
theorem inverse_source : ∀ n : Fin 19, inverse19 (phi (n.val • P))=n.val • P := by decide +kernel
theorem inverse_target : ∀ n : Fin 19, phi (inverse19 (n.val • U))=n.val • U := by decide +kernel
theorem source_kernel_control : K≠0 ∧ phi K=0 ∧ (19:ℕ) • K≠0 := by decide +kernel
theorem dual_kernel_control : TK≠0 ∧ dual TK=0 := by decide +kernel
theorem global_inverse_fails : inverse19 (phi K)≠K := by decide +kernel
theorem actual_source_reduction (Q : E.Point)
    (hQ : ∃ i : Fin 19, Q=i.val • P) (n : Fin 19) :
    phi Q=n.val • U ↔ Q=n.val • P := by
  obtain ⟨i,rfl⟩ := hQ
  rw [scalar_transport]
  constructor
  · intro h
    have hi := (target_scalar_injective i n).mp h
    rw [hi]
  · intro h
    rw [←scalar_transport,h,scalar_transport]
theorem no_short_scaling : ∀ u : k, ¬(7*u^4=28 ∧ 19*u^6=33) := by decide +kernel
theorem no_admissible_short_change : ∀ u r s t : k, u≠0 →
    ¬(2*u^5*s=0 ∧ 2*u^3*t=0 ∧ u^4*s^2=3*u^4*r ∧
      7*u^6+2*u^2*s*t=3*u^4*r^2+28*u^2 ∧
      19*u^6+t^2=r^3+28*r+33) := by
  intro u r s t hu h
  rcases h with ⟨hs,ht,hr,ha,hb⟩
  have hs0 : s=0 := (mul_eq_zero.mp hs).resolve_left
    (mul_ne_zero (by decide) (pow_ne_zero _ hu))
  have ht0 : t=0 := (mul_eq_zero.mp ht).resolve_left
    (mul_ne_zero (by decide) (pow_ne_zero _ hu))
  have hr0 : r=0 := by
    have hz : (3:k)*u^4*r=0 := by simpa [hs0] using hr.symm
    exact (mul_eq_zero.mp hz).resolve_left
      (mul_ne_zero (by decide) (pow_ne_zero _ hu))
  have ha' : 7*u^6=28*u^2 := by simpa [hs0,ht0,hr0] using ha
  have hb' : 19*u^6=33 := by simpa [ht0,hr0] using hb
  have hc : (7*u^4)*u^2=28*u^2 := by
    calc
      _ = 7*u^6 := by ring
      _ = 28*u^2 := ha'
  have hscale : 7*u^4=28 := mul_right_cancel₀ (pow_ne_zero 2 hu) hc
  exact no_short_scaling u ⟨hscale,hb'⟩
end CM8ThirtySeven
