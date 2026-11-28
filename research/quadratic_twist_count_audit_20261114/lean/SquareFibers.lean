import EquationCount

namespace QuadraticTwistCountAudit
noncomputable section
variable {K : Type*} [Field K]

def squareRoots (a : K) := {y : K // y^2=a}
def rootCard (a : K) : ℕ := Nat.card (squareRoots a)

instance squareRootsFintype [Fintype K] [DecidableEq K] (a : K) : Fintype (squareRoots a) :=
  inferInstanceAs (Fintype {y : K // y^2=a})

theorem root_card_zero : rootCard (0:K)=1 := by
  apply Nat.card_eq_one_iff_exists.mpr
  refine ⟨⟨0,by simp⟩,fun y => Subtype.ext ?_⟩
  exact (sq_eq_zero_iff).mp y.property

theorem root_card_nonsquare (a : K) (ha : ¬IsSquare a) : rootCard a=0 := by
  let : IsEmpty (squareRoots a) := ⟨fun y => ha ⟨y.val,by simpa only [pow_two] using y.property.symm⟩⟩
  exact Nat.card_of_isEmpty

theorem root_card_square [DecidableEq K] (h2 : (2:K)≠0) (a : K)
    (ha : a≠0) (hs : IsSquare a) : rootCard a=2 := by
  obtain ⟨y,hy⟩ := hs
  have hy0 : y≠0 := by intro h; apply ha; rw [hy,h]; simp
  have hyn : y≠ -y := by
    intro h
    have hz : 2*y=0 := by linear_combination h
    exact (mul_ne_zero h2 hy0) hz
  let s : Finset K := {y,-y}
  have he : {z : K | z^2=a}=(↑s:Set K) := by
    ext z
    simp only [Finset.mem_coe,Finset.mem_insert,Finset.mem_singleton,Set.mem_ofPred_eq,s]
    rw [hy,←pow_two]
    exact sq_eq_sq_iff_eq_or_eq_neg
  calc
    rootCard a=Nat.card ↥s := Nat.card_congr (Set.equivOfEq he)
    _ = 2 := by
      rw [Nat.card_eq_fintype_card,Fintype.card_coe]
      exact Finset.card_pair hyn

/-- Exact square-root fiber complement for an actual nonsquare scaling.
No elliptic point-count or trace theorem is assumed. -/
theorem scaled_root_card_sum [Fintype K] [DecidableEq K]
    (hF : ringChar K≠2) (d : K) (hd : ¬IsSquare d) (a : K) :
    rootCard a+rootCard (d^3*a)=2 := by
  have h2 : (2:K)≠0 := by
    intro h
    apply Ring.neg_one_ne_one_of_char_ne_two hF
    linear_combination -h
  have hd0 : d≠0 := by
    intro h
    apply hd
    rw [h]
    exact ⟨0,by simp⟩
  by_cases ha : a=0
  · rw [ha,mul_zero,root_card_zero]
  have hda : d^3*a≠0 := mul_ne_zero (pow_ne_zero 3 hd0) ha
  have hdPow : d^(Fintype.card K/2)= -1 :=
    (FiniteField.pow_dichotomy hF hd0).resolve_left (fun h => hd ((FiniteField.isSquare_iff hF hd0).mpr h))
  have hp : (d^3*a)^(Fintype.card K/2)= -(a^(Fintype.card K/2)) := by
    rw [mul_pow,pow_right_comm,hdPow]
    ring
  by_cases hs : IsSquare a
  · have haPow := (FiniteField.isSquare_iff hF ha).mp hs
    have hns : ¬IsSquare (d^3*a) := by
      intro h
      have hx := (FiniteField.isSquare_iff hF hda).mp h
      rw [hp,haPow] at hx
      exact Ring.neg_one_ne_one_of_char_ne_two hF hx
    rw [root_card_square h2 a ha hs,root_card_nonsquare _ hns]
  · have haPow : a^(Fintype.card K/2)= -1 :=
      (FiniteField.pow_dichotomy hF ha).resolve_left (fun h => hs ((FiniteField.isSquare_iff hF ha).mpr h))
    have hsq : IsSquare (d^3*a) := (FiniteField.isSquare_iff hF hda).mpr (by rw [hp,haPow]; ring)
    rw [root_card_nonsquare a hs,root_card_square h2 _ hda hsq]

#print axioms root_card_square
#print axioms scaled_root_card_sum
end
end QuadraticTwistCountAudit
