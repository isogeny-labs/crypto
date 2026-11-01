import KoblitzRound09

namespace KoblitzRound10

theorem inverse_witness_unique {R : Type*} [CommRing R] (d u v : R)
    (hu : d*u=1) (hv : d*v=1) : u=v := by
  calc
    u=u*(d*v) := by rw [hv];simp
    _=(d*u)*v := by ring
    _=v := by rw [hu];simp

theorem inverse_witness_cancel {R : Type*} [CommRing R] (d u x : R)
    (hu : d*u=1) (h : d*x=0) : x=0 := by
  calc
    x=(d*u)*x := by rw [hu];simp
    _=u*(d*x) := by ring
    _=0 := by rw [h];simp

/-- Saturation by a denominator already invertible modulo the ideal. -/
theorem inverse_ideal_cancel {R : Type*} [CommRing R] (I : Ideal R)
    (d u x : R) (hu : d*u-1∈I) (h : d*x∈I) : x∈I := by
  have a : u*(d*x)∈I := I.mul_mem_left u h
  have b : x*(d*u-1)∈I := I.mul_mem_left x hu
  have eq : u*(d*x)-x*(d*u-1)=x := by ring
  rw [← eq]
  exact I.sub_mem a b

/-- A regular denominator modulo I cancels all its powers. -/
theorem saturation_powers {R : Type*} [CommRing R] (I : Ideal R) (d : R)
    (regular : ∀ x, d*x∈I → x∈I) : ∀ n x, d^n*x∈I → x∈I := by
  intro n
  induction n with
  | zero => intro x h;simpa using h
  | succ n ih =>
    intro x h
    apply ih x
    apply regular (d^n*x)
    simpa [pow_succ,mul_assoc,mul_comm,mul_left_comm] using h

theorem homogeneous_saturation_certificate {R : Type*} [CommRing R]
    (x y z u h : R) :
    h*(z*u-y*h)=y*(x*u-h^2)-u*(x*y-z*h) := by ring

theorem naive_projective_false_point :
    (0:ZMod 2)*0-1*0=0 ∧ (0:ZMod 2)*1-0^2=0 ∧
      (1:ZMod 2)*1-0*0≠0 := by decide +kernel

/-- The newly added relation follows on the h≠0 chart. -/
theorem saturated_relation_on_affine_chart {K : Type*} [Field K]
    (x y z u h : K) (hh : h≠0)
    (h1 : x*y-z*h=0) (h2 : x*u-h^2=0) : z*u-y*h=0 := by
  have e := homogeneous_saturation_certificate x y z u h
  rw [h1,h2] at e
  have : h*(z*u-y*h)=0 := by simpa using e
  exact (mul_eq_zero.mp this).resolve_left hh

/-- In dual numbers, coefficient Frobenius fixes epsilon; absolute squaring kills it. -/
theorem dual_number_square {R : Type*} [CommRing R] [CharP R 2]
    (a e : R) (he : e^2=0) : (a+e)^2=a^2 := by
  have h2 : (2:R)=0 := by simpa using (CharP.natCast_eq_natCast_mod R 2 2)
  calc
    (a+e)^2=a^2+2*a*e+e^2 := by ring
    _=a^2 := by rw [h2,he];ring

/-- The open cyclic graph has at most one extension of its initial component. -/
theorem component_sequence_unique {A : Type*} (f : ℕ → A → A)
    (x y : ℕ → A) (h0 : x 0=y 0)
    (hx : ∀ n, x (n+1)=f n (x n)) (hy : ∀ n, y (n+1)=f n (y n)) : x=y := by
  funext n
  induction n with
  | zero => exact h0
  | succ n ih => rw [hx,hy,ih]

/-- Tensor coefficient multiplication is the bilinear expansion, for any base algebra. -/
theorem coefficient_bilinear_expansion {I J R : Type*}
    [Fintype I] [Fintype J] [CommRing R]
    (a : I → R) (b : J → R) :
    (∑ i, a i)*(∑ j, b j)=∑ i,∑ j,a i*b j := by
  simp_rw [Finset.sum_mul,Finset.mul_sum]

/-- Multiplication-matrix inverses yield the scalar norm-unit witness. -/
theorem determinant_inverse_witness {I R : Type*} [Fintype I] [DecidableEq I]
    [CommRing R] (A B : Matrix I I R) (h : A*B=1) :
    Matrix.det A*Matrix.det B=1 := by
  rw [← Matrix.det_mul,h,Matrix.det_one]

theorem product_cusp_degree : 5*244+5*244+50*4=2640 := by norm_num
theorem coupled_pole_degrees :
    10*242+50*2=2520 ∧ 10*363+50*3=3780 ∧
    10*244+50*4=2640 ∧ 10*365+50*5=3900 := by norm_num

theorem sum_difference_map_degrees : 660+660-2*20=1280 ∧
    660+660+2*20=1360 ∧ 1280+1360=2640 := by norm_num

/-- The radial line at an ordinary point has first coefficient x². -/
theorem radial_line_derivative {R : Type*} [CommRing R] [CharP R 2]
    (x y dy : R) (h : x*dy=y+x^2) : x*dy+y=x^2 := by
  have h2 : (2:R)=0 := by simpa using (CharP.natCast_eq_natCast_mod R 2 2)
  rw [h]
  calc y+x^2+y=x^2+2*y := by ring
       _=x^2 := by rw [h2];ring

/-- Formal pullback-product divisor bookkeeping; geometric divisor identities are separate. -/
theorem product_divisor_sum {A : Type*} [AddCommGroup A] (t0 ti o0 oi : A) :
    (2:ℤ) • (t0-o0)+(2:ℤ) • (ti-oi)=
      (2:ℤ) • (t0+ti)-(2:ℤ) • (o0+oi) := by
  simp only [zsmul_sub,zsmul_add]
  abel

theorem square_sum_residual {R : Type*} [CommRing R] (a e : R) :
    (a+e)^2-a^2-e^2=2*a*e := by ring

theorem collision_derivative_residual {R : Type*} [CommRing R] (x y : R) :
    (x^9*y^8+x^11*y^2+(x^3+x^11)*y^10)-
      x^3*y^2*((1+x^4)*y^4+x^3*y^3+x^4)^2=
    -2*(x^7*y^10+x^6*(1+x^4)*y^9+x^7*(1+x^4)*y^6+x^10*y^5) := by ring

theorem collision_derivative_factorization {R : Type*} [CommRing R] [CharP R 2]
    (x y : R) : x^9*y^8+x^11*y^2+(x^3+x^11)*y^10=
      x^3*y^2*((1+x^4)*y^4+x^3*y^3+x^4)^2 := by
  have h2 : (2:R)=0 := by simpa using (CharP.natCast_eq_natCast_mod R 2 2)
  have e := collision_derivative_residual x y
  rw [h2,neg_zero,zero_mul] at e
  exact sub_eq_zero.mp e

theorem collision_resultant_factorization {R : Type*} [CommRing R] [CharP R 2]
    (x : R) : x^96+x^88=x^88*(x+1)^8 := by
  have h : (x+1)^8=x^8+1 := by
    simpa using (add_pow_char_pow x (1:R) 2 3)
  rw [h]
  ring

theorem ordinary_transport_determinant {R : Type*} [CommRing R] (x y : R) :
    (x-4*y)*(-2*x+y)-(2*x+3*y)*(3*x+4*y)=
      -8*(x^2+x*y+2*y^2) := by ring

theorem ordinary_transport_finite_chart :
    ∀ a : ZMod 49, 1+a+2*a^2≠0 := by decide +kernel

theorem ordinary_transport_other_chart :
    ∀ a : ZMod 49, (7*a)^2+7*a+2≠0 := by decide +kernel

end KoblitzRound10

#print axioms KoblitzRound10.inverse_witness_unique
#print axioms KoblitzRound10.inverse_witness_cancel
#print axioms KoblitzRound10.inverse_ideal_cancel
#print axioms KoblitzRound10.saturation_powers
#print axioms KoblitzRound10.homogeneous_saturation_certificate
#print axioms KoblitzRound10.naive_projective_false_point
#print axioms KoblitzRound10.saturated_relation_on_affine_chart
#print axioms KoblitzRound10.dual_number_square
#print axioms KoblitzRound10.component_sequence_unique
#print axioms KoblitzRound10.coefficient_bilinear_expansion
#print axioms KoblitzRound10.determinant_inverse_witness
#print axioms KoblitzRound10.product_cusp_degree
#print axioms KoblitzRound10.coupled_pole_degrees
#print axioms KoblitzRound10.sum_difference_map_degrees
#print axioms KoblitzRound10.radial_line_derivative
#print axioms KoblitzRound10.product_divisor_sum
#print axioms KoblitzRound10.square_sum_residual
#print axioms KoblitzRound10.collision_derivative_residual
#print axioms KoblitzRound10.collision_derivative_factorization
#print axioms KoblitzRound10.collision_resultant_factorization
#print axioms KoblitzRound10.ordinary_transport_determinant
#print axioms KoblitzRound10.ordinary_transport_finite_chart
#print axioms KoblitzRound10.ordinary_transport_other_chart
