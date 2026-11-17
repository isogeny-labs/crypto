import JKSSTAudit

/-!
# Prime-order cyclic-group bridge for the JKSST probability model

The probability files work with scalars in `ZMod q`.  This file connects that
model to a genuine cyclic subgroup of an arbitrary additive commutative group.
If `P : G` has additive order `q`, the quotient construction gives an
injective additive map `ZMod q →+ G` sending an integer residue `n` to
`n • P`.  Thus a relation among encoded ECDLP samples implies the scalar
relation used by the exact finite-field theorem.

For division/recovery, `q` is assumed prime, so `ZMod q` is a field.  No
elliptic-curve or good-reduction claim is hidden here: an application must
still prove that its chosen point really has order `q` and that the sampled
curve points are represented by this cyclic subgroup map.
-/

open scoped BigOperators

namespace JKSSTCyclicECDLP

variable {G ι : Type*} [AddCommGroup G] [Fintype ι]

/-- The additive map from residue scalars to multiples of `P`, given an
explicit proof that `q` annihilates `P`. -/
noncomputable def cyclicScalarMap (q : ℕ) (P : G) (annihilates : q • P = 0) :
    ZMod q →+ G :=
  ZMod.lift q ⟨zmultiplesHom G P, by simpa using annihilates⟩

@[simp] theorem cyclicScalarMap_intCast
    (q : ℕ) (P : G) (annihilates : q • P = 0) (n : ℤ) :
    cyclicScalarMap q P annihilates (n : ZMod q) = n • P := by
  simp [cyclicScalarMap]

/-- Having exact additive order `q` supplies the annihilation proof needed by
`cyclicScalarMap`. -/
theorem annihilates_of_addOrderOf_eq
    (q : ℕ) (P : G) (order_eq : addOrderOf P = q) : q • P = 0 := by
  rw [← order_eq]
  exact addOrderOf_nsmul_eq_zero P

/-- The canonical scalar map attached to a point of exact order `q`. -/
noncomputable def orderScalarMap (q : ℕ) (P : G)
    (order_eq : addOrderOf P = q) : ZMod q →+ G :=
  cyclicScalarMap q P (annihilates_of_addOrderOf_eq q P order_eq)

@[simp] theorem orderScalarMap_intCast
    (q : ℕ) (P : G) (order_eq : addOrderOf P = q) (n : ℤ) :
    orderScalarMap q P order_eq (n : ZMod q) = n • P := by
  simp [orderScalarMap, cyclicScalarMap_intCast]

/-- Exact order makes the residue-scalar encoding injective. -/
theorem orderScalarMap_injective
    (q : ℕ) (P : G) (order_eq : addOrderOf P = q) :
    Function.Injective (orderScalarMap q P order_eq) := by
  unfold orderScalarMap cyclicScalarMap
  rw [ZMod.lift_injective]
  intro n hn
  rw [ZMod.intCast_zmod_eq_zero_iff_dvd]
  have hmem : n ∈ (zmultiplesHom G P).ker := hn
  rw [zmultiplesHom_ker_eq, order_eq] at hmem
  exact Int.mem_zmultiples_iff.mp hmem

/-- The encoded ECDLP target `Q = wP`. -/
noncomputable def encodedTarget
    (q : ℕ) (P : G) (order_eq : addOrderOf P = q) (w : ZMod q) : G :=
  orderScalarMap q P order_eq w

/-- An encoded random sample `sP - tQ`, expressed without choosing integer
representatives: it is the subgroup image of `s - w*t`. -/
noncomputable def encodedSample
    (q : ℕ) (P : G) (order_eq : addOrderOf P = q)
    (w s t : ZMod q) : G :=
  orderScalarMap q P order_eq (s - w * t)

/-- Integer scalar multiplication agrees with the residue encoding.  This is
the literal `sP - tQ` bridge for integer samples. -/
theorem integer_sample_eq_encodedSample
    (q : ℕ) (P : G) (order_eq : addOrderOf P = q)
    (w : ZMod q) (s t : ℤ) :
    s • P - t • encodedTarget q P order_eq w =
      encodedSample q P order_eq w (s : ZMod q) (t : ZMod q) := by
  simp only [encodedTarget, encodedSample, map_sub]
  rw [orderScalarMap_intCast]
  have ht : orderScalarMap q P order_eq (w * (t : ZMod q)) =
      t • orderScalarMap q P order_eq w := by
    calc
      orderScalarMap q P order_eq (w * (t : ZMod q)) =
          orderScalarMap q P order_eq ((t : ZMod q) * w) := by rw [mul_comm]
      _ = t • orderScalarMap q P order_eq w := by
        rw [← map_zsmul]
        simp [zsmul_eq_mul]
  rw [ht]

/-- A group relation among encoded samples descends to the corresponding
scalar relation modulo `q`. -/
theorem scalar_relation_of_group_relation
    (q : ℕ) (P : G) (order_eq : addOrderOf P = q)
    (w : ZMod q) (s t : ι → ZMod q) (m : ι → ℤ)
    (group_relation :
      ∑ i, m i • encodedSample q P order_eq w (s i) (t i) = 0) :
    ∑ i, (m i : ZMod q) * (s i - w * t i) = 0 := by
  let φ := orderScalarMap q P order_eq
  have mapped : φ (∑ i, m i • (s i - w * t i)) = 0 := by
    calc
      φ (∑ i, m i • (s i - w * t i)) =
          ∑ i, φ (m i • (s i - w * t i)) := by rw [map_sum]
      _ = ∑ i, m i • φ (s i - w * t i) := by
        apply Finset.sum_congr rfl
        intro i _
        exact map_zsmul φ (m i) (s i - w * t i)
      _ = ∑ i, m i • encodedSample q P order_eq w (s i) (t i) := by
        rfl
      _ = 0 := group_relation
  have scalar_zsmul : ∑ i, m i • (s i - w * t i) = 0 := by
    apply orderScalarMap_injective q P order_eq
    simpa [φ] using mapped
  simpa [zsmul_eq_mul] using scalar_zsmul

/-- Because the scalar encoding is injective, the group relation and scalar
relation are in fact equivalent. -/
theorem group_relation_iff_scalar_relation
    (q : ℕ) (P : G) (order_eq : addOrderOf P = q)
    (w : ZMod q) (s t : ι → ZMod q) (m : ι → ℤ) :
    (∑ i, m i • encodedSample q P order_eq w (s i) (t i) = 0) ↔
      (∑ i, (m i : ZMod q) * (s i - w * t i) = 0) := by
  constructor
  · exact scalar_relation_of_group_relation q P order_eq w s t m
  · intro scalar_relation
    have scalar_zsmul : ∑ i, m i • (s i - w * t i) = 0 := by
      simpa [zsmul_eq_mul] using scalar_relation
    let φ := orderScalarMap q P order_eq
    calc
      ∑ i, m i • encodedSample q P order_eq w (s i) (t i) =
          ∑ i, m i • φ (s i - w * t i) := by rfl
      _ = ∑ i, φ (m i • (s i - w * t i)) := by
        apply Finset.sum_congr rfl
        intro i _
        exact (map_zsmul φ (m i) (s i - w * t i)).symm
      _ = φ (∑ i, m i • (s i - w * t i)) := by rw [map_sum]
      _ = 0 := by rw [scalar_zsmul, map_zero]

/-- Prime-order ECDLP extraction from an actual cyclic-group relation.

The denominator condition is indispensable.  The conclusion is the exact
discrete logarithm `w`, not merely existence of some scalar relation. -/
theorem extract_log_of_group_relation
    (q : ℕ) [q_prime : Fact q.Prime]
    (P : G) (order_eq : addOrderOf P = q)
    (w : ZMod q) (s t : ι → ZMod q) (m : ι → ℤ)
    (group_relation :
      ∑ i, m i • encodedSample q P order_eq w (s i) (t i) = 0)
    (useful : ∑ i, (m i : ZMod q) * t i ≠ 0) :
    w = (∑ i, (m i : ZMod q) * s i) /
      (∑ i, (m i : ZMod q) * t i) := by
  apply JKSSTAudit.useful_relation_extract w s t
    (fun i ↦ (m i : ZMod q))
  · exact scalar_relation_of_group_relation q P order_eq w s t m group_relation
  · exact useful

/-- If the ECDLP denominator vanishes, the group relation also has zero
numerator and therefore reveals no value of `w`. -/
theorem zero_denominator_group_relation_is_tautological
    (q : ℕ) [Fact q.Prime]
    (P : G) (order_eq : addOrderOf P = q)
    (w : ZMod q) (s t : ι → ZMod q) (m : ι → ℤ)
    (group_relation :
      ∑ i, m i • encodedSample q P order_eq w (s i) (t i) = 0)
    (zero_denominator : ∑ i, (m i : ZMod q) * t i = 0) :
    ∑ i, (m i : ZMod q) * s i = 0 := by
  apply JKSSTAudit.zero_denominator_is_tautological w s t
    (fun i ↦ (m i : ZMod q))
  · exact scalar_relation_of_group_relation q P order_eq w s t m group_relation
  · exact zero_denominator

end JKSSTCyclicECDLP
