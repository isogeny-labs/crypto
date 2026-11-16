import SourceField
import S3Completion

namespace JVSourceFieldAudit
noncomputable section

def a : K := 44+52*alpha+60*alpha^2
def b : K := 58+87*alpha+74*alpha^2
def targetX : K := 85+84*alpha+37*alpha^2
def targetY : K := 15+3*alpha+86*alpha^2
def curve : WeierstrassCurve.Affine K := ⟨0,0,0,a,b⟩

local macro "alpha_ring" : tactic => `(tactic| (
  ring_nf
  all_goals simp only [alpha_cube,alpha_four,alpha_five,alpha_six]
  all_goals ring_nf
  all_goals reduce_mod_char!
  all_goals ring_nf
  all_goals reduce_mod_char!))

theorem target_equation : targetY^2=targetX^3+a*targetX+b := by
  have : CharP K 101 := inferInstance
  unfold targetY targetX a b
  alpha_ring

theorem short_discriminant_nonzero : 4*a^3+27*b^2≠0 := by
  have : CharP K 101 := inferInstance
  have hd : 4*a^3+27*b^2=representation 23 95 23 := by
    unfold a b representation
    norm_num only [map_ofNat]
    alpha_ring
  rw [hd]
  intro hz
  exact (by decide : (23:F)≠0) ((representation_zero_iff 23 95 23).mp hz).1

theorem curve_discriminant : curve.Δ= -16*(4*a^3+27*b^2) := by
  simp only [curve,WeierstrassCurve.Δ,WeierstrassCurve.b₂,WeierstrassCurve.b₄,
    WeierstrassCurve.b₆,WeierstrassCurve.b₈]
  ring

theorem characteristic_two_nonzero : (2:K)≠0 :=
  (CharP.cast_eq_zero_iff K 101 2).not.mpr (by decide)

theorem curve_nonsingular : curve.Δ≠0 := by
  rw [curve_discriminant]
  apply mul_ne_zero _ short_discriminant_nonzero
  exact neg_ne_zero.mpr ((CharP.cast_eq_zero_iff K 101 16).not.mpr (by decide))

def target : curve.Point := .some targetX targetY (by
  apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
    curve_nonsingular).mp
  rw [WeierstrassCurve.Affine.equation_iff]
  simpa [curve] using target_equation)

theorem target_nonzero : target≠0 := by
  intro h
  cases h

theorem target_x_coordinate : SemaevS3.pointX target=some targetX := rfl

theorem target_x_not_base (u : F) : algebraMap F K u≠targetX := by
  intro h
  have hr : targetX=representation 85 84 37 := by
    simp [targetX,representation,map_ofNat]
  have hu : algebraMap F K u=representation u 0 0 := by simp [representation]
  rw [hu,hr] at h
  have hc := congrArg (coordinate 1) h
  simp [coordinate_representation] at hc
  revert hc
  decide +kernel

end
end JVSourceFieldAudit
