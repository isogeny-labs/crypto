import Std
namespace PairedTrace
/-- Integer polynomial skeleton. Finite-field and Kummer bindings are not included. -/
theorem paired_trace_int (a b c d : Int) :
    (a-b)*(c-d)+(a+b)*(c+d)=2*(a*c+b*d) := by
  simp only [Int.sub_mul, Int.mul_sub, Int.add_mul, Int.mul_add]
  omega
#print axioms paired_trace_int
end PairedTrace
