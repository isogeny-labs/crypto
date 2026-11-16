= What can be proved about ECDLP across a large prime conductor gap
<what-can-be-proved-about-ecdlp-across-a-large-prime-conductor-gap>
Companion to #link("README.md")[`README.md`]. Each statement is labelled
#strong[proved here], #strong[proved in Lean], #strong[literature (as
reported)] or #strong[open]. The Lean file is
#link("lean/DeepGap.lean")[`lean/DeepGap.lean`]; it builds on
`TransferBound` from
#link("../conductor_gap_cm_pairs_20261010/lean/README.md")[`../conductor_gap_cm_pairs_20261010/lean/`].

== Setting
<setting>
- `E/F_p` is ordinary with Frobenius `π` and trace `t`, and
  `t² − 4p = D₀·f²` with `D₀` fundamental.
- The `F_p`-isogeny class is `𝓘_t`. Every `E ∈ 𝓘_t` has `End(E) ≅ O_c`,
  the order of conductor `c = c(E)` in `K = Q(√D₀)`, with `c ∣ f`.
- `N` is a prime with `N ∥ f` (the gap prime). The two sides are
  `S1 = {E : N ∤ c(E)}` and `S2 = {E : N ∣ c(E)}`. Every isogeny between
  `S1` and `S2` has degree divisible by `N` (Kohel 1996).
- `n` is a prime dividing `#E(F_p)` with `n > 4√p`, `P` has order `n`,
  and the instance is `(P, Q = [k]P)`.
- `T_A(E)` is the cost of algorithm `A` on `E`, and `T*(E)` the least
  cost over all always-correct algorithms.

== Theorem 1 — what every curve of the class shares
<theorem-1--what-every-curve-of-the-class-shares>
#emph[Status: proved here.]

For all `E, E' ∈ 𝓘_t`:

- `#E(F_p) = #E'(F_p) = p + 1 − t`;
- `E(F_p)[n] ≅ E'(F_p)[n] ≅ Z/n`;
- the embedding degree `ord_n(p)` is the same, and so is anomalousness
  (`#E = p`).

So every attack whose applicability and cost are functions of
`(p, #E, n, ord_n p)` applies identically on both sides of any conductor
gap. These are Pohlig–Hellman, MOV/Frey–Rück, Smart/SSSA, and BSGS and
rho in the generic group model, whose `Ω(√(εn))` lower bound (Shoup
1997) holds on every curve because the group is the same.

#emph[Proof.] Tate: isogenous over `F_p` ⟺ same `#E(F_p)`.
`n² > 16p > #E`, so the `n`-part is cyclic of order `n`. The embedding
degree depends on `n` and `p` only. ∎

The prior study\'s Lean file `GenericInvariance.lean` proves the
algorithm-level form for Nechaev-model algorithms. They have the same
transcript on isomorphic groups. That model does #strong[not] cover the
hashed rho walks ecbench runs (sheriff case SHF-20261016-3caa6f). For
those, equal cost is an empirical question, measured in
#link("README.md")[`README.md`].

== Theorem 2 — transfer, for every algorithm
<theorem-2--transfer-for-every-algorithm>
#emph[Status: proved here; Lean: `TransferBound.transfer`,
`DeepGap.transfer_comp_cost`, `DeepGap.chain_equivalence`.]

Let `φ = φ_m ∘ … ∘ φ_1 : E → E'` be a chain of `F_p`-isogenies with
`gcd(deg φ_i, n) = 1`, where `φ_i` costs `C_i` to evaluate at one point.
Then for every always-correct algorithm `A'` on `E'` there is one on
`E`, returning the same logarithm, with

`T(E) ≤ T_{A'}(E') + 2·Σ C_i` on every instance.

The solver may be anything: structural, reading the endomorphism ring,
non-generic.

#emph[Proof.]

+ `#ker φ_i = deg φ_i`; the isogenies are separable, since `p ∤ deg`.
+ So `ker φ ∩ ⟨P⟩` has order dividing `gcd(deg φ, n) = 1`.
+ Then `φ(P)` has order `n` and generates `E'(F_p)[n]`, and
  `φ(Q) = [k]φ(P)`.
+ Solve `(φ(P), φ(Q))` on `E'`: the answer is `k`. The cost is two
  evaluations of each `φ_i` plus the call.

Lean: composition preserves injectivity and the homomorphism laws, and
the costs add (`Isogeny.comp`, `comp_cost`). Transfer along the
composite costs `2·Σ C_i` and returns the same answer
(`transfer_comp_cost`, `transfer_comp_answer`). With chains both ways,
every solver on either end has a partner on the other
(`chain_equivalence`). ∎

So #strong[pairwise] hardness across a conductor gap is a question about
the cost of the chain, nothing else.

== Theorem 3 — the class measured here: explicit chains
<theorem-3--the-class-measured-here-explicit-chains>
#emph[Status: proved here; the arithmetic is in Lean. Chains cost
`O(N + d)`.]

Hypotheses:

- `t² − 4p = D₀·(2^d·N)²` with `h(D₀) = 1`, `D₀ ∉ {−3, −4}`, and 2 inert
  in `K` (`D₀ ≡ 5 (mod 8)`);
- `N` an odd prime, `N ∤ D₀`;
- #strong[`t ≡ −2 (mod N)`].

The class built by `volcano-gap` has `D₀ = −11`, `d = 6`, `N = 631409`
(Lean: `class_discriminant`, `gap_prime_divides_t_plus_2`).

#strong[(a) `S1`: every `N`-kernel is twist-rational.]

- Let `E ∈ S1`, so `N ∤ c` and `N ∣ f/c = [O_c : Z[π]]`. Write
  `π = a + (f/c)·ω_c` with `O_c = Z[ω_c]`. Then `(π − a)/N ∈ O_c`, so
  `π` acts on `E[N]` as the scalar `a`.
- From `a² − ta + p ≡ 0` and `t² ≡ 4p (mod N)`, `a ≡ t/2 ≡ −1`.
- So the quadratic twist `E^t` has `E^t[N] ⊆ E^t(F_p)`, which is
  `(Z/N)²`. Lean `twist_order_sq_dvd`: `N² ∣ 4·#E^t` from
  `4·#E^t = (t + 2)² − D₀f²`.
- Each of the `N + 1` cyclic `N`-subgroups of `E` has its `(N − 1)/2`
  abscissae in `F_p`. Vélu over `F_p` then builds the isogeny in `O(N)`
  field operations and evaluates it in `O(N)` per point.

#strong[(b) `S2`: one twist-rational line.]

- Let `E ∈ S2`, so `N ∣ c` and `N ∤ f/c`. Then `π` is not a scalar on
  `E[N]` (Kohel), so it is a Jordan block with eigenvalue `−1`.
- So `E^t(F_p)[N]` is one line. That line is the kernel of the unique
  ascending `N`-isogeny, which is built the same way in `O(N)`.

#strong[(c) The side certificate.] `E ∈ S1 ⟺ E^t(F_p)[N] ≅ (Z/N)²`. In
practice:

- S2 is proved by one witness point `R` with `[#E^t/N]R ≠ O`.
- S1 holds with error at most `N^{−16}` after 16 points that do vanish.

`volcano-gap certify` runs this on all 760 curve models.

#strong[(d) Shape.]

- `N`-isogenies leave the 2-part of the conductor alone, and 2-isogenies
  leave the `N`-part alone.
- `h(O_K) = 1` and 2 is inert, so `S1` is a single 2-volcano with a
  one-vertex crater (`j(D₀)`) and `3·2^{a−1}` curves on level `a ≥ 1`:
  `3·2^d − 2` curves in all (Lean `volcanoSize_closed`; 190 for `d = 6`,
  `depth6_size`).
- `S2` is `h(O_N) = N − (D₀/N)` volcanoes of the same shape.
- In this class `(−11/N) = (N/11) = +1` (Lean `gap_prime_mod_11`), so
  `h(O_N) = N − 1`. Two of the `N + 1` `N`-isogenies from `j(D₀)` are
  horizontal (endomorphisms) and `N − 1` descend, one to each `S2`
  crater.
- Each 2-edge is a Vélu 2-isogeny, and its dual is again a 2-isogeny
  edge of the volcano.

#strong[(e) Floor no harder than crater, explicitly.] Take `E ∈ S2` at
level `a` and `E' ∈ S1`:

+ ascend by `E`\'s twist-rational line, one `N`-isogeny into `S1` at
  level `a`;
+ walk at most `2d` 2-isogenies to `E'`.

By Theorem 2, `T(E) ≤ T_{A}(E') + O(N + d)` per instance, for every `A`.
The construction is `O(N + d + log p)`.

#strong[(f) Crater no harder than a given floor curve.] Take `E ∈ S1`
and a given `E' ∈ S2`:

+ `E'` ascends to some `E₁ ∈ S1`, and the dual `E₁ → E'` has a
  twist-rational kernel;
+ finding that kernel among `E₁`\'s `N + 1` lines costs `O(N²)` once, or
  `Õ(N)` with Bostan–Morain–Salvy–Schost (literature; not implemented
  here);
+ after that, `T(E) ≤ T_{A}(E') + O(N + d)` per instance.

#strong[(g) No KKM isolation in this class.]

- From `j(D₀)`, a uniformly random line of `E[N]` is horizontal with
  probability `2/(N + 1)`. Otherwise it lands on a uniformly random `S2`
  crater.
- A uniformly random descending 2-walk then lands on a uniform curve of
  any chosen level.
- So if some algorithm solves a fraction `ε` of the `S2` curves of one
  level, it solves the `S1` crater in expected `O((N + d)/ε)` field
  operations plus `O(1/ε)` calls.

The isolation Koblitz, Koblitz and Menezes (2011) proposed for crater
curves rests on Problem B, descending from a given crater curve. Problem
B is trivial in this class because `t ≡ −2 (mod N)`. The same argument
works with `O(N·k)`-size extension arithmetic whenever `t/2 mod N` has
small order `k` in `F_N^×/±1`.

#emph[Cost measured here] (`run/pair.json`, `volcano-gap construct`):

#align(center)[#table(
  columns: 2,
  align: (col, row) => (left,left,).at(col),
  inset: 6pt,
  [step], [measured],
  [build the descending `N`-isogeny],
  [0.15 s, about 4.4·10⁷ field multiplications],
  [evaluate it at one point],
  [about 3.2·10⁶ field multiplications, 0.019 s],
  [transfer one instance (2 points)],
  [about 6.3·10⁶ field multiplications, 0.04 s],
  [one `rho.negation` solve, for comparison],
  [about 1.29·10⁶ group operations, 0.39 s mean wall time
  (#link("README.md")[`README.md`])],
)
]

∎

== Theorem 4 — where \"the same\" fails: automorphisms
<theorem-4--where-the-same-fails-automorphisms>
#emph[Status: proved here (a constant factor); measured in this
repository.]

`Aut(E) = O_{c(E)}^×`, which is `{±1}` unless `O_c = Z[i]` (order 4) or
`Z[ζ₃]` (order 6). Those are maximal orders, so `c = 1`.

So in a class with `D₀ ∈ {−3, −4}`:

- exactly the crater curves with `c = 1` (`j = 1728` or `0`) have extra
  automorphisms, and they lie in `S1`;
- a rho walk on classes of size `|Aut|` costs about `√(πn/(2|Aut|))`, a
  factor `√3` or `√2` below negation rho (Duursma–Gaudry–Morain 1999).

Measured in this repository:

- `rho.cubic` against `rho.negation` read 1.69–1.92 against `√3 = 1.73`
  (#link("../drastic_prime_conductor_gap_round2_20261012/README.md")[drastic round 2],
  #link("../drastic_prime_conductor_gap_round3_20261012/README.md")[round 3]);
- the `√2` walk under-realizes, at 1.14–1.32.

This is a #strong[constant factor on one method run where the curve is
given]. It is not a difference in `T*`:

- by Theorem 2 and the cheap ascent (Theorem 3(e) here; Galbraith 2024
  in general), a floor curve\'s instance can be carried to the `j = 0`
  crater and solved there with the cubic walk;
- so `T*(floor) ≤ T*(crater) + transfer`;
- and `T*(crater) ≤ T*(floor) + transfer` by Theorem 2 with the descent.

The `√3` follows the instance once the map is built. In this study\'s
class `D₀ = −11`, so `Aut = ±1` on every curve and there is no
exception.

== Theorem 5 — general classes
<theorem-5--general-classes>
#emph[Status: literature, as reported; not re-derived here.]

Galbraith, #emph[Climbing and descending tall isogeny volcanos], ePrint
2024/924 (summary:
`crypto-autoresearcher/knowledge/literature/KN-LIT-ca35e0.md`):

- given two curves of a class with `q^{1/4} < N < q^{1/2}` joined by an
  `N`-isogeny, a representation is found in `Õ(N^{1/2}) = Õ(q^{1/4})`
  (Kani/Robert);
- ascending from any floor curve costs at most `Õ(q^{1/4})`;
- for tall volcanoes with polynomial-size craters, which include
  CM/class-number-one classes like this one, an isogeny between
  #strong[any two given curves] costs `Õ(q^{1/4})`, rigorously;
- in general classes, heuristically `Õ(q^{2/5})`.

With Theorem 2 this gives, for any two #strong[given] curves of a class:
`|T*(E) − T*(E')| ≤ Õ(q^{1/4})` (small crater) or `Õ(q^{2/5})`
(general). Both are `o(√q)`. So the generic exponent `1/2`, and any
sub-`√q` exponent above `1/4`, is shared by every curve of the class,
gap or no gap.

== Open — what cannot be claimed
<open--what-cannot-be-claimed>
#strong[Problem B / the KKM speculation.] Is there a weakness held by a
positive fraction of `S2` curves that a given `S1` curve cannot reach
faster than `√q`?

- Reaching it means producing `S2` curves from `S1`. The best known cost
  is `min(N², q^{1/2})` when `t/2 mod N` has large order (Galbraith 2024
  §11, stated as the main open problem).
- In such classes the random self-reduction (Jao–Miller–Venkatesan 2005;
  Galbraith Thm 5) covers `S2` but not `S1`.
- Theorem 3(g) closes it only for the special classes with `t/2` of
  small order mod `N`, like the one measured here.
- No proof, and no measurement at toy size, settles it in general.
  KKM\'s argument assumes the weakness is independent of the
  endomorphism ring, and no such weakness is known on prime fields.

#strong[Exact equality of one method\'s constant.] Theorems 1–2 bound
the best cost `T*`. They do not say that a particular hashed walk
(ecbench\'s `rho.negation`) has the same mean on every curve. That is
what the measurement in #link("README.md")[`README.md`] tests, within
±5%, at 52 bits.

== Proof map
<proof-map>
Declarations are in namespace `DeepGap` unless marked.

- #strong[Thm 2, one edge.]
  - Lean: `TransferBound.transfer`, `TransferBound.transfer_cost`,
    `TransferBound.transfer_answer`.
  - Hypotheses: a `Hom` (injective, preserves `one` and `mul`) and an
    always-correct `Solver`.
  - Status: proved in the 2026-10-10 study.
- #strong[Thm 2, chains.]
  - Lean: `Isogeny.comp`, `comp_cost`, `transfer_comp_cost`,
    `transfer_comp_answer`, `chain_equivalence`.
  - Hypotheses: the same.
  - Status: proved.
- #strong[Thm 3(a), the arithmetic.]
  - Lean: `twist_order_identity`, `twist_order_sq_dvd`, `order_mod`.
  - Hypotheses: integers `p, t, D, f, N` with `t² − 4p = Df²`, `N ∣ f`
    and `N ∣ t + 2`.
  - Status: proved.
- #strong[Thm 3, the class.]
  - Lean: `class_discriminant`, `gap_prime_divides_t_plus_2`,
    `class_twist_order`, `gap_prime_mod_11`.
  - Hypotheses: the numbers of `run/pair.json`.
  - Status: proved by `decide` in the kernel.
- #strong[Thm 3(d), the counts.]
  - Lean: `volcanoSize_closed`, `depth6_size`.
  - Hypotheses: the level-count recursion `1, 3·2^{k−1}`.
  - Status: proved. The recursion itself (the class-number formula for
    inert 2) is not formalized.
- #strong[Thm 1, the curve parts of Thm 3(a)–(c), and Thm 4.]
  - Lean: none. They need elliptic curves over `F_p`, `End`, Vélu and
    the Weil pairing.
  - Status: not formalizable against today\'s Mathlib. `volcano-gap`
    checks them numerically on every curve.

Axiom audit: `lean/axiom-audit.log`. Every declaration uses at most
`propext`, `Classical.choice` and `Quot.sound`.
`tools/lean/proof-gate.sh` passes the project: 18 theorems, 0 trusted.
