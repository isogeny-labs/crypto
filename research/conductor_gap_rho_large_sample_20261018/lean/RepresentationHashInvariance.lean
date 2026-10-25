/-!
# Representation-hashing walks are conductor-blind in distribution

Context: `research/conductor_gap_cm_pairs_20261010/lean/GenericInvariance.lean`
proves that Nechaev-model generic algorithms (BSGS-type: control flow depends only
on equality answers) have identical transcripts on isomorphic groups. The sheriff
correction SHF-20261016-3caa6f notes that this does **not** cover the Pollard rho
and kangaroo ecbench runs: their next step depends on a hash of the point
*representation*, which an isomorphism of curves changes.

This file closes the gap left open there, in the only form in which it is true:
not for one fixed hash, but for the *space* of hashes.

Setting (no group axioms are needed, only a binary operation and a bijection
`φ : G → H` compatible with it; an isomorphism of groups of points on two
isogenous curves, or of a curve and an isomorphic model, is a special case):

* an `r`-adding walk is a pair `(hash, mult)`: `hash : G → Fin r` is the
  partition of the representation, `mult : Fin r → G` the multipliers;
  `step x = x * mult (hash x)`;
* its cost on a start `x` with a step budget `bound` is the least `n ≤ bound` such
  that the trajectory `x, step x, …, stepⁿ x` contains a repeated element
  (the ρ-length), or `bound + 1` if none does.

Theorems:

* `iter_transport`, `traj_transport`: the transported walk
  `(hash ∘ φ⁻¹, φ ∘ mult)` on `H`, started at `φ x`, has trajectory
  `φ` applied to the original trajectory;
* `rhoCost_transport`: it has exactly the same cost at every budget;
* `transport_left_inv`, `transport_right_inv`: transport is a bijection of the
  walk spaces `Walk G r ≃ Walk H r`.

Consequence (read outside Lean): the push-forward of the uniform distribution on
`(hash, mult)` under the bijection is uniform, so rho's cost distribution over a
uniformly random partition is the same on both groups. The endomorphism conductor,
the j-invariant, or any other non-group invariant can only enter through the one
fixed hash an implementation uses, which is what the measurement in this study
tests. Nothing here formalizes curves, isogenies, or endomorphism rings.

Checked with Lean 4.34.1, no Mathlib. `#print axioms` at the end must report no
axioms beyond `propext`, `Classical.choice`, `Quot.sound` (none expected).
-/

namespace ConductorGapRho

/-- A bijection `G ≃ H` compatible with the two binary operations. -/
structure Iso (G H : Type) (mulG : G → G → G) (mulH : H → H → H) where
  to : G → H
  inv : H → G
  left : ∀ g, inv (to g) = g
  right : ∀ h, to (inv h) = h
  map_mul : ∀ a b, to (mulG a b) = mulH (to a) (to b)

theorem Iso.injective {G H mulG mulH} (φ : Iso G H mulG mulH) {a b : G}
    (h : φ.to a = φ.to b) : a = b := by
  have := congrArg φ.inv h
  rwa [φ.left, φ.left] at this

/-- The inverse isomorphism. -/
def Iso.symm {G H mulG mulH} (φ : Iso G H mulG mulH) : Iso H G mulH mulG where
  to := φ.inv
  inv := φ.to
  left := φ.right
  right := φ.left
  map_mul := fun a b => by
    apply φ.injective
    rw [φ.right, φ.map_mul, φ.right, φ.right]

/-- An `r`-adding walk: a partition of the representation and `r` multipliers. -/
structure Walk (G : Type) (r : Nat) where
  hash : G → Fin r
  mult : Fin r → G

variable {G H : Type} {r : Nat}

/-- One step: `x ↦ x * mult (hash x)`. -/
def Walk.step (mul : G → G → G) (w : Walk G r) (x : G) : G :=
  mul x (w.mult (w.hash x))

/-- `n` steps. -/
def Walk.iter (mul : G → G → G) (w : Walk G r) (x : G) : Nat → G
  | 0 => x
  | n + 1 => w.step mul (w.iter mul x n)

/-- The trajectory `[x, step x, …, stepⁿ x]`. -/
def Walk.traj (mul : G → G → G) (w : Walk G r) (x : G) (n : Nat) : List G :=
  (List.range (n + 1)).map (w.iter mul x)

/-- Decidable membership written out, so no `BEq` instance is involved. -/
def memb [DecidableEq G] (a : G) : List G → Bool
  | [] => false
  | b :: l => if a = b then true else memb a l

/-- Whether a list has a repeated element. -/
def hasDup [DecidableEq G] : List G → Bool
  | [] => false
  | a :: l => memb a l || hasDup l

/-- Search `n = start, start+1, …` for `fuel` steps; return the first `n` whose
trajectory has a repeat, else the `n` reached when fuel runs out. -/
def costAux [DecidableEq G] (mul : G → G → G) (w : Walk G r) (x : G) :
    Nat → Nat → Nat
  | 0, n => n
  | fuel + 1, n => if hasDup (w.traj mul x n) then n else costAux mul w x fuel (n + 1)

/-- The ρ-length of the walk from `x` with step budget `bound`: the least
`n ≤ bound` with a repeat in `traj n`, or `bound + 1`. -/
def rhoCost [DecidableEq G] (mul : G → G → G) (w : Walk G r) (x : G) (bound : Nat) : Nat :=
  costAux mul w x (bound + 1) 0

/-- Transport of a walk along an isomorphism: hash the preimage, map the multipliers. -/
def Walk.transport {mulG : G → G → G} {mulH : H → H → H} (φ : Iso G H mulG mulH)
    (w : Walk G r) : Walk H r where
  hash := fun h => w.hash (φ.inv h)
  mult := fun i => φ.to (w.mult i)

section transport
variable {mulG : G → G → G} {mulH : H → H → H} (φ : Iso G H mulG mulH) (w : Walk G r)

theorem step_transport (x : G) :
    (w.transport φ).step mulH (φ.to x) = φ.to (w.step mulG x) := by
  simp [Walk.step, Walk.transport, φ.left, φ.map_mul]

theorem iter_transport (x : G) : ∀ n,
    (w.transport φ).iter mulH (φ.to x) n = φ.to (w.iter mulG x n)
  | 0 => rfl
  | n + 1 => by
    simp only [Walk.iter]
    rw [iter_transport x n, step_transport]

theorem traj_transport (x : G) (n : Nat) :
    (w.transport φ).traj mulH (φ.to x) n = (w.traj mulG x n).map φ.to := by
  unfold Walk.traj
  rw [List.map_map]
  apply List.map_congr_left
  intro i _
  exact iter_transport φ w x i

variable [DecidableEq G] [DecidableEq H]

theorem memb_map (a : G) : ∀ l : List G, memb (φ.to a) (l.map φ.to) = memb a l
  | [] => rfl
  | b :: l => by
    simp only [List.map, memb]
    by_cases h : a = b
    · subst h; simp
    · have h' : ¬ φ.to a = φ.to b := fun e => h (φ.injective e)
      simp [h, h', memb_map a l]

theorem hasDup_map : ∀ l : List G, hasDup (l.map φ.to) = hasDup l
  | [] => rfl
  | a :: l => by
    simp only [List.map, hasDup]
    rw [memb_map φ a l, hasDup_map l]

theorem costAux_transport (x : G) : ∀ fuel n,
    costAux mulH (w.transport φ) (φ.to x) fuel n = costAux mulG w x fuel n
  | 0, _ => rfl
  | fuel + 1, n => by
    simp only [costAux]
    rw [traj_transport, hasDup_map]
    split <;> simp [costAux_transport x fuel]

/-- **Main theorem.** The transported walk has the same ρ-length at every budget. -/
theorem rhoCost_transport (x : G) (bound : Nat) :
    rhoCost mulH (w.transport φ) (φ.to x) bound = rhoCost mulG w x bound :=
  costAux_transport φ w x (bound + 1) 0

end transport

section bijection
variable {mulG : G → G → G} {mulH : H → H → H} (φ : Iso G H mulG mulH)

theorem transport_left_inv (w : Walk G r) :
    (w.transport φ).transport φ.symm = w := by
  cases w with
  | mk hash mult =>
    simp only [Walk.transport, Iso.symm]
    congr 1
    · funext g; rw [φ.left]
    · funext i; rw [φ.left]

theorem transport_right_inv (w : Walk H r) :
    (w.transport φ.symm).transport φ = w := by
  cases w with
  | mk hash mult =>
    simp only [Walk.transport, Iso.symm]
    congr 1
    · funext h; rw [φ.right]
    · funext i; rw [φ.right]

/-- Transport is a bijection `Walk G r ≃ Walk H r` that preserves the ρ-length
from every start point: the two walk spaces are isomorphic *as cost-weighted
spaces*. Over a uniform choice of walk this is identity of cost distributions. -/
theorem costPreservingBijection [DecidableEq G] [DecidableEq H] :
    (∀ w : Walk G r, (w.transport φ).transport φ.symm = w) ∧
    (∀ w : Walk H r, (w.transport φ.symm).transport φ = w) ∧
    (∀ (w : Walk G r) (x : G) (bound : Nat),
      rhoCost mulH (w.transport φ) (φ.to x) bound = rhoCost mulG w x bound) :=
  ⟨transport_left_inv φ, transport_right_inv φ, fun w x b => rhoCost_transport φ w x b⟩

end bijection

end ConductorGapRho

#print axioms ConductorGapRho.rhoCost_transport
#print axioms ConductorGapRho.costPreservingBijection
#print axioms ConductorGapRho.traj_transport
