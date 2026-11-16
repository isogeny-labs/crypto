import Mathlib

/-!
# Nagao Lemma 4 holds in the literal-`≥`, field-equations-inside cell

Corpus target: `CLAIM-NAGAO-LEMMA4` (WEAKNESS_TRIAGE W4). The repository refutes
`d_F ≤ d'_F` in three of the four (reading × placement) cells: equality reading
with the field equations outside or inside, and literal `≥` reading outside. The
fourth cell (literal `≥`, inside) was open. This file proves the lemma there, for
every system over every field and every finite family of "field equations".

Definitions (literal reading). A *fall at degree `d`* of a generating family `G`
is a tuple `g` with `max_i deg(g_i G_i) ≥ d`, `deg(Σ g_i G_i) < d` and
`Σ g_i G_i ≠ 0`. The *fake* version takes every degree after reduction modulo
the ideal `J` of the field equations and asks `Σ g_i G_i ∉ J`. The degree after
reduction is the least total degree of a representative of the class modulo `J`
(`fdeg`). For `J = ⟨x_j² − x_j⟩` this is the degree of the multilinear normal
form, since reduction never raises degree.

The proof lifts a fake fall: replace the sum by a least-degree representative,
paying the difference with field-equation multiples. That is allowed exactly
because the field equations are generators ("inside"). A witness index carrying
the degree must be a non-field-equation generator, because multiples of field
equations reduce to degree 0. Outside, or under the equality reading (the added
multiples can push the maximum above `d`), the argument fails, matching the
repository's counterexamples.
-/

open MvPolynomial
open scoped BigOperators

namespace NagaoLiteralInside

variable {σ K : Type*} [Field K]

local notation "P" => MvPolynomial σ K

/-- Degree after reduction modulo `J`: least total degree in the class of `p`. -/
noncomputable def fdeg (J : Ideal P) (p : P) : ℕ :=
  sInf {n | ∃ q : P, q - p ∈ J ∧ q.totalDegree = n}

theorem fdeg_spec (J : Ideal P) (p : P) :
    ∃ q : P, q - p ∈ J ∧ q.totalDegree = fdeg J p :=
  Nat.sInf_mem (s := {n | ∃ q : P, q - p ∈ J ∧ q.totalDegree = n})
    ⟨p.totalDegree, p, by simp, rfl⟩

theorem fdeg_le (J : Ideal P) (p : P) : fdeg J p ≤ p.totalDegree :=
  Nat.sInf_le ⟨p, by simp, rfl⟩

theorem fdeg_of_mem {J : Ideal P} {p : P} (hp : p ∈ J) : fdeg J p = 0 :=
  Nat.eq_zero_of_le_zero (Nat.sInf_le ⟨0, by simpa using J.neg_mem hp, by simp⟩)

/-- Literal (`≥`) fall at degree `d`. -/
def IsFallGE {α : Type*} [Fintype α] (G : α → P) (d : ℕ) : Prop :=
  ∃ g : α → P, (∃ i, d ≤ (g i * G i).totalDegree) ∧
    (∑ i, g i * G i).totalDegree < d ∧ ∑ i, g i * G i ≠ 0

/-- Literal (`≥`) fake fall at degree `d`: degrees after reduction modulo `J`. -/
def IsFakeFallGE {α : Type*} [Fintype α] (J : Ideal P) (G : α → P) (d : ℕ) : Prop :=
  ∃ g : α → P, (∃ i, d ≤ fdeg J (g i * G i)) ∧
    fdeg J (∑ i, g i * G i) < d ∧ (∑ i, g i * G i) ∉ J

variable {ι κ : Type*} [Fintype ι] [Fintype κ]

/-- **Inside placement, literal reading:** every fake fall is a true fall of the
generating family `F ∪ E` at the same degree. -/
theorem fake_fall_to_true_fall (F : ι → P) (E : κ → P) (d : ℕ)
    (h : IsFakeFallGE (Ideal.span (Set.range E)) (Sum.elim F E) d) :
    IsFallGE (Sum.elim F E) d := by
  classical
  set J := Ideal.span (Set.range E)
  obtain ⟨g, ⟨i, hi⟩, hlt, hnot⟩ := h
  set s := ∑ j, g j * Sum.elim F E j with hs
  obtain ⟨q, hqJ, hqdeg⟩ := fdeg_spec J s
  obtain ⟨c, hc⟩ := (Ideal.mem_span_range_iff_exists_fun).mp hqJ
  let g' : ι ⊕ κ → P := Sum.elim (fun a => g (Sum.inl a)) (fun k => g (Sum.inr k) + c k)
  have hsum : ∑ j, g' j * Sum.elim F E j = q := by
    have hsplit : s = (∑ a, g (Sum.inl a) * F a) + ∑ k, g (Sum.inr k) * E k := by
      rw [hs, Fintype.sum_sum_type]; simp
    rw [Fintype.sum_sum_type]
    simp only [g', Sum.elim_inl, Sum.elim_inr, add_mul, Finset.sum_add_distrib]
    have : q = s + ∑ k, c k * E k := by rw [hc]; ring
    rw [this, hsplit]; ring
  refine ⟨g', ?_, ?_, ?_⟩
  · rcases i with a | k
    · refine ⟨Sum.inl a, ?_⟩
      have := le_trans hi (fdeg_le J _)
      simpa [g'] using this
    · exfalso
      have hmem : g (Sum.inr k) * Sum.elim F E (Sum.inr k) ∈ J :=
        J.mul_mem_left _ (Ideal.subset_span ⟨k, rfl⟩)
      rw [fdeg_of_mem hmem] at hi
      omega
  · rw [hsum, hqdeg]; exact hlt
  · rw [hsum]
    intro hq0
    apply hnot
    have : -(q - s) ∈ J := J.neg_mem hqJ
    simpa [hq0] using this

/-- First fall degree (literal reading). -/
noncomputable def dF {α : Type*} [Fintype α] (G : α → P) : ℕ := sInf {d | IsFallGE G d}

/-- Fake first fall degree (literal reading). -/
noncomputable def dF' {α : Type*} [Fintype α] (J : Ideal P) (G : α → P) : ℕ :=
  sInf {d | IsFakeFallGE J G d}

/-- **Lemma 4, literal `≥` reading, field equations inside:** `d_F ≤ d'_F`
whenever a fake fall exists. -/
theorem lemma4_literal_inside (F : ι → P) (E : κ → P)
    (hex : ∃ d, IsFakeFallGE (Ideal.span (Set.range E)) (Sum.elim F E) d) :
    dF (Sum.elim F E) ≤ dF' (Ideal.span (Set.range E)) (Sum.elim F E) :=
  Nat.sInf_le (fake_fall_to_true_fall F E _ (Nat.sInf_mem hex))

end NagaoLiteralInside
