import RecodedCanonical
import SupportOneHot

noncomputable section
open MvPolynomial
namespace FourTrackFixture
abbrev F := SecantFixture.F
local instance : Fact (Nat.Prime 17) := ⟨by decide⟩

def oneHotGenerators (r s e f : F) :=
  SupportOneHot.generators r s 2 (4*e) (4*f) (9*e) (9*f) (9*e) (9*f)
    ![2*e,10*e,12*e,0,0,0] ![0,0,0,2*f,10*f,12*f]

theorem xor_degree_six (r s e f : F) (he : e ≠ 0) (hf : f ≠ 0) :
    SecantIdeal.FirstVanishing (DegreeBridge.topIdeal
      (RecodedCanonical.fixtureGenerators r s e f)) 6 :=
  RecodedCanonical.canonical_first_vanishing 2 r s (4*e) (4*f)
    (9*e) (9*f) (9*e) (9*f) (2*e) (12*e) (2*f) (12*f)
    (mul_ne_zero (by decide) he) (mul_ne_zero (by decide) hf)

theorem one_hot_degree_five (r s e f : F) (he : e ≠ 0) (hf : f ≠ 0) :
    SecantIdeal.FirstVanishing (DegreeBridge.topIdeal (oneHotGenerators r s e f)) 5 :=
  SupportOneHot.canonical_first_vanishing r s 2 (4*e) (4*f)
    (9*e) (9*f) (9*e) (9*f)
    (mul_ne_zero (by decide) he) (mul_ne_zero (by decide) hf) (by decide)
    ![2*e,10*e,12*e,0,0,0] ![0,0,0,2*f,10*f,12*f]


theorem xor_polynomial_complete {r s x z y v : F}
    (hP : PencilFixture.W.Nonsingular x y) (hQ : PencilFixture.W.Nonsingular z v)
    (hR : PencilFixture.W.Nonsingular r s) :
    (x ∈ PencilFixture.baseX ∧ z ∈ PencilFixture.baseX ∧
      WeierstrassCurve.Affine.Point.some hP+WeierstrassCurve.Affine.Point.some hQ =
        WeierstrassCurve.Affine.Point.some hR) ↔
      ∃ e f m : F, ∃ u : Fin 4 → F,
        (e = 1 ∨ e = -1) ∧ (f = 1 ∨ f = -1) ∧
        y = e*Recoding.graph 4 9 9 x (u 0) (u 1) ∧
        v = f*Recoding.graph 4 9 9 z (u 2) (u 3) ∧
        ∀ p ∈ RecodedCanonical.fixtureGenerators r s e f,
          eval (RecodedCanonical.coordinates x z m u) p = 0 := by
  rw [PencilFixture.polynomial_complete hP hQ hR]
  constructor
  · rintro ⟨e,f,m,u,he,hf,hy,hv,hp⟩
    have hs := (PencilFixture.zeroes_iff_system r s e f x z m u).mp hp
    have hu : PencilSelector.Boolean u := by
      intro i
      fin_cases i
      · exact hs.1 0
      · exact hs.1 1
      · exact hs.2.1 0
      · exact hs.2.1 1
    refine ⟨e,f,m,Recoding.transform u,he,hf,?_,?_,?_⟩
    · simpa only [Recoding.left_graph, Recoding.transform_involution u hu] using hy
    · simpa only [Recoding.right_graph, Recoding.transform_involution u hu] using hv
    · exact (RecodedCanonical.fixture_zeroes_iff r s e f x z m _).mpr
        (Recoding.original_to_system hs)
  · rintro ⟨e,f,m,u,he,hf,hy,hv,hp⟩
    have hs := Recoding.system_to_original
      ((RecodedCanonical.fixture_zeroes_iff r s e f x z m u).mp hp)
    refine ⟨e,f,m,Recoding.transform u,he,hf,?_,?_,?_⟩
    · simpa only [Recoding.left_graph] using hy
    · simpa only [Recoding.right_graph] using hv
    · exact (PencilFixture.zeroes_iff_system r s e f x z m _).mpr hs

end FourTrackFixture
