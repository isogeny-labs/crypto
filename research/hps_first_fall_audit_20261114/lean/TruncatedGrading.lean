module
public import Mathlib.RingTheory.AdjoinRoot
public import Mathlib.RingTheory.PowerBasis
public import Mathlib.Algebra.Algebra.Subalgebra.Lattice
public import Mathlib.Tactic

@[expose] public section

namespace HPSFirstFallAudit
noncomputable section
open Polynomial
variable (K : Type*) [Field K]

/-- The actual quotient K[X]/(X^q), not a numerical model. -/
abbrev Truncated (q : ℕ) := AdjoinRoot (X ^ q : Polynomial K)

def x (q : ℕ) : Truncated K q := AdjoinRoot.root (X ^ q : Polynomial K)

def powerBasis (q : ℕ) : PowerBasis K (Truncated K q) :=
  AdjoinRoot.powerBasis' (monic_X.pow q : (X ^ q : Polynomial K).Monic)

@[simp] theorem powerBasis_dim (q : ℕ) : (powerBasis K q).dim = q := by
  simp [powerBasis, AdjoinRoot.powerBasis'_dim]

@[simp] theorem powerBasis_gen (q : ℕ) : (powerBasis K q).gen = x K q := rfl

theorem x_pow_q (q : ℕ) : x K q ^ q = 0 := by
  have h := AdjoinRoot.mk_self (f := (X ^ q : Polynomial K))
  simpa [map_pow, AdjoinRoot.mk_X, x] using h

theorem x_pow_ne_zero (q k : ℕ) (hk : k < q) : x K q ^ k ≠ 0 := by
  have h := AdjoinRoot.mk_ne_zero_of_natDegree_lt
    (monic_X.pow q : (X ^ q : Polynomial K).Monic)
    (g := X ^ k) (by simp) (by simpa using hk)
  simpa [map_pow, AdjoinRoot.mk_X, x] using h

theorem x_pow_eq_zero (q k : ℕ) (hk : q ≤ k) : x K q ^ k = 0 := by
  obtain ⟨r, rfl⟩ := Nat.exists_eq_add_of_le hk
  rw [pow_add, x_pow_q, zero_mul]

/-- The actual degree-k homogeneous component in the one-variable quotient. -/
def piece (q k : ℕ) : Submodule K (Truncated K q) := Submodule.span K {x K q ^ k}

theorem mem_piece (q k : ℕ) (a : Truncated K q) :
    a ∈ piece K q k ↔ ∃ c : K, c • x K q ^ k = a := by
  exact Submodule.mem_span_singleton

theorem piece_mul (q i j : ℕ) {a b : Truncated K q}
    (ha : a ∈ piece K q i) (hb : b ∈ piece K q j) : a * b ∈ piece K q (i + j) := by
  obtain ⟨c, rfl⟩ := (mem_piece K q i a).mp ha
  obtain ⟨d, rfl⟩ := (mem_piece K q j b).mp hb
  apply (mem_piece K q (i + j) _).mpr
  exact ⟨c * d, by simp [smul_smul, pow_add, mul_comm]⟩

theorem piece_above (q k : ℕ) (hk : q ≤ k) : piece K q k = ⊥ := by
  simp [piece, x_pow_eq_zero K q k hk]

theorem basis_coordinates (q : ℕ) (a : Truncated K q) :
    ∃! c : Fin (powerBasis K q).dim → K,
      a = ∑ i, c i • x K q ^ i.val := by
  let b := (powerBasis K q).basis
  refine ⟨fun i => b.repr a i, ?_, ?_⟩
  · simpa [b, PowerBasis.basis_eq_pow] using (b.sum_repr a).symm
  · intro c hc
    have hp (i : Fin (powerBasis K q).dim) : x K q ^ i.val = b i := by
      simp [b, PowerBasis.basis_eq_pow]
    simp only [hp] at hc
    funext i
    have h := congrArg (fun z => b.repr z i) hc
    simpa [Finsupp.single_apply] using h.symm

theorem distinct_powers_not_scalar (q i j : ℕ) (hi : i < q) (hj : j < q) (hij : i ≠ j)
    (c : K) : c • x K q ^ i ≠ x K q ^ j := by
  let b := (powerBasis K q).basis
  let ii : Fin (powerBasis K q).dim := ⟨i, by simpa using hi⟩
  let jj : Fin (powerBasis K q).dim := ⟨j, by simpa using hj⟩
  have he : ii ≠ jj := fun h => hij (congrArg Fin.val h)
  have bi : b ii = x K q ^ i := by simp [b, ii, PowerBasis.basis_eq_pow]
  have bj : b jj = x K q ^ j := by simp [b, jj, PowerBasis.basis_eq_pow]
  intro h
  rw [← bi, ← bj] at h
  have hh := congrArg (fun z => b.repr z jj) h
  simp [he] at hh

theorem positive_part_nilpotent (q : ℕ) {a : Truncated K q}
    (ha : a ∈ Ideal.span ({x K q} : Set (Truncated K q))) : a ^ q = 0 := by
  obtain ⟨b, rfl⟩ := Ideal.mem_span_singleton.mp ha
  rw [mul_pow, x_pow_q, zero_mul]

theorem monomial_piece_iff (q d : ℕ) (hd : d < q) (k : ℕ) :
    x K q ^ d ∈ piece K q k ↔ k = d := by
  constructor
  · intro hm
    by_cases hk : k < q
    · by_contra hne
      obtain ⟨c, hc⟩ := (mem_piece K q k _).mp hm
      exact distinct_powers_not_scalar K q k d hk hd hne c hc
    · rw [piece_above K q k (by omega), Submodule.mem_bot] at hm
      exact (x_pow_ne_zero K q d hd hm).elim
  · rintro rfl
    exact Submodule.mem_span_singleton_self _

theorem actual_dimension (q : ℕ) : Module.finrank K (Truncated K q) = q := by
  rw [Module.finrank_eq_card_basis (powerBasis K q).basis, Fintype.card_fin, powerBasis_dim]

theorem finite_pieces_independent (q : ℕ) :
    iSupIndep (fun i : Fin (powerBasis K q).dim => piece K q i.val) := by
  simpa only [piece, PowerBasis.basis_eq_pow, powerBasis_gen] using
    (powerBasis K q).basis.linearIndependent.iSupIndep_span_singleton

theorem finite_pieces_span (q : ℕ) :
    (⨆ i : Fin (powerBasis K q).dim, piece K q i.val) = ⊤ := by
  rw [show (fun i : Fin (powerBasis K q).dim => piece K q i.val) =
      (fun i => Submodule.span K {(powerBasis K q).basis i}) from by
        funext i
        simp only [piece, PowerBasis.basis_eq_pow, powerBasis_gen]]
  rw [← Submodule.span_range_eq_iSup]
  exact (powerBasis K q).basis.span_eq

/-- Every positive graded piece lies in the actual augmentation ideal. -/
theorem positive_piece_mem (q k : ℕ) (hk : 0 < k) {a : Truncated K q}
    (ha : a ∈ piece K q k) : a ∈ Ideal.span ({x K q} : Set (Truncated K q)) := by
  obtain ⟨c, rfl⟩ := (mem_piece K q k a).mp ha
  rw [Algebra.smul_def]
  apply Ideal.mul_mem_left
  obtain ⟨j, rfl⟩ := Nat.exists_eq_succ_of_ne_zero (by omega : k ≠ 0)
  rw [pow_succ]
  exact Ideal.mul_mem_left _ _ (Ideal.subset_span (Set.mem_singleton _))

def positivePart (q : ℕ) : Submodule K (Truncated K q) :=
  ⨆ (k : ℕ) (_ : 0 < k), piece K q k

theorem positivePart_pow_zero (q : ℕ) {a : Truncated K q}
    (ha : a ∈ positivePart K q) : a ^ q = 0 := by
  have hle : positivePart K q ≤
      (Ideal.span ({x K q} : Set (Truncated K q))).restrictScalars K := by
    apply iSup_le
    intro k
    apply iSup_le
    intro hk z hz
    exact positive_piece_mem K q k hk hz
  exact positive_part_nilpotent K q (hle ha)

#print axioms x_pow_q
#print axioms x_pow_ne_zero
#print axioms piece_mul
#print axioms basis_coordinates
#print axioms distinct_powers_not_scalar
#print axioms positive_part_nilpotent
#print axioms monomial_piece_iff
#print axioms actual_dimension
#print axioms finite_pieces_independent
#print axioms finite_pieces_span
#print axioms positive_piece_mem
#print axioms positivePart_pow_zero
end
end HPSFirstFallAudit
