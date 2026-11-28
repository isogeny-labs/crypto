import BinaryFourth

namespace SemaevBinaryFourthControlAudit
noncomputable section
open SemaevBinaryFourthAudit GeneralS4Audit SemaevS3
abbrev F := DiemSmallFieldAudit.F
local instance : DecidableEq F := Classical.decEq _
abbrev alpha : F := DiemSmallFieldAudit.alpha
abbrev curve := binaryCurve (0:F)

theorem alpha_square : alpha^2=alpha+1 := by
  linear_combination DiemSmallFieldAudit.alpha_relation-
    (alpha+1)*DiemSmallFieldAudit.two_zero

theorem alpha_ne_zero : alpha≠0 := by
  intro h
  have hr := DiemSmallFieldAudit.alpha_relation
  simp only [show DiemSmallFieldAudit.alpha=(0:F) from h,zero_pow (by decide : (2:ℕ)≠0),
    zero_add] at hr
  exact one_ne_zero hr

theorem alpha_cube : alpha^3=1 := by
  apply mul_left_cancel₀ alpha_ne_zero
  calc
    alpha*alpha^3=alpha^4 := by ring
    _=alpha := DiemSmallFieldAudit.pow_four alpha
    _=alpha*1 := by ring

theorem tuple_sigma1 : sigma1 (1:F) 1 alpha (alpha^2)=1 := by
  simp only [sigma1,one_add_one_eq_two,CharTwo.two_eq_zero,zero_add,alpha_square]
  rw [←add_assoc,CharTwo.add_self_eq_zero,zero_add]

theorem tuple_sigma3 : sigma3 (1:F) 1 alpha (alpha^2)=1 := by
  simp only [sigma3,one_mul]
  rw [show alpha*alpha^2=alpha^3 by ring,alpha_cube,
    alpha_square]
  linear_combination DiemSmallFieldAudit.two_zero*alpha+
    DiemSmallFieldAudit.two_zero

theorem tuple_sigma4 : sigma4 (1:F) 1 alpha (alpha^2)=1 := by
  simpa only [sigma4,one_mul,←pow_succ'] using alpha_cube

theorem actual_resultant_zero : generalS4 curve 1 1 alpha (alpha^2)=0 := by
  rw [actual_resultant_corrected]
  norm_num [correctedFourth,tuple_sigma1,tuple_sigma3,tuple_sigma4,
    CharTwo.ofNat_eq_mod]
  linear_combination 4*DiemSmallFieldAudit.two_zero

theorem literal_value_one : literalPrintedFourth (1:F) 1 alpha (alpha^2)=1 := by
  norm_num [literalPrintedFourth,tuple_sigma1,tuple_sigma3,tuple_sigma4,
    CharTwo.ofNat_eq_mod]
  linear_combination 3*DiemSmallFieldAudit.two_zero

#print axioms actual_resultant_zero
#print axioms literal_value_one
end
end SemaevBinaryFourthControlAudit
