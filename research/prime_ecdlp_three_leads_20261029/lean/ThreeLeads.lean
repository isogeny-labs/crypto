import Mathlib.Algebra.Polynomial.Roots
import Mathlib.LinearAlgebra.Span.Basic
import Mathlib.Tactic

/-! Structural certificates, not a theorem of sub-square-root ECDLP.
The scalar-to-point map and the natural target relation are explicit premises.
-/
namespace ThreeLeads
open Polynomial
open scoped BigOperators

section Tower
variable {K : Type*} [Field K]

/-- Distinct-root factor-base membership: no splitting hypothesis about X^d-1
is needed; the proposed summands themselves are the finite set s. -/
theorem tower_membership (s : Finset K) (d : ℕ) :
    (∏ a ∈ s, (X - C a)) ∣ (X ^ d - 1 : K[X]) ↔
      ∀ a ∈ s, a ^ d = 1 := by
  classical
  constructor
  · intro h a ha
    have hfactor : (X - C a : K[X]) ∣ ∏ b ∈ s, (X - C b) :=
      Finset.dvd_prod_of_mem (fun b : K => (X - C b : K[X])) ha
    have hz := (dvd_iff_isRoot).mp (hfactor.trans h)
    simpa [Polynomial.IsRoot, sub_eq_zero] using hz
  · intro h
    apply Finset.prod_dvd_of_coprime
    · intro a ha b hb hab
      exact Polynomial.pairwise_coprime_X_sub_C (Function.injective_id) hab
    · intro a ha
      apply dvd_iff_isRoot.mpr
      simp [Polynomial.IsRoot, h a ha]

/-- The repeated-squaring part of the tower is valid in any commutative
ring, including the quotient by the monic root polynomial. -/
theorem squaring_tower {R : Type*} [CommMonoid R] (x : R) (r : ℕ → R)
    (h0 : r 0 = x) (hs : ∀ j, r (j+1) = r j ^ 2) :
    ∀ t, r t = x ^ (2 ^ t) := by
  intro t
  induction t with
  | zero => simpa using h0
  | succ t ih => rw [hs, ih, ← pow_mul, pow_succ]
end Tower

section Recovery
variable {K V I J : Type*} [Field K] [AddCommGroup V] [Module K V]
    [Fintype I] [Fintype J]

/-- A table hit yields the log only because its point-label certificates
and an injective scalar action are supplied. This includes signed labels. -/
theorem table_recovery (encode : K →ₗ[K] V) (hinj : Function.Injective encode)
    (labels : I → K) (k : K)
    (hit : (∑ i, encode (labels i)) = encode k) : (∑ i, labels i) = k := by
  apply hinj
  simpa only [map_sum] using hit

def rowEval (v z : I → K) : K := ∑ i, v i * z i

/-- Replaying a certified row combination gives its right hand side. -/
theorem row_combination (M : J → I → K) (z : I → K) (b weights : J → K)
    (rows : ∀ j, rowEval (M j) z = b j) :
    rowEval (fun i => ∑ j, weights j * M j i) z = ∑ j, weights j * b j := by
  simp only [rowEval, Finset.sum_mul]
  rw [Finset.sum_comm]
  apply Finset.sum_congr rfl
  intro j _
  rw [← rows j]
  simp only [rowEval, Finset.mul_sum, mul_assoc]

/-- The missing Hecke interface is the natural prescribed-target row.
Rank completion alone does not supply `attachment`. -/
theorem target_recovery (M : J → I → K) (z : I → K) (b weights : J → K)
    (rows : ∀ j, rowEval (M j) z = b j) (v : I → K)
    (certificate : v = fun i => ∑ j, weights j * M j i)
    (k α β : K) (nonzero : β ≠ 0)
    (attachment : β*k + α = rowEval v z) :
    k = ((∑ j, weights j * b j) - α) / β := by
  have hr := row_combination M z b weights rows
  rw [← certificate] at hr
  apply (eq_div_iff nonzero).mpr
  calc
    k*β = β*k := mul_comm _ _
    _ = (∑ j, weights j * b j) - α := by rw [← hr, ← attachment]; ring

omit [Fintype J] in
/-- A nonzero kernel perturbation preserves every recorded equation.
This is a concrete certificate of unresolved individual coordinates. -/
theorem kernel_ambiguity (M : J → I → K) (z h : I → K)
    (kernel : ∀ j, rowEval (M j) h = 0) (i : I) (hi : h i ≠ 0) :
    (∀ j, rowEval (M j) (z+h) = rowEval (M j) z) ∧ (z+h) i ≠ z i := by
  constructor
  · intro j
    simp only [rowEval, Pi.add_apply, mul_add, Finset.sum_add_distrib]
    simpa only [rowEval, add_zero] using congrArg (fun a => rowEval (M j) z + a) (kernel j)
  · simpa only [Pi.add_apply, add_ne_left] using hi
end Recovery

/-- A bounded shift router can cover at most one translate of the known-log
set per shift. No randomness, independence or heuristic density is assumed. -/
theorem shift_coverage_bound {G : Type*} [AddCommGroup G] [DecidableEq G]
    (known shifts : Finset G) :
    (shifts.biUnion (fun a => known.image (fun x => x-a))).card ≤
      shifts.card * known.card := by
  calc
    _ ≤ ∑ a ∈ shifts, (known.image (fun x => x-a)).card := Finset.card_biUnion_le
    _ ≤ ∑ _a ∈ shifts, known.card := by
      apply Finset.sum_le_sum
      intro a _
      exact Finset.card_image_le
    _ = shifts.card * known.card := by simp [Nat.mul_comm]

/-- The reported k/(2k-1) table-family model approaches one half from
above. This is arithmetic about that model, not a lower bound on ECDLP. -/
theorem table_model_exponent (k : ℕ) (hk : 1 ≤ k) :
    (1/2 : ℚ) < (k : ℚ) / (2*(k : ℚ)-1) := by
  have hkq : (1 : ℚ) ≤ k := by exact_mod_cast hk
  apply (lt_div_iff₀ (by linarith : (0 : ℚ) < 2*(k : ℚ)-1)).mpr
  linarith

#print axioms tower_membership
#print axioms squaring_tower
#print axioms table_recovery
#print axioms row_combination
#print axioms target_recovery
#print axioms kernel_ambiguity
#print axioms table_model_exponent
#print axioms shift_coverage_bound
end ThreeLeads
