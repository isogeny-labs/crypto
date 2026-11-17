import RecursiveSemaevSeededTransport

/-! Initial-coordinate permutations and their actually relabeled canonical
S4 base polynomials. No cross-permutation symmetry of S4 is assumed. -/
namespace RecursiveSemaevInitialDegreesAudit
noncomputable section
open Polynomial S4DegreeAudit RecursiveSemaevDegreeAudit RecursiveSemaevFocusAudit
open RecursiveSemaevSeededTransportAudit
variable {K : Type*} [Field K]

set_option backward.isDefEq.respectTransparency false in
def seedZero (K : Type*) [Field K] : InputRing K 1 ≃+* InputRing K 1 :=
  (Polynomial.mapEquiv (R:=K[X][X]) (S:=K[X][X])
    (Polynomial.Bivariate.swap (R:=K)).toRingEquiv).trans
    (Polynomial.Bivariate.swap (R:=K[X])).toRingEquiv

set_option backward.isDefEq.respectTransparency false in
def seedOne (K : Type*) [Field K] : InputRing K 1 ≃+* InputRing K 1 :=
  (Polynomial.Bivariate.swap (R:=K[X])).toRingEquiv

def initialZero (K : Type*) [Field K] : InputRing K 1 := C (C (X:K[X]))
def initialOne (K : Type*) [Field K] : InputRing K 1 := C (X:K[X][X])

set_option backward.isDefEq.respectTransparency false in
theorem seedZero_apply (p : InputRing K 1) : seedZero K p=rotate (S:=K) p := rfl

set_option backward.isDefEq.respectTransparency false in
theorem seedZero_coordinate : seedZero K (initialZero K)=(X : Polynomial (InputRing K 0)) := by
  change rotate (S:=K) (C (C (X:K[X])))=(X:K[X][X][X])
  rw [rotate_scalar_polynomial (S:=K) (X:K[X]),Polynomial.map_X]

set_option backward.isDefEq.respectTransparency false in
theorem seedOne_coordinate : seedOne K (initialOne K)=(X : Polynomial (InputRing K 0)) :=
  Polynomial.Bivariate.swap_X

set_option backward.isDefEq.respectTransparency false in
theorem seedZero_scalar (a : K) : seedZero K (coefficientEmbedding 1 a)=coefficientEmbedding 1 a :=
  rotate_scalar a

set_option backward.isDefEq.respectTransparency false in
theorem seedOne_scalar (a : K) : seedOne K (coefficientEmbedding 1 a)=coefficientEmbedding 1 a :=
  Polynomial.Bivariate.swap_C_C (C a)

abbrev FourRing (K : Type*) [Field K] := Polynomial (InputRing K 1)
def v0 : FourRing K := C (C (C (X:K[X])))
def v1 : FourRing K := C (C (X:K[X][X]))
def v2 : FourRing K := C (X:Polynomial (InputRing K 0))
def v3 : FourRing K := X
def scalar4 (a : K) : FourRing K := C (C (C (C a)))

set_option backward.isDefEq.respectTransparency false in
theorem canonicalS4_ring (W : WeierstrassCurve.Affine K) :
    recursivePolynomial W 1=
      s4Ring (scalar4 W.b₂) (scalar4 W.b₄) (scalar4 W.b₆) (scalar4 W.b₈)
        v0 v1 v2 v3 := by
  let φ : PairRing K→+*FourRing K :=
    (C : Polynomial (PairRing K)→+*FourRing K).comp
      (C : PairRing K→+*Polynomial (PairRing K))
  have ha : (C (C (pairA (K:=K))) : FourRing K)=qaRing v0 v1 :=
    map_qa (A:=PairRing K) (B:=FourRing K) φ pairX pairY
  have hb : (C (C (pairB W)) : FourRing K)=
      qbRing (scalar4 W.b₂) (scalar4 W.b₄) (scalar4 W.b₆) v0 v1 :=
    map_qb (A:=PairRing K) (B:=FourRing K) φ
      (liftPair W.b₂) (liftPair W.b₄) (liftPair W.b₆) pairX pairY
  have hc : (C (C (pairC W)) : FourRing K)=
      qcRing (scalar4 W.b₄) (scalar4 W.b₆) (scalar4 W.b₈) v0 v1 :=
    map_qc (A:=PairRing K) (B:=FourRing K) φ
      (liftPair W.b₄) (liftPair W.b₆) (liftPair W.b₈) pairX pairY
  change ResultantDegreeStepAudit.resultantStep 2
    (quad (pairA (K:=K)) (pairB W) (pairC W))
    (liftPair W.b₂) (liftPair W.b₄) (liftPair W.b₆) (liftPair W.b₈)=_
  unfold ResultantDegreeStepAudit.resultantStep s4Ring
  rw [firstLift_quad (R:=PairRing K) (pairA (K:=K)) (pairB W) (pairC W)]
  have hq := (congrArg₂ (fun a b : FourRing K => quad a b (C (C (pairC W)))) ha hb).trans
    (congrArg (fun c : FourRing K =>
      quad (qaRing v0 v1) (qbRing (scalar4 W.b₂) (scalar4 W.b₄) (scalar4 W.b₆) v0 v1) c) hc)
  exact congrArg (fun p : Polynomial (FourRing K) => Polynomial.resultant p
    (ResultantDegreeStepAudit.secondLift
      (liftPair W.b₂) (liftPair W.b₄) (liftPair W.b₆) (liftPair W.b₈)) 2 2) hq

theorem s4Ring_map {A B : Type*} [CommRing A] [CommRing B] (φ : A→+*B)
    (b2 b4 b6 b8 x y z t : A) :
    φ (s4Ring b2 b4 b6 b8 x y z t)=
      s4Ring (φ b2) (φ b4) (φ b6) (φ b8) (φ x) (φ y) (φ z) (φ t) := by
  have hq (p q : A) :
      (quad (qaRing p q) (qbRing b2 b4 b6 p q) (qcRing b4 b6 b8 p q)).map φ=
        quad (qaRing (φ p) (φ q)) (qbRing (φ b2) (φ b4) (φ b6) (φ p) (φ q))
          (qcRing (φ b4) (φ b6) (φ b8) (φ p) (φ q)) := by
    rw [map_quad (A:=A) (B:=B) φ]
    exact (congrArg₂ (fun a b : B => quad a b (φ (qcRing b4 b6 b8 p q)))
      (map_qa (A:=A) (B:=B) φ p q) (map_qb (A:=A) (B:=B) φ b2 b4 b6 p q)).trans
      (congrArg (fun c : B =>
        quad (qaRing (φ p) (φ q)) (qbRing (φ b2) (φ b4) (φ b6) (φ p) (φ q)) c)
        (map_qc (A:=A) (B:=B) φ b4 b6 b8 p q))
  exact (Polynomial.resultant_map_map (R:=A) (S:=B)
    (quad (qaRing x y) (qbRing b2 b4 b6 x y) (qcRing b4 b6 b8 x y))
    (quad (qaRing z t) (qbRing b2 b4 b6 z t) (qcRing b4 b6 b8 z t)) 2 2 φ).symm.trans
    (congrArg₂ (fun f g : B[X] => Polynomial.resultant f g 2 2) (hq x y) (hq z t))

def reading (seed : InputRing K 1 ≃+* InputRing K 1) : FourRing K→+*FourRing K :=
  (Polynomial.Bivariate.swap (R:=InputRing K 0)).toRingHom.comp
    (Polynomial.mapRingHom seed.toRingHom)

set_option backward.isDefEq.respectTransparency false in
theorem reading_C (seed : InputRing K 1 ≃+* InputRing K 1) (a : InputRing K 1) :
    reading seed (C a)=Polynomial.Bivariate.swap (R:=InputRing K 0) (C (seed a)) := by
  change Polynomial.Bivariate.swap ((C a).map seed.toRingHom)=_
  rw [Polynomial.map_C]
  rfl

set_option backward.isDefEq.respectTransparency false in
theorem reading_X (seed : InputRing K 1 ≃+* InputRing K 1) : reading seed v3=(v2 : FourRing K) := by
  change Polynomial.Bivariate.swap ((X:FourRing K).map seed.toRingHom)=_
  rw [Polynomial.map_X]
  exact Polynomial.Bivariate.swap_Y

set_option backward.isDefEq.respectTransparency false in
theorem reading_of_image (seed : InputRing K 1 ≃+* InputRing K 1) (x y : InputRing K 1)
    (h : seed x=y) : reading seed (C x)=Polynomial.Bivariate.swap (R:=InputRing K 0) (C y) :=
  (reading_C seed x).trans (congrArg (fun z : InputRing K 1 =>
    Polynomial.Bivariate.swap (R:=InputRing K 0) (C z)) h)

set_option backward.isDefEq.respectTransparency false in
theorem seedZero_middle : seedZero K (initialOne K)=initialZero K := rotate_Z (S:=K)
set_option backward.isDefEq.respectTransparency false in
theorem seedZero_outer : seedZero K (X:Polynomial (InputRing K 0))=initialOne K := rotate_T (S:=K)
set_option backward.isDefEq.respectTransparency false in
theorem seedOne_inner : seedOne K (initialZero K)=initialZero K :=
  Polynomial.Bivariate.swap_C_C (X:K[X])
set_option backward.isDefEq.respectTransparency false in
theorem seedOne_outer : seedOne K (X:Polynomial (InputRing K 0))=initialOne K :=
  Polynomial.Bivariate.swap_Y

set_option backward.isDefEq.respectTransparency false in
theorem reading_zero_v0 : reading (seedZero K) (v0 : FourRing K)=v3 :=
  (reading_of_image (seedZero K) (initialZero K) (X:Polynomial (InputRing K 0))
    (seedZero_coordinate (K:=K))).trans (Polynomial.Bivariate.swap_X (R:=InputRing K 0))
set_option backward.isDefEq.respectTransparency false in
theorem reading_zero_v1 : reading (seedZero K) (v1 : FourRing K)=v0 :=
  (reading_of_image (seedZero K) (initialOne K) (initialZero K)
    (seedZero_middle (K:=K))).trans
    (Polynomial.Bivariate.swap_C_C (R:=InputRing K 0) (C (X:K[X])))
set_option backward.isDefEq.respectTransparency false in
theorem reading_zero_v2 : reading (seedZero K) (v2 : FourRing K)=v1 :=
  (reading_of_image (seedZero K) (X:Polynomial (InputRing K 0)) (initialOne K)
    (seedZero_outer (K:=K))).trans
    (Polynomial.Bivariate.swap_C_C (R:=InputRing K 0) (X:K[X][X]))
set_option backward.isDefEq.respectTransparency false in
theorem reading_one_v0 : reading (seedOne K) (v0 : FourRing K)=v0 :=
  (reading_of_image (seedOne K) (initialZero K) (initialZero K)
    (seedOne_inner (K:=K))).trans
    (Polynomial.Bivariate.swap_C_C (R:=InputRing K 0) (C (X:K[X])))
set_option backward.isDefEq.respectTransparency false in
theorem reading_one_v1 : reading (seedOne K) (v1 : FourRing K)=v3 :=
  (reading_of_image (seedOne K) (initialOne K) (X:Polynomial (InputRing K 0))
    (seedOne_coordinate (K:=K))).trans (Polynomial.Bivariate.swap_X (R:=InputRing K 0))
set_option backward.isDefEq.respectTransparency false in
theorem reading_one_v2 : reading (seedOne K) (v2 : FourRing K)=v1 :=
  (reading_of_image (seedOne K) (X:Polynomial (InputRing K 0)) (initialOne K)
    (seedOne_outer (K:=K))).trans
    (Polynomial.Bivariate.swap_C_C (R:=InputRing K 0) (X:K[X][X]))

set_option backward.isDefEq.respectTransparency false in
theorem reading_scalar (seed : InputRing K 1 ≃+* InputRing K 1)
    (hs : ∀ a : K,seed (coefficientEmbedding 1 a)=coefficientEmbedding 1 a) (a : K) :
    reading seed (scalar4 a)=scalar4 a :=
  (reading_of_image seed (coefficientEmbedding 1 a) (coefficientEmbedding 1 a) (hs a)).trans
    (Polynomial.Bivariate.swap_C_C (R:=InputRing K 0) (coefficientEmbedding 0 a))

set_option backward.isDefEq.respectTransparency false in
theorem reading_zero_S4 (W : WeierstrassCurve.Affine K) :
    reading (seedZero K) (recursivePolynomial W 1)=symbolicAt W 0 := by
  calc
    _=reading (seedZero K) (s4Ring (scalar4 W.b₂) (scalar4 W.b₄) (scalar4 W.b₆)
        (scalar4 W.b₈) v0 v1 v2 v3) := congrArg (reading (seedZero K)) (canonicalS4_ring W)
    _=s4Ring (scalar4 W.b₂) (scalar4 W.b₄) (scalar4 W.b₆) (scalar4 W.b₈) v3 v0 v1 v2 := by
      rw [s4Ring_map (A:=FourRing K) (B:=FourRing K) (reading (seedZero K))]
      simp only [reading_scalar (K:=K) (seedZero K) seedZero_scalar,
        reading_zero_v0 (K:=K),reading_zero_v1 (K:=K),reading_zero_v2 (K:=K),
        reading_X (K:=K) (seedZero K)]
    _=symbolicAt W 0 := rfl

set_option backward.isDefEq.respectTransparency false in
theorem reading_one_S4 (W : WeierstrassCurve.Affine K) :
    reading (seedOne K) (recursivePolynomial W 1)=symbolicAt W 1 := by
  calc
    _=reading (seedOne K) (s4Ring (scalar4 W.b₂) (scalar4 W.b₄) (scalar4 W.b₆)
        (scalar4 W.b₈) v0 v1 v2 v3) := congrArg (reading (seedOne K)) (canonicalS4_ring W)
    _=s4Ring (scalar4 W.b₂) (scalar4 W.b₄) (scalar4 W.b₆) (scalar4 W.b₈) v0 v3 v1 v2 := by
      rw [s4Ring_map (A:=FourRing K) (B:=FourRing K) (reading (seedOne K))]
      simp only [reading_scalar (K:=K) (seedOne K) seedOne_scalar,
        reading_one_v0 (K:=K),reading_one_v1 (K:=K),reading_one_v2 (K:=K),
        reading_X (K:=K) (seedOne K)]
    _=symbolicAt W 1 := rfl

set_option backward.isDefEq.respectTransparency false in
theorem first_base_degree (W : WeierstrassCurve.Affine K) :
    (Polynomial.Bivariate.swap (R:=InputRing K 0) (seededPolynomial W (seedZero K) 0)).natDegree=4 := by
  change (reading (seedZero K) (recursivePolynomial W 1)).natDegree=4
  rw [reading_zero_S4 (K:=K) W]
  exact all_four_symbolic_degrees W 0

set_option backward.isDefEq.respectTransparency false in
theorem second_base_degree (W : WeierstrassCurve.Affine K) :
    (Polynomial.Bivariate.swap (R:=InputRing K 0) (seededPolynomial W (seedOne K) 0)).natDegree=4 := by
  change (reading (seedOne K) (recursivePolynomial W 1)).natDegree=4
  rw [reading_one_S4 (K:=K) W]
  exact all_four_symbolic_degrees W 1

set_option backward.isDefEq.respectTransparency false in
theorem first_input_degree (W : WeierstrassCurve.Affine K) (n : ℕ) :
    (Polynomial.Bivariate.swap (R:=InputRing K n) (seededPolynomial W (seedZero K) n)).natDegree=2^(n+2) := by
  calc
    _=2^n*(Polynomial.Bivariate.swap (R:=InputRing K 0)
        (seededPolynomial W (seedZero K) 0)).natDegree := seeded_degree W (seedZero K) seedZero_scalar n
    _=2^n*4 := congrArg (fun d : ℕ=>2^n*d) (first_base_degree W)
    _=2^(n+2) := by rw [pow_add]; norm_num

set_option backward.isDefEq.respectTransparency false in
theorem second_input_degree (W : WeierstrassCurve.Affine K) (n : ℕ) :
    (Polynomial.Bivariate.swap (R:=InputRing K n) (seededPolynomial W (seedOne K) n)).natDegree=2^(n+2) := by
  calc
    _=2^n*(Polynomial.Bivariate.swap (R:=InputRing K 0)
        (seededPolynomial W (seedOne K) 0)).natDegree := seeded_degree W (seedOne K) seedOne_scalar n
    _=2^n*4 := congrArg (fun d : ℕ=>2^n*d) (second_base_degree W)
    _=2^(n+2) := by rw [pow_add]; norm_num

#print axioms first_input_degree
#print axioms second_input_degree
#print axioms reading_zero_S4
#print axioms reading_one_S4
#print axioms canonicalS4_ring
#print axioms s4Ring_map
#print axioms seedZero_coordinate
#print axioms seedOne_coordinate
end
end RecursiveSemaevInitialDegreesAudit
