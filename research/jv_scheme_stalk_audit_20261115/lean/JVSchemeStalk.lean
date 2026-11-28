import JVLocalCertificate
import JVSchemeImports

namespace JVSchemeStalkAudit
noncomputable section
open JVHilbertAudit JVLocalMultiplicityAudit AlgebraicGeometry

def coneScheme : Scheme := AlgebraicGeometry.Spec (CommRingCat.of A)
def schemeOrigin : coneScheme := originPoint
abbrev OriginStalk := coneScheme.presheaf.stalk schemeOrigin

/-- The official structure-sheaf coefficient algebra on the actual stalk. -/
instance originStalkAlgebra : Algebra A OriginStalk :=
  StructureSheaf.stalkAlgebra A originPoint

/-- The official affine-scheme stalk comparison, applied to the actual origin. -/
def localizationStalkEquiv : OriginLocalization ≃ₐ[A] OriginStalk :=
  StructureSheaf.stalkIso A originPoint

def canonicalStalkEquiv : A ≃ₐ[A] OriginStalk :=
  originLocalEquiv.trans localizationStalkEquiv

theorem official_stalk_comparison : localizationStalkEquiv.toCommRingCatIso.symm =
    AlgebraicGeometry.Spec.stalkIso (CommRingCat.of A) originPoint := rfl

def actualGermMap : A →+* OriginStalk :=
  (coneScheme.presheaf.germ ⊤ schemeOrigin trivial).hom.comp
    (Scheme.ΓSpecIso (CommRingCat.of A)).inv.hom

theorem coefficient_is_actual_germ (a : A) :
    algebraMap A OriginStalk a = actualGermMap a := by
  have hc := AlgebraicGeometry.Spec.algebraMap_stalkIso_inv (R := CommRingCat.of A) originPoint
  have hv := congrArg (fun f => f a) hc
  change localizationStalkEquiv (algebraMap A OriginLocalization a) = actualGermMap a at hv
  exact (localizationStalkEquiv.commutes a).symm.trans hv

theorem canonical_coefficient (a : A) :
    canonicalStalkEquiv a = algebraMap A OriginStalk a :=
  canonicalStalkEquiv.commutes a

theorem canonical_map_bijective : Function.Bijective (algebraMap A OriginStalk) := by
  have he : (canonicalStalkEquiv : A → OriginStalk) = algebraMap A OriginStalk := by
    funext a
    exact canonical_coefficient a
  rw [← he]
  exact canonicalStalkEquiv.bijective

theorem actual_stalk_length : Module.length OriginStalk OriginStalk = 8 := by
  have hs : Module.length A OriginStalk = Module.length OriginStalk OriginStalk :=
    Module.length_eq_of_surjective (R := OriginStalk) (S := A) (M := OriginStalk)
      canonical_map_bijective.2
  calc
    Module.length OriginStalk OriginStalk = Module.length A OriginStalk := hs.symm
    _ = Module.length A A := canonicalStalkEquiv.toLinearEquiv.length_eq.symm
    _ = 8 := actual_ring_length

theorem actual_stalk_local : IsLocalRing OriginStalk := inferInstance

def stalkEvaluation : OriginStalk →+* F :=
  originEvaluation.toRingHom.comp canonicalStalkEquiv.symm.toRingHom

theorem stalk_evaluation_surjective : Function.Surjective stalkEvaluation :=
  origin_surjective.comp canonicalStalkEquiv.symm.surjective

theorem stalk_kernel_maximal : RingHom.ker stalkEvaluation = IsLocalRing.maximalIdeal OriginStalk :=
  IsLocalRing.ker_eq_maximalIdeal (R := OriginStalk) (K := F)
    stalkEvaluation stalk_evaluation_surjective

abbrev ActualStalkResidue := IsLocalRing.ResidueField OriginStalk

def stalkResidueEquiv : ActualStalkResidue ≃+* F :=
  (Ideal.quotEquivOfEq stalk_kernel_maximal.symm).trans
    (stalkEvaluation.quotientKerEquivOfSurjective stalk_evaluation_surjective)

theorem actual_scheme_not_reduced : ¬ AlgebraicGeometry.IsReduced coneScheme := by
  intro hr
  exact actual_not_reduced ((affine_isReduced_iff (CommRingCat.of A)).mp hr)

theorem actual_scheme_unique (p : coneScheme) : p = schemeOrigin := prime_unique p

theorem actual_scheme_origin_isolated : IsOpen ({schemeOrigin} : Set coneScheme) :=
  origin_isolated

#print axioms localizationStalkEquiv
#print axioms actual_stalk_length
#print axioms stalkResidueEquiv
#print axioms actual_scheme_not_reduced
end
end JVSchemeStalkAudit
