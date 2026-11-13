import DomainInterchange
import PrescribedDegree

namespace SemaevGeneralSplitAudit
open Polynomial ResultantInterchangeAudit SemaevReassociationAudit
noncomputable section
variable {R S : Type*} [CommRing R] [CommRing S]

def retainedY (p : Polynomial R) (q : Polynomial (Polynomial R)) (D dy : ℕ) : Polynomial R :=
  Polynomial.resultant (lift p) q D dy
def retainedX (q : Polynomial (Polynomial R)) (r : Polynomial R) (dx E : ℕ) : Polynomial R :=
  Polynomial.resultant (Polynomial.Bivariate.swap q) (lift r) dx E
def retainedLeft (p : Polynomial R) (q : Polynomial (Polynomial R)) (r : Polynomial R)
    (D E dx dy : ℕ) : R := Polynomial.resultant (retainedY p q D dy) r (D*dx) E
def retainedRight (p : Polynomial R) (q : Polynomial (Polynomial R)) (r : Polynomial R)
    (D E dx dy : ℕ) : R := Polynomial.resultant p (retainedX q r dx E) D (dy*E)

theorem retainedY_map (f : R →+* S) (p : Polynomial R) (q : Polynomial (Polynomial R))
    (D dy : ℕ) : retainedY (p.map f) (mapBiv f q) D dy=(retainedY p q D dy).map f := by
  unfold retainedY
  rw [lift_map]
  simpa only [mapBiv,Polynomial.coe_mapRingHom] using
    (Polynomial.resultant_map_map (lift p) q D dy (mapRingHom f))

theorem retainedX_map (f : R →+* S) (q : Polynomial (Polynomial R)) (r : Polynomial R)
    (dx E : ℕ) : retainedX (mapBiv f q) (r.map f) dx E=(retainedX q r dx E).map f := by
  unfold retainedX
  rw [swap_mapBiv,lift_map]
  simpa only [mapBiv,Polynomial.coe_mapRingHom] using
    (Polynomial.resultant_map_map (Polynomial.Bivariate.swap q) (lift r) dx E (mapRingHom f))

theorem retainedLeft_map (f : R →+* S) (p : Polynomial R) (q : Polynomial (Polynomial R))
    (r : Polynomial R) (D E dx dy : ℕ) :
    retainedLeft (p.map f) (mapBiv f q) (r.map f) D E dx dy=f (retainedLeft p q r D E dx dy) := by
  unfold retainedLeft
  rw [retainedY_map]
  exact Polynomial.resultant_map_map (retainedY p q D dy) r (D*dx) E f

theorem retainedRight_map (f : R →+* S) (p : Polynomial R) (q : Polynomial (Polynomial R))
    (r : Polynomial R) (D E dx dy : ℕ) :
    retainedRight (p.map f) (mapBiv f q) (r.map f) D E dx dy=f (retainedRight p q r D E dx dy) := by
  unfold retainedRight
  rw [retainedX_map]
  exact Polynomial.resultant_map_map p (retainedX q r dx E) D (dy*E) f

def auxScalar : R →+* Polynomial (Polynomial R) := C.comp C
def killAux : Polynomial (Polynomial R) →+* R :=
  (evalRingHom 0).comp (evalRingHom (0:Polynomial R))
def leftGeneric (p : Polynomial R) (D : ℕ) : Polynomial (Polynomial (Polynomial R)) :=
  (genericLeading p D).map C
def rightGeneric (r : Polynomial R) (E : ℕ) : Polynomial (Polynomial (Polynomial R)) :=
  genericLeading (r.map C) E

theorem killAux_scalar (a : R) : killAux (auxScalar a)=a := by simp [killAux,auxScalar]

theorem leftGeneric_degree [IsDomain R] (p : Polynomial R) (D : ℕ) (hp : p.natDegree≤D) :
    (leftGeneric p D).natDegree=D := by
  rw [leftGeneric,natDegree_map_eq_of_injective C_injective,genericLeading_degree p D hp]

theorem rightGeneric_degree [IsDomain R] (r : Polynomial R) (E : ℕ) (hr : r.natDegree≤E) :
    (rightGeneric r E).natDegree=E :=
  genericLeading_degree _ E (natDegree_map_le.trans hr)

theorem leftGeneric_specialize (p : Polynomial R) (D : ℕ) :
    (leftGeneric p D).map killAux=p := by
  unfold leftGeneric
  rw [Polynomial.map_map]
  have h : (killAux : Polynomial (Polynomial R) →+* R).comp C=evalRingHom 0 := by
    ext a <;> simp [killAux]
  rw [h,genericLeading_specialize]

theorem rightGeneric_specialize (r : Polynomial R) (E : ℕ) :
    (rightGeneric r E).map killAux=r := by
  unfold rightGeneric killAux
  rw [← Polynomial.map_map,genericLeading_specialize,Polynomial.map_map]
  have h : (evalRingHom (0:R)).comp C=RingHom.id R := by ext a; simp
  rw [h,Polynomial.map_id]

theorem auxBivariate_specialize (q : Polynomial (Polynomial R)) :
    mapBiv killAux (mapBiv auxScalar q)=q := by
  unfold mapBiv
  rw [Polynomial.map_map]
  have h : (mapRingHom (killAux : Polynomial (Polynomial R) →+* R)).comp
      (mapRingHom auxScalar)=RingHom.id (Polynomial R) := by
    ext p n <;> simp [killAux_scalar]
  rw [h,Polynomial.map_id]

/-- Both outer dimensions are retained, even for zero polynomials and
specializations that lose degree. -/
theorem fully_prescribed_interchange [IsDomain R]
    (p : Polynomial R) (q : Polynomial (Polynomial R)) (r : Polynomial R)
    (D E dx dy : ℕ) (hp : p.natDegree≤D) (hr : r.natDegree≤E)
    (hq : q.natDegree≤dy) (hx : (Polynomial.Bivariate.swap q).natDegree≤dx) :
    retainedLeft p q r D E dx dy=retainedRight p q r D E dx dy := by
  have h := fixed_interchange_domain (leftGeneric p D) (mapBiv auxScalar q)
    (rightGeneric r E) dx dy (natDegree_map_le.trans hq)
    (by rw [swap_mapBiv]; exact natDegree_map_le.trans hx)
  unfold leftFixed rightFixed innerY innerX at h
  rw [leftGeneric_degree p D hp,rightGeneric_degree r E hr] at h
  change retainedLeft (leftGeneric p D) (mapBiv auxScalar q) (rightGeneric r E) D E dx dy=
    retainedRight (leftGeneric p D) (mapBiv auxScalar q) (rightGeneric r E) D E dx dy at h
  have he := congrArg (killAux : Polynomial (Polynomial R) →+* R) h
  rw [← retainedLeft_map,← retainedRight_map,leftGeneric_specialize,
    rightGeneric_specialize,auxBivariate_specialize] at he
  exact he

#print axioms fully_prescribed_interchange
end
end SemaevGeneralSplitAudit
