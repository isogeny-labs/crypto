import CM8Transfer
/-! Actual finite F13 dual. The formulas are coordinate computations, not
logarithm tables. Finite tables below are proof witnesses only. -/
namespace CM8Dual
open CM8Transfer
open WeierstrassCurve.Affine
set_option maxRecDepth 1000000
set_option maxHeartbeats 0
def v (x : k) : k := 12*x-6
def dx (x : k) : k := 10*(v x+5+8/v x)+10
def dy (x y : k) : k := 12*y*(1-8/(v x)^2)
theorem denominator_nonzero : ∀ x : k, x≠7 → v x≠0 := by decide +kernel
theorem dual_equation : ∀ x y : k, y^2=x^3+4*x+6 → x≠7 →
    (dy x y)^2=(dx x)^3+dx x+4 := by decide +kernel
def dual : T.Point → E.Point
  | .zero => 0
  | .some x y h => if hx : x=7 then 0 else
      ep (dx x) (dy x y) (dual_equation x y (by
        simpa [equation_iff,T,TwistEmbedding.short] using h.left) hx)
def eCoord (x y : k) : E.Point := if h:y^2=x^3+x+4 then ep x y h else 0
def tCoord (x y : k) : T.Point := if h:y^2=x^3+4*x+6 then tp x y h else 0
def es : Fin 14 → E.Point := ![0,ep 0 2 (by decide),ep 0 11 (by decide),
 ep 2 1 (by decide),ep 2 12 (by decide),ep 5 2 (by decide),ep 5 11 (by decide),
 ep 7 4 (by decide),ep 7 9 (by decide),ep 8 2 (by decide),ep 8 11 (by decide),
 ep 9 1 (by decide),ep 9 12 (by decide),ep 10 0 (by decide)]
def ts : Fin 14 → T.Point := ![0,tp 2 3 (by decide),tp 2 10 (by decide),
 tp 6 5 (by decide),tp 6 8 (by decide),tp 7 0 (by decide),tp 8 2 (by decide),
 tp 8 11 (by decide),tp 9 2 (by decide),tp 9 11 (by decide),tp 11 4 (by decide),
 tp 11 9 (by decide),tp 12 1 (by decide),tp 12 12 (by decide)]
theorem e_coordinate_coverage : ∀ x y : k, ∃ i : Fin 14, eCoord x y=es i := by decide +kernel
theorem t_coordinate_coverage : ∀ x y : k, ∃ i : Fin 14, tCoord x y=ts i := by decide +kernel
theorem source_coverage (Q : E.Point) : ∃ i : Fin 14, Q=es i := by
  cases Q with
  | zero => exact ⟨0,rfl⟩
  | some x y h =>
    have he : y^2=x^3+x+4 := by simpa [equation_iff,E,TwistEmbedding.short] using h.left
    simpa [eCoord,he,ep] using e_coordinate_coverage x y
theorem target_coverage (R : T.Point) : ∃ i : Fin 14, R=ts i := by
  cases R with
  | zero => exact ⟨0,rfl⟩
  | some x y h =>
    have he : y^2=x^3+4*x+6 := by simpa [equation_iff,T,TwistEmbedding.short] using h.left
    simpa [tCoord,he,tp] using t_coordinate_coverage x y
theorem dual_phi_table : ∀ i : Fin 14, dual (phi (es i))=2 • es i := by decide +kernel
theorem phi_dual_table : ∀ i : Fin 14, phi (dual (ts i))=2 • ts i := by decide +kernel
theorem forward_add_table : ∀ i j : Fin 14,
    phi (es i+es j)=phi (es i)+phi (es j) := by decide +kernel
theorem dual_add_table : ∀ i j : Fin 14,
    dual (ts i+ts j)=dual (ts i)+dual (ts j) := by decide +kernel
theorem dual_phi (Q : E.Point) : dual (phi Q)=2 • Q := by
  obtain ⟨i,rfl⟩ := source_coverage Q
  exact dual_phi_table i
theorem phi_dual (R : T.Point) : phi (dual R)=2 • R := by
  obtain ⟨i,rfl⟩ := target_coverage R
  exact phi_dual_table i
theorem forward_add (Q R : E.Point) : phi (Q+R)=phi Q+phi R := by
  obtain ⟨i,rfl⟩ := source_coverage Q
  obtain ⟨j,rfl⟩ := source_coverage R
  exact forward_add_table i j
theorem dual_add (Q R : T.Point) : dual (Q+R)=dual Q+dual R := by
  obtain ⟨i,rfl⟩ := target_coverage Q
  obtain ⟨j,rfl⟩ := target_coverage R
  exact dual_add_table i j
def forwardHom : E.Point →+ T.Point :=
 {toFun := phi, map_zero' := rfl, map_add' := forward_add}
def dualHom : T.Point →+ E.Point :=
 {toFun := dual, map_zero' := rfl, map_add' := dual_add}
def inverse7 (R : T.Point) : E.Point := 4 • dual R
def TK : T.Point := tp 7 0 (by decide)
theorem kernel_exact : ∀ R : T.Point, dual R=0 ↔ R=0 ∨ R=TK := by
  intro R
  obtain ⟨i,rfl⟩ := target_coverage R
  have h : ∀ j : Fin 14, dual (ts j)=0 ↔ ts j=0 ∨ ts j=TK := by decide +kernel
  exact h i
theorem odd_annihilator_inverse {G H : Type*} [AddCommGroup G] [AddCommGroup H]
    (F : G →+ H) (D : H →+ G) (hcomp : ∀ P, D (F P)=2 • P)
    (r c : ℕ) (hc : 2*c=r+1) (P : G) (hP : r • P=0) : c • D (F P)=P := by
  calc
    c • D (F P) = (c*2) • P := by rw [hcomp,smul_smul]
    _ = (r+1) • P := by rw [Nat.mul_comm c 2,hc]
    _ = P := by simp [add_nsmul,hP]
theorem odd_source_scalar_reduction {G H : Type*} [AddCommGroup G] [AddCommGroup H]
    (F : G →+ H) (D : H →+ G) (hcomp : ∀ P, D (F P)=2 • P)
    (r c : ℕ) (hc : 2*c=r+1) (P Q : G) (hP : r • P=0) (hQ : r • Q=0)
    (n : ℕ) : F Q=n • F P ↔ Q=n • P := by
  constructor
  · intro he
    calc
      Q = c • D (F Q) := (odd_annihilator_inverse F D hcomp r c hc Q hQ).symm
      _ = c • D (n • F P) := by rw [he]
      _ = n • (c • D (F P)) := by rw [map_nsmul]; exact smul_comm c n _
      _ = n • P := by rw [odd_annihilator_inverse F D hcomp r c hc P hP]
  · intro he
    rw [he,map_nsmul]
theorem inverse7_left (Q : E.Point) (hQ : (7:ℕ) • Q=0) : inverse7 (phi Q)=Q := by
  exact odd_annihilator_inverse forwardHom dualHom dual_phi 7 4 (by decide) Q hQ
theorem inverse7_right (R : T.Point) (hR : (7:ℕ) • R=0) : phi (inverse7 R)=R := by
  change forwardHom (4 • dualHom R)=R
  rw [map_nsmul]
  exact odd_annihilator_inverse dualHom forwardHom phi_dual 7 4 (by decide) R hR
theorem actual_source_reduction (Q : E.Point) (hQ : (7:ℕ) • Q=0) (n : ℕ) :
    phi Q=n • U ↔ Q=n • P := by
  have h := odd_source_scalar_reduction forwardHom dualHom dual_phi 7 4
    (by decide) P Q CM8Transfer.source_order.1 hQ n
  change phi Q=n • phi P ↔ Q=n • P at h
  rw [generator_image] at h
  exact h
theorem two_torsion_exception : inverse7 TK=0 ∧ TK≠0 := by decide +kernel
#print axioms dual_phi
#print axioms phi_dual
#print axioms odd_annihilator_inverse
#print axioms odd_source_scalar_reduction
#print axioms actual_source_reduction
end CM8Dual
