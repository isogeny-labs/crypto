import SmartLargeRecovery

/-! Concrete correctness of the fixed four-lift arithmetic program, bound to
actual Mathlib points. This does not assert a general p-adic transfer theorem
or a refinement of the native executable. -/
namespace SmartConcreteAudit
open AnomalousPrimeAudit AnomalousCurveDataAudit AnomalousPointTraceAudit
open SmartArithmeticAudit SmartSpecificationAudit SmartLargeRecoveryAudit

def baseA : ℕ := 425706413842211054102700238164133538302169176474
def baseB : ℕ := 203362936548826936673264444982866339953265530166
def gx : ℕ := 1
def gy : ℕ := 310536468939899693718962354338996655381367569020
def qx : ℕ := 259479713158273757050367190534858072594223227627
def qy : ℕ := 39414799304662693276020416800594933604368475811

def publicAttempt (i : Fin 4) : Option ℕ :=
  attempt p (baseA+i.val*p) (baseB+(2*i.val+1)*p) gx gy qx qy

theorem fixed_lift_recovery (i : Fin 4) : publicAttempt i=some knownScalar := by
  fin_cases i
  · exact attempt_0
  · exact attempt_1
  · exact attempt_2
  · exact attempt_3

theorem public_coordinates :
    ∃ (hG : formulaCurve.Nonsingular (gx:F) (gy:F))
      (hQ : formulaCurve.Nonsingular (qx:F) (qy:F)),
      G=.some (gx:F) (gy:F) hG ∧ Q=.some (qx:F) (qy:F) hQ := by
  have hG : formulaCurve.Nonsingular (gx:F) (gy:F) := by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve,gx,gy]
    unfold F p
    reduce_mod_char
  have hQ : formulaCurve.Nonsingular (qx:F) (qy:F) := by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve,qx,qy]
    unfold F p
    reduce_mod_char
  exact ⟨hG,hQ,rfl,rfl⟩

theorem returned_scalar_is_log (i : Fin 4) (d : ℕ)
    (h : publicAttempt i=some d) : d • G=Q := by
  rw [fixed_lift_recovery] at h
  have hd : knownScalar=d := Option.some.inj h
  rw [←hd]
  exact known_answer_relation

theorem concrete_attack_certificate :
    Nat.Prime p ∧ 2^159≤p ∧ p<2^160 ∧ formulaCurve.Δ≠0 ∧
    addOrderOf G=p ∧
    (∃ (hG : formulaCurve.Nonsingular (gx:F) (gy:F))
      (hQ : formulaCurve.Nonsingular (qx:F) (qy:F)),
      G=.some (gx:F) (gy:F) hG ∧ Q=.some (qx:F) (qy:F) hQ) ∧
    (∀ i : Fin 4, ∃ d : ℕ, publicAttempt i=some d ∧ d<p ∧ d • G=Q) ∧
    (binaryOps p).length=210 := by
  refine ⟨p_prime,p_bit_bounds.1,p_bit_bounds.2,formula_discriminant_ne_zero,
    G_order,public_coordinates,?_,large_binary_operation_count⟩
  intro i
  exact ⟨knownScalar,fixed_lift_recovery i,by decide +kernel,
    returned_scalar_is_log i knownScalar (fixed_lift_recovery i)⟩

#print axioms public_coordinates
#print axioms returned_scalar_is_log
#print axioms concrete_attack_certificate
end SmartConcreteAudit
