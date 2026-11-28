import Mathlib.LinearAlgebra.Lagrange
import Mathlib.Logic.Equiv.Set
import Mathlib.SetTheory.Cardinal.Finite
import Mathlib.Tactic

/-! Exact bounded interpolation states over existing polynomial semantics.

Consumer: crypto PR4338 candidate A. Prospective statement/inheritance plan
1d7dc7c and inherited-law/actual-coordinate probe 0e381fb precede this file.
Different clean selected supports have different bounded evaluation kernels
while their size is at most the u-degree cap. This is an exact-state
enumeration statement, not a lower bound for symbolic unions or all selectors.
-/
set_option autoImplicit false

namespace CryptoLean.InterpolationKernels
open Polynomial
open scoped BigOperators

variable {F I : Type*} [Field F]

/-- The actual set of degree-bounded pairs satisfying the selected evaluations. -/
def kernel (x y : I → F) (dU dV : ℕ) (s : Finset I) : Set (F[X] × F[X]) :=
  {uv | uv.1.natDegree ≤ dU ∧ uv.2.natDegree ≤ dV ∧
    ∀ i ∈ s, uv.1.eval (x i) + y i * uv.2.eval (x i) = 0}

/-- The single membership interface that opens the bounded-kernel predicate. -/
theorem mem_kernel (x y : I → F) (dU dV : ℕ) (s : Finset I) (u v : F[X]) :
    (u, v) ∈ kernel x y dU dV s ↔
      u.natDegree ≤ dU ∧ v.natDegree ≤ dV ∧
        ∀ i ∈ s, u.eval (x i) + y i * v.eval (x i) = 0 := Iff.rfl

/-- Inherited nodal evaluation detects exactly the selected clean records. -/
theorem nodal_eval_zero_iff (x : I → F) (hx : Function.Injective x)
    (s : Finset I) (j : I) :
    (Lagrange.nodal s x).eval (x j) = 0 ↔ j ∈ s := by
  classical
  constructor
  · intro hz
    by_contra hj
    apply (Lagrange.eval_nodal_not_at_node (s := s) (v := x) ?_) hz
    intro i hi h
    exact hj ((hx h).symm ▸ hi)
  · exact Lagrange.eval_nodal_at_node

/-- The inherited nodal pair belongs to another kernel exactly when that
kernel's selected support is contained in its zero set. -/
theorem nodal_pair_mem_iff (x y : I → F) (hx : Function.Injective x)
    (dU dV : ℕ) (s t : Finset I) (hs : s.card ≤ dU) :
    (Lagrange.nodal s x, (0 : F[X])) ∈ kernel x y dU dV t ↔ t ⊆ s := by
  rw [mem_kernel]
  constructor
  · intro h j hj
    apply (nodal_eval_zero_iff x hx s j).mp
    simpa only [Polynomial.eval_zero, mul_zero, add_zero] using h.2.2 j hj
  · intro h
    refine ⟨?_, ?_, ?_⟩
    · simpa only [Lagrange.natDegree_nodal] using hs
    · simp
    · intro j hj
      simpa only [Polynomial.eval_zero, mul_zero, add_zero] using
        Lagrange.eval_nodal_at_node (v := x) (h hj)

/-- A nonzero feasible pair exists for every support within the u-degree cap. -/
theorem nodal_witness (x y : I → F) (hx : Function.Injective x)
    (dU dV : ℕ) (s : Finset I) (hs : s.card ≤ dU) :
    (Lagrange.nodal s x, (0 : F[X])) ∈ kernel x y dU dV s ∧
      (Lagrange.nodal s x, (0 : F[X])) ≠ 0 := by
  constructor
  · exact (nodal_pair_mem_iff x y hx dU dV s s hs).mpr (Finset.Subset.refl s)
  · intro h
    have hz : Lagrange.nodal s x = 0 := congrArg Prod.fst h
    exact Lagrange.nodal_ne_zero hz

/-- Equality of bounded kernels recovers their clean selected supports. -/
theorem supports_eq_of_kernels_eq (x y : I → F) (hx : Function.Injective x)
    (dU dV : ℕ) (s t : Finset I) (hs : s.card ≤ dU) (ht : t.card ≤ dU)
    (h : kernel x y dU dV s = kernel x y dU dV t) : s = t := by
  have hts : t ⊆ s := (nodal_pair_mem_iff x y hx dU dV s t hs).mp (by
    rw [← h]
    exact (nodal_witness x y hx dU dV s hs).1)
  have hst : s ⊆ t := (nodal_pair_mem_iff x y hx dU dV t s ht).mp (by
    rw [h]
    exact (nodal_witness x y hx dU dV t ht).1)
  exact Finset.Subset.antisymm hst hts

/-- One actual primitive record is selected from every prefix color. -/
def prefixSupport {t b : ℕ} (choice : Fin t → Fin b) : Finset (Fin t × Fin b) :=
  Finset.univ.image fun i => (i, choice i)

/-- The interface exposing the selected-support construction. -/
theorem prefixSupport_eq_image {t b : ℕ} (choice : Fin t → Fin b) :
    prefixSupport choice = Finset.univ.image (fun i => (i, choice i)) := rfl

theorem mem_prefixSupport {t b : ℕ} (choice : Fin t → Fin b) (i : Fin t) (j : Fin b) :
    (i, j) ∈ prefixSupport choice ↔ j = choice i := by
  rw [prefixSupport_eq_image]
  constructor
  · intro h
    obtain ⟨a, _, ha⟩ := Finset.mem_image.mp h
    have hai : a = i := congrArg Prod.fst ha
    subst a
    exact (congrArg Prod.snd ha).symm
  · intro h
    subst j
    exact Finset.mem_image.mpr ⟨i, Finset.mem_univ i, rfl⟩

theorem prefixSupport_card {t b : ℕ} (choice : Fin t → Fin b) :
    (prefixSupport choice).card = t := by
  rw [prefixSupport_eq_image, Finset.card_image_of_injective]
  · simp
  · intro i j h
    exact congrArg Prod.fst h

theorem prefixSupport_injective {t b : ℕ} :
    Function.Injective (prefixSupport (t := t) (b := b)) := by
  intro a b h
  funext i
  have ha : (i, a i) ∈ prefixSupport a := (mem_prefixSupport a i (a i)).mpr rfl
  have hb : (i, a i) ∈ prefixSupport b := h ▸ ha
  exact (mem_prefixSupport b i (a i)).mp hb

/-- Exact kernel memoization cannot identify distinct colored prefixes in
this range of the genuine polynomial degree bounds. -/
theorem prefix_kernel_injective {t b : ℕ} (x y : Fin t × Fin b → F)
    (hx : Function.Injective x) (dU dV : ℕ) (ht : t ≤ dU) :
    Function.Injective (fun choice : Fin t → Fin b =>
      kernel x y dU dV (prefixSupport choice)) := by
  intro a b h
  apply prefixSupport_injective
  apply supports_eq_of_kernels_eq x y hx dU dV _ _ ?_ ?_ h
  · simpa only [prefixSupport_card] using ht
  · simpa only [prefixSupport_card] using ht

/-- Finite count of actual distinct bounded-kernel sets, including empty
prefixes and empty choice domains. No surrogate state-count function. -/
theorem prefix_kernel_count {t b : ℕ} (x y : Fin t × Fin b → F)
    (hx : Function.Injective x) (dU dV : ℕ) (ht : t ≤ dU) :
    Nat.card (Set.range (fun choice : Fin t → Fin b =>
      kernel x y dU dV (prefixSupport choice))) = b ^ t := by
  classical
  calc
    Nat.card (Set.range (fun choice : Fin t → Fin b =>
        kernel x y dU dV (prefixSupport choice))) = Nat.card (Fin t → Fin b) :=
      Nat.card_congr (Equiv.ofInjective _ (prefix_kernel_injective x y hx dU dV ht)).symm
    _ = b ^ t := by simp [Nat.card_eq_fintype_card]

/-- A color is a product of its actual evaluation linear forms. -/
def colorProduct {t b : ℕ} (x y : Fin t × Fin b → F) (i : Fin t)
    (u v : F[X]) : F :=
  ∏ j : Fin b, (u.eval (x (i,j)) + y (i,j) * v.eval (x (i,j)))

theorem colorProduct_eq {t b : ℕ} (x y : Fin t × Fin b → F) (i : Fin t)
    (u v : F[X]) : colorProduct x y i u v =
      ∏ j : Fin b, (u.eval (x (i,j)) + y (i,j) * v.eval (x (i,j))) := rfl

/-- Preserve the existential choice inside a color, rather than replacing
it by simultaneous vanishing at every record. -/
theorem colorProduct_zero_iff {t b : ℕ} (x y : Fin t × Fin b → F) (i : Fin t)
    (u v : F[X]) : colorProduct x y i u v = 0 ↔
      ∃ j : Fin b, u.eval (x (i,j)) + y (i,j) * v.eval (x (i,j)) = 0 := by
  rw [colorProduct_eq]
  simp only [Finset.prod_eq_zero_iff, Finset.mem_univ, true_and]

theorem colorProducts_zero_iff_choice {t b : ℕ} (x y : Fin t × Fin b → F)
    (u v : F[X]) : (∀ i, colorProduct x y i u v = 0) ↔
      ∃ choice : Fin t → Fin b, ∀ i,
        u.eval (x (i,choice i)) + y (i,choice i) * v.eval (x (i,choice i)) = 0 := by
  simp only [colorProduct_zero_iff]
  constructor
  · exact Classical.axiomOfChoice
  · rintro ⟨choice, h⟩ i
    exact ⟨choice i, h i⟩

/-- The selected-support interface also reads as one equation per color. -/
theorem mem_prefix_kernel_iff {t b : ℕ} (x y : Fin t × Fin b → F)
    (dU dV : ℕ) (choice : Fin t → Fin b) (u v : F[X]) :
    (u,v) ∈ kernel x y dU dV (prefixSupport choice) ↔
      u.natDegree ≤ dU ∧ v.natDegree ≤ dV ∧
        ∀ i, u.eval (x (i,choice i)) + y (i,choice i) * v.eval (x (i,choice i)) = 0 := by
  rw [mem_kernel]
  constructor
  · rintro ⟨hu, hv, h⟩
    exact ⟨hu, hv, fun i => h (i,choice i) ((mem_prefixSupport choice i _).mpr rfl)⟩
  · rintro ⟨hu, hv, h⟩
    refine ⟨hu, hv, ?_⟩
    rintro ⟨i,j⟩ hj
    have he : j = choice i := (mem_prefixSupport choice i j).mp hj
    simpa only [he] using h i

/-- Compact membership predicate for the full union. This is not asserted
to be a linear module or an efficient nonzero-witness extractor. -/
def symbolic {t b : ℕ} (x y : Fin t × Fin b → F) (dU dV : ℕ) :
    Set (F[X] × F[X]) :=
  {uv | uv.1.natDegree ≤ dU ∧ uv.2.natDegree ≤ dV ∧
    ∀ i, colorProduct x y i uv.1 uv.2 = 0}

theorem mem_symbolic {t b : ℕ} (x y : Fin t × Fin b → F) (dU dV : ℕ)
    (u v : F[X]) : (u,v) ∈ symbolic x y dU dV ↔
      u.natDegree ≤ dU ∧ v.natDegree ≤ dV ∧
        ∀ i, colorProduct x y i u v = 0 := Iff.rfl

/-- The symbolic product constraints describe exactly the union of the
genuine bounded kernels, without an independence or cleanliness assumption. -/
theorem symbolic_mem_iff_exists_kernel {t b : ℕ} (x y : Fin t × Fin b → F)
    (dU dV : ℕ) (u v : F[X]) :
    (u,v) ∈ symbolic x y dU dV ↔
      ∃ choice : Fin t → Fin b, (u,v) ∈ kernel x y dU dV (prefixSupport choice) := by
  rw [mem_symbolic]
  constructor
  · rintro ⟨hu, hv, h⟩
    obtain ⟨choice, hc⟩ := (colorProducts_zero_iff_choice x y u v).mp h
    exact ⟨choice, (mem_prefix_kernel_iff x y dU dV choice u v).mpr ⟨hu, hv, hc⟩⟩
  · rintro ⟨choice, hc⟩
    obtain ⟨hu, hv, h⟩ := (mem_prefix_kernel_iff x y dU dV choice u v).mp hc
    exact ⟨hu, hv, (colorProducts_zero_iff_choice x y u v).mpr ⟨choice, h⟩⟩

/-- Nonzero extraction remains an explicit obligation on both sides. -/
theorem symbolic_nonzero_iff_exists_kernel {t b : ℕ} (x y : Fin t × Fin b → F)
    (dU dV : ℕ) :
    (∃ uv : F[X] × F[X], uv ≠ 0 ∧ uv ∈ symbolic x y dU dV) ↔
      ∃ choice : Fin t → Fin b, ∃ uv : F[X] × F[X],
        uv ≠ 0 ∧ uv ∈ kernel x y dU dV (prefixSupport choice) := by
  constructor
  · rintro ⟨⟨u,v⟩, hne, h⟩
    obtain ⟨choice, hc⟩ := (symbolic_mem_iff_exists_kernel x y dU dV u v).mp h
    exact ⟨choice, (u,v), hne, hc⟩
  · rintro ⟨choice, ⟨u,v⟩, hne, hc⟩
    exact ⟨(u,v), hne, (symbolic_mem_iff_exists_kernel x y dU dV u v).mpr ⟨choice, hc⟩⟩

end CryptoLean.InterpolationKernels
