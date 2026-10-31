import SameSupport
set_option autoImplicit false
namespace Add11Distinct
open Polynomial
variable {K : Type*} [Field K]

lemma distinct_linear_product_dvd (x1 x2 : K) (h : x1 ≠ x2) (P : K[X])
    (h1 : (X-C x1) ∣ P) (h2 : (X-C x2) ∣ P) :
    (X-C x1)*(X-C x2) ∣ P := by
  obtain ⟨a,ha⟩ := h1
  obtain ⟨b,hb⟩ := h2
  have hd : x2-x1 ≠ 0 := sub_ne_zero.mpr (Ne.symm h)
  have bez : C ((x2-x1)⁻¹)*((X-C x1)-(X-C x2)) = (1 : K[X]) := by
    rw [show (X-C x1)-(X-C x2) = C (x2-x1) by rw [C_sub]; ring]
    rw [← C_mul, inv_mul_cancel₀ hd, C_1]
  refine ⟨C ((x2-x1)⁻¹)*(b-a),?_⟩
  calc
    P = (C ((x2-x1)⁻¹)*((X-C x1)-(X-C x2)))*P := by rw [bez,one_mul]
    _ = C ((x2-x1)⁻¹)*((X-C x1)*P-(X-C x2)*P) := by ring
    _ = C ((x2-x1)⁻¹)*((X-C x1)*((X-C x2)*b)-(X-C x2)*((X-C x1)*a)) := by rw [← hb, ← ha]
    _ = (X-C x1)*(X-C x2)*(C ((x2-x1)⁻¹)*(b-a)) := by ring

theorem distinct_preserves_mumford (f : K[X]) (x1 y1 x2 y2 : K)
    (hx : x1 ≠ x2)
    (h1 : (X-C x1) ∣ f-(C y1)^2)
    (h2 : (X-C x2) ∣ f-(C y2)^2) :
    let q := -(x1+x2)
    let r := x1*x2
    let s := y1-y2
    let t := x1*y2-x2*y1
    let w := x1-x2
    w ≠ 0 ∧ (X^2+C q*X+C r) ∣ f-(C (s/w)*X+C (t/w))^2 := by
  dsimp
  have hd : x1-x2 ≠ 0 := sub_ne_zero.mpr hx
  let m := (y1-y2)/(x1-x2)
  let c := (x1*y2-x2*y1)/(x1-x2)
  let v : K[X] := C m*X+C c
  have e1 : c-y1 = -m*x1 := by dsimp [m,c]; field_simp; ring
  have e2 : c-y2 = -m*x2 := by dsimp [m,c]; field_simp; ring
  have v1 : v-C y1 = (X-C x1)*C m := by dsimp [v]; rw [show C m*X+C c-C y1 = C m*X+C (c-y1) by rw [C_sub]; ring,e1]; simp only [C_mul,C_neg]; ring
  have v2 : v-C y2 = (X-C x2)*C m := by dsimp [v]; rw [show C m*X+C c-C y2 = C m*X+C (c-y2) by rw [C_sub]; ring,e2]; simp only [C_mul,C_neg]; ring
  have d1 : (X-C x1) ∣ v^2-(C y1)^2 := by
    refine ⟨C m*(v+C y1),?_⟩
    calc
      _ = (v-C y1)*(v+C y1) := by ring
      _ = _ := by rw [v1]; ring
  have d2 : (X-C x2) ∣ v^2-(C y2)^2 := by
    refine ⟨C m*(v+C y2),?_⟩
    calc
      _ = (v-C y2)*(v+C y2) := by ring
      _ = _ := by rw [v2]; ring
  have a1 : (X-C x1) ∣ f-v^2 := by convert dvd_sub h1 d1 using 1 <;> ring
  have a2 : (X-C x2) ∣ f-v^2 := by convert dvd_sub h2 d2 using 1 <;> ring
  have hp := distinct_linear_product_dvd x1 x2 hx (f-v^2) a1 a2
  refine ⟨hd,?_⟩
  convert hp using 1 <;> simp only [C_neg,C_add,C_mul] <;> ring

inductive Prelude where
 | inverse | doubling | distinct
 deriving DecidableEq

def classify [DecidableEq K] (x1 y1 x2 y2 : K) : Prelude :=
 if x1=x2 ∧ y1= -y2 then .inverse
 else if x1=x2 ∧ y1=y2 then .doubling
 else .distinct

theorem distinct_guard [DecidableEq K] (x1 y1 x2 y2 : K) (hx : x1 ≠ x2) :
    classify x1 y1 x2 y2 = .distinct := by simp [classify,hx]

-- Full normalized validity and nonzero weights of the unit-input distinct payload.
theorem distinct_output_invariant (f : K[X]) (x1 y1 x2 y2 : K)
    (hx : x1 ≠ x2)
    (h1 : (X-C x1) ∣ f-(C y1)^2)
    (h2 : (X-C x2) ∣ f-(C y2)^2) :
    (1 : K) ≠ 0 ∧ x1-x2 ≠ 0 ∧
    (X^2+C (-(x1+x2)/(1 : K)^2)*X+C (x1*x2/(1 : K)^4)) ∣
      f-(C ((y1-y2)/((1 : K)^3*(x1-x2)))*X+
         C ((x1*y2-x2*y1)/((1 : K)^5*(x1-x2))))^2 := by
  have h := distinct_preserves_mumford f x1 y1 x2 y2 hx h1 h2
  exact ⟨one_ne_zero,h.1,by simpa using h.2⟩
end Add11Distinct
