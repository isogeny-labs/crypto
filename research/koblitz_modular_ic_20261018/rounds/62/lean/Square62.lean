import Mathlib.Tactic.FieldSimp
import Mathlib.Tactic.LinearCombination
import Mathlib.Tactic.NoncommRing

namespace Square62

/-- Sound rational parity reconstruction after clearing the actual denominator. -/
theorem rational_parity {K : Type*} [Field K] (n d e o u : K)
    (hd : d ≠ 0) (h : n*d = e^2 + u*o^2) :
    n/d = (e/d)^2 + u*(o/d)^2 := by
  field_simp [hd]
  linear_combination h

/-- Uniqueness under explicit characteristic-two and nonsquare-parameter premises. -/
theorem decomposition_unique {K : Type*} [Field K] (u g h g' h' : K)
    (hchar : (2 : K) = 0) (hu : ∀ t : K, t^2 ≠ u)
    (heq : g^2 + u*h^2 = g'^2 + u*h'^2) : g = g' ∧ h = h' := by
  have hn : (g-g')^2 = u*(h-h')^2 := by
    linear_combination heq + (-g*g' + g'^2 - u*h^2 + u*h*h') * hchar
  have hh : h = h' := by
    by_contra hc
    have hd : h-h' ≠ 0 := sub_ne_zero.mpr hc
    have hs : ((g-g')/(h-h'))^2 = u := by
      rw [div_pow]
      exact (div_eq_iff (pow_ne_zero 2 hd)).2 hn
    exact (hu _) hs
  have hz : (g-g')^2 = 0 := by simpa [hh] using hn
  exact ⟨sub_eq_zero.mp (pow_eq_zero hz), hh⟩

/-- For the unique decomposition, square membership is exactly zero odd part. -/
theorem square_iff_zero_odd {K : Type*} [Field K] (u f g h : K)
    (hchar : (2 : K) = 0) (hu : ∀ t : K, t^2 ≠ u)
    (hf : f = g^2 + u*h^2) : (∃ r : K, r^2 = f) ↔ h = 0 := by
  constructor
  · rintro ⟨r, hr⟩
    have hs : g^2 + u*h^2 = r^2 + u*0^2 := by
      calc
        g^2 + u*h^2 = f := hf.symm
        _ = r^2 + u*0^2 := by simpa using hr.symm
    exact (decomposition_unique u g h r 0 hchar hu hs).2
  · intro hh
    exact ⟨g, by simpa [hh] using hf.symm⟩

/-- The cyclic norm identity holds in noncommutative endomorphism rings. -/
theorem cyclic_five_norm {R : Type*} [Ring R] (t : R) (ht : t^5 = 1) :
    (1+t+t^2+t^3+t^4)^2 = 5*(1+t+t^2+t^3+t^4) := by
  have hp : (1+t+t^2+t^3+t^4)^2 - 5*(1+t+t^2+t^3+t^4) =
      (t^5-1)*(t^3+2*t^2+3*t+4) := by noncomm_ring
  rw [ht, sub_self, zero_mul] at hp
  exact sub_eq_zero.mp hp

/-- Five equals one in characteristic two; the unnormalized norm is a projector. -/
theorem cyclic_five_idempotent {R : Type*} [Ring R] (t : R)
    (ht : t^5 = 1) (hfive : (5 : R) = 1) :
    (1+t+t^2+t^3+t^4)^2 = 1+t+t^2+t^3+t^4 := by
  simpa only [hfive, one_mul] using cyclic_five_norm t ht

/-- The complementary projector can be used without assuming matrix commutativity. -/
theorem cyclic_five_primitive {R : Type*} [Ring R] (t : R)
    (ht : t^5 = 1) (hfive : (5 : R) = 1) :
    (1-(1+t+t^2+t^3+t^4))^2 = 1-(1+t+t^2+t^3+t^4) := by
  let n := 1+t+t^2+t^3+t^4
  have hn : n^2 = n := cyclic_five_idempotent t ht hfive
  change (1-n)^2 = 1-n
  have hp : (1-n)^2-(1-n) = n^2-n := by noncomm_ring
  rw [hn, sub_self] at hp
  exact sub_eq_zero.mp hp

theorem cyclic_five_orthogonal {R : Type*} [Ring R] (t : R)
    (ht : t^5 = 1) (hfive : (5 : R) = 1) :
    (1+t+t^2+t^3+t^4)*(1-(1+t+t^2+t^3+t^4)) = 0 := by
  let n := 1+t+t^2+t^3+t^4
  have hn : n^2 = n := cyclic_five_idempotent t ht hfive
  change n*(1-n) = 0
  calc
    n*(1-n) = n-n^2 := by noncomm_ring
    _ = 0 := by rw [hn, sub_self]

/-- A full-space nonzero control need not survive the primitive filter. -/
theorem primitive_at_identity {R : Type*} [Ring R] (hfive : (5 : R) = 1) :
    1-(1+(1 : R)+(1 : R)^2+(1 : R)^3+(1 : R)^4) = 0 := by
  have hn : (1+(1 : R)+(1 : R)^2+(1 : R)^3+(1 : R)^4) = 5 := by noncomm_ring
  rw [hn, hfive, sub_self]

end Square62
