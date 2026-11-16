import Mathlib.RingTheory.PowerBasis
import Mathlib.RingTheory.Length
import Mathlib.Order.RelSeries

namespace LocalAlgebraLengthAudit
noncomputable section
open Submodule

variable {K B : Type*} [Field K] [CommRing B] [Algebra K B]

def tailSpan (pb : PowerBasis K B) (k : ℕ) : Submodule K B :=
  Submodule.span K (pb.basis '' {i : Fin pb.dim | k ≤ i.val})

theorem basis_mem_tail_iff (pb : PowerBasis K B) (k : ℕ) (i : Fin pb.dim) :
    pb.basis i ∈ tailSpan pb k ↔ k ≤ i.val := by
  exact pb.basis.self_mem_span_image

theorem tailSpan_gen_closed (pb : PowerBasis K B) (hn : pb.gen ^ pb.dim = 0)
    (k : ℕ) {z : B} (hz : z ∈ tailSpan pb k) : pb.gen * z ∈ tailSpan pb k := by
  induction hz using Submodule.span_induction with
  | mem z hz =>
    obtain ⟨i, hi, rfl⟩ := hz
    change k ≤ i.val at hi
    rw [pb.basis_eq_pow, ← pow_succ']
    by_cases hh : i.val + 1 < pb.dim
    · rw [← pb.basis_eq_pow ⟨i.val + 1, hh⟩]
      apply (basis_mem_tail_iff pb k _).mpr
      change k ≤ i.val + 1
      exact hi.trans (Nat.le_succ _)
    · have he : i.val + 1 = pb.dim := by have := i.isLt; omega
      rw [he, hn]
      exact (tailSpan pb k).zero_mem
  | zero => simp
  | add z w _ _ hz hw => simpa only [mul_add] using (tailSpan pb k).add_mem hz hw
  | smul c z _ hz => simpa only [mul_smul_comm] using (tailSpan pb k).smul_mem c hz

theorem tailSpan_pow_gen_closed (pb : PowerBasis K B) (hn : pb.gen ^ pb.dim = 0)
    (k n : ℕ) {z : B} (hz : z ∈ tailSpan pb k) : pb.gen ^ n * z ∈ tailSpan pb k := by
  induction n with
  | zero => simpa using hz
  | succ n ih =>
    simpa only [pow_succ', mul_assoc] using tailSpan_gen_closed pb hn k ih

theorem tailSpan_mul_closed (pb : PowerBasis K B) (hn : pb.gen ^ pb.dim = 0)
    (k : ℕ) (a : B) {z : B} (hz : z ∈ tailSpan pb k) : a * z ∈ tailSpan pb k := by
  rw [← pb.basis.sum_repr a, Finset.sum_mul]
  apply (tailSpan pb k).sum_mem
  intro i _
  rw [smul_mul_assoc, pb.basis_eq_pow]
  exact (tailSpan pb k).smul_mem _ (tailSpan_pow_gen_closed pb hn k i.val hz)

/-- These are actual ideals: closure under multiplication by every algebra
    element is proved from the actual power basis, not assumed. -/
def tailIdeal (pb : PowerBasis K B) (hn : pb.gen ^ pb.dim = 0) (k : ℕ) : Ideal B where
  carrier := tailSpan pb k
  zero_mem' := (tailSpan pb k).zero_mem
  add_mem' := (tailSpan pb k).add_mem
  smul_mem' a z hz := tailSpan_mul_closed pb hn k a (z := z) hz

theorem basis_mem_tailIdeal_iff (pb : PowerBasis K B) (hn : pb.gen ^ pb.dim = 0)
    (k : ℕ) (i : Fin pb.dim) : pb.basis i ∈ tailIdeal pb hn k ↔ k ≤ i.val :=
  basis_mem_tail_iff pb k i

theorem tailSpan_antitone (pb : PowerBasis K B) {k l : ℕ} (hkl : k ≤ l) :
    tailSpan pb l ≤ tailSpan pb k := by
  apply Submodule.span_mono
  rintro z ⟨i, hi, rfl⟩
  exact ⟨i, hkl.trans hi, rfl⟩

theorem tailIdeal_antitone (pb : PowerBasis K B) (hn : pb.gen ^ pb.dim = 0)
    {k l : ℕ} (hkl : k ≤ l) : tailIdeal pb hn l ≤ tailIdeal pb hn k :=
  fun _ hz => tailSpan_antitone pb hkl hz

theorem tailIdeal_strict (pb : PowerBasis K B) (hn : pb.gen ^ pb.dim = 0)
    (k : ℕ) (hk : k < pb.dim) : tailIdeal pb hn (k + 1) < tailIdeal pb hn k := by
  refine lt_iff_le_not_ge.mpr ⟨tailIdeal_antitone pb hn (Nat.le_succ k), ?_⟩
  intro h
  have hm : pb.basis ⟨k, hk⟩ ∈ tailIdeal pb hn k :=
    (basis_mem_tailIdeal_iff pb hn k _).mpr le_rfl
  have hh := (basis_mem_tailIdeal_iff pb hn (k + 1) _).mp (h hm)
  exact Nat.not_succ_le_self k hh

def idealChain (pb : PowerBasis K B) (hn : pb.gen ^ pb.dim = 0) :
    LTSeries (Submodule B B) where
  length := pb.dim
  toFun i := tailIdeal pb hn (pb.dim - i.val)
  step i := by
    change tailIdeal pb hn (pb.dim - i.val) < tailIdeal pb hn (pb.dim - (i.val + 1))
    have hk : pb.dim - (i.val + 1) < pb.dim := by have := i.isLt; omega
    have he : pb.dim - i.val = (pb.dim - (i.val + 1)) + 1 := by have := i.isLt; omega
    rw [he]
    exact tailIdeal_strict pb hn _ hk

theorem scalar_length_upper : Module.length B B ≤ Module.length K B := by
  rw [← WithBot.coe_le_coe, Module.coe_length, Module.coe_length]
  exact Order.krullDim_le_of_orderEmbedding (Submodule.restrictScalarsEmbedding K B B)

theorem powerBasis_ring_length_lower (pb : PowerBasis K B) (hn : pb.gen ^ pb.dim = 0) :
    (pb.dim : ℕ∞) ≤ Module.length B B := by
  have h : (pb.dim : WithBot ℕ∞) ≤ (Module.length B B : WithBot ℕ∞) := by
    rw [Module.coe_length]
    exact Order.LTSeries.length_le_krullDim (idealChain pb hn)
  exact WithBot.coe_le_coe.mp h

theorem nilpotent_powerBasis_ring_length (pb : PowerBasis K B) (hn : pb.gen ^ pb.dim = 0) :
    Module.length B B = pb.dim := by
  let : Module.Finite K B := Module.Finite.of_basis pb.basis
  have hs : Module.length K B = pb.dim := by
    rw [Module.length_eq_finrank, Module.finrank_eq_card_basis pb.basis, Fintype.card_fin]
  exact le_antisymm (scalar_length_upper.trans_eq hs) (powerBasis_ring_length_lower pb hn)

#print axioms tailIdeal_strict
#print axioms powerBasis_ring_length_lower
#print axioms nilpotent_powerBasis_ring_length
end
end LocalAlgebraLengthAudit
