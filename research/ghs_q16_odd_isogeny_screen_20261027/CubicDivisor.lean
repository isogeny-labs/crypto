/- Kernel-checked finite arithmetic for the two classes excluded at all five
   screened odd primes. This proves the cubic-divisor predicate only; the
   Frobenius-module implication is explained and sourced in REPORT.md. -/

namespace CubicGHS

private def subMod (p x y : Nat) : Nat := (x % p + p - y % p) % p

-- In the quotient by T³+aT²+bT+c, multiplication by T on a remainder.
private def timesT (p a b c : Nat) (r : Nat × Nat × Nat) : Nat × Nat × Nat :=
  (subMod p 0 (r.2.2 * c),
   subMod p r.1 (r.2.2 * b),
   subMod p r.2.1 (r.2.2 * a))

private def tPower (p a b c : Nat) : Nat → Nat × Nat × Nat
  | 0 => (1 % p, 0, 0)
  | n + 1 => timesT p a b c (tPower p a b c n)

private def cubicDivides (order p a b c : Nat) : Bool := Id.run do
  let r3 := tPower p a b c 3
  let r6 := tPower p a b c 6
  let tr := (4097 + 1000 * p - order) % p
  return subMod p (r6.1 + 4096) (tr * r3.1) == 0 &&
    subMod p r6.2.1 (tr * r3.2.1) == 0 &&
    subMod p r6.2.2 (tr * r3.2.2) == 0

private def hasCubic (order p : Nat) : Bool :=
  (List.range p).any fun a =>
    (List.range p).any fun b =>
      (List.range p).any fun c => cubicDivides order p a b c

theorem order4076_mod3 : hasCubic 4076 3 = false := by decide
theorem order4076_mod5 : hasCubic 4076 5 = false := by decide
theorem order4076_mod7 : hasCubic 4076 7 = false := by decide
theorem order4076_mod11 : hasCubic 4076 11 = false := by decide
theorem order4076_mod13 : hasCubic 4076 13 = false := by decide

theorem order4118_mod3 : hasCubic 4118 3 = false := by decide
theorem order4118_mod5 : hasCubic 4118 5 = false := by decide
theorem order4118_mod7 : hasCubic 4118 7 = false := by decide
theorem order4118_mod11 : hasCubic 4118 11 = false := by decide
theorem order4118_mod13 : hasCubic 4118 13 = false := by decide

end CubicGHS
