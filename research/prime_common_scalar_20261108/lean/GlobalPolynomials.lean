module
public import LocalCharts
public section
namespace GlobalPolynomials
open CauchyNormalizer LocalCharts Polynomial

@[expose] noncomputable def basePolynomial (a : ℕ) (s : Fin (2^(a+1))) : Polynomial ℚ :=
  (∏ j ∈ Finset.univ.erase s, (X-C ((j:ℚ)-(2^a:ℕ)))) * C ((centerDen a)⁻¹)

/-- Exact polynomial representation of the imported actual kernel. -/
theorem eval_basePolynomial (a : ℕ) (s : Fin (2^(a+1))) (T : ℚ) :
    (basePolynomial a s).eval T = baseKernel a s T := by
  rw [base_as_fin_product]
  unfold basePolynomial
  change eval₂ (RingHom.id ℚ) T _ = _
  simp only [eval₂_mul, eval₂_C, eval₂_finsetProd, eval₂_sub, eval₂_X, RingHom.id_apply]
  ring

theorem basePolynomial_degree (a : ℕ) (s : Fin (2^(a+1))) :
    (basePolynomial a s).natDegree ≤ 2^(a+1)-1 := by
  unfold basePolynomial
  calc
    _ ≤ (∏ j ∈ Finset.univ.erase s, (X-C ((j:ℚ)-(2^a:ℕ)) : Polynomial ℚ)).natDegree + 0 := by
      simpa using natDegree_mul_le (p := ∏ j ∈ Finset.univ.erase s, (X-C ((j:ℚ)-(2^a:ℕ)) : Polynomial ℚ)) (q := C ((centerDen a)⁻¹))
    _ ≤ (Finset.univ.erase s).card := by
      have hprod := natDegree_prod_le (Finset.univ.erase s)
        (fun j => (X-C ((j:ℚ)-(2^a:ℕ)) : Polynomial ℚ))
      have hsum : (∑ j ∈ Finset.univ.erase s,
          (X-C ((j:ℚ)-(2^a:ℕ)) : Polynomial ℚ).natDegree) ≤
          ∑ _j ∈ Finset.univ.erase s, 1 := by
        apply Finset.sum_le_sum
        intro j hj
        exact natDegree_X_sub_C_le _
      simpa only [Nat.add_zero, Finset.sum_const, nsmul_eq_mul, mul_one, Nat.cast_id] using hprod.trans hsum
    _ = _ := by simp

/-- The generic degree theorem is now tied to the actual kernel polynomial. -/
theorem actual_squared_product_degree {ι : Type*} (A : Finset ι) (a : ℕ)
    (index : ι → Fin (2^(a+1))) :
    (∏ j ∈ A, (basePolynomial a (index j))^2).natDegree ≤
      A.card * (2*(2^(a+1)-1)) := by
  exact RationalScalar.squared_product_degree A (fun j => basePolynomial a (index j))
    (2^(a+1)-1) (fun j hj => basePolynomial_degree a (index j))

end GlobalPolynomials
#print axioms GlobalPolynomials.eval_basePolynomial
#print axioms GlobalPolynomials.basePolynomial_degree
#print axioms GlobalPolynomials.actual_squared_product_degree
