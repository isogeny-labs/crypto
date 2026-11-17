import GuardedTops

/-!+# Ordinary homogeneous-top first fall modulo graded Koszul relations

Degrees are the actual total degrees of the supplied polynomials. A relation
at degree d has a homogeneous multiplier of degree d-deg(f_i), or a zero
component when deg(f_i)>d. `KoszulAt` is the explicit homogeneous degree-d
span of the usual pair relations top(f_j)e_i-top(f_i)e_j. The six-generator
theorem below computes these objects; it does not assume their degree data.
-/
namespace WeightedFirstFall
noncomputable section
open MvPolynomial GridLower GuardedTops

variable {K : Type*} [Field K]

def Multiplier (d w : ℕ) (p : P K) : Prop :=
  if w ≤ d then p.IsHomogeneous (d-w) else p=0

def TopRelation (f : Fin 6 → P K) (d : ℕ) (v : Fin 6 → P K) : Prop :=
  (∀ i, Multiplier d (f i).totalDegree (v i)) ∧ ∑ i : Fin 6, v i*top f i = 0

def KoszulAt (f : Fin 6 → P K) (d : ℕ) (v : Fin 6 → P K) : Prop :=
  ∃ M : Fin 6 → Fin 6 → P K,
    (∀ i j, if i<j then Multiplier d ((f i).totalDegree+(f j).totalDegree) (M i j) else M i j=0) ∧
    ∀ k, v k = ∑ i : Fin 6, ∑ j : Fin 6,
      M i j*((if i=k then top f j else 0)-(if j=k then top f i else 0))

def FirstFallAt (f : Fin 6 → P K) (d : ℕ) : Prop :=
  ∃ v : Fin 6 → P K, TopRelation f d v ∧ ¬ KoszulAt f d v

def IsFirstFallDegree (f : Fin 6 → P K) (d : ℕ) : Prop :=
  FirstFallAt f d ∧ ∀ e < d, ¬ FirstFallAt f e

theorem zero_koszul (f : Fin 6 → P K) (d : ℕ) : KoszulAt f d 0 := by
  refine ⟨0,?_,?_⟩
  · intro i j
    split_ifs
    · unfold Multiplier
      split_ifs
      · exact isHomogeneous_zero (Fin 5) K _
      · rfl
    · rfl
  · intro k
    simp

theorem weight_pair_gt_four (i j : Fin 6) (h : i<j) : 4<weight i+weight j := by
  fin_cases i <;> fin_cases j <;> simp_all [weight]

theorem koszul_four_zero (α β a b c r : K) (v : Fin 6 → P K)
    (h : KoszulAt (generator α β a b c r) 4 v) : v=0 := by
  obtain ⟨M,hM,hv⟩ := h
  have hm (i j : Fin 6) : M i j=0 := by
    have hh := hM i j
    by_cases hij : i<j
    · have hn : ¬ ((generator α β a b c r i).totalDegree+
        (generator α β a b c r j).totalDegree ≤ 4) := by
        rw [generator_degrees,generator_degrees]
        exact Nat.not_le.mpr (weight_pair_gt_four i j hij)
      simpa only [if_pos hij,Multiplier,if_neg hn] using hh
    · simpa only [if_neg hij] using hh
  funext k
  rw [hv]
  simp [hm]

theorem relation_below_four_zero (α β a b c r : K) (d : ℕ) (hd : d<4)
    (v : Fin 6 → P K) (h : TopRelation (generator α β a b c r) d v) : v=0 := by
  have hc (i : Fin 6) (hi : (generator α β a b c r i).totalDegree=3) :
      ∃ t : K, v i=C t := by
    have hh := h.1 i
    rw [Multiplier,hi] at hh
    by_cases h3 : 3≤d
    · have hdeq : d=3 := by omega
      rw [if_pos h3,hdeq,Nat.sub_self] at hh
      have hdeg := (totalDegree_zero_iff_isHomogeneous (Fin 5)).mpr hh
      exact ⟨(v i).coeff 0,totalDegree_eq_zero_iff_eq_C.mp hdeg⟩
    · rw [if_neg h3] at hh
      exact ⟨0,by simp [hh]⟩
  obtain ⟨t₀,h₀⟩ := hc 0 (by simpa [weight] using generator_degrees α β a b c r 0)
  obtain ⟨t₁,h₁⟩ := hc 1 (by simpa [weight] using generator_degrees α β a b c r 1)
  obtain ⟨t₂,h₂⟩ := hc 2 (by simpa [weight] using generator_degrees α β a b c r 2)
  have h₃ : v 3=0 := by
    have hh := h.1 3
    rw [generator_degrees] at hh
    simpa [Multiplier,weight,show ¬4≤d by omega] using hh
  have h₄ : v 4=0 := by
    have hh := h.1 4
    rw [generator_degrees] at hh
    simpa [Multiplier,weight,show ¬4≤d by omega] using hh
  have hp : C t₀*X 1^3+C t₁*X 2^3+C t₂*X 3^3+v 5*FirstFallCore.guardTop=0 := by
    have hs := h.2
    simp [Fin.sum_univ_succ,generator_tops,h₀,h₁,h₂,h₃,h₄] at hs
    linear_combination hs
  obtain ⟨ht₀,ht₁,ht₂,h₅⟩ := FirstFallCore.cubic_guard_top_injective t₀ t₁ t₂ (v 5) hp
  funext i
  fin_cases i <;> simp [h₀,h₁,h₂,h₃,h₄,h₅,ht₀,ht₁,ht₂]

def witness : Fin 6 → P K := fun i =>
  if i=0 then X 4 else if i=1 then -X 4 else if i=5 then -FirstFallCore.pairTop else 0

theorem pairTop_homogeneous : (FirstFallCore.pairTop : P K).IsHomogeneous 2 :=
  ((isHomogeneous_X_pow (R := K) 1 2).add ((isHomogeneous_X K 1).mul (isHomogeneous_X K 2))).add
    (isHomogeneous_X_pow 2 2)

theorem witness_relation (α β a b c r : K) :
    TopRelation (generator α β a b c r) 4 witness := by
  refine ⟨?_,?_⟩
  · intro i
    rw [generator_degrees]
    fin_cases i
    · simpa [Multiplier,weight,witness] using (isHomogeneous_X K (4 : Fin 5))
    · simpa [Multiplier,weight,witness] using (isHomogeneous_X K (4 : Fin 5)).neg
    · simpa [Multiplier,weight,witness] using (isHomogeneous_zero (Fin 5) K 1 : (0 : P K).IsHomogeneous 1)
    · simpa [Multiplier,weight,witness] using (isHomogeneous_zero (Fin 5) K 0 : (0 : P K).IsHomogeneous 0)
    · simpa [Multiplier,weight,witness] using (isHomogeneous_zero (Fin 5) K 0 : (0 : P K).IsHomogeneous 0)
    · simpa [Multiplier,weight,witness] using (pairTop_homogeneous (K := K)).neg
  · simp [Fin.sum_univ_succ,generator_tops,witness]
    linear_combination (FirstFallCore.degree_four_top_relation (K := K))

theorem witness_not_koszul (α β a b c r : K) :
    ¬ KoszulAt (generator α β a b c r) 4 witness := by
  intro h
  have hh := congrFun (koszul_four_zero α β a b c r witness h) 0
  have hx : (X 4 : P K)=0 := by simpa [witness] using hh
  exact X_ne_zero 4 hx

/-- Exact first fall for the actual six generators under the stated
ordinary homogeneous-top/Koszul convention. -/
theorem exact_first_four (α β a b c r : K) :
    IsFirstFallDegree (generator α β a b c r) 4 := by
  refine ⟨⟨witness,witness_relation α β a b c r,witness_not_koszul α β a b c r⟩,?_⟩
  intro d hd h
  obtain ⟨v,hv,hnot⟩ := h
  have hz := relation_below_four_zero α β a b c r d hd v hv
  apply hnot
  rw [hz]
  exact zero_koszul _ _

def lift (f : Fin 6 → P K) (v : Fin 6 → P K) : P K := ∑ i : Fin 6, v i*f i

/-- The exhibited top relation genuinely falls rather than lifting to a
zero identity among the original inhomogeneous inputs. -/
theorem witness_lift_ne_zero (α β a b c r : K) :
    lift (generator α β a b c r) witness ≠ 0 := by
  intro h
  have hh := congrArg (eval (fun j : Fin 5 => if j=1 then (1 : K) else 0)) h
  simpa [lift,Fin.sum_univ_succ,witness,generator,GuardedModel.guard,FirstFallCore.pairTop] using hh

theorem witness_lift_identity (α β a b c r : K) :
    lift (generator α β a b c r) witness =
      -C (a+b+c)*X 4*(X 1^2-X 2^2)+C (a*b+a*c+b*c)*X 4*(X 1-X 2)+FirstFallCore.pairTop := by
  simp [lift,Fin.sum_univ_succ,witness,generator,GuardedModel.membership,
    GuardedModel.guard,FirstFallCore.pairTop,map_add,map_mul,map_neg]
  ring

theorem witness_lift_degree (α β a b c r : K) :
    (lift (generator α β a b c r) witness).totalDegree ≤ 3 := by
  have hp : (FirstFallCore.pairTop : P K).totalDegree ≤ 2 := by
    simpa [GuardedModel.pairDifference,FirstFallCore.pairTop] using
      GuardedModel.pairDifference_degree (0 : K) 0 0
  have h1 := totalDegree_sub (X 1^2 : P K) (X 2^2)
  have h2 := totalDegree_sub (X 1 : P K) (X 2)
  have h3 := totalDegree_mul (-C (a+b+c) : P K) (X 4)
  have h4 := totalDegree_mul (-C (a+b+c)*X 4 : P K) (X 1^2-X 2^2)
  have h5 := totalDegree_mul (C (a*b+a*c+b*c) : P K) (X 4)
  have h6 := totalDegree_mul (C (a*b+a*c+b*c)*X 4 : P K) (X 1-X 2)
  have h7 := totalDegree_add (-C (a+b+c)*X 4*(X 1^2-X 2^2) : P K)
    (C (a*b+a*c+b*c)*X 4*(X 1-X 2))
  have h8 := totalDegree_add
    (-C (a+b+c)*X 4*(X 1^2-X 2^2)+C (a*b+a*c+b*c)*X 4*(X 1-X 2) : P K)
    FirstFallCore.pairTop
  simp only [totalDegree_X,totalDegree_X_pow,totalDegree_C,totalDegree_neg] at h1 h2 h3 h5
  rw [witness_lift_identity]
  omega

end
end WeightedFirstFall

#print axioms WeightedFirstFall.exact_first_four
#print axioms WeightedFirstFall.witness_lift_ne_zero
#print axioms WeightedFirstFall.witness_lift_degree
