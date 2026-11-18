import Mathlib.RingTheory.MvPolynomial.Homogeneous
import Mathlib.Tactic

/-! Literal homogeneous quadratics from Joux–Vitse's empty F101^3 toy
example (author manuscript p7), not an assumed generic sequence.
The universal identities below certify actual ideal membership. -/
namespace JVMacaulayAudit
variable {R : Type*} [CommRing R]

def f0 (x y h : R) : R := 100*x^2+32*x*y+55*x*h+y^2+17*y*h+81*h^2
def f1 (x y h : R) : R := 29*x^2+34*x*y+71*x*h+83*y*h+16*h^2
def f2 (x y h : R) : R := 59*x^2+27*x*y+31*x*h+48*y*h+32*h^2
def gy (x y h : R) : R := y^2+19*x*h+3*y*h+39*h^2
def gxy (x y h : R) : R := x*y+42*x*h+34*y*h+29*h^2
def gx (x y h : R) : R := x^2+96*x*h+64*y*h+78*h^2
def r1 (x y h : R) : R :=
  (x+34*h)*gy x y h+(39*h-y)*gxy x y h-19*h*gx x y h
def r2 (x y h : R) : R :=
  64*h*gy x y h+(x+62*h)*gxy x y h-(y+42*h)*gx x y h
def cy (_x y h : R) : R := y*h^2+82*h^3
def cx (x _y h : R) : R := x*h^2+66*h^3

local macro "char_ring" : tactic => `(tactic| (
  ring_nf
  all_goals reduce_mod_char!
  all_goals ring_nf
  all_goals reduce_mod_char!))

variable [CharP R 101]

theorem quadratic_certificates (x y h : R) :
    gy x y h=f0 x y h+83*f1 x y h+14*f2 x y h ∧
    gxy x y h=88*f1 x y h+80*f2 x y h ∧
    gx x y h=71*f1 x y h+49*f2 x y h := by
  constructor
  · unfold gy f0 f1 f2
    char_ring
  constructor
  · unfold gxy f1 f2
    char_ring
  · unfold gx f1 f2
    char_ring

theorem reverse_quadratic_certificates (x y h : R) :
    f0 x y h=gy x y h+32*gxy x y h+100*gx x y h ∧
    f1 x y h=34*gxy x y h+29*gx x y h ∧
    f2 x y h=27*gxy x y h+59*gx x y h := by
  constructor
  · unfold gy gxy gx f0
    char_ring
  constructor
  · unfold gxy gx f1
    char_ring
  · unfold gxy gx f2
    char_ring

theorem cubic_certificates (x y h : R) :
    cy x y h=42*r1 x y h+93*r2 x y h ∧
    cx x y h=52*r1 x y h+59*r2 x y h := by
  constructor
  · unfold cy r1 r2 gy gxy gx
    char_ring
  · unfold cx r1 r2 gy gxy gx
    char_ring

theorem quartic_certificate (x y h : R) :
    h^4=77*(h^2*gx x y h-x*cx x y h-30*h*cx x y h-64*h*cy x y h) := by
  unfold gx cx cy
  char_ring

end JVMacaulayAudit
