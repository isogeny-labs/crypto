import Recoding
import RecodedLower

noncomputable section
open MvPolynomial
namespace RecodedCanonical
variable {K : Type*} [Field K]
set_option maxHeartbeats 1000000

section Generic
variable {σ : Type*} [Fintype σ]
local notation "P" => MvPolynomial σ K
def affine (c : K) (v : σ → K) : P := C c+∑ i, C (v i)*X i
theorem affine_above (c : K) (v : σ → K) (d : ℕ) (hd : 2 ≤ d) :
    homogeneousComponent d (affine c v) = 0 := by
  have h0 : d ≠ 0 := by omega
  have h1 : d ≠ 1 := by omega
  have hc : homogeneousComponent d (C c : P) = 0 := by
    simpa [h0] using (homogeneousComponent_of_mem (isHomogeneous_C σ c) (m := d))
  have hx (i : σ) : homogeneousComponent d (X i : P) = 0 := by
    simpa [h1] using (homogeneousComponent_of_mem (isHomogeneous_X K i) (m := d))
  simp only [affine, _root_.map_add, _root_.map_sum, homogeneousComponent_C_mul,
    hc, hx, mul_zero, Finset.sum_const_zero, add_zero]

theorem split_degree (H : P) (hH : H.IsHomogeneous 2) (hne : H ≠ 0)
    (c : K) (v : σ → K) : (H+affine c v).totalDegree = 2 := by
  apply DegreeBridge.degree_eq_of_components
  · intro d hd
    have h2 : d ≠ 2 := by omega
    simp only [_root_.map_add, affine_above c v d (by omega), add_zero]
    simpa [h2] using (homogeneousComponent_of_mem hH (m := d))
  · simpa only [_root_.map_add, affine_above c v 2 (by decide), add_zero,
      homogeneousComponent_of_mem hH, ↓reduceIte] using hne

theorem split_top (H : P) (hH : H.IsHomogeneous 2) (hne : H ≠ 0)
    (c : K) (v : σ → K) : DegreeBridge.top (H+affine c v) = H := by
  unfold DegreeBridge.top
  rw [split_degree H hH hne c v]
  simp only [_root_.map_add, affine_above c v 2 (by decide), add_zero,
    homogeneousComponent_of_mem hH, ↓reduceIte]
end Generic

local notation "P" => MvPolynomial (Fin 7) K

def h1 : P := X 2^2
def h2 (r : K) : P := X 0*X 1-C r*X 2^2
def hx (a : K) : P := C a*(X 0^2-X 3*X 4)-X 2*X 0
def hz (b : K) : P := C b*(X 1^2-X 5*X 6)-X 2*X 1
def G1 (r : K) : P := h1+affine (-r) ![-1,-1,0,0,0,0,0]
def G2 (A r s : K) : P := h2 r+affine (-(r^2+A)) ![0,0,-2*s,0,0,0,0]
def Qx (a r s b c w1 w2 : K) : P := hx a+affine (c+s) ![b,0,r,w1,w2,0,0]
def Qz (a r s b c w1 w2 : K) : P := hz a+affine (c+s) ![0,b,r,0,0,w1,w2]
def bit (i : Fin 4) : P := X (i.val+3 : Fin 7)^2-X (i.val+3 : Fin 7)
def generators (A r s a b cx cz dx dz wx1 wx2 wz1 wz2 : K) : Set P :=
  {G1 r,G2 A r s,Qx a r s cx dx wx1 wx2,Qz b r s cz dz wz1 wz2} ∪ Set.range bit

def probe : P →+* K := eval ![1,1,0,0,0,0,0]
@[simp] theorem probe_X (i : Fin 7) : probe (X i) = (![1,1,0,0,0,0,0] i : K) :=
  eval_X _
@[simp] theorem probe_C (c : K) : probe (C c) = c := eval_C _

theorem h1_homogeneous : (h1 : P).IsHomogeneous 2 := isHomogeneous_X_pow _ _
theorem h2_homogeneous (r : K) : (h2 r : P).IsHomogeneous 2 :=
  ((isHomogeneous_X K 0).mul (isHomogeneous_X K 1)).sub
    ((isHomogeneous_X_pow _ _).C_mul r)
theorem hx_homogeneous (a : K) : (hx a : P).IsHomogeneous 2 :=
  (((isHomogeneous_X_pow 0 2).sub ((isHomogeneous_X K 3).mul
    (isHomogeneous_X K 4))).C_mul a).sub ((isHomogeneous_X K 2).mul (isHomogeneous_X K 0))
theorem hz_homogeneous (a : K) : (hz a : P).IsHomogeneous 2 :=
  (((isHomogeneous_X_pow 1 2).sub ((isHomogeneous_X K 5).mul
    (isHomogeneous_X K 6))).C_mul a).sub ((isHomogeneous_X K 2).mul (isHomogeneous_X K 1))

theorem h1_ne_zero : (h1 : P) ≠ 0 := by simp [h1]
theorem h2_ne_zero (r : K) : (h2 r : P) ≠ 0 := by
  intro h
  have hh := congrArg (probe (K := K)) h
  simp [h2, _root_.map_sub, _root_.map_mul, _root_.map_pow] at hh
theorem hx_ne_zero (a : K) (ha : a ≠ 0) : (hx a : P) ≠ 0 := by
  intro h
  have hh := congrArg (probe (K := K)) h
  simp [hx, _root_.map_sub, _root_.map_mul, _root_.map_pow] at hh
  exact ha hh
theorem hz_ne_zero (a : K) (ha : a ≠ 0) : (hz a : P) ≠ 0 := by
  intro h
  have hh := congrArg (probe (K := K)) h
  simp [hz, _root_.map_sub, _root_.map_mul, _root_.map_pow] at hh
  exact ha hh

theorem bit_top (i : Fin 4) : DegreeBridge.top (bit (K := K) i) = X (i.val+3 : Fin 7)^2 := by
  let j : Fin 7 := (i.val+3 : Fin 7)
  change DegreeBridge.top ((X j : P)^2-X j) = X j^2
  have hp : ((X j : P)^2).IsHomogeneous 2 := isHomogeneous_X_pow j 2
  have hx : (X j : P).IsHomogeneous 1 := isHomogeneous_X K j
  have hdeg : ((X j : P)^2-X j).totalDegree = 2 := by
    apply DegreeBridge.degree_eq_of_components
    · intro d hd
      have h2 : d ≠ 2 := by omega
      have h1 : d ≠ 1 := by omega
      simp only [_root_.map_sub, homogeneousComponent_of_mem hp,
        homogeneousComponent_of_mem hx, if_neg h2, if_neg h1, sub_zero]
    · simp only [_root_.map_sub, homogeneousComponent_of_mem hp,
        homogeneousComponent_of_mem hx]
      norm_num
  unfold DegreeBridge.top
  rw [hdeg]
  simp only [_root_.map_sub, homogeneousComponent_of_mem hp,
    homogeneousComponent_of_mem hx]
  norm_num

theorem high_forms (A r s a b cx cz dx dz wx1 wx2 wz1 wz2 : K)
    (ha : a ≠ 0) (hb : b ≠ 0) :
    DegreeBridge.top '' generators A r s a b cx cz dx dz wx1 wx2 wz1 wz2 =
      {h1,h2 r,hx a,hz b} ∪ Set.range (fun i : Fin 4 => (X (i.val+3 : Fin 7) : P)^2) := by
  have hbits : DegreeBridge.top '' Set.range (bit (K := K)) =
      Set.range (fun i : Fin 4 => (X (i.val+3 : Fin 7) : P)^2) := by
    ext p
    constructor
    · rintro ⟨q,⟨i,rfl⟩,rfl⟩
      exact ⟨i,(bit_top i).symm⟩
    · rintro ⟨i,rfl⟩
      exact ⟨bit i,⟨i,rfl⟩,bit_top i⟩
  simp only [generators, Set.image_union, Set.image_insert_eq, Set.image_singleton,
    hbits, G1, G2, Qx, Qz, split_top _ h1_homogeneous h1_ne_zero,
    split_top _ (h2_homogeneous r) (h2_ne_zero r),
    split_top _ (hx_homogeneous a) (hx_ne_zero a ha),
    split_top _ (hz_homogeneous b) (hz_ne_zero b hb)]



theorem normalize_x (a : K) (ha : a ≠ 0) :
    C a⁻¹*hx a = (X 0^2-C a⁻¹*X 2*X 0-X 3*X 4 : P) := by
  unfold hx
  rw [mul_sub, ← mul_assoc, ← C_mul, inv_mul_cancel₀ ha, C_1, one_mul]
  ring

theorem normalize_z (a : K) (ha : a ≠ 0) :
    C a⁻¹*hz a = (X 1^2-C a⁻¹*X 2*X 1-X 5*X 6 : P) := by
  unfold hz
  rw [mul_sub, ← mul_assoc, ← C_mul, inv_mul_cancel₀ ha, C_1, one_mul]
  ring

theorem canonical_ideal (A r s a b cx cz dx dz wx1 wx2 wz1 wz2 : K)
    (ha : a ≠ 0) (hb : b ≠ 0) :
    DegreeBridge.topIdeal (generators A r s a b cx cz dx dz wx1 wx2 wz1 wz2) =
      RecodedTop.ideal a⁻¹ b⁻¹ := by
  unfold DegreeBridge.topIdeal
  rw [high_forms A r s a b cx cz dx dz wx1 wx2 wz1 wz2 ha hb]
  let I : Ideal P := Ideal.span ({h1,h2 r,hx a,hz b} ∪
    Set.range (fun i : Fin 4 => (X (i.val+3 : Fin 7) : P)^2))
  let J := RecodedTop.ideal a⁻¹ b⁻¹
  change I = J
  have hi1 : h1 ∈ I := Ideal.subset_span (Or.inl (Or.inl rfl))
  have hi2 : h2 r ∈ I := Ideal.subset_span (Or.inl (Or.inr (Or.inl rfl)))
  have hi3 : hx a ∈ I := Ideal.subset_span (Or.inl (Or.inr (Or.inr (Or.inl rfl))))
  have hi4 : hz b ∈ I := Ideal.subset_span (Or.inl (Or.inr (Or.inr (Or.inr rfl))))
  have hib (i : Fin 4) : (X (i.val+3 : Fin 7) : P)^2 ∈ I :=
    Ideal.subset_span (Or.inr ⟨i,rfl⟩)
  have hj1 : (X 2 : P)^2 ∈ J := Ideal.subset_span (Or.inl rfl)
  have hj2 : (X 0*X 1 : P) ∈ J := Ideal.subset_span (Or.inr (Or.inl rfl))
  have hj3 : (X 0^2-C a⁻¹*X 2*X 0-X 3*X 4 : P) ∈ J :=
    Ideal.subset_span (Or.inr (Or.inr (Or.inl rfl)))
  have hj4 : (X 1^2-C b⁻¹*X 2*X 1-X 5*X 6 : P) ∈ J :=
    Ideal.subset_span (Or.inr (Or.inr (Or.inr (Or.inl rfl))))
  have hjb (i : Fin 4) : (X (i.val+3 : Fin 7) : P)^2 ∈ J := by
    fin_cases i
    · exact Ideal.subset_span (Or.inr (Or.inr (Or.inr (Or.inr (Or.inl rfl)))))
    · exact Ideal.subset_span (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inl rfl))))))
    · exact Ideal.subset_span (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inl rfl)))))))
    · exact Ideal.subset_span (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr rfl)))))))
  have hax : C a*(X 0^2-C a⁻¹*X 2*X 0-X 3*X 4 : P) = hx a := by
    rw [← normalize_x a ha, ← mul_assoc, ← C_mul, mul_inv_cancel₀ ha, C_1, one_mul]
  have hbz : C b*(X 1^2-C b⁻¹*X 2*X 1-X 5*X 6 : P) = hz b := by
    rw [← normalize_z b hb, ← mul_assoc, ← C_mul, mul_inv_cancel₀ hb, C_1, one_mul]
  apply le_antisymm
  · apply Ideal.span_le.mpr
    intro p hp
    rcases hp with hp|hp
    · rcases hp with rfl|rfl|rfl|rfl
      · exact hj1
      · exact J.sub_mem hj2 (J.mul_mem_left (C r) hj1)
      · rw [← hax]; exact J.mul_mem_left (C a) hj3
      · rw [← hbz]; exact J.mul_mem_left (C b) hj4
    · obtain ⟨i,rfl⟩ := hp
      exact hjb i
  · apply Ideal.span_le.mpr
    intro p hp
    rcases hp with rfl|rfl|rfl|rfl|rfl|rfl|rfl|rfl
    · exact hi1
    · have h := I.add_mem hi2 (I.mul_mem_left (C r) hi1)
      simpa only [h2, h1, sub_add_cancel] using h
    · rw [← normalize_x a ha]; exact I.mul_mem_left (C a⁻¹) hi3
    · rw [← normalize_z b hb]; exact I.mul_mem_left (C b⁻¹) hi4
    · exact hib 0
    · exact hib 1
    · exact hib 2
    · exact hib 3

theorem canonical_vanishes_six (A r s a b cx cz dx dz wx1 wx2 wz1 wz2 : K)
    (ha : a ≠ 0) (hb : b ≠ 0) :
    SecantIdeal.Vanishes
      (DegreeBridge.topIdeal (generators A r s a b cx cz dx dz wx1 wx2 wz1 wz2)) 6 := by
  rw [canonical_ideal A r s a b cx cz dx dz wx1 wx2 wz1 wz2 ha hb]
  exact RecodedTop.vanishes a⁻¹ b⁻¹


theorem canonical_first_vanishing (A r s a b cx cz dx dz wx1 wx2 wz1 wz2 : K)
    (ha : a ≠ 0) (hb : b ≠ 0) :
    SecantIdeal.FirstVanishing
      (DegreeBridge.topIdeal (generators A r s a b cx cz dx dz wx1 wx2 wz1 wz2)) 6 := by
  rw [canonical_ideal A r s a b cx cz dx dz wx1 wx2 wz1 wz2 ha hb]
  exact RecodedTop.first_vanishing a⁻¹ b⁻¹

def coordinates (x z m : K) (u : Fin 4 → K) : Fin 7 → K :=
  ![x,z,m,u 0,u 1,u 2,u 3]

theorem eval_affine (c : K) (v : Fin 7 → K) (w : Fin 7 → K) :
    eval w (affine c v) = c+∑ i, v i*w i := by
  simp only [affine, eval_add, eval_sum, eval_mul, eval_C, eval_X]

theorem eval_g1 (r x z m : K) (u : Fin 4 → K) :
    eval (coordinates x z m u) (G1 r) = m^2-x-z-r := by
  simp [G1,h1,eval_affine,coordinates,Fin.sum_univ_succ]
  ring

theorem eval_g2 (A r s x z m : K) (u : Fin 4 → K) :
    eval (coordinates x z m u) (G2 A r s) = x*z-r^2-r*m^2-A-2*m*s := by
  simp [G2,h2,eval_affine,coordinates,Fin.sum_univ_succ]
  ring

theorem eval_qx (a r s b c w1 w2 x z m : K) (u : Fin 4 → K) :
    eval (coordinates x z m u) (Qx a r s b c w1 w2) =
      a*(x^2-u 0*u 1)-m*x+c+s+b*x+r*m+w1*u 0+w2*u 1 := by
  simp [Qx,hx,eval_affine,coordinates,Fin.sum_univ_succ]
  ring

theorem eval_qz (a r s b c w1 w2 x z m : K) (u : Fin 4 → K) :
    eval (coordinates x z m u) (Qz a r s b c w1 w2) =
      a*(z^2-u 2*u 3)-m*z+c+s+b*z+r*m+w1*u 2+w2*u 3 := by
  simp [Qz,hz,eval_affine,coordinates,Fin.sum_univ_succ]
  ring

theorem eval_bit (x z m : K) (u : Fin 4 → K) (i : Fin 4) :
    eval (coordinates x z m u) (bit i) = u i^2-u i := by
  fin_cases i <;> simp [bit,coordinates]

abbrev F := SecantFixture.F
local instance : Fact (Nat.Prime 17) := ⟨by decide⟩
def fixtureGenerators (r s e f : F) : Set (MvPolynomial (Fin 7) F) :=
  generators 2 r s (4*e) (4*f) (9*e) (9*f) (9*e) (9*f) (2*e) (12*e) (2*f) (12*f)

theorem fixture_zeroes_iff (r s e f x z m : F) (u : Fin 4 → F) :
    (∀ p ∈ fixtureGenerators r s e f, eval (coordinates x z m u) p = 0) ↔
      Recoding.System r s e f x z m u := by
  simp only [fixtureGenerators,generators,Set.mem_union,or_imp,forall_and,
    Set.forall_mem_insert,Set.mem_singleton_iff,forall_eq,Set.forall_mem_range,
    eval_g1,eval_g2,eval_qx,eval_qz,eval_bit]
  rw [← PencilSelector.boolean_iff_equations]
  unfold Recoding.System SecantGroupLaw.System Recoding.graph
  constructor
  · rintro ⟨⟨h1,h2,h3,h4⟩,hu⟩
    refine ⟨hu,h1,h2,?_,?_⟩
    · linear_combination h3
    · linear_combination h4
  · rintro ⟨hu,h1,h2,h3,h4⟩
    refine ⟨⟨h1,h2,?_,?_⟩,hu⟩
    · linear_combination h3
    · linear_combination h4

end RecodedCanonical
