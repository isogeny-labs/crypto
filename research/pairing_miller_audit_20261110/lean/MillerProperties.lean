import MillerTable

namespace PairingMillerPropertiesAudit
open PairingMillerCurveAudit PairingMillerExpressionsAudit
  PairingMillerTorsionAudit PairingMillerTableAudit

def weilValue (U V : threeTorsion) : F25 := weilMiller U.val V.val
def tateValue (U V : threeTorsion) : F25 := reducedTateMiller U.val V.val

theorem weilValue_indexed (u v : Index) :
    weilValue (torsionHom u) (torsionHom v)=omega^(determinant u v).val := weil_table u v

theorem tateValue_indexed (u v : Index) :
    tateValue (torsionHom u) (torsionHom v)=omega^(2*determinant u v).val := tate_table u v

theorem weil_left_bilinear (U V W : threeTorsion) :
    weilValue (U+V) W=weilValue U W*weilValue V W := by
  obtain ⟨u,rfl⟩:=torsionHom_bijective.surjective U
  obtain ⟨v,rfl⟩:=torsionHom_bijective.surjective V
  obtain ⟨w,rfl⟩:=torsionHom_bijective.surjective W
  rw [←map_add,weilValue_indexed,weilValue_indexed,weilValue_indexed,
    determinant_left_add,omega_power_add]

theorem weil_right_bilinear (U V W : threeTorsion) :
    weilValue U (V+W)=weilValue U V*weilValue U W := by
  obtain ⟨u,rfl⟩:=torsionHom_bijective.surjective U
  obtain ⟨v,rfl⟩:=torsionHom_bijective.surjective V
  obtain ⟨w,rfl⟩:=torsionHom_bijective.surjective W
  rw [←map_add,weilValue_indexed,weilValue_indexed,weilValue_indexed,
    determinant_right_add,omega_power_add]

theorem tate_left_bilinear (U V W : threeTorsion) :
    tateValue (U+V) W=tateValue U W*tateValue V W := by
  obtain ⟨u,rfl⟩:=torsionHom_bijective.surjective U
  obtain ⟨v,rfl⟩:=torsionHom_bijective.surjective V
  obtain ⟨w,rfl⟩:=torsionHom_bijective.surjective W
  rw [←map_add,tateValue_indexed,tateValue_indexed,tateValue_indexed,
    determinant_left_add,mul_add,omega_power_add]

theorem tate_right_bilinear (U V W : threeTorsion) :
    tateValue U (V+W)=tateValue U V*tateValue U W := by
  obtain ⟨u,rfl⟩:=torsionHom_bijective.surjective U
  obtain ⟨v,rfl⟩:=torsionHom_bijective.surjective V
  obtain ⟨w,rfl⟩:=torsionHom_bijective.surjective W
  rw [←map_add,tateValue_indexed,tateValue_indexed,tateValue_indexed,
    determinant_right_add,mul_add,omega_power_add]

theorem weil_left_nondegenerate (U : threeTorsion)
    (hU : ∀ V,weilValue U V=1) : U=0 := by
  obtain ⟨u,rfl⟩:=torsionHom_bijective.surjective U
  have hu : u=0 := determinant_left_detects u (fun v=>by
    apply (omega_power_one_iff_zero _).mp
    simpa only [weilValue_indexed] using hU (torsionHom v))
  rw [hu,map_zero]

theorem weil_right_nondegenerate (V : threeTorsion)
    (hV : ∀ U,weilValue U V=1) : V=0 := by
  obtain ⟨v,rfl⟩:=torsionHom_bijective.surjective V
  have hv : v=0 := determinant_right_detects v (fun u=>by
    apply (omega_power_one_iff_zero _).mp
    simpa only [weilValue_indexed] using hV (torsionHom u))
  rw [hv,map_zero]

theorem tate_left_nondegenerate (U : threeTorsion)
    (hU : ∀ V,tateValue U V=1) : U=0 := by
  obtain ⟨u,rfl⟩:=torsionHom_bijective.surjective U
  have hu : u=0 := determinant_left_detects u (fun v=>by
    apply (omega_power_twice_one_iff_zero _).mp
    simpa only [tateValue_indexed] using hU (torsionHom v))
  rw [hu,map_zero]

theorem tate_right_nondegenerate (V : threeTorsion)
    (hV : ∀ U,tateValue U V=1) : V=0 := by
  obtain ⟨v,rfl⟩:=torsionHom_bijective.surjective V
  have hv : v=0 := determinant_right_detects v (fun u=>by
    apply (omega_power_twice_one_iff_zero _).mp
    simpa only [tateValue_indexed] using hV (torsionHom u))
  rw [hv,map_zero]

def rationalPoint (a : ZMod 3) : threeTorsion := torsionHom (a,0)

theorem rational_restriction_trivial (a b : ZMod 3) :
    weilValue (rationalPoint a) (rationalPoint b)=1 ∧
    tateValue (rationalPoint a) (rationalPoint b)=1 := by
  simp [rationalPoint,weilValue_indexed,tateValue_indexed,determinant]

theorem rationalPoint_one_nonzero : rationalPoint 1≠0 := by
  intro h
  have hh:=congrArg Subtype.val h
  change indexedPoint (1,0)=0 at hh
  have hz: (1,0)=(0:Index) := (indexedPoint_zero_iff _).mp hh
  have h1:=congrArg Prod.fst hz
  exact (by decide : (1:ZMod 3)≠0) h1

/-- A direct failure of the nondegeneracy assertion after restricting the
second input to the same rational cyclic line. Both are actual point values. -/
theorem rational_line_nondegeneracy_false :
    ¬∀ U : threeTorsion,(∃ a : ZMod 3,rationalPoint a=U) →
      (∀ b : ZMod 3,weilValue U (rationalPoint b)=1) → U=0 := by
  intro h
  exact rationalPoint_one_nonzero (h (rationalPoint 1) ⟨1,rfl⟩
    (fun b=>(rational_restriction_trivial 1 b).1))

#print axioms weil_left_nondegenerate
#print axioms tate_left_nondegenerate
#print axioms rational_line_nondegeneracy_false
end PairingMillerPropertiesAudit
