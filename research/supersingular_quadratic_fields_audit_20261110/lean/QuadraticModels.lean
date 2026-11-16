import AllPrimeFieldModels
import Mathlib.Algebra.QuadraticAlgebra.Basic

namespace QuadraticFieldModelsAudit
open SmallCharacteristicCensusAudit

private instance primeTwo : Fact (Nat.Prime 2) := ⟨by decide⟩
private instance primeThree : Fact (Nat.Prime 3) := ⟨by decide⟩
private instance binaryIrred : Fact (∀ r : ZMod 2,r^2≠1+1*r) := ⟨by decide +kernel⟩
private instance ternaryIrred : Fact (∀ r : ZMod 3,r^2≠ -1+0*r) := ⟨by decide +kernel⟩

abbrev F4 := QuadraticAlgebra (ZMod 2) 1 1
abbrev F9 := QuadraticAlgebra (ZMod 3) (-1) 0

instance : Fintype F4 := Fintype.ofEquiv (ZMod 2×ZMod 2) (QuadraticAlgebra.equivProd 1 1).symm
instance : Fintype F9 := Fintype.ofEquiv (ZMod 3×ZMod 3) (QuadraticAlgebra.equivProd (-1) 0).symm
instance : CharP F4 2 := charP_of_injective_algebraMap' (ZMod 2) 2
instance : CharP F9 3 := charP_of_injective_algebraMap' (ZMod 3) 3

theorem field_cardinalities : Fintype.card F4=4 ∧ Fintype.card F9=9 := by decide +kernel

theorem coefficient_population_sizes :
    Fintype.card (F4×F4×F4×F4×F4)=1024 ∧
    Fintype.card (F9×F9×F9×F9×F9)=59049 := by
  simp only [Fintype.card_prod,field_cardinalities.1,field_cardinalities.2]
  norm_num

#print axioms field_cardinalities
end QuadraticFieldModelsAudit
