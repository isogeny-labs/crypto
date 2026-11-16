module
public import Mathlib.Data.Sym.Card
public import Mathlib.Data.Fintype.BigOperators
public import Mathlib.Data.Fintype.Prod
public import Mathlib.Data.Nat.Choose.Bounds
public import Mathlib.Data.Nat.Factorial.BigOperators
public import Mathlib.Tactic
public section

/-! Exact finite weighted-simplex counting. No solver or closure hypothesis. -/
namespace NormWeightedCount
open Finset

/-- Every exponent vector of weighted degree at most D; positive weights
are required by the completeness theorem, not hidden in the definition. -/
def monomials {v : ℕ} (w : Fin v → ℕ) (D : ℕ) : Finset (Fin v → ℕ) :=
  (Fintype.piFinset fun _ => range (D+1)).filter (fun e => ∑ i, w i * e i ≤ D)

theorem mem_monomials {v D : ℕ} {w : Fin v → ℕ} (hw : ∀ i, 0 < w i)
    (e : Fin v → ℕ) : e ∈ monomials w D ↔ ∑ i, w i * e i ≤ D := by
  classical
  simp only [monomials, mem_filter, Fintype.mem_piFinset, mem_range]
  constructor
  · exact And.right
  · intro h
    refine ⟨fun i => ?_, h⟩
    have hi : w i * e i ≤ ∑ j, w j * e j := single_le_sum (f := fun j => w j * e j) (fun _ _ => Nat.zero_le _) (mem_univ i)
    have he : e i ≤ w i * e i := by simpa using Nat.mul_le_mul_right (e i) (hw i)
    omega

/-- An ordinary simplex encoded as a multiset with one slack coordinate. -/
def simplexCode {v L : ℕ} (y : Fin v → ℕ) (hy : ∑ i, y i ≤ L) : Sym (Option (Fin v)) L :=
  Sym.mk ((∑ i, Multiset.replicate (y i) (some i)) +
    Multiset.replicate (L - ∑ i, y i) none) (by
      simp only [Multiset.card_add, Multiset.card_sum, Multiset.card_replicate]
      omega)

theorem simplexCode_count {v L : ℕ} (y : Fin v → ℕ) (hy : ∑ i, y i ≤ L) (i : Fin v) :
    Multiset.count (some i) (simplexCode y hy : Multiset (Option (Fin v))) = y i := by
  classical
  simp [simplexCode, Multiset.count_sum', Multiset.count_replicate]

theorem simplexCode_injective {v L : ℕ} (y z : Fin v → ℕ)
    (hy : ∑ i, y i ≤ L) (hz : ∑ i, z i ≤ L)
    (h : simplexCode y hy = simplexCode z hz) : y = z := by
  funext i
  have hc := congrArg (fun q : Sym (Option (Fin v)) L => Multiset.count (some i) (q : Multiset _)) h
  simpa only [simplexCode_count] using hc

def digits {v : ℕ} (w : Fin v → ℕ) : Finset (Fin v → ℕ) :=
  Fintype.piFinset fun i => range (w i)

def radix {v : ℕ} (w e t : Fin v → ℕ) : Fin v → ℕ := fun i => w i * e i + t i

theorem radix_injective {v : ℕ} (w : Fin v → ℕ) (hw : ∀ i, 0 < w i)
    (e e' t t' : Fin v → ℕ) (ht : t ∈ digits w) (ht' : t' ∈ digits w)
    (h : radix w e t = radix w e' t') : e = e' ∧ t = t' := by
  have hlt : ∀ i, t i < w i := by simpa [digits] using ht
  have hlt' : ∀ i, t' i < w i := by simpa [digits] using ht'
  have he : e = e' := by
    funext i
    have hi := congrArg (fun f : Fin v → ℕ => f i / w i) h
    simpa [radix, Nat.mul_add_div (hw i), Nat.div_eq_of_lt (hlt i),
      Nat.div_eq_of_lt (hlt' i), Nat.ne_of_gt (hw i)] using hi
  refine ⟨he, ?_⟩
  funext i
  have hi := congrFun h i
  simp only [radix, he] at hi
  omega

theorem radix_budget {v D : ℕ} (w : Fin v → ℕ) (hw : ∀ i, 0 < w i)
    (e t : Fin v → ℕ) (he : e ∈ monomials w D) (ht : t ∈ digits w) :
    ∑ i, radix w e t i ≤ D + ∑ i, (w i - 1) := by
  have hlt : ∀ i, t i < w i := by simpa [digits] using ht
  have hs : ∑ i, t i ≤ ∑ i, (w i - 1) := sum_le_sum fun i _ => by have := hlt i; omega
  have hdegree := (mem_monomials hw e).mp he
  simp only [radix, sum_add_distrib]
  omega

/-- General mixed-radix stars-and-bars bound. -/
theorem weighted_simplex_bound {v : ℕ} (w : Fin v → ℕ) (hw : ∀ i, 0 < w i) (D : ℕ) :
    (monomials w D).card * (∏ i, w i) ≤ (D + ∑ i, w i).choose v := by
  classical
  let L := D + ∑ i, (w i - 1)
  let f : (monomials w D) × (digits w) → Sym (Option (Fin v)) L :=
    fun p => simplexCode (radix w p.1.val p.2.val) (radix_budget w hw _ _ p.1.property p.2.property)
  have hf : Function.Injective f := by
    intro p q h
    have hr := simplexCode_injective _ _ _ _ h
    have hh := radix_injective w hw _ _ _ _ p.2.property q.2.property hr
    exact Prod.ext (Subtype.ext hh.1) (Subtype.ext hh.2)
  have hc := Fintype.card_le_of_injective f hf
  have hsum : (∑ i, (w i - 1)) + v = ∑ i, w i := by
    calc
      (∑ i, (w i - 1)) + v = ∑ i, ((w i - 1)+1) := by simp [sum_add_distrib]
      _ = ∑ i, w i := sum_congr rfl fun i _ => by have := hw i; omega
  simp only [Fintype.card_prod, Fintype.card_coe, Sym.card_sym_eq_choose,
    Fintype.card_option, Fintype.card_fin] at hc
  simp only [digits, Fintype.card_piFinset, card_range] at hc
  have hL : v + 1 + L - 1 = L + v := by omega
  rw [hL] at hc
  have hc' : (monomials w D).card * (∏ i, w i) ≤ (L + v).choose L := by
    exact hc
  rw [Nat.choose_symm_add] at hc'
  simpa only [L, Nat.add_assoc, hsum] using hc'

/-- Factorial-adjusted bound avoids real division or asymptotic assumptions. -/
theorem weighted_factorial_bound {v : ℕ} (w : Fin v → ℕ) (hw : ∀ i, 0 < w i) (D : ℕ) :
    (monomials w D).card * (∏ i, w i) * v.factorial ≤ (D + ∑ i, w i)^v := by
  calc
    _ ≤ (D + ∑ i, w i).choose v * v.factorial :=
      Nat.mul_le_mul_right _ (weighted_simplex_bound w hw D)
    _ = (D + ∑ i, w i).descFactorial v := by rw [Nat.mul_comm, Nat.descFactorial_eq_factorial_mul_choose]
    _ ≤ _ := Nat.descFactorial_le_pow _ _

theorem consecutive_product (v : ℕ) : (∏ i : Fin v, (i.val+1)) = v.factorial := by
  rw [← Finset.prod_range (fun i => i+1), prod_range_add_one_eq_factorial]

theorem consecutive_sum_bound (v : ℕ) : (∑ i : Fin v, (i.val+1)) ≤ v^2 := by
  calc
    _ ≤ ∑ _i : Fin v, v := sum_le_sum fun i _ => by omega
    _ = v^2 := by simp [pow_two]

/-- Exact count of all monomials for weights 1,...,v, including v=0. -/
theorem consecutive_bound (v D : ℕ) :
    (monomials (fun i : Fin v => i.val+1) D).card * v.factorial^2 ≤ (D+v^2)^v := by
  have h := weighted_factorial_bound (fun i : Fin v => i.val+1) (fun i => by omega) D
  rw [consecutive_product] at h
  calc
    _ = (monomials (fun i : Fin v => i.val+1) D).card * v.factorial * v.factorial := by ring
    _ ≤ (D + ∑ i : Fin v, (i.val+1))^v := h
    _ ≤ _ := Nat.pow_le_pow_left (Nat.add_le_add_left (consecutive_sum_bound v) D) _

end NormWeightedCount
