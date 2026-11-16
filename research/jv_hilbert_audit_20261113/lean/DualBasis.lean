import SharpCutoff
import Mathlib.LinearAlgebra.Dimension.StrongRankCondition

namespace JVHilbertAudit
open MvPolynomial
noncomputable section
abbrev A := JVMacaulayAudit.CoordinateAlgebra
def quotientMap : Poly →ₐ[F] A := Ideal.Quotient.mkₐ F JVMacaulayAudit.sourceIdeal

def basisExponent : Fin 8 → Fin 3 →₀ ℕ :=
  ![exponent 0 0 0,exponent 1 0 0,exponent 0 1 0,exponent 0 0 1,
    exponent 1 0 1,exponent 0 1 1,exponent 0 0 2,exponent 0 0 3]
def basisPoly (i : Fin 8) : Poly := monomial (basisExponent i) 1
def basisVector (i : Fin 8) : A := quotientMap (basisPoly i)

/-- Explicit dual tests for (1,x,y,h,xh,yh,h²,h³). The inverse of the
degree1/degree2 pairing block is checked below, not postulated. -/
def dualTest : Fin 8 → Poly :=
  ![monomial (exponent 0 0 3) 1,
    monomial (exponent 1 0 1) 87+monomial (exponent 0 1 1) 23+monomial (exponent 0 0 2) 53,
    monomial (exponent 1 0 1) 23+monomial (exponent 0 1 1) 10+monomial (exponent 0 0 2) 15,
    monomial (exponent 1 0 1) 53+monomial (exponent 0 1 1) 15+monomial (exponent 0 0 2) 83,
    monomial (exponent 1 0 0) 87+monomial (exponent 0 1 0) 23+monomial (exponent 0 0 1) 53,
    monomial (exponent 1 0 0) 23+monomial (exponent 0 1 0) 10+monomial (exponent 0 0 1) 15,
    monomial (exponent 1 0 0) 53+monomial (exponent 0 1 0) 15+monomial (exponent 0 0 1) 83,
    monomial (exponent 0 0 0) 1]

def dualEvaluation (i : Fin 8) : Poly →ₗ[F] F where
  toFun p := cubicDual (p*dualTest i)
  map_add' p q := by rw [add_mul,map_add]
  map_smul' c p := by simp only [smul_mul_assoc,map_smul,RingHom.id_apply]

theorem dualEvaluation_apply (i : Fin 8) (p : Poly) :
    dualEvaluation i p=cubicDual (p*dualTest i) := rfl

theorem dualEvaluation_basis (i j : Fin 8) :
    dualEvaluation i (basisPoly j)=if i=j then 1 else 0 := by
  have : CharP F 101 := inferInstance
  fin_cases i <;> fin_cases j
  all_goals norm_num [dualEvaluation_apply,basisPoly,basisExponent,dualTest,
    cubicDual_apply,mul_add,AddMonoidAlgebra.coeff_add,Finsupp.add_apply,
    monomial_mul_monomial,coeff_monomial,exponent_add,exponent_eq]
  all_goals reduce_mod_char!

theorem dualEvaluation_vanishes_on_sourceIdeal (i : Fin 8) {p : Poly}
    (hp : p∈JVMacaulayAudit.sourceIdeal) : dualEvaluation i p=0 := by
  have h : ∀ g, cubicDual (g*p)=0 := sourceIdeal_le_dualAnnihilator hp
  simpa only [dualEvaluation_apply,mul_comm] using h (dualTest i)

def representative (v : Fin 8 → F) : Poly := ∑ i, v i • basisPoly i

theorem dualEvaluation_representative (i : Fin 8) (v : Fin 8 → F) :
    dualEvaluation i (representative v)=v i := by
  simp [representative,map_sum,map_smul,dualEvaluation_basis]

theorem basisVector_linearIndependent : LinearIndependent F basisVector := by
  rw [Fintype.linearIndependent_iff]
  intro v hv i
  have hq : quotientMap (representative v)=0 := by
    simpa [representative,basisVector,map_sum,map_smul] using hv
  have hm : representative v∈JVMacaulayAudit.sourceIdeal :=
    Ideal.Quotient.eq_zero_iff_mem.mp hq
  have he := dualEvaluation_vanishes_on_sourceIdeal i hm
  rwa [dualEvaluation_representative] at he

end
end JVHilbertAudit
