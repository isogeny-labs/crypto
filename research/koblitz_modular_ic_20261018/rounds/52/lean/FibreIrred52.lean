import Closure52
import Mathlib.Algebra.Polynomial.Monic
import Mathlib.Data.Nat.GCD.Basic

namespace FibreIrred52

/-- Convert the two good-fiber degree restrictions into actual polynomial
irreducibility. The specialization/DVR argument producing those restrictions
and the native factor lists remain external inputs. -/
theorem irreducible_of_two_patterns {K : Type*} [Field K]
    (p : Polynomial K) (hp : p.Monic) (hdeg : p.natDegree = 16)
    (hpatterns : ∀ f : Polynomial K, f.Monic → f ∣ p →
      (f.natDegree = 0 ∨ f.natDegree = 8 ∨ f.natDegree = 16) ∧
      (f.natDegree = 0 ∨ f.natDegree = 2 ∨ f.natDegree = 14 ∨ f.natDegree = 16)) :
    Irreducible p := by
  apply hp.irreducible_iff_natDegree.mpr
  constructor
  · intro h
    simp [h] at hdeg
  · intro f g hf hg hfg
    have hdiv : f ∣ p := ⟨g, hfg.symm⟩
    obtain ⟨h88, h214⟩ := hpatterns f hf hdiv
    rcases Closure52.two_patterns_exclude_proper_degree f.natDegree h88 h214 with h0 | h16
    · exact Or.inl h0
    · have hsum : f.natDegree + g.natDegree = 16 := by
        rw [← Polynomial.natDegree_mul hf.ne_zero hg.ne_zero, hfg, hdeg]
      have hs : 16 + g.natDegree = 16 + 0 := by simpa [h16] using hsum
      exact Or.inr (Nat.add_left_cancel hs)

/-- A smooth degree-dividing-10 place rules out a constant extension of
prime degree7. Translating places and constants into divisibility is external. -/
theorem common_degree_divisor_one (d : ℕ) (h7 : d ∣ 7) (h10 : d ∣ 10) : d = 1 := by
  have h := Nat.dvd_gcd h7 h10
  have hg : Nat.gcd 7 10 = 1 := by decide
  rw [hg] at h
  exact Nat.eq_one_of_dvd_one h

end FibreIrred52
