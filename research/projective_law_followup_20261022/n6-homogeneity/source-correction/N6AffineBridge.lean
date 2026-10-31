import Mathlib.Algebra.Field.Basic
import Mathlib.Tactic.Ring

/-!
Polynomial bridge for the pinned Magma `DBL2` assignments and the printed
affine doubling coefficients in Ozbay Gurler–Hisil Eq. (7). The author source
defines `WW1 := W1^2` and uses `WW1` in `N6`. These definitions model that
assignment; the external source is separately byte/line checked by verify.py.
This is not a formal semantics proof for Magma or compiled Rust.
-/

set_option autoImplicit false
namespace N6AffineBridge

variable {R : Type*} [CommRing R]

def aAffine (q r s t a3 a2 : R) : R :=
  (((q ^ 2 - 4 * r + a3) * q - a2 + s ^ 2) * (q * s - t)) +
    (3 * q ^ 2 - 2 * r + a3) * r * s

def bAffine (q r s t : R) : R := 2 * ((q * s - t) * t - r * s ^ 2)

def cAffine (q r s t a3 a2 : R) : R :=
  ((q ^ 2 - 4 * r + a3) * q - a2 + s ^ 2) * s +
    (3 * q ^ 2 - 2 * r + a3) * t

def n1 (q s t z w : R) : R := (q * z ^ 2) * (s * z ^ 3 * w) - t * z ^ 5 * w
def n2 (r s z w : R) : R := (r * z ^ 4) * (s * z ^ 3 * w)
def n5 (q r s a3 a2 z w : R) : R :=
  (s * z ^ 3 * w) ^ 2 -
  ((q * z ^ 2) * (4 * (r * z ^ 4) - (q * z ^ 2) ^ 2) -
    ((q * z ^ 2) * a3 * z ^ 4 - a2 * z ^ 6)) * w ^ 2
def n6Squared (q r a3 z w : R) : R :=
  (2 * (r * z ^ 4) - 3 * (q * z ^ 2) ^ 2 - a3 * z ^ 4) * w ^ 2
def n6LinearMutation (q r a3 z w : R) : R :=
  (2 * (r * z ^ 4) - 3 * (q * z ^ 2) ^ 2 - a3 * z ^ 4) * w

def aWeighted (q r s t a3 a2 z w : R) : R :=
  n1 q s t z w * n5 q r s a3 a2 z w - n2 r s z w * n6Squared q r a3 z w
def bWeighted (q r s t z w : R) : R :=
  2 * (n1 q s t z w * (t * z ^ 5 * w) - n2 r s z w * (s * z ^ 3 * w))
def cWeighted (q r s t a3 a2 z w : R) : R :=
  (s * z ^ 3 * w) * n5 q r s a3 a2 z w -
    (t * z ^ 5 * w) * n6Squared q r a3 z w

theorem author_a_matches_printed_affine (q r s t a3 a2 z w : R) :
    aWeighted q r s t a3 a2 z w = z ^ 11 * w ^ 3 * aAffine q r s t a3 a2 := by
  simp [aWeighted, n1, n2, n5, n6Squared, aAffine]
  ring

theorem author_b_matches_printed_affine (q r s t z w : R) :
    bWeighted q r s t z w = z ^ 10 * w ^ 2 * bAffine q r s t := by
  simp [bWeighted, n1, n2, bAffine]
  ring

theorem author_c_matches_printed_affine (q r s t a3 a2 z w : R) :
    cWeighted q r s t a3 a2 z w = z ^ 9 * w ^ 3 * cAffine q r s t a3 a2 := by
  simp [cWeighted, n5, n6Squared, cAffine]
  ring

theorem linear_mutation_a_defect (q r s t a3 a2 z w : R) :
    (n1 q s t z w * n5 q r s a3 a2 z w -
      n2 r s z w * n6LinearMutation q r a3 z w) -
      aWeighted q r s t a3 a2 z w =
      z ^ 11 * w ^ 2 * (w - 1) * r * s * (2 * r - 3 * q ^ 2 - a3) := by
  simp [aWeighted, n1, n2, n5, n6Squared, n6LinearMutation]
  ring

theorem linear_mutation_c_defect (q r s t a3 a2 z w : R) :
    ((s * z ^ 3 * w) * n5 q r s a3 a2 z w -
      (t * z ^ 5 * w) * n6LinearMutation q r a3 z w) -
      cWeighted q r s t a3 a2 z w =
      z ^ 9 * w ^ 2 * (w - 1) * t * (2 * r - 3 * q ^ 2 - a3) := by
  simp [cWeighted, n5, n6Squared, n6LinearMutation]
  ring

end N6AffineBridge

#print axioms N6AffineBridge.author_a_matches_printed_affine
#print axioms N6AffineBridge.author_b_matches_printed_affine
#print axioms N6AffineBridge.author_c_matches_printed_affine
#print axioms N6AffineBridge.linear_mutation_a_defect
#print axioms N6AffineBridge.linear_mutation_c_defect
