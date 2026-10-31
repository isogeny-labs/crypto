import GuardedExactPair
import Mathlib.RingTheory.MvPolynomial.MonomialOrder
import Mathlib.RingTheory.MvPolynomial.Ideal

/-! Solving degree uses the existing actual bounded-degree closure and the
standard leading-monomial divisibility criterion for a Gröbner basis over a
field. Regularity means fullness of a homogeneous piece of the top ideal,
not Castelnuovo--Mumford regularity. No field equations are inserted. -/
namespace DegreeInvariants
noncomputable section
open MvPolynomial GridLower FilteredClosure GuardedTops RationalFibers
variable {K : Type*} [Field K]

def topIdeal (f : Fin 6 → P K) : Ideal (P K) := Ideal.span (Set.range (top f))

/-- The defining graded-fullness test for the BFS degree of regularity. -/
def RegularityAt (f : Fin 6 → P K) (d : ℕ) : Prop :=
  ∀ p : P K, p.IsHomogeneous d → p ∈ topIdeal f

def auxiliaryPoint : Fin 5 → K := fun j => if j=0 then 1 else 0

theorem actual_tops_vanish (α β a b c r : K) (i : Fin 6) :
    eval auxiliaryPoint (top (generator α β a b c r) i) = 0 := by
  rw [generator_tops]
  fin_cases i <;> simp [auxiliaryPoint, firstTop, secondTop, FirstFallCore.guardTop]

theorem topIdeal_le_eval_kernel (α β a b c r : K) :
    topIdeal (generator α β a b c r) ≤ RingHom.ker (eval auxiliaryPoint) := by
  apply Ideal.span_le.mpr
  rintro p ⟨i,rfl⟩
  exact actual_tops_vanish α β a b c r i

/-- A concrete homogeneous witness survives in every degree, including zero. -/
theorem auxiliary_power_not_in_topIdeal (α β a b c r : K) (d : ℕ) :
    (X 0 : P K)^d ∉ topIdeal (generator α β a b c r) := by
  intro h
  have hz := topIdeal_le_eval_kernel α β a b c r h
  simp [RingHom.mem_ker,auxiliaryPoint] at hz

/-- No finite BFS degree of regularity, without UNSAT or curve hypotheses. -/
theorem no_finite_regularity (α β a b c r : K) (d : ℕ) :
    ¬ RegularityAt (generator α β a b c r) d := by
  intro h
  exact auxiliary_power_not_in_topIdeal α β a b c r d
    (h _ (isHomogeneous_X_pow 0 d))

/-- Exponent comparison is componentwise monomial divisibility. -/
def IsGroebnerBasisFor (m : MonomialOrder (Fin 5)) (F G : Set (P K)) : Prop :=
  G ⊆ Ideal.span F ∧
  ∀ p ∈ Ideal.span F, p ≠ 0 → ∃ g ∈ G, g ≠ 0 ∧ m.degree g ≤ m.degree p

def SolvesAt (m : MonomialOrder (Fin 5)) (F : Set (P K)) (d : ℕ) : Prop :=
  ∃ G : Set (P K), IsGroebnerBasisFor m F G ∧ ∀ g ∈ G, Derivable F d g

def IsClosureSolvingDegree (m : MonomialOrder (Fin 5)) (F : Set (P K)) (d : ℕ) : Prop :=
  SolvesAt m F d ∧ ∀ e < d, ¬ SolvesAt m F e

theorem singleton_one_basis (m : MonomialOrder (Fin 5)) (F : Set (P K))
    (hunit : (1 : P K) ∈ Ideal.span F) : IsGroebnerBasisFor m F {1} := by
  constructor
  · intro p hp
    have he := Set.mem_singleton_iff.mp hp
    simpa [he] using hunit
  · intro p hp hn
    refine ⟨1,by simp,one_ne_zero,?_⟩
    simp only [m.degree_one]
    exact bot_le

/-- On a unit ideal, obtaining any Gröbner basis is equivalent to deriving one. -/
theorem solvesAt_iff_refutation (m : MonomialOrder (Fin 5)) (F : Set (P K))
    (hunit : (1 : P K) ∈ Ideal.span F) (d : ℕ) :
    SolvesAt m F d ↔ Derivable F d 1 := by
  constructor
  · rintro ⟨G,hG,hclose⟩
    obtain ⟨g,hg,hgn,hdeg⟩ := hG.2 1 hunit one_ne_zero
    have he : m.degree g = 0 := by
      apply le_antisymm
      · simpa using hdeg
      · exact bot_le
    obtain ⟨c,hc⟩ : ∃ c : K, g = C c := ⟨_,m.eq_C_of_degree_eq_zero he⟩
    have hcn : c ≠ 0 := by
      intro h
      apply hgn
      simp [hc,h]
    have hd := Derivable.smul c⁻¹ (hclose g hg)
    simpa [hc,smul_eq_C_mul,← map_mul,hcn] using hd
  · intro h
    refine ⟨{1},singleton_one_basis m F hunit,?_⟩
    intro g hg
    have he := Set.mem_singleton_iff.mp hg
    simpa [he] using h

/-- Sharp closed-Macaulay solving degree, with all original curve hypotheses. -/
theorem solving_degree_prime (p : ℕ) [Fact p.Prime] (hp3 : 3<p)
    (m : MonomialOrder (Fin 5)) (α β a b c r Y : ZMod p)
    (hab : a ≠ b) (hac : a ≠ c) (hbc : b ≠ c)
    (_hdisc : 4*α^3+27*β^2 ≠ 0) (_htarget : Y^2=r^3+α*r+β)
    (hlift : ∀ x ∈ ({a,b,c} : Set (ZMod p)), ∃ Z : ZMod p, Z^2=x^3+α*x+β)
    (hns : NoSolution α β a b c r) :
    IsClosureSolvingDegree m (GuardedModel.inputs α β a b c r) 5 := by
  have h6 := GuardedExactPair.prime_six_ne_zero p hp3
  have h2 : (2 : ZMod p) ≠ 0 := by
    intro h
    apply h6
    calc
      (6 : ZMod p) = 2*3 := by ring
      _ = 0 := by rw [h,zero_mul]
  have href := GuardedLastFall.upper_five α β a b c r hab hac hbc h2 hlift hns
  have hu := mem_span href
  constructor
  · exact (solvesAt_iff_refutation m _ hu 5).mpr href
  · intro e he hs
    exact GuardedModel.no_refutation_below_five α β a b c r hab hac hbc h6 e he
      ((solvesAt_iff_refutation m _ hu e).mp hs)

/-- The ordinary-binary Semaev formula; the curve's a2 does not occur. -/
def binaryS3 (β : K) (s t w : P K) : P K := (s*t+s*w+t*w)^2+s*t*w+C β

def binaryFirstTop : P K := (X 1*X 2+X 1*X 0+X 2*X 0)^2

theorem binary_first_top (β : K) :
    (binaryS3 β (X 1) (X 2) (X 0)).totalDegree = 4 ∧
    homogeneousComponent 4 (binaryS3 β (X 1) (X 2) (X 0)) = binaryFirstTop := by
  have hxy := (isHomogeneous_X (R := K) (1 : Fin 5)).mul (isHomogeneous_X K 2)
  have hxu := (isHomogeneous_X (R := K) (1 : Fin 5)).mul (isHomogeneous_X K 0)
  have hyu := (isHomogeneous_X (R := K) (2 : Fin 5)).mul (isHomogeneous_X K 0)
  have hhom : (binaryFirstTop : P K).IsHomogeneous 4 := by
    simpa [binaryFirstTop] using (((hxy.add hxu).add hyu).pow 2)
  have hne : (binaryFirstTop : P K) ≠ 0 := by
    intro h
    have hh := congrArg (eval (fun j : Fin 5 => if j=1 ∨ j=2 then (1 : K) else 0)) h
    simp [binaryFirstTop] at hh
  have hxy' := totalDegree_mul (X 1 : P K) (X 2)
  have hxyz := totalDegree_mul (X 1*X 2 : P K) (X 0)
  have hlow := totalDegree_add (X 1*X 2*X 0 : P K) (C β)
  simp only [totalDegree_X,totalDegree_C] at hxy' hxyz hlow
  have heq : binaryS3 β (X 1) (X 2) (X 0) = binaryFirstTop+(X 1*X 2*X 0+C β) := by
    simp [binaryS3,binaryFirstTop]; ring
  rw [heq]
  exact top_and_degree _ _ 4 hhom hne (by omega)

def binarySecondLow (β r : K) : P K :=
  C 2*(X 0*X 3)*(C r*(X 0+X 3))+(C r*(X 0+X 3))^2+C r*(X 0*X 3)+C β

theorem binary_second_low_degree (β r : K) :
    (binarySecondLow β r).totalDegree ≤ 3 := by
  have hsum := totalDegree_add (X 0 : P K) (X 3)
  have hlin := totalDegree_mul (C r : P K) (X 0+X 3)
  have hquad := totalDegree_mul (X 0 : P K) (X 3)
  have ht := totalDegree_mul (C 2 : P K) (X 0*X 3)
  have hc := totalDegree_mul (C 2*(X 0*X 3) : P K) (C r*(X 0+X 3))
  have hs := totalDegree_pow (C r*(X 0+X 3) : P K) 2
  have hr := totalDegree_mul (C r : P K) (X 0*X 3)
  have ha := totalDegree_add (C 2*(X 0*X 3)*(C r*(X 0+X 3)) : P K)
    ((C r*(X 0+X 3))^2)
  have hb := totalDegree_add
    (C 2*(X 0*X 3)*(C r*(X 0+X 3))+(C r*(X 0+X 3))^2 : P K) (C r*(X 0*X 3))
  have hd := totalDegree_add
    (C 2*(X 0*X 3)*(C r*(X 0+X 3))+(C r*(X 0+X 3))^2+C r*(X 0*X 3) : P K) (C β)
  simp only [totalDegree_C,totalDegree_X] at hsum hlin hquad ht hr hd
  unfold binarySecondLow
  omega

theorem binary_second_top (β r : K) :
    (binaryS3 β (X 0) (X 3) (C r)).totalDegree = 4 ∧
    homogeneousComponent 4 (binaryS3 β (X 0) (X 3) (C r)) = secondTop := by
  have heq : binaryS3 β (X 0) (X 3) (C r) = secondTop+binarySecondLow β r := by
    simp [binaryS3,secondTop,binarySecondLow,map_ofNat]; ring
  have hhom : (secondTop : P K).IsHomogeneous 4 :=
    (isHomogeneous_X_pow (R := K) 3 2).mul (isHomogeneous_X_pow 0 2)
  have hne : (secondTop : P K) ≠ 0 :=
    mul_ne_zero (pow_ne_zero 2 (X_ne_zero 3)) (pow_ne_zero 2 (X_ne_zero 0))
  rw [heq]
  exact top_and_degree _ _ 4 hhom hne (by have h := binary_second_low_degree β r; omega)

/-- Exactly the binary six-input polynomial presentation, in order u,x,y,z,v.
In characteristic two its last generator is v*(x+y)+1. -/
def binaryGenerator (β a b c r : K) (i : Fin 6) : P K :=
  if i=0 then GuardedModel.membership a b c (X 1) else
  if i=1 then GuardedModel.membership a b c (X 2) else
  if i=2 then GuardedModel.membership a b c (X 3) else
  if i=3 then binaryS3 β (X 1) (X 2) (X 0) else
  if i=4 then binaryS3 β (X 0) (X 3) (C r) else GuardedModel.guard

theorem binary_generator_tops (β a b c r : K) (i : Fin 6) :
    top (binaryGenerator β a b c r) i =
      if i=0 then X 1^3 else if i=1 then X 2^3 else if i=2 then X 3^3 else
      if i=3 then binaryFirstTop else if i=4 then secondTop else FirstFallCore.guardTop := by
  fin_cases i
  · simpa [top,binaryGenerator,(membership_top a b c 1).1] using (membership_top a b c 1).2
  · simpa [top,binaryGenerator,(membership_top a b c 2).1] using (membership_top a b c 2).2
  · simpa [top,binaryGenerator,(membership_top a b c 3).1] using (membership_top a b c 3).2
  · simpa [top,binaryGenerator,(binary_first_top β).1] using (binary_first_top β).2
  · simpa [top,binaryGenerator,(binary_second_top β r).1] using (binary_second_top β r).2
  · simpa [top,binaryGenerator,(guard_top (K := K)).1] using (guard_top (K := K)).2

/-- The same explicit surviving auxiliary power for the binary formula.
No first/last-fall assertion for this binary model is imported or assumed. -/
theorem binary_no_finite_regularity (β a b c r : K) (d : ℕ) :
    ¬ RegularityAt (binaryGenerator β a b c r) d := by
  have hk : topIdeal (binaryGenerator β a b c r) ≤ RingHom.ker (eval auxiliaryPoint) := by
    apply Ideal.span_le.mpr
    rintro p ⟨i,rfl⟩
    change eval auxiliaryPoint (top (binaryGenerator β a b c r) i) = 0
    rw [binary_generator_tops]
    fin_cases i <;> simp [auxiliaryPoint,binaryFirstTop,secondTop,FirstFallCore.guardTop]
  intro h
  have hz := hk (h _ (isHomogeneous_X_pow 0 d))
  simp [RingHom.mem_ker,auxiliaryPoint] at hz

/-- Explicit field equations really add new top forms for q>1. -/
theorem field_equation_top (q : ℕ) (hq : 1<q) (j : Fin 5) :
    ((X j : P K)^q-X j).totalDegree = q ∧
    homogeneousComponent q ((X j : P K)^q-X j) = X j^q := by
  have h := top_and_degree ((X j : P K)^q) (-X j) q
    (isHomogeneous_X_pow j q) (pow_ne_zero q (X_ne_zero j)) (by simpa using hq)
  simpa [sub_eq_add_neg] using h

def augmentedTopIdeal (f : Fin 6 → P K) (q : ℕ) : Ideal (P K) :=
  topIdeal f ⊔ Ideal.span (Set.range (fun j : Fin 5 => (X j : P K)^q))

/-- Control: the raw-model witness stops separating once field tops are added. -/
theorem auxiliary_power_in_augmented (f : Fin 6 → P K) (q d : ℕ) (hd : q≤d) :
    (X 0 : P K)^d ∈ augmentedTopIdeal f q := by
  have hu : (X 0 : P K)^q ∈ augmentedTopIdeal f q :=
    (show Ideal.span (Set.range (fun j : Fin 5 => (X j : P K)^q)) ≤
      augmentedTopIdeal f q from le_sup_right) (Ideal.subset_span ⟨0,rfl⟩)
  have hm := (augmentedTopIdeal f q).mul_mem_left ((X 0 : P K)^(d-q)) hu
  simpa [← pow_add,Nat.sub_add_cancel hd] using hm

theorem augmented_not_annihilated (f : Fin 6 → P K) (q : ℕ) :
    ¬ augmentedTopIdeal f q ≤ RingHom.ker (eval auxiliaryPoint) := by
  intro h
  have hz := h (auxiliary_power_in_augmented f q q le_rfl)
  simp [RingHom.mem_ker,auxiliaryPoint] at hz

/-- Pigeonhole bound for the presentation with explicit field equations.
It is a bound for graded fullness, not an assertion about a solver run. -/
theorem augmented_full_at_bound (f : Fin 6 → P K) (q d : ℕ)
    (hd : 5*(q-1)<d) (p : P K) (hp : p.IsHomogeneous d) :
    p ∈ augmentedTopIdeal f q := by
  classical
  have hm : p ∈ Ideal.span ((fun s : Fin 5 →₀ ℕ => monomial s (1 : K)) ''
      Set.range (fun j : Fin 5 => Finsupp.single j q)) := by
    apply mem_ideal_span_monomial_image.mpr
    intro s hs
    have hsum : (∑ j : Fin 5, s j) = d := by
      calc
        (∑ j : Fin 5, s j) = s.sum (fun _ n => n) :=
          (Finsupp.sum_fintype s (fun _ n => n) (by simp)).symm
        _ = d := (hp.degree_eq_sum_deg_support hs).symm
    have hj : ∃ j : Fin 5, q ≤ s j := by
      by_contra hn
      push Not at hn
      have hle : (∑ j : Fin 5, s j) ≤ 5*(q-1) := by
        calc
          _ ≤ ∑ _j : Fin 5, (q-1) := Finset.sum_le_sum (fun j _ => by
            have h := hn j
            omega)
          _ = _ := by simp
      omega
    obtain ⟨j,hj⟩ := hj
    refine ⟨Finsupp.single j q,⟨j,rfl⟩,?_⟩
    intro k
    by_cases hk : k=j
    · subst k
      simpa using hj
    · simp [hk]
  have he : (fun s : Fin 5 →₀ ℕ => monomial s (1 : K)) ''
      Set.range (fun j : Fin 5 => Finsupp.single j q) =
      Set.range (fun j : Fin 5 => (X j : P K)^q) := by
    rw [← Set.range_comp]
    simp only [Function.comp_def,← X_pow_eq_monomial]
  rw [he] at hm
  exact (show Ideal.span (Set.range (fun j : Fin 5 => (X j : P K)^q)) ≤
    augmentedTopIdeal f q from le_sup_right) hm

def fieldEquations (q : ℕ) : Set (P K) :=
  Set.range (fun j : Fin 5 => (X j : P K)^q-X j)

def augmentedInputs (F : Set (P K)) (q : ℕ) : Set (P K) := F ∪ fieldEquations q

theorem derivable_enlarge {F G : Set (P K)} (hFG : F ⊆ G) {d : ℕ} {p : P K}
    (h : Derivable F d p) : Derivable G d p := by
  induction h with
  | zero => exact .zero
  | input hf hd => exact .input (hFG hf) hd
  | add _ _ hp hq => exact .add hp hq
  | smul a _ hp => exact .smul a hp
  | mul q _ hd hp => exact .mul q hp hd

/-- Higher-degree field equations cannot enter a lower-degree derivation. -/
theorem ignore_high_field_equations (F : Set (P K)) (q d : ℕ)
    (hq : 1<q) (hd : d<q) (p : P K) :
    Derivable (augmentedInputs F q) d p ↔ Derivable F d p := by
  constructor
  · intro h
    induction h with
    | zero => exact .zero
    | input hf hdeg =>
      rcases hf with hf | ⟨j,rfl⟩
      · exact .input hf hdeg
      · have he := (field_equation_top (K := K) q hq j).1
        have hdeg' : ((X j : P K)^q-X j).totalDegree ≤ d := hdeg
        omega
    | add _ _ hp hq => exact .add hp hq
    | smul a _ hp => exact .smul a hp
    | mul q _ hd hp => exact .mul q hp hd
  · exact derivable_enlarge Set.subset_union_left

/-- For the original prime-field hypotheses, explicitly adjoining all five
field equations leaves the exact closed solving degree equal to five. -/
theorem solving_degree_prime_with_field_equations (p : ℕ) [Fact p.Prime] (hp3 : 3<p)
    (m : MonomialOrder (Fin 5)) (α β a b c r Y : ZMod p)
    (hab : a ≠ b) (hac : a ≠ c) (hbc : b ≠ c)
    (hdisc : 4*α^3+27*β^2 ≠ 0) (htarget : Y^2=r^3+α*r+β)
    (hlift : ∀ x ∈ ({a,b,c} : Set (ZMod p)), ∃ Z : ZMod p, Z^2=x^3+α*x+β)
    (hns : NoSolution α β a b c r) :
    IsClosureSolvingDegree m (augmentedInputs (GuardedModel.inputs α β a b c r) p) 5 := by
  have hp5 : 5≤p := by
    have hp4 : p≠4 := by
      intro h
      have hpr : Nat.Prime 4 := by simpa [h] using (Fact.out : p.Prime)
      exact (by decide : ¬ Nat.Prime 4) hpr
    omega
  have hr := solving_degree_prime p hp3 m α β a b c r Y hab hac hbc hdisc htarget hlift hns
  have hu : (1 : P (ZMod p)) ∈ Ideal.span (GuardedModel.inputs α β a b c r) := by
    -- The raw theorem constructs a refutation, so the unit ideal is a conclusion.
    have h2 : (2 : ZMod p) ≠ 0 := by
      intro h
      have hh := (ZMod.natCast_eq_zero_iff 2 p).mp h
      have hle := Nat.le_of_dvd (by decide : 0<2) hh
      omega
    exact mem_span (GuardedLastFall.upper_five α β a b c r hab hac hbc h2 hlift hns)
  have href := (solvesAt_iff_refutation m _ hu 5).mp hr.1
  have ha := derivable_enlarge (G := augmentedInputs (GuardedModel.inputs α β a b c r) p)
    Set.subset_union_left href
  have hau := mem_span ha
  constructor
  · exact (solvesAt_iff_refutation m _ hau 5).mpr ha
  · intro e he hs
    have heq := (solvesAt_iff_refutation m _ hau e).mp hs
    have hraw := (ignore_high_field_equations _ p e (by omega) (by omega) 1).mp heq
    exact hr.2 e he ((solvesAt_iff_refutation m _ hu e).mpr hraw)

theorem exact_solving_iff_last_fall (m : MonomialOrder (Fin 5)) (F : Set (P K))
    (hunit : (1 : P K) ∈ Ideal.span F) (d : ℕ) :
    IsClosureSolvingDegree m F d ↔ GuardedLastFall.IsLastFallDegree F d := by
  simp only [IsClosureSolvingDegree,GuardedLastFall.IsLastFallDegree,
    lastFallBound_iff_refutation hunit,solvesAt_iff_refutation m F hunit]

/-- The explicitly field-constrained prime presentation also has exact last
fall five. The hypotheses are those of the original theorem, not a unit-ideal
or refutation premise. -/
theorem last_fall_prime_with_field_equations (p : ℕ) [Fact p.Prime] (hp3 : 3<p)
    (α β a b c r Y : ZMod p) (hab : a ≠ b) (hac : a ≠ c) (hbc : b ≠ c)
    (hdisc : 4*α^3+27*β^2 ≠ 0) (htarget : Y^2=r^3+α*r+β)
    (hlift : ∀ x ∈ ({a,b,c} : Set (ZMod p)), ∃ Z : ZMod p, Z^2=x^3+α*x+β)
    (hns : NoSolution α β a b c r) :
    GuardedLastFall.IsLastFallDegree
      (augmentedInputs (GuardedModel.inputs α β a b c r) p) 5 := by
  have h2 : (2 : ZMod p) ≠ 0 := by
    intro h
    have hh := (ZMod.natCast_eq_zero_iff 2 p).mp h
    have hle := Nat.le_of_dvd (by decide : 0<2) hh
    omega
  have href := derivable_enlarge (G := augmentedInputs (GuardedModel.inputs α β a b c r) p)
    Set.subset_union_left (GuardedLastFall.upper_five α β a b c r hab hac hbc h2 hlift hns)
  exact (exact_solving_iff_last_fall MonomialOrder.degLex _ (mem_span href) 5).mp
    (solving_degree_prime_with_field_equations p hp3 MonomialOrder.degLex α β a b c r Y
      hab hac hbc hdisc htarget hlift hns)

end
end DegreeInvariants

#print axioms DegreeInvariants.no_finite_regularity
#print axioms DegreeInvariants.solvesAt_iff_refutation
#print axioms DegreeInvariants.solving_degree_prime
#print axioms DegreeInvariants.binary_no_finite_regularity
#print axioms DegreeInvariants.field_equation_top
#print axioms DegreeInvariants.auxiliary_power_in_augmented
#print axioms DegreeInvariants.augmented_not_annihilated
#print axioms DegreeInvariants.augmented_full_at_bound
#print axioms DegreeInvariants.solving_degree_prime_with_field_equations
#print axioms DegreeInvariants.last_fall_prime_with_field_equations
