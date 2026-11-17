import StagedResultant
import GuardedModel

namespace GuardedReduction
noncomputable section
open MvPolynomial CubicArithmetic CubicNormalization GridLower FilteredClosure

variable {K : Type*} [Field K]

def missing (a b c : K) : P K := C (a+b+c)-X 1-X 2
def q1A (s : Coeff K) : Coeff K := fun i =>
  if i=0 then (s 0)^2-4*s 1 else if i=1 then 2*s 0 else -3
def q1B (s : Coeff K) (α β : K) : Coeff K := fun i =>
  if i=0 then -2*s 0*(s 1+α)-4*β+2*s 2 else
  if i=1 then 2*((s 0)^2+α) else -2*s 0
def q1D (s : Coeff K) (α β : K) : Coeff K := fun i =>
  if i=0 then (s 1-α)^2-s 0*s 2-4*β*s 0 else
  if i=1 then -s 0*s 1+s 2+2*s 0*α+4*β else s 1-2*α
def q2A (r : K) : Coeff K := fun i => if i=0 then r^2 else if i=1 then -2*r else 1
def q2B (α β r : K) : Coeff K := fun i =>
  if i=0 then -2*r*α-4*β else if i=1 then -2*(r^2+α) else -2*r
def q2D (α β r : K) : Coeff K := fun i =>
  if i=0 then α^2-4*β*r else if i=1 then -2*r*α-4*β else r^2

theorem missing_degree (a b c : K) : (missing a b c).totalDegree ≤ 1 := by
  have h1 := totalDegree_sub (C (a+b+c) : P K) (X 1)
  have h2 := totalDegree_sub (C (a+b+c)-X 1 : P K) (X 2)
  simp only [totalDegree_C,totalDegree_X] at h1 h2
  unfold missing
  omega

theorem cubic_membership (a b c : K) (t : P K) :
    cubic t (rootsCoefficients a b c) = GuardedModel.membership a b c t := by
  simp [cubic,rootsCoefficients,GuardedModel.membership,map_add,map_mul]
  ring

theorem missing_cubic_identity (a b c : K) :
    cubic (missing a b c) (rootsCoefficients a b c) =
      GuardedModel.membership a b c (X 2)+
      (missing a b c-X 2)*GuardedModel.pairDifference a b c := by
  simp [cubic,missing,rootsCoefficients,GuardedModel.membership,
    GuardedModel.pairDifference,map_add,map_mul]
  ring

theorem missing_cubic_derivable (α β a b c r : K) :
    Derivable (GuardedModel.inputs α β a b c r) 4
      (cubic (missing a b c) (rootsCoefficients a b c)) := by
  have hy : Derivable (GuardedModel.inputs α β a b c r) 4
      (GuardedModel.membership a b c (X 2)) :=
    .input (by simp [GuardedModel.inputs]) ((GuardedModel.membership_degree a b c 2).trans (by omega))
  have hQ := GuardedModel.pairDifference_derivable α β a b c r
  have hQd := GuardedModel.pairDifference_degree a b c
  have hwd := (totalDegree_sub (missing a b c) (X 2)).trans
    (max_le (missing_degree a b c) (by simp))
  rw [missing_cubic_identity]
  exact Derivable.add hy (Derivable.mul (missing a b c-X 2) hQ
    ((totalDegree_mul _ _).trans (by omega)))

theorem z_cubic_derivable (α β a b c r : K) :
    Derivable (GuardedModel.inputs α β a b c r) 4
      (cubic (X 3) (rootsCoefficients a b c)) := by
  rw [cubic_membership]
  exact .input (by simp [GuardedModel.inputs]) ((GuardedModel.membership_degree a b c 3).trans (by omega))

theorem odd_degree (α β : K) (x y u : P K)
    (hx : x.totalDegree ≤ 1) (hy : y.totalDegree ≤ 1) (hu : u.totalDegree ≤ 1) :
    (PrimeBitLocal.oddS3 (C α) (C β) x y u).totalDegree ≤ 4 := by
  have hn (n : ℕ) : (n : P K).totalDegree = 0 := by
    simpa using (totalDegree_C (n : K) : (C (n : K) : P K).totalDegree = 0)
  have hn2 : (2 : P K).totalDegree = 0 := by simpa only [map_ofNat] using (totalDegree_C (2 : K) : (C (2 : K) : P K).totalDegree = 0)
  have hn4 : (4 : P K).totalDegree = 0 := by simpa only [map_ofNat] using (totalDegree_C (4 : K) : (C (4 : K) : P K).totalDegree = 0)
  have ha : (C α : P K).totalDegree = 0 := totalDegree_C α
  have hb : (C β : P K).totalDegree = 0 := totalDegree_C β
  have hxy := totalDegree_mul x y
  have hsum := totalDegree_add x y
  have hsub := totalDegree_sub x y
  have hp := totalDegree_pow (x-y) 2
  have hp' := totalDegree_pow u 2
  have hmlead := totalDegree_mul ((x-y)^2) (u^2)
  have hadd := totalDegree_add (x*y) (C α)
  have hlin := totalDegree_mul (x+y) (x*y+C α)
  have hbeta := totalDegree_mul (2 : P K) (C β)
  have hcoeff := totalDegree_add ((x+y)*(x*y+C α)) (2*C β)
  have htw := totalDegree_mul (2 : P K) ((x+y)*(x*y+C α)+2*C β)
  have hmid := totalDegree_mul (2*((x+y)*(x*y+C α)+2*C β)) u
  have hconst := totalDegree_sub (x*y) (C α)
  have hconst2 := totalDegree_pow (x*y-C α) 2
  have htail0 := totalDegree_mul (4 : P K) (C β)
  have htail := totalDegree_mul (4*C β) (x+y)
  have h1 := totalDegree_sub ((x-y)^2*u^2) (2*((x+y)*(x*y+C α)+2*C β)*u)
  have h2 := totalDegree_add ((x-y)^2*u^2-2*((x+y)*(x*y+C α)+2*C β)*u) ((x*y-C α)^2)
  have h3 := totalDegree_sub ((x-y)^2*u^2-2*((x+y)*(x*y+C α)+2*C β)*u+(x*y-C α)^2)
    (4*C β*(x+y))
  unfold PrimeBitLocal.oddS3
  omega

def pairMultiplier (a b c α : K) : P K :=
  4*X 0^2+2*(X 1+X 2)*X 0-
    2*(missing a b c^2-C (a+b+c)*missing a b c+C (a*b+a*c+b*c)-C α)+
    GuardedModel.pairDifference a b c

theorem first_reduction_identity (α β a b c : K) :
    GuardedModel.first α β-
      family (missing a b c) (X 0) (q1A (rootsCoefficients a b c))
        (q1B (rootsCoefficients a b c) α β) (q1D (rootsCoefficients a b c) α β) =
      GuardedModel.pairDifference a b c*pairMultiplier a b c α+
      cubic (missing a b c) (rootsCoefficients a b c)*(2*X 0+missing a b c-C (a+b+c)) := by
  simp [GuardedModel.first, PrimeBitLocal.oddS3, family, poly, q1A, q1B, q1D,
    rootsCoefficients, pairMultiplier, cubic, missing, GuardedModel.pairDifference,
    map_add, map_mul, map_sub, map_pow, map_neg, map_ofNat]
  ring

theorem second_family (α β r : K) : GuardedModel.second α β r =
    family (X 3) (X 0) (q2A r) (q2B α β r) (q2D α β r) := by
  simp [GuardedModel.second,PrimeBitLocal.oddS3,family,poly,q2A,q2B,q2D,
    map_add,map_mul,map_neg,map_pow,map_sub,map_ofNat]
  ring

theorem pairMultiplier_degree (a b c α : K) : (pairMultiplier a b c α).totalDegree ≤ 2 := by
  let w : P K := missing a b c
  let M : P K := w^2-C (a+b+c)*w+C (a*b+a*c+b*c)
  have hw : w.totalDegree ≤ 1 := missing_degree a b c
  have hu : (X 0 : P K).totalDegree = 1 := totalDegree_X 0
  have hx : (X 1 : P K).totalDegree = 1 := totalDegree_X 1
  have hy : (X 2 : P K).totalDegree = 1 := totalDegree_X 2
  have h2 : (2 : P K).totalDegree = 0 := by simpa only [map_ofNat] using (totalDegree_C (2 : K) : (C (2 : K) : P K).totalDegree = 0)
  have h4 : (4 : P K).totalDegree = 0 := by simpa only [map_ofNat] using (totalDegree_C (4 : K) : (C (4 : K) : P K).totalDegree = 0)
  have hwp := totalDegree_pow w 2
  have hcw := totalDegree_mul (C (a+b+c) : P K) w
  have hws := totalDegree_sub (w^2) (C (a+b+c)*w)
  have hwa := totalDegree_add (w^2-C (a+b+c)*w) (C (a*b+a*c+b*c))
  simp only [totalDegree_C] at hcw hwa
  have hm : M.totalDegree ≤ 2 := by unfold M; omega
  have hxy := totalDegree_add (X 1 : P K) (X 2)
  have hup := totalDegree_pow (X 0 : P K) 2
  have ht0 := totalDegree_mul (4 : P K) (X 0^2)
  have ht1 := totalDegree_mul (2 : P K) (X 1+X 2)
  have ht1' := totalDegree_mul (2*(X 1+X 2) : P K) (X 0)
  have ht2 := totalDegree_sub M (C α)
  have ht2' := totalDegree_mul (2 : P K) (M-C α)
  have hsum := totalDegree_add (4*X 0^2 : P K) (2*(X 1+X 2)*X 0)
  have hsub := totalDegree_sub (4*X 0^2+2*(X 1+X 2)*X 0 : P K) (2*(M-C α))
  have hQ := GuardedModel.pairDifference_degree a b c
  have hfinal := totalDegree_add (4*X 0^2+2*(X 1+X 2)*X 0-2*(M-C α) : P K)
    (GuardedModel.pairDifference a b c)
  simp only [totalDegree_C] at ht2
  change (4*X 0^2+2*(X 1+X 2)*X 0-2*(M-C α)+GuardedModel.pairDifference a b c).totalDegree ≤ 2
  omega

theorem q1_derivable (α β a b c r : K) :
    Derivable (GuardedModel.inputs α β a b c r) 4
      (family (missing a b c) (X 0) (q1A (rootsCoefficients a b c))
        (q1B (rootsCoefficients a b c) α β) (q1D (rootsCoefficients a b c) α β)) := by
  have hf : Derivable (GuardedModel.inputs α β a b c r) 4 (GuardedModel.first α β) :=
    .input (by simp [GuardedModel.inputs]) (odd_degree α β (X 1) (X 2) (X 0)
      (by simp) (by simp) (by simp))
  have hQ := GuardedModel.pairDifference_derivable α β a b c r
  have hg := missing_cubic_derivable α β a b c r
  have hQd := GuardedModel.pairDifference_degree a b c
  have hTd := pairMultiplier_degree a b c α
  have hgd := cubic_degree (missing a b c) (rootsCoefficients a b c) (missing_degree a b c)
  have hL : (2*X 0+missing a b c-C (a+b+c) : P K).totalDegree ≤ 1 := by
    have h2 : (2 : P K).totalDegree = 0 := by simpa only [map_ofNat] using (totalDegree_C (2 : K) : (C (2 : K) : P K).totalDegree = 0)
    have hm := totalDegree_mul (2 : P K) (X 0)
    have hs := totalDegree_add (2*X 0 : P K) (missing a b c)
    have hd := totalDegree_sub (2*X 0+missing a b c : P K) (C (a+b+c))
    have hw := missing_degree a b c
    simp only [totalDegree_X,totalDegree_C] at hm hd
    omega
  have hQT : Derivable (GuardedModel.inputs α β a b c r) 4
      (GuardedModel.pairDifference a b c*pairMultiplier a b c α) := by
    rw [mul_comm]
    exact .mul _ hQ ((totalDegree_mul _ _).trans (by omega))
  have hgL : Derivable (GuardedModel.inputs α β a b c r) 4
      (cubic (missing a b c) (rootsCoefficients a b c)*(2*X 0+missing a b c-C (a+b+c))) := by
    rw [mul_comm]
    exact .mul _ hg ((totalDegree_mul _ _).trans (by omega))
  have hid := first_reduction_identity α β a b c
  have heq : GuardedModel.first α β-
      (GuardedModel.pairDifference a b c*pairMultiplier a b c α+
        cubic (missing a b c) (rootsCoefficients a b c)*(2*X 0+missing a b c-C (a+b+c))) =
      family (missing a b c) (X 0) (q1A (rootsCoefficients a b c))
        (q1B (rootsCoefficients a b c) α β) (q1D (rootsCoefficients a b c) α β) := by
    linear_combination hid
  rw [← heq]
  exact FilteredClosure.sub hf (Derivable.add hQT hgL)

theorem q2_derivable (α β a b c r : K) :
    Derivable (GuardedModel.inputs α β a b c r) 4
      (family (X 3) (X 0) (q2A r) (q2B α β r) (q2D α β r)) := by
  rw [← second_family]
  apply Derivable.input (by simp [GuardedModel.inputs])
  rw [second_family]
  exact family_degree (X 3) (X 0) _ _ _ (by simp) (by simp)

theorem q2A_value (r x : K) : val (q2A r) x = (x-r)^2 := by
  simp [val,q2A]; ring
theorem q2B_value (α β r x : K) : val (q2B α β r) x = -2*((x+r)*(x*r+α)+2*β) := by
  simp [val,q2B]; ring
theorem q2D_value (α β r x : K) : val (q2D α β r) x = (x*r-α)^2-4*β*(x+r) := by
  simp [val,q2D]; ring

end
end GuardedReduction
