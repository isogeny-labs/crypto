# Two statements the index-calculus programme keeps using without writing them down (L6, L7)

**Status.** Proposed statements with proofs or proof sketches, 2026-10-18.
**Lean (2026-10-18):** the core of both lemmas is machine-checked in
`lean/ICLemmas.lean` (core Lean 4 v4.34.1, proof gate PASS, standard axioms
only). Lemma 1's independence statement (`lemma1_no_dependency`) and Lemma 2's
inequality (`lemma2_relation_phase`, `lemma2_budget`) are proved; the bridges to
Boolean degree, to `F_{2^n}` via Mathlib, and the `(Ne/D)^D` estimate are open
(`lean/README.md`, proof map).
Nothing here is measured; the evidence cited is the committed data the
statements explain. They are offered for the FFD defensive programme
(`research/notes/index-calculus/RESEARCH_FALL_DEGREE_BOUNDS.md`), which
currently records "nothing beyond `d_ff ≥ 2` is proved for `m ≥ 3`".

## Lemma 1 (shape closure at `m ≥ 3`, the chained presentation)

**Setting.** `K = F_{2^n}`, `V ⊆ K` an `F₂`-subspace with `V ≠ {0}`, and
the chained Weil-descended system of `build_decomposition_system` for
`m ≥ 3`: summand coordinates `x_i ∈ V` (ℓ Boolean unknowns each) and
intermediate points `u_j ∈ K` (n Boolean unknowns each), with the links
`S₃(x₁, x₂, u₁) = 0`, `S₃(u_j, x_{j+2}, u_{j+1}) = 0`, `S₃(u_{m−2}, x_m, x_R) = 0`,
`S₃(a, c, d) = (a+c)²d² + ac·d + (ac)² + b`.

**Statement.** The `n` Boolean equations of the first link have leading
(degree-3) forms equal to the `n` coordinate functions of the trilinear map
`V × V × K → K`, `(x₁, x₂, u₁) ↦ x₁x₂u₁`, and these `n` forms are linearly
independent over `F₂` for every `V ≠ {0}`. The same holds for every
interior link with `(u_j, x_{j+2}, u_{j+1})`. Hence no choice of `V` creates
a linear dependency among the leading forms of the chained system, and no
first fall below degree 3 can come from the shape of `V`.

**Proof.** The Weil restriction of `S₃` to coordinates is the sum of the
restrictions of its monomials; `(a+c)²d²` and `(ac)²` are squares, and
squaring is `F₂`-linear, so their Boolean degree is at most 2; `ac·d` is
trilinear in the three blocks and is the only degree-3 part. A linear
dependency among the `n` coordinate forms of `(x₁, x₂, u) ↦ x₁x₂u` is a
nonzero `F₂`-linear functional `λ : K → F₂` with `λ(x₁x₂u) = 0` for all
`x₁, x₂ ∈ V`, `u ∈ K`. Fix any `x₁ = x₂ = v ≠ 0` in `V`; then `λ(v²u) = 0`
for all `u ∈ K`, and `u ↦ v²u` is a bijection of `K`, so `λ = 0`. ∎

**Remark (the eliminated presentation).** For the eliminated `S₄`
(`m = 3`, intermediate removed) the leading forms are
`Σ_e c_e(x_R) · (coordinate forms of x₁^{a}x₂^{b}x₃^{c})` over the top
monomials, and the same argument needs the span of
`Σ_mon c_mon · V^{(a)}V^{(b)}V^{(c)}` to be `K`. For a progression base
each `V^{(a)}V^{(b)}V^{(c)}` has dimension `≤ 3ℓ − 2 < n`, so the claim rests
on the generic multipliers `c_mon(x_R)` rotating these subspaces apart.
That is the content of the shape search's G1 kill (every shape equals random
at `m = 3`, `(n, ℓ) ∈ {(13,4), (17,5), (19,6)}`), and it is stated here as a
conjecture, not a lemma: *for a generic `x_R`, the leading forms of the
eliminated `S₄` system are linearly independent for every `V`.* The
exhaustive isogeny-class check at `n ≤ 13` (`RESEARCH_ISOGENY_CLASS_SEARCH.md`)
is consistent with it.

**What the lemma does not say.** It bounds the first fall from the shape of
`V` only. The measured closure degree is 4 at `m·ℓ ≤ 18` and above 4 at
`m·ℓ = 21` for random `V`; those falls come from the interaction of the
degree-2 parts with the trilinear part, which the lemma does not control.
The coset-indicator pilot (`RESEARCH_IC_LEADS_20261018.md` §4) shows the
parity route is not where the closure finds them either.

**Corollary (what L1 escapes).** In a target-free system (`S₄` with every
variable in `V`, or a chain whose last link is `S₃(u, x_m, x_{m+1})` with
`x_{m+1} ∈ V`), the leading forms of the last link are coordinates of
`u·x_m·x_{m+1}` on `K × V × V`, still independent by the same argument; but
for the eliminated target-free `S₄` all multipliers are constants in `F₂`
and the leading forms live in `V^{(a)}V^{(b)}V^{(c)}V^{(d)}`, of dimension
`≤ 4ℓ − 3` for a progression base. For `4ℓ − 3 < n` at least `n − 4ℓ + 3`
dependencies exist at the lowest degree. That is the only place in this
family where the shape of `V` is allowed to matter, and it is why L1 is
listed.

*Measured 2026-10-18* (`research/target_free_s4_20261018/RESULTS.md`): the
dependencies above live at the field level. The Boolean top forms of the
descended eliminated `S₄` (degree 7) have full rank on the progression base
at every ℓ = 4 cell. The target-free system solves one F4 degree *above*
the target system. The corollary is true and does not bound the solving
degree.

## Lemma 2 (the admissible Macaulay degree)

**Setting.** Index calculus on `E(F_{2^n})` with a factor base of `2^ℓ`
points folded by signed Frobenius into `2^ℓ/2n` columns, decompositions
into `m` summands, and an algebraic oracle that answers one decomposition
by a Macaulay-type computation at degree `D` on `N` Boolean unknowns.
Matched reference: Pollard rho with the same automorphisms,
`T_ρ = √(π r / (8n))` group operations, `r = #E/cofactor`.

**Statement.** Write `R = 2^ℓ/(2n)` for the relations needed,
`A = max(1, m!·2^{n−mℓ})` for the attempts per relation, and
`B = T_ρ / (R·A)` for the budget per oracle call. If one oracle call costs at
least `C(N, ≤D)^w` operations for some `w ≥ 1`, then the relation phase
alone exceeds the matched rho unless

```
    C(N, ≤D)^w  ≤  B  =  √(π r/(8n)) · 2n · 2^{−ℓ} / max(1, m!·2^{n−mℓ}).
```

At `ℓ = n/m` exactly, `B ≈ 2^{n(1/2 − 1/m)} · 2n / (m!·√(8n/π))`, and with
`N ≈ n` the largest admissible `D` satisfies `w·D·log₂(N e/D) ≲ n(1/2 − 1/m)`,
so `D_adm ≈ n(1/2 − 1/m)/(w·log₂(N e/D_adm))`: it grows like `n/log n`.

**Proof.** The relation phase makes at least `R·A` oracle calls, each costing
at least `C(N, ≤D)^w`; `C(N, ≤D) ≤ (N e/D)^D` gives the logarithmic form. ∎

**Corollaries.** (i) The algebraic route beats matched rho at some `n` only
if its solving degree is `o(n / log n)` in `n`; a solving degree that grows
linearly in `m·ℓ ≈ n` with slope `s` fails for every `n` once
`s > 1/(w·log₂(N e/D))`, which at `n = 131` is about `0.03` for `w = 2.2`.
(ii) `dreg_score slope` prints the table for `w = 1` (the repository's
one-touch lower bound) on the chained `N = m·ℓ + (m−2)n` and on `N = m·ℓ`;
at `n = 131`, `D_adm = 3, 4, 5` for `m = 3, 4, 5` (chained, `w = 1`) and
`2` for `m ≥ 4` under `w = 2.2` with `N = m·ℓ`. (iii) The inequality counts
the relation phase only; linear algebra (`≈ R²·weight`) and the descent are
extra, so it is a necessary condition, never a sufficient one.

**Hypotheses to keep visible.** One touch per column is a lower bound on any
elimination that reads its matrix; the fold `2n` assumes a Frobenius-stable
base, which prime `n` with `ord_n(2) = n − 1` does not provide (then the fold
is 2 and `B` shrinks by `n`); `m!` attempts assume unordered summands;
`T_ρ` is the one-target matched reference of AGENTS.md §5a, not a
precomputation rho.
