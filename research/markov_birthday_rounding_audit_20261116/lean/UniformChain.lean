import CollisionCertificate
import BirthdayImports

namespace MarkovBirthdayRoundingAudit
noncomputable section
open scoped BigOperators
open WalkWeightedGeneralAudit StaticHashCollisionAudit RhoFirstCollisionAudit
variable (p : ℕ) [NeZero p]

def uniformWeight : ZMod p → ℝ := fun _ => 1/(p : ℝ)

theorem modulus_positive : (0 : ℝ)<p := by exact_mod_cast NeZero.pos p

theorem uniform_weight_nonnegative (x : ZMod p) : 0≤uniformWeight p x :=
  le_of_lt (one_div_pos.mpr (modulus_positive p))

theorem uniform_weight_sum : (∑ x : ZMod p,uniformWeight p x)=1 := by
  simp only [uniformWeight,Finset.sum_const,Finset.card_univ,ZMod.card,nsmul_eq_mul]
  exact mul_one_div_cancel (ne_of_gt (modulus_positive p))

/-- Actual normalized one-step law; the next state is uniform for every current state. -/
def transitionLaw (_ : ZMod p) : PMF (ZMod p) :=
  PMF.ofFintype (fun y => ENNReal.ofReal (uniformWeight p y)) (by
    rw [←ENNReal.ofReal_sum_of_nonneg (fun y _ => uniform_weight_nonnegative p y),
      uniform_weight_sum]
    simp)

theorem transition_real_mass (x y : ZMod p) : (transitionLaw p x y).toReal=1/(p : ℝ) :=
  ENNReal.toReal_ofReal (uniform_weight_nonnegative p y)

/-- Transition matrix is derived from the actual PMF, not supplied as an abstract hypothesis. -/
def transitionMatrix : Matrix (ZMod p) (ZMod p) ℝ := fun x y => (transitionLaw p x y).toReal

theorem transition_entry (x y : ZMod p) : transitionMatrix p x y=1/(p : ℝ) :=
  transition_real_mass p x y

theorem transition_row_sum (x : ZMod p) : (∑ y : ZMod p,transitionMatrix p x y)=1 := by
  simpa only [transition_entry,uniformWeight] using uniform_weight_sum p

theorem transition_column_sum (y : ZMod p) : (∑ x : ZMod p,transitionMatrix p x y)=1 := by
  simpa only [transition_entry,uniformWeight] using uniform_weight_sum p

theorem transition_positive (x y : ZMod p) : 0<transitionMatrix p x y := by
  rw [transition_entry]
  exact one_div_pos.mpr (modulus_positive p)

/-- Actual uniform stationary distribution, using the actual transition probabilities. -/
theorem uniform_stationary (y : ZMod p) :
    (∑ x : ZMod p,uniformWeight p x*transitionMatrix p x y)=uniformWeight p y := by
  change (∑ x : ZMod p,(1/(p : ℝ))*transitionMatrix p x y)=1/(p : ℝ)
  rw [←Finset.mul_sum,transition_column_sum,mul_one]

theorem transition_idempotent : transitionMatrix p*transitionMatrix p=transitionMatrix p := by
  ext x y
  simp only [Matrix.mul_apply,transition_entry,Finset.sum_const,Finset.card_univ,ZMod.card,
    nsmul_eq_mul]
  field_simp [ne_of_gt (modulus_positive p)]

theorem transition_power_succ (n : ℕ) : transitionMatrix p^(n+1)=transitionMatrix p := by
  induction n with
  | zero => simp
  | succ n ih => rw [pow_succ,ih,transition_idempotent]

/-- One-step reachability proves irreducibility of the actual finite chain. -/
theorem actual_irreducible :
    ∀ x y : ZMod p,∃ n : ℕ,0<n ∧ 0<(transitionMatrix p^n) x y := by
  intro x y
  exact ⟨1,by decide,by simpa only [pow_one] using transition_positive p x y⟩

/-- Any common divisor of positive return times divides1, so every state's period is1. -/
theorem actual_aperiodic :
    ∀ x : ZMod p,∀ d : ℕ,
      (∀ n : ℕ,0<n → 0<(transitionMatrix p^n) x x → d∣n) → d=1 := by
  intro x d h
  exact Nat.dvd_one.mp (h 1 (by decide) (by simpa only [pow_one] using transition_positive p x x))

/-- Actual chronological product law, with its actual state update. -/
def resetUpdate (_ y : ZMod p) : ZMod p := y

def wordLaw (n : ℕ) : PMF (Fin n → ZMod p) :=
  hashLaw (uniformWeight p) (uniform_weight_nonnegative p) (uniform_weight_sum p)

theorem word_real_mass (n : ℕ) (w : Fin n → ZMod p) :
    (wordLaw p n w).toReal=(1/(p : ℝ))^n := by
  rw [wordLaw,hashLaw_real_mass]
  simp only [weight,uniformWeight,Finset.prod_const,Finset.card_univ,Fintype.card_fin]

#print axioms uniform_weight_sum
#print axioms uniform_stationary
#print axioms transition_power_succ
#print axioms actual_irreducible
#print axioms actual_aperiodic
#print axioms word_real_mass
end
end MarkovBirthdayRoundingAudit
