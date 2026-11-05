import S3System

/-! Exact F43 degree certificates for the preregistered first main target.
The dual certificate concerns degree<=4 monomial multiples of these INPUTS,
not every possible Gröbner strategy or the homogeneous regularity. -/
namespace S3DegreeFixture
noncomputable section
set_option maxHeartbeats 2000000
open MvPolynomial S3System
abbrev k := S3System.F43.k
abbrev Poly := MvPolynomial (Fin 2) k

def term (i j : ℕ) (c : k) : Poly := monomial (exponent i j) c

theorem term_product (i j : ℕ) (c : k) : term i j c = C c*x^i*y^j := by
  simp only [term, exponent, X_pow_eq_monomial, x, y, C_mul_monomial,
    monomial_mul_monomial, mul_one]

theorem characteristic : (43 : Poly) = 0 := by
  exact CharP.cast_eq_zero Poly 43

theorem monomial_times_C (e : Fin 2 →₀ ℕ) (a b : k) :
    monomial e a * C b = monomial e (a*b) := by
  rw [mul_comm,C_mul_monomial]
  congr 1
  ring

def mainS3 : Poly := term 2 2 1 + term 2 1 24 + term 1 2 24 + term 2 0 15 +
  term 1 1 11 + term 0 2 15 + term 1 0 24 + term 0 1 24 + term 0 0 1
def gx : Poly := term 2 0 1 + term 1 0 26 + term 0 0 9
def gy : Poly := term 0 2 1 + term 0 1 26 + term 0 0 9

theorem mainS3_binding : mainS3 = equation (1 : k) 0 31 := by
  simp only [mainS3,term_product,equation,s3,map_one,map_zero,map_ofNat]
  linear_combination (2*x+45*x*y+2*x*y^2-22*x^2+2*x^2*y+2*y-22*y^2)*characteristic

theorem gx_binding : gx = (x-C (4 : k))*(x-C (13 : k)) := by
  simp only [gx,term_product,map_one,map_ofNat]
  linear_combination (x-1)*characteristic
theorem gy_binding : gy = (y-C (4 : k))*(y-C (13 : k)) := by
  simp only [gy,term_product,map_one,map_ofNat]
  linear_combination (y-1)*characteristic

def degree4Rows : Fin 13 → Poly := ![
  mainS3,
  gx,term 0 1 1*gx,term 1 0 1*gx,term 0 2 1*gx,term 1 1 1*gx,term 2 0 1*gx,
  gy,term 0 1 1*gy,term 1 0 1*gy,term 0 2 1*gy,term 1 1 1*gy,term 2 0 1*gy]

def dual : Poly →ₗ[k] k :=
  lcoeff k (exponent 0 0) + 14 • lcoeff k (exponent 0 1) +
  33 • lcoeff k (exponent 1 0) + 14 • lcoeff k (exponent 0 2) +
  27 • lcoeff k (exponent 1 1) + 36 • lcoeff k (exponent 2 0) +
  26 • lcoeff k (exponent 0 3) + 33 • lcoeff k (exponent 1 2) +
  32 • lcoeff k (exponent 2 1) + 14 • lcoeff k (exponent 3 0) +
  15 • lcoeff k (exponent 0 4) + 17 • lcoeff k (exponent 1 3) +
  5 • lcoeff k (exponent 2 2)

theorem dual_one : dual 1 = 1 := by
  norm_num [dual,lcoeff,exponent,coeff_one,Finsupp.ext_iff,Fin.forall_fin_two]

theorem dual_rows : ∀ i : Fin 13, dual (degree4Rows i) = 0 := by
  intro i
  fin_cases i <;>
    norm_num [dual,degree4Rows,mainS3,gx,gy,term,lcoeff,mul_add,add_mul,
      monomial_mul_monomial,monomial_times_C,coeff_monomial,coeff_C,coeff_one,
      exponent,Finsupp.ext_iff,Fin.forall_fin_two] <;> decide +kernel

theorem no_degree4_refutation :
    (1 : Poly) ∉ Submodule.span k (Set.range degree4Rows) := by
  have hs : Submodule.span k (Set.range degree4Rows) ≤ LinearMap.ker dual := by
    apply Submodule.span_le.mpr
    rintro _ ⟨i,rfl⟩
    exact dual_rows i
  intro h
  have hz : dual 1 = 0 := hs h
  rw [dual_one] at hz
  exact one_ne_zero hz

def w0 : Poly := 37+16*x+16*y
def w1 : Poly := 12+37*y+41*y^2+27*y^3+18*x+3*x*y+27*x*y^2
def w2 : Poly := 27+33*y+38*x+32*x*y

theorem degree5_unit_certificate : w0*mainS3+w1*gx+w2*gy=1 := by
  simp only [w0,w1,w2,mainS3,gx,gy,term_product,map_one,map_ofNat]
  linear_combination (9+40*x+80*x*y+81*x*y^2+26*x*y^3+33*x^2+33*x^2*y+
    36*x^2*y^2+x^2*y^3+6*x^3+9*x^3*y+x^3*y^2+52*y+51*y^2+12*y^3)*characteristic

/-- Universal cleared identity behind the characteristic-43 unit certificate.
The IDC1 backend checks this polynomial; Lean separately discharges 43=0. -/
theorem cleared_unit_identity {R : Type*} [CommRing R] (x y : R) :
    (37+16*x+16*y)*(x^2*y^2+24*x^2*y+24*x*y^2+15*x^2+11*x*y+15*y^2+24*x+24*y+1) +
    (12+37*y+41*y^2+27*y^3+18*x+3*x*y+27*x*y^2)*(x^2+26*x+9) +
    (27+33*y+38*x+32*x*y)*(y^2+26*y+9)-1 =
    43*(9+40*x+80*x*y+81*x*y^2+26*x*y^3+33*x^2+33*x^2*y+
      36*x^2*y^2+x^2*y^3+6*x^3+9*x^3*y+x^3*y^2+52*y+51*y^2+12*y^3) := by ring

def pullbackBase : Fin 3 → S3System.F43.E.Point := ![
  S3System.F43.ep 14 36 (by decide),S3System.F43.ep 13 24 (by decide),S3System.F43.P]
def twistWithTarget : Fin 3 → S3System.F43.T.Point := ![
  S3System.F43.twistBase 0,S3System.F43.twistBase 1,S3System.F43.U]

theorem pullback_images : ∀ i : Fin 3,
    S3System.F43.toTwist (pullbackBase i) = twistWithTarget i := by
  intro i
  fin_cases i
  · change TwistEmbedding.lift (4 : k) 0 21 (by decide) (35 : k) (by decide)
      (S3System.F43.ep 14 36 (by decide)) = S3System.F43.tp 15 9 (by decide)
    simp only [TwistEmbedding.lift,S3System.F43.ep,S3System.F43.tp]
    congr 1 <;> decide +kernel
  · change TwistEmbedding.lift (4 : k) 0 21 (by decide) (35 : k) (by decide)
      (S3System.F43.ep 13 24 (by decide)) = S3System.F43.tp 17 6 (by decide)
    simp only [TwistEmbedding.lift,S3System.F43.ep,S3System.F43.tp]
    congr 1 <;> decide +kernel
  · exact S3System.F43.source_image

theorem pulled_back_relation_sets (candidates : Finset (Fin 3 → ℤ)) :
    RelationYield.relationRows pullbackBase candidates =
      RelationYield.relationRows twistWithTarget candidates := by
  have heq : (fun i => S3System.F43.toTwist (pullbackBase i)) = twistWithTarget :=
    funext pullback_images
  rw [← heq,RelationYield.relationRows_transport S3System.F43.toTwist
    S3System.F43.toTwist_injective]

theorem pulled_back_first_relation : pullbackBase 0+pullbackBase 1=S3System.F43.P := by
  decide +kernel
theorem pulled_back_second_relation : -(pullbackBase 0+pullbackBase 0)=S3System.F43.P := by
  decide +kernel

#print axioms term_product
#print axioms characteristic
#print axioms monomial_times_C
#print axioms mainS3_binding
#print axioms gx_binding
#print axioms gy_binding
#print axioms dual_one
#print axioms dual_rows
#print axioms no_degree4_refutation
#print axioms degree5_unit_certificate
#print axioms cleared_unit_identity
#print axioms pullback_images
#print axioms pulled_back_relation_sets
#print axioms pulled_back_first_relation
#print axioms pulled_back_second_relation
end
end S3DegreeFixture
