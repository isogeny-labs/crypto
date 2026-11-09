import S3Roots
import Mathlib.FieldTheory.IsAlgClosed.Basic

/-! Repeated-coordinate completion of the affine S3 root identity.
These statements are over an arbitrary field of characteristic other than two.
They are not a proof of the general resultant recursion or its degrees. -/
namespace TheoremAudit
open SemaevS3
variable {K : Type*} [Field K]

def doubleX (a x y : K) : K := ((3*x^2+a)/(2*y))^2-2*x

theorem S3_repeated_identity (a b x y z : K)
    (h : y^2=x^3+a*x+b) :
    S3 a b x x z = (3*x^2+a)^2-4*y^2*(z+2*x) := by
  unfold S3
  linear_combination (4*z+8*x)*h

theorem S3_repeated_factor (a b x y z : K)
    (h : y^2=x^3+a*x+b) (h2 : (2:K) ≠ 0) (hy : y ≠ 0) :
    S3 a b x x z = 4*y^2*(doubleX a x y-z) := by
  rw [S3_repeated_identity a b x y z h]
  unfold doubleX
  field_simp
  <;> ring

theorem S3_repeated_root_iff (a b x y z : K)
    (h : y^2=x^3+a*x+b) (h2 : (2:K) ≠ 0) (hy : y ≠ 0) :
    S3 a b x x z = 0 ↔ z=doubleX a x y := by
  rw [S3_repeated_factor a b x y z h h2 hy]
  have h4 : (4:K) ≠ 0 := by
    rw [show (4:K)=2*2 by ring]
    exact mul_ne_zero h2 h2
  have hc : (4:K)*y^2 ≠ 0 := mul_ne_zero h4 (pow_ne_zero 2 hy)
  rw [mul_eq_zero]
  simp only [hc, false_or, sub_eq_zero]
  exact eq_comm

theorem S3_two_torsion_no_root (a b x z : K)
    (h : (0:K)=x^3+a*x+b) (hnonsing : 3*x^2+a ≠ 0) :
    S3 a b x x z ≠ 0 := by
  have hi := S3_repeated_identity a b x 0 z (by simpa using h)
  simpa using hi.trans_ne (by simpa using pow_ne_zero 2 hnonsing)

theorem pointX_neg [DecidableEq K] (W : WeierstrassCurve.Affine K) (T : W.Point) :
    pointX (-T)=pointX T := by
  cases T with
  | zero => rfl
  | some x y h => rw [WeierstrassCurve.Affine.Point.neg_some]; rfl

theorem pointX_double [DecidableEq K] (W : WeierstrassCurve.Affine K)
    (ha1 : W.a₁=0) (ha2 : W.a₂=0) (ha3 : W.a₃=0)
    (h2 : (2:K) ≠ 0) {x y : K} (h : W.Nonsingular x y) (hy : y≠0) :
    pointX (WeierstrassCurve.Affine.Point.some x y h +
      WeierstrassCurve.Affine.Point.some x y h)=some (doubleX W.a₄ x y) := by
  have hn : W.negY x y = -y := by simp [WeierstrassCurve.Affine.negY, ha1,ha3]
  have hne : y ≠ W.negY x y := by
    rw [hn]
    intro he
    have hh : 2*y=0 := by linear_combination he
    exact (mul_ne_zero h2 hy) hh
  rw [WeierstrassCurve.Affine.Point.add_self_of_Y_ne hne]
  change some (W.addX x x (W.slope x x y y))=some (doubleX W.a₄ x y)
  congr 1
  rw [WeierstrassCurve.Affine.slope_of_Y_ne rfl hne]
  simp only [WeierstrassCurve.Affine.addX, ha1, ha2, hn,
    zero_mul, mul_zero, add_zero, sub_zero, zero_add]
  rw [show y- -y=2*y by ring]
  unfold doubleX
  ring

/-- Repeated-x equivalence, including the vertical tangent case, in the
actual Mathlib point group. -/
theorem S3_repeated_mathlib_iff [DecidableEq K]
    (W : WeierstrassCurve.Affine K)
    (ha1 : W.a₁=0) (ha2 : W.a₂=0) (ha3 : W.a₃=0)
    (h2 : (2:K) ≠ 0) {x y : K} (h : W.Nonsingular x y) (z : K) :
    S3 W.a₄ W.a₆ x x z=0 ↔
      pointX (WeierstrassCurve.Affine.Point.some x y h +
        WeierstrassCurve.Affine.Point.some x y h)=some z := by
  have heq : y^2=x^3+W.a₄*x+W.a₆ := by
    have he := h.1
    rw [WeierstrassCurve.Affine.equation_iff, ha1,ha2,ha3] at he
    linear_combination he
  by_cases hy : y=0
  · subst y
    have hn : 3*x^2+W.a₄ ≠ 0 := by
      have hh := (WeierstrassCurve.Affine.nonsingular_iff x 0).mp h
      simpa [ha1,ha2,ha3, eq_comm] using hh.2
    have hv : (0:K)=W.negY x 0 := by simp [WeierstrassCurve.Affine.negY,ha1,ha3]
    rw [WeierstrassCurve.Affine.Point.add_self_of_Y_eq hv]
    constructor
    · intro hs
      exact (S3_two_torsion_no_root W.a₄ W.a₆ x z (by simpa using heq) hn hs).elim
    · intro hs
      cases hs
  · rw [pointX_double W ha1 ha2 ha3 h2 h hy,
      S3_repeated_root_iff W.a₄ W.a₆ x y z heq h2 hy]
    exact ⟨fun hz => congrArg some hz.symm, fun hz => (Option.some.inj hz).symm⟩

-- The two signed sums below use Mathlib's actual point group.
theorem S3_signed_zero_sum_iff [DecidableEq K]
    (W : WeierstrassCurve.Affine K)
    (ha1 : W.a₁=0) (ha2 : W.a₂=0) (ha3 : W.a₃=0)
    {xR yR xF yF : K} (hR : W.Nonsingular xR yR)
    (hF : W.Nonsingular xF yF) (hx : xR ≠ xF) (z : K) :
    S3 W.a₄ W.a₆ xR xF z = 0 ↔
      (∃ T : W.Point, pointX T = some z ∧
        WeierstrassCurve.Affine.Point.some xR yR hR +
        WeierstrassCurve.Affine.Point.some xF yF hF + T = 0) ∨
      (∃ T : W.Point, pointX T = some z ∧
        WeierstrassCurve.Affine.Point.some xR yR hR +
        -WeierstrassCurve.Affine.Point.some xF yF hF + T = 0) := by
  have hEq : ∀ {x y : K}, W.Equation x y → y^2=x^3+W.a₄*x+W.a₆ := by
    intro x y h
    rw [WeierstrassCurve.Affine.equation_iff, ha1, ha2, ha3] at h
    linear_combination h
  have negX : ∀ T : W.Point, pointX (-T)=pointX T := by
    intro T
    cases T with
    | zero => rfl
    | some x y h => rw [WeierstrassCurve.Affine.Point.neg_some]; rfl
  have hp := pointX_add_eq_xPlus W ha1 ha2 hR hF hx
  have hm := pointX_add_neg_eq_xMinus W ha1 ha2 ha3 hR hF hx
  rw [S3_root_iff W.a₄ W.a₆ xR yR xF yF z hx (hEq hR.1) (hEq hF.1)]
  constructor
  · rintro (hz | hz)
    · left
      refine ⟨-(_+_), ?_, add_neg_cancel _⟩
      rw [negX]
      exact hp.trans (congrArg some hz.symm)
    · right
      refine ⟨-(_+_), ?_, add_neg_cancel _⟩
      rw [negX]
      exact hm.trans (congrArg some hz.symm)
  · rintro (⟨T,hT,hs⟩ | ⟨T,hT,hs⟩)
    · left
      have ht : T = -(_+_) := eq_neg_of_add_eq_zero_right hs
      rw [ht, negX, hp] at hT
      exact (Option.some.inj hT).symm
    · right
      have ht : T = -(_+_) := eq_neg_of_add_eq_zero_right hs
      rw [ht, negX, hm] at hT
      exact (Option.some.inj hT).symm

theorem zero_sum_pointX_iff [DecidableEq K] (W : WeierstrassCurve.Affine K)
    (P : W.Point) (z : K) :
    (∃ T : W.Point, pointX T=some z ∧ P+T=0) ↔ pointX P=some z := by
  constructor
  · rintro ⟨T,hT,hs⟩
    have ht : T = -P := eq_neg_of_add_eq_zero_right hs
    rwa [ht, pointX_neg] at hT
  · intro hp
    exact ⟨-P,(pointX_neg W P).trans hp,add_neg_cancel P⟩

/-- Full affine root criterion for fixed lifts of the first two x coordinates,
with no distinct-coordinate assumption. This is a statement about actual
elliptic-curve point sums, including the tangent and 2-torsion cases. -/
theorem S3_pointX_iff [DecidableEq K] (W : WeierstrassCurve.Affine K)
    (ha1 : W.a₁=0) (ha2 : W.a₂=0) (ha3 : W.a₃=0) (h2 : (2:K) ≠ 0)
    {xR yR xF yF : K} (hR : W.Nonsingular xR yR)
    (hF : W.Nonsingular xF yF) (z : K) :
    S3 W.a₄ W.a₆ xR xF z=0 ↔
      pointX (WeierstrassCurve.Affine.Point.some xR yR hR +
        WeierstrassCurve.Affine.Point.some xF yF hF)=some z ∨
      pointX (WeierstrassCurve.Affine.Point.some xR yR hR +
        -WeierstrassCurve.Affine.Point.some xF yF hF)=some z := by
  by_cases hx : xR=xF
  · subst xF
    have heq : ∀ {x y : K}, W.Equation x y → y^2=x^3+W.a₄*x+W.a₆ := by
      intro x y h
      rw [WeierstrassCurve.Affine.equation_iff,ha1,ha2,ha3] at h
      linear_combination h
    have hysq : yF^2=yR^2 := (heq hF.1).trans (heq hR.1).symm
    rcases (sq_eq_sq_iff_eq_or_eq_neg.mp hysq) with hy | hy
    · subst yF
      rw [S3_repeated_mathlib_iff W ha1 ha2 ha3 h2 hR z]
      simp only [add_neg_cancel,pointX,reduceCtorEq,or_false]
    · subst yF
      have hneg : WeierstrassCurve.Affine.Point.some xR (-yR) hF =
          -WeierstrassCurve.Affine.Point.some xR yR hR := by
        rw [WeierstrassCurve.Affine.Point.neg_some]
        simp only [WeierstrassCurve.Affine.negY,ha1,ha3,zero_mul,sub_zero]
      rw [hneg,neg_neg,add_neg_cancel]
      simp only [pointX,reduceCtorEq,false_or]
      exact S3_repeated_mathlib_iff W ha1 ha2 ha3 h2 hR z
  · rw [S3_signed_zero_sum_iff W ha1 ha2 ha3 hR hF hx z,
      zero_sum_pointX_iff,zero_sum_pointX_iff]

/-- Semaev's S3 equivalence over an algebraically closed field of
characteristic other than two, with all repeated-coordinate cases and
the actual elliptic group law. The three input points are affine. -/
theorem S3_zero_iff_exists_points [DecidableEq K] [IsAlgClosed K]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0)
    (ha1 : W.a₁=0) (ha2 : W.a₂=0) (ha3 : W.a₃=0) (h2 : (2:K) ≠ 0)
    (x1 x2 x3 : K) :
    S3 W.a₄ W.a₆ x1 x2 x3=0 ↔
      ∃ P Q R : W.Point, pointX P=some x1 ∧ pointX Q=some x2 ∧
        pointX R=some x3 ∧ P+Q+R=0 := by
  constructor
  · intro hs
    obtain ⟨y1,hy1⟩ := IsAlgClosed.exists_pow_nat_eq (x1^3+W.a₄*x1+W.a₆) (n:=2) (by decide)
    obtain ⟨y2,hy2⟩ := IsAlgClosed.exists_pow_nat_eq (x2^3+W.a₄*x2+W.a₆) (n:=2) (by decide)
    have he1 : W.Equation x1 y1 := by
      rw [WeierstrassCurve.Affine.equation_iff,ha1,ha2,ha3]
      simpa using hy1
    have he2 : W.Equation x2 y2 := by
      rw [WeierstrassCurve.Affine.equation_iff,ha1,ha2,ha3]
      simpa using hy2
    have h1 := (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero hΔ).mp he1
    have h2p := (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero hΔ).mp he2
    rcases (S3_pointX_iff W ha1 ha2 ha3 h2 h1 h2p x3).mp hs with hp | hm
    · refine ⟨.some x1 y1 h1,.some x2 y2 h2p,-(_+_),rfl,rfl,?_,add_neg_cancel _⟩
      exact (pointX_neg W _).trans hp
    · refine ⟨.some x1 y1 h1,-(.some x2 y2 h2p),-(_+_),rfl,?_,?_,add_neg_cancel _⟩
      · exact pointX_neg W _
      · exact (pointX_neg W _).trans hm
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
        apply (S3_pointX_iff W ha1 ha2 ha3 h2 hp hq x3).mpr
        left
        exact (zero_sum_pointX_iff W _ x3).mp ⟨R,hR,hs⟩
end TheoremAudit
