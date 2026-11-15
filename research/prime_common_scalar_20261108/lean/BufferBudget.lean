module
public import FiniteRecovery
public section
namespace BufferBudget
open NodeMultiplicity HermiteError

/-- Weighted geometric count used when actual levels are listed in reverse. -/
@[expose] def weighted (r : ℤ) (u : ℕ) : ℤ :=
  ∑ j ∈ Finset.range u, ((j : ℤ)+1)*r^j

theorem weightedIdentity (r : ℤ) (u : ℕ) :
    (r-1)^2*weighted r u=((u : ℤ)*(r-1)-1)*r^u+1 := by
  induction u with
  | zero => simp [weighted]
  | succ u ih =>
    have hs : weighted r (u+1)=weighted r u+((u : ℤ)+1)*r^u := by
      simp [weighted, Finset.sum_range_succ]
    rw [hs, mul_add, ih, pow_succ]
    push_cast
    ring

theorem binaryWeighted (u : ℕ) :
    weighted 2 u=((u : ℤ)-1)*2^u+1 := by
  simpa using weightedIdentity 2 u

theorem quaternaryWeighted (u : ℕ) :
    9*weighted 4 u=(3*(u : ℤ)-1)*4^u+1 := by
  convert weightedIdentity 4 u using 1 <;> ring

/-- One materialized quotient vector per actual valuation level, in bits.
Under 0<u<=a the existing actualValuationCard theorem supplies each count.
This is a storage-accounting definition, not a runtime measurement. -/
@[expose] def levelBits (p a M u : ℕ) : ℕ :=
  ∑ i ∈ Finset.range u,
    2^(a-i)*(sourceCeiling M u*(u-i))*
      (p+8*sourceCeiling M u*2^(a-i)+8*M*(a+1))

/-- The actual odd-node level alone forces this many represented bits when
all prescribed quotient vectors are explicitly materialized. -/
theorem oddLevelBitsLower (p a M u : ℕ) (hu : 0<u) :
    8*(sourceCeiling M u)^2*u*(2^a)^2 ≤ levelBits p a M u := by
  have hb := Finset.single_le_sum
    (f:=fun i => 2^(a-i)*(sourceCeiling M u*(u-i))*
      (p+8*sourceCeiling M u*2^(a-i)+8*M*(a+1)))
    (fun i (_ : i∈Finset.range u) => Nat.zero_le _) (by simpa using hu : 0∈Finset.range u)
  simp only [Nat.sub_zero] at hb
  unfold levelBits
  nlinarith [Nat.zero_le (2^a*(sourceCeiling M u*u)*p),
    Nat.zero_le (2^a*(sourceCeiling M u*u)*(8*M*(a+1)))]


@[expose] def actualBufferBits (p a M u : ℕ) : ℕ :=
  ∑ t : Fin (2^(a+1)), HermiteUniqueness.sourceOrder a M u t*childPrecision p a M u t

/-- Exact actual-node accounting, with no assumed distribution of levels. -/
theorem actualBufferBitsEq (p a M u : ℕ) (hua : u≤a) :
    actualBufferBits p a M u=levelBits p a M u := by
  classical
  have hw (i : ℕ) (hi : i<u) :
      2^(a-i)*(sourceCeiling M u*(u-i))*(p+8*sourceCeiling M u*2^(a-i)+8*M*(a+1)) =
      ∑ t : Fin (2^(a+1)), if gridNode a t≠0 ∧ padicValInt 2 (gridNode a t)=i
        then sourceCeiling M u*(u-i)*(p+8*sourceCeiling M u*2^(a-i)+8*M*(a+1)) else 0 := by
    rw [←actualValuationCard a i (by omega), Fintype.card_subtype]
    simp [Finset.sum_ite,Finset.sum_const,nsmul_eq_mul,mul_assoc]
  unfold actualBufferBits levelBits
  have he := Finset.sum_congr rfl (fun i (hi : i∈Finset.range u) => hw i (Finset.mem_range.mp hi))
  rw [he,Finset.sum_comm]
  apply Finset.sum_congr rfl
  intro t ht
  by_cases hz : gridNode a t=0
  · simp [HermiteUniqueness.sourceOrder,hz]
  · simp only [hz, ne_eq, not_false_eq_true, true_and]
    have hp (i : ℕ) : (padicValInt 2 (gridNode a t)=i) ↔ i=padicValInt 2 (gridNode a t) := eq_comm
    simp_rw [hp]
    rw [Finset.sum_ite_eq']
    by_cases hv : padicValInt 2 (gridNode a t)<u
    · simp [Finset.mem_range,hv,HermiteUniqueness.sourceOrder,hz,childPrecision,nodeDensity]
    · have hs : u-padicValInt 2 (gridNode a t)=0 := Nat.sub_eq_zero_of_le (by omega)
      simp [Finset.mem_range,hv,HermiteUniqueness.sourceOrder,hz,hs]

theorem actualBufferBitsLower (p a M u : ℕ) (hu : 0<u) (hua : u≤a) :
    8*(sourceCeiling M u)^2*u*(2^a)^2 ≤ actualBufferBits p a M u := by
  rw [actualBufferBitsEq p a M u hua]
  exact oddLevelBitsLower p a M u hu


/-- Charged storage remains quadratic in real-port count after multiplying by
u; this accounts for prescribed dense buffers only. -/
theorem actualBufferBitsPortLower (p a M u : ℕ) (hu : 0<u) (hua : u≤a) :
    72*M^2*(2^a)^2 ≤ actualBufferBits p a M u*u := by
  have hc := sourceCeilingBudget M u hu
  have hs : (3*M)^2 ≤ (sourceCeiling M u*u)^2 := by gcongr
  calc
    72*M^2*(2^a)^2=(3*M)^2*(8*(2^a)^2) := by ring
    _ ≤ (sourceCeiling M u*u)^2*(8*(2^a)^2) := Nat.mul_le_mul_right _ hs
    _ = (8*(sourceCeiling M u)^2*u*(2^a)^2)*u := by ring
    _ ≤ actualBufferBits p a M u*u := Nat.mul_le_mul_right u (actualBufferBitsLower p a M u hu hua)

end BufferBudget
#print axioms BufferBudget.weightedIdentity
#print axioms BufferBudget.actualBufferBitsEq
#print axioms BufferBudget.actualBufferBitsLower
#print axioms BufferBudget.actualBufferBitsPortLower
