import KoblitzRound14
namespace KoblitzRound14Supports
def degrees0 : List Nat := [280,280,280,280,280,50,50,50,50,50,11,11,11,11,11,55]
theorem degree0_total : degrees0.sum=1760 := by decide +kernel
theorem degree0_count : degrees0.length=16 := by decide +kernel
def degrees1 : List Nat := [560,28,28,28,28,28,140,305,305,430]
theorem degree1_total : degrees1.sum=1880 := by decide +kernel
theorem degree1_count : degrees1.length=10 := by decide +kernel
def degrees2 : List Nat := [420,165,33,33,33,33,33,970,970,7,7,7,7,7,35]
theorem degree2_total : degrees2.sum=2760 := by decide +kernel
theorem degree2_count : degrees2.length=15 := by decide +kernel
def degrees3 : List Nat := [85,85,85,85,85,125,125,85,85,85,85,85,270]
theorem degree3_total : degrees3.sum=1370 := by decide +kernel
theorem degree3_count : degrees3.length=13 := by decide +kernel
def degrees4 : List Nat := [830,675,675]
theorem degree4_total : degrees4.sum=2180 := by decide +kernel
theorem degree4_count : degrees4.length=3 := by decide +kernel
end KoblitzRound14Supports
#print axioms KoblitzRound14Supports.degree0_total
#print axioms KoblitzRound14Supports.degree0_count
#print axioms KoblitzRound14Supports.degree1_total
#print axioms KoblitzRound14Supports.degree1_count
#print axioms KoblitzRound14Supports.degree2_total
#print axioms KoblitzRound14Supports.degree2_count
#print axioms KoblitzRound14Supports.degree3_total
#print axioms KoblitzRound14Supports.degree3_count
#print axioms KoblitzRound14Supports.degree4_total
#print axioms KoblitzRound14Supports.degree4_count
