import PencilSelector

noncomputable section
namespace PencilElimination
variable {K : Type*} [Field K]

theorem elimination_identity (d l u : K) :
    d*(d+l) = (d+l*u)*(d+l-l*u)+l^2*(u^2-u) := by ring

/-- Exact projection of one Boolean selector, even for a zero weight.
    No inversion, discarded chart or probabilistic branch is used. -/
theorem one_selector_iff (d l : K) :
    (∃ u : K, (u = 0 ∨ u = 1) ∧ d+l*u = 0) ↔ d*(d+l) = 0 := by
  constructor
  · rintro ⟨u, hu, h⟩
    rcases hu with rfl | rfl
    · simp only [mul_zero, add_zero] at h
      simp [h]
    · simp only [mul_one] at h
      rw [h, mul_zero]
  · intro h
    rcases mul_eq_zero.mp h with h | h
    · exact ⟨0, Or.inl rfl, by simpa using h⟩
    · exact ⟨1, Or.inr rfl, by simpa using h⟩

theorem two_selectors_iff (g1 g2 dx dz lx lz : K) :
    (∃ u v : K, (u = 0 ∨ u = 1) ∧ (v = 0 ∨ v = 1) ∧
      g1 = 0 ∧ g2 = 0 ∧ dx+lx*u = 0 ∧ dz+lz*v = 0) ↔
      g1 = 0 ∧ g2 = 0 ∧ dx*(dx+lx) = 0 ∧ dz*(dz+lz) = 0 := by
  constructor
  · rintro ⟨u, v, hu, hv, hg1, hg2, hx, hz⟩
    exact ⟨hg1, hg2, (one_selector_iff dx lx).mp ⟨u, hu, hx⟩,
      (one_selector_iff dz lz).mp ⟨v, hv, hz⟩⟩
  · rintro ⟨hg1, hg2, hx, hz⟩
    obtain ⟨u, hu, hx⟩ := (one_selector_iff dx lx).mpr hx
    obtain ⟨v, hv, hz⟩ := (one_selector_iff dz lz).mpr hz
    exact ⟨u, v, hu, hv, hg1, hg2, hx, hz⟩

theorem boolean_cons {k : ℕ} (u : K) (us : Fin k → K) :
    PencilSelector.Boolean (Fin.cons u us) ↔
      (u = 0 ∨ u = 1) ∧ PencilSelector.Boolean us := by
  simp [PencilSelector.Boolean, Fin.forall_fin_succ]

theorem graph_cons {k : ℕ} (a b c t0 w0 u x : K) (w us : Fin k → K) :
    PencilSelector.graph a b c t0 (Fin.cons w0 w) (Fin.cons u us) x =
      PencilSelector.graph a b c t0 w us x+w0*u := by
  simp [PencilSelector.graph, PencilSelector.offset, Fin.sum_univ_succ]
  ring

end PencilElimination

#print axioms PencilElimination.one_selector_iff
#print axioms PencilElimination.elimination_identity
#print axioms PencilElimination.two_selectors_iff
#print axioms PencilElimination.graph_cons
