import S3Completion

/-! General Weierstrass S3, independently checked against Mathlib's actual
elliptic group. Source formula: Kosters–Yeo2015 Section2/Proposition2.1.
No characteristic-specific normal form or resultant-recursion claim. -/
namespace GeneralS3Audit
open SemaevS3
variable {K : Type*} [Field K]

def generalS3 (W : WeierstrassCurve.Affine K) (x y z : K) : K :=
  (x-y)^2*z^2-(2*x*y*(x+y)+W.b₂*x*y+W.b₄*(x+y)+W.b₆)*z+
    (x*y)^2-W.b₄*x*y-W.b₆*(x+y)-W.b₈

/-- The symmetric invariant formula from the source, in Vieta coordinates. -/
theorem generalS3_vieta (W : WeierstrassCurve.Affine K) (x y z : K) :
    generalS3 W x y z=(x*y+x*z+y*z)^2-
      (4*(x+y+z)+W.b₂)*(x*y*z)-W.b₄*(x*y+x*z+y*z)-
      W.b₆*(x+y+z)-W.b₈ := by
  unfold generalS3
  ring

theorem line_elimination (W : WeierstrassCurve.Affine K) (l n x y : K)
    (hx : x≠y) (hR : W.Equation x (l*x+n)) (hF : W.Equation y (l*y+n)) :
    generalS3 W x y (W.addX x y l)=0 := by
  rw [WeierstrassCurve.Affine.equation_iff] at hR hF
  have hd : x-y≠0 := sub_ne_zero.mpr hx
  have hs1 : x+y+W.addX x y l=l^2+W.a₁*l-W.a₂ := by
    unfold WeierstrassCurve.Affine.addX
    ring
  have hs2 : x*y+(x+y)*W.addX x y l=W.a₄-2*l*n-W.a₁*n-W.a₃*l := by
    apply mul_left_cancel₀ hd
    dsimp only [WeierstrassCurve.Affine.addX]
    linear_combination hR-hF
  have hs3 : x*y*W.addX x y l=n^2+W.a₃*n-W.a₆ := by
    dsimp only [WeierstrassCurve.Affine.addX] at hs2 ⊢
    linear_combination -hR+x*hs2
  have hs2' : x*y+x*W.addX x y l+y*W.addX x y l=
      W.a₄-2*l*n-W.a₁*n-W.a₃*l := by linear_combination hs2
  rw [generalS3_vieta,hs1,hs2',hs3]
  unfold WeierstrassCurve.b₂ WeierstrassCurve.b₄ WeierstrassCurve.b₆ WeierstrassCurve.b₈
  ring

def chordX (W : WeierstrassCurve.Affine K) (x u y v : K) : K :=
  W.addX x y ((u-v)/(x-y))

theorem chord_root (W : WeierstrassCurve.Affine K) (x u y v : K)
    (hx : x≠y) (hR : W.Equation x u) (hF : W.Equation y v) :
    generalS3 W x y (chordX W x u y v)=0 := by
  have hd : x-y≠0 := sub_ne_zero.mpr hx
  let l : K := (u-v)/(x-y)
  let n : K := u-l*x
  have hr : l*x+n=u := by dsimp [n]; ring
  have hf : l*y+n=v := by dsimp [l,n]; field_simp; ring
  exact line_elimination W l n x y hx (by rwa [hr]) (by rwa [hf])

theorem d2_mul_chordX (W : WeierstrassCurve.Affine K) (x u y v : K)
    (hx : x≠y) :
    (x-y)^2*chordX W x u y v=(u-v)^2+W.a₁*(u-v)*(x-y)-
      (W.a₂+x+y)*(x-y)^2 := by
  have hd : x-y≠0 := sub_ne_zero.mpr hx
  unfold chordX WeierstrassCurve.Affine.addX
  field_simp
  ring

theorem chord_roots_sum (W : WeierstrassCurve.Affine K) (x u y v : K)
    (hx : x≠y) (hR : W.Equation x u) (hF : W.Equation y v) :
    (x-y)^2*(chordX W x u y v+chordX W x u y (W.negY y v))=
      2*x*y*(x+y)+W.b₂*x*y+W.b₄*(x+y)+W.b₆ := by
  rw [mul_add,d2_mul_chordX W x u y v hx,
    d2_mul_chordX W x u y (W.negY y v) hx]
  rw [WeierstrassCurve.Affine.equation_iff] at hR hF
  simp only [WeierstrassCurve.Affine.negY,WeierstrassCurve.b₂,
    WeierstrassCurve.b₄,WeierstrassCurve.b₆]
  linear_combination 2*hR+2*hF

theorem quadratic_factor (a b c r s z : K)
    (hr : a*r^2+b*r+c=0) (hs : a*(r+s)+b=0) :
    a*z^2+b*z+c=a*(z-r)*(z-s) := by
  linear_combination hr+(z-r)*hs

theorem generalS3_factor (W : WeierstrassCurve.Affine K) (x u y v z : K)
    (hx : x≠y) (hR : W.Equation x u) (hF : W.Equation y v) :
    generalS3 W x y z=(x-y)^2*(z-chordX W x u y v)*
      (z-chordX W x u y (W.negY y v)) := by
  let a := (x-y)^2
  let b := -(2*x*y*(x+y)+W.b₂*x*y+W.b₄*(x+y)+W.b₆)
  let c := (x*y)^2-W.b₄*x*y-W.b₆*(x+y)-W.b₈
  let r := chordX W x u y v
  let s := chordX W x u y (W.negY y v)
  have hr : a*r^2+b*r+c=0 := by
    have he := chord_root W x u y v hx hR hF
    dsimp [a,b,c,r]
    unfold generalS3 at he
    linear_combination he
  have hs : a*(r+s)+b=0 := by
    have he := chord_roots_sum W x u y v hx hR hF
    dsimp only [WeierstrassCurve.Affine.negY] at he
    dsimp [a,b,r,s]
    linear_combination he
  calc
    generalS3 W x y z=a*z^2+b*z+c := by dsimp [a,b,c]; unfold generalS3; ring
    _=a*(z-r)*(z-s) := quadratic_factor a b c r s z hr hs

theorem generalS3_root_iff (W : WeierstrassCurve.Affine K) (x u y v z : K)
    (hx : x≠y) (hR : W.Equation x u) (hF : W.Equation y v) :
    generalS3 W x y z=0 ↔ z=chordX W x u y v ∨ z=chordX W x u y (W.negY y v) := by
  rw [generalS3_factor W x u y v z hx hR hF]
  simp only [mul_eq_zero,pow_ne_zero 2 (sub_ne_zero.mpr hx),false_or,sub_eq_zero]

theorem pointX_chord [DecidableEq K] (W : WeierstrassCurve.Affine K)
    {x u y v : K} (hR : W.Nonsingular x u) (hF : W.Nonsingular y v) (hx : x≠y) :
    pointX (WeierstrassCurve.Affine.Point.some x u hR+
      WeierstrassCurve.Affine.Point.some y v hF)=some (chordX W x u y v) := by
  rw [WeierstrassCurve.Affine.Point.add_of_X_ne hx]
  change some (W.addX x y (W.slope x y u v))=some (chordX W x u y v)
  rw [WeierstrassCurve.Affine.slope_of_X_ne hx]
  rfl

def tangentNum (W : WeierstrassCurve.Affine K) (x y : K) : K :=
  3*x^2+2*W.a₂*x+W.a₄-W.a₁*y
def tangentDen (W : WeierstrassCurve.Affine K) (x y : K) : K :=
  2*y+W.a₁*x+W.a₃
def doubleX (W : WeierstrassCurve.Affine K) (x y : K) : K :=
  W.addX x x (tangentNum W x y/tangentDen W x y)

theorem repeated_identity (W : WeierstrassCurve.Affine K) (x y z : K)
    (h : W.Equation x y) :
    generalS3 W x x z=(tangentNum W x y)^2+
      W.a₁*tangentNum W x y*tangentDen W x y-
      (tangentDen W x y)^2*(W.a₂+2*x+z) := by
  rw [WeierstrassCurve.Affine.equation_iff] at h
  unfold generalS3 tangentNum tangentDen
  simp only [WeierstrassCurve.b₂,WeierstrassCurve.b₄,WeierstrassCurve.b₆,
    WeierstrassCurve.b₈]
  linear_combination (W.a₁^2+4*(W.a₂+2*x+z))*h

theorem repeated_factor (W : WeierstrassCurve.Affine K) (x y z : K)
    (h : W.Equation x y) (hd : tangentDen W x y≠0) :
    generalS3 W x x z=(tangentDen W x y)^2*(doubleX W x y-z) := by
  rw [repeated_identity W x y z h]
  unfold doubleX WeierstrassCurve.Affine.addX
  field_simp
  ring

theorem repeated_root_iff (W : WeierstrassCurve.Affine K) (x y z : K)
    (h : W.Equation x y) (hd : tangentDen W x y≠0) :
    generalS3 W x x z=0 ↔ z=doubleX W x y := by
  rw [repeated_factor W x y z h hd]
  simp only [mul_eq_zero,pow_ne_zero 2 hd,false_or,sub_eq_zero]
  exact eq_comm

theorem vertical_no_root (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (hd : tangentDen W x y=0) (z : K) :
    generalS3 W x x z≠0 := by
  have ha : tangentNum W x y≠0 := by
    rcases (WeierstrassCurve.Affine.nonsingular_iff' x y).mp h |>.2 with hg | hg
    · intro hz
      apply hg
      unfold tangentNum at hz
      linear_combination -hz
    · exact False.elim (hg hd)
  rw [repeated_identity W x y z h.1,hd]
  simpa using pow_ne_zero 2 ha

theorem pointX_double [DecidableEq K] (W : WeierstrassCurve.Affine K)
    {x y : K} (h : W.Nonsingular x y) (hd : tangentDen W x y≠0) :
    pointX (WeierstrassCurve.Affine.Point.some x y h+
      WeierstrassCurve.Affine.Point.some x y h)=some (doubleX W x y) := by
  have hr : y-W.negY x y=tangentDen W x y := by
    unfold WeierstrassCurve.Affine.negY tangentDen
    ring
  have hn : y≠W.negY x y := by
    intro hy
    exact hd (hr.symm.trans (sub_eq_zero.mpr hy))
  have hs : W.slope x x y y=tangentNum W x y/tangentDen W x y := by
    rw [WeierstrassCurve.Affine.slope_of_Y_ne rfl hn,hr]
    rfl
  rw [WeierstrassCurve.Affine.Point.add_self_of_Y_ne hn]
  change some (W.addX x x (W.slope x x y y))=some (doubleX W x y)
  rw [hs]
  rfl

theorem repeated_mathlib_iff [DecidableEq K] (W : WeierstrassCurve.Affine K)
    {x y : K} (h : W.Nonsingular x y) (z : K) :
    generalS3 W x x z=0 ↔
      pointX (WeierstrassCurve.Affine.Point.some x y h+
        WeierstrassCurve.Affine.Point.some x y h)=some z := by
  by_cases hd : tangentDen W x y=0
  · have hv : y=W.negY x y := by
      apply sub_eq_zero.mp
      unfold WeierstrassCurve.Affine.negY tangentDen at *
      linear_combination hd
    rw [WeierstrassCurve.Affine.Point.add_self_of_Y_eq hv]
    simp only [pointX,reduceCtorEq]
    exact iff_false_intro (vertical_no_root W h hd z)
  · rw [pointX_double W h hd,repeated_root_iff W x y z h.1 hd]
    exact ⟨fun hz=>congrArg some hz.symm,fun hz=>(Option.some.inj hz).symm⟩

theorem generalS3_pointX_iff [DecidableEq K] (W : WeierstrassCurve.Affine K)
    {xR yR xF yF : K} (hR : W.Nonsingular xR yR)
    (hF : W.Nonsingular xF yF) (z : K) :
    generalS3 W xR xF z=0 ↔
      pointX (WeierstrassCurve.Affine.Point.some xR yR hR+
        WeierstrassCurve.Affine.Point.some xF yF hF)=some z ∨
      pointX (WeierstrassCurve.Affine.Point.some xR yR hR+
        -WeierstrassCurve.Affine.Point.some xF yF hF)=some z := by
  by_cases hx : xR=xF
  · subst xF
    have hy : (yF-yR)*(yF-W.negY xR yR)=0 := by
      have heF := hF.1
      have heR := hR.1
      rw [WeierstrassCurve.Affine.equation_iff] at heF heR
      unfold WeierstrassCurve.Affine.negY
      linear_combination heF-heR
    rcases mul_eq_zero.mp hy with hy | hy
    · have hy' := sub_eq_zero.mp hy
      subst yF
      rw [repeated_mathlib_iff W hR z]
      simp only [add_neg_cancel,pointX,reduceCtorEq,or_false]
    · have hy' := sub_eq_zero.mp hy
      subst yF
      have hn : WeierstrassCurve.Affine.Point.some xR (W.negY xR yR) hF=
          -WeierstrassCurve.Affine.Point.some xR yR hR := by
        rw [WeierstrassCurve.Affine.Point.neg_some]
      rw [hn,neg_neg,add_neg_cancel]
      simp only [pointX,reduceCtorEq,false_or]
      exact repeated_mathlib_iff W hR z
  · rw [generalS3_root_iff W xR yR xF yF z hx hR.1 hF.1,
      pointX_chord W hR hF hx,WeierstrassCurve.Affine.Point.neg_some,
      pointX_chord W hR (WeierstrassCurve.Affine.nonsingular_neg .. |>.mpr hF) hx]
    simp only [Option.some.injEq]
    exact or_congr eq_comm eq_comm

theorem exists_curve_y [IsAlgClosed K] (W : WeierstrassCurve.Affine K) (x : K) :
    ∃ y : K, W.Equation x y := by
  open Polynomial in
  let p : Polynomial K := X^2+C (W.a₁*x+W.a₃)*X-
    C (x^3+W.a₂*x^2+W.a₄*x+W.a₆)
  have hd : p.degree≠0 := by
    intro hh
    have hz : p.coeff 2=0 := Polynomial.coeff_eq_zero_of_degree_lt (by rw [hh]; norm_num)
    simp only [p,Polynomial.coeff_sub,Polynomial.coeff_add,Polynomial.coeff_X_pow,
      Polynomial.coeff_C_mul,Polynomial.coeff_X,Polynomial.coeff_C] at hz
    norm_num at hz
  obtain ⟨y,hy⟩ := IsAlgClosed.exists_root p hd
  have he : y^2+(W.a₁*x+W.a₃)*y-(x^3+W.a₂*x^2+W.a₄*x+W.a₆)=0 := by
    simpa [Polynomial.IsRoot,p] using hy
  refine ⟨y,?_⟩
  rw [WeierstrassCurve.Affine.equation_iff]
  linear_combination he

/-- S3 equivalence for the general Weierstrass model over an algebraically
closed field, in every characteristic. All three prescribed points are
affine; repeated coordinates and vertical tangents are fully included. -/
theorem generalS3_zero_iff_exists_points [DecidableEq K] [IsAlgClosed K]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (x1 x2 x3 : K) :
    generalS3 W x1 x2 x3=0 ↔
      ∃ P Q R : W.Point, pointX P=some x1 ∧ pointX Q=some x2 ∧
        pointX R=some x3 ∧ P+Q+R=0 := by
  constructor
  · intro hs
    obtain ⟨y1,hy1⟩ := exists_curve_y W x1
    obtain ⟨y2,hy2⟩ := exists_curve_y W x2
    have h1 := (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero hΔ).mp hy1
    have h2 := (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero hΔ).mp hy2
    rcases (generalS3_pointX_iff W h1 h2 x3).mp hs with hp | hm
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
        apply (generalS3_pointX_iff W hp hq x3).mpr
        left
        exact (TheoremAudit.zero_sum_pointX_iff W _ x3).mp ⟨R,hR,hs⟩

theorem generalS3_short (W : WeierstrassCurve.Affine K)
    (ha1 : W.a₁=0) (ha2 : W.a₂=0) (ha3 : W.a₃=0) (x y z : K) :
    generalS3 W x y z=SemaevS3.S3 W.a₄ W.a₆ x y z := by
  unfold generalS3 SemaevS3.S3
  simp only [WeierstrassCurve.b₂,WeierstrassCurve.b₄,WeierstrassCurve.b₆,
    WeierstrassCurve.b₈,ha1,ha2,ha3]
  ring

theorem generalS3_map {L : Type*} [Field L] (f : K →+* L)
    (W : WeierstrassCurve.Affine K) (x y z : K) :
    generalS3 (W.map f) (f x) (f y) (f z)=f (generalS3 W x y z) := by
  have h2 : f (2:K)=(2:L) := map_ofNat f 2
  simp [generalS3,WeierstrassCurve.map_b₂,WeierstrassCurve.map_b₄,
    WeierstrassCurve.map_b₆,WeierstrassCurve.map_b₈,h2]

/-- Necessity on an actual affine point killed by three. No algebraic
closure, global discriminant or characteristic hypothesis is needed. -/
theorem three_torsion_diagonal_zero [DecidableEq K] (W : WeierstrassCurve.Affine K)
    {x y : K} (h : W.Nonsingular x y)
    (ht : (3:ℕ) • WeierstrassCurve.Affine.Point.some x y h=0) :
    generalS3 W x x x=0 := by
  have hs : WeierstrassCurve.Affine.Point.some x y h+
      WeierstrassCurve.Affine.Point.some x y h+
      WeierstrassCurve.Affine.Point.some x y h=0 := by
    simpa only [succ_nsmul,zero_nsmul,zero_add] using ht
  apply (generalS3_pointX_iff W h h x).mpr
  left
  exact (TheoremAudit.zero_sum_pointX_iff W _ x).mp ⟨.some x y h,rfl,hs⟩

#print axioms generalS3_pointX_iff
#print axioms generalS3_zero_iff_exists_points

end GeneralS3Audit
