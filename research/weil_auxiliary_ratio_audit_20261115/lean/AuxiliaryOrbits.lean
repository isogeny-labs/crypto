import AuxiliarySource

namespace WeilAuxiliaryRatioAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerFinitePointAudit MillerRecursionAudit FinitePointEvaluationAudit GeneralTranslationAudit
open NegationPullbackAudit
theorem quotient_flip_product {G : Type*} [CommGroup G] (a b c d : G) :
    (a*b/(c*d))*(d*c/(b*a))=1 := by
  calc
    (a*b/(c*d))*(d*c/(b*a))=(a*a⁻¹)*(b*b⁻¹)*(c*c⁻¹)*(d*d⁻¹) := by
      simp only [div_eq_mul_inv,mul_inv]
      ac_rfl
    _ =1 := by simp

theorem quotient_cycle_product {G : Type*} [CommGroup G] (a b c d e f : G) :
    (b*d/(a*e))*(c*f/(b*d))*(a*e/(c*f))=1 := by
  calc
    (b*d/(a*e))*(c*f/(b*d))*(a*e/(c*f))=
        (a*a⁻¹)*(b*b⁻¹)*(c*c⁻¹)*(d*d⁻¹)*(e*e⁻¹)*(f*f⁻¹) := by
      simp only [div_eq_mul_inv,mul_inv]
      ac_rfl
    _ =1 := by simp

variable {K : Type*} [Field K] [DecidableEq K]

def auxiliaryFactors (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P Q : W.Point) (f g : W.FunctionFieldˣ) : Finset W.FunctionFieldˣ := by
  classical
  exact {generalTranslatedUnit W hΔ Q f,negatedUnit W g,f,
    generalTranslatedUnit W hΔ (-P) (negatedUnit W g)}

theorem factors_admissible (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P Q S : W.Point) (f g : W.FunctionFieldˣ)
    (h : ∀ a∈auxiliaryFactors W hΔ P Q f g,pointOrder W a S=0) :
    AuxiliaryAdmissible W P Q f g S := by
  classical
  have ha := h (generalTranslatedUnit W hΔ Q f) (by simp [auxiliaryFactors])
  have hb := h (negatedUnit W g) (by simp [auxiliaryFactors])
  have hf := h f (by simp [auxiliaryFactors])
  have hd := h (generalTranslatedUnit W hΔ (-P) (negatedUnit W g)) (by simp [auxiliaryFactors])
  rw [general_translated_unit_coe,general_translation_order] at ha hd
  change pointOrder W (fieldNeg W (g : W.FunctionField)) S=0 at hb
  rw [point_order_pullback] at hb
  change pointOrder W (fieldNeg W (g : W.FunctionField)) (S+ -P)=0 at hd
  rw [point_order_pullback,show -(S+ -P)=P-S by abel] at hd
  exact ⟨ha,hb,hf,hd⟩

/-- An actual auxiliary point can simultaneously avoid every finite collection of translates. -/
theorem exists_translated_admissible [IsAlgClosed K]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (P Q : W.Point)
    (f g : W.FunctionFieldˣ) (T : Finset W.Point) :
    ∃ S : W.Point, ∀ R∈T,AuxiliaryAdmissible W P Q f g (S+R) := by
  classical
  let F := T.biUnion (fun R => (auxiliaryFactors W hΔ P Q f g).image (generalTranslatedUnit W hΔ R))
  obtain ⟨S,hS⟩ := exists_common_unit_point W hΔ F
  refine ⟨S,?_⟩
  intro R hR
  apply factors_admissible W hΔ P Q (S+R) f g
  intro a ha
  have hv := hS (generalTranslatedUnit W hΔ R a)
    (Finset.mem_biUnion.mpr ⟨R,hR,Finset.mem_image.mpr ⟨a,ha,rfl⟩⟩)
  rw [general_translated_unit_coe,general_translation_order] at hv
  exact hv

theorem diagonal_negative_admissible (W : WeierstrassCurve.Affine K) (P S : W.Point)
    (f : W.FunctionFieldˣ) (h : AuxiliaryAdmissible W P P f f S) :
    AuxiliaryAdmissible W P P f f (-S) := by
  refine ⟨?_,?_,h.2.1,?_⟩
  · simpa only [sub_eq_add_neg,add_comm] using h.2.2.2
  · simpa only [neg_neg] using h.2.2.1
  · simpa only [sub_neg_eq_add,add_comm] using h.1

theorem diagonal_negative_product (W : WeierstrassCurve.Affine K) (P S : W.Point)
    (f : W.FunctionFieldˣ) (h : AuxiliaryAdmissible W P P f f S) :
    auxiliaryValue W P P f f (-S) (diagonal_negative_admissible W P S f h)*
      auxiliaryValue W P P f f S h=1 := by
  unfold auxiliaryValue
  have h1 : -S+P=P-S := by abel
  have h2 : P- -S=S+P := by abel
  simp only [neg_neg,h1,h2]
  change (localUnitValue W (P-S) f h.2.2.2*localUnitValue W S f h.2.2.1/
    (localUnitValue W (-S) f h.2.1*localUnitValue W (S+P) f h.1))*
    (localUnitValue W (S+P) f h.1*localUnitValue W (-S) f h.2.1/
      (localUnitValue W S f h.2.2.1*localUnitValue W (P-S) f h.2.2.2))=1
  exact quotient_flip_product _ _ _ _

theorem diagonal_three_product (W : WeierstrassCurve.Affine K) (P S : W.Point)
    (f : W.FunctionFieldˣ) (ht : (3 : ℕ) • P=0)
    (h0 : AuxiliaryAdmissible W P P f f S)
    (h1 : AuxiliaryAdmissible W P P f f (S+P))
    (h2 : AuxiliaryAdmissible W P P f f (S-P)) :
    auxiliaryValue W P P f f S h0*auxiliaryValue W P P f f (S+P) h1*
      auxiliaryValue W P P f f (S-P) h2=1 := by
  have hp : P+P= -P := by
    apply eq_neg_iff_add_eq_zero.mpr
    simpa only [show (3 : ℕ)=2+1 by decide,add_nsmul,two_nsmul,one_nsmul] using ht
  have hs1 : (S+P)+P=S-P := by rw [add_assoc,hp];rfl
  have hs2 : (S-P)+P=S := sub_add_cancel S P
  have hn1 : -(S+P)= -S-P := by abel
  have hn2 : -(S-P)=P-S := by abel
  have hd1 : P-(S+P)= -S := by abel
  have hd2 : P-(S-P)= -S-P := by
    calc
      P-(S-P)= -S+(P+P) := by abel
      _ = -S+ -P := by rw [hp]
      _ = -S-P := rfl
  have hc : pointOrder W f (S-P)=0 := by simpa only [hs1] using h1.1
  have hf : pointOrder W f (-S-P)=0 := by simpa only [hn1] using h1.2.1
  unfold auxiliaryValue
  simp only [hs1,hs2,hn1,hn2,hd1,hd2]
  change (localUnitValue W (S+P) f h0.1*localUnitValue W (-S) f h0.2.1/
    (localUnitValue W S f h0.2.2.1*localUnitValue W (P-S) f h0.2.2.2))*
    (localUnitValue W (S-P) f hc*localUnitValue W (-S-P) f hf/
      (localUnitValue W (S+P) f h0.1*localUnitValue W (-S) f h0.2.1))*
    (localUnitValue W S f h0.2.2.1*localUnitValue W (P-S) f h0.2.2.2/
      (localUnitValue W (S-P) f hc*localUnitValue W (-S-P) f hf))=1
  exact quotient_cycle_product _ _ _ _ _ _

#print axioms exists_translated_admissible
#print axioms diagonal_negative_product
#print axioms diagonal_three_product
end
end WeilAuxiliaryRatioAudit
