import QuadraticWitness

namespace BKActualEmbeddingAudit
noncomputable section
open BKActualThreeAudit FrobeniusPointDescentAudit FrobeniusFiniteExtensionAudit
private instance primeTwo : Fact (Nat.Prime 2) := ⟨by decide⟩
private instance primeThree : Fact (Nat.Prime 3) := ⟨by decide⟩
variable {L : Type*} [Field L] [Algebra F L] [DecidableEq L]

abbrev sigma := pointFrobenius (L:=L) curve

theorem even_iterate_fixed (P : (geomCurve (L:=L)).Point)
    (ht : (3:ℕ) • P=0) (n : ℕ) : sigma^[2*n] P=P := by
  have h2 : sigma^[2] P=P := by
    simpa only [Function.iterate_succ_apply,Function.iterate_zero_apply]
      using frobenius_square_fixed P ht
  induction n with
  | zero => rfl
  | succ n ih =>
    rw [Nat.mul_succ,Function.iterate_add_apply,h2]
    exact ih

variable [IsAlgClosed L]

theorem odd_witness_not_fixed (n : ℕ) :
    sigma^[2*n+1] (BKActualWitnessAudit.P (L:=L))≠BKActualWitnessAudit.P := by
  rw [Function.iterate_succ_apply',
    even_iterate_fixed _ BKActualWitnessAudit.P_three]
  exact BKActualWitnessAudit.P_not_fixed

theorem all_torsion_fixed_iff_even (k : ℕ) :
    (∀ P : (geomCurve (L:=L)).Point,(3:ℕ) • P=0 → sigma^[k] P=P) ↔ Even k := by
  constructor
  · intro h
    rcases Nat.even_or_odd k with he | ho
    · exact he
    · obtain ⟨n,hn⟩ := ho
      have hk : k=2*n+1 := by omega
      exact False.elim (odd_witness_not_fixed n (by
        simpa only [hk] using h BKActualWitnessAudit.P BKActualWitnessAudit.P_three))
  · rintro ⟨n,hn⟩ P ht
    have hk : k=2*n := by omega
    rw [hk]
    exact even_iterate_fixed P ht n

omit [Field L] [Algebra F L] [DecidableEq L] [IsAlgClosed L] in
theorem divisibility_iff_even (k : ℕ) : 3∣5^k-1 ↔ Even k := by
  have hp0 : 0<5^k := pow_pos (by decide : (0:ℕ)<5) k
  have hp : 1≤5^k := hp0
  have hz : (1:ZMod 3)=(5:ZMod 3)^k ↔ Nat.ModEq 3 1 (5^k) := by
    simpa only [Nat.cast_one,Nat.cast_pow,Nat.cast_ofNat] using
      (ZMod.natCast_eq_natCast_iff 1 (5^k) 3)
  have ho : orderOf (5:ZMod 3)=2 :=
    orderOf_eq_prime (by decide +kernel) (by decide +kernel)
  calc
    3∣5^k-1 ↔ Nat.ModEq 3 1 (5^k) := (Nat.modEq_iff_dvd' hp).symm
    _ ↔ (1:ZMod 3)=(5:ZMod 3)^k := hz.symm
    _ ↔ (5:ZMod 3)^k=1 := eq_comm
    _ ↔ orderOf (5:ZMod 3)∣k := orderOf_dvd_iff_pow_eq_one.symm
    _ ↔ Even k := by rw [ho,even_iff_two_dvd]

theorem actual_frobenius_criterion (k : ℕ) :
    (∀ P : (geomCurve (L:=L)).Point,(3:ℕ) • P=0 → sigma^[k] P=P) ↔ 3∣5^k-1 :=
  (all_torsion_fixed_iff_even k).trans (divisibility_iff_even k).symm

section ChosenField
variable {K : Type*} [Field K] [Fintype K] [Algebra F K]
  [Algebra K L] [IsScalarTower F K L] [DecidableEq K]

theorem actual_extension_torsion_criterion (k : ℕ) (hc : Fintype.card K=5^k) :
    (∀ P : (geomCurve (L:=L)).Point,(3:ℕ) • P=0 →
      P∈Set.range (WeierstrassCurve.Affine.Point.map (W':=curve) (Algebra.ofId K L))) ↔
        3∣5^k-1 := by
  have hc' : Fintype.card K=Fintype.card F^k := by simpa only [ZMod.card] using hc
  rw [finite_extension_torsion_iff_of_card (F:=F) (K:=K) (B:=L) curve 3 k hc']
  exact actual_frobenius_criterion k
end ChosenField

#print axioms all_torsion_fixed_iff_even
#print axioms divisibility_iff_even
#print axioms actual_frobenius_criterion
#print axioms actual_extension_torsion_criterion
end
end BKActualEmbeddingAudit
