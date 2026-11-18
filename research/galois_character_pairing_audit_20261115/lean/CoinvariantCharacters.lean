module

public import Mathlib.GroupTheory.FiniteAbelian.Duality
public import Mathlib.GroupTheory.QuotientGroup.Basic

@[expose] public section
namespace FiniteTateDualityAudit
noncomputable section
variable {G : Type*} [CommGroup G] (σ : G ≃* G)
variable (K : Type*) [CommMonoid K]

/-- Multiplicative notation for sigma minus identity on a finite abelian group. -/
def difference : G →* G where
  toFun x := σ x/x
  map_one' := by simp
  map_mul' x y := by simp only [map_mul,mul_div_mul_comm]

abbrev Coinvariants := G ⧸ (difference σ).range

def invariantCharacters : Subgroup (G →* Kˣ) where
  carrier := {χ | ∀ x,χ (σ x)=χ x}
  one_mem' := by intro x; rfl
  mul_mem' := by
    intro χ ψ hχ hψ x
    change χ (σ x)*ψ (σ x)=χ x*ψ x
    rw [hχ x,hψ x]
  inv_mem' := by
    intro χ hχ x
    change (χ (σ x))⁻¹=(χ x)⁻¹
    rw [hχ x]

theorem invariant_kills_range (χ : invariantCharacters σ K) :
    (difference σ).range≤χ.val.ker := by
  rintro y ⟨x,rfl⟩
  change χ.val (σ x/x)=1
  rw [map_div,χ.property x,div_self']

def descend (χ : invariantCharacters σ K) : Coinvariants σ →* Kˣ :=
  QuotientGroup.lift _ χ.val (invariant_kills_range σ K χ)

theorem quotient_sigma (x : G) :
    (QuotientGroup.mk (σ x) : Coinvariants σ)=QuotientGroup.mk x := by
  apply QuotientGroup.eq_iff_div_mem.mpr
  exact ⟨x,rfl⟩

def pullback (χ : Coinvariants σ →* Kˣ) : invariantCharacters σ K :=
  ⟨χ.comp (QuotientGroup.mk' _),by
    intro x
    change χ (QuotientGroup.mk (σ x))=χ (QuotientGroup.mk x)
    rw [quotient_sigma σ x]⟩

/-- Invariant characters are exactly all characters of the actual coinvariant
quotient. No root or perfectness premise is needed for descent itself. -/
def characterDescent : invariantCharacters σ K ≃* (Coinvariants σ →* Kˣ) where
  toFun := descend σ K
  invFun := pullback σ K
  left_inv χ := by
    apply Subtype.ext
    ext x
    rfl
  right_inv χ := by
    ext q
    rfl
  map_mul' χ ψ := by
    ext q
    rfl

def evaluate (χ : invariantCharacters σ K) (q : Coinvariants σ) : Kˣ :=
  characterDescent σ K χ q

theorem evaluation_representative (χ : invariantCharacters σ K) (x : G) :
    evaluate σ K χ (QuotientGroup.mk x)=χ.val x := rfl

theorem evaluation_left (χ ψ : invariantCharacters σ K) (q : Coinvariants σ) :
    evaluate σ K (χ*ψ) q=evaluate σ K χ q*evaluate σ K ψ q := by
  exact congrArg (fun f : Coinvariants σ →* Kˣ => f q) ((characterDescent σ K).map_mul χ ψ)

theorem evaluation_right (χ : invariantCharacters σ K) (q r : Coinvariants σ) :
    evaluate σ K χ (q*r)=evaluate σ K χ q*evaluate σ K χ r :=
  map_mul _ q r

#print axioms characterDescent
#print axioms evaluation_representative
#print axioms evaluation_left
#print axioms evaluation_right
end
end FiniteTateDualityAudit
