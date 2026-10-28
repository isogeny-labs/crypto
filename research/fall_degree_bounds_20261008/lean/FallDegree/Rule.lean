/-!
# Fall-degree bounds: the scoring rules as Lean functions

The registered rules of `PREREGISTRATION.md` that are integer statements:

- **P3a** (E3): `d_F ≤ 5` on every draw with `n ≤ 32`.
- **P3b** (E3): between `n = 18` and `n = 32` the per-`n` median `d_F` steps
  from 4 to 5 at most once (i.e. the median sequence, in `n` order, is
  non-decreasing and changes value at most once).
- **P4a** (E4): `d_F ≤ D*` on every draw; **P4b**: `d_F < D*` on at least
  half the draws.
- **Amendment 4/6 readings** (E6, exploratory): "bounded at toy size" if
  every unsatisfiable draw has `d_F ≤ 4` and every satisfiable draw has
  `d_last ≤ 4`; "Semaev-specific" (Part D′) if every control shape has
  median `d_last ≥ 5`.
- **Amendment 9 reading** (E8, exploratory): per ladder at fixed `(n, ℓ)`, with the
  median `d_F` of each `m` (E7's `m = 6` draws join the `(8, 2)` ladder): "flat" if all
  medians `≤ 4`, non-decreasing in `m`, and last − first `≤ 1`; "grows" if some median
  `≥ 5` or last − first `≥ 2`; otherwise "mixed".
- **Amendment 10 readings** (E9, E10, exploratory): E9 is Amendment 7's rule on the
  large-host cells ("bounded" if every draw `≤ 4`; "grows" if some cell median `≥ 5`).
  E10, per `m` along `mℓ = n` (the m = 5 point is E9's (15, 3) cell): "flat" if every
  cell median `≤ 4`, "grows" if some cell median `≥ 5`, otherwise "mixed". Cross-`m`
  at fixed `N`: "rises" if the cell medians strictly increase with `m`.
- **Amendment 7/8 reading** (E7, exploratory): "bounded through m = 6" if
  every E7 draw has `d_F ≤ 4`; "grows" if some E7 cell has median `d_F ≥ 5`.

The LOG/LIN model comparison of E3 uses logarithms and is computed in Rust,
not here. A censored value (`none`) is "above the cap", recorded as the cap
plus one, so a censored draw can never satisfy a `≤` bound it might break.
-/

namespace FallDegree

/-- One measured draw: `kind` 0 = E3 (m = 2), 1 = E4/extension/Part A/B
(unsat, ladder), 2 = Part C (sat), 3 = Part D′ control, 4 = E7 (unsat,
`m ≥ 5`, new draws), 5 = E8 (unsat, `d_F` against `m`), 6 = E9 (unsat,
large-host tier `N = 58, 60`), 7 = E10 (unsat, balanced diagonal `mℓ = n`). `value` is `d_F`
or `d_last`; `none` = censored above `cap`. `dstar` is the ladder's plain
`D*` where recorded (a lower bound when `dstarAtLeast`). -/
structure Row where
  kind : Nat
  m : Nat
  n : Nat
  ell : Nat
  shape : Nat     -- grouping key within a kind (cell or n)
  value : Option Nat
  cap : Nat
  dstar : Option Nat
  dstarAtLeast : Bool
  deriving Repr

def Row.v (r : Row) : Nat := r.value.getD (r.cap + 1)

/-- insertion sort: structural, so the kernel can evaluate it -/
def ins (x : Nat) : List Nat → List Nat
  | [] => [x]
  | y :: t => if x ≤ y then x :: y :: t else y :: ins x t

def isort : List Nat → List Nat
  | [] => []
  | x :: t => ins x (isort t)

/-- lower median of a list of naturals (the ⌈k/2⌉-th smallest) -/
def lowerMedian (xs : List Nat) : Nat :=
  let s := isort xs
  s.getD ((s.length + 1) / 2 - 1) 0

def keys (rs : List Row) : List Nat := (rs.map (·.shape)).eraseDups

def medianOf (rs : List Row) (k : Nat) : Nat :=
  lowerMedian ((rs.filter (·.shape == k)).map (·.v))

def e3 (rs : List Row) : List Row := rs.filter (·.kind == 0)

def p3a (rs : List Row) : Bool := (e3 rs).all fun r => decide (r.n ≤ 32 → r.v ≤ 5)

/-- medians in increasing `n`, for 18 ≤ n ≤ 32 -/
def e3Medians (rs : List Row) : List Nat :=
  let sel := (e3 rs).filter fun r => decide (18 ≤ r.n ∧ r.n ≤ 32)
  (isort (keys sel)).map (medianOf sel)

def changes : List Nat → Nat
  | a :: b :: t => (if a == b then 0 else 1) + changes (b :: t)
  | _ => 0

def nondecreasing : List Nat → Bool
  | a :: b :: t => decide (a ≤ b) && nondecreasing (b :: t)
  | _ => true

def p3b (rs : List Row) : Bool :=
  let ms := e3Medians rs
  nondecreasing ms && decide (changes ms ≤ 1) && ms.all fun x => decide (x ≤ 5)

def e4 (rs : List Row) : List Row := rs.filter fun r => r.kind == 1 && r.m == 3 && r.n ≤ 7

def p4a (rs : List Row) : Bool :=
  (e4 rs).all fun r => match r.dstar with
    | some d => decide (r.v ≤ d)
    | none => false

def p4b (rs : List Row) : Bool :=
  let strict := ((e4 rs).filter fun r => match r.dstar with
    | some d => decide (r.v < d)
    | none => false).length
  decide (2 * strict ≥ (e4 rs).length)

def boundedAtToySize (rs : List Row) : Bool :=
  (rs.filter (·.kind == 1)).all (fun r => decide (r.v ≤ 4)) &&
  (rs.filter (·.kind == 2)).all (fun r => decide (r.v ≤ 4))

def semaevSpecific (rs : List Row) : Bool :=
  let c := rs.filter (·.kind == 3)
  !c.isEmpty && (keys c).all fun k => decide (5 ≤ medianOf c k)

def e7 (rs : List Row) : List Row := rs.filter (·.kind == 4)

def e7Bounded (rs : List Row) : Bool :=
  !(e7 rs).isEmpty && (e7 rs).all fun r => decide (r.v ≤ 4)

def e7Grows (rs : List Row) : Bool :=
  (keys (e7 rs)).any fun k => decide (5 ≤ medianOf (e7 rs) k)

/-- the E8 ladder at `(n, ℓ)`: E8 draws, plus E7's draws of that shape -/
def ladderRows (rs : List Row) (n ell : Nat) : List Row :=
  rs.filter fun r => (r.kind == 5 || r.kind == 4) && r.n == n && r.ell == ell

def ladderMedians (rs : List Row) (n ell : Nat) : List Nat :=
  let l := ladderRows rs n ell
  (isort (l.map (·.m)).eraseDups).map fun m =>
    lowerMedian ((l.filter (·.m == m)).map (·.v))

def spread (ms : List Nat) : Nat := ms.getLast?.getD 0 - ms.head?.getD 0

def ladderFlat (rs : List Row) (n ell : Nat) : Bool :=
  let ms := ladderMedians rs n ell
  !ms.isEmpty && ms.all (fun x => decide (x ≤ 4)) && nondecreasing ms && decide (spread ms ≤ 1)

def ladderGrows (rs : List Row) (n ell : Nat) : Bool :=
  let ms := ladderMedians rs n ell
  ms.any (fun x => decide (5 ≤ x)) || decide (spread ms ≥ 2)

def e9 (rs : List Row) : List Row := rs.filter (·.kind == 6)

def e9Bounded (rs : List Row) : Bool :=
  !(e9 rs).isEmpty && (e9 rs).all fun r => decide (r.v ≤ 4)

def e9Grows (rs : List Row) : Bool :=
  (keys (e9 rs)).any fun k => decide (5 ≤ medianOf (e9 rs) k)

/-- the E10 diagonal of one `m`: E10 draws with `m ℓ = n`, plus E9's draws that lie on it -/
def diagRows (rs : List Row) (m : Nat) : List Row :=
  rs.filter fun r => (r.kind == 7 || r.kind == 6) && r.m == m && r.m * r.ell == r.n

def diagMedians (rs : List Row) (m : Nat) : List Nat :=
  let d := diagRows rs m
  (isort (d.map (·.n)).eraseDups).map fun n =>
    lowerMedian ((d.filter (·.n == n)).map (·.v))

def diagFlat (rs : List Row) (m : Nat) : Bool :=
  let ms := diagMedians rs m
  !ms.isEmpty && ms.all fun x => decide (x ≤ 4)

def diagGrows (rs : List Row) (m : Nat) : Bool :=
  (diagMedians rs m).any fun x => decide (5 ≤ x)

/-- median `d_F` of the E9/E10 cell `(m, n, ℓ)`; the cap + 1 if the cell has no draw -/
def cellMedian (rs : List Row) (m n ell : Nat) : Nat :=
  lowerMedian ((rs.filter fun r => (r.kind == 7 || r.kind == 6) && r.m == m && r.n == n && r.ell == ell).map (·.v))

def strictlyIncreasing : List Nat → Bool
  | a :: b :: t => decide (a < b) && strictlyIncreasing (b :: t)
  | _ => true

structure Report where
  p3a : Bool
  p3b : Bool
  p4a : Bool
  p4b : Bool
  bounded : Bool
  semaevSpecific : Bool
  e7Bounded : Bool
  e7Grows : Bool
  e8Flat82 : Bool
  e8Grows82 : Bool
  e8Flat62 : Bool
  e8Grows62 : Bool
  e9Bounded : Bool
  e9Grows : Bool
  e10Flat3 : Bool
  e10Grows3 : Bool
  e10Flat4 : Bool
  e10Grows4 : Bool
  e10Flat5 : Bool
  e10Grows5 : Bool
  e10Flat6 : Bool
  e10Grows6 : Bool
  e10RisesN48 : Bool
  e10RisesN60 : Bool
  deriving DecidableEq, Repr

def report (rs : List Row) : Report :=
  { p3a := p3a rs, p3b := p3b rs, p4a := p4a rs, p4b := p4b rs,
    bounded := boundedAtToySize rs, semaevSpecific := semaevSpecific rs,
    e7Bounded := e7Bounded rs, e7Grows := e7Grows rs,
    e8Flat82 := ladderFlat rs 8 2, e8Grows82 := ladderGrows rs 8 2,
    e8Flat62 := ladderFlat rs 6 2, e8Grows62 := ladderGrows rs 6 2,
    e9Bounded := e9Bounded rs, e9Grows := e9Grows rs,
    e10Flat3 := diagFlat rs 3, e10Grows3 := diagGrows rs 3,
    e10Flat4 := diagFlat rs 4, e10Grows4 := diagGrows rs 4,
    e10Flat5 := diagFlat rs 5, e10Grows5 := diagGrows rs 5,
    e10Flat6 := diagFlat rs 6, e10Grows6 := diagGrows rs 6,
    e10RisesN48 := strictlyIncreasing [cellMedian rs 3 24 8, cellMedian rs 4 16 4],
    e10RisesN60 := strictlyIncreasing [cellMedian rs 4 20 5, cellMedian rs 5 15 3, cellMedian rs 6 12 2] }

-- sanity on hand-made rows
private def r0 (n v : Nat) : Row := { kind := 0, m := 2, n, ell := n / 2, shape := n, value := some v, cap := 6, dstar := none, dstarAtLeast := false }
example : p3b [r0 18 4, r0 20 4, r0 22 5, r0 24 5] = true := by decide
example : p3b [r0 18 4, r0 20 5, r0 22 4] = false := by decide
example : p3a [r0 30 6] = false := by decide
example : lowerMedian [5, 7, 6, 5] = 5 := by decide

end FallDegree
