#set page(paper: "a4", margin: (x: 1.5cm, y: 1.4cm), numbering: "1 / 1")
#set text(size: 8.6pt)
#set par(justify: true, leading: 0.5em)
#set heading(numbering: none)
#show heading.where(level: 1): set text(size: 12pt)
#show heading.where(level: 2): set text(size: 9.8pt)
#show heading.where(level: 3): set text(size: 9pt)
#show heading: set block(above: 0.8em, below: 0.4em)
#show table: set text(size: 7.2pt)
#show raw: set text(size: 7.6pt)
#show link: set text(fill: rgb("#2a78d6"))

= The Weil-restriction isogeny census
<the-weil-restriction-isogeny-census>
Which curves over `F_{q^n}` an `F_q`-rational `(ℓ, …, ℓ)`-isogeny kernel
in the Weil restriction reaches. Study
`research/weil_restriction_isogeny_census_20261018/`; protocol
`PROTOCOL.md`, frozen at `095c45c8a` with Amendment 1 at `d5bcdc081`
before the first run; tool `examples/weil_restriction_isogeny_census.rs`
(std-only Rust, 8 unit tests); recorded run
`results/run-20261018T1605Z/` (Parts G and H, checks 1–9 pass). Second
round of the Teske-variant search; round one is
`research/teske_variants_field_types_20261018/` (PR \#2170).

Labels: \[T\] proved or quoted from a source; \[E\] exact enumeration in
the recorded run; \[H\] heuristic; \[O\] open. No cost is measured here;
`Õ(q)`, `q^{3/2}`, `q^{4/3}` are exponents.

== Answer
<answer>
The question was whether Weil restriction reaches curves that the known
covering attacks do not. It does, and the reach can be written down
exactly.

- #strong[The association.] For `E/F_{q^n}` and a prime `ℓ ≠ p`, an
  `F_q`-rational maximal isotropic subgroup `Λ ⊂ W[ℓ]` of the Weil
  restriction `W = Res E` that meets no conjugate factor `E^{σ^i}[ℓ]` (a
  #emph[Tian kernel]) gives an `F_q`-rational `(ℓ, …, ℓ)`-isogeny
  `W → W/Λ` to a principally polarised abelian variety. For `n = 3` Tian
  (J. Cryptology 2023) turns `W/Λ` into the Jacobian of a genus-3 curve
  in `Õ(ℓ^3)` and solves the DLP there in `Õ(q)` (Diem–Thomé), against
  `q^{3/2}` for rho and `Õ(q^{4/3})` for Gaudry’s decomposition; when
  `#E` is prime the curve is a plane quartic. Tian #emph[assumes] `Λ`
  and gives one example (`q = 523`, `ℓ = 3`, prime order); no source
  read states when `Λ` exists \[T, §1\].
- #strong[Theorem 4 \[T, E\].] For `n = 3` the existence of a Tian
  kernel depends only on `(q mod ℓ, t mod ℓ)` and the type of
  `φ = π_{q^3}|E[ℓ]`: it exists iff `χ_φ(T) = T^2 − tT + q^3` has a root
  in `F_ℓ` and every root is a cube in `F_ℓ`, except that a Jordan `φ`
  (double root, `φ` not scalar) needs `ℓ ≡ 1 (mod 3)` as well. So
  nonsplit `φ`: never; split `diag(λ, μ)`: iff `λ` is a cube (automatic
  when `ℓ ≢ 1 mod 3`); scalar `λI`: always; Jordan `J(λ)`: iff
  `ℓ ≡ 1 (mod 3)`. The census confirms every cell at `ℓ ∈ {2, 3, 5, 7}`
  and every `q mod ℓ` (`kernels.csv`, 83 classes), and the split, scalar
  and nonsplit cases are proved for all `ℓ` (§3).
- #strong[Which curves (Part H) \[E\].] Over `F_{q^3}`,
  `q ∈ {3, 5, 7, 11, 13}`, every isomorphism class
  (`56, 252, 690, 2662, 4400` of them) was classified at every
  `ℓ ∈ {2, 3, 5, 7} ∖ {p}`. Prime-order classes with a Tian kernel at
  some odd `ℓ ≤ 7`: `0/8, 0/18, 24/61, 57/150, 144/323`; all classes:
  `28/56, 156/252, 372/690, 1984/2662, 2714/4400`. The prime-order
  curves reached are exactly those of Theorem 4 with no eigenvalue 1: at
  `q = 13` the `135` prime-order curves with `t ≡ 2 (mod 5)`
  (`φ = diag(3, 4)` on `E[5]`) and the `12` with `t ≡ 1 (mod 3)` and
  `φ = −I` on `E[3]`; at `q = 11` the `54` with `t ≡ 0 (mod 5)`, the `3`
  with `φ = −I` on `E[5]` and the `12` with `φ = J(−1)` on `E[7]`; at
  `q = 7` the `24` with `t ≡ 1 (mod 5)` and the `3` with `φ = −I` on
  `E[3]`. At `q = 3, 5` no admissible class exists for `ℓ ≤ 7`
  (`q mod ℓ` forbids it), so the count is `0`, as Theorem 4 predicts.
- #strong[Covers never reach them \[T, E\].] At `ℓ = 2` a Tian kernel
  exists only for `φ = I` (check 1), i.e. full rational 2-torsion,
  `#E ≡ 0 (mod 4)`: the exact statement behind the cover attacks’ "three
  rational points of order 2" (Diem 2003; Momose–Chao 2009;
  Tian–Li–Wang–Yu 2018), and the curves with an `ℓ = 2` kernel in Part H
  are precisely the `9, 42, 115, 443, 734` with full 2-torsion (check
  7). Prime-order curves have no kernel at `ℓ = 2` (check 8) and are
  reached only through odd `ℓ`.
- #strong[Assessment.] This is a #emph[different type of affected curve]
  in the sense of the search: a congruence class, `(q mod ℓ, t mod ℓ)`
  plus the splitting type of `E[ℓ]`, containing prime-order curves,
  which no covering attack reaches and which the sources read do not
  name. It is #strong[not hideable]: the test is `t mod ℓ` and the
  factor pattern of the `ℓ`-division polynomial, and the affected
  fraction is a constant (margin `a < 0` in round one’s framework). The
  attack path is Tian’s; this round adds the existence theorem, the
  exact counts, and the `ℓ = 2` identification with the cover attacks.
  Two provisos are taken from Tian and not recomputed here: that `W/Λ`
  is geometrically indecomposable when `Λ` meets no factor (then a
  Jacobian by Oort–Ueno), and the `Õ(ℓ^3)` isogeny algorithm \[T, O\].
- `n = 2`: a Tian kernel exists iff `φ^{−1}` has a square root `ψ` with
  `det ψ = −q̄^{−1}` (Amendment 1, check 2, unit test); the target is a
  genus-2 Jacobian, where index calculus costs `Õ(q)` \= rho, so nothing
  is gained. `n = 4`: kernels exist for `2, 8, 18` of `3, 10, 28`
  classes at `ℓ = 2, 3, 5`; whether `W/Λ` is a Jacobian is the Schottky
  question and is left open \[O\].

== 1. Question, and what the sources say
<question-and-what-the-sources-say>
Round one enumerated the #emph[covering] associations (GHS, gGHS, Kummer
compositums, the hyperelliptic genus-3 census) and proved (Theorem 3
there) that no hyperelliptic genus-3 Jacobian is isogenous to the Weil
restriction of a prime-order curve over `F_{q^3}`. Tian’s introduction
states the same boundary for covers \[T\]: "For `n = 3`, if we require
`C` to be of genus 3, then a necessary condition is that the elliptic
curve has three rational points of order 2 \[3,13\]" and "\[19\] shows
that `C` needs to be non-hyperelliptic if the elliptic curve has prime
order" (arXiv 2012.07173, §1; \[3\] Diem 2003, \[13\] Momose–Chao 2009,
\[19\] Tian–Li–Wang–Yu, DCC 2018). His method: "an `F_q`-rational
`(ℓ,ℓ,ℓ)`-isogeny from the Weil restriction … to the Jacobian variety of
a genus three curve over `F_q`", solving "the discrete logarithm problem
in some elliptic curve groups of prime order over `F_{q^3}` in a time of
`Õ(q)`" (abstract). Section 3 opens with "Assume that we have an
`F_q`-rational maximal isotropic subgroup `Λ` of `W[ℓ]` … In order for
the quotient `W/Λ` to be the Jacobian variety of a curve `C`, `Λ` should
not contain any subgroups of `E`, `E^σ` or `E^{σ^2}`", and the
hyperelliptic decision is by theta constants: "If 35 of them are
non-zero, then `C` is hyperelliptic (this can happen only if the order
of `E(F_{q^3})` is even). If 36 … non-hyperelliptic, … bound to happen
if `E` has prime order." The one example is `q = 523`,
`F_{523^3} = F_{523}[w]`, `w^3 + 2 = 0`,
`y^2 = x^3 + (82w^2 + 92w + 140)x + 359w^2 + 339w + 243`,
`#E = 143069629` prime, `Λ ⊂ W[3]`. No existence condition, no
dependence on `q mod ℓ` or on the trace, and no proportion appear in the
paper (the ar5iv text was read in full for these points). The web
searches of this round (IACR ePrint, Springer, arXiv; queries recorded
in the session) found no 2024–2026 follow-up stating an existence
condition.

So the open question was concrete: #strong[for which `E/F_{q^3}` does a
Tian kernel exist at all?] It is a finite question about
`(ℓ, q mod ℓ, [φ])`, answered here.

== 2. Setting (as frozen in the protocol)
<setting-as-frozen-in-the-protocol>
`φ = π_{q^n}|E[ℓ] ∈ GL_2(F_ℓ)`, `det φ = q^n`, `tr φ = t`, all mod `ℓ`.
`W[ℓ] = V = V_0 ⊕ … ⊕ V_{n−1}`, `V_i = E^{σ^i}[ℓ]` identified with
`E[ℓ]` through `π_q^i`; the `F_q`-Frobenius acts as
`π(v_0, …, v_{n−1}) = (φ v_{n−1}, v_0, …, v_{n−2})`, so `π^n = φ` on
each factor. The `ℓ`-Weil pairing of the Weil-restricted principal
polarisation is `ω_V = ⊕_i q̄^i ω` (`q̄ = q mod ℓ`, `ω` the standard form
on `F_ℓ^2`; the Frobenius raises `μ_ℓ` to the `q`-th power), and
`ω_V(πv, πw) = q̄ ω_V(v, w)` (checked in the protocol’s derivation and
used by the tool). A #strong[kernel] is an `F_ℓ[π]`-submodule of
dimension `n` on which `ω_V` vanishes; a #strong[Tian kernel] meets no
`V_i`. As an `F_ℓ[T]`-module (`T = π`), `V = Ind(E[ℓ])`: the induced
module of the `F_ℓ[S]`-module `E[ℓ]`, `S = φ`, along `S = T^n`. This is
the whole structure; `q` enters only through `q̄` and `det φ`.

== 3. Theorem 4: Tian kernels for `n = 3`
<theorem-4-tian-kernels-for-n-3>
#strong[Theorem 4.] Let `ℓ ≠ p` be prime and `E/F_{q^3}`. A Tian kernel
in `W[ℓ]` exists

- never, if `χ_φ = T^2 − tT + q^3` is irreducible over `F_ℓ` (nonsplit);
- iff `λ` is a cube in `F_ℓ`, if `φ ~ diag(λ, μ)` with `λ ≠ μ` (then `μ`
  is a cube too, since `λμ = q̄^3`);
- always, if `φ = λI`;
- iff `ℓ ≡ 1 (mod 3)`, if `φ ~ J(λ)` (Jordan).

Proof of the nonsplit case \[T\]. `E[ℓ]` is a cyclic `F_ℓ[S]`-module, so
`V ≅ F_ℓ[T]/(χ_φ(T^3))` is cyclic and its submodules are the monic
divisors of `χ_φ(T^3) = T^6 − tT^3 + q^3`. A dimension-3 submodule needs
a divisor of degree 3. The roots of `χ_φ(T^3)` are the cube roots of the
two roots `α, ᾱ ∈ F_{ℓ^2} ∖ F_ℓ` of `χ_φ`; a root `c` in `F_{ℓ^3}` would
have `c^3 ∈ F_{ℓ^3} ∩ F_{ℓ^2} = F_ℓ`, impossible, and a root in `F_ℓ`
likewise. So `χ_φ(T^3)` has no irreducible factor of degree 1 or 3,
hence no divisor of degree 3: there is no submodule of dimension 3 at
all, let alone a kernel. (The tool asserts `submodules_dim_n = 0` on
every nonsplit row; unit test `nonsplit_has_no_dim3_submodule`.)

Proof of the split case \[T\]. `V = A ⊕ B` with
`A = Ind L_λ ≅ F_ℓ[T]/(T^3 − λ)`, `B = Ind L_μ ≅ F_ℓ[T]/(T^3 − μ)`,
coprime, so every submodule is `A' ⊕ B'`. `A ∩ V_0 = L_λ` generates `A`,
so a proper submodule `A'` meets `V_0` trivially, and the Tian kernels
are the `A' ⊕ B'` with `(dim A', dim B') ∈ {(1, 2), (2, 1)}`. Submodules
of dimension 1 or 2 of `F_ℓ[T]/(T^3 − λ)` exist iff `T^3 − λ` has a root
`c ∈ F_ℓ`, i.e. iff `λ` is a cube. If it has, the eigenline
`A' = ⟨(e, c^{−1}e, c^{−2}e)⟩` (`e ∈ L_λ`, `πa = ca`) satisfies
`ω_V(a, πb) = (q̄/c) ω_V(a, b)`, so `A'^⊥ ∩ B` is a `π`-stable subspace
of dimension 2 on which `π` has the quotient eigenvalue `d = q̄/c`, and
`d^3 = q̄^3/λ = μ`: it is the degree-2 submodule of `B`. `B` is totally
isotropic (`ω(L_μ, L_μ) = 0`), so `A' ⊕ (A'^⊥ ∩ B)` is a Tian kernel; by
symmetry so is the `(2, 1)` one. Conversely without a cube root there is
no `A'` of dimension 1 or 2. The count is `2` per cube root of `λ`: `2`
when `ℓ ≢ 1 (mod 3)`, `6` when `ℓ ≡ 1 (mod 3)` and `λ` is a cube (both
seen in `kernels.csv`: `ℓ = 3, 5` give `2`, `ℓ = 7`, `diag(1, 6)` gives
`6`). Note `λμ = q̄^3` makes `λ` a cube iff `μ` is.

Scalar case \[T for existence, E for the count\]. `λ^2 = q̄^3` forces
`ord(λ) | gcd(2(ℓ−1)/3, ℓ−1) = (ℓ−1)/3` when `ℓ ≡ 1 (mod 3)`, so an
admissible scalar is always a cube and `T^3 − λ` has a root; `V = M^2`
with `M = F_ℓ[T]/(T^3 − λ)`, and the split construction applied to any
two independent eigenlines of `π` in different copies gives a Tian
kernel. The counts `6, 12, 30, 72` at `ℓ = 2, 3, 5, 7` are the census’s.

Jordan case \[T for the negative half, E for the positive\].
`V ≅ F_ℓ[T]/((T^3 − λ)^2)` is cyclic; `λ` is a cube as above. If
`ℓ ≢ 1 (mod 3)`, `T^3 − λ = (T − c)h` with `h` irreducible quadratic and
the only degree-3 divisor is `T^3 − λ` itself, whose submodule is
`Ind L` for the eigenline `L ⊂ E[ℓ]`: it meets `V_0`. So no Tian kernel,
as `kernels.csv` shows at `ℓ = 2, 3, 5`. If `ℓ ≡ 1 (mod 3)`, `T^3 − λ`
splits, the seven degree-3 divisors give seven submodules, three are
isotropic and two meet no factor (`ℓ = 7`, every admissible `q̄`; unit
test `jordan_seven_divisors`). That the positive half holds for every
`ℓ ≡ 1 (mod 3)` is \[H\] beyond `ℓ = 7`.

#strong[Corollary (prime order).] A prime-order `E/F_{q^3}` has no
eigenvalue 1 at any `ℓ`, so its Tian kernels come from: `φ = λI` with
`λ ≠ 1` (`λ^2 = q̄^3`; e.g. `φ = −I` on `E[ℓ]`, `t ≡ −2 (mod ℓ)`), split
`φ` with both eigenvalues cubes and `≠ 1`, or `φ = J(λ)`, `λ ≠ 1`, at
`ℓ ≡ 1 (mod 3)`. Since split-versus-nonsplit is decided by whether
`T^2 − tT + q^3` has a root mod `ℓ`, #strong[for `ℓ ≢ 1 (mod 3)` the
condition is purely a congruence on `(q, t) mod ℓ`] (plus "not Jordan"
at a double root).

== 4. Part G: the census over the Frobenius classes \[E\]
<part-g-the-census-over-the-frobenius-classes-e>
`kernels.csv`: one row per `(n, ℓ, q̄, [φ])`, `83` rows at `n = 3`. Every
cell of the frozen range ran (`n = 2, 3, 4`; `ℓ ≤ 7`, `ℓ ≤ 5` at
`n = 4`; 75 s). Checks: 1 (`n = 3`, `ℓ = 2`: kernels only for `φ = I`):
0 violations; 2 (`n = 2`: every Tian kernel is a graph with
`det ψ = −q̄^{−1}`, `φψ^2 = I`): 0 violations; 3 (P2): at `(3, 3, 1)` the
class `φ = 2I` (no eigenvalue 1) has 12 Tian kernels, which is where
Tian’s `q = 523 ≡ 1 (mod 3)`, `ℓ = 3` example lives; 4 (representative
independence, two alternative representatives per split/nonsplit/Jordan
class): 0 mismatches.

#align(center)[#table(
  columns: 4,
  align: (col, row) => (left,right,right,right,).at(col),
  inset: 6pt,
  [`(n, ℓ)`], [classes `[φ]` (all `q̄`)], [with a Tian kernel], [… and no
  eigenvalue 1],
  [(2, 2)],
  [3],
  [2],
  [1],
  [(2, 3)],
  [10],
  [5],
  [2],
  [(2, 5)],
  [28],
  [14],
  [9],
  [(2, 7)],
  [54],
  [27],
  [20],
  [(3, 2)],
  [3],
  [1],
  [0],
  [(3, 3)],
  [8],
  [3],
  [1],
  [(3, 5)],
  [24],
  [10],
  [6],
  [(3, 7)],
  [48],
  [15],
  [6],
  [(4, 2)],
  [3],
  [2],
  [1],
  [(4, 3)],
  [10],
  [8],
  [5],
  [(4, 5)],
  [28],
  [18],
  [10],
)
]

`n = 3`, the rows with a Tian kernel (all others have none):

#align(center)[#table(
  columns: 9,
  align: (col, row) => (right,right,left,right,right,left,right,right,right,).at(col),
  inset: 6pt,
  [`ℓ`], [`q̄`], [class], [`tr`], [`det`], [eigenvalue 1], [submodules
  (dim 3)], [kernels], [Tian kernels],
  [2],
  [1],
  [scalar `I`],
  [0],
  [1],
  [yes],
  [15],
  [9],
  [6],
  [3],
  [1],
  [scalar `2I`],
  [1],
  [1],
  [no],
  [40],
  [16],
  [12],
  [3],
  [1],
  [scalar `I`],
  [2],
  [1],
  [yes],
  [40],
  [16],
  [12],
  [3],
  [2],
  [split `(1,2)`],
  [0],
  [2],
  [yes],
  [4],
  [4],
  [2],
  [5],
  [1],
  [split `(2,3)`],
  [0],
  [1],
  [no],
  [4],
  [4],
  [2],
  [5],
  [1],
  [scalar `I`],
  [2],
  [1],
  [yes],
  [156],
  [36],
  [30],
  [5],
  [1],
  [scalar `−I`],
  [3],
  [1],
  [no],
  [156],
  [36],
  [30],
  [5],
  [2],
  [split `(2,4)`],
  [1],
  [3],
  [no],
  [4],
  [4],
  [2],
  [5],
  [2],
  [split `(1,3)`],
  [4],
  [3],
  [yes],
  [4],
  [4],
  [2],
  [5],
  [3],
  [split `(3,4)`],
  [2],
  [2],
  [no],
  [4],
  [4],
  [2],
  [5],
  [3],
  [split `(1,2)`],
  [3],
  [2],
  [yes],
  [4],
  [4],
  [2],
  [5],
  [4],
  [split `(1,4)`],
  [0],
  [4],
  [yes],
  [4],
  [4],
  [2],
  [5],
  [4],
  [scalar `3I`],
  [1],
  [4],
  [no],
  [156],
  [36],
  [30],
  [5],
  [4],
  [scalar `2I`],
  [4],
  [4],
  [no],
  [156],
  [36],
  [30],
  [7],
  [1, 2, 4],
  [scalar `I`],
  [2],
  [1],
  [yes],
  [560],
  [80],
  [72],
  [7],
  [1, 2, 4],
  [Jordan `J(1)`],
  [2],
  [1],
  [yes],
  [7],
  [3],
  [2],
  [7],
  [1, 2, 4],
  [scalar `−I`],
  [5],
  [1],
  [no],
  [560],
  [80],
  [72],
  [7],
  [1, 2, 4],
  [Jordan `J(−1)`],
  [5],
  [1],
  [no],
  [7],
  [3],
  [2],
  [7],
  [3, 5, 6],
  [split `(1,6)`],
  [0],
  [6],
  [yes],
  [20],
  [8],
  [6],
)
]

The nonsplit rows have `0` submodules of dimension 3 (Theorem 4). The
`ℓ = 7` split classes `(2,4)`, `(3,5)` (`det 1`) and `(2,3)`, `(4,5)`
(`det 6`) have exactly the two submodules `A`, `B` and no Tian kernel:
`2, 3, 4, 5` are not cubes mod 7 (the cubes are `1, 6`). The split rows
at `ℓ = 3, 5` are every split class there is: every element is a cube.

== 5. Part H: every curve over `F_{q^3}` \[E\]
<part-h-every-curve-over-f_q3-e>
`curves.csv` (`4400 + 2662 + 690 + 252 + 56 = 8060` classes, one row per
`(curve, ℓ)`), `summary.csv`, `by_class.csv`, `checks_h.txt`. Check 5
(class count) is exact at every `q`; checks 6–8 have 0 violations; the
class of `φ` was identified for every `(curve, ℓ)`
(`class_not_found = 0`).

#align(center)[#table(
  columns: 7,
  align: (col, row) => (right,right,right,right,right,right,right,).at(col),
  inset: 6pt,
  [`q`], [classes], [prime order], [cofactor 2], [full 2-torsion (\=
  kernel at `ℓ = 2`)], [kernel at some odd `ℓ ≤ 7`: all], [prime order],
  [3],
  [56],
  [8],
  [7],
  [9],
  [28],
  [0],
  [5],
  [252],
  [18],
  [25],
  [42],
  [156],
  [0],
  [7],
  [690],
  [61],
  [48],
  [115],
  [372],
  [24],
  [11],
  [2662],
  [150],
  [150],
  [443],
  [1984],
  [57],
  [13],
  [4400],
  [323],
  [288],
  [734],
  [2714],
  [144],
)
]

By `ℓ` (`with_tian`, prime-order, cofactor-2); `ℓ = p` is excluded:

#align(center)[#table(
  columns: 4,
  align: (col, row) => (right,left,left,left,).at(col),
  inset: 6pt,
  [`q`], [`ℓ = 3`], [`ℓ = 5`], [`ℓ = 7`],
  [3],
  [–],
  [26, 0, 6],
  [8, 0, 0],
  [5],
  [124, 0, 0],
  [–],
  [40, 0, 0],
  [7],
  [58, 3, 6],
  [342, 24, 0],
  [–],
  [11],
  [1330, 0, 0],
  [710, 57, 78],
  [758, 12, 12],
  [13],
  [368, 12, 24],
  [2198, 135, 114],
  [732, 0, 0],
)
]

The classes behind the prime-order counts (`by_class.csv`), each a cell
of Theorem 4 without eigenvalue 1: `q = 7`: `ℓ = 3`, `φ = 2I` (`3`
curves of `29` in the class); `ℓ = 5`, `q̄ = 2`, `diag(2,4)`, `t ≡ 1`
(`24` of `171`). `q = 11`: `ℓ = 5`, `q̄ = 1`, `φ = −I` (`3` of `22`) and
`diag(2,3)`, `t ≡ 0` (`54` of `666`); `ℓ = 7`, `q̄ = 4`, `J(−1)`, `t ≡ 5`
(`12` of `370`). `q = 13`: `ℓ = 3`, `φ = 2I` (`12` of `184`); `ℓ = 5`,
`q̄ = 3`, `diag(3,4)`, `t ≡ 2` (`135` of `1099`). At `q = 5` no
admissible class without eigenvalue 1 exists for `ℓ ≤ 7` (`q ≡ 2 mod 3`
gives only `diag(1,2)` at `ℓ = 3`, `q ≡ 5 mod 7` only `diag(1,6)` at
`ℓ = 7`), and at `q = 3` the one such class, `ℓ = 5`, `q̄ = 3`,
`diag(3,4)`, holds 13 curves of which none has prime order (6 have
cofactor 2): the zeros at `q = 3, 5` are the absence or emptiness of a
cell, not counter-examples to Theorem 4.

Reading. (i) The curves reached at odd `ℓ` are a constant fraction at
every `q` (`50–75%` of all classes, `37–45%` of the prime-order classes
once a non-empty admissible cell exists), and the fraction moves with
`q mod ℓ`, not with `q`: at `q = 11` the `ℓ = 3` cell is the
eigenvalue-1 class `diag(1,2)` (no prime-order curve), at `q = 13` it is
`2I` (12 prime-order curves). (ii) Cofactor-2 curves are reached at odd
`ℓ` too (per `ℓ`, `summary.csv`: `6` at `q = 3`; `6` at `q = 7`;
`78 + 12` at `q = 11`; `24 + 114` at `q = 13`, a curve with kernels at
two `ℓ` counted twice): the "cofactor 2" question of Tian–Li–Wang–Yu has
an isogeny-side answer. (iii) `full_2_torsion = with_tian(ℓ = 2)` at
every `q` and check 7 has no violation: the `ℓ = 2` Tian kernels are
exactly the full-2-torsion curves, the necessary condition of the
genus-3 cover attacks (Diem; Momose–Chao), and the compositum covers of
round one (Theorem 2 there) live inside this set, consistent with
Joux–Vitse’s "about half of the curves having their full 2-torsion".

== 6. What is affected, at what cost, and whether it can hide
<what-is-affected-at-what-cost-and-whether-it-can-hide>
- #strong[Affected set (`n = 3`).] Prime-order `E/F_{q^3}` with
  `(q mod ℓ,   t mod ℓ)` in a cell of Theorem 4 without eigenvalue 1,
  for some small `ℓ ≠ p`. For `ℓ = 3`: `q ≡ 1 (mod 3)` and
  `t ≡ 1 (mod 3)` with `φ = −I` (the 3-torsion field is `F_{q^6}`). For
  `ℓ = 5`: `q ≡ 1`: `t ≡ 0` (split) or `t ≡ 3` (`−I`); `q ≡ 2`: `t ≡ 1`;
  `q ≡ 3`: `t ≡ 2`; `q ≡ 4`: `t ≡ 1` or `4` with `φ` scalar. For
  `ℓ = 7`: `q ≡ 1, 2, 4` and `t ≡ 5` (`−I` or `J(−1)`). Larger `ℓ` add
  cells; heuristically \[H\] a random prime-order curve has a cube-split
  `φ` at a given `ℓ` with probability about `1/2 · 1/3` (`ℓ ≡ 1 mod 3`)
  or `1/2` (`ℓ ≡ 2 mod 3`) minus the eigenvalue-1 cases, so almost every
  prime-order curve is reached at some `ℓ ≤ 20`; the exact small-`ℓ`
  figures above are the evidence.
- #strong[Cost.] Transfer `Õ(ℓ^3)` \[T, Tian\] plus the Jacobian DLP
  `Õ(q)` \[T, Diem–Thomé\], against `q^{3/2}` (rho) and `Õ(q^{4/3})`
  (Gaudry’s decomposition, which applies to every curve over `F_{q^3}`).
  Exponents only; the constants of Diem–Thomé’s algorithm are not
  measured here \[O\].
- #strong[Auditor’s test.] Compute `t`, check the congruences, and
  factor the `ℓ`-division polynomial over `F_{q^3}` for the splitting
  type (Part H does exactly this; seconds at toy size, polynomial time
  in general). The affected set is a constant fraction of every
  admissible trace class, so in round one’s margin framework `a < 0`:
  nothing to hide, nothing to walk to; the isogeny class decides, up to
  the scalar/Jordan distinction at a double root (an `ℓ`-volcano-level
  property, also polynomial-time).
- #strong[What the census does not prove.] That `W/Λ` is a Jacobian
  (Tian’s indecomposability condition `Λ ∩ V_i = 0` is what we counted;
  Oort–Ueno then gives a Jacobian or its Serre twist for a geometrically
  indecomposable 3-dimensional ppav), and that the explicit isogeny is
  as cheap as stated: both are Tian’s \[T\], not re-derived; no isogeny
  was computed in this round \[O\].

== 7. `n = 2` and `n = 4`
<n-2-and-n-4>
`n = 2` \[T, E\]: Amendment 1’s closed form is confirmed (check 2; the
unit test `n2_closed_form` compares the census with a brute-force search
over `GL_2(F_ℓ)` at `ℓ = 3, 5`): a Tian kernel exists iff `φ^{−1} = ψ^2`
for some `ψ` with `det ψ = −q̄^{−1}`. Classes with a kernel and no
eigenvalue 1: `1, 2, 9, 20` of `3, 10, 28, 54` at `ℓ = 2, 3, 5, 7`. The
target is a 2-dimensional ppav, a genus-2 Jacobian or a Weil restriction
of an elliptic curve over `F_{q^2}` again; genus-2 index calculus costs
`Õ(q)`, the same as rho on `E(F_{q^2})`, so `n = 2` is "reached" without
being weaker (as Scholten’s and Joux–Vitse’s genus-2 covers already
are).

`n = 4` \[E, O\]: kernels exist for `2, 8, 18` of the `3, 10, 28`
classes at `ℓ = 2, 3, 5` (`1, 5, 10` without eigenvalue 1). `W/Λ` is a
4-dimensional ppav; a Jacobian only on the Schottky locus (codimension 1
in `A_4`), and if a non-hyperelliptic genus-4 Jacobian, Diem’s index
calculus would cost `Õ(q^{4/3})` against `q^2` (rho) and `Õ(q^{3/2})`
(Gaudry). Whether any `W/Λ` is a Jacobian is open; it is the one place
in this round where a #emph[sparse] affected set could exist (a `1/q`
Schottky fraction of the kernels), and the next step is explicit: at
`q ≤ 13`, `ℓ = 3`, construct `W/Λ` by Tian’s theta method generalised to
dimension 4 and test the Schottky–Igusa modular form.

== 8. Checks and controls
<checks-and-controls>
#align(center)[#table(
  columns: 3,
  align: (col, row) => (left,left,left,).at(col),
  inset: 6pt,
  [check], [where], [result],
  [1. `n = 3`, `ℓ = 2`: Tian kernels only for `φ = I` (P3)],
  [`checks_g.txt`],
  [pass (0 violations)],
  [2. `n = 2`: every Tian kernel is a graph with `det ψ = −q̄^{−1}`,
  `φψ^2 = I` (Amendment 1)],
  [`checks_g.txt`, unit test],
  [pass (0 violations; census \= brute force at `ℓ = 3, 5`)],
  [3. P2: a class without eigenvalue 1 has a kernel at `(3, 3, 1)`],
  [`checks_g.txt`],
  [pass (`φ = 2I`, 12 kernels)],
  [4. representative independence],
  [`checks_g.txt`],
  [pass (0 mismatches)],
  [5. class counts `2(q^3 − 2) + gcd(4, q^3 − 1) + gcd(6, q^3 − 1)`
  (`p ≥ 5`); `2(q^3 − 1) + 4` at `q = 3`],
  [`checks_h.txt`],
  [pass at every `q`],
  [6. division polynomials: degree, squarefree; cubic degree 3],
  [`checks_h.txt`],
  [pass (0 violations)],
  [7. `ℓ = 2` class `I` iff the cubic splits iff full 2-torsion;
  `4 | #E` then; kernel at `ℓ = 2` only then],
  [`checks_h.txt`, `summary.csv`],
  [pass (`full_2_torsion = with_tian`)],
  [8. prime-order curves: no eigenvalue 1 at any `ℓ`, no kernel at
  `ℓ = 2`],
  [`checks_h.txt`],
  [pass],
  [9. prime-order fractions reported per `q` and `ℓ`],
  [`summary.csv`, §5],
  [reported],
  [Hasse bound; class found for every `(curve, ℓ)`],
  [`checks_h.txt`],
  [pass],
  [unit tests (8): rref, `n = 2` closed form, orbit patterns, division
  polynomials at `q = 11`, 2-torsion vs order at `q = 7`, class count at
  `q = 5`, Jordan `ℓ = 7` divisors, nonsplit has no dim-3 submodule],
  [`rustc --test`],
  [pass],
)
]

Predictions: P1 as corrected (Amendment 1) holds; P2 holds; P3 holds; P4
holds in the form "a constant fraction once an admissible cell exists"
(`0` at `q = 3, 5` is the absence of a cell, see §5).

== 9. What is not established
<what-is-not-established>
- \[O\] No isogeny computed, no DLP solved, no constant measured. The
  Jacobian property of `W/Λ` and the `Õ(ℓ^3)` isogeny are Tian’s.
- \[H\] The Jordan positive half of Theorem 4 beyond `ℓ = 7`; the
  heuristic density of cube-split classes at larger `ℓ`.
- \[O\] `n = 4`: the Schottky question.
- \[O\] Characteristic 2 base fields in Part H (the table fields are
  odd); Part G is characteristic-free.
- \[O\] No Lean statement for Theorem 4 (its nonsplit and split halves
  are short module-theory arguments; a formalisation is owed under
  AGENTS.md).
- \[O\] Conductor could not be installed (installer HTTP 403 through the
  session proxy, as in round one); no task card.
- Nothing here concerns a standardised curve; `F_{q^3}` fields are the
  literature’s setting (Tian, Joux–Vitse, Momose–Chao).

== 10. Canonical graphs and diagram
<canonical-graphs-and-diagram>
- `figures/mechanism.svg/png`: the isogeny route, Theorem 4’s decision
  table, and the cover route for contrast (`figures/diagram.py`).
- `figures/affected.svg/png`: fraction of classes over `F_{q^3}` with a
  Tian kernel, all and prime-order, by `q`, and the prime-order fraction
  by `ℓ` (`summary.csv`).
- `figures/kernel_map.svg/png`: `n = 3`, Tian kernels by
  `(ℓ, q mod ℓ, [φ])` (`kernels.csv`).
- `figures/n2n4.svg/png`: classes with a kernel by `(n, ℓ)`
  (`kernels.csv`).
- `reports/run-20261018T1605Z/run-report.{typ,pdf}`: environment, log,
  checks, the summary and the `n = 3` kernel table, the graphs.
- `REPORT.typ/pdf` from this file (`build.sh`).

#figure([#image("figures/mechanism.svg")],
  caption: [
    The isogeny route, Theorem 4’s decision table, and the cover route
    (figures/mechanism.svg).
  ]
)

#figure([#image("figures/affected.svg")],
  caption: [
    Curves over `F_{q^3}` with a Tian kernel, by `q` and by `ℓ`
    (figures/affected.svg; `summary.csv`).
  ]
)

#figure([#image("figures/kernel_map.svg")],
  caption: [
    `n = 3`: Tian kernels by `(ℓ, q mod ℓ, [φ])`
    (figures/kernel\_map.svg; `kernels.csv`).
  ]
)

#figure([#image("figures/n2n4.svg")],
  caption: [
    Kernel census by `(n, ℓ)` (figures/n2n4.svg; `kernels.csv`).
  ]
)

== References
<references>
- S. Tian, #emph[Cover attacks for elliptic curves with prime order] /
  #emph[Cover attacks for elliptic curves over cubic extension fields],
  J. Cryptology 36 (2023), arXiv 2012.07173 (text read via ar5iv and
  pdftotext).
- S. Tian, B. Li, K. Wang, W. Yu, #emph[Cover attacks for elliptic
  curves with cofactor two], Designs, Codes and Cryptography 86 (2018)
  2451–2468.
- C. Diem, #emph[The GHS attack in odd characteristic], J. Ramanujan
  Math. Soc. 18 (2003); C. Diem, E. Thomé, #emph[Index calculus in class
  groups of non-hyperelliptic curves of genus three], J. Cryptology 21
  (2008).
- F. Momose, J. Chao, #emph[Elliptic curves with weak coverings over
  cubic extensions of finite fields with odd characteristic], ePrint
  2009/236.
- A. Joux, V. Vitse, #emph[Cover and decomposition index calculus on
  elliptic curves made practical], Eurocrypt 2012 (ePrint 2011/020).
- P. Gaudry, #emph[Index calculus for abelian varieties of small
  dimension and the elliptic curve discrete logarithm problem], J. Symb.
  Comp. 44 (2009).
- F. Oort, K. Ueno, #emph[Principally polarized abelian varieties of
  dimension two or three are Jacobian varieties], J. Fac. Sci. Univ.
  Tokyo 20 (1973).
- Round one: `research/teske_variants_field_types_20261018/REPORT.md`
  (Theorems 2 and 3, Part B’s `(3,5)`/`(2,6)` types, Part C’s census).
