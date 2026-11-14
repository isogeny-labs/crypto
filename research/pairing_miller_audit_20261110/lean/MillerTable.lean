import MillerExpressions
import TorsionIndex

namespace PairingMillerTableAudit
open PairingMillerCurveAudit PairingMillerExpressionsAudit PairingMillerTorsionAudit

def determinant (u v : Index) : ZMod 3 := u.1*v.2-u.2*v.1

/-- The prescribed expressions are actually computed on every ordered pair
of the nine actual torsion points. No bilinearity/expected table is assumed. -/
theorem weil_table : ∀ u v : Index,
    weilMiller (indexedPoint u) (indexedPoint v)=omega^(determinant u v).val := by
  decide +kernel

theorem tate_table : ∀ u v : Index,
    reducedTateMiller (indexedPoint u) (indexedPoint v)=omega^(2*determinant u v).val := by
  decide +kernel

/-- All denominators and numerators used in the Weil evaluations are nonzero. -/
theorem all_evaluations_nonzero : ∀ u v : Index,
    millerLine (indexedPoint v) (indexedPoint u+auxiliary)≠0 ∧
    millerLine (indexedPoint v) auxiliary≠0 ∧
    millerLine (indexedPoint u) (-auxiliary)≠0 ∧
    millerLine (indexedPoint u) (indexedPoint v-auxiliary)≠0 := by
  decide +kernel

theorem omega_power_add : ∀ a b : ZMod 3,
    omega^(a+b).val=omega^a.val*omega^b.val := by decide +kernel

theorem omega_power_one_iff_zero : ∀ a : ZMod 3,omega^a.val=1 ↔ a=0 := by decide +kernel
theorem omega_power_twice_one_iff_zero : ∀ a : ZMod 3,omega^(2*a).val=1 ↔ a=0 := by
  decide +kernel

theorem determinant_left_add (u v w : Index) :
    determinant (u+v) w=determinant u w+determinant v w := by
  simp only [determinant,Prod.fst_add,Prod.snd_add]
  ring

theorem determinant_right_add (u v w : Index) :
    determinant u (v+w)=determinant u v+determinant u w := by
  simp only [determinant,Prod.fst_add,Prod.snd_add]
  ring

theorem determinant_left_detects (u : Index) (hu : ∀ v,determinant u v=0) : u=0 := by
  have h1:=hu (0,1)
  have h2:=hu (1,0)
  simp only [determinant,mul_zero,mul_one,sub_zero,zero_sub] at h1 h2
  exact Prod.ext h1 (neg_eq_zero.mp h2)

theorem determinant_right_detects (v : Index) (hv : ∀ u,determinant u v=0) : v=0 := by
  have h1:=hv (1,0)
  have h2:=hv (0,1)
  simp only [determinant,zero_mul,one_mul,sub_zero,zero_sub] at h1 h2
  exact Prod.ext (neg_eq_zero.mp h2) h1

#print axioms weil_table
#print axioms tate_table
#print axioms all_evaluations_nonzero
end PairingMillerTableAudit
