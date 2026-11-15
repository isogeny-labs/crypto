# Sub-exponential ECDLP over prime fields: new obstructions, the residual search space, and three falsifiable probes (2026-11-10)

**What this is.** A desk note answering "devise novel ideas for a sub-exponential
ECDLP algorithm on prime-field curves", written against the program's prior
maps: `RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md` (§0–§5),
`research/prime_s5_small_root_leads_20261028/README.md` (lattice facts F1–F6),
`research/ecdlp_subexp_desk_20261101/README.md` (sixteen candidates, all
colliding with the 154-lead atlas) and
`research/prime_field_lead_assessment_20261107/` (160 leads, 27 in `ic-prime`).

**Classification.** Desk evaluation with derivations. **No algorithm is
claimed, nothing is measured, no disposition changes.** Every statement is
labelled `[derived]` (checkable from the text), `[literature]` (a named
source, recalled, not re-read this session), `[proposed]` (an idea or a
prediction) or `[closed on paper]` (an idea this note examined and found to
reduce to a recorded obstruction). Standing of every quoted number from the
repository: `[pcc:unverified]`.

**Sources checked.** The `research-kb` MCP server and the `scripts/research-kb`
CLI were both unavailable in this session (no executable on PATH; sibling
checkout absent), so the knowledge-base review required by AGENTS.md
"Knowledge base first" could not be run; this is recorded as a gap. The
substitute was a keyword census over `RESEARCH_*.md`, `docs/`,
`research/notes/`, the three prime-field assessments above and the S₅
small-root note: of the mechanisms below, "Chevalley", "multisection",
"Mordell–Weil lattice", "Shioda", "Ulmer", "theta group", "Heisenberg",
"Chow group" and "Newt(S_4)" had zero hits; "Kummer map" appears once
(`RESEARCH_ISOGENY_FIBERS_ECDLP.md`, in the ℓ-isogeny sense, not the
descent-map sense used here); "elliptic surface" appears only in the xedni /
function-field-section leads. The lead atlas itself
(`crypto-autoresearcher/knowledge/leads/ecdlp/`) was read through the
2026-11-07 survey, not directly.

---

## 0. Bottom line

1. `[derived]` **One lemma closes every "transfer to a structured group" idea
   at once** (§1). A map `E(F_p) → G(F_p)` into any commutative algebraic
   group `G/F_p` that is given by rational functions of degree `< p/4` and is
   additive on the prime-order subgroup is the restriction of an algebraic
   homomorphism (rigidity + a point count on `E × E`). By Chevalley's
   structure theorem such a homomorphism lands in an abelian variety that has
   `E` as an isogeny factor, so the only transfer targets are Jacobians
   containing `E` (`LEAD-TRF-same-field-genus-g-embedding`: cost `≥ p`) and,
   through the N-torsion, tori whose dimension is at least the embedding
   degree (`LEAD-TRF-mov-small-embedding-degree`). Theta-group / Heisenberg
   representations, Hecke-operator spectra, Chow-group and étale-cohomology
   "regulators", and class-group actions on points are all instances. The
   lemma is theorem-shaped and small enough for `crypto-lean`.
2. `[derived]` **The Kummer (descent) map *is* MOV** (§1.3). Over `F_p` the
   isomorphism `E(F_p)[N] → H¹(F_p, E[N]) ≅ Z/N` is the discrete logarithm
   itself; it is computable in polynomial time exactly when `E[N]` is rational
   over a small extension, i.e. when the embedding degree is small. This
   identifies the whole "cohomological" family with the pairing obstruction.
3. `[closed on paper]` **Elliptic surfaces give nothing new** (§2). For a
   pencil `E_t` with `E_{t₀} = E`, a curve in a linear system `|D|` passing
   through `d` prescribed points of the fibre exists iff those points sum to a
   fixed point `T_D` (restriction to the fibre is the Riemann–Roch space of
   `D|_{E_{t₀}}`), so "find a multisection through factor-base points" is
   Semaev's decomposition in different clothing; two-fibre constructions
   (transfer through a nodal fibre `≅ G_m` or a smooth-order fibre) fail because
   the specialisation kernels at different fibres are unrelated sublattices of
   the Mordell–Weil lattice. Ulmer's unbounded-rank families need generators of
   degree `~ p^n`, i.e. exponential size.
4. `[derived]` **The arity frontier depends on the linear algebra more than
   the program's table says** (§3). With a free `S₄` oracle and *quasi-linear*
   linear algebra the `m = 3` total is `p^{1/3}`, not the `p^{1/2}` tie of
   §2.4 of the program note; with a free `S₃` oracle it is still a `p^{1/2}`
   tie. So the first arity that can beat rho is `m = 3` *if* the relation
   matrix can be made structured, and `m = 4` otherwise. Nobody has a
   structured relation matrix and nobody has an `S₄` oracle at `δ = 1/3`, but
   the two targets are now stated separately, with a falsifier each.
5. `[proposed]` **The residual search space is non-algebraic integer
   arithmetic** (§4): operations on `x ∈ [0, p)` that are not polynomial maps
   over `F_p` and are not lattice reduction. The inventory (smoothness,
   lattices, continued fractions, carries/digits, Hensel, sieving by
   congruence, CRT, Frobenius on lifts, sum–product, fast evaluation) has
   exactly one entry without a recorded obstruction or an open lead: *sieving by
   congruence classes*, which is what makes NFS/FFS relation collection cheap
   and which has no elliptic analogue because factor-base membership is not a
   congruence condition. §5 states the one question that would make it one.
6. `[proposed]` **Three probes** (§6), each cheap and each with a stated
   falsification: (P1) a Lean statement of the rigidity lemma; (P2) an exact
   measurement of whether any `F_p`-affine structure on the decomposition set
   survives (the "congruence sieve" test); (P3) a structured-relation-matrix
   experiment at `m = 2` on toy curves to measure whether the relation graph
   admits a sub-quadratic solve. None of them can produce a sub-exponential
   algorithm by itself; each decides whether a door that is currently
   unexamined is open or shut.

The honest summary is that **the known inventory of non-generic operations
over `F_p` is small and every entry is either closed or an active lead**, and
that this note's contribution is to make the closure of the "transfer" and
"surface" families a theorem rather than a list, and to point to the single
unexamined operation.

---

## 1. The algebraic-transfer rigidity lemma

### 1.1 Statement

Let `E/F_p` be an elliptic curve with `N = #E(F_p)` prime, `G/F_p` a
connected commutative algebraic group, and `f : E → G` a morphism of
`F_p`-varieties (given by rational functions) of degree `d`, with `f(O) = 0`.
Suppose `f(X + Y) = f(X) + f(Y)` for all `X, Y ∈ E(F_p)`.

**Lemma A** `[derived]`. If `d < p/4` (any constant below `1/2` works for
`p` large), then `f` is a homomorphism of algebraic groups.

*Proof sketch.* The map `h : E × E → G`, `h(X, Y) = f(X + Y) − f(X) − f(Y)`,
is a morphism of degree `O(d)` that vanishes on all `N² ≈ p²` points of
`(E × E)(F_p)`. If `h ≠ 0`, its zero locus is a proper closed subset of the
surface `E × E` of degree `O(d)`; by Lang–Weil (or Bézout plus Hasse–Weil on
each component) a curve of degree `D` on `E × E` has at most
`D·(p + 1 + 2√p)` `F_p`-points, so `h` can vanish on at most `O(d·p) < p²`
points once `d < p/4`. Hence `h ≡ 0` and `f` is additive as a morphism; an
additive morphism of algebraic groups with `f(O) = 0` is a homomorphism. ∎

**Corollary B** `[derived; Chevalley, literature]`. Write
`1 → L → G → A → 0` with `L` linear and `A` an abelian variety (Chevalley).
`Hom(E, L) = 0` (a morphism from an abelian variety to a linear group is
constant), so `f` factors through `A` and `f(E) ⊂ A` is an elliptic curve
isogenous to `E`. In particular:

- *Abelian-variety targets.* `E` is an isogeny factor of `A` (Poincaré
  reducibility). A transfer to the Jacobian of a curve `C/F_p` needs `E | J(C)`,
  which is the cover route (`LEAD-TRF-same-field-genus-g-embedding`,
  `LEAD-TRF-cover-catalog-jacobian-transfer`), with index-calculus cost
  `p^{2−2/g} ≥ p` for `g ≥ 2`.
- *Linear targets.* The image of `E(F_p)[N]` in `L(F_p)` under any
  homomorphism is trivial unless `N | #L(F_p)`. For a connected linear group
  over `F_p`, `#L(F_p) = p^u · Π_i (p^{d_i} − ε_i)` with `Σ d_i ≤ dim L`
  `[literature: order formulas for groups of Lie type, recalled]`, so
  `N | #L(F_p)` forces `N = p` (anomalous) or `N | p^{d} − 1` for some
  `d ≤ dim L`, i.e. `dim L ≥ k := ord_N(p)`, the embedding degree. Any
  `F_p`-linear representation of `E(F_p)[N]` that is faithful has dimension
  `≥ k` for the same reason (its eigenvalues are `N`-th roots of unity, which
  live in `F_{p^k}`).

### 1.2 What the lemma closes, in the atlas's words

| idea family | where it lands | obstruction |
|:--|:--|:--|
| theta group / Heisenberg (Schrödinger) representation of `E[N]` acting on `H⁰(E, L)`, `deg L = N`; "read the log from the spectrum of `U_Q = U_P^k`" | linear target, eigenvalues in `μ_N` | dimension `≥ k`; the eigenvalue map is the Weil pairing (MOV) |
| Hecke operators / modular parametrisation `X₀(M) → E` read as an operator spectrum | linear target or `A = J₀(M)` | `E | J₀(M)` is the cover route; spectra need `μ_N` |
| Chow groups `CH₀(S)` of surfaces containing `E` as a fibre, motivic/étale "regulators" | `A = Alb(S)` or `H¹(F_p, E[N])` | §2 and §1.3 |
| correspondences `T ⊂ E × C` acting on divisor classes | `T_* : E → J(C)` is algebraic | `E | J(C)`: cover route |
| class-group action on points, cycles in the isogeny graph | a principal ideal `(α)` acts as the integer `a + b` on `E(F_p)` because `π = 1` there | known scalars only (consistent with `LEAD-STR-class-group-ecdlp-bridge`'s expected closure) |
| "point-count oracles": a variety `V_{P,Q}` whose `#V(F_p)` encodes `k` | an isogeny invariant of an algebraic construction | the graph of `[k]` has degree `k²`; a low-degree construction cannot depend on `k` (the additive form of `LEAD-EXO-lange-winterhof`) |

The hypothesis `d < p/4` is where the lemma is honest about its limits: maps
of degree `≥ p` can be cheap to evaluate (compositions of `x ↦ x^p`), and over
`F_p` every such map agrees on `F_p`-points with one of degree `< 2p` after
reduction modulo `x^p − x, y^p − y`. A complete theorem would have to bound
*straight-line complexity* rather than degree, and the only cheap high-degree
maps over `F_p` are built from the Frobenius, which is the identity on the
points we care about. **Conjecture A′** `[proposed]`: every map
`E(F_p) → G(F_p)` computable by a straight-line program of length
`poly(log p)` over `F_p` that is additive on `E(F_p)[N]` agrees on
`E(F_p)[N]` with an algebraic homomorphism. A counterexample would be a new
primitive; a proof would close the family unconditionally.

### 1.3 The Kummer map is MOV

For `Q ∈ E(F_p)[N]` choose `R ∈ E(F̄_p)` with `[N]R = Q`. The class of
`π(R) − R` in `E[N]/(π − 1)E[N]` is independent of the choice, and since
`ker(π − 1) ∩ E[N] = E(F_p)[N]` has order `N`, the quotient is cyclic of
order `N`. The map `Q ↦ [π(R) − R]` is an injective homomorphism
`E(F_p)[N] → Z/N`, i.e. **the discrete logarithm up to a unit**
`[derived]`. Computing it needs an `N`-division point of `Q`, which lives in
a field of degree up to `N²`, unless `E[N]` itself is rational over a small
field, which is exactly the condition `k = ord_N(p)` small. In `F_p^*` the
same construction with `μ_N ⊂ F_p` is the power-residue symbol
`h^{(p−1)/N}` and runs in polynomial time. So the pairing obstruction is not
one trick among many: it is the statement that `H¹(F_p, E[N])` is
inaccessible when the `N`-torsion is irrational, and every cohomological
reformulation of the ECDLP inherits it.

---

## 2. Elliptic surfaces, multisections and two-fibre transfers `[closed on paper]`

**Set-up.** Let `S → P¹` be an elliptic surface over `F_p` with a section
`O`, fibre class `F`, and a fibre `E_{t₀} = E`. Its Mordell–Weil group
`E(F_p(t))` is finitely generated of rank `r` (Lang–Néron; Shioda–Tate gives
`r ≤ ρ(S) − 2`), and specialisation `σ ↦ σ(t₀)` is a homomorphism
`Z^r → E(F_p)`.

**2.1 Multisection relations.** An irreducible curve `C ⊂ S` with `C·F = d`
meets `E_{t₀}` in `d` points `P₁..P_d`, and the trace of the point
`C ∈ E(F_p(C))` down to `F_p(t)` is a section `tr_C` with
`tr_C(t₀) = Σ P_i` `[derived]`. If one could find, for `d` *prescribed*
factor-base points, a member `C ∈ |D|` through them, the relation
`Σ P_i = Σ_j n_j s_j(t₀)` would express a sum of base points in `r` fixed
unknown logs at the cost of linear algebra in `H⁰(S, D)`. The count is
`h⁰(D) ≈ (D² + d)/2 + 1` on a rational surface, so `|D|` has room for `d`
point conditions. The catch `[derived]`: the restriction
`H⁰(S, D) → H⁰(E_{t₀}, D|_{E_{t₀}})` has image of dimension `≤ d` in a
space of dimension exactly `d` (`D|_{E_{t₀}}` is a degree-`d` line bundle
on an elliptic curve), and a section of a degree-`d` line bundle `L` vanishing
at `P₁..P_d` exists iff `L ≅ O(P₁ + ⋯ + P_d)`, i.e. iff `Σ P_i = T_D` for the
fixed point `T_D` determined by `D`. Members of `|D|` through `d` arbitrary
points exist only when they contain the whole fibre. So "a multisection
through these base points exists" *is* the Semaev condition
`Σ P_i = T_D`, and deciding it is the decomposition problem. The surface adds
no information beyond the fibre's Riemann–Roch space; the Mordell–Weil
coordinates `n_j` only re-encode the target `T_D`.

**2.2 Two fibres.** A curve through prescribed points on `E_{t₀}` *and* on a
second fibre `E_{t₁}` gives two Semaev conditions tied by the same section
`tr_C`: `Σ P_i = tr_C(t₀)`, `Σ Q_j = tr_C(t₁)`. If the DLP is easy on `E_{t₁}`
(a nodal fibre, whose smooth locus is `G_m` with sub-exponential DLP, or a
smooth fibre of smooth order), the integer vector `n` of `tr_C` is the same at
both fibres, so one may hope to read `n` at `t₁` and use it at `t₀`. This fails
`[derived]`: the specialisation kernels `Λ₀, Λ₁ ⊂ Z^r` at the two fibres are
unrelated full-rank sublattices, logs at `t₁` determine `n` only modulo `Λ₁`,
and `n mod Λ₁` says nothing about `Σ n_j s_j(t₀)` modulo `Λ₀`. Finding a
section through a prescribed point of `E_{t₀}` is the open
`LEAD-EXO-global-lift-height` / `LEAD-ICP-xedni-function-field-sections`
question (there, the frozen rational-surface family has lift exponent
`α = 3` against the `α < 1/2` a source would need); the second fibre does not
help find it.

**2.3 Unbounded rank.** Ulmer's families over `F_p(t)` (e.g.
`y² + xy = x³ − t^d`, `d | p^n + 1`) have rank growing with `d`
`[literature, recalled]`, but their explicit generators have degree `~ d`,
their specialisations at `t₀ ∈ F_p` are the thin family `y² + xy = x³ − t₀²`,
and a rank large enough to make the specialisation map cover `E(F_p)` with
small coefficients needs `d` exponential in `log p`. Not a route for a
prescribed curve.

**Why this matters.** The surface/Chow-group picture is the natural place a
geometer would look for "free relations", and it is where `CH₀(S)⁰` being
finite looks like it kills the fibre's `Pic⁰`. §2.1 says the killing is
witnessed only by curves whose existence is the Semaev condition. This closes
the family without an experiment.

---

## 3. The arity frontier with the linear algebra made explicit `[derived]`

The program note (§2.4) prices index calculus at arity `m` as

    cost(B) ≈ (m!/2^m) · p · T_dec / B^{m−1}  +  LA(B),

with `LA(B) = m·B²` (Wiedemann on weight-`m` rows). The table below redoes the
minimisation with `LA(B) = B^{1+ε}` ("structured") next to `B²` ("sparse
random"), for a *free* oracle (`T_dec = 1`), which is the most optimistic
case for the oracle and therefore the right one for asking what the linear
algebra alone can buy.

| m | oracle | optimum `B`, `LA = B²` | total, `LA = B²` | optimum `B`, `LA = Õ(B)` | total, `LA = Õ(B)` |
|--:|:--|:--|:--|:--|:--|
| 2 | free `S₃` | `p^{1/3}` | `p^{2/3}` | `p^{1/2}` | `p^{1/2}` (tie) |
| 3 | free `S₄` | `p^{1/4}` | **`p^{1/2}` (tie)** | `p^{1/3}` | **`p^{1/3}`** |
| 4 | free `S₅` | `p^{1/5}` | `p^{2/5}` | `p^{1/4}` | `p^{1/4}` |

Reading. Nothing at `m = 2` beats rho under any linear algebra (the relation
phase alone is `p/B ≥ p^{1/2}` for `B ≤ p^{1/2}`, and `B > p^{1/2}` makes the
factor base larger than rho's work). At `m = 3`, which the program note
records as "ties only", a quasi-linear solve turns the tie into `p^{1/3}`.
This splits the breakthrough target into two independent sub-targets, each
with its own falsifier:

- **(T1) an `S₄` oracle at `δ = 1/3`** (the lattice reach is `1/24`
  asymptotically and `1/35.5` at feasible dimension per the S₅ note, so this
  is an 8–12× exponent shortfall, less than the 16–63× at `S₅`), *or*
- **(T2) an `S₅` oracle at `δ = 1/5`** with ordinary sparse linear algebra;

and, for (T1), **(T3) a relation-generation scheme whose matrix admits a
sub-quadratic solve.** The relation matrix of index calculus is random-sparse
because the oracle's output (which base points appear) is not under the
attacker's control. (T3) asks whether *some* control exists: e.g. generating
targets as `R + jT` for a fixed `T` and `j` in a range gives rows whose
supports are correlated through the group law, but the correlation is not a
circulant or Toeplitz structure that a transform could use. §6 (P3) proposes
measuring, not assuming, whether the relation graph of a toy `m = 2` run has
any exploitable spectral structure. The expected outcome is "no" (random
sparse), which would close (T3) as a cost-model entry; a "yes" would move the
`m = 3` goalpost from `S₅`-at-`1/5` to `S₄`-at-`1/3`.

None of this is an algorithm. It is a correction to the goal statement: the
program note's "first arity that could beat rho is `m = 4`" assumes `B²`
linear algebra, and that assumption should be recorded as such.

---

## 4. The residual search space: non-algebraic operations on `[0, p)` `[proposed]`

Lemma A and the atlas together say that any sub-exponential method must use
operations on coordinates that are *not* polynomial maps over `F_p` and are
*not* the group law (Shoup). The integers-mod-`p` structure offers a short
list of such operations. Here is the inventory, with the recorded status of
each in this program:

| operation on `x ∈ [0, p)` | what it buys elsewhere | elliptic status |
|:--|:--|:--|
| smoothness of the integer lift | index calculus in `F_p^*`, class groups, NFS | needs a multiplicative lift of the group law; `E(Q)` has rank `r`, not infinitely many generators (xedni, `LEAD-ICP-char0-lifting-xedni-smoothness`; JKSST bound conditional, `α = 3` measured for the frozen family) |
| small roots mod `p` (lattices) | Coppersmith, HNP | reach `≤ 0.354` per `S₃` block against demand `8/9`; F1–F6 of the S₅ note; `LEAD-ICP-coppersmith-smallroot-semaev` |
| continued fractions / rational reconstruction `x ≡ a/b` | 2-dimensional lattice | a case of the row above (F4: degree-1 substitution, same box volume) |
| digit expansion, carries | the "Weil descent over `Z`" idea | `LEAD-ICP-digit-presentation-solving-degree`: first-fall degree curve-independent, reproduced by a curve-free null |
| Hensel / `p`-adic lifting | Smart's anomalous attack | prime-to-`p` torsion lifts to torsion; `LEAD-TRF-padic-canonical-section` (closed, proof) |
| Frobenius on a lift (canonical lift, Witt vectors) | Satoh point counting | same as above; `π = 1` on `E(F_p)` |
| CRT / several moduli | multi-modular arithmetic | there is one modulus; a relation mod `p` lifted to `Z` is F6 (global lifting wall) |
| sum–product / additive energy | Bourgain-type expansion | explains the measured null of `LEAD-ICP-coincidence-harvest-structured-base` (44/44 cells inside the null band); an obstruction, not a lever |
| fast multipoint evaluation of `Π S₃(x_i, x_j, Z)` | batch root finding | `Õ(B)` per target or the `B²` advice trade-off (program note §4.6) |
| **sieving by congruence classes** | the Eratosthenes trick that makes NFS/FFS relation collection cost `O(1)` per candidate instead of a smoothness test | **no elliptic analogue recorded** (see below) |

**The last row.** In NFS, "the norm of `a − bθ` is divisible by `q`" is a
congruence condition `a ≡ b·r_q (mod q)` on the *lattice of candidates*, so
all candidates divisible by `q` are enumerated in arithmetic progressions,
and the smoothness test is replaced by accumulating `log q` over the
progressions. The elliptic analogue would be: a condition on the target `R`
(or on the randomiser `a` in `R = Q + aG`) that is *affine in `a`* and that
raises the probability of `R` decomposing over the factor base. No such
condition is known, and Lemma A gives a reason: the set
`{a : Q + aG ∈ F + F + ⋯}` is the preimage of an algebraic set under the map
`a ↦ Q + aG`, which is not algebraic in `a` (its graph has degree `~ a²`). The
one setting in which an affine structure *does* exist is when the factor base
is itself defined through the scalar, `F = {kG : k ≤ B}`, and that base has
no cheap membership test (it is BSGS). **Question C** `[proposed]`: is there
a factor base `F ⊂ E(F_p)` with `|F| = p^{δ}` such that both (i) membership
costs `poly(log p)` and (ii) the indicator of `{a ∈ [0, N) : Q + aG ∈ F}` has
a non-trivial Fourier coefficient of size `≫ √|F|/N` at a frequency
computable from the curve? (i) alone is the coordinate-box base; (ii) alone is
the scalar base; both together would make an elliptic sieve. The measured
character-sum nulls (`LEAD-EXO-dl-character-sum-pseudorandom`) are the
evidence against (ii) for coordinate boxes at the reachable scale, and
Lemma A plus the Lange–Winterhof interpolation bounds
(`LEAD-EXO-lange-winterhof`) are the theoretical evidence; the probe P2 below
states the exact measurement. The expected outcome is a null.

---

## 5. What a sub-exponential algorithm would have to contain

Putting §1–§4 together, a sub-exponential prime-field ECDLP algorithm must
have one of the following, and each is the only unmeasured object of its
kind in the program:

1. **A non-lattice small-root solver for one structured congruence**
   (`S₄` at box volume `p^{1}`, i.e. `δ = 1/3` in three unknowns, or `S₅` at
   `δ = 1/5`). The S₅ note's F1–F6 say the information available to any such
   solver is the single polynomial `S` and its box; so the solver would have
   to exploit the *roots-are-points* structure in a way no lattice does.
   Nobody has a candidate.
2. **An affine-in-the-scalar structure on decomposability** (Question C).
   Would give a sieve. Expected to be refuted by P2.
3. **A map violating Conjecture A′**: a cheap non-algebraic additive map out
   of `E(F_p)[N]`. Would be a new primitive and would also break the
   generic-group-with-encoding model the program uses for its lower bounds.
4. **A structured relation matrix (T3) together with an `S₄` oracle at
   `δ = 1/3`.** Two independent gaps; closing (T3) alone changes a goalpost,
   not a cost.

Everything else in the program's inventory reduces to one of these or to a
recorded obstruction.

---

## 6. Three falsifiable probes

**P1 — Lean statement of Lemma A** `[proposed; desk]`. Formalise: a morphism
`E × E → G` over a finite field that vanishes on all `F_p`-points and has
degree below a stated bound is zero (the Lang–Weil / Bézout count), and the
rigidity step. `crypto-lean` already has elliptic-curve declarations
(1.7k Lean declarations per AGENTS.md); the count is the part worth checking
because it fixes the constant in `d < p/4`. *Outcome that matters:* a proof
closes §1.2's family with a theorem; a failure to prove the count would mean
the constant is wrong, not that the family is open. Cost: desk, no compute.

**P2 — The congruence-sieve null** `[proposed; toy compute]`. On toy
prime-order curves (`N ≈ 2^{20}..2^{28}`, ≥ 3 primes per size), fix the
coordinate-box base `F_B = {x ≤ B}` with `B = N^{1/3}`, enumerate the
indicator `1_{F_B}(Q + aG)` over all `a ∈ [0, N)`, and compute its full
Fourier transform over `Z/N`. Compare `max_{ξ≠0} |F̂(ξ)|` to the random-set
null `√(|F_B|)` (and to a planted positive control: the scalar base
`{kG : k ≤ B}`, whose indicator is an interval with `|F̂(ξ)| ~ |F_B|/ξ`).
Pre-register: the test statistic is the ratio to the null's mean over 400
size-matched random bases; the lane closes as *exhausted at scale* if the
ratio is within the null band at every size; it opens if the maximum
coefficient's frequency is reproducible across primes. Cost: `O(N log N)`
per cell, minutes at `2^{28}`. This is the exact form of §4's last row and
is not in the atlas as a Fourier measurement (the character-sum lead
measured multiplicative characters of `x`, which is a different transform).

**P3 — Spectral structure of the relation graph** `[proposed; toy compute]`.
From an existing toy `m = 2` index-calculus run (the autolab's 16–24-bit
prime-field sessions already produce relation sets), build the relation
matrix over `F_N`, and measure (a) the time of a structured Gaussian
elimination against Wiedemann, (b) the singular-value profile of a random
block, against an Erdős–Rényi matrix of the same row weight and density.
Pre-register: (T3) closes as *cost model* if no statistic separates from the
random null at `2^{16}, 2^{20}, 2^{24}`; it stays open if a separation grows
with `N`. Cost: CPU-hours, no new engine code. Expected outcome: closure.

Each probe follows `controlled-experiment` (protocol frozen before data,
negative control, positive control, A/A arm) if run; none is run here.

---

## 7. What this note does not do

It proposes no algorithm, runs nothing, and changes no disposition. Lemma A
is `[derived]` and its proof sketch has not been reviewed by anyone else;
the order formula for `#L(F_p)` and Chevalley's theorem are `[literature,
recalled]`. The claim that the operations table in §4 is complete is a claim
about the program's inventory, not about mathematics: a new row would be
exactly the kind of idea this note was asked for and did not find. The
knowledge-base review mandated by AGENTS.md was not possible in this
session (tool unavailable) and should be run before any of P1–P3 is
scheduled, with the queries "Chevalley algebraic group homomorphism ECDLP
transfer", "elliptic surface multisection relation", "Kummer map descent
embedding degree", "sieve congruence elliptic factor base" and their negative
forms.

## 8. Files

- `README.md` — this note.
- `figures/route_closure_map.svg|png` — the route map of §1–§5 (which
  families Lemma A closes, which operations in §4 are closed, open or
  unexamined), written by `plot.py` (pure Python, SVG) from
  `data/routes.json`; it computes nothing. PNG rendered by `render.js`.
- `REPORT.md|html|pdf` — the two-page report (`pandoc REPORT.md -s -o
  REPORT.html && node render.js`; the PDF is printed with the pre-installed
  Chromium because no typst or matplotlib could be installed in this session).
