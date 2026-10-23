import KoblitzRound19
namespace KoblitzRound19Polarization
open scoped BigOperators
def R : Matrix (Fin 4) (Fin 4) (ZMod 5) := !![-1,-1,-1,-1;1,0,0,0;0,1,0,0;0,0,1,0]
def v : Fin 4 → ZMod 5 := fun _ => 1
def H0 : Matrix (Fin 4) (Fin 4) (ZMod 5) := !![5,2,2,4;1,3,0,2;3,1,3,2;3,3,1,5]
def H1 : Matrix (Fin 4) (Fin 4) (ZMod 5) := !![3,2,1,1;2,4,2,1;2,3,4,2;1,2,2,3]
def transfer (M:Matrix (Fin 4) (Fin 4) (ZMod 5)) := ∑ j:Fin 5,(R^j.val).transpose*M*(R^j.val)
theorem actual_cycle : R^5=1 ∧ (∑ j:Fin 5,(R^j.val).transpose)=0 ∧ R.mulVec v=v := by decide +kernel
theorem transfer_annihilates (M:Matrix (Fin 4) (Fin 4) (ZMod 5)) : (transfer M).mulVec v=0 := by
 have fixed (k:Nat) : (R^k).mulVec v=v := by
  induction k with
  | zero => simp
  | succ k hk => rw [pow_succ,← Matrix.mulVec_mulVec,actual_cycle.2.2,hk]
 unfold transfer
 rw [Matrix.sum_mulVec]
 simp_rw [← Matrix.mulVec_mulVec,fixed]
 rw [← Matrix.sum_mulVec,actual_cycle.2.1]
 simp
theorem sum_transfers (M N:Matrix (Fin 4) (Fin 4) (ZMod 5)) : transfer (M+N)=transfer M+transfer N := by
 simp [transfer,mul_add,add_mul,Finset.sum_add_distrib]
theorem actual_H0_nonzero : H0.mulVec v≠0 ∧ ((3:ZMod 5) • H0).mulVec v≠0 := by decide +kernel
theorem no_transfer_H0 : ¬ ∃ M:Matrix (Fin 4) (Fin 4) (ZMod 5), transfer M=H0 ∨ transfer M=(3:ZMod 5) • H0 := by
 rintro ⟨M,hM⟩
 have hz:=transfer_annihilates M
 rcases hM with hM|hM
 · exact actual_H0_nonzero.1 (hM ▸ hz)
 · exact actual_H0_nonzero.2 (hM ▸ hz)
theorem actual_H1_nonzero : H1.mulVec v≠0 ∧ ((3:ZMod 5) • H1).mulVec v≠0 := by decide +kernel
theorem no_transfer_H1 : ¬ ∃ M:Matrix (Fin 4) (Fin 4) (ZMod 5), transfer M=H1 ∨ transfer M=(3:ZMod 5) • H1 := by
 rintro ⟨M,hM⟩
 have hz:=transfer_annihilates M
 rcases hM with hM|hM
 · exact actual_H1_nonzero.1 (hM ▸ hz)
 · exact actual_H1_nonzero.2 (hM ▸ hz)
end KoblitzRound19Polarization
#print axioms KoblitzRound19Polarization.actual_cycle
#print axioms KoblitzRound19Polarization.transfer_annihilates
#print axioms KoblitzRound19Polarization.sum_transfers
#print axioms KoblitzRound19Polarization.actual_H0_nonzero
#print axioms KoblitzRound19Polarization.actual_H1_nonzero
#print axioms KoblitzRound19Polarization.no_transfer_H0
#print axioms KoblitzRound19Polarization.no_transfer_H1
