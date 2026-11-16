/-
At most two roots of a quadratic congruence, in the form that matters for the
conic enumeration (Idea 2) and the box count (Idea 18).

`hp` is Euclid's lemma, the only property of primes we need; it is taken as a
hypothesis because core Lean 4 ships no `Nat.Prime`.
-/
import Init.Omega

theorem no_three_roots
    (p a b c x1 x2 x3 : Int)
    (hp : ∀ u v : Int, p ∣ u * v → p ∣ u ∨ p ∣ v)
    (ha : ¬ p ∣ a)
    (h1 : p ∣ a*x1*x1 + b*x1 + c)
    (h2 : p ∣ a*x2*x2 + b*x2 + c)
    (h3 : p ∣ a*x3*x3 + b*x3 + c)
    (h12 : ¬ p ∣ (x1 - x2)) (h13 : ¬ p ∣ (x1 - x3)) (h23 : ¬ p ∣ (x2 - x3)) :
    False := by
  -- step 1: p divides (x1-x2)*(a*(x1+x2)+b) and (x1-x3)*(a*(x1+x3)+b)
  have e12 : a*x1*x1 + b*x1 + c - (a*x2*x2 + b*x2 + c)
           = (x1 - x2)*(a*(x1+x2) + b) := by
    simp [Int.mul_add, Int.mul_sub, Int.sub_mul, Int.mul_comm, Int.mul_left_comm]
    omega
  have e13 : a*x1*x1 + b*x1 + c - (a*x3*x3 + b*x3 + c)
           = (x1 - x3)*(a*(x1+x3) + b) := by
    simp [Int.mul_add, Int.mul_sub, Int.sub_mul, Int.mul_comm, Int.mul_left_comm]
    omega
  have d12 : p ∣ (x1 - x2)*(a*(x1+x2) + b) := by
    rw [← e12]; exact Int.dvd_sub h1 h2
  have d13 : p ∣ (x1 - x3)*(a*(x1+x3) + b) := by
    rw [← e13]; exact Int.dvd_sub h1 h3
  -- step 2: drop the differences
  have s12 : p ∣ a*(x1+x2) + b := by
    rcases hp _ _ d12 with h | h
    · exact absurd h h12
    · exact h
  have s13 : p ∣ a*(x1+x3) + b := by
    rcases hp _ _ d13 with h | h
    · exact absurd h h13
    · exact h
  -- step 3: subtract to get p | a*(x2-x3)
  have e23 : (a*(x1+x2) + b) - (a*(x1+x3) + b) = a*(x2 - x3) := by
    simp [Int.mul_add, Int.mul_sub, Int.add_mul, Int.sub_mul, Int.mul_comm,
          Int.mul_left_comm, Int.mul_assoc]
    omega
  have d23 : p ∣ a*(x2 - x3) := by
    rw [← e23]; exact Int.dvd_sub s12 s13
  -- step 4: Euclid again, contradiction
  rcases hp _ _ d23 with h | h
  · exact ha h
  · exact h23 h
