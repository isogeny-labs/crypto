import Mathlib.RingTheory.AdjoinRoot
import Mathlib.FieldTheory.Finiteness
import Mathlib.Algebra.Polynomial.SpecificDegree
import Mathlib.Tactic

namespace JVSourceFieldAudit
open Polynomial
noncomputable section
private instance prime101 : Fact (Nat.Prime 101) := ⟨by decide⟩
abbrev F := ZMod 101
def modulus : Polynomial F := Polynomial.X^3+Polynomial.X+1

theorem modulus_monic : modulus.Monic := by
  unfold modulus
  monicity
  norm_num

theorem modulus_degree : modulus.natDegree=3 := by
  unfold modulus
  compute_degree
  norm_num

theorem modulus_irreducible : Irreducible modulus := by
  apply Polynomial.irreducible_of_degree_le_three_of_not_isRoot
  · rw [modulus_degree]
    decide
  · intro x hx
    have h : x^3+x+1=(0:F) := by simpa [Polynomial.IsRoot,modulus] using hx
    exact (by decide +kernel : ∀ x:F, x^3+x+1≠0) x h

instance : Fact (Irreducible modulus) := ⟨modulus_irreducible⟩
abbrev K := AdjoinRoot modulus
instance : CharP K 101 := charP_of_injective_algebraMap' F 101
noncomputable instance : DecidableEq K := Classical.decEq _
def alpha : K := AdjoinRoot.root modulus

theorem field_finrank : Module.finrank F K=3 := by
  rw [(AdjoinRoot.powerBasis modulus_monic.ne_zero).finrank]
  exact modulus_degree

theorem field_cardinality : Nat.card K=1030301 := by
  have : Module.Finite F K := (AdjoinRoot.powerBasis modulus_monic.ne_zero).finite
  rw [Module.natCard_eq_pow_finrank (K:=F),field_finrank]
  norm_num [F,Nat.card_zmod]

theorem alpha_cube : alpha^3= -alpha-1 := by
  have h := AdjoinRoot.mk_self (f:=modulus)
  change AdjoinRoot.mk modulus (Polynomial.X^3+Polynomial.X+1)=0 at h
  rw [map_add,map_add,map_pow,map_one,AdjoinRoot.mk_X] at h
  change alpha^3+alpha+1=0 at h
  linear_combination h

theorem alpha_four : alpha^4= -alpha^2-alpha := by
  calc
    alpha^4=alpha*alpha^3 := by ring
    _ = -alpha^2-alpha := by rw [alpha_cube]; ring

theorem alpha_five : alpha^5= -alpha^2+alpha+1 := by
  calc
    alpha^5=alpha*alpha^4 := by ring
    _ = -alpha^3-alpha^2 := by rw [alpha_four]; ring
    _ = -alpha^2+alpha+1 := by simp only [alpha_cube]; ring

theorem alpha_six : alpha^6=alpha^2+2*alpha+1 := by
  calc
    alpha^6=(alpha^3)^2 := by ring
    _ = alpha^2+2*alpha+1 := by rw [alpha_cube]; ring

def representation (c0 c1 c2 : F) : K :=
  algebraMap F K c0+algebraMap F K c1*alpha+algebraMap F K c2*alpha^2

def representationPolynomial (c0 c1 c2 : F) : Polynomial F :=
  Polynomial.C c0+Polynomial.C c1*Polynomial.X+Polynomial.C c2*Polynomial.X^2

theorem representation_as_mk (c0 c1 c2 : F) :
    representation c0 c1 c2=AdjoinRoot.mk modulus (representationPolynomial c0 c1 c2) := by
  simp [representation,representationPolynomial,alpha,AdjoinRoot.mk_C]

def coordinateLinear (i : ℕ) : K →ₗ[F] F :=
  (Polynomial.lcoeff F i).comp (AdjoinRoot.modByMonicHom modulus_monic)
def coordinate (i : ℕ) (z : K) : F := coordinateLinear i z

theorem coordinate_representation (c0 c1 c2 : F) (i : ℕ) :
    coordinate i (representation c0 c1 c2)=
      (if i=0 then c0 else 0)+(if i=1 then c1 else 0)+(if i=2 then c2 else 0) := by
  rw [representation_as_mk]
  unfold coordinate coordinateLinear
  simp only [LinearMap.comp_apply,Polynomial.lcoeff_apply,AdjoinRoot.modByMonicHom_mk]
  have hd : (representationPolynomial c0 c1 c2).degree<modulus.degree := by
    rw [Polynomial.degree_eq_natDegree modulus_monic.ne_zero,modulus_degree]
    unfold representationPolynomial
    compute_degree
    norm_num
  rw [(Polynomial.modByMonic_eq_self_iff modulus_monic).mpr hd]
  simp [representationPolynomial,Polynomial.coeff_C_mul,Polynomial.coeff_X_pow,
    Polynomial.coeff_C,Polynomial.coeff_X,eq_comm]

theorem coordinate_add (i : ℕ) (u v : K) :
    coordinate i (u+v)=coordinate i u+coordinate i v := (coordinateLinear i).map_add u v
theorem coordinate_zero (i : ℕ) : coordinate i (0:K)=0 := (coordinateLinear i).map_zero

theorem coordinate_nat_mul (i n : ℕ) (z : K) :
    coordinate i ((n:K)*z)=(n:F)*coordinate i z := by
  simpa [coordinate,Algebra.smul_def] using (coordinateLinear i).map_smul (n:F) z

theorem coordinate_mul_nat (i n : ℕ) (z : K) :
    coordinate i (z*(n:K))=(n:F)*coordinate i z := by
  rw [mul_comm]
  exact coordinate_nat_mul i n z

theorem coordinate_one (i : ℕ) : coordinate i (1:K)=if i=0 then 1 else 0 := by
  simpa [representation] using coordinate_representation (1:F) 0 0 i

theorem coordinate_natCast (i n : ℕ) :
    coordinate i (n:K)=if i=0 then (n:F) else 0 := by
  simpa [coordinate_one] using coordinate_nat_mul i n (1:K)

theorem coordinate_alpha (i : ℕ) : coordinate i alpha=if i=1 then 1 else 0 := by
  simpa [representation] using coordinate_representation (0:F) 1 0 i
theorem coordinate_alpha_sq (i : ℕ) : coordinate i (alpha^2)=if i=2 then 1 else 0 := by
  simpa [representation] using coordinate_representation (0:F) 0 1 i

theorem representation_zero_iff (c0 c1 c2 : F) :
    representation c0 c1 c2=0 ↔ c0=0 ∧ c1=0 ∧ c2=0 := by
  constructor
  · intro h
    have h0 := congrArg (coordinate 0) h
    have h1 := congrArg (coordinate 1) h
    have h2 := congrArg (coordinate 2) h
    simp [coordinate_representation,coordinate_zero] at h0 h1 h2
    exact ⟨h0,h1,h2⟩
  · rintro ⟨rfl,rfl,rfl⟩
    simp [representation]

end
end JVSourceFieldAudit
