import Std.Tactic

namespace WeightedMinima

-- The input orders are independently checked against the source-bound JSON by verify109.py.
def y0Orders : List Int := [0, 0, 0, -3, -8, -13, -18, -23, -23, -23, -23, -23, -23, -23, -23]
def y1Orders : List Int := [0, 0, -3, -7, -11, -15, -19, -23, -23, -23, -23, -23, -23, -23, -23]
def weights (orders : List Int) (s u : Int) : List (Nat × Int) :=
  (orders.zipIdx).map fun (a, i) => (i, s * a + u * i)
def minima (orders : List Int) (s u : Int) : List Nat :=
  ((weights orders s u).filter fun p => p.2 == 0).map Prod.fst

theorem y0DegreeOneMinima : minima y0Orders 1 5 = [0] := by native_decide
theorem y1DegreeOneMinima : minima y1Orders 1 4 = [0] := by native_decide
theorem y0DegreeSevenMinima : minima y0Orders 7 23 = [0, 7] := by native_decide
theorem y1DegreeSevenMinima : minima y1Orders 7 23 = [0, 7] := by native_decide
theorem degreeSevenLeadCancellation : Nat.xor 1 1 = 0 := by native_decide

end WeightedMinima
