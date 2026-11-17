import S3Completion
import Mathlib.Algebra.CharP.Two

/-! The ordinary characteristic-two S3 formula for
`y² + xy = x³ + a*x² + b`. No assertion about resultant recursion. -/
namespace BinaryS3Audit
open SemaevS3
variable {K : Type*} [Field K] [CharP K 2]

private theorem three_eq_one : (3:K)=1 := by
  linear_combination (CharTwo.two_eq_zero (R:=K))
private theorem four_eq_zero : (4:K)=0 := by
  linear_combination 2*(CharTwo.two_eq_zero (R:=K))
private theorem six_eq_zero : (6:K)=0 := by
  linear_combination 3*(CharTwo.two_eq_zero (R:=K))

local macro "binary_ring" : tactic =>
  `(tactic| (ring_nf; simp [CharTwo.two_eq_zero, three_eq_one, four_eq_zero,
    six_eq_zero,CharTwo.neg_eq,CharTwo.sub_eq_add]))

def binaryS3 (b x y z : K) : K := (x*y+x*z+y*z)^2+x*y*z+b

theorem binaryS3_quadratic (b x y z : K) :
    binaryS3 b x y z = (x+y)^2*z^2+x*y*z+(x*y)^2+b := by
  unfold binaryS3
  ring_nf
  simp [CharTwo.two_eq_zero]

def doubleX (a x y : K) : K := ((x^2+y)/x)^2+(x^2+y)/x+a

theorem repeated_identity (a b x y z : K)
    (h : y^2+x*y=x^3+a*x^2+b) (hx : x≠0) :
    binaryS3 b x x z = x^2*(doubleX a x y-z) := by
  unfold binaryS3 doubleX
  field_simp
  linear_combination (norm := binary_ring) h

theorem repeated_root_iff (a b x y z : K)
    (h : y^2+x*y=x^3+a*x^2+b) (hx : x≠0) :
    binaryS3 b x x z=0 ↔ z=doubleX a x y := by
  rw [repeated_identity a b x y z h hx,mul_eq_zero]
  simp only [pow_ne_zero 2 hx,false_or,sub_eq_zero]
  exact eq_comm

theorem zero_coordinate_no_root (b z : K) (hb : b≠0) :
    binaryS3 b 0 0 z≠0 := by simpa [binaryS3] using hb

def chordX (a x u y v : K) : K :=
  ((u+v)/(x+y))^2+(u+v)/(x+y)+x+y+a

/-- Eliminate the intercept of an actual nonvertical line through two
points of the binary curve. The third intersection has the sum's x. -/
theorem line_elimination (a b l n x y : K) (hx : x≠y)
    (hR : (l*x+n)^2+x*(l*x+n)=x^3+a*x^2+b)
    (hF : (l*y+n)^2+y*(l*y+n)=y^3+a*y^2+b) :
    binaryS3 b x y (l^2+l+x+y+a)=0 := by
  have hd : x+y≠0 := by simpa [← CharTwo.sub_eq_add] using sub_ne_zero.mpr hx
  have hn : n=x*y+(x+y)*(l^2+l+x+y+a) := by
    apply mul_left_cancel₀ hd
    linear_combination (norm := binary_ring) hR+hF
  have hs : x*y+x*(l^2+l+x+y+a)+y*(l^2+l+x+y+a)=n := by
    linear_combination (norm := binary_ring) hn
  unfold binaryS3
  rw [hs]
  linear_combination (norm := binary_ring) hR+x*hn

theorem chord_root (a b x u y v : K) (hx : x≠y)
    (hR : u^2+x*u=x^3+a*x^2+b)
    (hF : v^2+y*v=y^3+a*y^2+b) :
    binaryS3 b x y (chordX a x u y v)=0 := by
  have hd : x+y≠0 := by simpa [← CharTwo.sub_eq_add] using sub_ne_zero.mpr hx
  let l : K := (u+v)/(x+y)
  let n : K := u+l*x
  have hr : l*x+n=u := by
    dsimp [n]
    binary_ring
  have hf : l*y+n=v := by
    dsimp [l,n]
    field_simp
    binary_ring
  exact line_elimination a b l n x y hx (by rwa [hr]) (by rwa [hf])

theorem chord_neg_relation (a x u y v : K) (hx : x≠y) :
    chordX a x u y (v+y)=chordX a x u y v+x*y/(x+y)^2 := by
  have hd : x+y≠0 := by simpa [← CharTwo.sub_eq_add] using sub_ne_zero.mpr hx
  unfold chordX
  field_simp
  binary_ring

theorem quadratic_factor (b x y root z : K) (hx : x≠y)
    (hr : binaryS3 b x y root=0) :
    binaryS3 b x y z=(x+y)^2*(z-root)*(z-(root+x*y/(x+y)^2)) := by
  have hd : x+y≠0 := by simpa [← CharTwo.sub_eq_add] using sub_ne_zero.mpr hx
  unfold binaryS3 at hr ⊢
  field_simp
  linear_combination (norm := binary_ring) hr

theorem chord_root_iff (a b x u y v z : K) (hx : x≠y)
    (hR : u^2+x*u=x^3+a*x^2+b)
    (hF : v^2+y*v=y^3+a*y^2+b) :
    binaryS3 b x y z=0 ↔ z=chordX a x u y v ∨ z=chordX a x u y (v+y) := by
  have hd : x+y≠0 := by simpa [← CharTwo.sub_eq_add] using sub_ne_zero.mpr hx
  rw [quadratic_factor b x y _ z hx (chord_root a b x u y v hx hR hF),
    chord_neg_relation a x u y v hx]
  simp only [mul_eq_zero,pow_ne_zero 2 hd,false_or,sub_eq_zero]

theorem curve_equation (W : WeierstrassCurve.Affine K)
    (ha1 : W.a₁=1) (ha3 : W.a₃=0) (ha4 : W.a₄=0) {x y : K}
    (h : W.Equation x y) : y^2+x*y=x^3+W.a₂*x^2+W.a₆ := by
  simpa [WeierstrassCurve.Affine.equation_iff,ha1,ha3,ha4] using h

theorem negY_binary (W : WeierstrassCurve.Affine K)
    (ha1 : W.a₁=1) (ha3 : W.a₃=0) (x y : K) : W.negY x y=y+x := by
  simp [WeierstrassCurve.Affine.negY,ha1,ha3,CharTwo.sub_eq_add,CharTwo.neg_eq]

theorem pointX_chord [DecidableEq K] (W : WeierstrassCurve.Affine K)
    (ha1 : W.a₁=1) {x u y v : K} (hR : W.Nonsingular x u)
    (hF : W.Nonsingular y v) (hx : x≠y) :
    pointX (WeierstrassCurve.Affine.Point.some x u hR+
      WeierstrassCurve.Affine.Point.some y v hF)=some (chordX W.a₂ x u y v) := by
  rw [WeierstrassCurve.Affine.Point.add_of_X_ne hx]
  change some (W.addX x y (W.slope x y u v))=some (chordX W.a₂ x u y v)
  rw [WeierstrassCurve.Affine.slope_of_X_ne hx]
  simp only [WeierstrassCurve.Affine.addX,ha1,one_mul,CharTwo.sub_eq_add,chordX]
  congr 1
  ring

theorem pointX_double [DecidableEq K] (W : WeierstrassCurve.Affine K)
    (ha1 : W.a₁=1) (ha3 : W.a₃=0) (ha4 : W.a₄=0) {x y : K}
    (h : W.Nonsingular x y) (hx : x≠0) :
    pointX (WeierstrassCurve.Affine.Point.some x y h+
      WeierstrassCurve.Affine.Point.some x y h)=some (doubleX W.a₂ x y) := by
  have hne : y≠W.negY x y := by
    rw [negY_binary W ha1 ha3]
    intro he
    apply hx
    linear_combination (norm := binary_ring) he
  have hs : W.slope x x y y=(x^2+y)/x := by
    rw [WeierstrassCurve.Affine.slope_of_Y_ne rfl hne,negY_binary W ha1 ha3]
    congr 1
    · simp only [ha1,ha4,one_mul]
      binary_ring
    · binary_ring
  rw [WeierstrassCurve.Affine.Point.add_self_of_Y_ne hne]
  change some (W.addX x x (W.slope x x y y))=some (doubleX W.a₂ x y)
  rw [hs]
  congr 1
  unfold WeierstrassCurve.Affine.addX doubleX
  rw [ha1]
  binary_ring

theorem repeated_mathlib_iff [DecidableEq K] (W : WeierstrassCurve.Affine K)
    (ha1 : W.a₁=1) (ha3 : W.a₃=0) (ha4 : W.a₄=0) (hb : W.a₆≠0)
    {x y : K} (h : W.Nonsingular x y) (z : K) :
    binaryS3 W.a₆ x x z=0 ↔
      pointX (WeierstrassCurve.Affine.Point.some x y h+
        WeierstrassCurve.Affine.Point.some x y h)=some z := by
  by_cases hx : x=0
  · subst x
    have hv : y=W.negY 0 y := by rw [negY_binary W ha1 ha3]; simp
    rw [WeierstrassCurve.Affine.Point.add_self_of_Y_eq hv]
    simp only [pointX,reduceCtorEq]
    exact iff_false_intro (zero_coordinate_no_root W.a₆ z hb)
  · rw [pointX_double W ha1 ha3 ha4 h hx,
      repeated_root_iff W.a₂ W.a₆ x y z (curve_equation W ha1 ha3 ha4 h.1) hx]
    exact ⟨fun hz=>congrArg some hz.symm,fun hz=>(Option.some.inj hz).symm⟩

theorem binaryS3_pointX_iff [DecidableEq K] (W : WeierstrassCurve.Affine K)
    (ha1 : W.a₁=1) (ha3 : W.a₃=0) (ha4 : W.a₄=0) (hb : W.a₆≠0)
    {xR yR xF yF : K} (hR : W.Nonsingular xR yR)
    (hF : W.Nonsingular xF yF) (z : K) :
    binaryS3 W.a₆ xR xF z=0 ↔
      pointX (WeierstrassCurve.Affine.Point.some xR yR hR+
        WeierstrassCurve.Affine.Point.some xF yF hF)=some z ∨
      pointX (WeierstrassCurve.Affine.Point.some xR yR hR+
        -WeierstrassCurve.Affine.Point.some xF yF hF)=some z := by
  by_cases hx : xR=xF
  · subst xF
    have hy : (yF-yR)*(yF-(yR+xR))=0 := by
      linear_combination (norm := binary_ring)
        (curve_equation W ha1 ha3 ha4 hF.1)-(curve_equation W ha1 ha3 ha4 hR.1)
    rcases mul_eq_zero.mp hy with hy | hy
    · have hy' := sub_eq_zero.mp hy
      subst yF
      rw [repeated_mathlib_iff W ha1 ha3 ha4 hb hR z]
      simp only [add_neg_cancel,pointX,reduceCtorEq,or_false]
    · have hy' := sub_eq_zero.mp hy
      subst yF
      have hn : WeierstrassCurve.Affine.Point.some xR (yR+xR) hF=
          -WeierstrassCurve.Affine.Point.some xR yR hR := by
        rw [WeierstrassCurve.Affine.Point.neg_some]
        simp only [negY_binary W ha1 ha3]
      rw [hn,neg_neg,add_neg_cancel]
      simp only [pointX,reduceCtorEq,false_or]
      exact repeated_mathlib_iff W ha1 ha3 ha4 hb hR z
  · rw [chord_root_iff W.a₂ W.a₆ xR yR xF yF z hx
      (curve_equation W ha1 ha3 ha4 hR.1) (curve_equation W ha1 ha3 ha4 hF.1)]
    rw [pointX_chord W ha1 hR hF hx,WeierstrassCurve.Affine.Point.neg_some,
      pointX_chord W ha1 hR (WeierstrassCurve.Affine.nonsingular_neg .. |>.mpr hF) hx,
      negY_binary W ha1 ha3]
    simp only [Option.some.injEq]
    exact or_congr eq_comm eq_comm

theorem discriminant_binary (W : WeierstrassCurve.Affine K)
    (ha1 : W.a₁=1) (ha3 : W.a₃=0) (ha4 : W.a₄=0) : W.Δ=W.a₆ := by
  have h2 : W.b₂=1 := by simp [WeierstrassCurve.b₂,ha1,four_eq_zero]
  have h4 : W.b₄=0 := by simp [WeierstrassCurve.b₄,ha1,ha3,ha4]
  have h6 : W.b₆=0 := by simp [WeierstrassCurve.b₆,ha3,four_eq_zero]
  have h8 : W.b₈=W.a₆ := by simp [WeierstrassCurve.b₈,ha1,ha3,ha4,four_eq_zero]
  simp [WeierstrassCurve.Δ,h2,h4,h6,h8,CharTwo.neg_eq]

theorem exists_binary_y [IsAlgClosed K] (a b x : K) :
    ∃ y : K, y^2+x*y=x^3+a*x^2+b := by
  open Polynomial in
  let p : Polynomial K := X^2+C x*X-C (x^3+a*x^2+b)
  have hd : p.degree≠0 := by
    intro hh
    have hz : p.coeff 2=0 := Polynomial.coeff_eq_zero_of_degree_lt (by rw [hh]; norm_num)
    simp only [p,Polynomial.coeff_sub,Polynomial.coeff_add,
      Polynomial.coeff_X_pow,Polynomial.coeff_C_mul,Polynomial.coeff_X,
      Polynomial.coeff_C] at hz
    norm_num at hz
  obtain ⟨y,hy⟩ := IsAlgClosed.exists_root p hd
  refine ⟨y,?_⟩
  simpa [Polynomial.IsRoot,p,sub_eq_zero] using hy

/-- Full binary S3 equivalence on an ordinary binary Weierstrass model,
with actual affine points in Mathlib's elliptic-curve group. This includes
all repeated-coordinate and two-torsion cases. -/
theorem binaryS3_zero_iff_exists_points [DecidableEq K] [IsAlgClosed K]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (ha1 : W.a₁=1) (ha3 : W.a₃=0) (ha4 : W.a₄=0) (x1 x2 x3 : K) :
    binaryS3 W.a₆ x1 x2 x3=0 ↔
      ∃ P Q R : W.Point, pointX P=some x1 ∧ pointX Q=some x2 ∧
        pointX R=some x3 ∧ P+Q+R=0 := by
  have hb : W.a₆≠0 := by rwa [discriminant_binary W ha1 ha3 ha4] at hΔ
  constructor
  · intro hs
    obtain ⟨y1,hy1⟩ := exists_binary_y W.a₂ W.a₆ x1
    obtain ⟨y2,hy2⟩ := exists_binary_y W.a₂ W.a₆ x2
    have he1 : W.Equation x1 y1 := by
      simpa [WeierstrassCurve.Affine.equation_iff,ha1,ha3,ha4] using hy1
    have he2 : W.Equation x2 y2 := by
      simpa [WeierstrassCurve.Affine.equation_iff,ha1,ha3,ha4] using hy2
    have h1 := (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero hΔ).mp he1
    have h2 := (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero hΔ).mp he2
    rcases (binaryS3_pointX_iff W ha1 ha3 ha4 hb h1 h2 x3).mp hs with hp | hm
    · refine ⟨.some x1 y1 h1,.some x2 y2 h2,-(_+_),rfl,rfl,?_,add_neg_cancel _⟩
      exact (TheoremAudit.pointX_neg W _).trans hp
    · refine ⟨.some x1 y1 h1,-(.some x2 y2 h2),-(_+_),rfl,?_,?_,add_neg_cancel _⟩
      · exact TheoremAudit.pointX_neg W _
      · exact (TheoremAudit.pointX_neg W _).trans hm
  · rintro ⟨P,Q,R,hP,hQ,hR,hs⟩
    cases P with
    | zero => cases hP
    | some u v hp =>
      have hu : u=x1 := Option.some.inj hP
      subst u
      cases Q with
      | zero => cases hQ
      | some w t hq =>
        have hw : w=x2 := Option.some.inj hQ
        subst w
        apply (binaryS3_pointX_iff W ha1 ha3 ha4 hb hp hq x3).mpr
        left
        exact (TheoremAudit.zero_sum_pointX_iff W _ x3).mp ⟨R,hR,hs⟩

#print axioms binaryS3_pointX_iff
#print axioms binaryS3_zero_iff_exists_points

end BinaryS3Audit
