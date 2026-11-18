import Mathlib.Tactic.Ring

/-! Universal identities and exact exported integer unit certificates.
No geometric transfer, CM embedding array, or large-prime theorem is imported. -/
set_option maxRecDepth 100000
namespace CMRecovery42
variable {R : Type*} [CommRing R]

theorem quadratic_bezout_numerator (f u v s : R) :
    (u+2*v-s*f)*(f^2+s*v*f+2*v^2)-
      (v+2*u-s*f)*(f^2+s*u*f+2*u^2) =
    2*(v-u)*(2*u^2+3*u*v+2*v^2)+(1-s^2)*(v-u)*f^2 := by ring

theorem normalized_quadratic_recovery (f u v s d : R) (hs : s^2=1)
    (hu : (2*(v-u)*(2*u^2+3*u*v+2*v^2))*d=1) :
    ((u+2*v-s*f)*d)*(f^2+s*v*f+2*v^2)-
      ((v+2*u-s*f)*d)*(f^2+s*u*f+2*u^2)=1 := by
  calc
    _ = ((u+2*v-s*f)*(f^2+s*v*f+2*v^2)-
          (v+2*u-s*f)*(f^2+s*u*f+2*u^2))*d := by ring
    _ = (2*(v-u)*(2*u^2+3*u*v+2*v^2))*d := by
      rw [quadratic_bezout_numerator, hs]
      ring
    _ = 1 := hu

theorem signed_cm_root_identity (a u s : R) :
    (-s*a*u)^2+s*u*(-s*a*u)+2*u^2 =
      (a^2-a+2)*u^2+(s^2-1)*(a^2-a)*u^2 := by ring

theorem signed_cm_annihilator (a u s : R) (hs : s^2=1)
    (ha : a^2-a+2=0) :
    (-s*a*u)^2+s*u*(-s*a*u)+2*u^2=0 := by
  rw [signed_cm_root_identity, hs, ha]
  ring

theorem cast_unit_of_integer_bezout (r : ℕ) (a b k : ℤ)
    (h : a*b+(r : ℤ)*k=1) (hr : (r : R)=0) :
    (a : R)*(b : R)=1 := by
  have hc := congrArg (fun z : ℤ => (z : R)) h
  simp only [Int.cast_add, Int.cast_mul, Int.cast_natCast, Int.cast_one] at hc
  rw [hr, zero_mul, add_zero] at hc
  exact hc

theorem affine_projector_sum (a b x d : R) (h : (a-b)*d=1) :
    (x-b)*d+(a-x)*d=1 := by
  calc
    _ = (a-b)*d := by ring
    _ = 1 := h

theorem affine_projector_first_nodes (a b d : R) (h : (a-b)*d=1) :
    ((a-b)*d,(b-b)*d)=(1,0) := by simp [h]

theorem affine_projector_second_nodes (a b d : R) (h : (a-b)*d=1) :
    ((a-a)*d,(a-b)*d)=(0,1) := by simp [h]

theorem equal_system_no_separator {S : Type*} [Nontrivial R]
    (x y : S) (he : x=y) (f : S → R) (h₁ : f x=1) (h₀ : f y=0) : False := by
  rw [he] at h₁
  exact one_ne_zero (h₁.symm.trans h₀)

end CMRecovery42

namespace CMRecovery42Data
theorem unit_001 : (10 : ℤ)*(10 : ℤ)+(11 : ℤ)*(-9 : ℤ)=1 := by decide

theorem unit_002 : (-2 : ℤ)*(5 : ℤ)+(11 : ℤ)*(1 : ℤ)=1 := by decide

theorem unit_003 : (-1 : ℤ)*(10 : ℤ)+(11 : ℤ)*(1 : ℤ)=1 := by decide

theorem unit_004 : (1 : ℤ)*(1 : ℤ)+(11 : ℤ)*(0 : ℤ)=1 := by decide

theorem unit_005 : (10 : ℤ)*(10 : ℤ)+(11 : ℤ)*(-9 : ℤ)=1 := by decide

theorem unit_006 : (2 : ℤ)*(6 : ℤ)+(11 : ℤ)*(-1 : ℤ)=1 := by decide

theorem unit_007 : (-1 : ℤ)*(10 : ℤ)+(11 : ℤ)*(1 : ℤ)=1 := by decide

theorem unit_008 : (1 : ℤ)*(1 : ℤ)+(11 : ℤ)*(0 : ℤ)=1 := by decide

theorem unit_009 : (4 : ℤ)*(6 : ℤ)+(23 : ℤ)*(-1 : ℤ)=1 := by decide

theorem unit_010 : (23 : ℤ)*(474 : ℤ)+(991 : ℤ)*(-11 : ℤ)=1 := by decide

theorem unit_011 : (902 : ℤ)*(579 : ℤ)+(991 : ℤ)*(-527 : ℤ)=1 := by decide

theorem unit_012 : (274 : ℤ)*(387 : ℤ)+(991 : ℤ)*(-107 : ℤ)=1 := by decide

theorem unit_013 : (384 : ℤ)*(911 : ℤ)+(991 : ℤ)*(-353 : ℤ)=1 := by decide

theorem unit_014 : (764 : ℤ)*(406 : ℤ)+(991 : ℤ)*(-313 : ℤ)=1 := by decide

theorem unit_015 : (2417851639230448765924272 : ℤ)*(708278092064773670860439 : ℤ)+(2417851639230796216685689 : ℤ)*(-708278092064671889693063 : ℤ)=1 := by decide

theorem unit_016 : (1908826191049493886840860 : ℤ)*(418125298201728363790074 : ℤ)+(2417851639230796216685689 : ℤ)*(-330098219178473559081551 : ℤ)=1 := by decide

theorem unit_017 : (1078354185484476628052286 : ℤ)*(1413116525978634487311393 : ℤ)+(2417851639230796216685689 : ℤ)*(-630245502098354915671973 : ℤ)=1 := by decide

theorem unit_018 : (38531015900842053623 : ℤ)*(671980027613102003882969958903234406811 : ℤ)+(680564733841876926932320129493409985129 : ℤ)*(-38044982117784118188 : ℤ)=1 := by decide

theorem unit_019 : (287542585610154742088255099331519257216 : ℤ)*(347816621393907566520146488377445811900 : ℤ)+(680564733841876926932320129493409985129 : ℤ)*(-146954559442437039926337844673769326631 : ℤ)=1 := by decide

theorem unit_020 : (-84445472508592179912442669808708738725 : ℤ)*(406122850681270047655254600005235720825 : ℤ)+(680564733841876926932320129493409985129 : ℤ)*(50392320255437251633660881176922774094 : ℤ)=1 := by decide

theorem unit_021 : (535663857831296635784697078025813492358 : ℤ)*(508464070538653320952820128714109785880 : ℤ)+(680564733841876926932320129493409985129 : ℤ)*(-400205611677523857592366195269747892791 : ℤ)=1 := by decide

theorem unit_022 : (474449378653274604922996087227417989253 : ℤ)*(39721742589248131730515190325441639979 : ℤ)+(680564733841876926932320129493409985129 : ℤ)*(-27691643650275859101928927699121109734 : ℤ)=1 := by decide
end CMRecovery42Data
