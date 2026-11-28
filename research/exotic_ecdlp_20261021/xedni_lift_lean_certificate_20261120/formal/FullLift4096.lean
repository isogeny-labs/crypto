import Sieve4096

/-! The checked finite modular sieve excludes every canonical rational lift
of the declared public point through height 4096. -/

namespace XedniFullLift4096

open XedniSieve4096

theorem no_canonical_lift_height4096 (a c : ℤ) (b : ℕ)
    (ha₁ : -4096 ≤ a) (ha₂ : a ≤ 4096)
    (hb₁ : 1 ≤ b) (hb₂ : b ≤ 4096)
    (hcoprime : a.natAbs.Coprime b)
    (hx : (a - 744 * (b : ℤ)^2) % 1009 = 0)
    (_hy : (c - 244 * (b : ℤ)^3) % 1009 = 0)
    (hcurve : c^2 = curveRight a b) : False := by
  have hi : b - 1 ∈ List.range 4096 := List.mem_range.mpr (by omega)
  have hcheck : checkB b 4096 = true := by
    have h := (List.all_eq_true.mp check4096) (b - 1) hi
    simpa [fullCheck, Nat.sub_add_cancel hb₁] using h
  have hamem : a ∈ candidateList b :=
    XedniCandidateList.x_candidate_mem a b ha₁ ha₂ hx
  have hpass : rejected a b = true := by
    have h := (List.all_eq_true.mp hcheck) a hamem
    simpa [checkB, ha₁, ha₂, hcoprime] using h
  have hdisj : curveRight a b < 0 ∨
      ∃ m ∈ witnessPrimes, nonsquareMod a b m = true := by
    simpa [rejected] using hpass
  rcases hdisj with hneg | ⟨m, hmem, hnon⟩
  · nlinarith [sq_nonneg c]
  have hm : 0 < m := by
    simp [witnessPrimes] at hmem
    omega
  obtain ⟨z, hz, hroot⟩ :=
    XedniSquareModulo.square_has_modular_root c (curveRight a b) m hm hcurve
  have hreject : ∀ x < m, ¬(m : ℤ) ∣ (x : ℤ)^2 - curveRight a b := by
    simpa [nonsquareMod] using hnon
  exact (hreject z hz) (Int.dvd_iff_emod_eq_zero.mpr hroot)

#print axioms no_canonical_lift_height4096

end XedniFullLift4096
