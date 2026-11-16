import Lean.Elab.Tactic.Omega

/- Monomial coverage for (m^2, x*z, x^d, z^d), the top ideal of
   the secant/graph system when d >= 3. The polynomial and group-law
   bridges are written in SECANT_GRAPH.md, not formalized here. -/
namespace SecantGraphTop

def Covered (d i j k : Nat) : Prop :=
  d ≤ i ∨ d ≤ j ∨ (1 ≤ i ∧ 1 ≤ j) ∨ 2 ≤ k

theorem all_above (d i j k : Nat) (hd : d + 1 ≤ i + j + k) :
    Covered d i j k := by
  unfold Covered
  omega

theorem survives_at_top (d : Nat) (hd : 2 ≤ d) :
    (d - 1) + 0 + 1 = d ∧ ¬ Covered d (d - 1) 0 1 := by
  unfold Covered
  omega

theorem first_full_degree (d : Nat) (hd : 2 ≤ d) :
    (∀ i j k, i + j + k = d + 1 → Covered d i j k) ∧
    (∀ n, n < d + 1 → ∃ i j k, i + j + k = n ∧ ¬ Covered d i j k) := by
  constructor
  · intro i j k hn
    apply all_above d i j k
    omega
  · intro n hn
    by_cases h : n < d
    · refine ⟨n, 0, 0, ?_, ?_⟩
      · omega
      · unfold Covered
        omega
    · refine ⟨d - 1, 0, 1, ?_, ?_⟩
      · omega
      · unfold Covered
        omega

theorem strict_dense_reduction (d : Nat) (hd : 2 ≤ d) :
    d + 1 < 2 * d + 1 := by omega

end SecantGraphTop

#print axioms SecantGraphTop.all_above
#print axioms SecantGraphTop.survives_at_top
#print axioms SecantGraphTop.first_full_degree
#print axioms SecantGraphTop.strict_dense_reduction
