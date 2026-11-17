import GeneralS4
import GeneralS3Controls
import DiemSmallField

/-! Original Semaev2004 p5 Remark1, exact source hash in source receipt.
Literal product parsing of the missing-plus continuation is distinguished
from the actual resultant and the corrected additive expression. -/
namespace SemaevBinaryFourthAudit
noncomputable section
open GeneralS4Audit GeneralS3Audit
variable {K : Type*} [Field K] [CharP K 2]

def binaryCurve (a : K) : WeierstrassCurve.Affine K := ⟨1,a,0,0,1⟩
def sigma1 (x y z w : K) := x+y+z+w
def sigma3 (x y z w : K) := x*y*z+x*y*w+x*z*w+y*z*w
def sigma4 (x y z w : K) := x*y*z*w

def correctedFourth (x y z w : K) : K :=
  (sigma1 x y z w)^4+(sigma3 x y z w)^4+
  sigma4 x y z w*(sigma3 x y z w+sigma1 x y z w)^2+
  (sigma4 x y z w)^2*(sigma1 x y z w)^2+(sigma3 x y z w)^2

def literalPrintedFourth (x y z w : K) : K :=
  (sigma1 x y z w)^4+(sigma3 x y z w)^4+
  sigma4 x y z w*(sigma3 x y z w+sigma1 x y z w)^2*
  (sigma4 x y z w)^2*(sigma1 x y z w)^2+(sigma3 x y z w)^2

theorem binary_resultant_coefficients (a x y z w : K) :
    generalS4 (binaryCurve a) x y z w=
      res22 ((x+y)^2) (x*y) ((x*y)^2+1)
        ((z+w)^2) (z*w) ((z*w)^2+1) := by
  have h4 : (4:K)=0 := by linear_combination 2*(CharTwo.two_eq_zero (R:=K))
  rw [generalS4_coeff]
  simp [qa,qb,qc,binaryCurve,WeierstrassCurve.b₂,WeierstrassCurve.b₄,
    WeierstrassCurve.b₆,WeierstrassCurve.b₈,h4,CharTwo.two_eq_zero,
    CharTwo.neg_eq,CharTwo.sub_eq_add]

theorem actual_resultant_corrected (a x y z w : K) :
    generalS4 (binaryCurve a) x y z w=correctedFourth x y z w := by
  rw [binary_resultant_coefficients]
  unfold correctedFourth sigma1 sigma3 sigma4 res22
  simp only [CharTwo.sub_eq_add,
    show (4:ℕ)=2*2 by decide,pow_mul,CharTwo.add_sq,mul_pow]
  ring_nf

#print axioms actual_resultant_corrected
end
end SemaevBinaryFourthAudit
