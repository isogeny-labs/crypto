import Mathlib.Algebra.CharP.Basic
import Mathlib.Data.ZMod.Basic
import Mathlib.Tactic.FieldSimp
import Mathlib.Tactic.Ring
import Lean.Elab.Tactic.Omega

/-! Universal collector identities. Geometric divisor and quotient bridges
remain written; no actual modular factor inclusion is assumed as an axiom. -/
namespace KoblitzCollector27

theorem fixed_divisor_difference {A : Type*} [AddGroup A]
    (s : A → A) (d : A) (h : s d = d) : d - s d = 0 := by simp [h]

theorem fixed_function_ratio {K : Type*} [Field K]
    (s : K → K) (f : K) (hf : f ≠ 0) (h : s f = f) :
    f / s f = 1 := by simp [h, hf]

theorem ratio_norm {K : Type*} [Field K] (s : K ≃+* K)
    (hss : ∀ x, s (s x) = x) (f : K) (hf : f ≠ 0) :
    (f / s f) * s (f / s f) = 1 := by
  have hsf : s f ≠ 0 := by simpa using hf
  simp only [map_div₀, hss]
  field_simp

theorem ratio_nontrivial {K : Type*} [Field K] (f g : K)
    (hg : g ≠ 0) (hne : f ≠ g) : f / g ≠ 1 := by
  intro h
  apply hne
  exact (div_eq_one_iff_eq hg).mp h

theorem hilbert90_explicit {K : Type*} [Field K] (g : K)
    (hg : g ≠ 0) (hplus : 1 + g ≠ 0) :
    (1 + g) / (1 + g⁻¹) = g := by
  have hden : 1 + g⁻¹ ≠ 0 := by
    intro h
    apply hplus
    have hh := congrArg (fun x : K => x * g) h
    simpa [add_mul, hg, add_comm] using hh
  apply (div_eq_iff hden).2
  simp [mul_add, hg, add_comm]

theorem translation_not_fixed {K : Type*} [AddGroup K] (x b : K)
    (hb : b ≠ 0) : x + b ≠ x := by
  intro h
  apply hb
  exact add_left_cancel (show x + b = x + 0 by simpa using h)

theorem transport_not_fixed {X : Type*} (s t : X → X)
    (ht : Function.Injective t) (hcomm : ∀ x, s (t x) = t (s x))
    (x : X) (hx : s x ≠ x) : s (t x) ≠ t x := by
  intro h
  apply hx
  apply ht
  simpa [← hcomm x] using h

theorem etale_double_genus_numeric (g : ℤ)
    (h : 2 * 61 - 2 = 2 * (2 * g - 2)) : g = 31 := by omega

theorem prym_dimension_numeric : (61 : ℤ) - 31 = 30 := by decide

theorem rr_dimensions_numeric :
    (128 : ℤ) > 2*61-2 ∧ 128+1-61=68 ∧
    (64 : ℤ) > 2*31-2 ∧ 64+1-31=34 ∧ (68 : ℤ)=34+34 := by decide

def cuspRelation (t : ZMod 11) : ZMod 11 :=
  11 + 7*t^2 + 11*t^3 + 5*t^4

theorem cusp_relation_at_six : cuspRelation 6 = 0 := by decide

theorem cusp_relation_other_primitive (t : ZMod 11)
    (ht : t=2 ∨ t=7 ∨ t=8) : cuspRelation t ≠ 0 := by
  rcases ht with rfl|rfl|rfl <;> decide

theorem cusp_bezout_evaluation : ∀ t : ZMod 11,
    (2*t^2+t+5)*(t^5+1)+(4*t^3+2*t^2-5)*cuspRelation t=t+5 := by decide

theorem cusp_common_root_exact : ∀ t : ZMod 11,
    (t^5+1=0 ∧ cuspRelation t=0) ↔ t=6 := by decide

theorem cusp_determinant_factorization :
    (1477212 : ℤ)=2^2*3*11*19^2*31 := by decide

theorem cusp_determinant_eleven_once :
    1477212 % 11=0 ∧ 1477212 % 121≠0 := by decide

theorem source_genus_from_tame_degree (g : ℤ)
    (h : 2*g-2=5*(2*6)+10*6) : g=61 := by omega

theorem jet_129_degree_obstruction (zeroDegree poleDegree : ℤ)
    (hz : 129 ≤ zeroDegree) (hp : poleDegree ≤ 128)
    (hbalance : zeroDegree ≤ poleDegree) : False := by omega

end KoblitzCollector27
