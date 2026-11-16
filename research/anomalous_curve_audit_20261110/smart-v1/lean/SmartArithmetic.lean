import PointTrace
import Mathlib.Data.Nat.Bits

/-! Pure finite-ring program semantics. No generic local-ring elliptic group
or p-adic logarithm theorem is assumed. Concrete executions will be checked
against these functions and the separately proved actual point relation. -/
namespace SmartArithmeticAudit

structure J (R : Type*) where
  x : R
  y : R
  z : R
  deriving DecidableEq

inductive Op where
  | double
  | addBase
  deriving DecidableEq

section RingProgram
variable {R : Type*} [CommRing R] [DecidableEq R]

def infinity : J R := ⟨1,1,0⟩

def double (a : R) (P : J R) : J R :=
  if P.z=0 then infinity else
  let yy := P.y^2
  let zz := P.z^2
  let s := 4*P.x*yy
  let m := 3*P.x^2+a*zz^2
  let x := m^2-2*s
  ⟨x,m*(s-x)-8*yy^2,2*P.y*P.z⟩

def add (a : R) (P Q : J R) : J R :=
  if P.z=0 then Q else if Q.z=0 then P else
  let z₁z₁ := P.z^2
  let z₂z₂ := Q.z^2
  let u₁ := P.x*z₂z₂
  let u₂ := Q.x*z₁z₁
  let s₁ := P.y*Q.z*z₂z₂
  let s₂ := Q.y*P.z*z₁z₁
  if u₁=u₂ then (if s₁=s₂ then double a P else infinity) else
  let h := u₂-u₁
  let r := s₂-s₁
  let hh := h^2
  let hhh := h*hh
  let v := u₁*hh
  let x := r^2-hhh-2*v
  ⟨x,r*(v-x)-s₁*hhh,P.z*Q.z*h⟩

def curveEquation (a b : R) (P : J R) : Prop :=
  P.y^2=P.x^3+a*P.x*P.z^4+b*P.z^6

def step (a : R) (base P : J R) (op : Op) : J R :=
  match op with
  | .double => double a P
  | .addBase => add a P base

def runOps (a : R) (base : J R) (ops : List Op) : J R :=
  ops.foldl (step a base) base

theorem runOps_snoc (a : R) (base : J R) (ops : List Op) (op : Op) :
    runOps a base (ops++[op])=step a base (runOps a base ops) op := by
  simp [runOps,List.foldl_append]

inductive Trace (a : R) (base : J R) : List Op→J R→Prop where
  | initial : Trace a base [] base
  | append {ops : List Op} {P Q : J R} (before : Trace a base ops P)
      (op : Op) (hstep : step a base P op=Q) : Trace a base (ops++[op]) Q

theorem Trace.sound {a : R} {base final : J R} {ops : List Op}
    (h : Trace a base ops final) : runOps a base ops=final := by
  induction h with
  | initial => rfl
  | append before op hstep ih =>
    rw [runOps_snoc,ih]
    exact hstep

end RingProgram

def binaryOps (n : ℕ) : List Op :=
  (n.bits.reverse.drop 1).flatMap fun b =>
    if b then [.double,.addBase] else [.double]

def run {R : Type*} [CommRing R] [DecidableEq R]
    (a : R) (n : ℕ) (base : J R) : J R :=
  if n=0 then infinity else runOps a base (binaryOps n)

theorem trace_run {R : Type*} [CommRing R] [DecidableEq R]
    {a : R} {base final : J R} {ops : List Op} {n : ℕ}
    (hn : n≠0) (hops : ops=binaryOps n) (h : Trace a base ops final) :
    run a n base=final := by
  rw [run,ite_eq_right hn,←hops]
  exact h.sound

/-- Finite Hensel square step, proved as an integer polynomial identity.
The quotient and inverse/correction congruence must be checked separately. -/
theorem hensel_square_step (p x y a b t c s : ℤ)
    (hbase : x^3+a*x+b-y^2=p*t) (hlinear : 2*y*c-t=p*s) :
    (y+p*c)^2-(x^3+a*x+b)=p^2*(s+c^2) := by
  linear_combination p*hlinear-hbase

theorem hensel_reduces (p y c : ℤ) : (y+p*c)%p=y%p := by
  simp [Int.add_emod]

def phiValue (p : ℕ) (P : J (ZMod (p^2))) : ZMod (p^2) :=
  -P.x*P.z*P.y⁻¹

def coefficient (p : ℕ) (P : J (ZMod (p^2))) : ℕ :=
  (phiValue p P).val/p

def recover (p cG cQ : ℕ) : ZMod p :=
  (cQ : ZMod p)*(cG : ZMod p)⁻¹

theorem large_binary_operation_count :
    (binaryOps AnomalousPrimeAudit.p).length=210 := by decide +kernel

theorem small_binary_operations :
    binaryOps 5=[.double,.double,.addBase] := by decide +kernel

theorem recover_of_inverse_witness (p cG cQ i : ℕ)
    (h : (cG : ZMod p)*(i : ZMod p)=1) :
    recover p cG cQ=(cQ : ZMod p)*(i : ZMod p) := by
  rw [recover,ZMod.inv_eq_of_mul_eq_one p _ _ h]

#print axioms Trace.sound
#print axioms hensel_square_step
#print axioms recover_of_inverse_witness
end SmartArithmeticAudit
