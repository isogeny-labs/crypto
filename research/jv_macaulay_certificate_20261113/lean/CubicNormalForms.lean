import ActualIdeal

namespace JVMacaulayAudit
variable {R : Type*} [CommRing R] [CharP R 101]
local macro "char_ring" : tactic => `(tactic| (
  ring_nf
  all_goals reduce_mod_char!
  all_goals ring_nf
  all_goals reduce_mod_char!))

structure CubicNormal (x y h : R) : Prop where
  xxx : x^3=79*h^3
  xxy : x^2*y=36*h^3
  xyy : x*y^2=71*h^3
  yyy : y^3=79*h^3
  xxh : x^2*h=93*h^3
  xyh : x*y*h=77*h^3
  yyh : y^2*h=47*h^3
  xhh : x*h^2=35*h^3
  yhh : y*h^2=19*h^3

theorem actual_cubic_normal_forms (x y h : R)
    (h0 : f0 x y h=0) (h1 : f1 x y h=0) (h2 : f2 x y h=0) :
    CubicNormal x y h := by
  have hy : gy x y h=0 := by rw [(quadratic_certificates x y h).1,h0,h1,h2]; ring
  have hxy : gxy x y h=0 := by rw [(quadratic_certificates x y h).2.1,h1,h2]; ring
  have hx : gx x y h=0 := by rw [(quadratic_certificates x y h).2.2,h1,h2]; ring
  have hr1 : r1 x y h=0 := by simp [r1,hy,hxy,hx]
  have hr2 : r2 x y h=0 := by simp [r2,hy,hxy,hx]
  have hcy : cy x y h=0 := by rw [(cubic_certificates x y h).1,hr1,hr2]; ring
  have hcx : cx x y h=0 := by rw [(cubic_certificates x y h).2,hr1,hr2]; ring
  unfold gy at hy
  unfold gxy at hxy
  unfold gx at hx
  unfold cy at hcy
  unfold cx at hcx
  have xhh : x*h^2=35*h^3 := by linear_combination (norm:=char_ring) hcx
  have yhh : y*h^2=19*h^3 := by linear_combination (norm:=char_ring) hcy
  have xxh : x^2*h=93*h^3 := by
    linear_combination (norm:=char_ring) h*hx-96*xhh-64*yhh
  have xyh : x*y*h=77*h^3 := by
    linear_combination (norm:=char_ring) h*hxy-42*xhh-34*yhh
  have yyh : y^2*h=47*h^3 := by
    linear_combination (norm:=char_ring) h*hy-19*xhh-3*yhh
  have xxx : x^3=79*h^3 := by
    linear_combination (norm:=char_ring) x*hx-96*xxh-64*xyh-78*xhh
  have xxy : x^2*y=36*h^3 := by
    linear_combination (norm:=char_ring) y*hx-96*xyh-64*yyh-78*yhh
  have xyy : x*y^2=71*h^3 := by
    linear_combination (norm:=char_ring) x*hy-19*xxh-3*xyh-39*xhh
  have yyy : y^3=79*h^3 := by
    linear_combination (norm:=char_ring) y*hy-19*xyh-3*yyh-39*yhh
  exact ⟨xxx,xxy,xyy,yyy,xxh,xyh,yyh,xhh,yhh⟩

theorem actual_quartic_vanishing (x y h : R)
    (h0 : f0 x y h=0) (h1 : f1 x y h=0) (h2 : f2 x y h=0) : h^4=0 := by
  have hm := quartic_mem_of_source_mem (⊥ : Ideal R) x y h
    (by simpa using h0) (by simpa using h1) (by simpa using h2)
  exact hm

theorem all_degree_four_vanish (x y h : R)
    (h0 : f0 x y h=0) (h1 : f1 x y h=0) (h2 : f2 x y h=0)
    (i j k : ℕ) (hs : i+j+k=4) : x^i*y^j*h^k=0 := by
  have c := actual_cubic_normal_forms x y h h0 h1 h2
  have q := actual_quartic_vanishing x y h h0 h1 h2
  have xhhh : x*h^3=0 := by
    linear_combination (norm:=char_ring) h*c.xhh+35*q
  have yhhh : y*h^3=0 := by
    linear_combination (norm:=char_ring) h*c.yhh+19*q
  have hi : i≤4 := by omega
  interval_cases i
  · have hj : j≤4 := by omega
    interval_cases j
    · have hk : k=4 := by omega
      subst k
      simpa using q
    · have hk : k=3 := by omega
      subst k
      simpa using yhhh
    · have hk : k=2 := by omega
      subst k
      linear_combination (norm:=char_ring) h*c.yyh+47*q
    · have hk : k=1 := by omega
      subst k
      linear_combination (norm:=char_ring) h*c.yyy+79*q
    · have hk : k=0 := by omega
      subst k
      linear_combination (norm:=char_ring) y*c.yyy+79*yhhh
  · have hj : j≤3 := by omega
    interval_cases j
    · have hk : k=3 := by omega
      subst k
      simpa using xhhh
    · have hk : k=2 := by omega
      subst k
      linear_combination (norm:=char_ring) h*c.xyh+77*q
    · have hk : k=1 := by omega
      subst k
      linear_combination (norm:=char_ring) h*c.xyy+71*q
    · have hk : k=0 := by omega
      subst k
      linear_combination (norm:=char_ring) x*c.yyy+79*xhhh
  · have hj : j≤2 := by omega
    interval_cases j
    · have hk : k=2 := by omega
      subst k
      linear_combination (norm:=char_ring) h*c.xxh+93*q
    · have hk : k=1 := by omega
      subst k
      linear_combination (norm:=char_ring) h*c.xxy+36*q
    · have hk : k=0 := by omega
      subst k
      linear_combination (norm:=char_ring) y*c.xxy+36*yhhh
  · have hj : j≤1 := by omega
    interval_cases j
    · have hk : k=1 := by omega
      subst k
      linear_combination (norm:=char_ring) h*c.xxx+79*q
    · have hk : k=0 := by omega
      subst k
      linear_combination (norm:=char_ring) y*c.xxx+79*yhhh
  · have hj : j=0 := by omega
    have hk : k=0 := by omega
    subst j k
    linear_combination (norm:=char_ring) x*c.xxx+79*xhhh

end JVMacaulayAudit
