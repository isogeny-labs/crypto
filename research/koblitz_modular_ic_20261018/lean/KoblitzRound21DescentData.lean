import KoblitzRound20Retraction
namespace KoblitzRound21DescentData
open scoped BigOperators
open KoblitzRound19OrbitCover
def rotation : Fin 4→Fin 4→CM := ![![(-1,0),(-1,0),(-1,0),(-1,0)],![(1,0),(0,0),(0,0),(0,0)],![(0,0),(1,0),(0,0),(0,0)],![(0,0),(0,0),(1,0),(0,0)]]
def averaged : Fin 4→Fin 20→CM := ![![(-1663,43098),(-563,20696),(80615,43791),(-80060,-99806),(1671,-7779),(60614,-39338),(18653,-35155),(-2815,-12210),(19542,5457),(-95994,81246),(53941,49918),(25014,16538),(-4036,-4622),(4315,11393),(-79234,-73227),(56182,30734),(32497,15588),(6312,-2597),(4932,836),(-99923,-44561)],![(-563,20696),(80615,43791),(-80060,-99806),(1671,-7779),(-1663,43098),(18653,-35155),(-2815,-12210),(19542,5457),(-95994,81246),(60614,-39338),(25014,16538),(-4036,-4622),(4315,11393),(-79234,-73227),(53941,49918),(32497,15588),(6312,-2597),(4932,836),(-99923,-44561),(56182,30734)],![(80615,43791),(-80060,-99806),(1671,-7779),(-1663,43098),(-563,20696),(-2815,-12210),(19542,5457),(-95994,81246),(60614,-39338),(18653,-35155),(-4036,-4622),(4315,11393),(-79234,-73227),(53941,49918),(25014,16538),(6312,-2597),(4932,836),(-99923,-44561),(56182,30734),(32497,15588)],![(-80060,-99806),(1671,-7779),(-1663,43098),(-563,20696),(80615,43791),(19542,5457),(-95994,81246),(60614,-39338),(18653,-35155),(-2815,-12210),(4315,11393),(-79234,-73227),(53941,49918),(25014,16538),(-4036,-4622),(4932,836),(-99923,-44561),(56182,30734),(32497,15588),(6312,-2597)]]
def shift (i:Fin 20) : Fin 20 := ⟨5*(i.val/5)+(i.val+1)%5,by omega⟩
theorem actual_averaged_recovery : ∀i j:Fin 4,(∑k:Fin 20,cmul (averaged i k) (KoblitzRound20Retraction.row k j))=(if i=j then(5,0)else(0,0)) := by decide +kernel
theorem actual_equivariance : ∀i:Fin 4,∀j:Fin 20,averaged i (⟨5*(j.val/5)+(j.val+4)%5,by omega⟩)=(∑k:Fin 4,cmul (rotation i k) (averaged k j)) := by decide +kernel
theorem actual_rows_equivariance : ∀i:Fin 20,∀j:Fin 4,KoblitzRound20Retraction.row (shift i) j=(∑k:Fin 4,cmul (KoblitzRound20Retraction.row i k) (rotation k j)) := by decide +kernel
end KoblitzRound21DescentData
#print axioms KoblitzRound21DescentData.actual_averaged_recovery
#print axioms KoblitzRound21DescentData.actual_equivariance
#print axioms KoblitzRound21DescentData.actual_rows_equivariance
