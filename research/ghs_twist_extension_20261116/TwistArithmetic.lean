import Init

/-! Checked integer inputs to the quadratic-twist transfer receipt.
The curve identity, finite-field point count, divisor map, and isogeny
argument are checked or justified separately; these are not Lean theorems. -/
namespace GHSTwistArithmetic

theorem base_order : (243 : Nat) + 1 + 26 = 270 := by decide

theorem trace_recurrence_s7 :
    (-26 : Int) * (-26798930) - 243 * 1797094 = 260078338 := by decide

theorem s7_order_from_trace :
    (3 : Nat)^35 + 1 - 260078338 = 50031544838921370 := by decide

theorem s7_prime_factor_arithmetic :
    (270 : Nat) * 185302017921931 = 50031544838921370 := by decide

theorem s7_source_cofactor :
    (50031544838921370 : Nat) / 185302017921931 = 270 := by decide

theorem s7_bare_exponent_screen :
    (185302017921931 : Nat)^5 > 7^5 * ((3 : Nat)^7)^16 := by decide

end GHSTwistArithmetic
