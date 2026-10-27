import KoblitzRound18
namespace KoblitzRound18Hecke
theorem all_residue_inverse_characters : ∀ u:ZMod 11,u≠0 → u*u^9=1 := by decide +kernel
theorem actual_shared_rational_system : (5*(2:ZMod 11)^(4*9)=1 ∧ 7*(2:ZMod 11)^(7*9)=1 ∧ (2:ZMod 11)^9=6) := by decide +kernel
theorem no_scalar_separator (p:ZMod 11 → ZMod 11 → ZMod 11) : ¬ (p 1 6=1 ∧ p 1 6=0) := by intro h;have : (1:ZMod 11)=0 := h.1.symm.trans h.2;norm_num at this
end KoblitzRound18Hecke
#print axioms KoblitzRound18Hecke.all_residue_inverse_characters
#print axioms KoblitzRound18Hecke.actual_shared_rational_system
#print axioms KoblitzRound18Hecke.no_scalar_separator
