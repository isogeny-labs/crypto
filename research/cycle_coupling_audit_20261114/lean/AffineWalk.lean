module

public import CycleMixing
public import Mathlib.Algebra.GroupWithZero.Units.Equiv

@[expose] public section

namespace CycleCouplingAudit

def twoStep {n : ℕ} (a b : ZMod n) (coin : Bool) (x : ZMod n) : ZMod n :=
  if coin then x+b else x+a

theorem walk_translate {n : ℕ} (t : ℕ) (f : ZMod n → ℝ) (u z : ZMod n) :
    average walkStep t f (u+z) = average walkStep t (fun w => f (u+w)) z := by
  induction t generalizing z with
  | zero => rfl
  | succ t ih =>
    change (average walkStep t f (u+z) + average walkStep t f ((u+z)+1)) / 2 = _
    rw [add_assoc, ih, ih]
    rfl

theorem two_average {n : ℕ} (a b : ZMod n) (t : ℕ) (f : ZMod n → ℝ) (x : ZMod n) :
    average (twoStep a b) t f x =
      average walkStep t (fun z => f (x + (t : ZMod n)*a + (b-a)*z)) 0 := by
  induction t generalizing x with
  | zero => simp [average]
  | succ t ih =>
    let g : ZMod n → ℝ := fun z => f (x + ((t+1 : ℕ) : ZMod n)*a + (b-a)*z)
    change (average (twoStep a b) t f (x+a) + average (twoStep a b) t f (x+b)) / 2 =
      (average walkStep t g 0 + average walkStep t g (0+1)) / 2
    rw [zero_add]
    have h₀ : average (twoStep a b) t f (x+a) = average walkStep t g 0 := by
      rw [ih]
      congr 1
      funext z
      dsimp [g]
      congr 1
      push_cast
      ring
    have h₁ : average (twoStep a b) t f (x+b) = average walkStep t g 1 := by
      rw [ih]
      have hs := walk_translate t g (1 : ZMod n) 0
      simp only [add_zero] at hs
      rw [hs]
      congr 1
      funext z
      dsimp [g]
      congr 1
      push_cast
      ring
    rw [h₀,h₁]

noncomputable def twoProbability {p : ℕ} [NeZero p] (a b : ZMod p) (t : ℕ)
    (x : ZMod p) (A : Finset (ZMod p)) : ℝ :=
  expectation (fairLaw (twoStep a b) t x) (eventIndicator A)

noncomputable def affineEquiv {p : ℕ} [Fact p.Prime] (a b x : ZMod p) (t : ℕ)
    (hab : a ≠ b) : ZMod p ≃ ZMod p :=
  (Equiv.mulLeft₀ (b-a) (sub_ne_zero.mpr hab.symm)).trans
    (Equiv.addLeft (x + (t : ZMod p)*a))

theorem two_support_mixing {p : ℕ} [Fact p.Prime] (a b : ZMod p) (hab : a ≠ b)
    {t : ℕ} (ht : 0 < t) (x : ZMod p) (A : Finset (ZMod p)) :
    |twoProbability a b t x A - (A.card : ℝ)/p| ≤ (p : ℝ)^2/(4*t) := by
  let e := affineEquiv a b x t hab
  let B := A.map e.symm.toEmbedding
  have hc : B.card = A.card := Finset.card_map _
  have hi : (fun z => eventIndicator A (e z)) = eventIndicator B := by
    funext z
    simp [eventIndicator, B]
  have he : twoProbability a b t x A = walkProbability t 0 B := by
    rw [twoProbability, actual_expectation, two_average, walkProbability_eq]
    change average walkStep t (fun z => eventIndicator A (e z)) 0 = _
    rw [hi]
  rw [he, ← hc]
  exact cyclic_mixing_bound (Fact.out : p.Prime).two_le ht 0 B

def deterministicStep {n : ℕ} (_ : Bool) (x : ZMod n) : ZMod n := x+1

theorem deterministic_average {n : ℕ} (t : ℕ) (f : ZMod n → ℝ) (x : ZMod n) :
    average deterministicStep t f x = f (x + (t : ZMod n)) := by
  induction t generalizing x with
  | zero => simp [average]
  | succ t ih =>
    simp only [average, deterministicStep, ih]
    have he : (x+1)+(t : ZMod n) = x+((t+1 : ℕ) : ZMod n) := by push_cast; ring
    rw [he]
    ring

theorem deterministic_nonmixing (t : ℕ) :
    |expectation (fairLaw deterministicStep t (0 : ZMod 2))
      (eventIndicator {(t : ZMod 2)}) - (1 : ℝ)/2| = 1/2 := by
  rw [actual_expectation, deterministic_average]
  norm_num [eventIndicator]

theorem time_zero_counter :
    ¬ |walkProbability 0 (0 : ZMod 2) {0} - (1 : ℝ)/2| ≤ (2 : ℝ)^2 / (4*0) := by
  norm_num [walkProbability_eq, average, eventIndicator]

end CycleCouplingAudit
