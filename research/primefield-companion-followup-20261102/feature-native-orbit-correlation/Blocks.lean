import Init
namespace OrbitCorrelation

def run {A : Type} : List (A → A) → A → A
  | [], x => x
  | f :: fs, x => run fs (f x)

-- Successive blocks act right-after-left; coefficient/matrix interpretation is separate.
theorem run_append {A : Type} (left right : List (A → A)) (x : A) :
    run (left ++ right) x = run right (run left x) := by
  induction left generalizing x with
  | nil => rfl
  | cons f fs ih => exact ih (f x)

#print axioms run_append
end OrbitCorrelation
