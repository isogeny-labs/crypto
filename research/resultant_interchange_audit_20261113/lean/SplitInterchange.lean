import Definitions

namespace ResultantInterchangeAudit
open Polynomial
noncomputable section
variable {K : Type*} [Field K]

theorem eval_C_eq_map_swap (q : Polynomial (Polynomial K)) (a : K) :
    q.eval (C a) = (Polynomial.Bivariate.swap q).map (evalRingHom a) := by
  simpa [aeval_def, coe_mapAlgHom] using
    (Polynomial.Bivariate.aeval_eq_map_swap (R := K) (A := K) a q)

theorem innerY_prod (p : Polynomial K) (q : Polynomial (Polynomial K))
    (dy : ℕ) (hp : p.Splits) (hq : q.natDegree ≤ dy) :
    innerY p q dy = C p.leadingCoeff ^ dy *
      (p.roots.map fun a => q.eval (C a)).prod := by
  have hi : Function.Injective (C : K →+* Polynomial K) := C_injective
  have hd : (p.map C).natDegree = p.natDegree := natDegree_map_eq_of_injective hi p
  unfold innerY lift
  conv_lhs => rw [← hd]
  rw [resultant_eq_prod_eval _ _ _ hq (hp.map C)]
  rw [leadingCoeff_map_of_injective hi, hp.roots_map_of_injective hi]
  simp only [Multiset.map_map, Function.comp_def]

theorem innerY_degree (p : Polynomial K) (q : Polynomial (Polynomial K))
    (dx dy : ℕ) (hp : p.Splits) (hq : q.natDegree ≤ dy)
    (hx : (Polynomial.Bivariate.swap q).natDegree ≤ dx) :
    (innerY p q dy).natDegree ≤ p.natDegree * dx := by
  rw [innerY_prod p q dy hp hq]
  calc
    _ ≤ 0 + ((p.roots.map fun a => q.eval (C a)).map natDegree).sum := by
      exact (natDegree_mul_le ..).trans (Nat.add_le_add (by simp) (natDegree_multiset_prod_le _))
    _ ≤ p.roots.card * dx := by
      simp only [zero_add, Multiset.map_map, Function.comp_def]
      have ht (a : K) : (q.eval (C a)).natDegree ≤ dx := by
        rw [eval_C_eq_map_swap]
        exact natDegree_map_le.trans hx
      have hs (s : Multiset K) :
          (s.map fun a => (q.eval (C a)).natDegree).sum ≤ s.card * dx := by
        induction s using Multiset.induction_on with
        | empty => simp
        | cons a s ih =>
          simp only [Multiset.map_cons, Multiset.sum_cons, Multiset.card_cons]
          nlinarith [ht a]
      exact hs p.roots
    _ = _ := by rw [hp.natDegree_eq_card_roots]

theorem innerY_eval (p : Polynomial K) (q : Polynomial (Polynomial K))
    (dy : ℕ) (hp : p.Splits) (hq : q.natDegree ≤ dy) (b : K) :
    (innerY p q dy).eval b = p.leadingCoeff ^ dy *
      (p.roots.map fun a => q.evalEval b a).prod := by
  rw [innerY_prod p q dy hp hq]
  simp only [eval_mul, eval_pow, eval_C, eval_multiset_prod, Multiset.map_map,
    Function.comp_def, evalEval]

theorem innerX_prod (q : Polynomial (Polynomial K)) (r : Polynomial K)
    (dx : ℕ) (hr : r.Splits) (hx : (Polynomial.Bivariate.swap q).natDegree ≤ dx) :
    innerX q r dx = C ((-1 : K) ^ (dx * r.natDegree) * r.leadingCoeff ^ dx) *
      (r.roots.map fun b => q.map (evalRingHom b)).prod := by
  unfold innerX
  rw [resultant_comm]
  change (-1 : Polynomial K) ^ (dx * r.natDegree) *
      innerY r (Polynomial.Bivariate.swap q) dx = _
  rw [innerY_prod r _ dx hr hx]
  simp only [eval_C_eq_map_swap, Polynomial.Bivariate.swap_swap_apply,
    map_mul, map_pow, map_neg, map_one]
  ring

theorem innerX_degree (q : Polynomial (Polynomial K)) (r : Polynomial K)
    (dx dy : ℕ) (hr : r.Splits) (hq : q.natDegree ≤ dy)
    (hx : (Polynomial.Bivariate.swap q).natDegree ≤ dx) :
    (innerX q r dx).natDegree ≤ dy * r.natDegree := by
  unfold innerX
  rw [resultant_comm]
  change ((-1 : Polynomial K) ^ (dx * r.natDegree) *
    innerY r (Polynomial.Bivariate.swap q) dx).natDegree ≤ _
  calc
    _ ≤ 0 + r.natDegree * dy := (natDegree_mul_le ..).trans
      (Nat.add_le_add (by simp) (innerY_degree r _ dy dx hr hx (by simpa only [Polynomial.Bivariate.swap_swap_apply] using hq)))
    _ = _ := by simp only [zero_add, Nat.mul_comm]

theorem innerX_eval (q : Polynomial (Polynomial K)) (r : Polynomial K)
    (dx : ℕ) (hr : r.Splits) (hx : (Polynomial.Bivariate.swap q).natDegree ≤ dx)
    (a : K) :
    (innerX q r dx).eval a = (-1 : K) ^ (dx * r.natDegree) * r.leadingCoeff ^ dx *
      (r.roots.map fun b => q.evalEval b a).prod := by
  rw [innerX_prod q r dx hr hx]
  simp only [eval_mul, eval_C, eval_multiset_prod, Multiset.map_map,
    Function.comp_def, map_evalRingHom_eval]

/-- Formal Sylvester sizes are retained even when either intermediate degree drops. -/
theorem fixed_interchange_of_splits (p : Polynomial K) (q : Polynomial (Polynomial K))
    (r : Polynomial K) (dx dy : ℕ) (hp : p.Splits) (hr : r.Splits)
    (hq : q.natDegree ≤ dy) (hx : (Polynomial.Bivariate.swap q).natDegree ≤ dx) :
    leftFixed p q r dx dy = rightFixed p q r dx dy := by
  unfold leftFixed rightFixed
  rw [resultant_comm (innerY p q dy) r,
    resultant_eq_prod_eval r _ _ (innerY_degree p q dx dy hp hq hx) hr,
    resultant_eq_prod_eval p _ _ (innerX_degree q r dx dy hr hq hx) hp]
  simp_rw [innerY_eval p q dy hp hq, innerX_eval q r dx hr hx]
  simp only [Multiset.prod_map_mul, Multiset.map_const', Multiset.prod_replicate]
  rw [← hp.natDegree_eq_card_roots, ← hr.natDegree_eq_card_roots,
    Multiset.prod_map_prod_map]
  simp only [mul_pow, ← pow_mul, Nat.mul_assoc, Nat.mul_left_comm, Nat.mul_comm]
  ring

#print axioms fixed_interchange_of_splits

end
end ResultantInterchangeAudit
