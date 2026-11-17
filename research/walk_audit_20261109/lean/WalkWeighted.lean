import WalkBoundary

/-! Exact unequal-weight in-degree moments of the actual Z/3 rho graph.
This is a fixed finite random-function ensemble, not a collision-time
asymptotic or a practical hash-family model. -/
namespace WalkWeightedAudit
noncomputable section
open scoped BigOperators ENNReal
open WalkBoundaryAudit

abbrev Hash := ZMod 3 → Fin 2

def table (a b c : Fin 2) : Hash :=
  fun x=>if x=0 then a else if x=1 then b else c

def tableMap (t : Fin 2 × (Fin 2 × Fin 2)) : Hash := table t.1 t.2.1 t.2.2

theorem table_bijective : Function.Bijective tableMap := by decide +kernel

def tableEquiv : (Fin 2 × (Fin 2 × Fin 2)) ≃ Hash :=
  Equiv.ofBijective tableMap table_bijective

theorem sum_hashes (f : Hash → ℝ) :
    (∑ h : Hash,f h)=∑ a : Fin 2,∑ b : Fin 2,∑ c : Fin 2,f (table a b c) := by
  rw [←Equiv.sum_comp tableEquiv]
  simp [Fintype.sum_prod_type,tableEquiv,tableMap]

theorem univ_three : (Finset.univ : Finset (ZMod 3))={0,1,2} := by decide

/-- Product of independent label probabilities at all actual vertices. -/
def hashWeight (a : ℝ) (h : Hash) : ℝ :=
  ∏ x : ZMod 3,if h x=0 then a else 1-a

def inDegree (h : Hash) : ℝ := (smallInDegree h : ℝ)

theorem product_three (f : ZMod 3 → ℝ) :
    (∏ x : ZMod 3,f x)=f 0*f 1*f 2 := by
  calc
    _=f 0*(f 1*f 2) := by
      rw [univ_three,Finset.prod_insert (by decide),Finset.prod_insert (by decide),
        Finset.prod_singleton]
    _=f 0*f 1*f 2 := by ring

theorem hashWeight_table (a : ℝ) (i j k : Fin 2) :
    hashWeight a (table i j k)=
      (if i=0 then a else 1-a)*(if j=0 then a else 1-a)*(if k=0 then a else 1-a) := by
  rw [hashWeight,product_three]
  have hz : (2 : ZMod 3)≠0 := by decide
  have ho : (2 : ZMod 3)≠1 := by decide
  simp [table,hz,ho]

theorem smallInDegree_table : ∀ i j k : Fin 2,
    smallInDegree (table i j k)=
      (if j=1 then (1:ℚ) else 0)+(if k=0 then (1:ℚ) else 0) := by decide +kernel

theorem inDegree_table (i j k : Fin 2) :
    inDegree (table i j k)=
      (if j=1 then (1:ℝ) else 0)+(if k=0 then (1:ℝ) else 0) := by
  fin_cases i <;> fin_cases j <;> fin_cases k <;>
    norm_num [inDegree,smallInDegree_table]

theorem hashWeight_nonnegative (a : ℝ) (ha : 0≤a) (ha1 : a≤1) (h : Hash) :
    0≤hashWeight a h := by
  apply Finset.prod_nonneg
  intro x _
  split_ifs
  · exact ha
  · linarith

theorem weight_sum (a : ℝ) : (∑ h : Hash,hashWeight a h)=1 := by
  rw [sum_hashes]
  norm_num [Fin.sum_univ_two,hashWeight_table]
  ring

theorem inDegree_mean (a : ℝ) : (∑ h : Hash,hashWeight a h*inDegree h)=1 := by
  rw [sum_hashes]
  norm_num [Fin.sum_univ_two,hashWeight_table,inDegree_table]
  ring

theorem inDegree_variance (a : ℝ) :
    (∑ h : Hash,hashWeight a h*(inDegree h-1)^2)=1-(a^2+(1-a)^2) := by
  rw [sum_hashes]
  norm_num [Fin.sum_univ_two,hashWeight_table,inDegree_table]
  ring

/-- A genuine finite PMF on every actual hash function, with unequal labels. -/
def hashLaw (a : ℝ) (ha : 0≤a) (ha1 : a≤1) : PMF Hash :=
  PMF.ofFintype (fun h=>ENNReal.ofReal (hashWeight a h)) (by
    rw [←ENNReal.ofReal_sum_of_nonneg (fun h _=>hashWeight_nonnegative a ha ha1 h)]
    rw [weight_sum]
    simp)

theorem hashLaw_mass (a : ℝ) (ha : 0≤a) (ha1 : a≤1) (h : Hash) :
    hashLaw a ha ha1 h=ENNReal.ofReal (hashWeight a h) := rfl

theorem hashLaw_real_mass (a : ℝ) (ha : 0≤a) (ha1 : a≤1) (h : Hash) :
    (hashLaw a ha ha1 h).toReal=hashWeight a h := by
  rw [hashLaw_mass,ENNReal.toReal_ofReal (hashWeight_nonnegative a ha ha1 h)]

theorem actual_weighted_moments (a : ℝ) (ha : 0≤a) (ha1 : a≤1) :
    (∑ h : Hash,(hashLaw a ha ha1 h).toReal*inDegree h)=1 ∧
    (∑ h : Hash,(hashLaw a ha ha1 h).toReal*(inDegree h-1)^2)=
      1-(a^2+(1-a)^2) := by
  simp_rw [hashLaw_real_mass]
  exact ⟨inDegree_mean a,inDegree_variance a⟩

/-- Concrete unequal probabilities, retained as an exact known-answer control. -/
def unequalLaw : PMF Hash := hashLaw (3/4) (by norm_num) (by norm_num)

theorem actual_unequal_weight_control :
    (∑ h : Hash,(unequalLaw h).toReal*inDegree h)=1 ∧
    (∑ h : Hash,(unequalLaw h).toReal*(inDegree h-1)^2)=3/8 := by
  have hm := actual_weighted_moments (3/4) (by norm_num) (by norm_num)
  change (∑ h : Hash,(hashLaw (3/4) _ _ h).toReal*inDegree h)=1 ∧
    (∑ h : Hash,(hashLaw (3/4) _ _ h).toReal*(inDegree h-1)^2)=3/8
  refine ⟨hm.1,?_⟩
  rw [hm.2]
  norm_num

theorem weighted_variance_audit_certificate :
    Fintype.card Hash=8 ∧
    Function.Injective (fun i : Fin 2=>((i.val:ZMod 3)+1)) ∧
    (∀ i : Fin 2,((i.val:ZMod 3)+1)≠0) ∧
    (∀ (a : ℝ) (ha : 0≤a) (ha1 : a≤1),
      (∑ h : Hash,(hashLaw a ha ha1 h).toReal*inDegree h)=1 ∧
      (∑ h : Hash,(hashLaw a ha ha1 h).toReal*(inDegree h-1)^2)=
        1-(a^2+(1-a)^2)) ∧
    (∑ h : Hash,(unequalLaw h).toReal*inDegree h)=1 ∧
    (∑ h : Hash,(unequalLaw h).toReal*(inDegree h-1)^2)=3/8 := by
  exact ⟨small_hash_space_cardinality,small_increments_distinct_nonzero.1,
    small_increments_distinct_nonzero.2,actual_weighted_moments,
    actual_unequal_weight_control.1,actual_unequal_weight_control.2⟩

#print axioms weighted_variance_audit_certificate
#print axioms actual_weighted_moments
end
end WalkWeightedAudit
