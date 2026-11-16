import TangentCovariance

namespace ThreeTangentPairingAudit
noncomputable section
open WeierstrassCurve.Affine ThreeTorsionRankAudit.General Module
private instance : Fact (Nat.Prime 3) := ⟨by decide⟩
variable {K : Type*} [Field K]

theorem cube_pow_mul (z : K) (hz : z^3=1) (i j : ZMod 3) :
    (z^i.val)^j.val=z^(i*j).val := by
  rw [←pow_mul,ZMod.val_mul]
  exact pow_eq_pow_mod _ hz

theorem cube_pow_neg (z : K) (hz : z^3=1) (i : ZMod 3) :
    z^(-i).val=(z^i.val)⁻¹ := by
  have hn : z≠0 := by intro h; simp [h] at hz
  apply mul_right_cancel₀ (pow_ne_zero i.val hn)
  rw [inv_mul_cancel₀ (pow_ne_zero i.val hn),←cube_pow_add z hz,neg_add_cancel]
  simp only [ZMod.val_zero,pow_zero]

variable [DecidableEq K] [IsAlgClosed K] [NeZero (3:K)]

theorem tangentPair_smul_left (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P Q : ThreeTorsion W) (i : ZMod 3) :
    tangentPair W (i • P).val Q.val=(tangentPair W P.val Q.val)^i.val :=
  tangentPair_nsmul_left W hΔ P Q i.val

theorem tangentPair_smul_right (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P Q : ThreeTorsion W) (i : ZMod 3) :
    tangentPair W P.val (i • Q).val=(tangentPair W P.val Q.val)^i.val :=
  tangentPair_nsmul_right W hΔ P Q i.val

theorem tangentPair_coordinates (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P Q : ThreeTorsion W) (i j k l : ZMod 3) :
    tangentPair W (i • P+j • Q).val (k • P+l • Q).val=
      (tangentPair W P.val Q.val)^(i*l-j*k).val := by
  let z := tangentPair W P.val Q.val
  have hz : z^3=1 := tangentPair_cube W hΔ P Q
  calc tangentPair W (i • P+j • Q).val (k • P+l • Q).val=
      (z^l.val)^i.val*((z⁻¹)^k.val)^j.val := by
        rw [tangentPair_add_left W hΔ,
          tangentPair_add_right W hΔ,tangentPair_add_right W hΔ]
        simp only [tangentPair_smul_left W hΔ,tangentPair_smul_right W hΔ,
          tangentPair_self,one_pow,one_mul,mul_one]
        rw [tangentPair_swap W Q.val P.val]
    _=z^(l*i).val*(z^(k*j).val)⁻¹ := by
      rw [cube_pow_mul z hz,inv_pow,inv_pow,cube_pow_mul z hz]
    _=z^(i*l-j*k).val := by
      rw [←cube_pow_neg z hz,←cube_pow_add z hz]
      rw [show l*i+ -(k*j)=i*l-j*k by ring]

theorem basis_pairing_nontrivial (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (b : Basis (Fin 2) (ZMod 3) (ThreeTorsion W)) :
    tangentPair W (b 0).val (b 1).val≠1 := by
  intro h
  apply b.ne_zero 0
  apply tangentPair_left_nondegenerate W hΔ (b 0)
  intro Q
  have hr : (b.repr Q 0) • b 0+(b.repr Q 1) • b 1=Q := by
    simpa only [Fin.sum_univ_two] using b.sum_repr Q
  rw [←hr,tangentPair_add_right W hΔ,tangentPair_smul_right W hΔ,
    tangentPair_smul_right W hΔ,tangentPair_self,h,one_pow,one_pow,one_mul]

#print axioms tangentPair_coordinates
#print axioms basis_pairing_nontrivial
end
end ThreeTangentPairingAudit
