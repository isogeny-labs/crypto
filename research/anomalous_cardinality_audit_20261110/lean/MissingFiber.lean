import PointTrace
import Mathlib.NumberTheory.LegendreSymbol.Basic

namespace AnomalousMissingFiberAudit
open AnomalousPrimeAudit AnomalousCurveDataAudit

def constantB : F := 203362936548826936673264444982866339953265530166

theorem constantB_ne_zero : constantB≠0 := by
  unfold constantB F p
  reduce_mod_char
  decide +kernel

/-- A closed candidate nonresidue certificate, not an assumed root-search result. -/
theorem constantB_euler : constantB^(p/2)=(-1:F) := by
  unfold constantB F p
  reduce_mod_char

theorem missing_zero_fiber : ¬∃ y : F, y^2=constantB := by
  rintro ⟨y,hy⟩
  have hs : IsSquare constantB := ⟨y,by simpa [pow_two] using hy.symm⟩
  have he := (ZMod.euler_criterion p constantB_ne_zero).mp hs
  rw [constantB_euler] at he
  have hn : (-1:F)≠1 := by
    unfold F p
    reduce_mod_char
    decide +kernel
  exact hn he

#print axioms constantB_euler
#print axioms missing_zero_fiber
end AnomalousMissingFiberAudit
