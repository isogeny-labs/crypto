import KoblitzRound19Polarization
import KoblitzRound18Refinement
namespace KoblitzRound19OrbitCover
open scoped BigOperators
abbrev CM := ℤ×ℤ
def cmul (x y:CM):CM := (x.1*y.1-2*x.2*y.2,x.1*y.2+x.2*y.1-x.2*y.2)
def bar (x:CM):CM := (x.1-x.2,-x.2)
def scale (n:ℤ) (x:CM):CM := (n*x.1,n*x.2)
def rows : Fin 4→Fin 5→Fin 4→CM := ![![![(7,-7),(21,0),(0,-7),(14,-7)],![(14,7),(-7,0),(7,0),(-7,7)],![(-21,-7),(-7,-7),(-21,0),(-14,-7)],![(14,0),(0,7),(7,0),(21,7)],![(-14,7),(-7,0),(7,7),(-14,0)]],![![(6,2),(0,0),(7,0),(2,1)],![(-6,-2),(1,-2),(-4,-1),(-6,-2)],![(7,0),(2,1),(0,0),(6,2)],![(-5,1),(-7,0),(-1,2),(-7,0)],![(-2,-1),(4,1),(-2,-1),(5,-1)]],![![(15,0),(0,0),(0,0),(10,4)],![(-15,0),(-15,0),(-5,4),(-15,0)],![(0,0),(10,4),(0,0),(15,0)],![(10,4),(0,0),(15,0),(0,0)],![(-10,-4),(5,-4),(-10,-4),(-10,-4)]],![![(0,0),(0,0),(0,0),(7,0)],![(0,0),(0,0),(7,0),(0,0)],![(0,0),(7,0),(0,0),(0,0)],![(7,0),(0,0),(0,0),(0,0)],![(-7,0),(-7,0),(-7,0),(-7,0)]]]
def H : Fin 4→Fin 4→CM := ![![(5,0),(2,1),(2,-1),(4,1)],![(1,-1),(3,0),(0,-1),(2,-1)],![(3,1),(1,1),(3,0),(2,1)],![(3,-1),(3,1),(1,-1),(5,0)]]
def weights : Fin 4→ℤ := ![45,315,63,171]
def gram (i j:Fin 4):CM := ∑ k:Fin 4,∑ s:Fin 5,scale (weights k) (cmul (bar (rows k s i)) (rows k s j))
def rotateRow (r:Fin 4→CM):Fin 4→CM := ![-r 0+r 1,-r 0+r 2,-r 0+r 3,-r 0]
def ix (i:Fin 4) (b:Fin 2):Fin 8 := ⟨2*i.val+b.val,by omega⟩
theorem actual_weighted_Gram : ∀i j:Fin 4,gram i j=scale 33075 (H i j) := by decide +kernel
theorem actual_orbits : ∀k:Fin 4,∀s:Fin 5,rotateRow (rows k s)=rows k (s+1) := by decide +kernel
theorem actual_weight_bounds : ∀k:Fin 4,3<weights k := by decide +kernel
theorem actual_refinement_binding : ∀i j:Fin 4,scale (if i.val<2 then 5 else 1) (rows i 0 j)=(KoblitzRound18Refinement.C (ix i 0) (ix j 0),KoblitzRound18Refinement.C (ix i 1) (ix j 0)) := by decide +kernel
end KoblitzRound19OrbitCover
#print axioms KoblitzRound19OrbitCover.actual_weighted_Gram
#print axioms KoblitzRound19OrbitCover.actual_orbits
#print axioms KoblitzRound19OrbitCover.actual_weight_bounds
#print axioms KoblitzRound19OrbitCover.actual_refinement_binding
