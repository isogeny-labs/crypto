import Mathlib.RingTheory.GradedAlgebra.HomogeneousLocalization
set_option autoImplicit false
noncomputable section
namespace WeightedChartKernelRec
open DirectSum
variable {A σ : Type*} [CommRing A] [SetLike σ A] [AddSubgroupClass σ A]
variable (𝒜 : ℕ → σ) [GradedRing 𝒜] {z q h : A} {n : ℕ}

theorem component_z_mul_succ (hz : z ∈ 𝒜 1) (i : ℕ) :
    (decompose 𝒜 (z * q) (i+1) : A) = z * (decompose 𝒜 q i : A) := by
  simpa using (DirectSum.coe_decompose_mul_of_left_mem_of_le 𝒜 hz (Nat.le_add_left 1 i))

theorem component_z_mul_zero (hz : z ∈ 𝒜 1) :
    (decompose 𝒜 (z * q) 0 : A) = 0 := by
  exact DirectSum.coe_decompose_mul_of_left_mem_of_not_le 𝒜 hz (by omega)

theorem kernel_component_recurrence (hz : z ∈ 𝒜 1) (heq : h = (z - 1) * q)
    (i : ℕ) :
    (decompose 𝒜 h (i+1) : A) =
      z * (decompose 𝒜 q i : A) - (decompose 𝒜 q (i+1) : A) := by
  have heq' : h = z*q-q := by simpa [sub_mul] using heq
  calc
    (decompose 𝒜 h (i+1) : A) =
        (decompose 𝒜 (z*q) (i+1) : A) - (decompose 𝒜 q (i+1) : A) := by
          rw [heq', DirectSum.decompose_sub]
          rfl
    _ = z * (decompose 𝒜 q i : A) - (decompose 𝒜 q (i+1) : A) := by
          rw [component_z_mul_succ 𝒜 hz i]

theorem kernel_component_zero (hz : z ∈ 𝒜 1) (heq : h = (z - 1) * q) :
    (decompose 𝒜 h 0 : A) = - (decompose 𝒜 q 0 : A) := by
  have heq' : h = z*q-q := by simpa [sub_mul] using heq
  calc
    (decompose 𝒜 h 0 : A) =
        (decompose 𝒜 (z*q) 0 : A) - (decompose 𝒜 q 0 : A) := by
          rw [heq', DirectSum.decompose_sub]
          rfl
    _ = -(decompose 𝒜 q 0 : A) := by rw [component_z_mul_zero 𝒜 hz]; ring

theorem kernel_component_formula (hz : z ∈ 𝒜 1) (hh : h ∈ 𝒜 n)
    (heq : h = (z - 1) * q) (i : ℕ) :
    (decompose 𝒜 q i : A) = if i < n then 0 else -(z ^ (i - n) * h) := by
  induction i with
  | zero =>
      have hrec := kernel_component_zero 𝒜 hz heq
      by_cases hn : n = 0
      · subst n
        have hh0 : (decompose 𝒜 h 0 : A) = h :=
          DirectSum.decompose_of_mem_same 𝒜 hh
        simp only [Nat.lt_irrefl, ↓reduceIte, Nat.sub_zero, pow_zero, one_mul]
        rw [hh0] at hrec
        simpa [hrec]
      · have hh0 : (decompose 𝒜 h 0 : A) = 0 :=
          DirectSum.decompose_of_mem_ne 𝒜 hh (by omega)
        have hq0 : (decompose 𝒜 q 0 : A) = 0 := by
          rw [hh0] at hrec
          exact neg_eq_zero.mp hrec.symm
        simpa [Nat.pos_of_ne_zero hn] using hq0
  | succ i ih =>
      have hrec := kernel_component_recurrence 𝒜 hz heq i
      have hq : (decompose 𝒜 q (i+1) : A) =
          z * (decompose 𝒜 q i : A) - (decompose 𝒜 h (i+1) : A) := by
        calc
          (decompose 𝒜 q (i+1) : A) =
              z * (decompose 𝒜 q i : A) -
                (z * (decompose 𝒜 q i : A) - (decompose 𝒜 q (i+1) : A)) := by ring
          _ = z * (decompose 𝒜 q i : A) - (decompose 𝒜 h (i+1) : A) := by rw [hrec]
      by_cases hlt : i + 1 < n
      · have hi : i < n := by omega
        have hhi : (decompose 𝒜 h (i+1) : A) = 0 :=
          DirectSum.decompose_of_mem_ne 𝒜 hh (by omega)
        simp [hlt, hi, ih, hhi] at hq ⊢
        exact hq
      · by_cases heq' : i + 1 = n
        · have hi : i < n := by omega
          have hhi : (decompose 𝒜 h (i+1) : A) = h := by
            rw [heq']
            exact DirectSum.decompose_of_mem_same 𝒜 hh
          simp [hlt, hi, ih, hhi, heq'] at hq ⊢
          exact hq
        · have hi : n ≤ i := by omega
          have hhi : (decompose 𝒜 h (i+1) : A) = 0 :=
            DirectSum.decompose_of_mem_ne 𝒜 hh (by omega)
          simp [hlt, Nat.not_lt.mpr hi, ih, hhi] at hq ⊢
          have hexp : i + 1 - n = (i - n) + 1 := by omega
          rw [hexp, pow_succ]
          calc
            (decompose 𝒜 q (i+1) : A) = -(z * (z ^ (i-n) * h)) := hq
            _ = -(z ^ (i - n) * z * h) := by ring

theorem homogeneous_kernel_saturation (hz : z ∈ 𝒜 1) (hh : h ∈ 𝒜 n)
    (heq : h = (z - 1) * q) : ∃ k : ℕ, z ^ k * h = 0 := by
  classical
  let s : Finset ℕ := (decompose 𝒜 q).support
  let N : ℕ := s.sup id + n + 1
  have hn : n ≤ N := by dsimp [N]; omega
  have hN : (decompose 𝒜 q N : A) = 0 := by
    have hzN : decompose 𝒜 q N = 0 := by
      by_contra hne
      have hmem : N ∈ s := DFinsupp.mem_support_iff.mpr hne
      have hle : N ≤ s.sup id := Finset.le_sup (f := id) hmem
      dsimp [N] at hle
      omega
    exact congrArg Subtype.val hzN
  have hf := kernel_component_formula 𝒜 hz hh heq N
  rw [if_neg (Nat.not_lt.mpr hn), hN] at hf
  refine ⟨N-n, ?_⟩
  exact neg_eq_zero.mp hf.symm

#print axioms WeightedChartKernelRec.homogeneous_kernel_saturation

#print axioms WeightedChartKernelRec.kernel_component_recurrence
#print axioms WeightedChartKernelRec.kernel_component_zero
end WeightedChartKernelRec
