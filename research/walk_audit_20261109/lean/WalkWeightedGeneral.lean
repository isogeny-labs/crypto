import WalkWeighted

/-! Exact ideal random-function in-degree moments for finite step supports.
The graph uses actual addition, not an assumed random-map transition table.
No collision-time or mixing-time asymptotic is asserted. -/
namespace WalkWeightedGeneralAudit
noncomputable section
open scoped BigOperators ENNReal
variable {G I : Type*} [Fintype G]

abbrev Hash (G I : Type*) := G → I

def weight (p : I → ℝ) (h : Hash G I) : ℝ := ∏ x : G,p (h x)
theorem weight_nonnegative (p : I → ℝ) (hp : ∀ i,0≤p i) (h : Hash G I) :
    0≤weight p h := Finset.prod_nonneg (fun x _=>hp (h x))

section Coordinates
variable [DecidableEq G] [DecidableEq I]

def delta (i j : I) : ℝ := if i=j then 1 else 0

def coordinateGate (y : G) (i : I) (x : G) (j : I) : ℝ :=
  if x=y then delta j i else 1

theorem coordinate_product (y : G) (i : I) (h : Hash G I) :
    (∏ x : G,coordinateGate y i x (h x))=delta (h y) i := by
  exact Fintype.prod_ite_eq' y (fun x=>delta (h x) i)

theorem delta_idempotent (i j : I) : delta i j*delta i j=delta i j := by
  unfold delta
  split_ifs <;> norm_num

end Coordinates

variable [Fintype I] [DecidableEq G]

def expectation (p : I → ℝ) (F : Hash G I → ℝ) : ℝ :=
  ∑ h : Hash G I,weight p h*F h

theorem weight_sum (p : I → ℝ) (hp : ∑ i,p i=1) :
    (∑ h : Hash G I,weight p h)=1 := by
  unfold weight
  rw [←Fintype.prod_sum]
  simp only [hp,Finset.prod_const_one]

def hashLaw (p : I → ℝ) (hp0 : ∀ i,0≤p i) (hp : ∑ i,p i=1) : PMF (Hash G I) :=
  PMF.ofFintype (fun h=>ENNReal.ofReal (weight p h)) (by
    rw [←ENNReal.ofReal_sum_of_nonneg (fun h _=>weight_nonnegative p hp0 h)]
    rw [weight_sum p hp]
    simp)

theorem hashLaw_real_mass (p : I → ℝ) (hp0 : ∀ i,0≤p i) (hp : ∑ i,p i=1)
    (h : Hash G I) : (hashLaw p hp0 hp h).toReal=weight p h := by
  exact ENNReal.toReal_ofReal (weight_nonnegative p hp0 h)

theorem expectation_product (p : I → ℝ) (F : G → I → ℝ) :
    expectation p (fun h=>∏ x : G,F x (h x))=∏ x : G,∑ i : I,p i*F x i := by
  unfold expectation weight
  simp only [←Finset.prod_mul_distrib]
  exact (Fintype.prod_sum (fun (x : G) (i : I)=>p i*F x i)).symm

theorem expectation_sum {J : Type*} [Fintype J] (p : I → ℝ)
    (F : J → Hash G I → ℝ) :
    expectation p (fun h=>∑ i : J,F i h)=∑ i : J,expectation p (F i) := by
  unfold expectation
  simp only [Finset.mul_sum]
  exact Finset.sum_comm

variable [DecidableEq I]

theorem expectation_coordinate (p : I → ℝ) (hp : ∑ i,p i=1) (y : G) (i : I) :
    expectation p (fun h=>delta (h y) i)=p i := by
  calc
    _=expectation p (fun h=>∏ x : G,coordinateGate y i x (h x)) := by
      simp only [coordinate_product]
    _=∏ x : G,∑ j : I,p j*coordinateGate y i x j := expectation_product p (coordinateGate y i)
    _=∏ x : G,if x=y then p i else 1 := by
      congr 1
      ext x
      by_cases hx : x=y <;> simp [coordinateGate,hx,delta,mul_ite,hp]
    _=p i := Fintype.prod_ite_eq' y (fun _=>p i)

theorem expectation_two_coordinates (p : I → ℝ) (hp : ∑ i,p i=1)
    (y z : G) (hyz : y≠z) (i j : I) :
    expectation p (fun h=>delta (h y) i*delta (h z) j)=p i*p j := by
  calc
    _=expectation p (fun h=>∏ x : G,
        coordinateGate y i x (h x)*coordinateGate z j x (h x)) := by
      simp only [Finset.prod_mul_distrib,coordinate_product]
    _=∏ x : G,∑ k : I,p k*(coordinateGate y i x k*coordinateGate z j x k) :=
      expectation_product p (fun (x : G) (k : I)=>
        coordinateGate y i x k*coordinateGate z j x k)
    _=∏ x : G,(if x=y then p i else 1)*(if x=z then p j else 1) := by
      congr 1
      ext x
      by_cases hxy : x=y
      · subst x
        simp [coordinateGate,hyz,delta,mul_ite]
      · by_cases hxz : x=z
        · subst x
          simp [coordinateGate,hxy,delta,mul_ite]
        · simp [coordinateGate,hxy,hxz,hp]
    _=p i*p j := by simp only [Finset.prod_mul_distrib,Fintype.prod_ite_eq']

section Graph
variable [AddCommGroup G]

def step (increments : I → G) (h : Hash G I) (x : G) : G := x+increments (h x)

def inDegree (increments : I → G) (h : Hash G I) (target : G) : ℝ :=
  ((Finset.univ.filter (fun x : G=>step increments h x=target)).card : ℝ)

theorem inDegree_eq_coordinates (increments : I → G) (h : Hash G I) (target : G) :
    inDegree increments h target=∑ i : I,delta (h (target-increments i)) i := by
  classical
  have hs : (∑ x : G,if x+increments (h x)=target then (1:ℝ) else 0)=
      ∑ i : I,delta (h (target-increments i)) i := by
    calc
      _=∑ x : G,∑ i : I,if h x=i then
          (if x+increments i=target then (1:ℝ) else 0) else 0 := by simp
      _=∑ i : I,∑ x : G,if x=target-increments i then delta (h x) i else 0 := by
        rw [Finset.sum_comm]
        congr 1
        ext i
        congr 1
        ext x
        simp only [eq_sub_iff_add_eq]
        by_cases hi : h x=i <;> by_cases hx : x+increments i=target <;>
          simp [delta,hi,hx]
      _=∑ i : I,delta (h (target-increments i)) i := by simp
  unfold inDegree step
  rw [Finset.card_filter]
  calc
    _=∑ x : G,if x+increments (h x)=target then (1:ℝ) else 0 := by
      rw [Nat.cast_sum]
      apply Finset.sum_congr rfl
      intro x _
      split_ifs <;> norm_num
    _=_ := hs

theorem inDegree_mean (p : I → ℝ) (hp : ∑ i,p i=1) (increments : I → G)
    (target : G) : expectation p (fun h=>inDegree increments h target)=1 := by
  simp_rw [inDegree_eq_coordinates]
  rw [expectation_sum]
  simp only [expectation_coordinate p hp,hp]

theorem coordinate_pair_moment (p : I → ℝ) (hp : ∑ i,p i=1)
    (increments : I → G) (hi : Function.Injective increments) (target : G) (i j : I) :
    expectation p (fun h=>delta (h (target-increments i)) i*
      delta (h (target-increments j)) j)=if i=j then p i else p i*p j := by
  by_cases hij : i=j
  · subst j
    simp only [delta_idempotent,ite_true]
    exact expectation_coordinate p hp _ i
  · rw [ite_eq_right hij]
    apply expectation_two_coordinates p hp
    intro he
    exact hij (hi (sub_right_injective he))

theorem diagonal_sum (p : I → ℝ) (hp : ∑ i,p i=1) :
    (∑ i : I,∑ j : I,if i=j then p i else p i*p j)=2-∑ i : I,(p i)^2 := by
  have he (i j : I) : (if i=j then p i else p i*p j)=
      p i*p j+(if i=j then p i-(p i)^2 else 0) := by
    by_cases hij : i=j
    · subst j
      simp only [ite_true]
      ring
    · simp only [ite_eq_right hij,add_zero]
  simp_rw [he]
  simp only [Finset.sum_add_distrib,Finset.sum_ite_eq,Finset.mem_univ,ite_true,
    ←Finset.mul_sum,hp,mul_one,Finset.sum_sub_distrib]
  ring

theorem inDegree_second_moment (p : I → ℝ) (hp : ∑ i,p i=1)
    (increments : I → G) (hi : Function.Injective increments) (target : G) :
    expectation p (fun h=>(inDegree increments h target)^2)=2-∑ i : I,(p i)^2 := by
  have he (h : Hash G I) : (inDegree increments h target)^2=
      ∑ i : I,∑ j : I,delta (h (target-increments i)) i*
        delta (h (target-increments j)) j := by
    rw [inDegree_eq_coordinates,pow_two,Finset.sum_mul]
    simp only [Finset.mul_sum]
  calc
    _=expectation p (fun h=>∑ i : I,∑ j : I,
        delta (h (target-increments i)) i*delta (h (target-increments j)) j) := by
      simp only [he]
    _=∑ i : I,∑ j : I,if i=j then p i else p i*p j := by
      simp only [expectation_sum,coordinate_pair_moment p hp increments hi target]
    _=2-∑ i : I,(p i)^2 := diagonal_sum p hp

theorem inDegree_variance (p : I → ℝ) (hp : ∑ i,p i=1)
    (increments : I → G) (hi : Function.Injective increments) (target : G) :
    expectation p (fun h=>(inDegree increments h target-1)^2)=1-∑ i : I,(p i)^2 := by
  calc
    _=expectation p (fun h=>(inDegree increments h target)^2)-
        2*expectation p (fun h=>inDegree increments h target)+
        (∑ h : Hash G I,weight p h) := by
      unfold expectation
      rw [Finset.mul_sum,←Finset.sum_sub_distrib,←Finset.sum_add_distrib]
      apply Finset.sum_congr rfl
      intro h _
      ring
    _=1-∑ i : I,(p i)^2 := by
      rw [inDegree_second_moment p hp increments hi target,
        inDegree_mean p hp increments target,weight_sum p hp]
      ring

theorem actual_graph_moments (p : I → ℝ) (hp0 : ∀ i,0≤p i) (hp : ∑ i,p i=1)
    (increments : I → G) (hi : Function.Injective increments) (target : G) :
    (∑ h : Hash G I,(hashLaw p hp0 hp h).toReal*inDegree increments h target)=1 ∧
    (∑ h : Hash G I,(hashLaw p hp0 hp h).toReal*
      (inDegree increments h target-1)^2)=1-∑ i : I,(p i)^2 := by
  simp_rw [hashLaw_real_mass]
  exact ⟨inDegree_mean p hp increments target,inDegree_variance p hp increments hi target⟩

end Graph

theorem inDegree_constant {A B : Type*} [Fintype A] [DecidableEq A] [AddCommGroup A]
    (c : A) (h : Hash A B) (target : A) :
    inDegree (fun _ : B=>c) h target=1 := by
  unfold inDegree step
  have hs : (Finset.univ.filter (fun x : A=>x+c=target))={target-c} := by
    ext x
    simp only [Finset.mem_filter,Finset.mem_univ,true_and,Finset.mem_singleton]
    exact eq_sub_iff_add_eq.symm
  rw [hs]
  simp

def uniformTwoLabelLaw : PMF (Hash (ZMod 3) (Fin 2)) :=
  hashLaw (fun _ : Fin 2=>(1/2:ℝ)) (by intro i; norm_num)
    (by norm_num [Fin.sum_univ_two])

theorem constant_increment_variance_zero :
    (∑ h : Hash (ZMod 3) (Fin 2),(uniformTwoLabelLaw h).toReal*
      (inDegree (fun _ : Fin 2=>(1:ZMod 3)) h 0-1)^2)=0 := by
  simp [inDegree_constant]

theorem distinctness_cannot_be_dropped :
    ¬ ∀ increments : Fin 2 → ZMod 3, (∀ i,increments i≠0) →
      (∑ h : Hash (ZMod 3) (Fin 2),(uniformTwoLabelLaw h).toReal*
        (inDegree increments h 0-1)^2)=1-∑ _i : Fin 2,(1/2:ℝ)^2 := by
  intro h
  have hh := h (fun _=>1) (by intro i; decide)
  rw [constant_increment_variance_zero] at hh
  norm_num [Fin.sum_univ_two] at hh

/-- Exact finite random-function moments, with the distinctness hypothesis
fully explicit and a direct negative control when it is omitted. -/
theorem general_variance_audit_certificate :
    (∀ (A : Type*) (B : Type*) [AddCommGroup A] [Fintype A] [Fintype B]
      [DecidableEq A] [DecidableEq B] (p : B → ℝ)
      (hp0 : ∀ i,0≤p i) (hp : ∑ i,p i=1) (increments : B → A),
      Function.Injective increments → ∀ target : A,
      (∑ h : Hash A B,(hashLaw p hp0 hp h).toReal*inDegree increments h target)=1 ∧
      (∑ h : Hash A B,(hashLaw p hp0 hp h).toReal*
        (inDegree increments h target-1)^2)=1-∑ i : B,(p i)^2) ∧
    (¬ ∀ increments : Fin 2 → ZMod 3, (∀ i,increments i≠0) →
      (∑ h : Hash (ZMod 3) (Fin 2),(uniformTwoLabelLaw h).toReal*
        (inDegree increments h 0-1)^2)=1-∑ _i : Fin 2,(1/2:ℝ)^2) := by
  constructor
  · intro A B _ _ _ _ _ p hp0 hp increments hi target
    exact actual_graph_moments p hp0 hp increments hi target
  · exact distinctness_cannot_be_dropped

#print axioms distinctness_cannot_be_dropped
#print axioms general_variance_audit_certificate
#print axioms actual_graph_moments
#print axioms inDegree_mean
end
end WalkWeightedGeneralAudit
