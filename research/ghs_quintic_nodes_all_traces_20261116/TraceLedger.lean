-- Finite arithmetic audit for the nineteen newly modeled sparse trace classes.
-- Geometry, point counts, Waterhouse's theorem, and Tate's theorem are external.
namespace GHSQuinticTraceLedger

def traces : List Int :=
  [-29, -28, -27, -25, -23, -19, -17, -13, -11, -7,
   -5, -1, 1, 7, 13, 17, 19, 23, 28]

def orders : List Int :=
  [273, 272, 271, 269, 267, 263, 261, 257, 255, 251,
   249, 245, 243, 237, 231, 227, 225, 221, 216]

theorem nineteen_traces : traces.length = 19 := by decide
theorem nineteen_orders : orders.length = 19 := by decide

theorem hasse_integer_bounds :
    traces.all (fun t => decide (-31 ≤ t ∧ t ≤ 31)) = true := by decide

theorem elliptic_trace_arithmetic_filter :
    traces.all (fun t => decide (t % 3 ≠ 0 ∨ t = 0 ∨ t = -27 ∨ t = 27)) = true := by decide

theorem jacobian_orders :
    (traces.zip orders).all (fun p => decide (p.2 = 244 - p.1)) = true := by decide

theorem fifth_point_counts :
    (traces.zip orders).all (fun p => decide (244 - 5 * p.1 = 5 * p.2 - 976)) = true := by decide

end GHSQuinticTraceLedger
