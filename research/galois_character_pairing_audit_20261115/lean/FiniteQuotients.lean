module
public import GaloisGenerator

@[expose] public section
namespace ProcyclicGaloisCocycleAudit
noncomputable section

variable (K : Type*) [Field K] [Fintype K]
variable (p : ℕ) [Fact p.Prime] [CharP K p] (n : ℕ) [NeZero n]

def extensionEmbedding : FiniteField.Extension K p n →ₐ[K] Omega K := IsAlgClosed.lift
def imageField : IntermediateField K (Omega K) := (extensionEmbedding K p n).fieldRange

instance imageFieldFinite : Finite (imageField K p n) :=
  Finite.of_equiv (FiniteField.Extension K p n) (extensionEmbedding K p n).equivFieldRange.toEquiv

theorem imageField_degree : Module.finrank K (imageField K p n) = n := by
  change Module.finrank K (extensionEmbedding K p n).fieldRange = n
  rw [← (extensionEmbedding K p n).equivFieldRange.toLinearEquiv.finrank_eq]
  exact FiniteField.finrank_extension K p n

def finiteFrobenius : Gal(imageField K p n / K) :=
  FiniteField.frobeniusAlgEquivOfAlgebraic K (imageField K p n)

theorem finiteFrobenius_generates (g : Gal(imageField K p n / K)) :
    g ∈ Subgroup.zpowers (finiteFrobenius K p n) := by
  obtain ⟨⟨i, _⟩, hi⟩ :=
    (FiniteField.bijective_frobeniusAlgEquivOfAlgebraic_pow K (imageField K p n)).surjective g
  exact ⟨(i : ℤ), by simpa [finiteFrobenius] using hi⟩

theorem finiteGalois_card : Nat.card Gal(imageField K p n / K) = n :=
  (IsGalois.card_aut_eq_finrank _ _).trans (imageField_degree K p n)

def finiteRestriction : Gamma K →ₜ* Gal(imageField K p n / K) where
  __ := AlgEquiv.restrictNormalHom (imageField K p n)
  continuous_toFun := InfiniteGalois.restrictNormalHom_continuous (imageField K p n)

theorem finiteRestriction_frobenius :
    finiteRestriction K p n (frobenius K) = finiteFrobenius K p n := by
  ext x
  have h := AlgEquiv.restrictNormal_commutes (frobenius K) (imageField K p n) x
  change algebraMap (imageField K p n) (Omega K)
    ((AlgEquiv.restrictNormal (frobenius K) (imageField K p n)) x) =
    (x : Omega K) ^ Fintype.card K
  exact h

def finiteCyclicEquiv : Multiplicative (ZMod n) ≃* Gal(imageField K p n / K) :=
  zmodMulEquivOfGenerator (finiteFrobenius_generates K p n) (finiteGalois_card K p n)

theorem finiteCyclicEquiv_one :
    finiteCyclicEquiv K p n (Multiplicative.ofAdd 1) = finiteFrobenius K p n := by
  exact zmodMulEquivOfGenerator_apply_ofAdd_one _ _

variable {K p n}

theorem finite_target_extension (H : Type*) [Group H] [Finite H]
    [TopologicalSpace H] [DiscreteTopology H] (h : H) :
    ∃ f : Gamma K →ₜ* H, f (frobenius K) = h := by
  let p := ringChar K
  let : Fact p.Prime := ⟨CharP.char_is_prime K p⟩
  let : NeZero (orderOf h) := ⟨(orderOf_pos h).ne'⟩
  let t : Subgroup.zpowers h := ⟨h, Subgroup.mem_zpowers h⟩
  have ht (x : Subgroup.zpowers h) : x ∈ Subgroup.zpowers t := by
    obtain ⟨i, hi⟩ := x.property
    exact ⟨i, Subtype.ext (by simpa [t] using hi)⟩
  let eh : Multiplicative (ZMod (orderOf h)) ≃* Subgroup.zpowers h :=
    zmodMulEquivOfGenerator ht (Nat.card_zpowers h)
  let e := (finiteCyclicEquiv K p (orderOf h)).symm.trans eh
  let g : Gal(imageField K p (orderOf h) / K) →ₜ* H := {
    toFun σ := (e σ).val
    map_one' := by simp
    map_mul' := by intro σ τ; simp
    continuous_toFun := continuous_of_discreteTopology }
  refine ⟨g.comp (finiteRestriction K p (orderOf h)), ?_⟩
  change ((eh ((finiteCyclicEquiv K p (orderOf h)).symm
    (finiteRestriction K p (orderOf h) (frobenius K)))) : H) = h
  rw [finiteRestriction_frobenius]
  have he : (finiteCyclicEquiv K p (orderOf h)).symm (finiteFrobenius K p (orderOf h)) =
      Multiplicative.ofAdd 1 := by
    rw [← finiteCyclicEquiv_one K p (orderOf h), MulEquiv.symm_apply_apply]
  rw [he]
  change ((zmodMulEquivOfGenerator ht (Nat.card_zpowers h) (Multiplicative.ofAdd 1)) : H) = h
  rw [zmodMulEquivOfGenerator_apply_ofAdd_one]

theorem finite_hom_evaluation_bijective (H : Type*) [Group H] [Finite H]
    [TopologicalSpace H] [DiscreteTopology H] :
    Function.Bijective (fun f : Gamma K →ₜ* H => f (frobenius K)) := by
  exact ⟨fun _ _ h => hom_eq_of_frobenius K _ _ h,
    fun h => finite_target_extension H h⟩

#print axioms imageField_degree
#print axioms finiteRestriction_frobenius
#print axioms finite_target_extension
#print axioms finite_hom_evaluation_bijective
end
end ProcyclicGaloisCocycleAudit
