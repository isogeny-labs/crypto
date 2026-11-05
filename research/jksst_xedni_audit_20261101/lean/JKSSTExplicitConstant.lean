import JKSSTAudit

/-!
# An explicit replacement for JKSST's omitted asymptotic error term

The geometry-of-numbers step should first produce a finite packing estimate
`#S ≤ T * (A*C)^k`, where `k` is the rank of the generated Mordell--Weil
lattice, `T` bounds its torsion, `A` is an explicit separation/height-ratio
factor, and the original number of sampled points satisfies `r ≥ k+1`.

This file proves that the explicit coefficient radius `C = T*A^k`
makes the pigeonhole argument work. No asymptotic notation occurs.
-/

open scoped BigOperators

namespace JKSSTAudit

/-- The exact arithmetic inequality behind the explicit coefficient choice. -/
theorem explicit_coefficient_threshold (T A k : ℕ) :
    let C := T * A ^ k
    T * (A * C) ^ k < (C + 1) ^ (k + 1) := by
  dsimp
  let C := T * A ^ k
  calc
    T * (A * C) ^ k = (T * A ^ k) * C ^ k := by
      rw [mul_pow]
      ac_rfl
    _ = C * C ^ k := rfl
    _ = C ^ (k + 1) := by rw [pow_succ]; ac_rfl
    _ < (C + 1) ^ (k + 1) :=
      Nat.pow_lt_pow_left (Nat.lt_succ_self C) (by omega)

/-- If an exact packing theorem supplies `S.card ≤ T*(A*C)^k`, the explicit
choice `C=T*A^k` yields a nonzero relation bounded by `C` whenever the
generated rank has codimension at least one (`k+1 ≤ r`). -/
theorem bounded_relation_of_explicit_packing_count
    {G ι : Type*} [AddCommGroup G] [Fintype ι]
    (P : ι → G) (T A k : ℕ)
    (rank_gap : k + 1 ≤ Fintype.card ι)
    (S : Finset G)
    (range_mem : ∀ c : ι → Fin (T * A ^ k + 1),
      (∑ i, (c i : ℤ) • P i) ∈ S)
    (packing_count :
      S.card ≤ T * (A * (T * A ^ k)) ^ k) :
    ∃ m : ι → ℤ,
      m ≠ 0 ∧
      (∀ i, |m i| ≤ T * A ^ k) ∧
      ∑ i, m i • P i = 0 := by
  apply bounded_relation_of_finite_range P (T * A ^ k) S range_mem
  calc
    S.card ≤ T * (A * (T * A ^ k)) ^ k := packing_count
    _ < (T * A ^ k + 1) ^ (k + 1) :=
      explicit_coefficient_threshold T A k
    _ ≤ (T * A ^ k + 1) ^ Fintype.card ι := by
      apply Nat.pow_le_pow_right
      · omega
      · exact rank_gap

end JKSSTAudit
