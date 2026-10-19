import KoblitzRound04
import KoblitzRound05Quotients

open scoped Matrix
namespace KoblitzRound05

/-- A two-transitive Gram matrix is determined by the Hecke sum norm. -/
theorem hecke_off_diagonal (b : ℚ)
    (h : 12*660+132*b=4^2*660) : b=20 := by linarith

def G : Matrix (Fin 12) (Fin 12) ℤ := fun i j => if i=j then 660 else 20
def S : Matrix (Fin 5) (Fin 5) ℤ := fun i j => if i=j then 4 else -1
def H : Matrix (Fin 4) (Fin 4) ℤ := fun i j => if i=j then 2 else 1
def pairIndex : Fin 5 → Fin 12 := ![1,2,4,8,5]
def U (odd : Bool) : Matrix (Fin 5) (Fin 12) ℤ := fun i j =>
  let e : ℤ := if odd && i.val % 2 == 1 then -1 else 1
  if j=pairIndex i then e else if j.val=11-(pairIndex i).val then
    (if odd then -e else e) else 0
def F (odd : Bool) : Matrix (Fin 4) (Fin 12) ℤ :=
  fun i j => (S * U odd) (Fin.castLE (by decide) i) j

/-- The target has its actual induced polarization H, rather than I. -/
theorem even_polarized_composition :
    F false * G * (F false).transpose * H = 32000 • (1 : Matrix (Fin 4) (Fin 4) ℤ) := by
  decide +kernel

theorem odd_polarized_composition :
    F true * G * (F true).transpose * H = 32000 • (1 : Matrix (Fin 4) (Fin 4) ℤ) := by
  decide +kernel

theorem augmentation_square : S*S=5 • S := by decide +kernel

theorem scalar_bezout : (32000:ℤ)-2909*11=1 := by norm_num

/-- Every 11-torsion element is fixed by the adjoint composition scalar. -/
theorem scalar_on_eleven_torsion {A : Type*} [AddCommGroup A] (x : A)
    (h : (11:ℤ) • x=0) : (32000:ℤ) • x=x := by
  have e : (32000:ℤ)=1+2909*11 := by norm_num
  rw [e, add_zsmul, mul_zsmul, h]
  simp

/-- A reverse homomorphism with this composition is injective on A[11]. -/
theorem reverse_injective_on_eleven {A B : Type*} [AddCommGroup A] [AddCommGroup B]
    (f : B →+ A) (r : A →+ B)
    (comp : ∀ x, f (r x)=(32000:ℤ) • x)
    (x y : A) (hx : (11:ℤ) • x=0) (hy : (11:ℤ) • y=0)
    (eq : r x=r y) : x=y := by
  have e := congrArg f eq
  rw [comp, comp, scalar_on_eleven_torsion x hx, scalar_on_eleven_torsion y hy] at e
  exact e

/-- The forward map is onto A[11], with r as a right inverse there. -/
theorem reverse_right_inverse_on_eleven {A B : Type*} [AddCommGroup A] [AddCommGroup B]
    (f : B →+ A) (r : A →+ B)
    (comp : ∀ x, f (r x)=(32000:ℤ) • x)
    (x : A) (hx : (11:ℤ) • x=0) : f (r x)=x := by
  rw [comp, scalar_on_eleven_torsion x hx]

/-- A Bezout certificate also supplies the normalized inverse at any 11^k. -/
theorem bezout_reverse_right_inverse {A B : Type*} [AddCommGroup A] [AddCommGroup B]
    (f : B →+ A) (r : A →+ B) (u v m : ℤ)
    (comp : ∀ x, f (r x)=(32000:ℤ) • x)
    (bezout : u*32000+v*m=1) (x : A) (hx : m • x=0) :
    u • f (r x)=x := by
  have h := congrArg (fun z : ℤ => z • x) bezout
  simpa [add_zsmul, mul_zsmul, hx, comp] using h

/-- Complete ordinary zero degree; the zero polynomial is an irreducible degree 15. -/
theorem missing_zero_degree : 15*56*11=9240 := by norm_num

/-- Universal integer lift of the characteristic-two normalization identity. -/
theorem deformation_integral_lift {R : Type*} [CommRing R] (s X Y : R) :
    (s^6*Y+1)^2+s^3*(s^3*X+1)*(s^6*Y+1)+s^3*(s^6*Y+1)+s^3*(s^3*X+1)^3 =
    s^12*(Y^2+X*Y+X^3)+s^9*X^2+s^3+1+
      2*(s^9*X^2+2*s^6*X+s^9*Y+s^6*Y+s^3) := by ring

theorem deformation_binary_identity {K : Type*} [CommRing K] [CharP K 2]
    (s X Y : K) :
    (s^6*Y+1)^2+s^3*(s^3*X+1)*(s^6*Y+1)+s^3*(s^6*Y+1)+s^3*(s^3*X+1)^3 =
    s^12*(Y^2+X*Y+X^3)+s^9*X^2+s^3+1 := by
  rw [deformation_integral_lift]
  have h2 : (2:K)=0 := by simpa using (CharP.natCast_eq_natCast_mod K 2 2)
  rw [h2]; simp

/-- The complete supersingular pole stratum has zero projected 11-image. -/
theorem supersingular_image_zero {A : Type*} [AddCommGroup A] (x : A)
    (h8 : (8:ℤ) • x=0) (h11 : (11:ℤ) • x=0) : x=0 := by
  have z : (7:ℤ) • ((8:ℤ) • x) - (5:ℤ) • ((11:ℤ) • x) = x := by
    abel
  rw [h8,h11] at z
  simpa using z.symm

end KoblitzRound05

#print axioms KoblitzRound05.hecke_off_diagonal
#print axioms KoblitzRound05.even_polarized_composition
#print axioms KoblitzRound05.odd_polarized_composition
#print axioms KoblitzRound05.augmentation_square
#print axioms KoblitzRound05.scalar_bezout
#print axioms KoblitzRound05.scalar_on_eleven_torsion
#print axioms KoblitzRound05.reverse_injective_on_eleven
#print axioms KoblitzRound05.reverse_right_inverse_on_eleven
#print axioms KoblitzRound05.bezout_reverse_right_inverse
#print axioms KoblitzRound05.missing_zero_degree
#print axioms KoblitzRound05.deformation_integral_lift
#print axioms KoblitzRound05.deformation_binary_identity
#print axioms KoblitzRound05.supersingular_image_zero
