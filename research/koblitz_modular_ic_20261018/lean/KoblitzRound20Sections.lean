import KoblitzRound20Retraction
namespace KoblitzRound20Sections
def degrees : Fin 5→Nat := ![45,315,63,171,57]
def basis (n:Nat):List (Nat×Nat) := [(0,0)]++(List.range (n/2)).map (fun i=>(i+1,0))++(List.range ((n-3)/2+1)).map (fun i=>(i,1))
theorem actual_basis_dimensions : ∀i:Fin 5,(basis (degrees i)).length=degrees i := by decide +kernel
theorem actual_affine_coordinates : ∀i:Fin 5,(basis (degrees i)).contains (1,0)=true ∧ (basis (degrees i)).contains (0,1)=true := by decide +kernel
theorem actual_pole_bounds : ∀i:Fin 5,(basis (degrees i)).all (fun (a,b)=>2*a+3*b≤degrees i)=true := by decide +kernel
theorem actual_infinity_coordinates : ∀i:Fin 5,(basis (degrees i)).contains ((degrees i-1)/2,0)=true ∧ (basis (degrees i)).contains ((degrees i-3)/2,1)=true := by decide +kernel
theorem actual_consecutive_poles : ∀i:Fin 5,2*((degrees i-1)/2)=degrees i-1 ∧ 2*((degrees i-3)/2)+3=degrees i := by decide +kernel
theorem actual_nonproportional_rows : ∀i j:Fin 20,i≠j→∃a b:Fin 4,KoblitzRound19OrbitCover.cmul (KoblitzRound20Retraction.row i a) (KoblitzRound20Retraction.row j b)≠KoblitzRound19OrbitCover.cmul (KoblitzRound20Retraction.row i b) (KoblitzRound20Retraction.row j a) := by decide +kernel
end KoblitzRound20Sections
#print axioms KoblitzRound20Sections.actual_basis_dimensions
#print axioms KoblitzRound20Sections.actual_affine_coordinates
#print axioms KoblitzRound20Sections.actual_pole_bounds
#print axioms KoblitzRound20Sections.actual_infinity_coordinates
#print axioms KoblitzRound20Sections.actual_consecutive_poles
#print axioms KoblitzRound20Sections.actual_nonproportional_rows
