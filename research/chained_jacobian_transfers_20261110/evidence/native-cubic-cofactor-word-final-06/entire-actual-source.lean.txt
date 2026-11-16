import Std.Tactic
import Init.GrindInstances.Ring.Fin

/-! A concrete arithmetic obstruction in the original cubic representation.
The coefficients below are the schoolbook square reduced by z^3=d. The
theorems do not construct a field, a Jacobian or a polarized identification,
and assert nothing about other supplied-source models or other twists. -/
set_option autoImplicit false
set_option maxRecDepth 8192
namespace CubicNormDescent
open Lean.Grind

def norm {R : Type} [CommRing R] (d a b c : R) : R :=
  a^3 + d*b^3 + d^2*c^3 - 3*d*a*b*c

/-- The exact schoolbook coefficient convention used by the original Kf.mul. -/
def square0 {R : Type} [CommRing R] (d a b c : R) : R :=
  a*a + d*(b*c+c*b)
def square1 {R : Type} [CommRing R] (d a b c : R) : R :=
  a*b+b*a+d*c*c
def square2 {R : Type} [CommRing R] (a b c : R) : R :=
  a*c+b*b+c*a

/-- Expansion of the determinant of multiplication by a+b*z+c*z^2 in
the basis 1,z,z^2, using the relation z^3=d. -/
theorem determinant_formula {R : Type} [CommRing R] (d a b c : R) :
    a*(a*a-d*c*b)-d*c*(b*a-d*c*c)+d*b*(b*b-a*c)=norm d a b c := by
  unfold norm
  grind

theorem norm_square {R : Type} [CommRing R] (d a b c : R) :
    norm d (square0 d a b c) (square1 d a b c) (square2 a b c) =
      (norm d a b c)^2 := by
  unfold norm square0 square1 square2
  grind

theorem norm_of_constant_square {R : Type} [CommRing R]
    (d a b c D : R)
    (h0 : square0 d a b c = D) (h1 : square1 d a b c = 0)
    (h2 : square2 a b c = 0) : (norm d a b c)^2 = D^3 := by
  have h := norm_square d a b c
  rw [h0, h1, h2] at h
  unfold norm at h ⊢
  grind

/-- For a unit D, a square root of D^3 yields a base-ring square root of D. -/
theorem recover_base_square_root {R : Type} [CommRing R]
    (D invD u : R) (hinv : invD*D=1) (hu : u^2=D^3) :
    (u*invD)^2=D := by
  grind

/-- This is the already known finite residue calculation, now stated in the
actual coefficient ring used by the algebraic implication. Kernel reduction
checks only223 base residues, never the223^3 coefficient triples. -/
theorem base_nonroots :
    ∀ x : Fin 223, x^2 ≠ (12 : Fin 223) ∧ x^2 ≠ (24 : Fin 223) := by
  decide

theorem actual_constant_units :
    (93 : Fin 223)*12=1 ∧ (158 : Fin 223)*24=1 := by decide

theorem no_cubic_square_root12 (a b c : Fin 223) :
    ¬ (square0 3 a b c=12 ∧ square1 3 a b c=0 ∧ square2 a b c=0) := by
  intro ⟨h0,h1,h2⟩
  have hn := norm_of_constant_square 3 a b c 12 h0 h1 h2
  have hb := recover_base_square_root 12 93 (norm 3 a b c)
    actual_constant_units.1 hn
  exact (base_nonroots (norm 3 a b c * 93)).1 hb

theorem no_cubic_square_root24 (a b c : Fin 223) :
    ¬ (square0 3 a b c=24 ∧ square1 3 a b c=0 ∧ square2 a b c=0) := by
  intro ⟨h0,h1,h2⟩
  have hn := norm_of_constant_square 3 a b c 24 h0 h1 h2
  have hb := recover_base_square_root 24 158 (norm 3 a b c)
    actual_constant_units.2 hn
  exact (base_nonroots (norm 3 a b c * 158)).2 hb

/-- A square constant remains allowed: the obstruction does not reject all
models merely because their square root is represented in the cubic algebra. -/
theorem square_constant_control :
    square0 (3 : Fin 223) 2 0 0=4 ∧ square1 (3 : Fin 223) 2 0 0=0 ∧
    square2 (2 : Fin 223) 0 0=0 := by decide

/-- The original Kf.mul square's five unreduced/reduced natural-number
temporaries, specialized only to q223,c3. Native word-overflow and binary
execution correspondence are separate from this exact arithmetic model. -/
def originalSquare223 (a b c : Fin 223) : Fin 223 × Fin 223 × Fin 223 :=
  let t0 := (a.val*a.val)%223
  let t1 := (a.val*b.val+b.val*a.val)%223
  let t2 := (a.val*c.val+b.val*b.val+c.val*a.val)%223
  let t3 := (b.val*c.val+c.val*b.val)%223
  let t4 := (c.val*c.val)%223
  (Fin.ofNat 223 (t0+3*t3),Fin.ofNat 223 (t1+3*t4),Fin.ofNat 223 t2)

theorem original_square_coordinates (a b c : Fin 223) :
    originalSquare223 a b c =
      (square0 3 a b c,square1 3 a b c,square2 a b c) := by
  have three : (3 : Fin 223).val = 3 := rfl
  apply Prod.ext
  · apply Fin.ext
    simp [originalSquare223,square0,Fin.add_def,Fin.mul_def,Nat.add_mod,Nat.mul_mod,three]
  · apply Prod.ext
    · have reassociate : square1 (3 : Fin 223) a b c = a*b+b*a+3*(c*c) := by
        unfold square1
        grind
      rw [reassociate]
      apply Fin.ext
      simp [originalSquare223,Fin.add_def,Fin.mul_def,Nat.add_mod,Nat.mul_mod,three]
    · apply Fin.ext
      simp [originalSquare223,square2,Fin.add_def,Fin.mul_def,Nat.add_mod,Nat.mul_mod]

theorem original_square_ne12 (a b c : Fin 223) :
    originalSquare223 a b c ≠ (12,0,0) := by
  intro h
  rw [original_square_coordinates] at h
  exact no_cubic_square_root12 a b c
    ⟨congrArg Prod.fst h,congrArg (fun x => x.2.1) h,congrArg (fun x => x.2.2) h⟩

theorem original_square_ne24 (a b c : Fin 223) :
    originalSquare223 a b c ≠ (24,0,0) := by
  intro h
  rw [original_square_coordinates] at h
  exact no_cubic_square_root24 a b c
    ⟨congrArg Prod.fst h,congrArg (fun x => x.2.1) h,congrArg (fun x => x.2.2) h⟩

end CubicNormDescent
#print axioms CubicNormDescent.norm_square
#print axioms CubicNormDescent.determinant_formula
#print axioms CubicNormDescent.norm_of_constant_square
#print axioms CubicNormDescent.recover_base_square_root
#print axioms CubicNormDescent.base_nonroots
#print axioms CubicNormDescent.actual_constant_units
#print axioms CubicNormDescent.no_cubic_square_root12
#print axioms CubicNormDescent.no_cubic_square_root24
#print axioms CubicNormDescent.square_constant_control
#print axioms CubicNormDescent.original_square_coordinates
#print axioms CubicNormDescent.original_square_ne12
#print axioms CubicNormDescent.original_square_ne24

/-! The exact immutable CubicNormDescent source above is retained as a prefix.
This additive component models coefficient arithmetic only: the Rust counter
updates, constructor, compiler and actual native binary are separate. -/
namespace CubicNormWordBridge01

def wordSquare223 (a b c : UInt64) : UInt64 × UInt64 × UInt64 :=
  let t0 := a*a%223
  let t1 := (a*b+b*a)%223
  let t2 := (a*c+b*b+c*a)%223
  let t3 := (b*c+c*b)%223
  let t4 := c*c%223
  ((t0+3*t3)%223,(t1+3*t4)%223,t2)

def decode (x : UInt64 × UInt64 × UInt64) : Fin 223 × Fin 223 × Fin 223 :=
  (Fin.ofNat 223 x.1.toNat,Fin.ofNat 223 x.2.1.toNat,Fin.ofNat 223 x.2.2.toNat)

theorem canonical_product_bound (a b : Fin 223) : a.val*b.val ≤ 49284 := by
  have ha : a.val ≤ 222 := by omega
  have hb : b.val ≤ 222 := by omega
  exact Nat.mul_le_mul ha hb

theorem word_square_coordinates (a b c : Fin 223) :
    decode (wordSquare223 (UInt64.ofNat a.val) (UInt64.ofNat b.val)
      (UInt64.ofNat c.val)) = CubicNormDescent.originalSquare223 a b c := by
  have haa := canonical_product_bound a a
  have hab := canonical_product_bound a b
  have hac := canonical_product_bound a c
  have hba := canonical_product_bound b a
  have hbb := canonical_product_bound b b
  have hbc := canonical_product_bound b c
  have hca := canonical_product_bound c a
  have hcb := canonical_product_bound c b
  have hcc := canonical_product_bound c c
  have va : a.val < 2^64 := by have := a.isLt; omega
  have vb : b.val < 2^64 := by have := b.isLt; omega
  have vc : c.val < 2^64 := by have := c.isLt; omega
  have products (x y : Fin 223) : (x.val*y.val)%2^64=x.val*y.val := by
    apply Nat.mod_eq_of_lt
    have := canonical_product_bound x y
    omega
  have pairs (x y z w : Fin 223) :
      (x.val*y.val+z.val*w.val)%2^64=x.val*y.val+z.val*w.val := by
    apply Nat.mod_eq_of_lt
    have := canonical_product_bound x y
    have := canonical_product_bound z w
    omega
  have triples (x y z w u v : Fin 223) :
      (x.val*y.val+z.val*w.val+u.val*v.val)%2^64=
        x.val*y.val+z.val*w.val+u.val*v.val := by
    apply Nat.mod_eq_of_lt
    have := canonical_product_bound x y
    have := canonical_product_bound z w
    have := canonical_product_bound u v
    omega
  have scales (n : Nat) : (3*(n%223))%2^64=3*(n%223) := by
    apply Nat.mod_eq_of_lt
    have := Nat.mod_lt n (by decide : 0<223)
    omega
  have finals (m n : Nat) :
      (m%223+3*(n%223))%2^64=m%223+3*(n%223) := by
    apply Nat.mod_eq_of_lt
    have := Nat.mod_lt m (by decide : 0<223)
    have := Nat.mod_lt n (by decide : 0<223)
    omega
  apply Prod.ext
  · apply Fin.ext
    simp only [decode,wordSquare223,CubicNormDescent.originalSquare223,
      Fin.val_ofNat,UInt64.toNat_add,UInt64.toNat_mul,UInt64.toNat_mod,
      UInt64.toNat_ofNat',UInt64.toNat_ofNat,Nat.reducePow,Nat.reduceMod,
      Nat.mod_eq_of_lt va,Nat.mod_eq_of_lt vb,Nat.mod_eq_of_lt vc,
      products,pairs,triples,scales,finals,Nat.mod_mod]
  · apply Prod.ext <;> apply Fin.ext <;>
      simp only [decode,wordSquare223,CubicNormDescent.originalSquare223,
        Fin.val_ofNat,UInt64.toNat_add,UInt64.toNat_mul,UInt64.toNat_mod,
        UInt64.toNat_ofNat',UInt64.toNat_ofNat,Nat.reducePow,Nat.reduceMod,
        Nat.mod_eq_of_lt va,Nat.mod_eq_of_lt vb,Nat.mod_eq_of_lt vc,
        products,pairs,triples,scales,finals,Nat.mod_mod]

theorem word_square_ne12 (a b c : Fin 223) :
    decode (wordSquare223 (UInt64.ofNat a.val) (UInt64.ofNat b.val)
      (UInt64.ofNat c.val)) ≠ (12,0,0) := by
  rw [word_square_coordinates]
  exact CubicNormDescent.original_square_ne12 a b c

theorem word_square_ne24 (a b c : Fin 223) :
    decode (wordSquare223 (UInt64.ofNat a.val) (UInt64.ofNat b.val)
      (UInt64.ofNat c.val)) ≠ (24,0,0) := by
  rw [word_square_coordinates]
  exact CubicNormDescent.original_square_ne24 a b c

theorem word_square_control : decode (wordSquare223 2 0 0) = (4,0,0) := by decide

theorem word_square_canonical (a b c : UInt64) :
    let w := wordSquare223 a b c
    w.1.toNat<223 ∧ w.2.1.toNat<223 ∧ w.2.2.toNat<223 := by
  simp only [wordSquare223,UInt64.toNat_mod,UInt64.toNat_ofNat,Nat.reducePow,Nat.reduceMod]
  exact ⟨Nat.mod_lt _ (by decide),Nat.mod_lt _ (by decide),Nat.mod_lt _ (by decide)⟩

theorem word_square_exact_values (a b c : Fin 223) :
    let w := wordSquare223 (UInt64.ofNat a.val) (UInt64.ofNat b.val) (UInt64.ofNat c.val)
    let n := CubicNormDescent.originalSquare223 a b c
    (w.1.toNat,w.2.1.toNat,w.2.2.toNat) = (n.1.val,n.2.1.val,n.2.2.val) := by
  have hc := word_square_canonical (UInt64.ofNat a.val) (UInt64.ofNat b.val) (UInt64.ofNat c.val)
  have h := congrArg (fun x : Fin 223 × Fin 223 × Fin 223 => (x.1.val,x.2.1.val,x.2.2.val))
    (word_square_coordinates a b c)
  simpa only [decode,Fin.val_ofNat,Nat.mod_eq_of_lt hc.1,Nat.mod_eq_of_lt hc.2.1,
    Nat.mod_eq_of_lt hc.2.2] using h

theorem word_square_raw_ne12 (a b c : Fin 223) :
    wordSquare223 (UInt64.ofNat a.val) (UInt64.ofNat b.val) (UInt64.ofNat c.val) ≠ (12,0,0) := by
  intro h
  apply word_square_ne12 a b c
  rw [h]
  decide

theorem word_square_raw_ne24 (a b c : Fin 223) :
    wordSquare223 (UInt64.ofNat a.val) (UInt64.ofNat b.val) (UInt64.ofNat c.val) ≠ (24,0,0) := by
  intro h
  apply word_square_ne24 a b c
  rw [h]
  decide

theorem word_square_raw_control : wordSquare223 2 0 0 = (4,0,0) := by decide

end CubicNormWordBridge01
#print axioms CubicNormWordBridge01.canonical_product_bound
#print axioms CubicNormWordBridge01.word_square_coordinates
#print axioms CubicNormWordBridge01.word_square_ne12
#print axioms CubicNormWordBridge01.word_square_ne24
#print axioms CubicNormWordBridge01.word_square_control
#print axioms CubicNormWordBridge01.word_square_canonical
#print axioms CubicNormWordBridge01.word_square_exact_values
#print axioms CubicNormWordBridge01.word_square_raw_ne12
#print axioms CubicNormWordBridge01.word_square_raw_ne24
#print axioms CubicNormWordBridge01.word_square_raw_control

/-! Normalized unsigned-word arithmetic for the separately versioned native
cofactor. Full original Word01 is the exact prefix. This models operations,
not a Rust compiler or the original base inversion loop. -/
namespace CubicCofactorWordBridge22307
open Lean.Grind

def wordMul (a b : UInt64) : UInt64 := a*b%223
def wordSub (a b : UInt64) : UInt64 := if a>=b then a-b else a+223-b
def decodeWord (a : UInt64) : Fin 223 := Fin.ofNat 223 a.toNat
def decodeTriple (x : UInt64 × UInt64 × UInt64) : Fin 223 × Fin 223 × Fin 223 :=
  (decodeWord x.1,decodeWord x.2.1,decodeWord x.2.2)

def wordCofactor (a b c : UInt64) : UInt64 × UInt64 × UInt64 :=
  let aa := wordMul a a
  let bc := wordMul b c
  let cc := wordMul c c
  let ab := wordMul a b
  let bb := wordMul b b
  let ac := wordMul a c
  let d_bc := wordMul 3 bc
  let d_cc := wordMul 3 cc
  (wordSub aa d_bc,wordSub d_cc ab,wordSub bb ac)

theorem word_mul_value (a b : UInt64) (ha : a.toNat<223) (hb : b.toNat<223) :
    (wordMul a b).toNat=(a.toNat*b.toNat)%223 := by
  have hbound : a.toNat*b.toNat<2^64 := by
    have h := Nat.mul_le_mul (by omega : a.toNat≤222) (by omega : b.toNat≤222)
    omega
  simp only [wordMul,UInt64.toNat_mod,UInt64.toNat_mul,UInt64.toNat_ofNat,
    Nat.reducePow,Nat.reduceMod]
  rw [Nat.mod_eq_of_lt hbound]

theorem word_mul_canonical (a b : UInt64) (ha : a.toNat<223) (hb : b.toNat<223) :
    (wordMul a b).toNat<223 := by
  rw [word_mul_value a b ha hb]
  exact Nat.mod_lt _ (by decide)

theorem word_sub_value (a b : UInt64) (ha : a.toNat<223) (hb : b.toNat<223) :
    (wordSub a b).toNat=
      if b.toNat≤a.toNat then a.toNat-b.toNat else a.toNat+223-b.toNat := by
  by_cases h : b≤a
  · have hn : b.toNat≤a.toNat := UInt64.le_iff_toNat_le.mp h
    simp only [wordSub,if_pos h,if_pos hn]
    exact UInt64.toNat_sub_of_le a b h
  · have hn : ¬ b.toNat≤a.toNat := fun h' => h (UInt64.le_iff_toNat_le.mpr h')
    have hsum : (a+223).toNat=a.toNat+223 := by
      simp only [UInt64.toNat_add,UInt64.toNat_ofNat,Nat.reducePow,Nat.reduceMod]
      apply Nat.mod_eq_of_lt
      omega
    have hle : b≤a+223 := UInt64.le_iff_toNat_le.mpr (by rw [hsum]; omega)
    simp only [wordSub,if_neg h,if_neg hn]
    rw [UInt64.toNat_sub_of_le _ _ hle,hsum]

theorem word_sub_canonical (a b : UInt64) (ha : a.toNat<223) (hb : b.toNat<223) :
    (wordSub a b).toNat<223 := by
  rw [word_sub_value a b ha hb]
  split <;> omega

theorem word_mul_decode (a b : UInt64) (ha : a.toNat<223) (hb : b.toNat<223) :
    decodeWord (wordMul a b)=decodeWord a*decodeWord b := by
  apply Fin.ext
  simp only [decodeWord,Fin.val_ofNat,Fin.val_mul,word_mul_value a b ha hb,
    Nat.mod_mod,Nat.mod_eq_of_lt ha,Nat.mod_eq_of_lt hb]

theorem word_sub_decode (a b : UInt64) (ha : a.toNat<223) (hb : b.toNat<223) :
    decodeWord (wordSub a b)=decodeWord a-decodeWord b := by
  apply Fin.ext
  simp only [decodeWord,Fin.val_ofNat,Fin.val_sub,word_sub_value a b ha hb,
    Nat.mod_eq_of_lt ha,Nat.mod_eq_of_lt hb]
  split <;> omega

theorem word_cofactor_canonical (a b c : UInt64)
    (ha : a.toNat<223) (hb : b.toNat<223) (hc : c.toNat<223) :
    (wordCofactor a b c).1.toNat<223 ∧
      (wordCofactor a b c).2.1.toNat<223 ∧ (wordCofactor a b c).2.2.toNat<223 := by
  have h3 : (3 : UInt64).toNat<223 := by decide
  dsimp only [wordCofactor]
  constructor
  · exact word_sub_canonical _ _ (word_mul_canonical a a ha ha)
      (word_mul_canonical 3 _ h3 (word_mul_canonical b c hb hc))
  · constructor
    · exact word_sub_canonical _ _
        (word_mul_canonical 3 _ h3 (word_mul_canonical c c hc hc))
        (word_mul_canonical a b ha hb)
    · exact word_sub_canonical _ _ (word_mul_canonical b b hb hb)
        (word_mul_canonical a c ha hc)

theorem word_cofactor_decode (a b c : UInt64)
    (ha : a.toNat<223) (hb : b.toNat<223) (hc : c.toNat<223) :
    decodeTriple (wordCofactor a b c)=
      ((decodeWord a)^2-3*decodeWord b*decodeWord c,
       3*(decodeWord c)^2-decodeWord a*decodeWord b,
       (decodeWord b)^2-decodeWord a*decodeWord c) := by
  have h3 : (3 : UInt64).toNat<223 := by decide
  have d3 : decodeWord (3 : UInt64)=(3 : Fin 223) := by decide
  have haa := word_mul_canonical a a ha ha
  have hbc := word_mul_canonical b c hb hc
  have hcc := word_mul_canonical c c hc hc
  have hab := word_mul_canonical a b ha hb
  have hbb := word_mul_canonical b b hb hb
  have hac := word_mul_canonical a c ha hc
  have h3bc := word_mul_canonical 3 _ h3 hbc
  have h3cc := word_mul_canonical 3 _ h3 hcc
  dsimp only [wordCofactor,decodeTriple]
  apply Prod.ext
  · rw [word_sub_decode _ _ haa h3bc,word_mul_decode a a ha ha,
      word_mul_decode 3 _ h3 hbc,word_mul_decode b c hb hc,d3]
    grind
  · apply Prod.ext
    · rw [word_sub_decode _ _ h3cc hab,word_mul_decode 3 _ h3 hcc,
        word_mul_decode c c hc hc,word_mul_decode a b ha hb,d3]
      grind
    · rw [word_sub_decode _ _ hbb hac,word_mul_decode b b hb hb,
        word_mul_decode a c ha hc]
      grind

theorem word_cofactor_exact_values (a b c : UInt64)
    (ha : a.toNat<223) (hb : b.toNat<223) (hc : c.toNat<223) :
    (wordCofactor a b c).1.toNat=
        ((decodeWord a)^2-3*decodeWord b*decodeWord c).val ∧
      (wordCofactor a b c).2.1.toNat=
        (3*(decodeWord c)^2-decodeWord a*decodeWord b).val ∧
      (wordCofactor a b c).2.2.toNat=
        ((decodeWord b)^2-decodeWord a*decodeWord c).val := by
  have h := word_cofactor_decode a b c ha hb hc
  have hw := word_cofactor_canonical a b c ha hb hc
  have h0 := congrArg (fun x : Fin 223 × Fin 223 × Fin 223 => x.1.val) h
  have h1 := congrArg (fun x : Fin 223 × Fin 223 × Fin 223 => x.2.1.val) h
  have h2 := congrArg (fun x : Fin 223 × Fin 223 × Fin 223 => x.2.2.val) h
  simp only [decodeTriple,decodeWord,Fin.val_ofNat,Nat.mod_eq_of_lt hw.1] at h0
  simp only [decodeTriple,decodeWord,Fin.val_ofNat,Nat.mod_eq_of_lt hw.2.1] at h1
  simp only [decodeTriple,decodeWord,Fin.val_ofNat,Nat.mod_eq_of_lt hw.2.2] at h2
  exact ⟨h0,h1,h2⟩

theorem canonical_word_controls :
    wordCofactor 1 1 1=(221,2,0) ∧ wordCofactor 0 1 1=(220,3,1) := by decide

/-- Without normalization a native word product may wrap before reduction. -/
theorem noncanonical_overflow_control :
    wordMul (UInt64.ofNat (2^63)) 2=0 ∧
      decodeWord (wordMul (UInt64.ofNat (2^63)) 2)≠
        decodeWord (UInt64.ofNat (2^63))*decodeWord 2 := by decide

end CubicCofactorWordBridge22307
#print axioms CubicCofactorWordBridge22307.word_mul_value
#print axioms CubicCofactorWordBridge22307.word_mul_canonical
#print axioms CubicCofactorWordBridge22307.word_sub_value
#print axioms CubicCofactorWordBridge22307.word_sub_canonical
#print axioms CubicCofactorWordBridge22307.word_mul_decode
#print axioms CubicCofactorWordBridge22307.word_sub_decode
#print axioms CubicCofactorWordBridge22307.word_cofactor_canonical
#print axioms CubicCofactorWordBridge22307.word_cofactor_decode
#print axioms CubicCofactorWordBridge22307.word_cofactor_exact_values
#print axioms CubicCofactorWordBridge22307.canonical_word_controls
#print axioms CubicCofactorWordBridge22307.noncanonical_overflow_control
