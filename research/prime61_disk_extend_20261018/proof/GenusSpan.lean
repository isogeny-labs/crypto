import Std

/- Finite arithmetic and XOR witnesses only. This file does not define the
imaginary quadratic class group, prove genus theory, verify the old PARI class
order computation, or certify a full class number. See the proof map. -/
namespace Prime61Genus
set_option maxRecDepth 100000
set_option maxHeartbeats 100000000

def discriminant : Int := -6649310858224389603
def rowMasks : List Nat := [5, 18, 9, 18, 30, 23, 9, 24]
def evenParity (n : Nat) : Bool :=
  ((List.range 5).filter (fun i => n.testBit i)).length % 2 == 0
def xorSubset (s : Nat) : Nat :=
  (List.range 8).foldl
    (fun acc i => if s.testBit i then Nat.xor acc rowMasks[i]! else acc) 0

theorem factor_product :
    (-3 : Int) * 97 * 113 * (-35831) * (-5643470711) = discriminant := by decide

def forms : List (Int × Int × Int) :=
  [(3,3,554109238185365801), (7,5,237475387793728201),
   (11,7,151120701323281583), (13,5,127871362658161339),
   (19,9,87490932345057759), (23,17,72275118024178151),
   (53,27,31364673859549011), (61,23,27251274009116353)]
theorem form_discriminants :
    forms.all (fun f => f.2.1 * f.2.1 - 4 * f.1 * f.2.2 == discriminant) = true := by decide
theorem ramified_represented_norm_coprime :
    Nat.gcd 554109238185365801 6649310858224389603 = 1 := by decide

theorem all_selected_masks_have_even_parity :
    rowMasks.all evenParity = true := by decide
theorem complete_even_pattern_cover :
    ∀ n : Fin 32, evenParity n.val = true →
      ∃ s : Fin 256, xorSubset s.val = n.val := by decide
theorem no_odd_pattern_in_span :
    ∀ s : Fin 256, evenParity (xorSubset s.val) = true := by decide
theorem even_pattern_count :
    ((List.range 32).filter evenParity).length = 16 := by decide
theorem combined_divisor_arithmetic : Nat.lcm 16 68162026 = 545296208 := by decide

theorem subgroup_size_lower_bound {h : Nat} (positive : 0 < h)
    (genus_divides : 16 ∣ h) (old_order_divides : 68162026 ∣ h) :
    545296208 ≤ h := by
  have both := Nat.lcm_dvd genus_divides old_order_divides
  rw [combined_divisor_arithmetic] at both
  exact Nat.le_of_dvd positive both

theorem conditional_size_equality {h : Nat} (positive : 0 < h)
    (genus_divides : 16 ∣ h) (old_order_divides : 68162026 ∣ h)
    (upper : h ≤ 545296208) : h = 545296208 :=
  Nat.le_antisymm upper (subgroup_size_lower_bound positive genus_divides old_order_divides)

/-- The coefficient identities used by the companion polynomial certificate. -/
theorem discriminant_polynomial_identity (x0 x1 x2 x3 x4 x5 x6 x7 : Int) :
    (3*3 - 4*3*554109238185365801 - discriminant)*x0 +
    (5*5 - 4*7*237475387793728201 - discriminant)*x1 +
    (7*7 - 4*11*151120701323281583 - discriminant)*x2 +
    (5*5 - 4*13*127871362658161339 - discriminant)*x3 +
    (9*9 - 4*19*87490932345057759 - discriminant)*x4 +
    (17*17 - 4*23*72275118024178151 - discriminant)*x5 +
    (27*27 - 4*53*31364673859549011 - discriminant)*x6 +
    (23*23 - 4*61*27251274009116353 - discriminant)*x7 = 0 := by
  change (0 : Int)*x0 + 0*x1 + 0*x2 + 0*x3 + 0*x4 + 0*x5 + 0*x6 + 0*x7 = 0
  simp

#print axioms factor_product
#print axioms form_discriminants
#print axioms ramified_represented_norm_coprime
#print axioms all_selected_masks_have_even_parity
#print axioms complete_even_pattern_cover
#print axioms no_odd_pattern_in_span
#print axioms even_pattern_count
#print axioms combined_divisor_arithmetic
#print axioms subgroup_size_lower_bound
#print axioms conditional_size_equality
#print axioms discriminant_polynomial_identity
end Prime61Genus
