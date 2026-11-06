import KoblitzRound18
import Mathlib.LinearAlgebra.Matrix.Nondegenerate
namespace KoblitzRound19

theorem weighted_map_identity {R : Type*} [CommRing R] (q x y a : R) :
    (q^3*y)^2+(q^2*x)*(q^3*y)*q+(q^2*x)^3+a*(q^2*x)^2*q^2+q^6
      = q^6*(y^2+x*y+x^3+a*x^2+1) := by ring

theorem orbit_product_rotation {R : Type*} [CommRing R] (a b c d e t : R) :
    (1+e*t)*(1+a*t)*(1+b*t)*(1+c*t)*(1+d*t)
      = (1+a*t)*(1+b*t)*(1+c*t)*(1+d*t)*(1+e*t) := by ring

theorem controlled_pole_map_cleared {K : Type*} [Field K]
    (q N M a : K) (hq : q ≠ 0) :
    q^6 * ((M/q^3)^2 + (N/q^2)*(M/q^3) + (N/q^2)^3 + a*(N/q^2)^2 + 1)
      = M^2 + N*M*q + N^3 + a*N^2*q^2 + q^6 := by
  field_simp

theorem elliptic_norm_identity {R : Type*} [CommRing R] (x y S a : R) :
    (y*S)^2 + x*S*(y*S) + S^2*(x^3+a*x^2+1)
      = S^2*(y^2+x*y+x^3+a*x^2+1) := by ring

theorem elliptic_norm_zero {R : Type*} [CommRing R] (x y S a : R)
    (hc : y^2+x*y+x^3+a*x^2+1=0) :
    (y*S)^2+x*S*(y*S)+S^2*(x^3+a*x^2+1)=0 := by
  rw [elliptic_norm_identity, hc, mul_zero]

theorem reduced_boundary_monic {K : Type*} [Field K]
    (h U tail : K) (hh : h=0) (he : U^19+h*tail=0) : U=0 := by
  have hu : U^19=0 := by simpa [hh] using he
  exact (pow_eq_zero_iff (by decide : 19 ≠ 0)).mp hu

theorem reduced_boundary_monic_power {K : Type*} [Field K]
    (h U tail : K) (n : Nat) (hn : n ≠ 0)
    (hh : h=0) (he : U^n+h*tail=0) : U=0 := by
  have hu : U^n=0 := by simpa [hh] using he
  exact (pow_eq_zero_iff hn).mp hu

theorem saturation_on_unit_chart {R : Type*} [CommRing R]
    (h F : R) (k : Nat) (hh : IsUnit h) (he : h^k*F=0) : F=0 := by
  exact (hh.pow k).mul_left_eq_zero.mp (by simpa [mul_comm] using he)

theorem determinant_vanishes_from_evaluation {K : Type*} [Field K]
    (n : Nat) (M : Matrix (Fin (n+1)) (Fin (n+1)) K)
    (v : Fin (n+1) → K) (hv : v 0=1) (hm : Matrix.vecMul v M=0) : M.det=0 := by
  by_contra hd
  have hz : v=0 := Matrix.eq_zero_of_vecMul_eq_zero hd hm
  have h0 := congrFun hz 0
  have ho : (1:K)=0 := by simpa [hv] using h0
  exact one_ne_zero ho

noncomputable def orbitPolynomial {R : Type*} [CommRing R] (a b c d e : R) : Polynomial R :=
  (1+Polynomial.C a*Polynomial.X)*(1+Polynomial.C b*Polynomial.X)*
  (1+Polynomial.C c*Polynomial.X)*(1+Polynomial.C d*Polynomial.X)*
  (1+Polynomial.C e*Polynomial.X)

theorem orbit_polynomial_rotation {R : Type*} [CommRing R] (a b c d e : R) :
    orbitPolynomial e a b c d = orbitPolynomial a b c d e := by
  unfold orbitPolynomial
  ring

theorem orbit_coefficient_rotation {R : Type*} [CommRing R]
    (a b c d e : R) (k : Nat) :
    (orbitPolynomial e a b c d).coeff k = (orbitPolynomial a b c d e).coeff k := by
  rw [orbit_polynomial_rotation]

theorem orbit_coefficient_field_map {R S : Type*} [CommRing R] [CommRing S]
    (f : R →+* S) (a b c d e : R) (k : Nat) :
    f ((orbitPolynomial a b c d e).coeff k) =
      (orbitPolynomial (f a) (f b) (f c) (f d) (f e)).coeff k := by
  have hm : (orbitPolynomial a b c d e).map f =
      orbitPolynomial (f a) (f b) (f c) (f d) (f e) := by
    simp [orbitPolynomial]
  rw [← hm, Polynomial.coeff_map]

theorem scalar_kernel_translation {A : Type*} [AddCommGroup A]
    (P T : A) (hT : (105:Nat) • T=0) : (105:Nat) • (P+T)=(105:Nat) • P := by
  rw [nsmul_add,hT,add_zero]

theorem augmentation_rotation_orbit {A : Type*} [AddCommGroup A]
    (a b c d : A) : -(-(a+b+c+d)+a+b+c)=d := by abel

def auxiliaryPolarization : Matrix (Fin 4) (Fin 4) ℤ :=
  !![4,2,2,2;2,4,2,2;2,2,4,2;2,2,2,4]

theorem auxiliary_dimension : auxiliaryPolarization.det=80 := by decide +kernel

theorem auxiliary_no_19_descent_dimension : ¬ (361 ∣ (80*105^8 : Nat)) ∧ ¬ (361 ∣ (57*80*105^8 : Nat)) := by
  norm_num

theorem auxiliary_dimension_not_361_multiple :
    ¬ ∃ m : Nat, 57*80*105^8=361*m := by
  rintro ⟨m,hm⟩
  exact auxiliary_no_19_descent_dimension.2 ⟨m,hm⟩

end KoblitzRound19
#print axioms KoblitzRound19.controlled_pole_map_cleared
#print axioms KoblitzRound19.elliptic_norm_identity
#print axioms KoblitzRound19.elliptic_norm_zero
#print axioms KoblitzRound19.reduced_boundary_monic
#print axioms KoblitzRound19.saturation_on_unit_chart
#print axioms KoblitzRound19.orbit_polynomial_rotation
#print axioms KoblitzRound19.orbit_coefficient_rotation
#print axioms KoblitzRound19.orbit_coefficient_field_map
#print axioms KoblitzRound19.scalar_kernel_translation
#print axioms KoblitzRound19.augmentation_rotation_orbit
#print axioms KoblitzRound19.auxiliary_dimension
#print axioms KoblitzRound19.auxiliary_no_19_descent_dimension
#print axioms KoblitzRound19.auxiliary_dimension_not_361_multiple

#print axioms KoblitzRound19.reduced_boundary_monic_power
#print axioms KoblitzRound19.determinant_vanishes_from_evaluation
#print axioms KoblitzRound19.weighted_map_identity
#print axioms KoblitzRound19.orbit_product_rotation
