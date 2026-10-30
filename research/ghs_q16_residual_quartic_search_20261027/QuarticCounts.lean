-- Independently checked finite arithmetic underlying the four retained orders.
-- The point counts and smoothness themselves are certified by Sage replay.
namespace ResidualQuarticCounts

def sourceOrders : List Nat := [4006, 4054, 4078, 4126]
def primeOrders : List Nat := [2003, 2027, 2039, 2063]
def c3 : List Int := [-91, -43, -19, 29]

theorem source_order_identity :
    sourceOrders = primeOrders.map (fun r => 2*r) := by decide

theorem weil_order_identity :
    c3.map (fun c => (4097 : Int) + c) = [4006, 4054, 4078, 4126] := by decide

theorem point_count_identity :
    ([(3824 : Int), 3968, 4040, 4184].map
      (fun n => (n-4097)/3)) = c3 := by decide

theorem all_cofactor_two :
    sourceOrders.zip primeOrders =
      [(4006,2003),(4054,2027),(4078,2039),(4126,2063)] := by decide

end ResidualQuarticCounts
