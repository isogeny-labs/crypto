import ListKernels

namespace SemaevGeneralSplitAudit
open Polynomial SemaevReassociationAudit
noncomputable section
variable {K S : Type*} [Field K] [CommRing S]

def kernelSize (xs : List S) : ℕ := 2^(xs.length-1)

theorem kernelSize_pair (x y : S) : kernelSize [x,y]=2 := rfl

theorem kernelSize_cons (xs : List S) (h : 2≤xs.length) (z : S) :
    kernelSize (z::xs)=kernelSize xs*2 := by
  have he : (z::xs).length-1=(xs.length-1)+1 := by simp only [List.length_cons]; omega
  unfold kernelSize
  rw [he,pow_succ]

def joinedValue (W : WeierstrassCurve.Affine K) (f : K →+* S)
    (xs ys : List S) (hx : 2≤xs.length) (hy : 2≤ys.length) : S :=
  listValue W f (xs++ys) (by simp only [List.length_append]; omega)

theorem joinedValue_move (W : WeierstrassCurve.Affine K) (f : K →+* S)
    (xs ys : List S) (hx : 2≤xs.length) (hy : 2≤ys.length) (z : S)
    (hx' : 2≤(z::xs).length) (hy' : 2≤(z::ys).length) :
    joinedValue W f (z::xs) ys hx' hy=joinedValue W f xs (z::ys) hx hy' := by
  unfold joinedValue
  apply listValue_perm
  simpa only [List.cons_append] using
    (List.perm_middle (a:=z) (l₁:=xs) (l₂:=ys)).symm

theorem listKernel_pair (W : WeierstrassCurve.Affine K) (f : K →+* S)
    (x y : S) (h : 2≤([x,y]:List S).length) :
    listKernel W f [x,y] h=pairPoly (f W.b₂) (f W.b₄) (f W.b₆) (f W.b₈) x y := by
  have hc : List.ofFn (![x,y] : Fin 2 → S)=[x,y] := rfl
  have hh : 2≤(List.ofFn (![x,y] : Fin 2 → S)).length := by simp
  calc
    listKernel W f [x,y] h=listKernel W f (List.ofFn (![x,y] : Fin 2 → S)) hh :=
      (listKernel_congr W f hc hh h).symm
    _ = sourceKernel W 0 f ![x,y] := listKernel_ofFn W f 0 _ hh
    _ = pairPoly (f W.b₂) (f W.b₄) (f W.b₆) (f W.b₈) y x := by
      simpa using sourceKernel_zero W f ![x,y]
    _ = pairPoly (f W.b₂) (f W.b₄) (f W.b₆) (f W.b₈) x y := pairPoly_swap ..

theorem split_pair (W : WeierstrassCurve.Affine K) (f : K →+* S)
    (xs : List S) (hx : 2≤xs.length) (y z : S) (hyz : 2≤([y,z]:List S).length) :
    Polynomial.resultant (listKernel W f xs hx) (listKernel W f [y,z] hyz)
      (kernelSize xs) (kernelSize [y,z])=joinedValue W f xs [y,z] hx hyz := by
  have hz : 2≤(z::xs).length := by simp only [List.length_cons]; omega
  have hv : 3≤(y::z::xs).length := by simp only [List.length_cons]; omega
  have hj : 3≤(xs++[y,z]).length := by simp only [List.length_append,List.length_cons,List.length_nil]; omega
  have hp : (y::z::xs).Perm (xs++[y,z]) :=
    List.perm_append_comm (l₁:=[y,z]) (l₂:=xs)
  calc
    Polynomial.resultant (listKernel W f xs hx) (listKernel W f [y,z] hyz)
        (kernelSize xs) (kernelSize [y,z])=
      Polynomial.resultant (listKernel W f xs hx)
        (pairPoly (f W.b₂) (f W.b₄) (f W.b₆) (f W.b₈) z y) (kernelSize xs) 2 := by
      rw [kernelSize_pair,listKernel_pair,pairPoly_swap]
    _ = (cubicConvolution (kernelSize xs) (listKernel W f xs hx)
          (f W.b₂) (f W.b₄) (f W.b₆) (f W.b₈) z).eval y :=
      (cubicConvolution_eval ..).symm
    _ = (listKernel W f (z::xs) hz).eval y := by
      rw [listKernel_cons W f xs hx z hz]
      rfl
    _ = listValue W f (y::z::xs) hv := (listValue_cons_eval W f (z::xs) hz y hv).symm
    _ = joinedValue W f xs [y,z] hx hyz := by
      simpa only [joinedValue] using listValue_perm W f _ _ hp hv hj

/-- Every two-block split of the actual canonical source, with both
Sylvester dimensions retained. No genericity or distinct-input premise. -/
theorem all_list_splits [IsDomain S] (W : WeierstrassCurve.Affine K) (f : K →+* S)
    (ys : List S) : ∀ (xs : List S) (hx : 2≤xs.length) (hy : 2≤ys.length),
    Polynomial.resultant (listKernel W f xs hx) (listKernel W f ys hy)
      (kernelSize xs) (kernelSize ys)=joinedValue W f xs ys hx hy := by
  induction ys with
  | nil => intro xs hx hy; simp at hy
  | cons z ys ih =>
      intro xs hx hy
      cases ys with
      | nil => simp at hy
      | cons w zs =>
          cases zs with
          | nil => exact split_pair W f xs hx z w hy
          | cons t ts =>
              have ht : 2≤(w::t::ts).length := by simp
              have hx' : 2≤(z::xs).length := by simp only [List.length_cons]; omega
              have hi := ih (z::xs) hx' ht
              rw [listKernel_cons W f xs hx z hx',kernelSize_cons xs hx z] at hi
              have hb₁ : (listKernel W f xs hx).natDegree≤kernelSize xs :=
                listKernel_degree_le W f xs hx
              have hb₂ : (listKernel W f (w::t::ts) ht).natDegree≤kernelSize (w::t::ts) :=
                listKernel_degree_le W f _ ht
              have hc := cubicConvolution_interchange (listKernel W f xs hx)
                (listKernel W f (w::t::ts) ht) (kernelSize xs) (kernelSize (w::t::ts))
                hb₁ hb₂ (f W.b₂) (f W.b₄) (f W.b₆) (f W.b₈) z
              calc
                Polynomial.resultant (listKernel W f xs hx) (listKernel W f (z::w::t::ts) hy)
                    (kernelSize xs) (kernelSize (z::w::t::ts))=
                  Polynomial.resultant
                    (cubicConvolution (kernelSize xs) (listKernel W f xs hx)
                      (f W.b₂) (f W.b₄) (f W.b₆) (f W.b₈) z)
                    (listKernel W f (w::t::ts) ht) (kernelSize xs*2) (kernelSize (w::t::ts)) := by
                  rw [listKernel_cons W f (w::t::ts) ht z hy,kernelSize_cons (w::t::ts) ht z]
                  simpa only [Nat.mul_comm,kernelSize] using hc.symm
                _ = joinedValue W f (z::xs) (w::t::ts) hx' ht := hi
                _ = joinedValue W f xs (z::w::t::ts) hx hy :=
                  joinedValue_move W f xs (w::t::ts) hx ht z hx' hy

#print axioms all_list_splits
end
end SemaevGeneralSplitAudit
