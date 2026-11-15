import Mathlib

/- Source-only finite incidence lemma. Does not identify these masks with
   geometric torsion, establish Frobenius, or construct a polarized quotient. -/
namespace P256TransferSearch

def plane (i : Fin 3) : List Nat :=
  match i.val with
  | 0 => [39, 75, 108]
  | 1 => [51, 71, 116]
  | _ => [43, 83, 120]

def kernel (i : Fin 9) : List Nat :=
  match i.val with
  | 0 => [3, 36, 39, 72, 75, 108, 111]
  | 1 => [3, 40, 43, 80, 83, 120, 123]
  | 2 => [3, 48, 51, 68, 71, 116, 119]
  | 3 => [29, 39, 58, 75, 86, 108, 113]
  | 4 => [29, 43, 54, 78, 83, 101, 120]
  | 5 => [29, 46, 51, 71, 90, 105, 116]
  | 6 => [30, 39, 57, 75, 85, 108, 114]
  | 7 => [30, 43, 53, 77, 83, 102, 120]
  | _ => [30, 45, 51, 71, 89, 105, 116]

theorem unique_plane_in_each_kernel :
    ∀ k : Fin 9, ∃! i : Fin 3,
      (plane i).all (fun v => (kernel k).contains v) = true := by
  decide

/- Universal cleared Vélu identity for an inflection-point parameterization.
   It is an integer polynomial lemma, not a curve/isogeny construction. -/
theorem velu_three_cleared (x u s : ℤ) :
    let r := 3*u^2
    let a := 6*u*s-27*u^4
    let b := s^2-18*u^3*s+54*u^6
    let v := 12*u*s
    let w := 4*s^2
    let d := x-r
    let n := x*d^2+v*d+w
    let m := d^3-v*d-2*w
    let aa := a-5*v
    let bb := b-7*(w+r*v)
    (x^3+a*x+b)*m^2 = n^3+aa*n*d^4+bb*d^6 := by
  dsimp
  ring

end P256TransferSearch
