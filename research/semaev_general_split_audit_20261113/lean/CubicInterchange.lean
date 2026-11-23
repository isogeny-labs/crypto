import RetainedInterchange
import CanonicalBridge

namespace SemaevGeneralSplitAudit
open Polynomial ResultantInterchangeAudit SemaevReassociationAudit
open ResultantDegreeStepAudit
noncomputable section
variable {R : Type*} [CommRing R]

/-- The actual prescribed S3 convolution, leaving its inner variable free. -/
def cubicConvolution (D : ℕ) (p : Polynomial R) (b2 b4 b6 b8 z : R) : Polynomial R :=
  retainedY p (s3Bi b2 b4 b6 b8 z) D 2

theorem cubicConvolution_as_step (D : ℕ) (p : Polynomial R) (b2 b4 b6 b8 z : R) :
    cubicConvolution D p b2 b4 b6 b8 z=
      (resultantStep D p b2 b4 b6 b8).map (evalRingHom z) := by
  exact (step_map_eval D p b2 b4 b6 b8 z).symm

theorem cubicConvolution_map {S : Type*} [CommRing S] (f : R →+* S)
    (D : ℕ) (p : Polynomial R) (b2 b4 b6 b8 z : R) :
    (cubicConvolution D p b2 b4 b6 b8 z).map f=
      cubicConvolution D (p.map f) (f b2) (f b4) (f b6) (f b8) (f z) := by
  unfold cubicConvolution
  rw [← retainedY_map,s3Bi_map]

/-- Formulate the sign identity over the abstract coefficient ring before
specializing to polynomial rings, preserving their canonical instances. -/
theorem quadratic_resultant_comm (p q : Polynomial R) (E : ℕ) :
    Polynomial.resultant p q 2 E=Polynomial.resultant q p E 2 := by
  rw [Polynomial.resultant_comm]
  simp [pow_mul]

/-- Exact S3 kernel transfer, with no hidden natural-degree reset, no
distinct-coordinate restriction and no unproved resultant sign choice. -/
theorem cubicConvolution_interchange [IsDomain R] (p r : Polynomial R)
    (D E : ℕ) (hp : p.natDegree≤D) (hr : r.natDegree≤E)
    (b2 b4 b6 b8 z : R) :
    Polynomial.resultant (cubicConvolution D p b2 b4 b6 b8 z) r (D*2) E=
      Polynomial.resultant p (cubicConvolution E r b2 b4 b6 b8 z) D (2*E) := by
  have h := fully_prescribed_interchange p (s3Bi b2 b4 b6 b8 z) r D E 2 2 hp hr
    (s3Bi_degree_le ..) (by rw [s3Bi_swap]; exact s3Bi_degree_le ..)
  unfold retainedLeft retainedRight retainedX at h
  rw [s3Bi_swap,quadratic_resultant_comm] at h
  exact h

#print axioms cubicConvolution_interchange
end
end SemaevGeneralSplitAudit
