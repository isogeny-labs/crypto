module
public import NormTargetGrading
public import Mathlib.Algebra.MvPolynomial.Degrees
public import Mathlib.LinearAlgebra.Span.Basic
import all NormTargetGrading
import all NormTargetFiber
public section

/-!
Full iterated budgeted closure, using the literal multiplier budget of
ml-cryptanalysis formal/ICFallDegrees/Closure.lean (8020e131a).
The degree function is explicit: ordinary and weighted degree are separate
instantiations. No static-Macaulay substitution or solver bound is assumed.
-/
namespace NormRowClosure
open Polynomial
variable {K A : Type*} [Field K] [CommRing A] [Algebra K A]

inductive InClosure (K : Type*) {A : Type*} [Field K] [CommRing A] [Algebra K A] (deg : A → ℕ) (F : Set A) (D : ℕ) : A → Prop
  | input {f} (member : f∈F) (degree : deg f≤D) : InClosure K deg F D f
  | zero : InClosure K deg F D 0
  | add {f g} : InClosure K deg F D f → InClosure K deg F D g → InClosure K deg F D (f+g)
  | smul (c : K) {f} : InClosure K deg F D f → InClosure K deg F D (c • f)
  | mul (a : A) {f} : InClosure K deg F D f → deg a+deg f≤D → InClosure K deg F D (a*f)

def closureSpace (deg : A → ℕ) (F : Set A) (D : ℕ) : Submodule K A where
  carrier := {f | InClosure K deg F D f}
  zero_mem' := .zero
  add_mem' := InClosure.add
  smul_mem' := InClosure.smul

theorem transport (deg : A → ℕ) (F G : Set A) (D : ℕ)
    (inputs : ∀ f∈F, deg f≤D → InClosure K deg G D f) {p : A}
    (h : InClosure K deg F D p) : InClosure K deg G D p := by
  induction h with
  | input member degree => exact inputs _ member degree
  | zero => exact .zero
  | add _ _ ih ih' => exact .add ih ih'
  | smul c _ ih => exact .smul c ih
  | mul a _ budget ih => exact .mul a ih budget

/-- Every admitted input span is present before any polynomial multiplication. -/
theorem inputSpan (deg : A → ℕ) (F : Set A) (D : ℕ)
    (admitted : ∀ f∈F, deg f≤D) : Submodule.span K F≤closureSpace deg F D := by
  apply Submodule.span_le.mpr
  intro f hf
  exact InClosure.input hf (admitted f hf)

/-- Constant row-span changes preserve full closure above input admission.
No property of the degree function, or a completed GB, is assumed here. -/
theorem closureSpanInvariant (deg : A → ℕ) (F G : Set A) (D : ℕ)
    (hF : ∀ f∈F, deg f≤D) (hG : ∀ g∈G, deg g≤D)
    (same : Submodule.span K F=Submodule.span K G) (p : A) :
    InClosure K deg F D p ↔ InClosure K deg G D p := by
  constructor
  · apply transport deg F G D
    intro f hf _
    apply inputSpan deg G D hG
    rw [← same]
    exact Submodule.subset_span hf
  · apply transport deg G F D
    intro g hg _
    apply inputSpan deg F D hF
    rw [same]
    exact Submodule.subset_span hg

def oldRows (V : A[X]) : Set A := Set.range (fun i : Fin 5 => V.coeff i.val)
def newRows (V : A[X]) (q c : K) : Set A :=
  insert (c • V.eval (algebraMap K A q))
    (Set.range (fun i : Fin 4 => V.coeff (i.val+1)))

/-- Actual bounded-polynomial evaluation is a public constant row combination. -/
theorem evaluationRows (V : A[X]) (hV : V.natDegree≤4) (q : K) :
    V.eval (algebraMap K A q)=V.coeff 0+q • V.coeff 1+
      q^2 • V.coeff 2+q^3 • V.coeff 3+q^4 • V.coeff 4 := by
  rw [Polynomial.eval_eq_sum_range' (by omega : V.natDegree<5)]
  norm_num [Finset.sum_range_succ,Algebra.smul_def,map_pow]
  ring

theorem rowsMembership (W : Submodule K A) (V : A[X]) (hV : V.natDegree≤4)
    (q c : K) (hc : c≠0) :
    (∀ i : Fin 5, V.coeff i.val∈W) ↔
      c • V.eval (algebraMap K A q)∈W ∧ ∀ i : Fin 4, V.coeff (i.val+1)∈W := by
  have tail (h : ∀ i : Fin 4, V.coeff (i.val+1)∈W) :
      q • V.coeff 1+q^2 • V.coeff 2+q^3 • V.coeff 3+q^4 • V.coeff 4∈W := by
    exact W.add_mem (W.add_mem (W.add_mem
      (W.smul_mem q (h 0)) (W.smul_mem (q^2) (h 1)))
      (W.smul_mem (q^3) (h 2))) (W.smul_mem (q^4) (h 3))
  constructor
  · intro h
    have ht : ∀ i : Fin 4, V.coeff (i.val+1)∈W := fun i => h ⟨i.val+1,by omega⟩
    refine ⟨W.smul_mem c ?_,ht⟩
    rw [evaluationRows V hV q]
    simpa [add_assoc] using W.add_mem (h 0) (tail ht)
  · rintro ⟨he,ht⟩
    have hev : V.eval (algebraMap K A q)∈W := by
      simpa only [smul_smul,inv_mul_cancel₀ hc,one_smul] using W.smul_mem c⁻¹ he
    have hz : V.coeff 0∈W := by
      have hh := W.sub_mem hev (tail ht)
      rw [evaluationRows V hV q] at hh
      convert hh using 1; abel
    intro i
    fin_cases i
    · exact hz
    · exact ht 0
    · exact ht 1
    · exact ht 2
    · exact ht 3

/-- Equality of actual generator spans, not only equality of zero sets. -/
theorem rowSpans (V : A[X]) (hV : V.natDegree≤4) (q c : K) (hc : c≠0) :
    Submodule.span K (oldRows V)=Submodule.span K (newRows V q c) := by
  have old (W : Submodule K A) :
      oldRows V⊆W ↔ ∀ i : Fin 5, V.coeff i.val∈W := by
    simp only [oldRows,Set.range_subset_iff,SetLike.mem_coe]
  have new (W : Submodule K A) :
      newRows V q c⊆W ↔ c • V.eval (algebraMap K A q)∈W ∧
        ∀ i : Fin 4, V.coeff (i.val+1)∈W := by
    simp only [newRows,Set.insert_subset_iff,Set.range_subset_iff,SetLike.mem_coe]
  apply le_antisymm
  · apply Submodule.span_le.mpr
    apply (old _).mpr
    apply (rowsMembership _ V hV q c hc).mpr
    exact (new _).mp Submodule.subset_span
  · apply Submodule.span_le.mpr
    apply (new _).mpr
    apply (rowsMembership _ V hV q c hc).mp
    exact (old _).mp Submodule.subset_span

/-- Full degree-budgeted closure of the actual row change is identical once
both presentations' inputs are admitted. This includes newly derived rows. -/
theorem rowClosure (deg : A → ℕ) (V : A[X]) (hV : V.natDegree≤4)
    (q c : K) (hc : c≠0) (D : ℕ)
    (hOld : ∀ f∈oldRows V, deg f≤D) (hNew : ∀ f∈newRows V q c, deg f≤D) (p : A) :
    InClosure K deg (oldRows V) D p ↔ InClosure K deg (newRows V q c) D p :=
  closureSpanInvariant deg _ _ D hOld hNew (rowSpans V hV q c hc) p

/-- Degree guard for the actual monic remainder, over any nontrivial algebra. -/
theorem actualMembershipWindow [Nontrivial A] (e a b r z a1 a2 c0 c1 terminal : A)
    (hp : z^2=r^3+a*r+b) (n : ℕ) :
    (NormTargetFiber.membership e a b r z a1 a2 c0 c1 terminal n).natDegree≤4 := by
  have hu := NormTargetFiber.scaledSupportWindowRing e a b r z a1 a2 c0 c1 hp
  have hn : NormTargetFiber.scaledSupport e a b r z a1 a2 c0 c1≠1 := by
    intro hh
    have hh' := congrArg Polynomial.natDegree hh
    rw [hu.2,natDegree_one] at hh'
    contradiction
  have hd := natDegree_modByMonic_lt (NormTargetFiber.dividend e terminal n) hu.1 hn
  rw [hu.2] at hd
  exact Nat.le_of_lt_succ hd

/-- The actual normalized row is a public nonzero field-scalar recombination
in the coefficient algebra itself, not only at its finite-field points. -/
theorem actualNormalizedRow (e a b r z : K) (he : e≠0) (hp : z^2=r^3+a*r+b)
    (a1 a2 c0 c1 terminal : A) (n : ℕ) (hd : 3≤2^(n+1)) :
    ((e^3)⁻¹) •
      (NormTargetFiber.membership (algebraMap K A e) (algebraMap K A a)
        (algebraMap K A b) (algebraMap K A r) (algebraMap K A z)
        a1 a2 c0 c1 terminal n).eval (algebraMap K A (e^2*r))=
      NormTargetGrading.normalized (algebraMap K A e) (algebraMap K A a)
        (algebraMap K A b) (algebraMap K A r) (algebraMap K A z)
        a1 a2 c0 c1 terminal n := by
  have hpA : (algebraMap K A z)^2=(algebraMap K A r)^3+
      algebraMap K A a*algebraMap K A r+algebraMap K A b := by
    simpa only [map_pow,map_add,map_mul] using congrArg (algebraMap K A) hp
  rw [map_mul,map_pow,NormTargetGrading.actualTargetFactor _ _ _ _ _ _ _ _ _ _ hpA n hd]
  simp only [← map_pow,← Algebra.smul_def,smul_smul,inv_mul_cancel₀ (pow_ne_zero 3 he),one_smul]

/-- Actual old and normalized input rows have identical full closure above
both input-admission thresholds. A may be a multivariate polynomial ring. -/
theorem actualRowClosure [Nontrivial A] (deg : A → ℕ)
    (e a b r z : K) (he : e≠0) (hp : z^2=r^3+a*r+b)
    (a1 a2 c0 c1 terminal : A) (n : ℕ) (hd : 3≤2^(n+1)) (D : ℕ) :
    let V := NormTargetFiber.membership (algebraMap K A e) (algebraMap K A a)
      (algebraMap K A b) (algebraMap K A r) (algebraMap K A z) a1 a2 c0 c1 terminal n
    let T := NormTargetGrading.normalized (algebraMap K A e) (algebraMap K A a)
      (algebraMap K A b) (algebraMap K A r) (algebraMap K A z) a1 a2 c0 c1 terminal n
    let G := insert T (Set.range (fun i : Fin 4 => V.coeff (i.val+1)))
    (∀ f∈oldRows V, deg f≤D) → (∀ g∈G, deg g≤D) → ∀ p,
      InClosure K deg (oldRows V) D p ↔ InClosure K deg G D p := by
  dsimp only
  intro hOld hNew p
  have hpA : (algebraMap K A z)^2=(algebraMap K A r)^3+
      algebraMap K A a*algebraMap K A r+algebraMap K A b := by
    simpa only [map_pow,map_add,map_mul] using congrArg (algebraMap K A) hp
  have ht := actualNormalizedRow e a b r z he hp a1 a2 c0 c1 terminal n hd
  have hNew' := hNew
  rw [← ht] at hNew' ⊢
  exact rowClosure deg _ (actualMembershipWindow _ _ _ _ _ _ _ _ _ _ hpA n)
    (e^2*r) ((e^3)⁻¹) (inv_ne_zero (pow_ne_zero 3 he)) D hOld hNew' p

end NormRowClosure

#print axioms NormRowClosure.transport
#print axioms NormRowClosure.inputSpan
#print axioms NormRowClosure.closureSpanInvariant
#print axioms NormRowClosure.evaluationRows
#print axioms NormRowClosure.rowsMembership
#print axioms NormRowClosure.rowSpans
#print axioms NormRowClosure.rowClosure
#print axioms NormRowClosure.actualMembershipWindow
#print axioms NormRowClosure.actualNormalizedRow
#print axioms NormRowClosure.actualRowClosure
