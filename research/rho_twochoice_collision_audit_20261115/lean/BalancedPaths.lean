import BinaryMaximal

namespace RhoTwoChoiceCollisionAudit
noncomputable section
open scoped BigOperators
open StaticHashCollisionAudit

/-- Actual number of label-one steps in each prefix. -/
def onesPrefix : {n : ℕ} → (Fin n → Fin 2) → Fin (n + 1) → ℕ
  | 0, _ => fun _ => 0
  | _n + 1, w => Fin.cons 0 (fun j => (w 0).val + onesPrefix (Fin.tail w) j)

theorem sign_val (i : Fin 2) : sign i = 2 * (i.val : ℤ) - 1 := by
  fin_cases i <;> norm_num [sign]

theorem prefixState_shift {n : ℕ} (s : ℤ) (w : Fin n → Fin 2) (j : Fin (n + 1)) :
    prefixState s w j = s + prefixState 0 w j := by
  induction n generalizing s with
  | zero => simp [prefixState]
  | succ n ih =>
    refine Fin.cases ?_ (fun k => ?_) j
    · simp [prefixState]
    · change prefixState (s + sign (w 0)) (Fin.tail w) k =
        s + prefixState (0 + sign (w 0)) (Fin.tail w) k
      simp only [zero_add]
      rw [ih, ih (sign (w 0))]
      ring

theorem prefixState_count {n : ℕ} (w : Fin n → Fin 2) (j : Fin (n + 1)) :
    prefixState 0 w j = 2 * (onesPrefix w j : ℤ) - j.val := by
  induction n with
  | zero => simp [prefixState, onesPrefix, Fin.fin_one_eq_zero]
  | succ n ih =>
    refine Fin.cases ?_ (fun k => ?_) j
    · simp [prefixState, onesPrefix]
    · change prefixState (0 + sign (w 0)) (Fin.tail w) k =
        2 * (((w 0).val + onesPrefix (Fin.tail w) k : ℕ) : ℤ) - (k.val + 1 : ℕ)
      rw [prefixState_shift, ih, sign_val]
      push_cast
      ring

theorem wordPath_cons {G I : Type*} (U : G → I → G) (x : G) {n : ℕ}
    (i : I) (w : Fin n → I) (j : ℕ) (hj : j ≤ n) :
    wordPath U x (Fin.cons i w) (j + 1) = wordPath U (U x i) w j := by
  induction j with
  | zero => simp [wordPath]
  | succ j ih =>
    rw [wordPath_succ U x _ (j + 1) (by omega),
      wordPath_succ U (U x i) w j (by omega), ih (by omega)]
    rfl

def twoUpdate {p : ℕ} (a δ : ZMod p) (x : ZMod p) (i : Fin 2) : ZMod p :=
  x + a + (i.val : ZMod p) * δ

theorem actual_path_count {p n : ℕ} (a δ x : ZMod p) (w : Fin n → Fin 2)
    (j : Fin (n + 1)) :
    wordPath (twoUpdate a δ) x w j.val =
      x + (j.val : ZMod p) * a + (onesPrefix w j : ZMod p) * δ := by
  induction n generalizing x with
  | zero => simp [onesPrefix, Fin.fin_one_eq_zero]
  | succ n ih =>
    refine Fin.cases ?_ (fun k => ?_) j
    · simp [onesPrefix]
    · change wordPath (twoUpdate a δ) x w (k.val + 1) =
        x + ((k.val + 1 : ℕ) : ZMod p) * a +
          (((w 0).val + onesPrefix (Fin.tail w) k : ℕ) : ZMod p) * δ
      have he : w = Fin.cons (w 0) (Fin.tail w) := (Fin.cons_self_tail w).symm
      conv_lhs => rw [he]
      rw [wordPath_cons _ _ _ _ _ (Nat.le_of_lt_succ k.isLt), ih]
      simp only [twoUpdate, Nat.cast_add, Nat.cast_one]
      ring

/-- Finite list of all short balanced-count relations. -/
def shortRelation {p : ℕ} (T W : ℕ) (a δ : ZMod p) : Prop :=
  ∃ d : Fin T, ∃ u : Fin (2 * W + 1),
    ((d.val + 1 : ℕ) : ZMod p) * a +
      ((((d.val + 1) / 2 : ℕ) : ZMod p) + (u.val : ZMod p) - (W : ZMod p)) * δ = 0

instance shortRelationDecidable {p : ℕ} (T W : ℕ) (a δ : ZMod p) :
    Decidable (shortRelation T W a δ) :=
  inferInstanceAs (Decidable (∃ d : Fin T, ∃ u : Fin (2 * W + 1),
    ((d.val + 1 : ℕ) : ZMod p) * a +
      ((((d.val + 1) / 2 : ℕ) : ZMod p) + (u.val : ZMod p) - (W : ZMod p)) * δ = 0))

theorem balanced_collision_relation {p T W : ℕ} (a δ : ZMod p)
    (w : Fin T → Fin 2) (hw : ¬Hit W 0 w)
    (hc : ¬SelfAvoiding (twoUpdate a δ) 0 w) : shortRelation T W a δ := by
  classical
  obtain ⟨i, j, he, hne⟩ := Function.not_injective_iff.mp hc
  have hneq : i.val ≠ j.val := fun h => hne (Fin.ext h)
  wlog hij : i.val < j.val generalizing i j
  · exact this j i he.symm hne.symm hneq.symm (by omega)
  have hi := no_hit_prefix_bounds W w hw i
  have hj := no_hit_prefix_bounds W w hw j
  rw [prefixState_count] at hi hj
  let d := j.val - i.val
  let k : ℤ := (onesPrefix w j : ℤ) - (onesPrefix w i : ℤ)
  let u : ℤ := k + W - (d / 2 : ℕ)
  have hd : 0 < d ∧ d ≤ T := by dsimp [d]; have := j.isLt; omega
  have hdiv := Nat.mod_add_div d 2
  have hmod := Nat.mod_lt d (by decide : 0 < 2)
  have hu : 0 ≤ u ∧ u ≤ 2 * (W : ℤ) := by dsimp [u, k, d]; omega
  let uf : Fin (2 * W + 1) := ⟨u.toNat, by omega⟩
  refine ⟨⟨d - 1, by omega⟩, uf, ?_⟩
  have hd1 : d - 1 + 1 = d := by omega
  have hk : ((d / 2 : ℕ) : ℤ) + (uf.val : ℤ) - W = k := by
    dsimp [uf]
    rw [Int.toNat_of_nonneg hu.1]
    dsimp [u]
    ring
  have hcast : (((d / 2 : ℕ) : ZMod p) + (uf.val : ZMod p) - (W : ZMod p)) =
      (onesPrefix w j : ZMod p) - (onesPrefix w i : ZMod p) := by
    have h := congrArg (fun z : ℤ => (z : ZMod p)) hk
    simpa only [Int.cast_add, Int.cast_sub, Int.cast_natCast, k] using h
  change ((d - 1 + 1 : ℕ) : ZMod p) * a +
    ((((d - 1 + 1) / 2 : ℕ) : ZMod p) + (uf.val : ZMod p) - (W : ZMod p)) * δ = 0
  rw [hd1, hcast]
  have hdcast : (d : ZMod p) = (j.val : ZMod p) - (i.val : ZMod p) :=
    Nat.cast_sub (Nat.le_of_lt hij)
  rw [hdcast]
  change wordPath (twoUpdate a δ) 0 w i.val = wordPath (twoUpdate a δ) 0 w j.val at he
  rw [actual_path_count, actual_path_count] at he
  linear_combination he.symm

theorem balanced_no_relation_survives {p T W : ℕ} (a δ : ZMod p)
    (w : Fin T → Fin 2) (hw : ¬Hit W 0 w) (hs : ¬shortRelation T W a δ) :
    SelfAvoiding (twoUpdate a δ) 0 w := by
  by_contra hc
  exact hs (balanced_collision_relation a δ w hw hc)

#print axioms balanced_no_relation_survives
end
end RhoTwoChoiceCollisionAudit
