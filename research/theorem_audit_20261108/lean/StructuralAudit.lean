import Mathlib.Tactic
import Mathlib.Algebra.Group.Subgroup.Basic
import Mathlib.Data.ZMod.Basic
import Mathlib.GroupTheory.OrderOfElement

namespace TheoremAudit

/-- Alternation makes the restriction to a cyclic subgroup trivial.
This does not assert that the original full-domain pairing is degenerate. -/
theorem cyclic_restriction_trivial {G H : Type*} [AddCommGroup G] [AddCommGroup H]
    (e : G →+ G →+ H) (P : G) (halt : e P P=0) (m n : ℕ) :
    e (m • P) (n • P)=0 := by
  simp [halt]

/-- Every finite sum of factor-base elements in a subgroup remains there.
Applies to any Jacobian group, but does not supply a Jacobian family. -/
theorem sum_confined {G : Type*} [AddCommGroup G] (H : AddSubgroup G)
    (F : Set G) (hF : F ⊆ H) (xs : List G) (hxs : ∀ x ∈ xs, x ∈ F) :
    xs.sum ∈ H := by
  exact H.list_sum_mem (fun x hx => hF (hxs x hx))

theorem outside_subgroup_not_smooth {G : Type*} [AddCommGroup G]
    (H : AddSubgroup G) (F : Set G) (hF : F ⊆ H) (Q : G) (hQ : Q ∉ H) :
    ¬ ∃ xs : List G, (∀ x ∈ xs, x ∈ F) ∧ xs.sum=Q := by
  rintro ⟨xs,hxs,rfl⟩
  exact hQ (sum_confined H F hF xs hxs)

/-- Exact one-step collision mass for two independent branch labels.
The asymptotic adding-walk penalty requires additional walk assumptions. -/
theorem branch_collision_mass {ι : Type*} [Fintype ι] [DecidableEq ι]
    (p : ι → ℚ) :
    (∑ i : ι, ∑ j : ι, if i=j then p i*p j else 0)=∑ i : ι, (p i)^2 := by
  simp [pow_two]

/-- Conditional modular arithmetic in the supersingular prime-field proof.
Supersingularity and Hasse's bound must separately establish q=-1 mod r. -/
theorem minus_one_embedding_order {r : ℕ} [Fact r.Prime]
    (hr : 2<r) (q : ZMod r) (hq : q = -1) :
    q^2=1 ∧ q≠1 := by
  subst q
  constructor
  · ring
  · intro h
    have h2 : (2 : ZMod r)=0 := by linear_combination -h
    have hd : r ∣ 2 := (ZMod.natCast_eq_zero_iff 2 r).mp h2
    exact (not_le.mpr hr) (Nat.le_of_dvd (by decide) hd)

/-- Exact embedding degree from the trace-zero group-order condition.
The supersingularity-to-trace-zero and Hasse steps remain outside this lemma. -/
theorem trace_zero_prime_embedding_degree {r : ℕ} [Fact r.Prime]
    (hr : 2 < r) (q : ℕ) (hdiv : r ∣ q+1) : orderOf (q : ZMod r)=2 := by
  have hz : (q : ZMod r)+1=0 := by
    simpa only [Nat.cast_add,Nat.cast_one] using
      (ZMod.natCast_eq_zero_iff (q+1) r).mpr hdiv
  have hq : (q : ZMod r) = -1 := (eq_neg_iff_add_eq_zero).mpr hz
  have hh := minus_one_embedding_order hr (q : ZMod r) hq
  exact orderOf_eq_prime hh.1 hh.2

theorem two_subgroup_embedding_degree (q : ℕ) (hq : Odd q) :
    orderOf (q : ZMod 2)=1 := by
  have hq1 : (q : ZMod 2)=1 := by
    obtain ⟨k,hk⟩ := hq
    rw [hk]
    push_cast
    rw [show (2 : ZMod 2)=0 from rfl]
    simp
  simp [hq1]

/-- The literal branch-locus form of Diem Condition 2.7. The predicate
`branched` means ramification above the point, not a rational-point count. -/
def DiemCondition {α : Type*} (σ : α → α) (branched : α → Prop) (n : ℕ) : Prop :=
  ∃ R, branched R ∧ Function.Injective (fun i : Fin n => σ^[i.val] R) ∧
    ∀ i : ℕ, 0 < i → i < n → ¬ branched (σ^[i] R)

/-- A Frobenius-stable branch locus cannot satisfy Condition 2.7 for n>=2.
The scheme-theoretic fact that base-defined covers have stable branch loci
is separate from this orbit argument. -/
theorem stable_branch_locus_excludes_diem {α : Type*}
    (σ : α → α) (branched : α → Prop) (n : ℕ) (hn : 2≤n)
    (hstable : ∀ R, branched R → branched (σ R)) :
    ¬ DiemCondition σ branched n := by
  rintro ⟨R,hR,_,hrest⟩
  exact hrest 1 (by omega) (by omega) (by simpa using hstable R hR)

/-- The finite ramification x coordinates of a nonsingular odd-characteristic
short Weierstrass projection are roots of x^3+a*x+b. A field endomorphism
fixing the coefficients preserves those roots. -/
theorem short_branch_locus_stable {K : Type*} [Field K]
    (σ : K →+* K) (a b : K) (ha : σ a=a) (hb : σ b=b) :
    ∀ x, x^3+a*x+b=0 → (σ x)^3+a*(σ x)+b=0 := by
  intro x hx
  have hh := congrArg σ hx
  simpa only [map_add,map_mul,map_pow,map_zero,ha,hb] using hh

end TheoremAudit
