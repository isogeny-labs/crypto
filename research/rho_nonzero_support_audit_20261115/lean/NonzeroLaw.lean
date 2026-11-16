import ConditionedMean

namespace RhoNonzeroSupportAudit
noncomputable section
open scoped BigOperators
open RhoTwoChoiceCollisionAudit WalkWeightedGeneralAudit StaticHashCollisionAudit RhoFirstCollisionAudit

abbrev NonzeroEmbedding (p : ℕ) := Fin 2 ↪ {x : ZMod p // x ≠ 0}

def toNonzeroEmbedding {p : ℕ} (z : NonzeroSupport p) : NonzeroEmbedding p where
  toFun i := ⟨supportEmbedding z.val i, by
    change z.val.1 + (i.val : ZMod p) * z.val.2.val ≠ 0
    fin_cases i
    · simpa using z.property.1
    · simpa using z.property.2⟩
  inj' i j h := (supportEmbedding z.val).injective (congrArg Subtype.val h)

def nonzeroEquiv (p : ℕ) : NonzeroSupport p ≃ NonzeroEmbedding p where
  toFun := toNonzeroEmbedding
  invFun f := ⟨((f 0).val, ⟨(f 1).val - (f 0).val, sub_ne_zero.mpr (by
    intro h
    have h10 := f.injective (Subtype.ext h)
    norm_num at h10)⟩), (f 0).property, by simpa using (f 1).property⟩
  left_inv z := by
    apply Subtype.ext
    apply Prod.ext
    · change z.val.1 + ((0 : Fin 2).val : ZMod p) * z.val.2.val = z.val.1
      simp
    · apply Subtype.ext
      change (z.val.1 + ((1 : Fin 2).val : ZMod p) * z.val.2.val) -
        (z.val.1 + ((0 : Fin 2).val : ZMod p) * z.val.2.val) = z.val.2.val
      simp
  right_inv f := by
    apply Function.Embedding.ext
    intro i
    apply Subtype.ext
    change (f 0).val + (i.val : ZMod p) * ((f 1).val - (f 0).val) = (f i).val
    fin_cases i <;> simp

variable {p : ℕ} [Fact p.Prime]

def nonzeroWitness (hp : 2 < p) : NonzeroSupport p :=
  ⟨(1, ⟨1, one_ne_zero⟩), one_ne_zero, by
    intro h
    have h2 : (2 : ZMod p) = 0 := by simpa only [one_add_one_eq_two] using h
    have hd := (ZMod.natCast_eq_zero_iff 2 p).mp h2
    have hh := Nat.le_of_dvd (by decide : 0 < 2) hd
    omega⟩

instance nonemptyNonzeroSupport [Fact (2 < p)] : Nonempty (NonzeroSupport p) :=
  ⟨nonzeroWitness Fact.out⟩

instance nonemptyNonzeroEmbedding [Fact (2 < p)] : Nonempty (NonzeroEmbedding p) :=
  ⟨toNonzeroEmbedding (nonzeroWitness Fact.out)⟩

theorem nonzeroMean_embedding : nonzeroMean p =
    finiteMean (fun f : NonzeroEmbedding p => expectation fair
      (fun h : Hash (ZMod p) (Fin 2) =>
        (firstCollision p (fun x => x + (f (h x)).val) 0 : ℝ))) := by
  unfold nonzeroMean
  rw [← finiteMean_equiv (nonzeroEquiv p)]
  congr 1
  funext z
  unfold tableMean
  congr 1
  funext h
  congr 2
  funext x
  change x + z.val.1 + ((h x).val : ZMod p) * z.val.2.val =
    x + (z.val.1 + ((h x).val : ZMod p) * z.val.2.val)
  ring

theorem nonzeroMean_actual_law [Fact (2 < p)] : nonzeroMean p =
    ∑ f : NonzeroEmbedding p, (uniformLaw (NonzeroEmbedding p) f).toReal *
      ∑ h : Hash (ZMod p) (Fin 2), (hashLaw fair fair_nonneg fair_sum h).toReal *
        (firstCollision p (fun x => x + (f (h x)).val) 0 : ℝ) := by
  rw [nonzeroMean_embedding, finiteMean_actual_law]
  simp only [hashLaw_real_mass]
  rfl

theorem small_population_controls : Fintype.card (NonzeroSupport 2) = 0 ∧
    Fintype.card (NonzeroSupport 3) = 2 := by
  decide +kernel

#print axioms nonzeroMean_actual_law
#print axioms small_population_controls
end
end RhoNonzeroSupportAudit
