import PencilTop
import DegreeBridge

noncomputable section
open MvPolynomial
open scoped BigOperators
namespace PencilCanonical
variable {K : Type*} [Field K]
variable (n : ℕ)
local notation "P" => PencilTop.P K n
local notation "x" => (X (Sum.inl 0) : P)
local notation "z" => (X (Sum.inl 1) : P)
local notation "m" => (X (Sum.inl 2) : P)

def g1 (r : K) : P := m^2-x-z-C r
def g2 (r s A : K) : P := x*z-C (r^2)-C r*m^2-C A-C (2*s)*m
def lower (i : Fin 3) (b c : K) (v : Fin n → K) : P :=
  C b*X (Sum.inl i)+C c+∑ j, C (v j)*X (Sum.inr j)
def q (i : Fin 3) (a r s b c : K) (v : Fin n → K) : P :=
  C a*X (Sum.inl i)^2+lower n i b c v-m*(X (Sum.inl i)-C r)+C s
def bitEq (i : Fin n) : P := X (Sum.inr i)^2-X (Sum.inr i)
def generators (r s A a b cx cz dx dz : K) (vx vz : Fin n → K) : Set P :=
  {g1 n r, g2 n r s A, q n 0 a r s cx dx vx, q n 1 b r s cz dz vz} ∪
    Set.range (bitEq n)

def coordinates (xx zz sl : K) (u : Fin n → K) : PencilTop.Vars n → K :=
  Sum.elim ![xx,zz,sl] u

theorem eval_g1 (r xx zz sl : K) (u : Fin n → K) :
    eval (coordinates n xx zz sl u) (g1 n r) = sl^2-xx-zz-r := by
  simp [g1, coordinates]

theorem eval_g2 (r s A xx zz sl : K) (u : Fin n → K) :
    eval (coordinates n xx zz sl u) (g2 n r s A) =
      xx*zz-r^2-r*sl^2-A-2*sl*s := by
  simp [g2, coordinates]
  ring

theorem eval_q (i : Fin 3) (a r s b c xx zz sl : K) (v u : Fin n → K) :
    eval (coordinates n xx zz sl u) (q n i a r s b c v) =
      a*(![xx,zz,sl] i)^2+(b*(![xx,zz,sl] i)+c+∑ j, v j*u j)-
        sl*(![xx,zz,sl] i-r)+s := by
  simp [q, lower, coordinates]

theorem eval_bit (i : Fin n) (xx zz sl : K) (u : Fin n → K) :
    eval (coordinates n xx zz sl u) (bitEq n i) = u i^2-u i := by
  simp [bitEq, coordinates]

theorem zeroes_iff (r s A a b cx cz dx dz xx zz sl : K) (vx vz u : Fin n → K) :
    (∀ p ∈ generators n r s A a b cx cz dx dz vx vz,
      eval (coordinates n xx zz sl u) p = 0) ↔
      (∀ i, u i^2-u i = 0) ∧ sl^2-xx-zz-r = 0 ∧
        xx*zz-r^2-r*sl^2-A-2*sl*s = 0 ∧
        a*xx^2+(cx*xx+dx+∑ i, vx i*u i)-sl*(xx-r)+s = 0 ∧
        b*zz^2+(cz*zz+dz+∑ i, vz i*u i)-sl*(zz-r)+s = 0 := by
  simp only [generators, Set.mem_union, or_imp, forall_and, Set.forall_mem_insert,
    Set.mem_singleton_iff, forall_eq, Set.forall_mem_range,
    eval_g1, eval_g2, eval_q, eval_bit, Matrix.cons_val_zero, Matrix.cons_val_one]
  exact and_comm

lemma hc_C (d : ℕ) (a : K) : homogeneousComponent d (C a : P) =
    if d = 0 then C a else 0 :=
  homogeneousComponent_of_mem (isHomogeneous_C (PencilTop.Vars n) a)
lemma hc_X (d : ℕ) (i : PencilTop.Vars n) : homogeneousComponent d (X i : P) =
    if d = 1 then X i else 0 :=
  homogeneousComponent_of_mem (isHomogeneous_X K i)
lemma hc_pow (d e : ℕ) (i : PencilTop.Vars n) : homogeneousComponent d ((X i : P)^e) =
    if d = e then X i^e else 0 :=
  homogeneousComponent_of_mem (isHomogeneous_X_pow i e)
lemma hc_product (d : ℕ) (i j : PencilTop.Vars n) :
    homogeneousComponent d ((X i : P)*X j) = if d = 2 then X i*X j else 0 :=
  homogeneousComponent_of_mem ((isHomogeneous_X K i).mul (isHomogeneous_X K j))

theorem lower_above (d : ℕ) (hd : 2 ≤ d) (i : Fin 3) (b c : K) (v : Fin n → K) :
    homogeneousComponent d (lower n i b c v) = 0 := by
  have h0 : d ≠ 0 := by omega
  have h1 : d ≠ 1 := by omega
  simp [lower, _root_.map_add, _root_.map_sum, homogeneousComponent_C_mul,
    hc_X, hc_C, h0, h1]

theorem g1_component (r : K) : homogeneousComponent 2 (g1 n r) = m^2 := by
  simp [g1, _root_.map_sub, hc_pow, hc_X, hc_C]
theorem g2_component (r s A : K) :
    homogeneousComponent 2 (g2 n r s A) = x*z-C r*m^2 := by
  simp only [g2, (homogeneousComponent 2 : P →ₗ[K] P).map_sub,
    homogeneousComponent_C_mul, hc_product, hc_pow, hc_X, hc_C]
  norm_num
theorem q_component (i : Fin 3) (a r s b c : K) (v : Fin n → K) :
    homogeneousComponent 2 (q n i a r s b c v) =
      C a*X (Sum.inl i)^2-m*X (Sum.inl i) := by
  rw [q, mul_sub]
  simp [_root_.map_add, _root_.map_sub, homogeneousComponent_C_mul,
    lower_above n 2 (by decide), hc_pow, hc_product, hc_X, hc_C,
    mul_comm m (C r)]
theorem bit_component (i : Fin n) : homogeneousComponent 2 (bitEq n i) =
    (X (Sum.inr i) : P)^2 := by
  simp [bitEq, _root_.map_sub, hc_pow, hc_X]

def probe : P →+* K := eval (Sum.elim
  (fun i : Fin 3 => if i = 2 then 0 else 1) (fun _ : Fin n => 0))
@[simp] lemma probe_core (i : Fin 3) : probe (K := K) n (X (Sum.inl i)) =
    (if i = 2 then 0 else 1 : K) := by simp [probe]
@[simp] lemma probe_C (a : K) : probe n (C a) = a := by simp [probe]

theorem g1_degree (r : K) : (g1 n r).totalDegree = 2 := by
  apply DegreeBridge.degree_eq_of_components
  · intro d hd
    have h0 : d ≠ 0 := by omega
    have h1 : d ≠ 1 := by omega
    have h2 : d ≠ 2 := by omega
    simp [g1, _root_.map_sub, hc_pow, hc_X, hc_C, h0, h1, h2]
  · rw [g1_component]
    simp [X_pow_eq_monomial]

theorem g2_degree (r s A : K) : (g2 n r s A).totalDegree = 2 := by
  apply DegreeBridge.degree_eq_of_components
  · intro d hd
    have h0 : d ≠ 0 := by omega
    have h1 : d ≠ 1 := by omega
    have h2 : d ≠ 2 := by omega
    simp only [g2, (homogeneousComponent d : P →ₗ[K] P).map_sub,
      homogeneousComponent_C_mul, hc_product, hc_pow, hc_X, hc_C]
    simp [h0, h1, h2]
  · rw [g2_component]
    intro hz
    have hh := congrArg (probe n) hz
    simp [_root_.map_sub, _root_.map_mul, _root_.map_pow] at hh

theorem q_degree (i : Fin 3) (hi : i ≠ 2) (a r s b c : K)
    (ha : a ≠ 0) (v : Fin n → K) : (q n i a r s b c v).totalDegree = 2 := by
  apply DegreeBridge.degree_eq_of_components
  · intro d hd
    have h0 : d ≠ 0 := by omega
    have h1 : d ≠ 1 := by omega
    have h2 : d ≠ 2 := by omega
    rw [q, mul_sub]
    simp [_root_.map_add, _root_.map_sub, homogeneousComponent_C_mul,
      lower_above n d (by omega), hc_pow, hc_product, hc_X, hc_C,
      h0, h1, h2, mul_comm m (C r)]
  · rw [q_component]
    intro hz
    have hh := congrArg (probe n) hz
    have hh' : a = 0 := by
      simpa [_root_.map_sub, _root_.map_mul, _root_.map_pow, hi] using hh
    exact ha hh'

theorem bit_degree (i : Fin n) : (bitEq n i : P).totalDegree = 2 := by
  apply DegreeBridge.degree_eq_of_components
  · intro d hd
    have h1 : d ≠ 1 := by omega
    have h2 : d ≠ 2 := by omega
    simp [bitEq, _root_.map_sub, hc_pow, hc_X, h1, h2]
  · rw [bit_component]
    simp [X_pow_eq_monomial]

lemma normalize (a : K) (ha : a ≠ 0) (u v : P) :
    C a⁻¹*(C a*u^2-v*u) = u^2-C a⁻¹*v*u := by
  calc
    _ = (C a⁻¹*C a)*u^2-C a⁻¹*v*u := by ring
    _ = _ := by rw [← _root_.map_mul, inv_mul_cancel₀ ha, _root_.map_one, one_mul]

lemma denormalize (a : K) (ha : a ≠ 0) (u v : P) :
    C a*(u^2-C a⁻¹*v*u) = C a*u^2-v*u := by
  calc
    _ = C a*u^2-(C a*C a⁻¹)*v*u := by ring
    _ = _ := by rw [← _root_.map_mul, mul_inv_cancel₀ ha, _root_.map_one, one_mul]

theorem canonical_top_ideal (r s A a b cx cz dx dz : K)
    (ha : a ≠ 0) (hb : b ≠ 0) (vx vz : Fin n → K) :
    DegreeBridge.topIdeal (generators n r s A a b cx cz dx dz vx vz) =
      PencilTop.ideal n a⁻¹ b⁻¹ := by
  simp only [generators, DegreeBridge.topIdeal, Set.image_union,
    Set.image_insert_eq, Set.image_singleton, ← Set.range_comp',
    DegreeBridge.top, g1_degree, g2_degree, q_degree n 0 (by decide) a r s cx dx ha vx,
    q_degree n 1 (by decide) b r s cz dz hb vz, bit_degree,
    g1_component, g2_component, q_component, bit_component]
  let I : Ideal P := Ideal.span ({m^2, x*z-C r*m^2,
    C a*x^2-m*x, C b*z^2-m*z} ∪ Set.range (fun i : Fin n => (X (Sum.inr i) : P)^2))
  let J := PencilTop.ideal n a⁻¹ b⁻¹
  change I = J
  have hi1 : m^2 ∈ I := Ideal.subset_span (by simp)
  have hi2 : x*z-C r*m^2 ∈ I := Ideal.subset_span (by simp)
  have hi3 : C a*x^2-m*x ∈ I := Ideal.subset_span (by simp)
  have hi4 : C b*z^2-m*z ∈ I := Ideal.subset_span (by simp)
  have hx : x^2-C a⁻¹*m*x ∈ I := by
    have hh := I.mul_mem_left (C a⁻¹) hi3
    rw [normalize n a ha] at hh
    exact hh
  have hz : z^2-C b⁻¹*m*z ∈ I := by
    have hh := I.mul_mem_left (C b⁻¹) hi4
    rw [normalize n b hb] at hh
    exact hh
  have hxz : x*z ∈ I := by
    simpa using I.add_mem hi2 (I.mul_mem_left (C r) hi1)
  have hj1 : m^2 ∈ J := Ideal.subset_span (by simp [J, PencilTop.ideal])
  have hj2 : x*z ∈ J := Ideal.subset_span (by simp [J, PencilTop.ideal])
  have hj3 : x^2-C a⁻¹*m*x ∈ J := Ideal.subset_span (by simp [J, PencilTop.ideal])
  have hj4 : z^2-C b⁻¹*m*z ∈ J := Ideal.subset_span (by simp [J, PencilTop.ideal])
  apply le_antisymm
  · apply Ideal.span_le.mpr
    intro p hp
    rcases hp with hp | hp
    · simp only [Set.mem_insert_iff, Set.mem_singleton_iff] at hp
      rcases hp with rfl | rfl | rfl | rfl
      · exact hj1
      · exact J.sub_mem hj2 (J.mul_mem_left (C r) hj1)
      · rw [← denormalize n a ha]
        exact J.mul_mem_left (C a) hj3
      · rw [← denormalize n b hb]
        exact J.mul_mem_left (C b) hj4
    · obtain ⟨i, rfl⟩ := hp
      exact Ideal.subset_span (by simp [J, PencilTop.ideal])
  · apply Ideal.span_le.mpr
    intro p hp
    rcases hp with hp | hp
    · simp only [Set.mem_insert_iff, Set.mem_singleton_iff] at hp
      rcases hp with rfl | rfl | rfl | rfl
      · exact hi1
      · exact hxz
      · exact hx
      · exact hz
    · obtain ⟨i, rfl⟩ := hp
      exact Ideal.subset_span (by simp [I])

theorem canonical_first_vanishing (r s A a b cx cz dx dz : K)
    (ha : a ≠ 0) (hb : b ≠ 0) (h2 : (2 : K) ≠ 0) (vx vz : Fin n → K) :
    SecantIdeal.FirstVanishing
      (DegreeBridge.topIdeal (generators n r s A a b cx cz dx dz vx vz)) (n+3) := by
  rw [canonical_top_ideal n r s A a b cx cz dx dz ha hb vx vz]
  exact PencilTop.first_vanishing n a⁻¹ b⁻¹ h2

end PencilCanonical

#print axioms PencilCanonical.canonical_top_ideal
#print axioms PencilCanonical.canonical_first_vanishing
#print axioms PencilCanonical.zeroes_iff
