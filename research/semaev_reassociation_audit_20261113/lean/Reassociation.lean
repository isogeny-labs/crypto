import ActualS3

namespace SemaevReassociationAudit
open Polynomial S4DegreeAudit ResultantDegreeStepAudit
open ResultantInterchangeAudit RecursiveSemaevEvaluationAudit
noncomputable section
variable {R : Type*} [CommRing R]

def leftAssoc (D : ℕ) (p : Polynomial R) (b2 b4 b6 b8 x y z : R) : R :=
  Polynomial.resultant
    (Polynomial.resultant (lift p) (s3Bi b2 b4 b6 b8 z) D 2)
    (pairPoly b2 b4 b6 b8 x y) (D*2) 2

def rightAssoc (D : ℕ) (p : Polynomial R) (b2 b4 b6 b8 x y z : R) : R :=
  Polynomial.resultant p (lastPolynomial b2 b4 b6 b8 x y z) D 4

theorem lifted_pair (b2 b4 b6 b8 x y : R) :
    lift (pairPoly b2 b4 b6 b8 x y) =
      quad (C (qaRing x y)) (C (qbRing b2 b4 b6 x y))
        (C (qcRing b4 b6 b8 x y)) := by
  simp [lift, pairPoly, quad]

theorem innerX_actual_S4 [IsDomain R] (b2 b4 b6 b8 x y z : R) (h : x ≠ y) :
    innerX (s3Bi b2 b4 b6 b8 z) (pairPoly b2 b4 b6 b8 x y) 2 =
      lastPolynomial b2 b4 b6 b8 x y z := by
  unfold innerX
  rw [s3Bi_swap, pairPoly_degree b2 b4 b6 b8 x y h, resultant_comm]
  simp only [show (-1 : Polynomial R) ^ (2*2) = 1 by norm_num, one_mul]
  rw [lifted_pair]
  rfl

theorem reassociation_distinct [IsDomain R] (p : Polynomial R)
    (b2 b4 b6 b8 x y z : R) (h : x ≠ y) :
    leftAssoc p.natDegree p b2 b4 b6 b8 x y z =
      rightAssoc p.natDegree p b2 b4 b6 b8 x y z := by
  have hi := fixed_interchange_domain p (s3Bi b2 b4 b6 b8 z)
    (pairPoly b2 b4 b6 b8 x y) 2 2 (s3Bi_degree_le ..)
    (by rw [s3Bi_swap]; exact s3Bi_degree_le ..)
  unfold leftFixed rightFixed at hi
  rw [pairPoly_degree b2 b4 b6 b8 x y h, innerX_actual_S4 b2 b4 b6 b8 x y z h] at hi
  exact hi

section Mapping
variable {S : Type*} [CommRing S]

theorem quad_map (f : R →+* S) (a b c : R) :
    (quad a b c).map f = quad (f a) (f b) (f c) := by simp [quad]

theorem pairPoly_map (f : R →+* S) (b2 b4 b6 b8 x y : R) :
    (pairPoly b2 b4 b6 b8 x y).map f =
      pairPoly (f b2) (f b4) (f b6) (f b8) (f x) (f y) := by
  simp [pairPoly, quad, qaRing, qbRing, qcRing, map_ofNat]

theorem s3Bi_map (f : R →+* S) (b2 b4 b6 b8 z : R) :
    mapBiv f (s3Bi b2 b4 b6 b8 z) = s3Bi (f b2) (f b4) (f b6) (f b8) (f z) := by
  simp [mapBiv, s3Bi, quad, qaRing, qbRing, qcRing, map_ofNat]

theorem lastPolynomial_map (f : R →+* S) (b2 b4 b6 b8 x y z : R) :
    (lastPolynomial b2 b4 b6 b8 x y z).map f =
      lastPolynomial (f b2) (f b4) (f b6) (f b8) (f x) (f y) (f z) := by
  simp only [lastPolynomial, quartic, quadratic_resultant_ring]
  simp [resForm, qaRing, qbRing, qcRing, map_ofNat]

theorem leftAssoc_map (f : R →+* S) (D : ℕ) (p : Polynomial R)
    (b2 b4 b6 b8 x y z : R) :
    f (leftAssoc D p b2 b4 b6 b8 x y z) =
      leftAssoc D (p.map f) (f b2) (f b4) (f b6) (f b8) (f x) (f y) (f z) := by
  unfold leftAssoc
  rw [← resultant_map_map _ _ _ _ f, pairPoly_map]
  change Polynomial.resultant
    ((mapRingHom f) (Polynomial.resultant (lift p) (s3Bi b2 b4 b6 b8 z) D 2))
    _ _ _ = _
  rw [← resultant_map_map _ _ _ _ (mapRingHom f)]
  change Polynomial.resultant
    (Polynomial.resultant (mapBiv f (lift p)) (mapBiv f (s3Bi b2 b4 b6 b8 z)) D 2)
    _ _ _ = _
  rw [← lift_map, s3Bi_map]

theorem rightAssoc_map (f : R →+* S) (D : ℕ) (p : Polynomial R)
    (b2 b4 b6 b8 x y z : R) :
    f (rightAssoc D p b2 b4 b6 b8 x y z) =
      rightAssoc D (p.map f) (f b2) (f b4) (f b6) (f b8) (f x) (f y) (f z) := by
  unfold rightAssoc
  rw [← resultant_map_map _ _ _ _ f, lastPolynomial_map]

end Mapping

#print axioms reassociation_distinct

end
end SemaevReassociationAudit
