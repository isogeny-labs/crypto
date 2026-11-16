import S3System

/-! The exact algebraic recovery step used by the phase-accounted native route.
The native audit checks the base-log and signed-point hypotheses for each call.
No cost model or general twist map is assumed by this result. -/
namespace PhaseRecovery

theorem shifted_relation_recovers {A B : Type*} [AddCommGroup A] [AddCommGroup B]
    (φ : A →+ B) (hφ : Function.Injective φ) (P Q : A) (B0 B1 : B)
    (l0 l1 i j s : ℤ)
    (hb0 : B0=l0 • φ P) (hb1 : B1=l1 • φ P)
    (hrel : i • B0+j • B1=φ Q+s • φ P) :
    Q=(i*l0+j*l1-s) • P := by
  apply hφ
  rw [map_zsmul]
  have h : (i*l0+j*l1) • φ P=φ Q+s • φ P := by
    simpa only [hb0,hb1,add_zsmul,mul_zsmul] using hrel
  rw [sub_zsmul,h]
  abel

theorem annihilator_mod {A : Type*} [AddCommGroup A]
    (P Q : A) (z r : ℤ) (hr : r • P=0) (hz : Q=z • P) :
    Q=(z%r) • P := by
  rw [hz]
  have hi := Int.emod_add_mul_ediv z r
  calc
    z • P = (z%r+r*(z/r)) • P := by rw [hi]
    _ = (z%r) • P := by
      rw [add_zsmul,mul_comm r,mul_zsmul,hr]
      simp

theorem shifted_relation_recovers_mod {A B : Type*} [AddCommGroup A] [AddCommGroup B]
    (φ : A →+ B) (hφ : Function.Injective φ) (P Q : A) (B0 B1 : B)
    (l0 l1 i j s r : ℤ) (hr : r • P=0)
    (hb0 : B0=l0 • φ P) (hb1 : B1=l1 • φ P)
    (hrel : i • B0+j • B1=φ Q+s • φ P) :
    Q=((i*l0+j*l1-s)%r) • P := by
  exact annihilator_mod P Q (i*l0+j*l1-s) r hr
    (shifted_relation_recovers φ hφ P Q B0 B1 l0 l1 i j s hb0 hb1 hrel)

namespace F43
open S3System.F43

theorem source_annihilator : (11:ℤ) • P=0 := by decide +kernel

/-- Instantiate the generic group argument on the actual checked F43 map.
All field/curve/map definitions and injectivity come from the frozen project. -/
theorem actual_recovery_mod11 (Q : E.Point) (B0 B1 : T.Point)
    (l0 l1 i j s : ℤ)
    (hb0 : B0=l0 • U) (hb1 : B1=l1 • U)
    (hrel : i • B0+j • B1=toTwist Q+s • U) :
    Q=((i*l0+j*l1-s)%11) • P := by
  apply shifted_relation_recovers_mod toTwist toTwist_injective P Q B0 B1
    l0 l1 i j s 11 source_annihilator
  · simpa only [source_image] using hb0
  · simpa only [source_image] using hb1
  · simpa only [source_image] using hrel

#print axioms source_annihilator
#print axioms actual_recovery_mod11
end F43
#print axioms shifted_relation_recovers
#print axioms annihilator_mod
#print axioms shifted_relation_recovers_mod
end PhaseRecovery
