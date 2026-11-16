module

public import WeightedKernel
public import Mathlib.Algebra.GroupWithZero.Units.Fintype
public import Mathlib.Data.ZMod.Basic
public import Mathlib.Data.Nat.Prime.Infinite

@[expose] public section

namespace HildebrandCellAudit

noncomputable def uniformLaw (T : Type*) [Fintype T] [Nonempty T] : PMF T :=
  PMF.ofFintype (fun _ => (Fintype.card T : ENNReal)⁻¹) (by
    simp only [Finset.sum_const,Finset.card_univ,nsmul_eq_mul]
    exact ENNReal.mul_inv_cancel (by exact_mod_cast (Fintype.card_pos : 0 < Fintype.card T).ne')
      (by simp))

theorem uniform_real (T : Type*) [Fintype T] [Nonempty T] (t : T) :
    (uniformLaw T t).toReal = 1/(Fintype.card T : ℝ) := by
  simp [uniformLaw,one_div]

theorem uniform_map_equiv {A B : Type*} [Fintype A] [Nonempty A] [Fintype B] [Nonempty B]
    (e : A ≃ B) : (uniformLaw A).map e = uniformLaw B := by
  classical
  ext b
  simp only [PMF.map_apply,tsum_fintype,uniformLaw,PMF.ofFintype_apply]
  have he (a : A) : b=e a ↔ a=e.symm b := by
    constructor
    · intro h; simpa using congrArg e.symm h.symm
    · intro h; simp [h]
  simp_rw [he]
  simp [Fintype.card_congr e]

def Choice (p : ℕ) := {a : ZMod p // a ≠ 0}

def ConditionalSupports (p : ℕ) := {f : Fin 2 → ZMod p // f 0=0 ∧ Function.Injective f}

variable {p : ℕ} [Fact p.Prime]

instance : Fintype (Choice p) := inferInstanceAs (Fintype {a : ZMod p // a ≠ 0})

instance : Nonempty (Choice p) := ⟨⟨1,one_ne_zero⟩⟩

noncomputable instance : Fintype (ConditionalSupports p) :=
  Fintype.ofFinite {f : Fin 2 → ZMod p // f 0=0 ∧ Function.Injective f}

theorem choice_card : Fintype.card (Choice p) = p-1 := by
  classical
  have h := Fintype.card_congr (unitsEquivNeZero : (ZMod p)ˣ ≃ Choice p)
  rw [Fintype.card_units,ZMod.card] at h
  exact h.symm

def pairSupport (u : Choice p) (i : Fin 2) : ZMod p := if i=0 then 0 else u.val

theorem support_injective (u : Choice p) : Function.Injective (pairSupport u) := by
  intro i j h
  fin_cases i <;> fin_cases j
  · rfl
  · have he : (0 : ZMod p)=u.val := by simpa [pairSupport] using h
    exact False.elim (u.property he.symm)
  · have he : u.val=(0 : ZMod p) := by simpa [pairSupport] using h
    exact False.elim (u.property he)
  · rfl

noncomputable def supportEquiv : Choice p ≃ ConditionalSupports p where
  toFun u := ⟨pairSupport u,by simp [pairSupport],support_injective u⟩
  invFun f := ⟨f.val 1,by
    intro hz
    have he := f.property.2 (show f.val 1=f.val 0 by rw [hz,f.property.1])
    norm_num at he⟩
  left_inv u := by apply Subtype.ext; simp [pairSupport]
  right_inv f := by
    apply Subtype.ext
    funext i
    fin_cases i
    · simpa [pairSupport] using f.property.1.symm
    · simp [pairSupport]

noncomputable instance : Nonempty (ConditionalSupports p) :=
  ⟨supportEquiv (⟨1,one_ne_zero⟩ : Choice p)⟩

theorem actual_uniform_support_law :
    (uniformLaw (Choice p)).map supportEquiv = uniformLaw (ConditionalSupports p) :=
  uniform_map_equiv supportEquiv

theorem actual_uniform_choice_mass (u : Choice p) :
    (uniformLaw (Choice p) u).toReal = 1/((p-1 : ℕ) : ℝ) := by
  rw [uniform_real,choice_card]

def minusOneChoice : Choice p := ⟨-1,neg_ne_zero.mpr one_ne_zero⟩

theorem actual_source_argument (u : Choice p) :
    pairSupport u 0=0 ∧ Function.Injective (pairSupport u) ∧
      (pairSupport u 0-pairSupport u 1)*(1 : ZMod p) = -u.val := by
  refine ⟨by simp [pairSupport],support_injective u,?_⟩
  simp [pairSupport]

end HildebrandCellAudit
