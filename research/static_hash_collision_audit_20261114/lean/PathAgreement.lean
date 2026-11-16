import Cylinder

namespace StaticHashCollisionAudit
noncomputable section
variable {G I : Type*}

/-- Deterministic state update, using one static label per state. -/
def staticStep (U : G → I → G) (h : G → I) (x : G) : G := U x (h x)

/-- A chronological finite label word defines actual successive states.
After the word ends the auxiliary natural-indexed path stays at its last state. -/
def wordPath (U : G → I → G) (x : G) {t : ℕ} (w : Fin t → I) : ℕ → G
  | 0 => x
  | n + 1 => if hn : n < t then U (wordPath U x w n) (w ⟨n, hn⟩)
      else wordPath U x w n

@[simp] theorem wordPath_zero (U : G → I → G) (x : G) {t : ℕ} (w : Fin t → I) :
    wordPath U x w 0 = x := rfl

theorem wordPath_succ (U : G → I → G) (x : G) {t : ℕ} (w : Fin t → I)
    (n : ℕ) (hn : n < t) :
    wordPath U x w (n + 1) = U (wordPath U x w n) (w ⟨n, hn⟩) := by
  simp only [wordPath, dite_eq_left hn]

def observations (U : G → I → G) (h : G → I) (x : G) (t : ℕ) : Fin t → I :=
  fun i => h ((staticStep U h)^[i.val] x)

theorem observed_wordPath (U : G → I → G) (h : G → I) (x : G) (t n : ℕ)
    (hn : n ≤ t) :
    wordPath U x (observations U h x t) n = (staticStep U h)^[n] x := by
  induction n with
  | zero => rfl
  | succ n ih =>
    have hnt : n < t := by omega
    rw [wordPath_succ U x _ n hnt, ih (by omega), Function.iterate_succ_apply']
    rfl

theorem agreement_wordPath (U : G → I → G) (h : G → I) (x : G)
    {t : ℕ} (w : Fin t → I)
    (ha : ∀ i : Fin t, h (wordPath U x w i.val) = w i)
    (n : ℕ) (hn : n ≤ t) : wordPath U x w n = (staticStep U h)^[n] x := by
  induction n with
  | zero => rfl
  | succ n ih =>
    have hnt : n < t := by omega
    rw [wordPath_succ U x w n hnt, ← ha ⟨n, hnt⟩, ih (by omega),
      Function.iterate_succ_apply']
    rfl

theorem observations_eq_iff (U : G → I → G) (h : G → I) (x : G)
    {t : ℕ} (w : Fin t → I) :
    observations U h x t = w ↔ ∀ i : Fin t, h (wordPath U x w i.val) = w i := by
  constructor
  · intro he i
    rw [← he, observed_wordPath U h x t i.val (Nat.le_of_lt i.isLt)]
    rfl
  · intro ha
    funext i
    rw [← ha i, agreement_wordPath U h x w ha i.val (Nat.le_of_lt i.isLt)]
    rfl

def SelfAvoiding (U : G → I → G) (x : G) {t : ℕ} (w : Fin t → I) : Prop :=
  Function.Injective (fun i : Fin (t + 1) => wordPath U x w i.val)

def StaticSurvives (U : G → I → G) (h : G → I) (x : G) (t : ℕ) : Prop :=
  Function.Injective (fun i : Fin (t + 1) => (staticStep U h)^[i.val] x)

instance selfAvoidingDecidable [DecidableEq G] (U : G → I → G) (x : G)
    {t : ℕ} (w : Fin t → I) : Decidable (SelfAvoiding U x w) :=
  inferInstanceAs (Decidable (Function.Injective
    (fun i : Fin (t + 1) => wordPath U x w i.val)))

instance staticSurvivesDecidable [DecidableEq G] (U : G → I → G) (h : G → I)
    (x : G) (t : ℕ) : Decidable (StaticSurvives U h x t) :=
  inferInstanceAs (Decidable (Function.Injective
    (fun i : Fin (t + 1) => (staticStep U h)^[i.val] x)))

theorem selfAvoiding_queries_injective (U : G → I → G) (x : G) {t : ℕ}
    (w : Fin t → I) (hw : SelfAvoiding U x w) :
    Function.Injective (fun i : Fin t => wordPath U x w i.val) := by
  intro i j he
  have hh : i.castSucc = j.castSucc := hw he
  exact Fin.castSucc_injective t hh

theorem observed_selfAvoiding_iff (U : G → I → G) (h : G → I) (x : G) (t : ℕ) :
    SelfAvoiding U x (observations U h x t) ↔ StaticSurvives U h x t := by
  unfold SelfAvoiding StaticSurvives
  have he : (fun i : Fin (t + 1) => wordPath U x (observations U h x t) i.val) =
      (fun i : Fin (t + 1) => (staticStep U h)^[i.val] x) := by
    funext i
    exact observed_wordPath U h x t i.val (by have := i.isLt; omega)
  rw [he]

#print axioms observations_eq_iff
#print axioms observed_selfAvoiding_iff
end
end StaticHashCollisionAudit
