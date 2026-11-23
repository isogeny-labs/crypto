import KernelBase

namespace SemaevGeneralSplitAudit
open Polynomial
noncomputable section
variable {K S : Type*} [Field K] [CommRing S]

def listKernel (W : WeierstrassCurve.Affine K) (f : K →+* S)
    (xs : List S) (h : 2≤xs.length) : Polynomial S :=
  sourceKernel W (xs.length-2) f (fun i => xs.get (Fin.cast (Nat.sub_add_cancel h) i))

def listValue (W : WeierstrassCurve.Affine K) (f : K →+* S)
    (xs : List S) (h : 3≤xs.length) : S :=
  sourceValue W (xs.length-3) f (fun i => xs.get (Fin.cast (Nat.sub_add_cancel h) i))

theorem listKernel_congr (W : WeierstrassCurve.Affine K) (f : K →+* S)
    {xs ys : List S} (e : xs=ys) (hx : 2≤xs.length) (hy : 2≤ys.length) :
    listKernel W f xs hx=listKernel W f ys hy := by cases e; rfl

theorem listValue_congr (W : WeierstrassCurve.Affine K) (f : K →+* S)
    {xs ys : List S} (e : xs=ys) (hx : 3≤xs.length) (hy : 3≤ys.length) :
    listValue W f xs hx=listValue W f ys hy := by cases e; rfl

theorem sourceKernel_cast (W : WeierstrassCurve.Affine K) (f : K →+* S)
    {n m : ℕ} (h : n=m) (xs : Fin (n+2) → S) :
    sourceKernel W n f xs=sourceKernel W m f
      (fun i => xs (Fin.cast (congrArg (fun a => a+2) h.symm) i)) := by cases h; rfl

theorem sourceValue_cast (W : WeierstrassCurve.Affine K) (f : K →+* S)
    {n m : ℕ} (h : n=m) (xs : Fin (n+3) → S) :
    sourceValue W n f xs=sourceValue W m f
      (fun i => xs (Fin.cast (congrArg (fun a => a+3) h.symm) i)) := by cases h; rfl

theorem listKernel_ofFn (W : WeierstrassCurve.Affine K) (f : K →+* S) (n : ℕ)
    (xs : Fin (n+2) → S) (h : 2≤(List.ofFn xs).length) :
    listKernel W f (List.ofFn xs) h=sourceKernel W n f xs := by
  have hn : (List.ofFn xs).length-2=n := by simp
  unfold listKernel
  rw [sourceKernel_cast W f hn]
  congr 1
  funext i
  rw [List.get_ofFn]
  congr 1

theorem listValue_ofFn (W : WeierstrassCurve.Affine K) (f : K →+* S) (n : ℕ)
    (xs : Fin (n+3) → S) (h : 3≤(List.ofFn xs).length) :
    listValue W f (List.ofFn xs) h=sourceValue W n f xs := by
  have hn : (List.ofFn xs).length-3=n := by simp
  unfold listValue
  rw [sourceValue_cast W f hn]
  congr 1
  funext i
  rw [List.get_ofFn]
  congr 1

theorem listKernel_degree_le (W : WeierstrassCurve.Affine K) (f : K →+* S)
    (xs : List S) (h : 2≤xs.length) :
    (listKernel W f xs h).natDegree≤2^(xs.length-1) := by
  have he : xs.length-2+1=xs.length-1 := by omega
  have hb := sourceKernel_degree_le W (xs.length-2) f
    (fun i => xs.get (Fin.cast (Nat.sub_add_cancel h) i))
  simpa only [listKernel,he] using hb

theorem listKernel_cons (W : WeierstrassCurve.Affine K) (f : K →+* S)
    (xs : List S) (h : 2≤xs.length) (z : S) (hz : 2≤(z::xs).length) :
    listKernel W f (z::xs) hz=
      cubicConvolution (2^(xs.length-1)) (listKernel W f xs h)
        (f W.b₂) (f W.b₄) (f W.b₆) (f W.b₈) z := by
  induction xs using List.ofFnRec with
  | h m vs =>
      have hm : 2≤m := by simpa using h
      obtain ⟨n,hn⟩ := Nat.exists_eq_add_of_le hm
      have he : m=n+2 := by omega
      clear hn
      subst m
      have hc : List.ofFn (Fin.cases z vs)=z::List.ofFn vs := by
        simp only [List.ofFn_succ,Fin.cases_zero,Fin.cases_succ]
      have hz' : 2≤(List.ofFn (Fin.cases z vs)).length := by rw [hc]; exact hz
      rw [← listKernel_congr W f hc hz' hz,listKernel_ofFn,listKernel_ofFn]
      have hexp : n+2-1=n+1 := by omega
      simpa only [List.length_ofFn,hexp] using sourceKernel_succ W n f vs z

theorem listValue_cons_eval (W : WeierstrassCurve.Affine K) (f : K →+* S)
    (xs : List S) (h : 2≤xs.length) (z : S) (hz : 3≤(z::xs).length) :
    listValue W f (z::xs) hz=(listKernel W f xs h).eval z := by
  induction xs using List.ofFnRec with
  | h m vs =>
      have hm : 2≤m := by simpa using h
      obtain ⟨n,hn⟩ := Nat.exists_eq_add_of_le hm
      have he : m=n+2 := by omega
      clear hn
      subst m
      have hc : List.ofFn (Fin.cases z vs)=z::List.ofFn vs := by
        simp only [List.ofFn_succ,Fin.cases_zero,Fin.cases_succ]
      have hz' : 3≤(List.ofFn (Fin.cases z vs)).length := by rw [hc]; exact hz
      rw [← listValue_congr W f hc hz' hz,listValue_ofFn,listKernel_ofFn]
      exact (sourceKernel_eval W n f vs z).symm

theorem listValue_perm (W : WeierstrassCurve.Affine K) (f : K →+* S)
    (xs ys : List S) (h : xs.Perm ys) (hx : 3≤xs.length) (hy : 3≤ys.length) :
    listValue W f xs hx=listValue W f ys hy := by
  induction xs using List.ofFnRec with
  | h m vs =>
      induction ys using List.ofFnRec with
      | h n ws =>
          have hmn : m=n := by simpa only [List.length_ofFn] using h.length_eq
          subst n
          have hm : 3≤m := by simpa using hx
          obtain ⟨n,hn⟩ := Nat.exists_eq_add_of_le hm
          have he : m=n+3 := by omega
          clear hn
          subst m
          rw [listValue_ofFn,listValue_ofFn]
          exact sourceValue_ofFn_perm W n f vs ws h

#print axioms listKernel_cons
#print axioms listValue_perm
end
end SemaevGeneralSplitAudit
