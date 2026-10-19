import Init

namespace ScalarTransport

/-- Fixed recursive natural scalar multiplication. The concrete curve/group
implementation and its relation to this operation remain a bridge obligation. -/
def natMul {A : Type} [Zero A] [Add A] : Nat → A → A
  | 0, _ => 0
  | n + 1, p => natMul n p + p

/-- A zero- and addition-preserving map transports every natural multiple.
This is the algebraic dependency of the paper's scalar-transport argument;
it does not construct an isogeny or prove its kernel degree. -/
theorem mapNatMul {A B : Type} [Zero A] [Add A] [Zero B] [Add B]
    (phi : A → B) (hzero : phi 0 = 0)
    (hadd : ∀ a b, phi (a + b) = phi a + phi b) (p : A) (n : Nat) :
    phi (natMul n p) = natMul n (phi p) := by
  induction n with
  | zero => exact hzero
  | succ n ih =>
    simp only [natMul, hadd, ih]

/-- Injectivity additionally reflects equality tests between transported
points. An actual injective restriction of the concrete isogeny is not assumed
to follow merely from point sampling. -/
theorem equalityReflected {A B : Type} (phi : A → B)
    (hinj : ∀ a b, phi a = phi b → a = b) (p q : A) :
    phi p = phi q ↔ p = q := by
  constructor
  · exact hinj p q
  · exact congrArg phi

/-- Under the explicit homomorphism and injectivity hypotheses, the same
natural scalar is an answer before and after transport. This does not prove
the full generic-algorithm distribution/cost statement or a native-code
refinement. -/
theorem answersReflected {A B : Type} [Zero A] [Add A] [Zero B] [Add B]
    (phi : A → B) (hzero : phi 0 = 0)
    (hadd : ∀ a b, phi (a + b) = phi a + phi b)
    (hinj : ∀ a b, phi a = phi b → a = b) (p q : A) (n : Nat) :
    natMul n (phi p) = phi q ↔ natMul n p = q := by
  rw [← mapNatMul phi hzero hadd p n]
  exact equalityReflected phi hinj (natMul n p) q

#print axioms ScalarTransport.mapNatMul
#print axioms ScalarTransport.equalityReflected
#print axioms ScalarTransport.answersReflected

end ScalarTransport
