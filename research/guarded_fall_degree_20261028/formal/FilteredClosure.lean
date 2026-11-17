import Mathlib.Algebra.MvPolynomial.NoZeroDivisors
import Mathlib.RingTheory.Ideal.Span

/-!+# Ordinary bounded-degree closure

`Derivable F d p` is membership in the smallest coefficient-linear space
containing inputs of degree at most `d` and closed under multiplication
whenever the actual product has total degree at most `d`. In particular,
degree falls are allowed; this is not just the Macaulay row space.

The last-fall convention is Huang--Kosters--Yang--Yeo, Definition 2.2.
Zero has natural total degree zero. This makes no difference to membership.
-/
namespace FilteredClosure

open MvPolynomial

variable {K σ : Type*} [Field K]

/-- Finite derivation trees for ordinary filtered closure. -/
inductive Derivable (F : Set (MvPolynomial σ K)) (d : ℕ) : MvPolynomial σ K → Prop
  | zero : Derivable F d 0
  | input {p} : p ∈ F → p.totalDegree ≤ d → Derivable F d p
  | add {p q} : Derivable F d p → Derivable F d q → Derivable F d (p + q)
  | smul (a : K) {p} : Derivable F d p → Derivable F d (a • p)
  | mul (q) {p} : Derivable F d p → (q * p).totalDegree ≤ d →
      Derivable F d (q * p)

variable {F : Set (MvPolynomial σ K)} {d e : ℕ} {p : MvPolynomial σ K}

theorem neg (h : Derivable F d p) : Derivable F d (-p) := by
  simpa using Derivable.smul (-1 : K) h

theorem sub {q : MvPolynomial σ K} (hp : Derivable F d p) (hq : Derivable F d q) :
    Derivable F d (p-q) := by
  simpa only [sub_eq_add_neg] using Derivable.add hp (neg hq)

theorem degree_le (h : Derivable F d p) : p.totalDegree ≤ d := by
  induction h with
  | zero => simp
  | input _ h => exact h
  | add _ _ hp hq => exact (totalDegree_add _ _).trans (max_le hp hq)
  | smul a _ hp => exact (totalDegree_smul_le a _).trans hp
  | mul _ _ h => exact h

theorem monotone (h : d ≤ e) (hp : Derivable F d p) : Derivable F e p := by
  induction hp with
  | zero => exact .zero
  | input hf hd => exact .input hf (hd.trans h)
  | add _ _ hp hq => exact .add hp hq
  | smul a _ hp => exact .smul a hp
  | mul q _ hd hp => exact .mul q hp (hd.trans h)

theorem mem_span (hp : Derivable F d p) : p ∈ Ideal.span F := by
  induction hp with
  | zero => exact (Ideal.span F).zero_mem
  | input hf _ => exact Ideal.subset_span hf
  | add _ _ hp hq => exact (Ideal.span F).add_mem hp hq
  | smul a _ hp =>
    simpa only [C_mul'] using (Ideal.span F).mul_mem_left (C a) hp
  | mul q _ _ hp => exact (Ideal.span F).mul_mem_left q hp

/-- Universal property: every closed linear space contains every derivation. -/
theorem least (V : Submodule K (MvPolynomial σ K))
    (hinput : ∀ p ∈ F, p.totalDegree ≤ d → p ∈ V)
    (hmul : ∀ q p, p ∈ V → (q * p).totalDegree ≤ d → q * p ∈ V)
    (hp : Derivable F d p) : p ∈ V := by
  induction hp with
  | zero => exact V.zero_mem
  | input hf hd => exact hinput _ hf hd
  | add _ _ hp hq => exact V.add_mem hp hq
  | smul a _ hp => exact V.smul_mem a hp
  | mul q _ hd hp => exact hmul q _ hp hd

/-- The derivation relation really is a coefficient-linear subspace. -/
def space (F : Set (MvPolynomial σ K)) (d : ℕ) : Submodule K (MvPolynomial σ K) where
  carrier := {p | Derivable F d p}
  zero_mem' := .zero
  add_mem' := Derivable.add
  smul_mem' := Derivable.smul

/-- A last-fall bound, without assuming that the ideal is the unit ideal. -/
def LastFallBound (F : Set (MvPolynomial σ K)) (d : ℕ) : Prop :=
  ∀ p ∈ Ideal.span F, Derivable F (max d p.totalDegree) p

/-- For a unit ideal, a refutation at degree d implies the full last-fall
bound, including consequences of degree greater than d. -/
theorem lastFallBound_of_refutation (h : Derivable F d 1) : LastFallBound F d := by
  intro p _
  have h1 : Derivable F (max d p.totalDegree) 1 := monotone (Nat.le_max_left _ _) h
  have hp := Derivable.mul p h1 (by simp)
  simpa using hp

theorem refutation_of_lastFallBound (hunit : (1 : MvPolynomial σ K) ∈ Ideal.span F)
    (h : LastFallBound F d) : Derivable F d 1 := by
  simpa using h 1 hunit

theorem lastFallBound_iff_refutation
    (hunit : (1 : MvPolynomial σ K) ∈ Ideal.span F) :
    LastFallBound F d ↔ Derivable F d 1 :=
  ⟨refutation_of_lastFallBound hunit, lastFallBound_of_refutation⟩

/-- A closed space that excludes one certifies non-refutation. -/
theorem not_refutable_of_separator (V : Submodule K (MvPolynomial σ K))
    (hinput : ∀ p ∈ F, p.totalDegree ≤ d → p ∈ V)
    (hmul : ∀ q p, p ∈ V → (q * p).totalDegree ≤ d → q * p ∈ V)
    (hsep : (1 : MvPolynomial σ K) ∉ V) : ¬ Derivable F d 1 :=
  fun h => hsep (least V hinput hmul h)

/-- If all degree-at-most-three members of W lie in an ideal I, then W
is closed under every multiplication permitted at degree four. The
constant-multiplier branch and zero branch are included explicitly. -/
theorem closed_at_four_of_low_part (I : Ideal (MvPolynomial σ K))
    (W : Submodule K (MvPolynomial σ K))
    (hlow : ∀ p ∈ W, p.totalDegree ≤ 3 → p ∈ I)
    (hinclude : ∀ p ∈ I, p.totalDegree ≤ 4 → p ∈ W)
    (q p : MvPolynomial σ K) (hp : p ∈ W) (hd : (q * p).totalDegree ≤ 4) :
    q * p ∈ W := by
  by_cases hq : q = 0
  · simp [hq]
  by_cases hp0 : p = 0
  · simp [hp0]
  by_cases hc : q.totalDegree = 0
  · have hconst := (totalDegree_eq_zero_iff_eq_C).mp hc
    rw [hconst, C_mul']
    exact W.smul_mem _ hp
  · have hdeg := totalDegree_mul_of_isDomain hq hp0
    have hpd : p.totalDegree ≤ 3 := by omega
    exact hinclude _ (I.mul_mem_left q (hlow p hp hpd)) hd

end FilteredClosure

#print axioms FilteredClosure.degree_le
#print axioms FilteredClosure.neg
#print axioms FilteredClosure.sub
#print axioms FilteredClosure.monotone
#print axioms FilteredClosure.mem_span
#print axioms FilteredClosure.least
#print axioms FilteredClosure.lastFallBound_of_refutation
#print axioms FilteredClosure.refutation_of_lastFallBound
#print axioms FilteredClosure.lastFallBound_iff_refutation
#print axioms FilteredClosure.not_refutable_of_separator
#print axioms FilteredClosure.closed_at_four_of_low_part
