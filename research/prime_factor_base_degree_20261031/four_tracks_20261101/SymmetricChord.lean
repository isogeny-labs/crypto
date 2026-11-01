import Recoding
noncomputable section
namespace SymmetricChord
variable {K : Type*} [Field K]

def sigma (r m : K) := m^2-r
def pi (A r s m : K) := r^2+r*m^2+A+2*m*s
def disc (A r s m : K) := sigma r m^2-4*pi A r s m
def recoverX (r m d : K) := (sigma r m+d)/2
def recoverZ (r m d : K) := (sigma r m-d)/2
def h (a b c x t : K) := a*x^2+b*x+c+t
def qx (a b c r s e m d t : K) :=
  e*h a b c (recoverX r m d) t-m*(recoverX r m d-r)+s
def qz (a b c r s f m d u : K) :=
  f*h a b c (recoverZ r m d) u-m*(recoverZ r m d-r)+s
def H (A a b c r s m : K) :=
  a*(sigma r m^2-2*pi A r s m)+b*sigma r m+2*c
def a1 (a b r e f m : K) := (e-f)*(a*sigma r m+b)
def a2 (a b r e f m : K) := (e+f)*(a*sigma r m+b)-2*m
def b1 (A a b c r s e f m t u : K) :=
  (e+f)*H A a b c r s m+2*(e*t+f*u)-2*m*(sigma r m-2*r)+4*s
def b2 (A a b c r s e f m t u : K) :=
  (e-f)*H A a b c r s m+2*(e*t-f*u)

theorem recovery_sum (r m d : K) (h2 : (2 : K) ≠ 0) :
    m^2-recoverX r m d-recoverZ r m d-r = 0 := by
  unfold recoverX recoverZ sigma
  field_simp [h2]
  ring

theorem recovery_product (A r s m d : K) (h2 : (2 : K) ≠ 0) :
    4*(recoverX r m d*recoverZ r m d-r^2-r*m^2-A-2*m*s) =
      disc A r s m-d^2 := by
  unfold recoverX recoverZ disc sigma pi
  field_simp [h2]
  ring

theorem sum_identity (A a b c r s e f m d t u : K) (h2 : (2 : K) ≠ 0) :
    4*(qx a b c r s e m d t+qz a b c r s f m d u) =
      2*(a1 a b r e f m*d+b1 A a b c r s e f m t u)+
        a*(e+f)*(d^2-disc A r s m) := by
  unfold qx qz h recoverX recoverZ a1 b1 H disc sigma pi
  field_simp [h2]
  ring

theorem difference_identity (A a b c r s e f m d t u : K) (h2 : (2 : K) ≠ 0) :
    4*(qx a b c r s e m d t-qz a b c r s f m d u) =
      2*(a2 a b r e f m*d+b2 A a b c r s e f m t u)+
        a*(e-f)*(d^2-disc A r s m) := by
  unfold qx qz h recoverX recoverZ a2 b2 H disc sigma pi
  field_simp [h2]
  ring

/-- Both signs are arbitrary here; all four signed branches are included. -/
theorem system_iff (A a b c r s e f m d t u : K) (h2 : (2 : K) ≠ 0) :
    SecantGroupLaw.System A r s (recoverX r m d) (recoverZ r m d)
      (e*h a b c (recoverX r m d) t) (f*h a b c (recoverZ r m d) u) m ↔
      d^2 = disc A r s m ∧
      a1 a b r e f m*d+b1 A a b c r s e f m t u = 0 ∧
      a2 a b r e f m*d+b2 A a b c r s e f m t u = 0 := by
  have h4 : (4 : K) ≠ 0 := by
    have : (4 : K) = 2*2 := by ring
    rw [this]
    exact mul_ne_zero h2 h2
  have hp := recovery_product A r s m d h2
  have hs := sum_identity A a b c r s e f m d t u h2
  have hd := difference_identity A a b c r s e f m d t u h2
  change (_ = 0 ∧ _ = 0 ∧ qx a b c r s e m d t = 0 ∧
    qz a b c r s f m d u = 0) ↔ _
  constructor
  · rintro ⟨_, hg, hx, hz⟩
    have hn : d^2 = disc A r s m := by linear_combination hp-4*hg
    refine ⟨hn, ?_, ?_⟩
    · have hh : 2*(a1 a b r e f m*d+b1 A a b c r s e f m t u) = 0 := by
        linear_combination -hs+4*hx+4*hz-a*(e+f)*hn
      exact (mul_eq_zero.mp hh).resolve_left h2
    · have hh : 2*(a2 a b r e f m*d+b2 A a b c r s e f m t u) = 0 := by
        linear_combination -hd+4*hx-4*hz-a*(e-f)*hn
      exact (mul_eq_zero.mp hh).resolve_left h2
  · rintro ⟨hn, hl1, hl2⟩
    refine ⟨recovery_sum r m d h2, ?_, ?_⟩
    · have hh : 4*(recoverX r m d*recoverZ r m d-r^2-r*m^2-A-2*m*s) = 0 := by
        linear_combination hp-hn
      exact (mul_eq_zero.mp hh).resolve_left h4
    · have hsum : qx a b c r s e m d t+qz a b c r s f m d u = 0 := by
        have hh : 4*(qx a b c r s e m d t+qz a b c r s f m d u) = 0 := by
          linear_combination hs+2*hl1+a*(e+f)*hn
        exact (mul_eq_zero.mp hh).resolve_left h4
      have hdiff : qx a b c r s e m d t-qz a b c r s f m d u = 0 := by
        have hh : 4*(qx a b c r s e m d t-qz a b c r s f m d u) = 0 := by
          linear_combination hd+2*hl2+a*(e-f)*hn
        exact (mul_eq_zero.mp hh).resolve_left h4
      constructor
      · have hh : 2*qx a b c r s e m d t = 0 := by linear_combination hsum+hdiff
        exact (mul_eq_zero.mp hh).resolve_left h2
      · have hh : 2*qz a b c r s f m d u = 0 := by linear_combination hsum-hdiff
        exact (mul_eq_zero.mp hh).resolve_left h2

theorem recover_original (r m x z : K) (h2 : (2 : K) ≠ 0)
    (h : m^2-x-z-r = 0) :
    recoverX r m (x-z) = x ∧ recoverZ r m (x-z) = z := by
  unfold recoverX recoverZ sigma
  constructor <;> field_simp [h2] <;> linear_combination h

def LinearSquare (D a b c d : K) : Prop :=
  ∃ z : K, z^2 = D ∧ a*z+b = 0 ∧ c*z+d = 0

/-- Exact elimination on a nonzero pivot; rational root recovery is explicit. -/
theorem first_chart (D a b c d : K) (ha : a ≠ 0) :
    LinearSquare D a b c d ↔ b^2 = a^2*D ∧ a*d-c*b = 0 := by
  constructor
  · rintro ⟨z, hz, h1, h2⟩
    constructor
    · linear_combination a^2*hz-(a*z-b)*h1
    · linear_combination a*h2-c*h1
  · rintro ⟨hs, hc⟩
    refine ⟨-b/a, ?_, ?_, ?_⟩
    · field_simp [ha]
      linear_combination hs
    · field_simp [ha]
      ring
    · field_simp [ha]
      linear_combination hc

/-- No zero-pivot chart is discarded, and squareness is over K itself. -/
theorem complete_charts (D a b c d : K) :
    LinearSquare D a b c d ↔
      (a ≠ 0 ∧ b^2 = a^2*D ∧ a*d-c*b = 0) ∨
      (a = 0 ∧ c ≠ 0 ∧ d^2 = c^2*D ∧ c*b-a*d = 0) ∨
      (a = 0 ∧ c = 0 ∧ b = 0 ∧ d = 0 ∧ ∃ z : K, z^2 = D) := by
  by_cases ha : a = 0
  · by_cases hc : c = 0
    · simp [LinearSquare, ha, hc, and_assoc, and_left_comm, and_comm]
    · have hswap : LinearSquare D a b c d ↔ LinearSquare D c d a b := by
        simp only [LinearSquare, and_assoc, and_left_comm, and_comm]
      rw [hswap, first_chart D c d a b hc]
      simp [ha, hc]
  · rw [first_chart D a b c d ha]
    simp [ha]

end SymmetricChord

#print axioms SymmetricChord.system_iff
#print axioms SymmetricChord.recover_original
#print axioms SymmetricChord.first_chart
#print axioms SymmetricChord.complete_charts
