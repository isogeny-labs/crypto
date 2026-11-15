import Mathlib

/-!
# Degree-bounded closures (abstract framework)

The repository's last-fall-degree engine computes, for a system `F` and a degree bound `c`, the
smallest subspace `V_{F,c}` that contains the generators of degree `≤ c` and is closed under
`g ∈ V, deg(h g) ≤ c ⇒ h g ∈ V` (Huang–Kosters–Yeo; Caminata–Gorla's Macaulay-matrix closure).
Here this is abstracted to a *shift system*: a module `M` with a filtration `F c` ("weight ≤ c"),
a family of linear shift operators `σ i` (multiplication by the variables), and one generator `g`
(the other generators of the chain system are absorbed into the quotient algebra, see the report).

`closure c` is the least submodule containing `g` and closed under every shift applied to its
elements of weight `≤ c - 1`.  The theorems are the structural "transfers" used in the report:

* `closure_mono`: monotone in the degree bound;
* `closure_le_F`: contained in the weight-`c` part when the shifts raise weight by at most one;
* `closure_transfer`: a weight-non-increasing linear map that intertwines (or trivialises) the
  shifts maps the closure into the closure of the target system.  Depth monotonicity
  (`LFD(k') ≤ LFD(k)` for `k' ≤ k`) and the no-wrap (ℤ-exponent) containment are instances;
* `lastFallDegrees_eq_refutationDegrees`: for a unit ideal the Huang–Kosters–Yeo last fall degree
  and the degree-bounded refutation degree are the same set of degrees, given the reachability
  property that `1` at degree `c` fills the whole weight-`c'` part for every `c' ≥ c`
  (proved for the chain algebra in `ChainSd.GroupAlgebra`).
-/

namespace ChainSd

universe u v w

/-- A degree-bounded shift system. -/
structure ShiftSystem (R : Type u) (M : Type v) [Semiring R] [AddCommMonoid M] [Module R M]
    (ι : Type w) where
  /-- `F c` is the submodule of elements of weight at most `c`. -/
  F : ℕ → Submodule R M
  mono : Monotone F
  /-- the shift operators (multiplication by the variables) -/
  σ : ι → M →ₗ[R] M
  /-- the generator -/
  g : M

namespace ShiftSystem

variable {R : Type u} {M : Type v} [Semiring R] [AddCommMonoid M] [Module R M] {ι : Type w}
variable (S : ShiftSystem R M ι)

/-- `V` is `c`-closed: it contains the generator and `σ i u ∈ V` whenever `u ∈ V` has weight
`≤ c - 1` (no shift is admitted at `c = 0`). -/
def IsClosed (c : ℕ) (V : Submodule R M) : Prop :=
  S.g ∈ V ∧ ∀ i, ∀ u ∈ V, 0 < c → u ∈ S.F (c - 1) → S.σ i u ∈ V

/-- The degree-`c` closure `V_{F,c}`: the least `c`-closed submodule. -/
def closure (c : ℕ) : Submodule R M :=
  sInf {V | S.IsClosed c V}

theorem isClosed_closure (c : ℕ) : S.IsClosed c (S.closure c) := by
  refine ⟨?_, ?_⟩
  · simp only [closure, Submodule.mem_sInf, Set.mem_ofPred_eq]
    intro V hV
    exact hV.1
  · intro i u hu hc hw
    simp only [closure, Submodule.mem_sInf, Set.mem_ofPred_eq] at hu ⊢
    intro V hV
    exact hV.2 i u (hu V hV) hc hw

theorem closure_le {c : ℕ} {V : Submodule R M} (h : S.IsClosed c V) : S.closure c ≤ V :=
  sInf_le h

theorem g_mem_closure (c : ℕ) : S.g ∈ S.closure c :=
  (S.isClosed_closure c).1

theorem shift_mem_closure {c : ℕ} (i : ι) {u : M} (hu : u ∈ S.closure c) (hc : 0 < c)
    (hw : u ∈ S.F (c - 1)) : S.σ i u ∈ S.closure c :=
  (S.isClosed_closure c).2 i u hu hc hw

/-- The closure is monotone in the degree bound. -/
theorem closure_mono {c c' : ℕ} (h : c ≤ c') : S.closure c ≤ S.closure c' := by
  apply S.closure_le
  refine ⟨S.g_mem_closure c', ?_⟩
  intro i u hu hc hw
  exact S.shift_mem_closure i hu (lt_of_lt_of_le hc h)
    (S.mono (Nat.sub_le_sub_right h 1) hw)

/-- If the generator has weight `≤ c` and every shift raises weight by at most one, the closure
lies in the weight-`c` part. -/
theorem closure_le_F (c : ℕ) (hg : S.g ∈ S.F c)
    (hσ : ∀ i, ∀ u ∈ S.F (c - 1), S.σ i u ∈ S.F c) : S.closure c ≤ S.F c :=
  S.closure_le ⟨hg, fun i u _ _ hw => hσ i u hw⟩

/-- An element of the closure has weight `≤ c` under the same hypotheses. -/
theorem mem_F_of_mem_closure {c : ℕ} (hg : S.g ∈ S.F c)
    (hσ : ∀ i, ∀ u ∈ S.F (c - 1), S.σ i u ∈ S.F c) {u : M} (hu : u ∈ S.closure c) :
    u ∈ S.F c :=
  S.closure_le_F c hg hσ hu

section Transfer

variable {M' : Type*} [AddCommMonoid M'] [Module R M'] {ι' : Type*}

/-- **Transfer.** Let `φ` be linear, weight-non-increasing (`φ (F c) ⊆ F' c`), and let every
shift of `S` either intertwine with some shift of `T` or be trivialised by `φ`.  If `φ g` lies
in the degree-`c` closure of `T`, then `φ` maps the degree-`c` closure of `S` into it.

Instances: (1) reduction of exponents `ℤ/2^k → ℤ/2^{k'}` (depth monotonicity), (2) the
projection from the ℤ-exponent (no-wrap) model to the chain algebra, (3) extension of the
coefficient field. -/
theorem closure_transfer (T : ShiftSystem R M' ι') (φ : M →ₗ[R] M')
    (hF : ∀ c, ∀ u ∈ S.F c, φ u ∈ T.F c)
    (hσ : ∀ i, (∃ j, ∀ u, φ (S.σ i u) = T.σ j (φ u)) ∨ (∀ u, φ (S.σ i u) = φ u))
    (c : ℕ) (hg : φ S.g ∈ T.closure c) :
    ∀ u ∈ S.closure c, φ u ∈ T.closure c := by
  have hcl : S.IsClosed c ((T.closure c).comap φ) := by
    refine ⟨hg, ?_⟩
    intro i u hu hc hw
    simp only [Submodule.mem_comap] at hu ⊢
    rcases hσ i with ⟨j, hj⟩ | htriv
    · rw [hj]
      exact T.shift_mem_closure j hu hc (hF _ u hw)
    · rw [htriv]
      exact hu
  intro u hu
  exact S.closure_le hcl hu

/-- The membership consequence used for lower-bound transfer: if `o'` is not in the target
closure, no preimage of it is in the source closure. -/
theorem not_mem_closure_of_transfer (T : ShiftSystem R M' ι') (φ : M →ₗ[R] M')
    (hF : ∀ c, ∀ u ∈ S.F c, φ u ∈ T.F c)
    (hσ : ∀ i, (∃ j, ∀ u, φ (S.σ i u) = T.σ j (φ u)) ∨ (∀ u, φ (S.σ i u) = φ u))
    (c : ℕ) (hg : φ S.g ∈ T.closure c) {o : M} (ho : φ o ∉ T.closure c) :
    o ∉ S.closure c :=
  fun h => ho (S.closure_transfer T φ hF hσ c hg o h)

end Transfer

section Degrees

/-- The degrees at which `o` (think: `1`) is derived: the degree-bounded refutation degrees. -/
def refutationDegrees (o : M) : Set ℕ := {c | o ∈ S.closure c}

/-- The degree at which the pinned degree-by-degree algorithm stops on a unit ideal: the least
degree whose closure contains `1`.  (For the unit ideal "contains a Gröbner basis" is "contains
a nonzero constant".) -/
noncomputable def solvingDegree (o : M) : ℕ := sInf (S.refutationDegrees o)

/-- Huang–Kosters–Yeo last-fall degrees for the ideal `(F) = M` (unit ideal): `c` such that every
`f` of weight `≤ w` lies in `V_{F, max c w}`. -/
def lastFallDegrees : Set ℕ := {c | ∀ f : M, ∀ w, f ∈ S.F w → f ∈ S.closure (max c w)}

/-- The last fall degree as a number. -/
noncomputable def lastFallDegree : ℕ := sInf S.lastFallDegrees

/-- **Last fall degree = refutation degree** for a unit ideal, given reachability: once `o`
is derived at degree `c`, the closure at every degree `c' ≥ c` is the whole weight-`c'` part. -/
theorem lastFallDegrees_eq_refutationDegrees (o : M) (ho : o ∈ S.F 0)
    (hreach : ∀ c, o ∈ S.closure c → ∀ c', c ≤ c' → S.F c' ≤ S.closure c') :
    S.lastFallDegrees = S.refutationDegrees o := by
  ext c
  constructor
  · intro h
    have := h o 0 ho
    simpa [refutationDegrees] using this
  · intro h f w hf
    exact hreach c h (max c w) (le_max_left _ _) (S.mono (le_max_right c w) hf)

theorem lastFallDegree_eq_solvingDegree (o : M) (ho : o ∈ S.F 0)
    (hreach : ∀ c, o ∈ S.closure c → ∀ c', c ≤ c' → S.F c' ≤ S.closure c') :
    S.lastFallDegree = S.solvingDegree o := by
  unfold lastFallDegree solvingDegree
  rw [S.lastFallDegrees_eq_refutationDegrees o ho hreach]

/-- Refutation degrees are upward closed. -/
theorem refutationDegrees_upward {o : M} {c c' : ℕ} (h : c ∈ S.refutationDegrees o)
    (hle : c ≤ c') : c' ∈ S.refutationDegrees o :=
  S.closure_mono hle h

/-- A lower bound certificate: `o ∉ closure c` gives `solvingDegree o > c` whenever the
refutation degrees are nonempty (finite solving degree). -/
theorem lt_solvingDegree_of_not_mem {o : M} {c : ℕ} (hne : (S.refutationDegrees o).Nonempty)
    (h : o ∉ S.closure c) : c < S.solvingDegree o := by
  by_contra hcon
  push Not at hcon
  have hmem : S.solvingDegree o ∈ S.refutationDegrees o := Nat.sInf_mem hne
  exact h (S.refutationDegrees_upward hmem hcon)

/-- An upper bound certificate: `o ∈ closure c` gives `solvingDegree o ≤ c`. -/
theorem solvingDegree_le_of_mem {o : M} {c : ℕ} (h : o ∈ S.closure c) :
    S.solvingDegree o ≤ c :=
  Nat.sInf_le h

end Degrees

end ShiftSystem

end ChainSd
