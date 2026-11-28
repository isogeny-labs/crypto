import Mathlib
namespace KernelObstruction
/-- Cubic reciprocal product has independent coefficients
c+a*b, b+a*b*c+a*c, a*(2+b*b+c*c) when a is nonzero in F3. -/
theorem field1039 : ¬ ∃ a b c : ZMod 3,
    a ≠ 0 ∧ c+a*b=0 ∧ b+a*b*c+a*c=1 ∧ a*(2+b*b+c*c)=1 := by decide
theorem field2011 : ¬ ∃ a b c : ZMod 3,
    a ≠ 0 ∧ c+a*b=0 ∧ b+a*b*c+a*c=1 ∧ a*(2+b*b+c*c)=0 := by decide
theorem field4003 : ¬ ∃ a b c : ZMod 3,
    a ≠ 0 ∧ c+a*b=1 ∧ b+a*b*c+a*c=0 ∧ a*(2+b*b+c*c)=0 := by decide
end KernelObstruction
#print axioms KernelObstruction.field1039
#print axioms KernelObstruction.field2011
#print axioms KernelObstruction.field4003
