import Init

/- Finite arithmetic for the proposed cyclic-11 F5 screen. The genus,
   point counts, and elliptic trace criterion require external mathematics. -/
namespace GHSCyclic11F5

def fifthTrace (s : Int) : Int := s^5 - 25*s^3 + 125*s

theorem five_has_order_five_mod_eleven :
    (5 : Nat)^5 % 11 = 1 ∧
    (5 : Nat)^1 % 11 ≠ 1 ∧
    (5 : Nat)^2 % 11 ≠ 1 ∧
    (5 : Nat)^3 % 11 ≠ 1 ∧
    (5 : Nat)^4 % 11 ≠ 1 := by decide

theorem three_branch_genus_arithmetic :
    (2 : Int)*5-2 = 11*(-2)+3*(11-1) := by decide

theorem fifth_extension_trace_values :
    ([-4,-3,-2,-1,0,1,2,3,4] : List Int).map fifthTrace =
      [76,57,-82,-101,0,101,82,-57,-76] := by decide

#print axioms five_has_order_five_mod_eleven
#print axioms three_branch_genus_arithmetic
#print axioms fifth_extension_trace_values

end GHSCyclic11F5
