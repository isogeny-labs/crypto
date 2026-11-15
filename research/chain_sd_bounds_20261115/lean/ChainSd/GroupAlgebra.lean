import Mathlib
import ChainSd.Closure
import ChainSd.Popcount

/-!
# The chain algebra and depth monotonicity

The normalized two-summand Kummer chain system of depth `k` lives, after reduction modulo the
tower relations, in the group algebra `F_p[(ℤ/2^k)²]`: the squarefree monomial
`∏ y_{1,j}^{a_j} ∏ y_{2,j}^{b_j}` is the character `x₁^a x₂^b` with `a = ∑ a_j 2^j`, and its degree
is the binary weight `popcount a + popcount b`.  Multiplication by the variable `y_{s,j}` is the
shift by `2^j` in coordinate `s`.  This file instantiates the abstract `ShiftSystem` on this
algebra and proves **depth monotonicity**: for `k' ≤ k` and the same `(p, A, B, t)`, if `1` is
derived at degree `c` at depth `k` then it is derived at degree `c` at depth `k'`; hence
`LFD(k') ≤ LFD(k)`.  The transfer is the ring homomorphism induced by `ℤ/2^k → ℤ/2^{k'}`, which
does not increase the binary weight and sends the shift `2^j` to the shift `2^j` (or to `1` when
`j ≥ k'`).
-/

namespace ChainSd

open AddMonoidAlgebra

/-- The exponent group of depth `k`. -/
abbrev Expo (k : ℕ) := ZMod (2 ^ k) × ZMod (2 ^ k)

/-- Binary weight of a residue: the number of set bits of its canonical value. -/
def bitWeight (k : ℕ) (a : ZMod (2 ^ k)) : ℕ := popcount a.val

/-- Weight of an exponent pair. -/
def wt (k : ℕ) (v : Expo k) : ℕ := bitWeight k v.1 + bitWeight k v.2

/-- The chain algebra `F_p[(ℤ/2^k)²]`. -/
abbrev ChainAlg (p k : ℕ) := AddMonoidAlgebra (ZMod p) (Expo k)

variable (p k : ℕ) [Fact p.Prime]

/-- The weight filtration: elements whose coefficients are supported on exponents of weight
`≤ c`. -/
noncomputable def weightSub (c : ℕ) : Submodule (ZMod p) (ChainAlg p k) :=
  (Finsupp.supported (ZMod p) (ZMod p) {v : Expo k | wt k v ≤ c}).comap
    (coeffLinearEquiv (R := ZMod p) (S := ZMod p) (M := Expo k)).toLinearMap

theorem mem_weightSub {c : ℕ} {u : ChainAlg p k} :
    u ∈ weightSub p k c ↔ ∀ v ∈ u.coeff.support, wt k v ≤ c := by
  simp only [weightSub, Submodule.mem_comap, LinearEquiv.coe_coe, Finsupp.mem_supported,
    Set.subset_def, Finset.mem_coe, Set.mem_ofPred_eq]
  rfl

theorem weightSub_mono : Monotone (weightSub p k) := by
  intro c c' h u hu
  rw [mem_weightSub] at hu ⊢
  intro v hv
  exact le_trans (hu v hv) h

/-- The shift exponent of variable `(j, s)`: `2^j` in coordinate `s`. -/
def shiftExpo (j : Fin k) (s : Bool) : Expo k :=
  if s then ((0 : ZMod (2 ^ k)), ((2 ^ (j : ℕ) : ℕ) : ZMod (2 ^ k)))
  else (((2 ^ (j : ℕ) : ℕ) : ZMod (2 ^ k)), (0 : ZMod (2 ^ k)))

/-- Multiplication by the variable `y_{s,j}`, as a linear map. -/
noncomputable def shiftMap (js : Fin k × Bool) : ChainAlg p k →ₗ[ZMod p] ChainAlg p k :=
  LinearMap.mulLeft (ZMod p) (single (shiftExpo k js.1 js.2) (1 : ZMod p))

/-- A character `x₁^{e₁} x₂^{e₂}` with natural exponents. -/
noncomputable def mono (e1 e2 : ℕ) (c : ZMod p) : ChainAlg p k :=
  single (((e1 : ℕ) : ZMod (2 ^ k)), ((e2 : ℕ) : ZMod (2 ^ k))) c

/-- The reduced Semaev generator `S₃(y_{1,0}, y_{2,0}, t)` on `y² = x³ + A x + B`. -/
noncomputable def gen (A B t : ZMod p) : ChainAlg p k :=
  mono p k 2 2 1 + mono p k 2 1 (-2 * t) + mono p k 1 2 (-2 * t) + mono p k 2 0 (t ^ 2)
    + mono p k 0 2 (t ^ 2) + mono p k 1 1 (-2 * t ^ 2 - 2 * A) + mono p k 1 0 (-2 * t * A - 4 * B)
    + mono p k 0 1 (-2 * t * A - 4 * B) + mono p k 0 0 (A ^ 2 - 4 * B * t)

/-- The normalized two-summand chain system of depth `k` as a shift system. -/
noncomputable def chainSystem (A B t : ZMod p) :
    ShiftSystem (ZMod p) (ChainAlg p k) (Fin k × Bool) where
  F := weightSub p k
  mono := weightSub_mono p k
  σ := shiftMap p k
  g := gen p k A B t

section Transfer

variable {k} {k' : ℕ} (hkk : k' ≤ k)

/-- Reduction of exponents `ℤ/2^k → ℤ/2^{k'}` as a ring homomorphism. -/
noncomputable def castZ : ZMod (2 ^ k) →+* ZMod (2 ^ k') :=
  ZMod.castHom (pow_dvd_pow 2 hkk) (ZMod (2 ^ k'))

theorem castZ_natCast (n : ℕ) : castZ hkk (n : ZMod (2 ^ k)) = (n : ZMod (2 ^ k')) := by
  simp [castZ]

theorem castZ_val (a : ZMod (2 ^ k)) : (castZ hkk a).val = a.val % 2 ^ k' := by
  rw [castZ, ZMod.castHom_apply, ZMod.cast_eq_val, ZMod.val_natCast]

/-- Reduction does not increase the binary weight. -/
theorem bitWeight_castZ_le (a : ZMod (2 ^ k)) : bitWeight k' (castZ hkk a) ≤ bitWeight k a := by
  unfold bitWeight
  rw [castZ_val]
  exact popcount_mod_pow_le _ _

/-- The exponent reduction on pairs, as an additive monoid homomorphism. -/
noncomputable def castExpo : Expo k →+ Expo k' :=
  AddMonoidHom.prodMap (castZ hkk).toAddMonoidHom (castZ hkk).toAddMonoidHom

theorem castExpo_apply (v : Expo k) : castExpo hkk v = (castZ hkk v.1, castZ hkk v.2) := rfl

theorem wt_castExpo_le (v : Expo k) : wt k' (castExpo hkk v) ≤ wt k v := by
  unfold wt
  rw [castExpo_apply]
  exact Nat.add_le_add (bitWeight_castZ_le hkk v.1) (bitWeight_castZ_le hkk v.2)

/-- The induced algebra homomorphism on chain algebras. -/
noncomputable def castAlg : ChainAlg p k →ₐ[ZMod p] ChainAlg p k' :=
  mapDomainAlgHom (ZMod p) (ZMod p) (castExpo hkk)

theorem castAlg_apply (u : ChainAlg p k) :
    castAlg p hkk u = mapDomain (castExpo hkk) u := rfl

theorem castAlg_single (v : Expo k) (c : ZMod p) :
    castAlg p hkk (single v c) = single (castExpo hkk v) c := by
  rw [castAlg_apply, mapDomain_single]

theorem castAlg_mono (e1 e2 : ℕ) (c : ZMod p) :
    castAlg p hkk (mono p k e1 e2 c) = mono p k' e1 e2 c := by
  rw [mono, castAlg_single, castExpo_apply, mono]
  simp

theorem castAlg_gen (A B t : ZMod p) : castAlg p hkk (gen p k A B t) = gen p k' A B t := by
  simp only [gen, map_add, castAlg_mono]

/-- The reduction map respects the weight filtration. -/
theorem castAlg_mem_weightSub (c : ℕ) (u : ChainAlg p k) (hu : u ∈ weightSub p k c) :
    castAlg p hkk u ∈ weightSub p k' c := by
  classical
  rw [mem_weightSub] at hu ⊢
  intro w hw
  rw [castAlg_apply, mapDomain, Finsupp.mapDomain] at hw
  -- coefficient of w in the mapped element is a sum over preimages; nonzero only if some preimage
  -- in the support maps to w
  simp only at hw
  have hw' := Finsupp.support_sum hw
  rw [Finset.mem_biUnion] at hw'
  obtain ⟨v, hv, hvw⟩ := hw'
  have hvw' := Finsupp.support_single_subset hvw
  rw [Finset.mem_singleton] at hvw'
  subst hvw'
  exact le_trans (wt_castExpo_le hkk v) (hu v hv)

/-- The shift exponents are compatible: `2^j` maps to `2^j`, or to `0` when `j ≥ k'`. -/
theorem castExpo_shiftExpo (j : Fin k) (s : Bool) :
    (∃ j' : Fin k', castExpo hkk (shiftExpo k j s) = shiftExpo k' j' s) ∨
      castExpo hkk (shiftExpo k j s) = 0 := by
  have h2 : castZ hkk (2 : ZMod (2 ^ k)) = 2 := map_ofNat _ 2
  by_cases hj : (j : ℕ) < k'
  · left
    refine ⟨⟨j, hj⟩, ?_⟩
    cases s <;> simp [shiftExpo, castExpo_apply, h2]
  · right
    have hdvd : 2 ^ k' ∣ 2 ^ (j : ℕ) := pow_dvd_pow 2 (not_lt.mp hj)
    have hz : ((2 ^ (j : ℕ) : ℕ) : ZMod (2 ^ k')) = 0 := by
      obtain ⟨m, hm⟩ := hdvd
      rw [hm, Nat.cast_mul, ZMod.natCast_self, zero_mul]
    push_cast at hz
    cases s <;> simp [shiftExpo, castExpo_apply, h2, hz, Prod.ext_iff]

/-- Shifts intertwine with the reduction map, or are trivialised by it. -/
theorem castAlg_shift (js : Fin k × Bool) :
    (∃ js' : Fin k' × Bool,
        ∀ u, castAlg p hkk (shiftMap p k js u) = shiftMap p k' js' (castAlg p hkk u)) ∨
      (∀ u, castAlg p hkk (shiftMap p k js u) = castAlg p hkk u) := by
  rcases castExpo_shiftExpo hkk js.1 js.2 with ⟨j', hj'⟩ | h0
  · left
    refine ⟨(j', js.2), fun u => ?_⟩
    simp only [shiftMap, LinearMap.mulLeft_apply, map_mul, castAlg_single, hj']
  · right
    intro u
    simp only [shiftMap, LinearMap.mulLeft_apply, map_mul, castAlg_single, h0]
    rw [← one_def, one_mul]

/-- **Depth monotonicity.** If `1` lies in the degree-`c` closure at depth `k`, it lies in the
degree-`c` closure at every depth `k' ≤ k` (same `p, A, B, t`).  Hence `LFD(k') ≤ LFD(k)`. -/
theorem one_mem_closure_of_le (hkk : k' ≤ k) (A B t : ZMod p) (c : ℕ)
    (h : (1 : ChainAlg p k) ∈ (chainSystem p k A B t).closure c) :
    (1 : ChainAlg p k') ∈ (chainSystem p k' A B t).closure c := by
  have hg : castAlg p hkk (chainSystem p k A B t).g ∈ (chainSystem p k' A B t).closure c := by
    show castAlg p hkk (gen p k A B t) ∈ _
    rw [castAlg_gen]
    exact (chainSystem p k' A B t).g_mem_closure c
  have := (chainSystem p k A B t).closure_transfer (chainSystem p k' A B t)
    (castAlg p hkk).toLinearMap (fun c u hu => castAlg_mem_weightSub p hkk c u hu)
    (fun js => by
      rcases castAlg_shift p hkk js with ⟨js', hjs⟩ | htriv
      · exact Or.inl ⟨js', fun u => hjs u⟩
      · exact Or.inr htriv) c hg 1 h
  simpa using this

/-- The solving degree (= refutation degree = last fall degree for a unit ideal) is monotone in the
depth, whenever it is finite at depth `k`. -/
theorem solvingDegree_mono (hkk : k' ≤ k) (A B t : ZMod p)
    (hfin : ((chainSystem p k A B t).refutationDegrees 1).Nonempty) :
    (chainSystem p k' A B t).solvingDegree 1 ≤ (chainSystem p k A B t).solvingDegree 1 := by
  apply Nat.sInf_le
  have hmem := Nat.sInf_mem hfin
  exact one_mem_closure_of_le p hkk A B t _ hmem

end Transfer

end ChainSd
