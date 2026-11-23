import SmartArithmetic

/-! Composition of the predata finite-ring functions into one public-input
attempt. This reads no known scalar. Concrete postdata certificates must
prove all lifting, execution and recovery obligations rather than assuming
the trace correct or a p-adic transfer theorem. -/
namespace SmartSpecificationAudit
open SmartArithmeticAudit

def henselValue (p a b x y : ℕ) : ℕ :=
  let t : ℤ := ((x:ℤ)^3+(a:ℤ)*x+b-(y:ℤ)^2)/(p:ℤ)
  let c : ZMod p := (t:ZMod p)*((2*y:ℕ):ZMod p)⁻¹
  y+p*c.val

theorem henselValue_of_witness (p a b x y c u : ℕ) (t : ℤ)
    (ht : ((x:ℤ)^3+(a:ℤ)*x+b-(y:ℤ)^2)/(p:ℤ)=t)
    (hu : ((2*y:ℕ):ZMod p)*(u:ZMod p)=1)
    (hc : (t:ZMod p)*(u:ZMod p)=(c:ZMod p)) (hbound : c<p) :
    henselValue p a b x y=y+p*c := by
  dsimp only [henselValue]
  rw [ht,ZMod.inv_eq_of_mul_eq_one p _ _ hu,hc,ZMod.val_natCast,
    Nat.mod_eq_of_lt hbound]

def lifted (p a b x y : ℕ) : J (ZMod (p^2)) :=
  ⟨x,henselValue p a b x y,1⟩

def attempt (p a b gx gy qx qy : ℕ) : Option ℕ :=
  let pG := run (a:ZMod (p^2)) p (lifted p a b gx gy)
  let pQ := run (a:ZMod (p^2)) p (lifted p a b qx qy)
  let cG := coefficient p pG
  let cQ := coefficient p pQ
  if cG=0 then none else some (recover p cG cQ).val

theorem attempt_of_witness (p a b gx gy qx qy cG cQ d : ℕ)
    (outG outQ : J (ZMod (p^2)))
    (hg : run (a:ZMod (p^2)) p (lifted p a b gx gy)=outG)
    (hq : run (a:ZMod (p^2)) p (lifted p a b qx qy)=outQ)
    (hcg : coefficient p outG=cG) (hcq : coefficient p outQ=cQ)
    (hne : cG≠0) (hd : (recover p cG cQ).val=d) :
    attempt p a b gx gy qx qy=some d := by
  simp only [attempt,hg,hq,hcg,hcq,ite_eq_right hne,hd]

#print axioms henselValue_of_witness
#print axioms attempt_of_witness
end SmartSpecificationAudit

namespace SmartSpecificationAudit
open SmartArithmeticAudit

theorem coefficient_of_inverse_witness (p x y z u v c : ℕ)
    (hinv : (y:ZMod (p^2))*(u:ZMod (p^2))=1)
    (hphi : -(x:ZMod (p^2))*(z:ZMod (p^2))*(u:ZMod (p^2))=(v:ZMod (p^2)))
    (hbound : v<p^2) (hquot : v/p=c) :
    coefficient p ⟨x,y,z⟩=c := by
  dsimp only [coefficient,phiValue]
  rw [ZMod.inv_eq_of_mul_eq_one (p^2) _ _ hinv,hphi,
    ZMod.val_natCast,Nat.mod_eq_of_lt hbound,hquot]

#print axioms coefficient_of_inverse_witness
end SmartSpecificationAudit

namespace SmartSpecificationAudit
open SmartArithmeticAudit

theorem recovery_value_of_witness (p cG cQ u d : ℕ)
    (hinv : (cG:ZMod p)*(u:ZMod p)=1)
    (hprod : (cQ:ZMod p)*(u:ZMod p)=(d:ZMod p))
    (hbound : d<p) : (recover p cG cQ).val=d := by
  rw [recover_of_inverse_witness p cG cQ u hinv,hprod,
    ZMod.val_natCast,Nat.mod_eq_of_lt hbound]

#print axioms recovery_value_of_witness
end SmartSpecificationAudit
