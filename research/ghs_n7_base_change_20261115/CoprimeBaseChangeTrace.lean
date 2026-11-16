import Init

/-!
Finite arithmetic for the ±9 base-change trace recurrence. The initial trace
and q2 witness are inputs from the independent census, not proved here. This
file does not formalize finite-field tensor products, Weil restriction,
Jacobian base change, Tate isogeny, or W1 membership.
-/

namespace GHSCoprimeBaseChangeTrace

def tracePair (a : Int) : Nat → Int × Int
  | 0 => (2, a)
  | n + 1 =>
    let (u, v) := tracePair a n
    (v, a * v - 128 * u)

def traceAt (a : Int) (n : Nat) : Int := (tracePair a n).1

theorem minus_trace_samples :
    traceAt (-9) 0 = 2 ∧
    traceAt (-9) 1 = -9 ∧
    traceAt (-9) 2 = -175 ∧
    traceAt (-9) 3 = 2727 ∧
    traceAt (-9) 4 = -2143 := by
  decide

theorem plus_trace_samples :
    traceAt 9 0 = 2 ∧
    traceAt 9 1 = 9 ∧
    traceAt 9 2 = -175 ∧
    traceAt 9 3 = -2727 ∧
    traceAt 9 4 = -2143 := by
  decide

theorem q4_seventh_count :
    (4 ^ 7 + 1 : Int) - 7 * (-175) = 17610 := by
  decide

#print axioms minus_trace_samples
#print axioms plus_trace_samples
#print axioms q4_seventh_count

end GHSCoprimeBaseChangeTrace
