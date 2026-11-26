module
public section
namespace TorsionColumn

-- Instance obligations for the RFC7748 subgroup order and the fixed toy.
-- No native group/cardinality/map correctness theorem is asserted.
theorem actual_inverse8_bezout :
    8 * ((3 * (2^252 + 27742317777372353535851937790883648493) + 1) / 8) =
      3 * (2^252 + 27742317777372353535851937790883648493) + 1 := by decide

theorem toy_inverse8_bezout : 8 * 5 = 3 * 13 + 1 := by decide

#print axioms actual_inverse8_bezout
#print axioms toy_inverse8_bezout
end TorsionColumn
