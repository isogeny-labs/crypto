# Prime-field index calculus: why the needle is not moving, and the only two places it can

**Date.** 2026-11-04. **Status.** Analysis note, no experiment run. Every claim is
labelled **PROVED** (elementary, sketch given), **MEASURED** (quoted from a named
repo file; not re-run here) or **HEURISTIC**. Written after reading the three
repos (`crypto`, `cryptanalysis`, `crypto-autoresearcher`), the lead atlas
summaries, and the ~400 `ECDLP-IDEA-*` dispositions.

## 0. Summary

1. **The summation polynomial is not the bottleneck.** In every prime-field arm
   the repos have measured, the Gröbner degree of the group-law part is at the
   floor (`d_reg = 2`, `FINDING-PF-IC-001`, `EXP-R6-001`), and the tower oracle
   sits at `D = 6–7` (`RESEARCH_PKM_TOWER_ORACLE.md` §§11–16). What costs is
   *membership*: expressing "`x_i` lies in a size-`B` subset of `F_p`". Replacing
   `S_{m+1}` by any lower-degree encoding of addition (bilinear "function
   through `m+1` points", elliptic-code columns, chain/tree `S_3` topologies,
   Edwards/Montgomery forms) leaves that cost untouched. The repo's own
   Edwards-vs-Weierstrass null (`curve25519_edwards_semaev_20261102`, 8/8 equal
   `D`) and shape-matched tower null (same `D` as the elliptic tower) are exactly
   this statement, measured.
2. **Three paradigms exploit the `F_p` representation, and each has an
   elementary obstruction** (§2): algebraic (no fractional-dimensional sets, no
   conjugate equations, binomial density), archimedean/lattice (Coppersmith
   reach is pinned by the degree of the `x`-projection, which is optimal),
   `p`-adic (the reduction `E(Z/p²) → E(F_p)` splits canonically for `p ∤ N`).
3. **Every pointwise-defined factor base turns decomposition into a `k`-SUM
   instance over `Z/N` on a pseudorandom set** (§3). Your measured exponents sit
   on the `k`-SUM line (MITM `0.58–0.66`; algebraic `e ≈ 2.19` at `m = 3`, i.e.
   `B²`). Under the `k`-SUM conjecture no combinatorial, SAT, tensor or
   enumeration oracle beats `B^{⌈m/2⌉}`, and the "top lead" in
   `research/prime_one_shot_20261029/continuation/` (Boolean tensor
   contraction, conditional on the manuscript *The Exponential Time Hypothesis is
   False*) is logically the statement "the `k`-SUM conjecture is false". That
   manuscript is an AI-generated preprint (OpenAI, dated 2026-10-17, revised
   10-25) whose sharpest bound is **not** Lean-checked and whose expert review
   (Impagliazzo, Paturi, Williams) is listed as "starting". It should be filed
   as an unverified external assertion, not a lead to build on.
4. **The Kummer tower and the Boolean/SAT encoding are the same object in two
   bases** (§4): your §2.4 already shows the tower quotient is the group algebra
   `F_p[(Z/2^t)^m]` with square-free monomials = characters. F4 on the tower is
   the Fourier dual of the bit-address encoding. This unifies the two surviving
   leads and gives a quantitative bar: to beat rho at 256 bits the tower solving
   degree must stay **`D ≲ 10` at `N ≈ 256`** (quadratic cost model), and the
   `k`-SUM conjecture forces a slope `D ≳ 0.04·N + c` (so the decisive question
   is whether the measured slope `≈ 0.125–0.15` flattens, not whether it is
   zero).
5. **Proposals** (§6): a rejection filter for the autoresearcher that would have
   killed most of the 255 rejected ideas at proposal time, and one
   discriminating experiment on the tower (radix vs. levels). That experiment
   needs an engine build first: `f4_fp_tower` supports only degree-2 steps
   (checked in code, §6.1). No sub-rho mechanism is claimed.

## 1. Where the cost actually is (MEASURED, quoted)

| arm | file | what was measured | reading |
|:--|:--|:--|:--|
| explicit-membership PDP, `m = 3` | `crypto-autoresearcher` `EXP-R6-001`, `FINDING-PF-IC-001` | `d_reg = 2` in every lever (model, isogeny class, base structure); total exponent `2.05 [1.86, 2.29]` | group-law degree is at the floor; the `|F|`-degree membership quotient is the cost |
| algebraic vs group oracle, `m = 3` | `crypto/research/prime_field_leads_20261031/L2a_slope_e_m3`, rerun 20261101 | per-attempt work `e_S = 2.19 [2.16, 2.22]` (S₄ roots), `e_G = 1.97` (group MITM) | algebraic route = `B²` = the `k`-SUM/MITM bound, times `log p` |
| PKM towers | `RESEARCH_PKM_TOWER_ORACLE.md` §§10–16 | `D`: 3,4,4,4,4,5,5,5,5,6,6 (`m=2`, `t=1..11`); 6,6,7,7 (`m=3`, `N=9..18`); 6,7,7 (`m=4`, `N=8..16`); null control identical | `D` grows slowly; slope undecided; decisive cells need 130–380 GB |
| small roots | `RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md` §2.2; `smallroot_optimal_reach_20261026` | reach `δ ≤ 1/6` at `S₃`, need `1/2`; proved `0.3293` per block vs. demand `0.8889` | closed |
| structured bases | `docs/ic/factor-base-encyclopedia` §7, `KN-FIND-007` | yield ratio `1.000000` over 288/288 cells | bases behave as random subsets of `E(F_p)` |

Nothing in this table is about `S_{m+1}`'s degree. All of it is about membership.

## 2. Three paradigms, three obstructions

Notation: `E/F_p`, `#E(F_p) = N` prime, factor base `F = {P : x(P) ∈ V}`,
`|V| = B = p^δ`, `0 < δ < 1`, arity `m`.

### 2.1 Algebraic

**L1 (no fractional dimension, PROVED).** Any `F_p`-variety `W` has
`#W(F_p) = p^{dim W}(1 + o(1))` (Lang–Weil). Hence a subset of `F_p` of size
`p^δ`, `0 < δ < 1`, is not the point set of a positive-dimensional variety, and
as a 0-dimensional scheme its vanishing polynomial has degree `B`. Over
`F_{q^n}` the subfield `F_q` is the point set of the 0-dim scheme `x^q − x`, but
with the *additive* Frobenius structure; `F_p` has no subfield. This is the whole
reason Gaudry/Diem descent exists for `F_{q^n}` and not for `F_p`.

**L2 (no conjugate equations, PROVED).** Weil descent gets its `n` equations in
`m` unknowns from the ring endomorphism `σ: x ↦ x^q` of `F_{q^n}[x]`, which
fixes `F_q` pointwise, has degree 1 after descent, and transports
`S_{m+1}(x, x_R) = 0` to `S_{m+1}(x, x_R^σ) = 0`. Over `F_p`, let `σ` be any
`F_p`-algebra endomorphism of `F_p[x]` with `σ(v) = v` for all `v ∈ V`. Then
`σ(x) − x` vanishes on `V`, so `σ(x) = x + L_V(x)·h(x)` with `deg L_V = B`;
either `σ = id` or `deg σ ≥ B`. Transporting a relation through a non-identity
`σ` costs degree `≥ B` — the same as naive membership. Curve endomorphisms do not
help either: on a prime-order curve every `φ ∈ End(E)` acts on `E(F_p)` as a
scalar, so a `φ`-stable `V` is a union of scalar orbits (membership = DLP) unless
`φ` is an automorphism (`|Aut| ≤ 6`, the GLV constant already in the ledger).
Adding the *consequence* `S_{m+1}(x([n]·)(x), x_{[n]R}) = 0` is valid for any `V`
but lies in the radical of `⟨S⟩` and cuts no dimension; it cannot play the role
of a conjugate equation.

**L3 (binomial density, PROVED).** For `B | p − 1`, membership `c − x ∈ μ_B`
reads `(c − x)^B = 1`, and `(c − x)^B = Σ_j C(B,j) c^{B−j}(−x)^j` has all `B+1`
terms nonzero mod `p` (Lucas: `B < p`). Over `F_{p^n}` with `B = p^s`,
`(c − x)^{p^s} = c^{p^s} − x^{p^s}` is 2-sparse. This is the elementary form of
L2: the only additive power maps are Frobenius powers, and over `F_p` those are
the identity. Every "exotic" subset of the Handbook (cyclotomic classes, Gauss
periods, Dickson/torus images, permutation- or exceptional-polynomial images,
linearized polynomials, Drinfeld/Carlitz actions, which are `F_p`-linear only
over extensions where `τ = x^p ≠ id`) reduces to a tower of degree-`d` maps of
height `log_d B`, i.e. to the PKM presentation. There is no fourth presentation.

**Corollary (HEURISTIC, standard).** The same statement in additive-combinatorics
form is Bourgain–Katz–Tao: no `A ⊂ F_p` with `|A| = p^δ` is simultaneously
additively and multiplicatively structured. The factor base must be
`F_p`-structured (for membership) while relations are `E`-structured; the two
never coincide on a set of fractional size.

### 2.2 Archimedean (Coppersmith / small roots)

The `x`-coordinate is the unique degree-2 function on `E` up to affine change;
any degree-`d` coordinate gives a summation polynomial of degree `d^{m−1}` or
worse. Hence the Coppersmith reach `δ_max = 1/(m(m+1)2^{m−2})` (your table) is
not an artefact of the presentation; a bilinear "function through `m+1` points"
encoding only moves the degree into unknowns (`y_i`, function coefficients) that
are not small and must be eliminated back to `S_{m+1}`. Side remark: with small
`a, b` and *all* points small, `S_3(x) = j·p` with `|j| ≤ 10 p^{4δ−1}`; the
surface `S_3 = 0` is the Kummer surface of `E × E`, whose integer points in a box
are the reductions of collinear triples on quadratic twists over `Q` (finitely
many, Siegel). So the archimedean route is closed from both ends: too few global
relations below `δ = 1/4`, Coppersmith reach too small above it.

### 2.3 `p`-adic

For `p ∤ N`, Hensel gives `E(Z/p²) ≅ E(F_p) × F_p` **canonically** (the
prime-to-`p` torsion of `E(Z_p)` maps isomorphically onto `E(F_p)`). The map
`ψ(P) = [N]·L(P)` for the integer-representative lift `L` is a quasi-homomorphism
with defect `N·c(P,Q)`, `c` the coboundary of `L`; the defect vanishes mod `p`
iff `N = p` (Smart). Jets, formal groups, canonical lifts and high-rank
Mordell–Weil lifts (`E(K)` for multiquadratic `K` has rank `≈ 2^{r−1}`, so every
`P` has a lift of height `≈ p^{2/rank}`, but finding it is a knapsack under an
unknown linear functional, and relations are not forced because
`ĥ(P̃_1 + P̃_2 − P̃_3) ≤ 9·max ĥ` exceeds the kernel-of-reduction floor
`(2/[K:Q]) log p` by a dimension-free factor) all die here. Consistent with
`RESEARCH_IC_LEADS_AND_CONNECTIONS_20261010.md` §7c.

## 3. The `k`-SUM framing (HEURISTIC, but it fits every number you have)

Fix a pointwise-defined `V`. Write `ℓ: F → Z/N` for the (unknown) logs. Because
`F` is `F_p`-structured and `E(F_p) ≅ Z/N` is `E`-structured, `ℓ(F)` is a
pseudorandom subset of `Z/N` of size `B` (this is what "yield depends only on
`|F|`, 288/288" measures). Decomposing a target is then exactly `m`-SUM over
`Z/N` on a pseudorandom set, with the extra twist that the oracle may use the
`F_p`-coordinates. Known combinatorial bounds: `B^{⌈m/2⌉}` (MITM), conjecturally
optimal for pseudorandom sets (3SUM / `k`-SUM conjecture). Consequences:

* `m = 2`: `B` per target, `B = p^{1/2}`: the root-finding oracle (`x(R ∓ F_i)`),
  `S_family = 0.75·r^{1/6}` above rho — measured.
* `m = 3`: `B²` per target; measured `e = 1.97–2.19`.
* `m = 5, 7` MITM: exponents `0.60, 0.58` — the `N^{max(h, m−h+1)/m}` law in the
  `index_calculus/README.md` is the `k`-SUM bound with the linear-algebra term.
* Beating rho at arity `m` needs per-target cost `B^{m/2−1}`, i.e. a factor `B`
  below the `k`-SUM bound. **Every proposal must say where that factor comes
  from.** The three paradigms of §2 are the only known sources of
  representation-dependent structure, and each is closed.

The one solver family to which the `k`-SUM bound does *not* obviously apply is
an algebraic solver with degree falls (F4/F5), because its cost is not a function
of the set alone but of the presentation. That is precisely why the tower slope
is the live question, and why no SAT/CDCL(T)/tensor route can be a second live
question unless `k`-SUM is false.

**Slope floor.** With cost `≈ (#columns)^ω` and columns `= Σ_{k≤D} C(N,k)` at
`B^m ≈ 2^N`, the `k`-SUM bound `2^{N/2}` forces `H(D/N) ≥ 1/(2ω)`:
`D/N ≳ 0.035` (`ω = 2.37`) or `≳ 0.07` (`ω = 2`). Your measured line
(`D = 3 → 6` over `N = 2 → 22`) has slope `≈ 0.14`, above the floor, so nothing
measured contradicts `k`-SUM. **Beat-rho condition.** At 256 bits with `m = 3`,
`N ≈ 256 + log₂ 3! ≈ 259`; `(Σ_{k≤D} C(259,k))² < 2^{128}` needs `D ≤ 10`. So
the tower wins only if `D` stops growing somewhere below 10, which the current
data (`D = 7` at `N = 18`, one flat step at `m = 3`, one at `m = 4`) neither
supports nor refutes. The registered conjecture `d_F = 4 + ⌊(t−2)/4⌋` would give
`D ≈ 36` at `t = 130`: cost `≈ 2^{330}`, dead.

## 4. Unification: tower F4 = Fourier dual of the Boolean encoding (PROVED, from your §2.4)

For the Kummer tower `V = g·μ_{2^t}`, `R = F_p[y]/(towers) ≅ F_p[(Z/2^t)^m]`, and
the square-free monomial `∏_j y_{i,j}^{b_j}` is the character `k ↦ ζ^{k·B}` with
`B = Σ b_j 2^j`. Hence:

* the multilinear degree of a function on `V^m` = max popcount of its Fourier
  support;
* the coupling `S_{m+1}(g ζ^{k_1}, …, x_R)` has Fourier support in the box
  `[0, 2^{m−1}]^m` (degree `≤ 2^{m−1}` in each `x_i`);
* multiplication by `y_{i,j}` is "add `2^j` to coordinate `i`, with carry";
* F4's degree-`D` row space is the span of carry-shifts of `spec(S)` by
  frequencies of popcount `≤ D − deg S`;
* the solution is the delta at the address `k*`, whose spectrum is full.

The bit-address (Boolean/SAT/tensor) encoding uses the same `N` bits `k` as
variables. So the tower lead and the tensor lead are the *same* instance, one in
the position basis, one in the frequency basis. Whatever is true of one is true
of the other up to the solver. (Note on why `D ≥ N` is **not** forced: the
Nullstellensatz cofactor `(y − s)/S` is a generic function of multilinear degree
`≈ N`, but F4's working degree bounds only the ring degree of handled rows, not
of accumulated cofactors; degree falls through `y_{i,j}² → y_{i,j+1} + c` are
exactly what keep `D` small. This is the same gap as XL-degree vs F4-degree in
binary descent.)

## 5. A rejection filter (for `ideas/validate_ideas.py`)

Reject at proposal time, without a run, any `ECDLP-IDEA` for prime fields that
does not name which of the following it evades and by what mechanism:

1. **L1** — the factor base is a pointwise `F_p`-subset of size `p^δ`;
2. **L2/L3** — membership is presented by a tower (or by degree-`B` polynomials);
3. **`k`-SUM** — the oracle is combinatorial (enumeration, MITM, hashing, LSH,
   ISD/decoding, sieving, SAT, tensor contraction) on `ℓ(F)`;
4. **`p`-adic splitting** — the lift map is a homomorphism `E(Z/p^k) → E(F_p)` with
   `p ∤ N`;
5. **Height parallelogram** — the lift is global (`E(Q)`, `E(K)`) and relations are
   not forced.

By my reading this filter catches every family in `rejected/README.md`
("supplied source", "aggregates lose labels", "scalar orientation = DLP",
"homomorphism kills signal", "reduces to same dreg/support", "no dense supply",
"charged cost > rho") as special cases, and it catches them before the 17
mandatory headings are written. It also makes explicit that the only
non-filtered proposal class is *an algebraic solver whose working degree on the
tower stays bounded*.

## 6. Proposals

### 6.1 Radix-vs-levels discriminator (needs an engine extension, then cheap)

**Question.** Is `D` governed by the number of tower levels `t` (variables) or
by the information `log₂|V|` (bits)? The `k`-SUM framing predicts the latter: a
radix-3 tower has `t' = t·log₃2 ≈ 0.63 t` variables for the same `|V|`, so the
`(#columns)^ω` cost at the *same* `D` would be lower; `k`-SUM says the saving
must be paid back by a larger `D`. If instead `D` tracks `t`, mixed-radix towers
are a real polynomial-factor win and §3's cost model is wrong in your favour.

**Design.** Kummer family only (exact group-algebra structure). Primes with
`2^a 3^b | p − 1` at toy size (`p < 2^32`, same engine limits as rounds 2–7).
Cells: `m = 2`, `|V| ∈ {2^6, 2^12, 2^18}` (`= 3^{3.8}, 3^{7.6}, 3^{11.4}`; use
the nearest `2^a 3^b` with equal `|V|` up to a factor 2, matched across arms).
Arms: radix 2 (`t = log₂|V|`), radix 3 (`t = log₃|V|`), mixed (`2,2,3,…`).
Same planted-target protocol as §10.1, 3 primes × 2 targets per cell, shape-matched
null per arm. **Output:** `D`, widest matrix, multiply-adds. **Pre-registered
prediction (mine):** `D_radix3 ≥ D_radix2 + 1` at equal `|V|`, with multiply-adds
equal within a factor 2. **Falsifier:** `D_radix3 = D_radix2` and multiply-adds
lower by ≥ 2× at `|V| = 2^18`.

**Build required (checked 2026-11-04).** The design note lists `degrees=2,2,3`
as a parameter (`RESEARCH_PKM_TOWER_ORACLE.md` §2.1, "Stage A covers them only
at the end"). The engine never implemented it:
- `src/cryptanalysis/f4_fp_tower.rs` has a single rewrite type, `SquareRule`
  (`y_j² → a·y_{j+1} + …`);
- `Mono = u128` is a square-free bitmask (one bit per variable), and degree is
  popcount.

A radix-3 step `y_j³ = y_{j+1}` needs exponents in `{0,1,2}`. That means a
2-bit-per-variable monomial (64 variables in a `u128`, enough for `N ≤ 40`), a
`CubeRule`, carry-on-overflow in `mul_var_into` and the normal form, and degree
as an exponent sum rather than popcount. The same extension gives the
radix-4 control for free: `y^4 = y_next` is the radix-2 tower with every
other level eliminated. The two presentations generate the same ideal, so
comparing them separates "variables" from "bits" without changing the
factor base at all.

Pass gate before any cell: the radix-4 presentation reproduces the radix-2
solution sets on every committed round-2 system (`D` may differ; solutions may
not). Under `AGENTS.md` §3 this is engineering until §6.1's falsifier fires.

**Expected value, stated honestly.** Under §3 the prediction is null. A
positive result would be a polynomial-factor win at fixed `|V|`, not a change
of exponent. Spend on this only if the cheaper decisive cell (Kummer `m=2`,
`t=14`, cap 6, 112 GiB, `prime_fall_degree_t14_20261101`) is blocked for longer
than the build takes.

### 6.2 Withdrawn: frequency-indexed row storage

An earlier draft proposed spectral row storage to bring the `m=4, N=20` cell
onto a small host. The code and round 3 rule it out:
- `Row` is already sparse (`cols: Vec<u32>, vals: Vec<u32>`), and inputs are
  sparse.
- The memory is in the *reduced basis*. Round 3 measured "a new element's tail
  is dense over the step's columns without a divisor": 274M terms after
  step 48 at `N = 16`, kept since round 3 as dense echelon rows at 4 B/entry
  (`RESEARCH_PKM_TOWER_ORACLE.md` §12.1).
- Elimination fill-in is the cost, and changing the basis in which input rows
  are written does not touch it.

### 6.3 What I would *not* spend compute on

* Any further "exotic presentation" of membership (Handbook chapters 2.1.6, 5.3,
  8, 9.6, 13.5): all reduce to towers by L3.
* CDCL(T)/SAT at `m = 3` as a *scientific* arm: bounded by `k`-SUM; keep only as
  an engineering baseline.
* Transfers (covers, Jacobians, extension fields, twists, isogeny classes):
  factor bases cannot be smaller than a curve's point count (`≈ p`) by L1, so the
  cost is `≥ p^{1}`; the ledger's `p^{4/3}` and `(2,2)`-gluing results are
  instances of this.
* Building on the tensor/ETH route until the Impagliazzo–Paturi–Williams review
  reports. If that manuscript survives, the right move is not to transcribe it
  but to note that `k`-SUM falls with it and re-open §5 item 3 wholesale.

## 7. What this note does not claim

No sub-rho mechanism, no new exponent, no measurement. L1–L3 and the `p`-adic
splitting are elementary and I believe correct as stated; the `k`-SUM framing is
a conjecture-level heuristic used to organise evidence, not a theorem; §6.1's
prediction is mine and is falsifiable by the design given. Independent review
under `review-breakthrough` is not needed because nothing here is a breakthrough;
`review-run` applies to §6.1 once it has data.

## Sources (external)

* Petit, Kosters, Messeng, *Algebraic approaches for the ECDLP over prime fields*,
  PKC 2016 — https://christophe.petit.web.ulb.be/files/16PKC_primeECDLP.pdf
* Galbraith, Gaudry, *Recent progress on the ECDLP*, 2015 — https://eprint.iacr.org/2015/1022.pdf
* Amadori, Pintore, Sala, *On the discrete logarithm problem for prime-field elliptic curves*,
  FFA 2018 / https://eprint.iacr.org/2017/609.pdf
* *Implementation analysis of index calculus on elliptic curves over prime fields*,
  https://eprint.iacr.org/2024/1923
* ETH-manuscript status (secondary, AI-assisted press): https://1023jack.com/news/openai-says-its-ai-proved-the-exponential-time-hypothesis-false-here-is-what-is/
