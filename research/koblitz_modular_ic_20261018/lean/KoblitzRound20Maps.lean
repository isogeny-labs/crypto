import KoblitzRound19Maps
namespace KoblitzRound20Maps
def cases : List (Nat×Nat×Nat) := [(0,0,1),(0,1,0),(0,1,1),(1,0,1),(0,1,0),(0,1,0),(0,1,0),(0,1,0),(0,1,0),(0,1,0)]
theorem all_verified_constant_residues : cases.all (fun c=> KoblitzRound19Maps.residual c==0)=true := by decide +kernel
theorem actual_constant_control_count : cases.length=10 := by decide +kernel
end KoblitzRound20Maps
#print axioms KoblitzRound20Maps.all_verified_constant_residues
#print axioms KoblitzRound20Maps.actual_constant_control_count
