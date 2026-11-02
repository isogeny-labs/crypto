# Endomorphism-invariant factor bases across curve families: plan, pilot, experiments E1–E18, and the road to the state of the art

**Modules:** `src/cryptanalysis/glv_invariant_base.rs` (the fold, prime-field automorphisms, Vélu degree-2 and degree-3 endomorphisms, CM instance generators, the folded rho classes), `src/cryptanalysis/ext_curve.rs` (`ExtField` over `F_{p²}` and `F_{p³}`, the generic `ExtCurve` counted group, diagonal automorphisms and Frobenius-type maps on it), `src/cryptanalysis/gls_fp2.rs` (GLS `ψ`, the `ψ`-stable line, the `j = 0` and `j = 1728` twists with their lifted automorphisms), `src/cryptanalysis/subfield_fp3.rs` (`E/F_p` on `E(F_{p³})`, the Frobenius eigenline), `src/cryptanalysis/line_oracle.rs` (the Weil-descent resultant oracle for a line, E2b), `src/cryptanalysis/glv_invariant_experiments.rs` (one relation stream feeding both arms to full rank), `src/cryptanalysis/ic_framework/plugins.rs` (`glv-orbit`, `gls-line`), `src/cryptanalysis/ic_boundary.rs` (`FactorBase::from_column_map`), `src/cryptanalysis/orbit_pair_table.rs` (the pair table over orbit representatives, E12), `src/cryptanalysis/fghr_line.rs` (the `Y`-line, the `τ_T` fold and the `D₃` conic-resultant oracle, E13), `src/cryptanalysis/q_curve.rs` (Q-curves of degree 2 and 3 over `F_{p²}` and `ψ = π ∘ ι ∘ φ`, E14)
**CLI:** `ic bench --bits 20 --family j0 --factor-base glv-orbit:size=64 --oracle mitm:negation_folded=1` (control: `glv-orbit:size=64,no_fold=1`)
**Bench (pilot):** `cargo run --release --example glv_invariant_bench -- --families j0,j1728,generic,d7,d8 --bits 16,20,24 --seeds 2 --oracles subtract,mitm --json experiments/23_glv_invariant_pilot.json`; `--families gls --bits 8,10,12 --oracles subtract --json experiments/23_glv_invariant_gls_pilot.json`
**Runner (E1–E7):** `cargo run --release --example glv_invariant_experiments -- --exp e1 --bits 16,20,24,28 --seeds 6 --json experiments/23_glv_invariant_e1.json` (the exact command of every file is its `command` field)
**Data:** `experiments/23_glv_invariant_pilot.{json,log}`, `experiments/23_glv_invariant_gls_pilot.{json,log}` (2026-09-28), `experiments/23_glv_invariant_e{1,1_32,2,3,4,5,6,7}.{json,log}` (2026-09-29), `experiments/23_glv_invariant_e{8,9,11,11_12,11_13,11_14}.{json,log}` (2026-10-01), `experiments/23_glv_invariant_e{12,12p,13,13_13,14,15}.{json,log}` (2026-10-02), `experiments/23_glv_invariant_e13_p{14,15,16}.{json,log}` (2026-10-08); this host: Linux x86-64, 4 threads; wall time is recorded and is not a result
**Tables:** `python3 scripts/glv_invariant_tables.py experiments/23_glv_invariant_pilot.json experiments/23_glv_invariant_gls_pilot.json` (§5; legacy Python, not extended) and `cargo run --release --example glv_invariant_experiment_tables -- experiments/23_glv_invariant_e*.json` (§6, §8; the native replacement of the retired `scripts/glv_invariant_experiment_tables.py`, byte-identical output on E1–E11); every number in §5, §6 and §8 is printed by them from the frozen files

> **Status.**  Implementation, pilot, and the seven experiments of §4
> run and read (§6).  The fold is one function over the framework's
> `CountedGroup`, verified end to end on `F_p`, `F_{p²}` and `F_{p³}`
> groups; the pilot (§5) is at `2^13`–`2^24` with two seeds per size,
> the experiments (§6) at `2^13`–`2^32` with four to six seeds, both
> arms fed one relation stream to full rank.  What they establish: the
> fold is exact (every row recovers its planted logarithm); the column
> count is the orbit count; after `columns` relations both arms reach
> the same rank fraction, so the relation count moves with its floor
> and nothing else moves; the fold by **any** finite group of
> endomorphisms is the order of the subgroup of `(Z/rZ)^*` its
> eigenvalues generate (`12` on the `j = 0` GLS twist, `4` — not `8` —
> on the `j = 1728` twist, `3` — not `9` — on the `j = 0` subfield
> curve); the degree-2 and degree-3 CM endomorphisms keep **zero** base
> points in the base at eigenvalue orders of `10²`–`10⁸`; and the rho
> folded by the same group takes `√(w/2)` fewer steps, verified.
> Class, where anything moved: **engineering** (§6.8); nothing moved a
> ratio to a floor, and no scoreboard row claims a speed.  §8 places
> this against the literature (Galbraith–Granger–Merz–Petit's invariant
> bases, Faugère–Gaudry–Huot–Renault's system symmetries), ranks what
> is next, and runs the first three: three summands on the Frobenius
> line (E8: the fold's relation count falls with the columns once the
> two-summand degeneracy of §6.5 is gone, and every phase is priced in
> one unit beside the matched rho), the folded rho on the `F_{p²}` and
> `F_{p³}` groups (E9), the fitted exponents (E10), and the algebraic
> `S₄` oracle on the line (E11: flat in `p`, ~~the first arm of this note
> whose fitted exponent sits below rho's `1/2`~~ — corrected in §8.10: its
> fit is `0.53`, and on these groups rho's `1/2` is the best a line base can
> reach — and still not faster than rho at any size run), then the pair table over orbit representatives
> (E12: the table `÷ w/2`, worth `S ÷ 1.18` where the table is the cost,
> `F_p` `j = 0`, and nothing on the line) and FGHR's `2`-torsion symmetry
> with the fold (E13: a degree-`16` resultant in place of a `64`-dimensional
> quotient, `S ÷ 27`, and the same `T` folding the base `12` a column for
> `S ÷ 2.5` more; the two compound, and the best arm is `267×` rho),
> and the transfer check to the ECC2K-130 family (E15: on `E_0` at the
> challenge's `4·prime` shape the fold is unconfined, `D = 1`; on
> `m = 31`, which cannot have that shape, the `373` cofactor confines
> `73 %` of two-summand rows), and Q-curves of degree 2 and 3 over
> `F_{p²}` (E14: every `ψ` verified, eigenvalue orders `≥ 1817`, base
> points kept at chance — no fold).  E16 derives the boundary: on these
> groups `r ≈ p²`, so rho costs `Θ(p)` and a line base needs `Θ(p)`
> relations — a constant at best, never an exponent; the best arm's ratio
> to rho grows as `r^{0.18}` to `p = 2^{16}`, and two earlier exponent
> claims are corrected.  Every class is engineering or accounting; no row
> claims a speed.  §9 sets goal G2 — the cheapest levers, moved to curves where index calculus
> has rho's exponent or better — and runs it: E17 carries the `D₃` solve (`÷ 38`) and the `τ_T`
> fold to the full group `E(F_{p³})`; E18 folds the `k = 4` linear algebra, where the `ψ` fold on
> `j = 0` curves takes the asymptotic ratio to the matched rho from `1.23` (plain) to `0.138` and
> the extrapolated handover from `2^{156.1}` to `2^{146.0}`, while the `τ_T` fold cannot pay for
> its curves' cofactor and yield.  Both are engineering; §11.19's `0.518` reads `0.733` against the
> negation walk.

## 1. What is new, against what exists

The repository already folds a factor base by an endomorphism in three
places, each written for one family:

| where | family | fold | what it established |
|:--|:--|:--|:--|
| `koblitz_index_calculus.rs`, `ic_boundary::koblitz_factor_base`, `koblitz-orbit` | Koblitz over `F_{2^n}` | signed Frobenius orbit, `2n` points a column, coefficient `±λ^k` | the ledger's one *advance* row: an orbit base needs `2n` fewer relations (`RESEARCH_IC_BOUNDARY_LEDGER.md`) |
| `glv_gaudry.rs`, `OrbitBase::fold_terms` | `j = 0` over `F_{p³}` with the subspace base | `⟨ψ⟩` orbit, `3` abscissae a column | `S ÷ 3.0` at every size with `residuals / floor ≈ 1.0`: engineering (`RESEARCH_GLV_INDEX_CALCULUS.md` §3) |
| `residual_walk.rs`, `Fold::Automorphism` | `j = 0` over `F_p` | sixfold residual fold | the prime-field sixfold of `RESEARCH_RESIDUAL_WALKS.md` §10.3 |
| `ec_index_calculus_j0.rs` | `j = 0` over `F_p` | orbit representatives only | documents that `ψ`-shifted rows are `λ^k` multiples of one row (no rank) |
| `cryptanalysis` repository, `prime_orbit_index_calculus.rs` | `j = 0`, `j = 1728`, generic over `F_p` | `Aut(E)` orbit, gain-graph solver | the `w²/4` coverage argument for pair sums |

What none of them is: **one fold, over any group the framework
counts, for any finite list of endomorphisms**, with the control the
same call.  That is what `fold_by_endomorphisms` is.  A family adds an
`Endomorphism` (a name, a degree, the eigenvalue `λ` modulo `r`, the
map on points) and a seed set; the fold closes the seed under the maps
or refuses a set that is not closed, walks the orbits, checks that two
paths to one point carry the same `Π λ`, and returns a
`FactorBase` with one column per orbit.  `verify_endomorphism` checks
every constructor's output on random points before it is used.  With
that in place the families this note covers are:

| type | family | endomorphism | `deg` | `ord_r(λ)` | invariant base | points a column | status |
|:--|:--|:--|--:|--:|:--|--:|:--|
| A | `j = 0`, `p ≡ 1 (3)` | `ψ(x, y) = (ζx, y)` | 1 | 3 | closure of any abscissa set under `x ↦ ζx` | 6 | implemented, `glv-orbit` |
| A | `j = 1728`, `p ≡ 1 (4)` | `ι(x, y) = (−x, iy)`, `ι² = −1` | 1 | 4 | closure under `x ↦ −x` | 4 | implemented, `glv-orbit` |
| A | generic | negation | 1 | 2 | any | 2 | implemented (the control family) |
| B | Koblitz, `F_{2^n}` | `τ` | 2 | `n` | `τ`-stable `F_2`-subspaces | `2n` | existing, `koblitz-orbit` |
| B | GLS twist over `F_{p²}` | `ψ = τ_u π_p τ_u⁻¹`, `ψ² = −1` | `p` | 4 | the line `x ∈ u·s·F_p` (`s² = ν`) | 4 | implemented, `gls-line` |
| B | subfield curve `E/F_p` on `E(F_{p^n})` | `π_p` | `p` | `n` | `π`-stable `F_p`-subspaces of `F_{p^n}` not inside `F_p` | `2n` | **pending** (§4, E5) |
| A×B | `j = 1728` twisted over `F_{p²}` (FourQ-shaped) | `⟨ι, ψ⟩` | — | 8 | a line stable under both | 8 | **pending** (E3); the fold call takes both generators as is |
| C | `D = −7` (`j = −3375`), `D = −8` (`j = 8000`) | degree-2 by Vélu, `φ² − tφ + 2 = 0`, `t ∈ {±1}`, `0` | 2 | `10³`–`10⁶` measured | **none** inside `⟨G⟩` (§2) | — | implemented as a measurement: `velu_degree2_endomorphisms`, `endomorphism_overlap` |
| C | `1 + i` on `j = 1728` | Vélu through `(0, 0)`, trace `±2` | 2 | large | none | — | implemented, verified in tests |
| C | degree 3 (`D = −11`; `√−3` on `j = 0`), Q-curves (Smith) | 3-isogeny by Vélu; `ψ² = ±d` | 3, `dp` | large | none | — | **pending** (E4b) |

The one derivation the module rests on, stated once: relations are
written over `[h]P`, and `[h]φ(P) = φ([h]P) = [λ][h]P`, so
`log [h]φ(P) = λ · log [h]P` for every base point, in or out of `⟨G⟩`.
The framework's cofactor curves (`h` up to 8 in the pilot) fold by the
same code path as the prime-order ones, and the test
`the_fold_is_sound_on_a_cofactor_curve` holds it to that.

## 2. Boundaries, stated before measuring

- **The fold is bounded by the orbit length, and the orbit length by
  `ord_r(λ)`.**  A base inside `⟨G⟩` that is stable under `φ = [λ]` is
  a union of orbits of multiplication by `λ` on `Z/rZ`, each of length
  `ord_r(λ)`.  For an automorphism of order `w` that is `w` (up to the
  fixed points, which are torsion); for the Frobenius on `E(F_{q^n})`
  it is `n`; for a degree-`d ≥ 2` CM endomorphism `λ` is a root of
  `λ² − tλ + d` modulo `r` and its order divides `r − 1` with nothing
  forcing it small.  So an invariant base under a type-C map is either
  essentially all of `⟨G⟩` or empty, and the most a type-C map can
  offer a base of `|F|` points is the chance overlap `|F|² / #E` of
  free relations `log φ(P) = λ log P`.  That is the boundary for E4,
  and the pilot measures it (§5.3).
- **Relation count.**  A square system needs about `columns` relations,
  hence `columns / ρ` targets at decomposition rate `ρ`; both arms see
  the same `ρ` because they hold the same points.  The fold divides
  `columns` by `w/2` (`w` the automorphism group's order), so the floor
  divides with it.  An *advance* would be `relations / columns < 0.9`
  on the folded arm with the control at `≈ 1.0`; the same ratio on
  both is the count moving with its bound (AGENTS.md §3).  One
  qualification the pilot forced into the open: the framework's
  relation loop stops when the incremental matrix first determines the
  target's logarithm, which on a two-summand base is the first cycle of
  the relation graph, at about half the columns; so what the pilot
  compares is relations-to-first-determination, not relations-to-full
  rank, and the ratio scatters around `w/2` from seed to seed.  E1's
  protocol fixes the stopping rule (§4).
- **Reference.**  A counted Pollard rho with the negation map
  (`rho_reference_negation`, `A = 2`) on the same instance and target,
  eight walks per row in the pilot.  The matched reference for a curve
  with `Aut` of order `w` is a rho that folds by all of `Aut`
  (`√(w/2)` cheaper; `aut_folded_rho.rs` has the `j = 0` walk in
  BigInt arithmetic, the framework does not have it in counted form):
  every "vs rho" figure in this note is therefore against the
  `A = 2` walk and understates the gap by up to `√3` on `j = 0` and
  `√2` on `j = 1728` and GLS.  Marked as such wherever it appears.
- **Falsification targets.**  E1/E2: an advance requires
  `relations / columns` to fall by more than the fold on the folded
  arm; the expectation, and the pilot's reading, is that it does not.
  E4: an overlap above three times the chance fraction on any family
  at any size would contradict §2's first boundary and would be the
  interesting result.  E3: fewer than `8` points a column on the
  `⟨ι, ψ⟩` line, or a fold that does not verify, falsifies the
  composite construction.  Inadmissible: changing the point set between
  the arms (the control **is** the folded arm's points), changing the
  oracle or the stopping rule between arms, dropping the base build or
  the table from `S`, counting `λ^k`-multiples of one row as
  independent, or a row whose answer is not the planted logarithm.

## 3. What the implementation checks so that a wrong number cannot get through

- Every constructor's map goes through `verify_endomorphism`: image
  on the curve, additive on random pairs, `φ([k]G) = [λ][k]G` on
  random `k`.  A perturbed eigenvalue is caught
  (`a_wrong_eigenvalue_is_caught_by_verification`).
- Instance orders are certified, never assumed: the CM candidate
  orders come from Cornacchia (`4p = t² + |D|v²`), the one that kills
  three random points is taken, and `[r]G = O` is checked on the
  generator.  Generic instances count points in `O(p)`.
- The fold refuses what it cannot fold, by name: a base a generator
  maps outside (`Strict`), a base point in a kernel, an orbit past
  `max_orbit` (an infinite-order generator), and two paths to one
  point that disagree on `Π λ`.  The type-C test asks for exactly that
  refusal.
- The GLS derivation is checked, not trusted: `ψ² = −1` on random
  points of the whole twist, `ψ_x(u s t) = −u s t` for every `t`, and
  the other `ψ`-stable line `u · F_p` carrying at most the 2-torsion.
- The shared relation loop recovers the planted logarithm over the
  folded base and over the control on `j = 0`, `j = 1728`, generic,
  `D = −7` (cofactor 4) and GLS
  (`a_folded_base_recovers_the_planted_logarithm`,
  `the_line_base_folds_four_to_one_and_recovers_the_logarithm`).

## 4. The plan

Every experiment below is a framework sweep: the same instance, the
same planted targets, the same seeds, the folded base and its control,
every phase inside `S`, a counted rho beside it.  Sizes are at least
four per family so an exponent can be fitted (AGENTS.md §5), seeds at
least six so the rho reference has its own spread
(`RESEARCH_GLV_INDEX_CALCULUS.md` §3 needed 48 walks to settle it).

| id | question | families | sizes | arms | measure | prediction | status |
|:--|:--|:--|:--|:--|:--|:--|:--|
| **E1** | Does the automorphism fold move anything but the column count on `F_p`? | `j0`, `j1728`, `generic` | `2^16`–`2^32`, six seeds | `glv-orbit` vs `no_fold`, oracles `subtract` and `mitm` | relations to **full rank** on one target stream (a second matrix fed the same rows until every column is pinned; the loop's first-determination count kept beside it), `S`, `S / rho` | `relations / columns ≈ 1.0` on both arms; `S` ratio `→ w/2` only where the relation phase dominates and `→ 1.0` where the pair table does (§5.2) | **done** (§6.1): column ratio exactly `3`, `2`; same rank fraction after `columns` relations on both arms at every size to `2^32`; engineering |
| **E2** | Does the GLS line fold match the Koblitz orbit fold in the same unit? | `gls` at `p = 2^8`–`2^16`; Koblitz `n = 13`–`31` from the ledger | six seeds | `gls-line` vs `no_fold`; `koblitz-orbit` vs `no_fold` | column ratio, relations, `S`, `S / rho`; the `descent-algebraic` analogue for `F_{p²}` (E2b) | `2×` on GLS against `n×` on Koblitz: the fold is the eigenvalue order, `4` against `2n`, and nothing else | **done** (§6.2): `4` and `2n` points a column, column ratios `2` and `n`, one driver; engineering |
| **E2b** | An `O(1)` decomposition oracle for the line: Weil-descend `S₃(u s t₁, u s t₂, x_R)` to two `F_p` equations in `(t₁, t₂)`, resultant, roots by `gcd(t^p − t, ·)` | `gls` | as E2 | `subtract` vs the resultant oracle on the same base | oracle cost per target, hit rate agreement target by target (AGENTS.md §6 cross-check) | same hits, `O(p)` fewer group operations per target; the fold ratio unchanged | **done** (§6.2): agrees with `subtract` on every target, `O(10³)` `F_p` multiplications a target against `|F|` additions; engineering |
| **E3** | Does a composite group fold as its order says? | `j = 1728` twisted over `F_{p²}` (`⟨ι, ψ⟩`, order 8); `j = 0` twisted over `F_{p²}` (`⟨ψ₃, ψ⟩`, order 12) | `p = 2^8`–`2^14` | the line stable under both vs negation | points a column, verification, `S` | `8` and `12` points a column; engineering | **done** (§6.3): `12` on `j = 0`; `4`, not `8`, on `j = 1728` (`ι = ±ψ` on `⟨G⟩`); the fold is the order of the eigenvalue subgroup of `(Z/rZ)^*` |
| **E4** | How much of a base does a type-C map keep in the base? | `d7`, `d8`, `1 + i` on `j1728` | `2^16`–`2^28` | one base per size, every rational degree-2 map | `ord_r(λ)`, `images_in_base / base_points` against `|F| / #E` | chance level; zero at pilot sizes | **done** (§6.4): `0` of `43,684` base points at `2^16`–`2^28`; accounting |
| **E4b** | The same for degree 3 (`D = −11`, `√−3` on `j = 0`) and for a Q-curve of degree 2 or 3 over `F_{p²}` | as named | as E4 | as E4 | chance level | **done for degree 3** (§6.4): `0` of `47,520` base points on `D = −11` and `j = 0`; Q-curves **not done** (§7) |
| **E5** | Subfield curves on `E(F_{p^n})`: a `π`-stable subspace not inside `F_p` | `E/F_p` on `E(F_{p³})`, with and without `j = 0` | `p = 2^6`–`2^11` | `⟨π⟩`, `⟨ψ⟩`, `⟨π, ψ⟩` folds vs negation on one base | points a column (`3`, `3`, `9`), relations, `S` | multiplicative; `glv_gaudry`'s `3.0×` is the `⟨ψ⟩` row | **done** (§6.5): `6` a column for `⟨−1, π⟩` (generic and `j = 0`) and for `⟨−1, π, ζ⟩` alike — `ζ` is a power of `π` on `⟨G⟩`, so not `9`; the `j = 0` line is the subgroup itself and its descent degenerates |
| **E6** | The matched reference | all folding families | as E1 | rho folded by the same `Aut` (counted) beside the `A = 2` walk | `S / rho_folded` | the fold's `w/2` in the relation count against rho's `√(w/2)` in steps: the gap widens by `√(w/2)` | **done** (§6.6): steps ratio `1.72` on `j = 0` (expected `1.73`), `1.51` on `j = 1728` (expected `1.41`), 384 walks, every answer verified |
| **E7** | Does the fold interact with a three-summand oracle? | `j0` | `2^18`–`2^30` | `mitm` at `m = 3` and the `S₄` oracle on folded vs control | relations, solver calls, rows that fold to `0 = 0` (the pair-generator finding of `RESEARCH_GLV_INDEX_CALCULUS.md` §4) | canonicalisation saves nothing on a uniform stream and everything on a pair sieve | **done** (§6.7): fold unchanged at `m = 3`; `0` rows fold to `0 = 0`; orbit duplicates at the birthday count |

What every experiment owes on completion, per AGENTS.md: the frozen
JSON under `experiments/23_glv_invariant_<id>.json`, the table printed
by `scripts/glv_invariant_tables.py`, the class of every row, the
scoreboard row, and — for anything claimed as a speedup — the frozen
regression suite of §8.

## 5. Pilot

Two seeds per size, `bits ∈ {16, 20, 24}` for the prime families
(`r` between `2^12.8` and `2^23.9`, cofactors up to 8) and
`p ∈ {2^8, 2^10, 2^12}` for GLS (`r` up to `2^22.9`); base seeded with
`max(8, r^{1/3})` abscissae and closed under the group (so `3×` as
many abscissae on `j = 0`, `2×` on `j = 1728`); GLS base the whole
line (`p − 1` abscissae); targets from the guarded r-adding walk;
oracles `subtract` (no table, `|F|` additions a target) and `mitm`
(pair table, negation-folded); eight `A = 2` rho walks per row.  Square
roots and Legendre symbols in the base build are counted and unpriced
(no pinned ratio for a generated curve, as in `ic bench`).  Every row
recovered its planted logarithm.

### 5.1 The fold, by family and oracle

| family | oracle | instances | column ratio | trial ratio (min–max) | S ratio (min–max) | S fold / rho S (min–max) |
|:--|:--|--:|--:|--:|--:|--:|
| d7 | mitm | 6 | 1.0 | 1.00–1.00 | 1.00–1.00 | 7×–24× |
| d7 | subtract | 6 | 1.0 | 1.00–1.00 | 1.00–1.00 | 36×–2718× |
| d8 | mitm | 6 | 1.0 | 1.00–1.00 | 1.00–1.00 | 6×–24× |
| d8 | subtract | 6 | 1.0 | 1.00–1.00 | 1.00–1.00 | 64×–3227× |
| generic | mitm | 6 | 1.0 | 1.00–1.00 | 1.00–1.00 | 5×–24× |
| generic | subtract | 6 | 1.0 | 1.00–1.00 | 1.00–1.00 | 54×–4250× |
| gls | subtract | 6 | 2.0 | 1.31–2.04 | 1.31–1.98 | 40×–796× |
| j0 | mitm | 6 | 3.0 | 1.00–2.66 | 1.00–1.02 | 21×–81× |
| j0 | subtract | 6 | 3.0 | 1.00–2.66 | 1.00–2.65 | 18×–1328× |
| j1728 | mitm | 6 | 2.0 | 1.00–4.74 | 1.00–1.13 | 13×–94× |
| j1728 | subtract | 6 | 2.0 | 1.00–4.74 | 1.00–3.82 | 16×–2834× |

The control families (`generic`, `d7`, `d8`) fold by negation on both
arms, so their two arms are the same run: a ratio of exactly `1.00` on
every row is the check that the harness changes nothing but the
column map.

The folding families, row by row:

| family | log2 r | h | oracle | signed points | columns fold | columns control | column ratio | relations fold | relations control | trials fold | trials control | S fold | S control | S control / S fold | rho S (A = 2) | S fold / rho S | correct |
|:--|--:|--:|:--|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|:--|
| gls | 13.4 | 5 | subtract | 248 | 62 | 124 | 2.00 | 44 | 69 | 97 | 157 | 169.9 | 267.0 | 1.57 | 1.92 | 88× | yes |
| gls | 15.3 | 1 | subtract | 180 | 45 | 90 | 2.00 | 29 | 57 | 89 | 182 | 69.6 | 137.9 | 1.98 | 1.75 | 40× | yes |
| gls | 16.1 | 13 | subtract | 952 | 238 | 476 | 2.00 | 93 | 114 | 222 | 290 | 594.8 | 780.6 | 1.31 | 1.62 | 368× | yes |
| gls | 17.0 | 4 | subtract | 704 | 176 | 352 | 2.00 | 97 | 172 | 253 | 445 | 366.1 | 635.5 | 1.74 | 2.15 | 170× | yes |
| gls | 22.9 | 1 | subtract | 2748 | 687 | 1374 | 2.00 | 403 | 783 | 1112 | 2072 | 821.5 | 1512.6 | 1.84 | 1.03 | 796× | yes |
| gls | 22.9 | 1 | subtract | 2816 | 704 | 1408 | 2.00 | 428 | 864 | 1094 | 2169 | 802.4 | 1577.3 | 1.97 | 1.17 | 683× | yes |
| j0 | 13.8 | 3 | mitm | 144 | 24 | 72 | 3.00 | 16 | 34 | 72 | 156 | 52.0 | 52.9 | 1.02 | 2.48 | 21× | yes |
| j0 | 15.7 | 1 | mitm | 222 | 37 | 111 | 3.00 | 17 | 17 | 49 | 49 | 56.9 | 56.9 | 1.00 | 2.27 | 25× | yes |
| j0 | 18.0 | 3 | mitm | 378 | 63 | 189 | 3.00 | 32 | 87 | 354 | 941 | 75.0 | 76.4 | 1.02 | 1.56 | 48× | yes |
| j0 | 19.7 | 1 | mitm | 564 | 94 | 282 | 3.00 | 66 | 157 | 340 | 845 | 88.3 | 89.0 | 1.01 | 1.24 | 71× | yes |
| j0 | 20.5 | 7 | mitm | 678 | 113 | 339 | 3.00 | 79 | 132 | 3560 | 5597 | 101.8 | 104.0 | 1.02 | 1.49 | 69× | yes |
| j0 | 22.3 | 3 | mitm | 1032 | 172 | 516 | 3.00 | 86 | 224 | 2611 | 6245 | 119.9 | 121.6 | 1.01 | 1.47 | 81× | yes |
| j0 | 13.8 | 3 | subtract | 144 | 24 | 72 | 3.00 | 16 | 34 | 72 | 156 | 81.4 | 168.5 | 2.07 | 2.48 | 33× | yes |
| j0 | 15.7 | 1 | subtract | 222 | 37 | 111 | 3.00 | 17 | 17 | 49 | 49 | 40.0 | 40.0 | 1.00 | 2.27 | 18× | yes |
| j0 | 18.0 | 3 | subtract | 378 | 63 | 189 | 3.00 | 32 | 87 | 354 | 941 | 250.3 | 663.9 | 2.65 | 1.56 | 160× | yes |
| j0 | 19.7 | 1 | subtract | 564 | 94 | 282 | 3.00 | 66 | 157 | 340 | 845 | 182.0 | 456.5 | 2.51 | 1.24 | 147× | yes |
| j0 | 20.5 | 7 | subtract | 678 | 113 | 339 | 3.00 | 79 | 132 | 3560 | 5597 | 1972.6 | 3095.1 | 1.57 | 1.49 | 1328× | yes |
| j0 | 22.3 | 3 | subtract | 1032 | 172 | 516 | 3.00 | 86 | 224 | 2611 | 6245 | 1159.7 | 2769.0 | 2.39 | 1.47 | 788× | yes |
| j1728 | 13.3 | 4 | mitm | 84 | 21 | 42 | 2.00 | 12 | 12 | 153 | 153 | 27.5 | 27.5 | 1.00 | 2.07 | 13× | yes |
| j1728 | 13.8 | 4 | mitm | 96 | 24 | 48 | 2.00 | 3 | 13 | 31 | 147 | 27.1 | 29.2 | 1.07 | 1.89 | 14× | yes |
| j1728 | 16.6 | 8 | mitm | 180 | 45 | 90 | 2.00 | 26 | 50 | 1584 | 2481 | 37.5 | 41.1 | 1.10 | 1.33 | 28× | yes |
| j1728 | 17.9 | 4 | mitm | 244 | 61 | 122 | 2.00 | 16 | 61 | 604 | 2227 | 35.0 | 39.6 | 1.13 | 1.92 | 18× | yes |
| j1728 | 20.9 | 8 | mitm | 496 | 124 | 248 | 2.00 | 79 | 120 | 7944 | 11651 | 52.1 | 55.2 | 1.06 | 1.05 | 50× | yes |
| j1728 | 23.0 | 2 | mitm | 808 | 202 | 404 | 2.00 | 126 | 207 | 6483 | 10828 | 59.8 | 61.4 | 1.03 | 0.63 | 94× | yes |
| j1728 | 13.3 | 4 | subtract | 84 | 21 | 42 | 2.00 | 12 | 12 | 153 | 153 | 130.2 | 130.2 | 1.00 | 2.07 | 63× | yes |
| j1728 | 13.8 | 4 | subtract | 96 | 24 | 48 | 2.00 | 3 | 13 | 31 | 147 | 30.7 | 117.3 | 3.82 | 1.89 | 16× | yes |
| j1728 | 16.6 | 8 | subtract | 180 | 45 | 90 | 2.00 | 26 | 50 | 1584 | 2481 | 913.0 | 1417.7 | 1.55 | 1.33 | 685× | yes |
| j1728 | 17.9 | 4 | subtract | 244 | 61 | 122 | 2.00 | 16 | 61 | 604 | 2227 | 302.1 | 1100.6 | 3.64 | 1.92 | 158× | yes |
| j1728 | 20.9 | 8 | subtract | 496 | 124 | 248 | 2.00 | 79 | 120 | 7944 | 11651 | 2811.1 | 4116.2 | 1.46 | 1.05 | 2675× | yes |
| j1728 | 23.0 | 2 | subtract | 808 | 202 | 404 | 2.00 | 126 | 207 | 6483 | 10828 | 1792.9 | 2994.5 | 1.67 | 0.63 | 2834× | yes |

### 5.2 Reading the pilot

- **The column fold is exact and the points are the same.**  `6.0`,
  `4.0`, `4.0` and `2.0` signed points a column on `j = 0`, `j = 1728`,
  GLS and the controls, on every row, with the folded and control arms
  holding the same point set (the harness's `signed points` column is
  one number per row).
- **Relations track columns, with the stopping rule's scatter.**  The
  loop stops at the first determination of the target's logarithm,
  which on a two-summand base arrives at about half the columns
  (relations `16`–`428` against columns `24`–`704` on the folded arms).
  The ratio of relations control-to-fold is `1.0`–`3.8` on `j = 1728`
  and `1.0`–`2.7` on `j = 0`, around the column ratios `2` and `3`;
  the rows at `1.00` (`j0 15.7`, `j1728 13.3`) are runs where the same
  early target determined the logarithm on both arms.  This is why E1
  asks for relations to full rank on one stream: the first-cycle count
  is what the framework measures for a whole pipeline, and the
  full-rank count is what the fold's `w/2` is a statement about.
- **`S` moves only where the relation phase is the cost.**  With
  `subtract` (`|F|` additions a target) the relation phase is
  `> 97 %` of `S` and the `S` ratio is the trial ratio, `1.5`–`2.7` on
  `j = 0` and `1.5`–`3.8` on `j = 1728`.  With `mitm` the pair table
  is `85`–`99 %` of `S` at these sizes, both arms build the same table
  from the same points, and the `S` ratio is `1.00`–`1.13`: the fold
  saves trials that cost one lookup each.  Neither is a speedup claim;
  both are the same engineering fold seen through two oracles, and the
  phase table in `scripts/glv_invariant_tables.py` shows where each
  `S` sits.
- **Against rho, everything is far behind**, `5×`–`4,250×` on the
  `A = 2` walk, and by `√(w/2)` more against the matched folded walk
  (E6, extrapolation).  `S` grows with `r` on every family as the
  two-summand method must; no exponent is fitted on two seeds at three
  sizes, and none is claimed.

### 5.3 Type C, measured

| family | log2 r | rational degree-2 maps | ord_r(λ) | images in base | base points | chance fraction | verified |
|:--|--:|--:|--:|--:|--:|--:|:--|
| d7 | 12.8 | 4 | 6910 | 0 | 38 | 0.00069 | yes |
| d7 | 12.8 | 4 | 3554 | 0 | 38 | 0.00067 | yes |
| d7 | 17.5 | 4 | 30135 | 0 | 112 | 0.00015 | yes |
| d7 | 17.6 | 4 | 64002 | 0 | 114 | 0.00015 | yes |
| d7 | 21.5 | 4 | 243126 | 0 | 284 | 0.00002 | yes |
| d7 | 21.7 | 4 | 3424060 | 0 | 300 | 0.00002 | yes |
| d8 | 13.2 | 2 | 4613 | 0 | 40 | 0.00072 | yes |
| d8 | 14.9 | 2 | 14861 | 0 | 60 | 0.00101 | yes |
| d8 | 17.1 | 2 | 137218 | 0 | 102 | 0.00012 | yes |
| d8 | 17.9 | 2 | 33840 | 0 | 122 | 0.00013 | yes |
| d8 | 21.3 | 2 | 2545240 | 0 | 272 | 0.00002 | yes |
| d8 | 22.5 | 2 | 2969588 | 0 | 362 | 0.00003 | yes |

Vélu through the rational 2-torsion finds the degree-2 endomorphism on
every `D = −7` and `D = −8` instance (four maps on `D = −7`: two
kernels, each with `±φ`; two on `D = −8`), each verified as
`[λ]` on `⟨G⟩` with `λ² − tλ + 2 ≡ 0`, `t = ±1` and `0`.  The
eigenvalue order is `10³`–`10⁶` — comparable to `r` — and **no** base
point maps into the base at any size, against an expectation of
`0.00002`–`0.001` of them.  Both readings are §2's first boundary
holding: a type-C map has no invariant base to offer and no free
relations worth harvesting, and `fold_by_endomorphisms` refuses it by
name (`degree_two_cm_endomorphisms_exist_verify_and_do_not_fold`).
The `1 + i` map on `j = 1728` is the same object next to a type-A
automorphism: the automorphism folds, its degree-2 relative does not
(`one_plus_i_is_a_degree_two_endomorphism_of_a_j1728_curve`).

### 5.4 Verdict, for the pilot

| lever | what moved | class | test against the boundary |
|:--|:--|:--|:--|
| automorphism fold, `F_p` (`j = 0`, `j = 1728`) | columns `÷ 3`, `÷ 2` on the same points; relations and trials with them, with first-determination scatter; `S ÷ 1.5–3.8` under `subtract`, `÷ 1.0–1.1` under `mitm` | engineering | count moves with its floor; `18×`–`2,834×` the `A = 2` rho |
| GLS line fold, `F_{p²}` | columns `÷ 2`; relations `÷ 1.2–2.0`; `S ÷ 1.3–2.0` | engineering | the eigenvalue order `4` against Koblitz's `2n`: the same fold, a smaller group |
| degree-2 CM maps | `0` base points kept in the base, `ord_r(λ) ∈ [3.5·10³, 3.4·10⁶]` | accounting | the first boundary of §2 holds at every size |
| the unified fold itself | one function for five families; the control is the same call | engineering | — |

## 6. Experiments E1–E7: what was run and what it says

**Runner:** `cargo run --release --example glv_invariant_experiments -- --exp e<k> …` (the exact
commands are in each file's `command` field).  **Data:** `experiments/23_glv_invariant_e1.json`
(`2^16`–`2^28`, six seeds), `…_e1_32.json` (`2^32`, two seeds), `…_e2.json`, `…_e3.json`,
`…_e4.json`, `…_e5.json`, `…_e6.json`, `…_e7.json`, each with its `.log`.  **Tables:**
`cargo run --release --example glv_invariant_experiment_tables -- experiments/23_glv_invariant_e*.json`
(native; it replaced the Python script these tables were first printed by, with identical output); every
number below is printed by it.  Host: this container (Linux x86-64, 4 threads); wall time is
recorded in the files and is not a result.

### 6.0 Two things the runs taught the driver before they taught anything else

- **Full rank has a structural ceiling on cofactor curves.**  A two-summand relation
  `R = εP + ε'Q` with `R ∈ ⟨G⟩` forces the `E[h]`-components of `P` and `Q` to be negatives
  (`εu + ε'u' = 0`), so every odd function `χ` on `E[h]` — with `χ(gu) = λ_g χ(u)` on a folded
  arm — is a functional `Σ χ(u_i)x_i` no row can ever determine.  The rank saturates `D` short
  of `columns + 1`, `D` the number of classes of components that are not their own negatives
  (`glv_invariant_experiments.rs`, header).  The logarithm is pinned all the same, which is why
  the pilot never saw it.  The driver computes `D` from `[r]P` for every base point and calls a
  matrix **full** at `rank = columns + 1 − D`; the tables carry `D` for both arms.  Three-summand
  relations carry no such constraint.
- **Full rank is a coupon-collector count.**  The last column has to be touched and its component
  connected, so "relations to full rank" carries a `ln(columns)` factor on both arms and the ratio
  between arms overshoots the column ratio by the ratio of those logarithms.  The size-matched
  reading is the **rank fraction reached after exactly `columns` relations**; it is reported
  beside the full-rank count and is the number to read the floor against.  The framework's own
  first-determination count (the pilot's) is kept as well.
- **On a subfield curve the cofactor is fixed by the Frobenius.**  `E(F_p) ⊂ E(F_{p³})` is
  cofactor, `π` fixes it, so two-summand relations on the Frobenius line pair a point with its
  own orbit and the matrix is block-diagonal by `E(F_p)`-component on both arms (§6.5).  The
  driver reports single-column rows and `D` per arm so this reads off the table.

### 6.1 E1 — the automorphism fold at full rank

**Summary by family, oracle and size (means over seeds; per-row table: the tables binary, 106 rows)**

| family | oracle | log2 r (mean) | seeds | column ratio | full-rank ratio mean (min–max) | full-rank rel / cols, fold | full-rank rel / cols, control | rank fraction at k = cols, fold | control | first-pin ratio mean | all correct |
|:--|:--|--:|--:|--:|--:|--:|--:|--:|--:|--:|:--|
| generic | mitm | 14.4 | 6 | 1.0 | 1.00 (1.00–1.00) | 1.94 | 1.94 | 0.84 | 0.84 | 1.00 | yes |
| generic | mitm | 18.4 | 6 | 1.0 | 1.00 (1.00–1.00) | 2.54 | 2.54 | 0.85 | 0.85 | 1.00 | yes |
| generic | mitm | 22.6 | 6 | 1.0 | 1.00 (1.00–1.00) | 3.14 | 3.14 | 0.83 | 0.83 | 1.00 | yes |
| generic | subtract | 14.4 | 6 | 1.0 | 1.00 (1.00–1.00) | 1.94 | 1.94 | 0.84 | 0.84 | 1.00 | yes |
| generic | subtract | 18.4 | 6 | 1.0 | 1.00 (1.00–1.00) | 2.54 | 2.54 | 0.85 | 0.85 | 1.00 | yes |
| j0 | mitm | 14.2 | 6 | 3.0 | 5.48 (2.23–8.88) | 2.17 | 3.47 | 0.79 | 0.84 | 2.63 | yes |
| j0 | mitm | 18.0 | 6 | 3.0 | 4.24 (3.05–5.77) | 2.10 | 2.83 | 0.82 | 0.84 | 4.20 | yes |
| j0 | mitm | 21.9 | 6 | 3.0 | 2.83 (2.35–3.76) | 3.30 | 3.03 | 0.83 | 0.85 | 2.95 | yes |
| j0 | mitm | 25.5 | 6 | 3.0 | 3.34 (2.73–4.12) | 3.48 | 3.83 | 0.84 | 0.84 | 2.43 | yes |
| j0 | mitm | 29.5 | 2 | 3.0 | 3.46 (3.15–3.77) | 3.53 | 4.07 | 0.84 | 0.84 | 2.50 | yes |
| j0 | subtract | 14.2 | 6 | 3.0 | 5.48 (2.23–8.88) | 2.17 | 3.47 | 0.79 | 0.84 | 2.63 | yes |
| j0 | subtract | 18.0 | 6 | 3.0 | 4.65 (3.39–5.77) | 2.00 | 2.98 | 0.82 | 0.84 | 4.17 | yes |
| j1728 | mitm | 13.4 | 6 | 2.0 | 2.22 (1.63–4.03) | 2.73 | 2.79 | 0.79 | 0.79 | 2.94 | yes |
| j1728 | mitm | 17.7 | 6 | 2.0 | 2.96 (1.25–4.07) | 2.73 | 3.51 | 0.81 | 0.81 | 2.27 | yes |
| j1728 | mitm | 21.8 | 6 | 2.0 | 2.41 (1.45–3.14) | 2.77 | 3.23 | 0.84 | 0.83 | 1.97 | yes |
| j1728 | mitm | 26.0 | 6 | 2.0 | 2.76 (2.32–3.09) | 3.12 | 4.30 | 0.84 | 0.84 | 1.98 | yes |
| j1728 | mitm | 30.6 | 2 | 2.0 | 2.63 (1.97–3.30) | 3.17 | 4.04 | 0.83 | 0.83 | 1.95 | yes |
| j1728 | subtract | 13.4 | 6 | 2.0 | 2.22 (1.63–4.03) | 2.73 | 2.79 | 0.79 | 0.79 | 2.94 | yes |
| j1728 | subtract | 17.7 | 6 | 2.0 | 2.96 (1.25–4.07) | 2.73 | 3.51 | 0.81 | 0.81 | 2.27 | yes |


Reading.  The column ratio is exactly `3` on `j = 0` and `2` on `j = 1728` at every size and
seed, and `1` on the generic control where the two arms are the same run.  After `columns`
relations both arms sit at the same rank fraction (folded and control within a few percent of
each other at every size), so the fold changes the number of unknowns and nothing about how fast
relations fill them: the count moves with its floor.  Relations to full rank fall by more than the
column ratio (the `ln(columns)` factor), and the first-determination count by about the column
ratio with the scatter the pilot showed.  `subtract` and `mitm` see the same relation stream
except where a target has more than one decomposition and the two oracles return different ones
(`2^18` on `j = 0`: `4.65` against `4.24`), so the oracle changes the cost of a target and little
else.
**Class: engineering**, as predicted; no `ratio-to-floor` fell.

### 6.2 E2 — GLS line against Koblitz orbit, in one driver

**Summary by family and eigenvalue order (per-row table: the tables binary)**

| family | eigenvalue order | instances | log2 r | points per column | column ratio | full-rank ratio mean (min–max) | rank fraction at k = cols, fold / control | oracle F_p muls / call | agreement with subtract | all correct |
|:--|--:|--:|:--|--:|--:|--:|--:|--:|:--|:--|
| gls | 4 | 18 | 9.3–24.0 | 4.0 | 2.0 | 2.20 (1.31–3.41) | 0.82 / 0.84 | 2606 | 3600/3600 (0 disagree) | yes |
| koblitz | 13 | 6 | 11.0–11.0 | 25.8 | 12.92 | — (—–—) | 0.45 / — | — | — | yes |
| koblitz | 15 | 6 | 7.7–7.7 | 19.4 | 9.8 | 12.00 (9.50–13.50) | 1.00 / — | — | — | yes |
| koblitz | 17 | 6 | 16.0–16.0 | 29.3 | 14.71 | 25.80 (17.11–36.22) | 0.90 / 0.85 | — | — | yes |
| koblitz | 23 | 6 | 22.0–22.0 | 45.0 | 22.52 | 45.69 (25.45–88.58) | 0.88 / 0.84 | — | — | yes |
| koblitz | 31 | 6 | 20.5–20.5 | 59.0 | 29.5 | 54.33 (28.09–83.70) | 0.70 / 0.58 | — | — | yes |


Reading.  On the GLS twist the fold is `4` points a column and the column ratio `2`; on the
Koblitz curves it is up to `2n` points a column (`19`–`59` on average, the abscissae in a proper
subfield having shorter orbits) and the column ratio `n` less that shortfall — `ord_r(λ)` in both
cases, which is the whole content of "Frobenius-type".  Both arms sit at the same rank fraction
after `columns` relations on the GLS twist (`0.82` against `0.84`) and on Koblitz `n = 17, 23`
(`0.90` / `0.85`, `0.88` / `0.84`); on `n = 31` the folded arm is ahead (`0.70` / `0.58`).  The
line oracle (E2b) agrees with `subtract` on every one of `3,600` targets across the small
instances and costs about `2,600` `F_p` multiplications a target where `subtract` costs
`|F| ≈ p` group additions, so the GLS arm runs at `p = 2^12` (`r ≈ 2^24`) in seconds.  On the
Koblitz arm only degrees whose `x^n − 1` has a factor of intermediate degree have a base of the
right size (`n = 15, 17, 23, 31`); `n = 13`'s only divisor gives a `2,003`-column base on a
group of order `2^11`, and its stream exhausts the group's targets before either arm reaches
full rank, so its row carries the fold (`25.8` a column) and no ratio.  The Koblitz controls
carry the coupon-collector factor of a `500`–`1,000`-column base, which is why their full-rank
ratios (`12`–`54`) exceed `n`.  **Class: engineering** on both arms.

### 6.3 E3 — composite groups on twisted CM curves

**Every instance**

| family | p | log2 r | h | ord λ_ψ | ord λ_aut | aut = ±ψ on ⟨G⟩ | pts/col negation | pts/col ψ | pts/col aut | pts/col ψ+aut | cols ψ+aut | square ratio ψ+aut vs negation | square ratio ψ vs negation | correct |
|:--|--:|--:|--:|--:|--:|:--|--:|--:|--:|--:|--:|--:|--:|:--|
| j0 | 2^8 | 13.8 | 4 | 4 | 3 | False | 2.0 | 4.0 | 6.0 | 12.0 | 20 | 10.33 | 2.10 | yes |
| j0 | 2^8 | 15.2 | 1 | 4 | 3 | False | 2.0 | 4.0 | 6.0 | 12.0 | 20 | 10.63 | 2.30 | yes |
| j0 | 2^8 | 12.7 | 4 | 4 | 3 | False | 2.0 | 4.0 | 6.0 | 12.0 | 15 | 5.27 | 2.30 | yes |
| j0 | 2^8 | 11.0 | 25 | 4 | 3 | False | 2.0 | 4.0 | 6.0 | 12.0 | 16 | 13.18 | 1.07 | yes |
| j0 | 2^10 | 16.5 | 4 | 4 | 3 | False | 2.0 | 4.0 | 6.0 | 12.0 | 54 | 10.82 | 2.77 | yes |
| j0 | 2^10 | 17.1 | 4 | 4 | 3 | False | 2.0 | 4.0 | 6.0 | 12.0 | 63 | 3.99 | 1.24 | yes |
| j0 | 2^10 | 17.6 | 4 | 4 | 3 | False | 2.0 | 4.0 | 6.0 | 12.0 | 73 | 12.72 | 1.90 | yes |
| j0 | 2^10 | 18.0 | 4 | 4 | 3 | False | 2.0 | 4.0 | 6.0 | 12.0 | 84 | 5.70 | 2.60 | yes |
| j0 | 2^12 | 19.2 | 13 | 4 | 3 | False | 2.0 | 4.0 | 6.0 | 12.0 | 240 | 7.14 | 1.76 | yes |
| j0 | 2^12 | 20.0 | 13 | 4 | 3 | False | 2.0 | 4.0 | 6.0 | 12.0 | 309 | 8.77 | 2.20 | yes |
| j0 | 2^12 | 19.8 | 13 | 4 | 3 | False | 2.0 | 4.0 | 6.0 | 12.0 | 273 | 7.95 | 1.85 | yes |
| j0 | 2^12 | 23.0 | 1 | 4 | 3 | False | 2.0 | 4.0 | 6.0 | 12.0 | 236 | 6.34 | 1.99 | yes |
| j1728 | 2^8 | 6.8 | 340 | 4 | 4 | True | 2.0 | 4.0 | 4.0 | 4.0 | 56 | — | — | folds only (r < 16·columns) |
| j1728 | 2^8 | 6.7 | 324 | 4 | 4 | True | 2.0 | 4.0 | 4.0 | 4.0 | 50 | — | — | folds only (r < 16·columns) |
| j1728 | 2^8 | 6.7 | 324 | 4 | 4 | True | 2.0 | 4.0 | 4.0 | 4.0 | 50 | — | — | folds only (r < 16·columns) |
| j1728 | 2^8 | 6.2 | 340 | 4 | 4 | True | 2.0 | 4.0 | 4.0 | 4.0 | 36 | — | — | folds only (r < 16·columns) |
| j1728 | 2^10 | 8.4 | 1220 | 4 | 4 | True | 2.0 | 4.0 | 4.0 | 4.0 | 174 | — | — | folds only (r < 16·columns) |
| j1728 | 2^10 | 6.6 | 9860 | 4 | 4 | True | 2.0 | 4.0 | 4.0 | 4.0 | 240 | — | — | folds only (r < 16·columns) |
| j1728 | 2^10 | 8.8 | 1780 | 4 | 4 | True | 2.0 | 4.0 | 4.0 | 4.0 | 216 | — | — | folds only (r < 16·columns) |
| j1728 | 2^10 | 8.8 | 1508 | 4 | 4 | True | 2.0 | 4.0 | 4.0 | 4.0 | 216 | — | — | folds only (r < 16·columns) |
| j1728 | 2^12 | 8.1 | 28260 | 4 | 4 | True | 2.0 | 4.0 | 4.0 | 4.0 | 690 | — | — | folds only (r < 16·columns) |
| j1728 | 2^12 | 10.3 | 5380 | 4 | 4 | True | 2.0 | 4.0 | 4.0 | 4.0 | 644 | — | — | folds only (r < 16·columns) |
| j1728 | 2^12 | 7.1 | 34112 | 4 | 4 | True | 2.0 | 4.0 | 4.0 | 4.0 | 544 | — | — | folds only (r < 16·columns) |
| j1728 | 2^12 | 11.0 | 7780 | 4 | 4 | True | 2.0 | 4.0 | 4.0 | 4.0 | 1034 | — | — | folds only (r < 16·columns) |


Reading.  The composite fold is not the product of the generators' orders; it is the **order of
the subgroup of `(Z/rZ)^*` their eigenvalues generate**.  On a `j = 0` twist `λ_ζ` (order 3) and
`λ_ψ` (order 4) generate the twelfth roots of unity and the line folds `12` to a column, exactly.
On a `j = 1728` twist `ψ² = −1 = ι²` and there are only two square roots of `−1` modulo `r`, so
`ι = ±ψ` on `⟨G⟩` (measured on every instance) and `⟨−1, ψ, ι⟩` folds `4`, as `⟨−1, ψ⟩` alone
does.  A `j = 1728` GLS twist also always carries a cofactor of about `p` (its `ψ` and `ι`
coincide up to sign as maps of the subgroup, and the twist is isogenous to a curve over `F_p`),
so its usable `r` is only about `p`; the rows say so.  The `j = 0` composite arm recovers the
logarithm over the line oracle with the control on the same points.  This corrects the plan's
prediction of `8` for the `j = 1728` composite.  **Class: engineering** on `j = 0`;
**accounting** on `j = 1728` (the composite adds nothing).

### 6.4 E4 — type C, degree 2 and degree 3

**Summary (per-map table: the tables binary, 80 rows)**

| family | degree | instances | maps per instance | ord_r(λ) min–max | images in base, total | base points, total | all verified |
|:--|--:|--:|--:|--:|--:|--:|:--|
| d11 | 3 | 16 | 4 | 33–245977410 | 0 | 22056 | yes |
| d7 | 2 | 16 | 4 | 337–37072938 | 0 | 15568 | yes |
| d8 | 2 | 16 | 2 | 4613–28560484 | 0 | 9484 | yes |
| j0 | 3 | 16 | 6 | 263–66152880 | 0 | 25464 | yes |
| j1728 | 2 | 16 | 4 | 41–124755636 | 0 | 18632 | yes |


Reading.  Vélu finds every rational degree-2 map on `D = −7`, `D = −8` and `1 + i` on
`j = 1728`, and every rational degree-3 map on `D = −11` and `√−3` on `j = 0` (six on `j = 0`:
three cube roots of the isomorphism scaling, two signs), each verified as `[λ]` on `⟨G⟩`.  The
eigenvalue orders run from `10²` to `2·10⁸` — comparable to `r` at every size — and **no** base
point maps into the base on any instance, `2^13`–`2^28`, against a chance expectation of
`10⁻⁵`–`10⁻³` per point.  §2's boundary holds at every size for both degrees: no type-C map
folds, and none offers free relations worth harvesting.  **Class: accounting.**

### 6.5 E5 — subfield curves on E(F_{p³})

**Summary by family (per-instance table: the tables binary)**

| family | instances | log2 r | pts/col negation | pts/col π | pts/col ζ | pts/col π+ζ | ζ ∈ ⟨π⟩ on ⟨G⟩ | streams | full-rank ratio mean (min–max) | rank fraction at k = cols, π / negation | oracle muls/call | degenerate | all correct |
|:--|--:|:--|--:|--:|--:|--:|:--|--:|--:|--:|--:|--:|:--|
| subfield | 20 | 10.7–21.7 | 2.0 | 6.0 | — | — | — | 20 | 1.61 (1.19–3.31) | 0.80 / 0.83 | 1997 | 0 | yes |
| subfield-j0 | 20 | 6.2–10.8 | 2.0 | 6.0 | 6.0 | 6.0 | all | 0 | — (—–—) | — / — | — | — | folds only |


Reading.  On a generic subfield curve the Frobenius eigenline `x ∈ s^k·F_p` (the line whose
points survive `[h]`; the other line carries the other cubic twist) folds `6` to a column,
column ratio `3`, and the line oracle over `F_{p³}` recovers the logarithm on both arms — the
odd-characteristic twin of the Koblitz fold, at `ord_r(λ_π) = 3`.  But the relation count does
**not** fall with the columns: relations to full rank fall by `1.2`–`1.6` against a column ratio
of `3`, and after `columns` relations the folded arm is at `0.60`–`0.68` of its full rank against
the control's `0.81`–`0.92` (the rows with `h = #E(F_p)`).  The files say why.  `E(F_p)` is a
subgroup of `E(F_{p³})` and never of `⟨G⟩`, so it is always in the cofactor, and `π` fixes it:
every point of a `π`-orbit has the **same** `E(F_p)`-component.  A two-summand relation
`R = εP + ε'Q` forces those components to be negatives (§6.0), so — up to the birthday
coincidences between orbits — `Q = −π^i P` and the relation is `(1 − λ^i)·x_c = log R`: a
**single-column row**.  The tables carry the count: on the folded arm `1,338` of the `1,339`
relations of the `2^21.3` instance pin one column each (`D = 0`, nothing is void), and on the
control the matrix is block-diagonal with a block of three columns and one undetermined
functional per block (`D = 232` against `264` orbits: a block per distinct `±` component, a few blocks holding two orbits that share one).  Both arms are then coupon collectors over
the **same** set of blocks — the fold merges each block's three unknowns into one and lowers the
rank a block needs from two to one, which is what the `1.2`–`1.6` is — and the rank fractions
are those of a coupon collector at one row per coupon (`1 − 1/e ≈ 0.63`) against two rows per
coupon at three per block (`≈ 0.80`).  With a cofactor beyond `E(F_p)` (`h / #E(F_p) = 4, 7`)
the extra component is not fixed by `π`, `D` grows on both arms, and the fold reaches full rank
before `columns` relations.  This is a property of two-summand decomposition on any subfield
curve, not of the fold: Gaudry's `E(F_{p³})` index calculus uses three summands for exactly the
size reason and inherits none of it; the three-summand line oracle was not run here (§7).

On a `j = 0` subfield curve the picture collapses further: `N(π² + π + 1)` splits as
`N(π − ω)·N(π − ω²)` in `Z[ω]`, so the "new" part of `E(F_{p³})` is two subgroups of about
`p` each, `r ≈ p`, and the eigenline carrying `⟨G⟩` is the `F_p`-points of a cubic twist — the
base **is** the subgroup, every target decomposes in about `p/2` ways, `x_R` lies on the line
and the descent degenerates.  The fold still measures `6` for `⟨−1, π⟩`, `⟨−1, ζ⟩` and
`⟨−1, π, ζ⟩` alike, because `λ_ζ ∈ {λ_π, λ_π²}` on every instance: `ζ` is a power of `π` on
`⟨G⟩`, the same rule as E3.  This corrects the plan's prediction of `9` for `⟨π, ζ⟩`.
**Class: engineering** for the columns on the generic curve, **accounting** for the relation
count (the block structure, not the fold, sets it) and for `j = 0`.

### 6.6 E6 — the matched folded rho

**Summary (per-instance table: the tables binary, 48 rows)**

| family | instances | walks | S ratio mean (min–max) | steps ratio mean | expected | all verified |
|:--|--:|--:|--:|--:|--:|:--|
| j0 | 24 | 192 | 1.58 (1.06–3.01) | 1.72 | 1.73 | True |
| j1728 | 24 | 192 | 1.41 (0.96–2.23) | 1.51 | 1.41 | True |


Reading.  The rho folded by the same group verifies its answer on every walk and takes about
`√3` (`j = 0`) and `√2` (`j = 1728`) fewer steps than the negation walk, with the spread eight
walks per instance give.  That is the `√(A/2)` a generic algorithm already takes from the
group, and it is what every "vs rho" figure of §5 understated: the fold's `w/2` in the relation
count against rho's `√(w/2)` in steps widens the gap by `√(w/2)`.  **Class: accounting** for the
reference.

### 6.7 E7 — three summands and orbit duplicates

**Every instance**

| log2 r | seed | m | group order | seed abscissae | points | cols fold | cols control | square rel fold | square rel control | ratio | zero-support rows fold / control | single-column rows fold / control | orbit duplicates | targets | hit rate | correct |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|:--|:--|--:|--:|--:|:--|
| 17.2 | 3 | 2 | 6 | 19 | 114 | 19 | 57 | 35 | 124 | 3.54 | 0 / 0 | 5 / 1 | 148 | 3101 | 0.0400 | yes |
| 17.2 | 3 | 3 | 6 | 19 | 114 | 19 | 57 | 30 | 58 | 1.93 | 0 / 0 | 0 / 0 | 0 | 70 | 0.8286 | yes |
| 17.4 | 4 | 2 | 6 | 20 | 120 | 20 | 60 | 24 | 185 | 7.71 | 0 / 0 | 7 / 2 | 289 | 4800 | 0.0385 | yes |
| 17.4 | 4 | 3 | 6 | 20 | 120 | 20 | 60 | 46 | 85 | 1.85 | 0 / 0 | 0 / 0 | 0 | 103 | 0.8252 | yes |
| 17.7 | 2 | 2 | 6 | 21 | 126 | 21 | 63 | 52 | 159 | 3.06 | 0 / 0 | 4 / 0 | 239 | 4679 | 0.0340 | yes |
| 17.7 | 2 | 3 | 6 | 21 | 126 | 21 | 63 | 41 | 104 | 2.54 | 0 / 0 | 0 / 0 | 1 | 133 | 0.7820 | yes |
| 17.7 | 1 | 2 | 6 | 21 | 126 | 21 | 63 | 32 | 145 | 4.53 | 0 / 0 | 10 / 3 | 193 | 3966 | 0.0366 | yes |
| 17.7 | 1 | 3 | 6 | 21 | 126 | 21 | 63 | 25 | 143 | 5.72 | 0 / 0 | 0 / 0 | 0 | 181 | 0.7901 | yes |
| 18.9 | 4 | 2 | 6 | 26 | 156 | 26 | 78 | 74 | 288 | 3.89 | 0 / 0 | 14 / 4 | 24149 | 82085 | 0.0035 | yes |
| 18.9 | 4 | 3 | 6 | 26 | 156 | 26 | 78 | 59 | 80 | 1.36 | 0 / 0 | 0 / 0 | 0 | 453 | 0.1766 | yes |
| 20.0 | 3 | 2 | 6 | 32 | 192 | 32 | 96 | 84 | 193 | 2.30 | 0 / 0 | 14 / 3 | 2946 | 36381 | 0.0053 | yes |
| 20.0 | 3 | 3 | 6 | 32 | 192 | 32 | 96 | 34 | 155 | 4.56 | 0 / 0 | 0 / 0 | 0 | 449 | 0.3452 | yes |
| 20.1 | 1 | 2 | 6 | 32 | 192 | 32 | 96 | 71 | 267 | 3.76 | 0 / 0 | 15 / 1 | 5310 | 52344 | 0.0051 | yes |
| 20.1 | 1 | 3 | 6 | 32 | 192 | 32 | 96 | 33 | 165 | 5.00 | 0 / 0 | 1 / 1 | 1 | 581 | 0.2840 | yes |
| 20.3 | 2 | 2 | 6 | 33 | 198 | 33 | 99 | 75 | 312 | 4.16 | 0 / 0 | 12 / 1 | 6583 | 62178 | 0.0050 | yes |
| 20.3 | 2 | 3 | 6 | 33 | 198 | 33 | 99 | 42 | 165 | 3.93 | 0 / 0 | 0 / 0 | 0 | 590 | 0.2797 | yes |
| 22.8 | 4 | 2 | 6 | 51 | 306 | 51 | 153 | 170 | 602 | 3.54 | 0 / 0 | 18 / 9 | 140858 | 714665 | 0.0008 | yes |
| 22.8 | 4 | 3 | 6 | 51 | 306 | 51 | 153 | 60 | 227 | 3.78 | 0 / 0 | 0 / 0 | 6 | 2385 | 0.0952 | yes |
| 23.1 | 2 | 2 | 6 | 54 | 324 | 54 | 162 | 80 | 303 | 3.79 | 0 / 0 | 10 / 2 | 35128 | 373187 | 0.0008 | yes |
| 23.1 | 2 | 3 | 6 | 54 | 324 | 54 | 162 | 102 | 368 | 3.61 | 0 / 0 | 0 / 0 | 5 | 4134 | 0.0890 | yes |
| 24.3 | 1 | 2 | 6 | 67 | 402 | 67 | 201 | 115 | 701 | 6.10 | 0 / 0 | 16 / 5 | 36476 | 567338 | 0.0012 | yes |
| 24.3 | 1 | 3 | 6 | 67 | 402 | 67 | 201 | 132 | 425 | 3.22 | 0 / 0 | 0 / 0 | 0 | 2580 | 0.1647 | yes |
| 25.7 | 3 | 2 | 6 | 85 | 510 | 85 | 255 | 188 | 664 | 3.53 | 0 / 0 | 5 / 0 | 3524 | 275060 | 0.0024 | yes |
| 25.7 | 3 | 3 | 6 | 85 | 510 | 85 | 255 | 124 | 641 | 5.17 | 0 / 0 | 0 / 0 | 1 | 1839 | 0.3486 | yes |


Reading.  With three summands the fold on `j = 0` divides the columns by `3` as before and the
full-rank relation count with them; the deficiency of §6.0 does not apply (`h = 1` here in any
case).  Rows whose base part folds to nothing (`0 = ha + hbd`) do not occur on a uniform target
stream; single-column rows do (a target that is a sum of points of one orbit), a few percent on
the folded arm.  Orbit duplicates among the targets — a target whose `⟨−1, ζ⟩`-orbit repeats an
earlier one's, the solver call canonicalisation would save — are the birthday count
`3T²/r` and nothing more, as `RESEARCH_GLV_INDEX_CALCULUS.md` §4 found on the `F_{p³}` harness.
**Class: accounting.**

### 6.8 Verdict after E1–E7

| lever | measured | class |
|:--|:--|:--|
| automorphism fold, `F_p` (E1) | columns `÷ 3`, `÷ 2`; same rank fraction after `columns` relations on both arms at every size to `2^32` | engineering |
| GLS line (E2) | `4` points a column, column ratio `2`, same rank fraction after `columns` relations on both arms; Koblitz up to `2n` on the same driver | engineering |
| subfield line (E5) | `6` points a column, column ratio `3`; relations to full rank `÷ 1.2–1.6` only, every folded row a single-column row: the cofactor `E(F_p)` is `π`-fixed and two-summand relations stay inside one orbit | engineering (columns) / accounting (relations) |
| composite groups (E3, E5) | the fold is the order of the eigenvalue subgroup of `(Z/rZ)^*`: `12` on the `j = 0` twist, `4` on the `j = 1728` twist, `3` on the `j = 0` subfield curve | engineering / accounting |
| type C, degree 2 and 3 (E4) | `0` base points kept in the base at every size; eigenvalue orders `10²`–`10⁸` | accounting |
| matched folded rho (E6) | `√3`, `√2` fewer steps, verified | accounting (reference) |
| three summands, orbit duplicates (E7) | fold unchanged; duplicates at the birthday count | accounting |
| the line oracle (E2b) | about `2,600` `F_p` multiplications a target on `F_{p²}`, `2,000` on `F_{p³}`; agrees with `subtract` on `3,600` of `3,600` targets | engineering |

Nothing here moves a ratio to a floor.  The fold by any finite group of endomorphisms is the
order of the subgroup of `(Z/rZ)^*` that the group's eigenvalues generate — no more, whatever
the group of maps looks like — and rho takes the square root of the same number.  Where the
group has a cofactor the endomorphisms fix, the fold buys columns and not relations (E5).  That is the
closed form the plan's E3 and E5 predictions lacked, and it is the reason the search for a
larger fold on a fixed prime-order subgroup ends here: the roots of unity modulo `r` that
endomorphisms of a curve can realise are the automorphism group's, the Frobenius's, and their
products.

## 7. What was not done

- **Q-curves (E4b, second half).**  No degree-2 or degree-3 Q-curve over
  `F_{p²}` was built; Smith's construction is not in the repository and
  the type-C boundary is only measured on CM curves over `F_p`.
- **E2's Koblitz arm covers four degrees.**  The ledger's
  `koblitz_instance` roster has usable divisors of `x^n − 1` of
  intermediate degree at `n = 15, 17, 23, 31` only; `n = 13` has a hit
  rate too low to reach full rank within the trial cap and `n = 19` a
  subspace too large for the pair table.  Four sizes is the minimum the
  plan asked for and no exponent is fitted to them.
- **E5 on `j = 0` has no relation stream.**  The eigenline carrying
  `⟨G⟩` is the subgroup itself, so the descent is degenerate and only
  the fold is measured; a `j = 0` subfield curve with `r` larger than
  `p` would need `E(F_{p^n})` for `n` prime to `3` or a different
  eigenline, neither of which the driver builds.
- **The algebraic `S₄` oracle (E11, §8.5) is run to `p = 2^13` only**,
  and its Frobenius symmetry breaking is measured as the orbit-duplicate
  count rather than reproduced from the paper, which this session could
  not retrieve.
- **`S` at full rank is measured for E8 only.**  §8.2 prices every
  phase of the three-summand pair-table pipeline and the matched rho in
  one unit on the subfield curves; E1–E7 count relations, rank, trials,
  oracle cost and rho steps, and their end-to-end `S` columns stay the
  pilot's (§5.1).  No row anywhere claims a speed.
- **The line oracle is a library and example component only.**  It is
  not a framework plugin and `ic bench` cannot select it; the `subtract`
  and `mitm` oracles remain what the CLI offers on the line bases.
- **The `F_{p²}` and `F_{p³}` "vs rho" figures of §5 are against the
  `A = 2` walk**; E9 (§8.3) measures the matched folded walk on those
  groups, so the reader can rescale them, but the §5 rows are not
  re-run.
- The pilot's base build prices no square roots or Legendre symbols
  (no pinned ratio for generated curves); at these sizes the build is
  under `3 %` of `S` on every row, but a larger sweep should pin them
  as `ic bench` does for the roster.
- The scoreboard carries the pilot and E1–E13 as two panels with their
  column, rank and relation ratios; no scoreboard row claims a speed,
  and the exponent panel is untouched.
- Nothing here bears on any deployed curve: `r ≤ 2^32`, certified toy
  instances, and a fold that rho already takes as `√(w/2)`.
- Every experiment of §8.1 has run.  E14 builds its Q-curves by scanning `F_{p²}` for the `j` with `Φ_d(j, j^p) = 0` and for the kernel, so it stops at `p = 2^{12}`.  E15 ran on `E_0` at `m = 13, 15, 23, 31` only: the faithful `4·prime` degrees `19` and `41` are beyond the pair-table driver, and the §8a `m = 83` gate was not run (E15 claims no improvement).  E13 does not price the negation arm on the `Y`-line (it stalls below full rank there, not derived, §8.7), so the plan's fold-against-negation ratio is unmeasured on that base.

## 8. Toward the state of the art: what the literature does, what is next, and E8–E16

### 8.0 Where E1–E7 stand against the literature

Three published lines of work bear on an endomorphism-invariant factor base, and
§6 reproduces two of them at framework level and adds two things they do not
report.

- **Galbraith, Granger, Merz, Petit, "On Index Calculus Algorithms for Subfield
  Curves"** (SAC 2020; ePrint 2020/1315).  Factor bases invariant under the
  `q`-power Frobenius on subfield (Koblitz) curves: the number of decomposition
  systems to solve falls by `1/n`, the linear algebra by `n²`, and the Frobenius
  is used for *symmetry breaking* in the polynomial systems.  E2 (Koblitz orbit,
  GLS line) and E5 (the `F_{p³}` line) are the odd-characteristic, counted
  reproduction of the first two claims; §6.8's closed form — the fold is the
  order of the eigenvalue subgroup of `(Z/rZ)^*`, so `⟨π, ζ⟩` folds `3` and not
  `9` — and §6.5's block degeneracy of two-summand relations under a
  `π`-fixed cofactor are not in that paper.  Its third lever, symmetry breaking
  inside the decomposition system, is **not** implemented here (E11 below).
- **Faugère, Gaudry, Huot, Renault, "Using Symmetries in the Index Calculus for
  Elliptic Curves Discrete Logarithm"** (ePrint 2012/199; J. Cryptology 2014),
  and the follow-up on symmetrised summation polynomials from small-order
  torsion.  These act on the *decomposition system*: the symmetric group `S_m`
  on the summands and, from a rational `2`-torsion point, `(Z/2)^{m−1}`, give an
  invariant-ring presentation whose Gröbner cost falls by an exponential factor
  in `m`; on binary Koblitz curves the Frobenius is added.  An endomorphism `φ`
  of `⟨G⟩` is **not** a symmetry of that system: `R = ΣP_i` does not give
  `R = Σ φ^{a_i} P_i` unless every `a_i` is equal, in which case the target
  moves.  So the fold of this note and the FGHR symmetries are different levers
  on different objects — the base and the system — and multiply (E13).
- **Chi-Domínguez, Rodríguez-Henríquez, Smith, "Extending the GLS endomorphism
  to speed up GHS Weil descent using Magma"** (2021; `F_{2^{155}}` solved).  The
  GLS endomorphism induces an endomorphism of the GHS Jacobian and gives a
  factor `n` there; it is the same `ord_r(λ)` lever on a different group, and
  reads as further evidence that the lever is a constant.
- In this repository, `RESEARCH_GLV_INDEX_CALCULUS.md` §3 measured the `⟨ψ⟩`
  quotient on `E(F_{p³})` with the algebraic `S₄` oracle: `S ÷ 3.0` with the
  count on its floor — the same reading as §6 with the other oracle.

The honest position: no published or measured use of an endomorphism moves the
exponent.  The state of the art is the **combination** — an invariant base (GGMP,
this note) under an FGHR-symmetrised decomposition system with Frobenius symmetry
breaking — and nothing here or in the literature has measured that combination
on an odd-characteristic subfield curve at full rank with every phase priced.
That is the ordering below.

### 8.1 Ranked next steps

| id | question | prediction | falsification target (in advance) | class if it holds | status |
|:--|:--|:--|:--|:--|:--|
| **E8** | Three summands on the `π`-line of a subfield curve: does the fold's relation count fall with the columns once the §6.5 block degeneracy is gone? | relations to full rank `÷ 3` (times the coupon factor), `0` single-column rows, both arms at one rank fraction after `columns` relations | full-rank ratio `< 2.0` on a majority of instances, or single-column rows `> 1 %`, falsifies | engineering | **done** (§8.2) |
| **E9** | The matched folded rho on the `F_{p²}` and `F_{p³}` groups, so every "vs rho" on those groups has its reference | steps ratio `√(A/2)`: `√2` (GLS), `√6` (`j = 0` twist), `√3` (subfield) | a mean steps ratio below `0.8·√(A/2)` or any unverified walk falsifies | accounting (reference) | **done** (§8.3) |
| **E10** | One `S` column in one unit (`F_p` multiplications; inversions by a measured factor; linear-algebra row operations as one each) for both arms and both walks on one instance, five sizes, exponent fitted | the fold moves `S` by its column ratio where the relation phase dominates and by its square where the linear algebra does; the exponent of neither arm falls below rho's `1/2` with the pair-table oracle | an arm whose fitted exponent is below the control's by more than the fit's scatter would be an advance; expected: none | engineering | **done**, inside E8 (§8.2) |
| **E11** | The algebraic three-summand oracle on the line: `S₄(s t₁, s t₂, s t₃, x_R)` Weil-descended, `S₃`-symmetrised in the elementary symmetric functions of `t_i`, solved as `gaudry_cubic::solve_s4_subspace` solves the `x_i ∈ F_p` base; then GGMP's Frobenius symmetry breaking — only canonical `⟨−1, π⟩`-orbit representatives admitted as solutions | oracle cost `O(1)` in `p` (the Macaulay solve at degree `10`–`13`); the fold ratio unchanged at `3`; symmetry breaking removes the `3`-fold redundancy among a system's solutions, not systems | the fold's `S` ratio below `2.7` at any size with this oracle, or an oracle cost growing with `p`, falsifies | engineering | **done** (§8.5): `9.42e+05` multiplications a call, flat in `p`; agrees with the pair table on `3600` of `3600` targets; fitted exponent `0.53` on the fold; symmetry breaking reduces to the orbit duplicates |
| **E12** | The pair table over orbit representatives: `P + φ^c P'` for representatives `P, P'` and `c < w/2` | table `w/2` smaller and `w/2` cheaper to build, one probe per target unchanged; `S` moves only by the table's share | a probe cost above `1.2×` the full table's falsifies "unchanged" | engineering (memory) | **done** (§8.6): table `÷ 2.95`–`2.97` (`w = 6`) and `÷ 1.98` (`w = 4`); probe `1.13×` at worst, so "unchanged" survives; `S ÷ 1.18` on `F_p` `j = 0`, `× 1.02` on the line |
| **E13** | FGHR's `2`-torsion symmetry **and** the fold together: close the line base under translation by `T ∈ E(F_p)[2]` and under `π`, fold by `⟨−1, π⟩`, symmetrise the system by `(Z/2)^{m−1} ⋊ S_m` | the two levers multiply — columns `÷ 3`, system degree `÷ 2^{m−1}` — because one acts on the base and the other on the system | a combined `S` ratio below the product of the separate ratios by more than `20 %` falsifies "multiply" | engineering | **done** (§8.7), with the base closed under `τ_T` and folded by it too (`12` a column): `R(q₃)` of degree `16 = 64/4`, `S ÷ 27.0` from the system, `÷ 2.52` from `τ_T`; combined / product `≥ 0.82`, so "multiply" survives; the negation arm stalls below full rank on the `Y`-line and is not priced |
| **E14** | Q-curves of degree 2 and 3 over `F_{p²}` (the second half of E4b) | type C: `0` base points kept, no fold | any image in the base beyond chance falsifies | accounting | **done** (§8.9): `40` Q-curves (`d = 2, 3`, `r = 2^{12}`–`2^{22}`), every `ψ` verified with `λ² ≡ ±d`, `ord_r(λ) ≥ 1817`; `32` of `43310` base points kept against `39.3` at chance — no fold |
| **E15** | Transfer to the binary Koblitz program (ECC2K-130, AGENTS.md §8a–8b): is the §6.5 degeneracy present there? | no: `E(F_2) ⊂ E(F_{2^n})` has order `2` or `4`, so at most two component classes and no block structure; the fold there is the known `2n` | a measured deficiency `D > 2` on a prime-order-times-`4` Koblitz subgroup with two summands falsifies | accounting | **done** (§8.8): on `E_0` at the challenge's `4·prime` shape (`m = 23`) `D = 1` on both arms and `4 %` single-column rows, so the prediction holds; at `m = 31`, whose `E_0` cannot be `4·prime` (cofactor `4·373`), `D = 12 / 342` and `73 %` single-column rows — `m = 31` is not a faithful proxy for two-summand relation structure |
| **E16** | The boundary on subfield curves: with `r ≈ p²/k`, does any arm's relation phase grow more slowly than rho, and where does the best arm's ratio go? (stated after the fact, §8.10) | no: a line base needs `Θ(p)` relations, rho costs `Θ(p)`, the linear algebra `Θ(p²)`; the ratio does not fall | a fitted `S / rho S` exponent below `−0.05` over `p = 2^7`–`2^{16}` falsifies | accounting | **done** (§8.10): `S / rho S ∝ r^{0.18}`, linear algebra `r^{1.01}`, total `r^{0.62}` against rho's `r^{0.44}`; two earlier exponent claims corrected |

What is **not** on the list, and why: a larger fold on a fixed prime-order
subgroup.  §6.8's closed form bounds it by the roots of unity the curve's
endomorphisms realise modulo `r` — `lcm(|Aut E|, ord λ_π)` at most — and every
family with `|Aut E| > 2` in odd characteristic is `j = 0` or `j = 1728`; there
is no fourth lever on the base.

### 8.2 E8 — three summands on the Frobenius line, every phase priced

**Relations, every instance**

| p | log2 r | h / #E(F_p) | cols π | cols negation | full-rank rel π | full-rank rel negation | ratio | rank fraction at k = cols, π / negation | single-column rows π / negation | orbit duplicates | targets | hit rate | correct |
|--:|--:|--:|--:|--:|--:|--:|--:|:--|:--|--:|--:|--:|:--|
| 2^7 | 10.7 | 4 | 11 | 33 | — | — | — | 0.75 / 0.74 | 0 / 0 | 1410 | 10180 | 0.0094 | yes |
| 2^8 | 11.8 | 7 | 26 | 78 | 50 | 181 | 3.62 | 0.85 / 0.90 | 0 / 0 | 598 | 1325 | 0.1366 | yes |
| 2^7 | 11.9 | 4 | 19 | 57 | — | — | — | 0.75 / 0.72 | 0 / 0 | 3110 | 31286 | 0.0084 | yes |
| 2^7 | 12.1 | 1 | 14 | 42 | 19 | 94 | 4.95 | 0.80 / 0.91 | 0 / 0 | 59 | 353 | 0.2663 | yes |
| 2^8 | 12.4 | 7 | 33 | 99 | 42 | 226 | 5.38 | 0.88 / 0.90 | 0 / 0 | 647 | 1637 | 0.1381 | yes |
| 2^7 | 13.4 | 1 | 18 | 54 | 46 | 116 | 2.52 | 0.95 / 0.89 | 0 / 0 | 110 | 755 | 0.1536 | yes |
| 2^9 | 14.8 | 7 | 87 | 261 | 100 | 672 | 6.72 | 0.98 / 0.91 | 0 / 0 | 658 | 3218 | 0.2088 | yes |
| 2^9 | 15.1 | 4 | 62 | 186 | 99 | 506 | 5.11 | 0.95 / 0.91 | 0 / 0 | 604 | 3204 | 0.1579 | yes |
| 2^8 | 15.5 | 1 | 44 | 132 | 76 | 321 | 4.22 | 0.89 / 0.90 | 0 / 0 | 117 | 1522 | 0.2109 | yes |
| 2^8 | 15.5 | 1 | 48 | 144 | 110 | 383 | 3.48 | 0.90 / 0.95 | 0 / 0 | 116 | 1579 | 0.2426 | yes |
| 2^9 | 15.7 | 4 | 81 | 243 | 225 | 527 | 2.34 | 0.88 / 0.91 | 0 / 0 | 332 | 2760 | 0.1909 | yes |
| 2^10 | 16.2 | 4 | 84 | 252 | 152 | 508 | 3.34 | 0.93 / 0.92 | 0 / 0 | 630 | 4691 | 0.1083 | yes |
| 2^10 | 16.5 | 4 | 104 | 312 | 134 | 629 | 4.69 | 0.96 / 0.93 | 0 / 0 | 369 | 3965 | 0.1586 | yes |
| 2^9 | 17.1 | 1 | 58 | 174 | 67 | 451 | 6.73 | 0.97 / 0.95 | 0 / 0 | 193 | 3319 | 0.1359 | yes |
| 2^10 | 17.2 | 4 | 133 | 399 | 276 | 676 | 2.45 | 0.93 / 0.92 | 0 / 0 | 242 | 3889 | 0.1738 | yes |
| 2^11 | 17.9 | 7 | 222 | 666 | 462 | 1430 | 3.10 | 0.91 / 0.94 | 0 / 0 | 782 | 8956 | 0.1597 | yes |
| 2^11 | 19.0 | 4 | 264 | 792 | 612 | 2197 | 3.59 | 0.92 / 0.94 | 0 / 0 | 642 | 11860 | 0.1852 | yes |
| 2^10 | 19.2 | 1 | 141 | 423 | 196 | 1091 | 5.57 | 0.95 / 0.96 | 0 / 0 | 146 | 5818 | 0.1875 | yes |
| 2^11 | 21.3 | 1 | 264 | 792 | 545 | 2569 | 4.71 | 0.95 / 0.93 | 0 / 0 | 314 | 17765 | 0.1446 | yes |
| 2^11 | 21.7 | 1 | 307 | 921 | 776 | 2370 | 3.05 | 0.94 / 0.93 | 0 / 0 | 122 | 14351 | 0.1651 | yes |

**Every phase priced (rows at full rank on both arms)**

| p | log2 r | inv / mul (measured) | muls per addition | base build π / neg | pair table | stream π / neg | LA π / neg | total π | total negation | S π | S negation | S neg / S π | rho S negation | rho S folded | rho muls per group op, negation / folded | rho steps ratio (expected) | S π / rho S folded | all verified |
|--:|--:|--:|--:|:--|--:|:--|:--|--:|--:|--:|--:|--:|--:|--:|:--|:--|--:|:--|
| 2^7 | 10.7 | — | — | not full rank on both arms: the stream exhausted the group's targets | | | | | | | | | | | | | | |
| 2^8 | 11.8 | 2.4 | 83.9 | 3.37e+05 / 3.36e+05 | 5.09e+05 | 4.09e+06 / 1.66e+07 | 1.63e+03 / 3.42e+03 | 4.94e+06 | 1.75e+07 | 83240.0 | 294518.5 | 3.54 | 221.1 | 202.4 | 89 / 104 | 1.58 (1.73) | 411 | yes |
| 2^7 | 11.9 | — | — | not full rank on both arms: the stream exhausted the group's targets | | | | | | | | | | | | | | |
| 2^7 | 12.1 | 2.4 | 85.7 | 1.21e+05 / 1.2e+05 | 1.48e+05 | 4.58e+05 / 3e+06 | 786 / 1.52e+03 | 7.28e+05 | 3.27e+06 | 10945.5 | 49116.3 | 4.49 | 189.6 | 202.5 | 90 / 106 | 1.20 (1.73) | 54 | yes |
| 2^8 | 12.4 | 2.6 | 83.5 | 4.52e+05 / 4.51e+05 | 8.21e+05 | 3.51e+06 / 2.54e+07 | 2.07e+03 / 6.54e+03 | 4.78e+06 | 2.67e+07 | 65028.0 | 362761.0 | 5.58 | 200.4 | 168.5 | 89 / 107 | 1.87 (1.73) | 386 | yes |
| 2^7 | 13.4 | 2.8 | 84.9 | 1.86e+05 / 1.86e+05 | 2.45e+05 | 3.77e+06 / 8.44e+06 | 1.31e+03 / 1.87e+03 | 4.2e+06 | 8.87e+06 | 39768.1 | 84004.3 | 2.11 | 202.6 | 161.4 | 88 / 110 | 1.77 (1.73) | 246 | yes |
| 2^9 | 14.8 | 3.1 | 82.2 | 1.29e+06 / 1.29e+06 | 5.73e+06 | 1.7e+07 / 1.24e+08 | 7.79e+03 / 4.03e+04 | 2.41e+07 | 1.31e+08 | 140331.9 | 762621.8 | 5.43 | 145.9 | 104.1 | 88 / 109 | 2.26 (1.73) | 1348 | yes |
| 2^9 | 15.1 | 3.6 | 82.6 | 9.32e+05 / 9.3e+05 | 2.93e+06 | 1.82e+07 / 9.62e+07 | 5.14e+03 / 1.45e+04 | 2.2e+07 | 1e+08 | 118711.7 | 539163.0 | 4.54 | 173.3 | 145.0 | 90 / 106 | 1.93 (1.73) | 819 | yes |
| 2^8 | 15.5 | 2.6 | 83.3 | 5.5e+05 / 5.49e+05 | 1.46e+06 | 8.59e+06 / 3.37e+07 | 3.14e+03 / 8.58e+03 | 1.06e+07 | 3.57e+07 | 49135.3 | 165494.7 | 3.37 | 203.2 | 151.3 | 87 / 107 | 2.24 (1.73) | 325 | yes |
| 2^8 | 15.5 | 2.6 | 83.1 | 6.26e+05 / 6.24e+05 | 1.74e+06 | 9.82e+06 / 3.7e+07 | 3.62e+03 / 1.56e+04 | 1.22e+07 | 3.94e+07 | 56054.2 | 181145.9 | 3.23 | 167.7 | 153.2 | 88 / 107 | 1.63 (1.73) | 366 | yes |
| 2^9 | 15.7 | 3.4 | 82.3 | 1.2e+06 / 1.2e+06 | 4.99e+06 | 4.41e+07 / 1.06e+08 | 7.15e+03 / 2.79e+04 | 5.03e+07 | 1.12e+08 | 221091.8 | 494196.1 | 2.24 | 175.5 | 136.4 | 88 / 108 | 2.15 (1.73) | 1621 | yes |
| 2^10 | 16.2 | 3.5 | 82.2 | 1.29e+06 / 1.28e+06 | 5.37e+06 | 6.1e+07 / 1.99e+08 | 6.06e+03 / 4.51e+04 | 6.76e+07 | 2.05e+08 | 243220.5 | 738352.7 | 3.04 | 152.4 | 143.8 | 88 / 110 | 1.50 (1.73) | 1691 | yes |
| 2^10 | 16.5 | 3.6 | 82.1 | 1.65e+06 / 1.65e+06 | 8.24e+06 | 4.15e+07 / 1.97e+08 | 9e+03 / 4.99e+04 | 5.14e+07 | 2.07e+08 | 167825.5 | 673631.6 | 4.01 | 157.9 | 137.8 | 88 / 111 | 1.67 (1.73) | 1218 | yes |
| 2^9 | 17.1 | 3.0 | 82.8 | 7.77e+05 / 7.75e+05 | 2.55e+06 | 1.64e+07 / 1.01e+08 | 5.15e+03 / 1.75e+04 | 1.97e+07 | 1.05e+08 | 52125.2 | 276570.0 | 5.31 | 150.3 | 95.9 | 87 / 109 | 2.67 (1.73) | 544 | yes |
| 2^10 | 17.2 | 3.6 | 81.9 | 2.19e+06 / 2.19e+06 | 1.35e+07 | 9.7e+07 / 2.43e+08 | 9.9e+03 / 1.54e+05 | 1.13e+08 | 2.59e+08 | 290849.5 | 667415.4 | 2.29 | 112.8 | 116.7 | 89 / 111 | 1.29 (1.73) | 2492 | yes |
| 2^11 | 17.9 | 4.4 | 81.6 | 3.58e+06 / 3.57e+06 | 3.79e+07 | 2.93e+08 / 9.16e+08 | 3.6e+04 / 7.48e+05 | 3.35e+08 | 9.58e+08 | 688620.5 | 1970035.7 | 2.86 | 139.1 | 122.1 | 88 / 114 | 1.59 (1.73) | 5641 | yes |
| 2^11 | 19.0 | 4.0 | 81.5 | 5.07e+06 / 5.06e+06 | 5.33e+07 | 3.86e+08 / 1.42e+09 | 5.92e+04 / 8.85e+05 | 4.45e+08 | 1.48e+09 | 615720.3 | 2044461.1 | 3.32 | 107.5 | 91.1 | 87 / 115 | 1.69 (1.73) | 6759 | yes |
| 2^10 | 19.2 | 3.7 | 81.9 | 2.11e+06 / 2.1e+06 | 1.52e+07 | 6.61e+07 / 3.85e+08 | 1.9e+04 / 1.7e+05 | 8.34e+07 | 4.03e+08 | 107666.4 | 519523.6 | 4.83 | 124.4 | 59.5 | 87 / 112 | 3.57 (1.73) | 1809 | yes |
| 2^11 | 21.3 | 4.9 | 81.6 | 4.42e+06 / 4.41e+06 | 5.39e+07 | 4.48e+08 / 2.25e+09 | 5.98e+04 / 8.87e+05 | 5.06e+08 | 2.31e+09 | 318217.3 | 1451200.0 | 4.56 | 106.8 | 85.4 | 87 / 118 | 1.73 (1.73) | 3727 | yes |
| 2^11 | 21.7 | 5.2 | 81.5 | 5.09e+06 / 5.08e+06 | 7.31e+07 | 6.79e+08 / 2.07e+09 | 7.37e+04 / 9.61e+05 | 7.57e+08 | 2.15e+09 | 412185.9 | 1171277.2 | 2.84 | 110.0 | 97.4 | 87 / 119 | 1.53 (1.73) | 4232 | yes |

**Summary**

| arms | instances (of run) | log2 r | column ratio | full-rank ratio mean (min–max) | rank fraction at k = cols, π / negation | S neg / S π mean (min–max) | S π / rho S folded, min–max | rho steps ratio mean (expected) | rho muls per group op, negation / folded | rho S negation / rho S folded, mean | fitted exponent of total: π, negation, rho folded, rho negation (rho: 0.50) | all verified |
|:--|--:|:--|--:|--:|--:|--:|--:|--:|--:|--:|:--|:--|
| π-line fold vs negation, m = 3, pair table | 18 (20) | 11.8–21.7 | 3.0 | 4.20 (2.34–6.73) | 0.92 / 0.92 | 3.75 (2.11–5.58) | 54–6759 | 1.88 (1.73) | 88 / 110 | 1.24 | 0.88, 0.86, 0.37, 0.39 | yes |


Reading.  With three summands the §6.5 degeneracy is gone: **zero** single-column
rows on either arm at every size, the fold's relation count falls by `4.20`
(`2.34`–`6.73`) against a column ratio of `3` — the coupon-collector
factor on top of `3`, as in E1 — and both arms sit at the same rank fraction after
`columns` relations (`0.92` against `0.92`).  The prediction of E8
holds; the two `2^7` instances with `r < 2^12` exhausted the group's targets before
full rank and carry no ratio.  The cost column is the first end-to-end `S` of this
note on a group where index calculus is asymptotically competitive (`n = 3`): with
the pair-table oracle every phase is counted in `F_p` multiplications — the base
build (square roots on the line), the pair table (`|F|²/4` additions, shared), the
stream (`|F|` additions a target), the linear algebra (row operations, one `Z/rZ`
multiplication each) — and the matched rho, negation and `π`-folded, is counted in
the same unit on the same instance, with inversions priced by the measured
`2.2`–`5.5` multiplications each.  The fold divides `S` by `3.75`
(`2.11`–`5.58`): the stream is `63`–`90 %`
of the folded arm's total and falls with the relations, the pair table
(`6`–`24 %`) does not fold (E12), and the linear
algebra is below `1 %` on both arms at these sizes.  The folded arm costs
`54`–`6759×` the `π`-folded rho.  Fitted exponents of the total
against `r` over the `18` instances at full rank: fold `0.88`, control
`0.86`, folded rho `0.37`, negation rho `0.39` — the
pair-table arms grow faster than rho's `1/2`, as the pair table (`p² ∝ r^{2/3}`) and
the stream (`p` targets at `p` additions) say they must; the algebraic oracle (E11)
is what would bring the relation phase to `p^{1+o(1)}`.  One number the unit
exposes that the step count hides: the `π`-folded walk takes `1.88×` fewer
steps (expected `√3 = 1.73`) but canonicalises on every step — the `⟨−1, π⟩`-orbit
by breadth-first search, three Frobenius maps and their keys — and that costs
`22` multiplications a group operation on top of the addition's
`88` (`110` against `88`), so in `F_p` multiplications the
negation rho costs `1.24×` the folded one on `E(F_{p³})`, not `1.73×`.  A first
version of the map that multiplied by the unit twist constants paid `230` a step
and made the folded walk the dearer of the two in this unit; that is why the
canonicalisation's price is reported beside the step ratio, and why the same
accounting applies to a folded base's canonicalisations in a relation search.
**Class: engineering** on the fold; the `S`
column is a stage-complete measurement of a method that is not faster than rho
here, and claims nothing else.

### 8.3 E9 — the matched folded rho on `F_{p²}` and `F_{p³}`

**Summary by family (per-instance table: the tables binary, 60 rows)**

| family | A | instances | log2 r | walks | S ratio mean (min–max) | steps ratio mean | expected | all verified |
|:--|--:|--:|:--|--:|--:|--:|--:|:--|
| gls-generic | 4 | 12 | 9.3–22.9 | 192 | 1.28 (0.90–1.87) | 1.40 | 1.41 | True |
| gls-j0 | 12 | 12 | 11.0–23.0 | 192 | 1.71 (1.21–2.17) | 2.16 | 2.45 | True |
| gls-j1728 | 4 | 12 | 6.2–11.0 | 192 | 1.06 (0.93–1.28) | 1.15 | 1.41 | True |
| subfield | 6 | 12 | 11.8–23.8 | 192 | 1.48 (1.16–2.70) | 1.74 | 1.73 | True |
| subfield-j0 | 6 | 12 | 6.2–11.9 | 192 | 1.23 (0.96–1.64) | 1.72 | 1.73 | True |


Reading.  Every one of the `960` walks verifies its answer.  The steps ratios
track `√(A/2)` with the spread eight walks per instance give: `1.40` on the GLS
twist (`A = 4`, expected `1.41`), `1.74` on the subfield curve (`A = 6`,
expected `1.73`), and `2.16` on the `j = 0` GLS twist (`A = 12`, expected
`2.45`) — the four-dimensional GLV–GLS curves of Longa–Sica, the largest fold any
ordinary curve over `F_{p²}` admits, where the walks at `r ≤ 2^23` are too short
(`2^5`–`2^11` steps) to reach their asymptote.  The `j = 1728` twist and the `j = 0`
subfield curve are measured at `r ≈ p` (§6.3, §6.5) and show `1.15` and
`1.72` at `r ≤ 2^12`, where a walk is a few hundred steps and its set-up
dominates.  Every "vs rho" figure on these groups in §5 and §6 can now be read
against the matched walk instead of the `A = 2` one, with E8's caveat that the
matched walk's cost in field multiplications depends on what a canonicalisation
costs.  **Class: accounting** (reference).

### 8.5 E11 — the algebraic `S₄` oracle on the Frobenius line

**Relations, every instance**

| p | log2 r | h / #E(F_p) | cols π | cols negation | full-rank rel π | full-rank rel negation | ratio | rank fraction at k = cols, π / negation | single-column rows π / negation | orbit duplicates | targets | hit rate | correct |
|--:|--:|--:|--:|--:|--:|--:|--:|:--|:--|--:|--:|--:|:--|
| 2^7 | 10.7 | 4 | 11 | 33 | — | — | — | 0.75 / 0.74 | 0 / 0 | 1410 | 10180 | 0.0094 | yes |
| 2^8 | 11.8 | 7 | 26 | 78 | 50 | 181 | 3.62 | 0.85 / 0.90 | 0 / 0 | 598 | 1325 | 0.1366 | yes |
| 2^7 | 11.9 | 4 | 19 | 57 | — | — | — | 0.75 / 0.72 | 0 / 0 | 3110 | 31286 | 0.0084 | yes |
| 2^7 | 12.1 | 1 | 14 | 42 | 19 | 94 | 4.95 | 0.80 / 0.91 | 0 / 0 | 59 | 353 | 0.2663 | yes |
| 2^8 | 12.4 | 7 | 33 | 99 | 42 | 226 | 5.38 | 0.88 / 0.90 | 0 / 0 | 647 | 1637 | 0.1381 | yes |
| 2^7 | 13.4 | 1 | 18 | 54 | 46 | 116 | 2.52 | 0.95 / 0.89 | 0 / 0 | 110 | 755 | 0.1536 | yes |
| 2^9 | 14.8 | 7 | 87 | 261 | 100 | 672 | 6.72 | 0.98 / 0.91 | 0 / 0 | 658 | 3218 | 0.2088 | yes |
| 2^9 | 15.1 | 4 | 62 | 186 | 99 | 505 | 5.10 | 0.95 / 0.91 | 0 / 0 | 604 | 3204 | 0.1576 | yes |
| 2^8 | 15.5 | 1 | 44 | 132 | 75 | 317 | 4.23 | 0.91 / 0.91 | 0 / 0 | 117 | 1522 | 0.2083 | yes |
| 2^8 | 15.5 | 1 | 48 | 144 | 109 | 380 | 3.49 | 0.92 / 0.95 | 0 / 0 | 116 | 1579 | 0.2407 | yes |
| 2^9 | 15.7 | 4 | 81 | 243 | 225 | 527 | 2.34 | 0.88 / 0.91 | 0 / 0 | 332 | 2760 | 0.1909 | yes |
| 2^10 | 16.2 | 4 | 84 | 252 | 152 | 508 | 3.34 | 0.93 / 0.92 | 0 / 0 | 630 | 4691 | 0.1083 | yes |
| 2^10 | 16.5 | 4 | 104 | 312 | 133 | 626 | 4.71 | 0.96 / 0.93 | 0 / 0 | 369 | 3965 | 0.1579 | yes |
| 2^9 | 17.1 | 1 | 58 | 174 | 67 | 451 | 6.73 | 0.97 / 0.95 | 0 / 0 | 193 | 3319 | 0.1359 | yes |
| 2^10 | 17.2 | 4 | 133 | 399 | 274 | 671 | 2.45 | 0.93 / 0.92 | 0 / 0 | 242 | 3889 | 0.1725 | yes |
| 2^11 | 17.9 | 7 | 222 | 666 | 462 | 1429 | 3.09 | 0.91 / 0.94 | 0 / 0 | 782 | 8956 | 0.1596 | yes |
| 2^11 | 19.0 | 4 | 264 | 792 | 612 | 2193 | 3.58 | 0.92 / 0.94 | 0 / 0 | 642 | 11860 | 0.1849 | yes |
| 2^10 | 19.2 | 1 | 141 | 423 | 195 | 1086 | 5.57 | 0.95 / 0.96 | 0 / 0 | 146 | 5818 | 0.1867 | yes |
| 2^12 | 19.2 | 7 | 350 | 1050 | 768 | 2848 | 3.71 | 0.94 / 0.94 | 0 / 0 | 1503 | 20176 | 0.1412 | yes |
| 2^12 | 20.2 | 4 | 361 | 1083 | 1003 | 2892 | 2.88 | 0.94 / 0.94 | 0 / 0 | 877 | 20684 | 0.1398 | yes |
| 2^11 | 21.3 | 1 | 264 | 792 | 544 | 2564 | 4.71 | 0.95 / 0.93 | 0 / 0 | 314 | 17765 | 0.1443 | yes |
| 2^11 | 21.7 | 1 | 307 | 921 | 776 | 2368 | 3.05 | 0.94 / 0.93 | 0 / 0 | 122 | 14351 | 0.1650 | yes |
| 2^12 | 23.7 | 1 | 614 | 1842 | 1199 | 3979 | 3.32 | 0.93 / 0.93 | 0 / 0 | 113 | 26283 | 0.1514 | yes |
| 2^12 | 23.8 | 1 | 659 | 1977 | 1589 | 5156 | 3.24 | 0.95 / 0.93 | 0 / 0 | 184 | 33288 | 0.1549 | yes |
| 2^14 | 24.2 | 4 | 1409 | 4227 | 4086 | 12670 | 3.10 | 0.93 / 0.94 | 0 / 0 | 968 | 87047 | 0.1456 | yes |
| 2^13 | 25.0 | 1 | 977 | 2931 | 2349 | 9548 | 4.06 | 0.93 / 0.94 | 0 / 0 | 260 | 56958 | 0.1676 | yes |
| 2^13 | 25.0 | 1 | 956 | 2868 | 1875 | 7993 | 4.26 | 0.93 / 0.94 | 0 / 0 | 212 | 53004 | 0.1508 | yes |
| 2^14 | 27.0 | 1 | 1935 | 5805 | 5690 | 18731 | 3.29 | 0.94 / 0.94 | 0 / 0 | 258 | 123811 | 0.1513 | yes |

**Every phase priced (rows at full rank on both arms)**

| p | log2 r | inv / mul (measured) | muls per addition | base build π / neg | pair table | stream (incl. oracle) π / neg | LA π / neg | total π | total negation | S π | S negation | S neg / S π | rho S negation | rho S folded | rho muls per group op, negation / folded | rho steps ratio (expected) | S π / rho S folded | all verified |
|--:|--:|--:|--:|:--|--:|:--|:--|--:|--:|--:|--:|--:|--:|--:|:--|:--|--:|:--|
| 2^7 | 10.7 | — | — | not full rank on both arms: the stream exhausted the group's targets | | | | | | | | | | | | | | |
| 2^8 | 11.8 | 2.5 | 92.7 | 3.38e+05 / 3.37e+05 | 0 | 2.4e+08 / 9.74e+08 | 1.63e+03 / 3.42e+03 | 2.4e+08 | 9.74e+08 | 4046821.5 | 16430454.0 | 4.06 | 221.4 | 202.6 | 89 / 105 | 1.58 (1.73) | 19972 | yes |
| 2^7 | 11.9 | — | — | not full rank on both arms: the stream exhausted the group's targets | | | | | | | | | | | | | | |
| 2^7 | 12.1 | 2.2 | 91.4 | 1.21e+05 / 1.2e+05 | 0 | 4.48e+07 / 2.93e+08 | 786 / 1.52e+03 | 4.49e+07 | 2.93e+08 | 675457.9 | 4405395.8 | 6.52 | 189.0 | 202.0 | 89 / 106 | 1.20 (1.73) | 3344 | yes |
| 2^8 | 12.4 | 2.7 | 92.9 | 4.53e+05 / 4.51e+05 | 0 | 1.69e+08 / 1.23e+09 | 2.07e+03 / 6.54e+03 | 1.7e+08 | 1.23e+09 | 2307959.1 | 16678827.1 | 7.23 | 200.7 | 168.7 | 89 / 107 | 1.87 (1.73) | 13683 | yes |
| 2^7 | 13.4 | 2.4 | 93.0 | 1.86e+05 / 1.85e+05 | 0 | 2.84e+08 / 6.35e+08 | 1.31e+03 / 1.87e+03 | 2.84e+08 | 6.35e+08 | 2686978.2 | 6017586.0 | 2.24 | 201.6 | 160.8 | 87 / 110 | 1.77 (1.73) | 16714 | yes |
| 2^9 | 14.8 | 3.2 | 92.2 | 1.29e+06 / 1.29e+06 | 0 | 3.83e+08 / 2.78e+09 | 7.79e+03 / 4.03e+04 | 3.85e+08 | 2.79e+09 | 2244087.6 | 16254073.7 | 7.24 | 146.0 | 104.1 | 88 / 109 | 2.26 (1.73) | 21550 | yes |
| 2^9 | 15.1 | 2.8 | 92.9 | 9.27e+05 / 9.25e+05 | 0 | 5.15e+08 / 2.73e+09 | 5.13e+03 / 1.45e+04 | 5.16e+08 | 2.73e+09 | 2783147.6 | 14717631.2 | 5.29 | 171.7 | 143.9 | 89 / 105 | 1.93 (1.73) | 19338 | yes |
| 2^8 | 15.5 | 3.0 | 92.5 | 5.52e+05 / 5.5e+05 | 0 | 3.4e+08 / 1.33e+09 | 3.07e+03 / 8.97e+03 | 3.41e+08 | 1.33e+09 | 1579001.2 | 6186425.2 | 3.92 | 204.1 | 151.8 | 88 / 107 | 2.24 (1.73) | 10400 | yes |
| 2^8 | 15.5 | 2.6 | 92.1 | 6.26e+05 / 6.24e+05 | 0 | 3.74e+08 / 1.41e+09 | 3.73e+03 / 1.64e+04 | 3.74e+08 | 1.41e+09 | 1722359.5 | 6482738.5 | 3.76 | 167.7 | 153.1 | 88 / 107 | 1.63 (1.73) | 11247 | yes |
| 2^9 | 15.7 | 3.4 | 92.7 | 1.2e+06 / 1.2e+06 | 0 | 1.02e+09 / 2.46e+09 | 7.15e+03 / 2.79e+04 | 1.02e+09 | 2.46e+09 | 4495401.1 | 10819235.9 | 2.41 | 175.4 | 136.4 | 88 / 108 | 2.15 (1.73) | 32962 | yes |
| 2^10 | 16.2 | 3.3 | 93.7 | 1.28e+06 / 1.28e+06 | 0 | 1.25e+09 / 4.08e+09 | 6.06e+03 / 4.51e+04 | 1.25e+09 | 4.08e+09 | 4505118.5 | 14665701.5 | 3.26 | 152.1 | 143.5 | 88 / 110 | 1.50 (1.73) | 31393 | yes |
| 2^10 | 16.5 | 3.6 | 93.1 | 1.65e+06 / 1.65e+06 | 0 | 7.46e+08 / 3.53e+09 | 8.98e+03 / 5.19e+04 | 7.47e+08 | 3.53e+09 | 2437562.0 | 11513227.3 | 4.72 | 157.9 | 137.8 | 88 / 111 | 1.67 (1.73) | 17689 | yes |
| 2^9 | 17.1 | 3.5 | 93.4 | 7.79e+05 / 7.77e+05 | 0 | 4.77e+08 / 2.95e+09 | 5.15e+03 / 1.75e+04 | 4.77e+08 | 2.95e+09 | 1262964.5 | 7795194.7 | 6.17 | 151.2 | 96.3 | 87 / 109 | 2.67 (1.73) | 13111 | yes |
| 2^10 | 17.2 | 3.9 | 93.1 | 2.2e+06 / 2.19e+06 | 0 | 1.39e+09 / 3.48e+09 | 9.86e+03 / 1.16e+05 | 1.39e+09 | 3.49e+09 | 3598552.7 | 8997342.6 | 2.50 | 113.2 | 117.0 | 89 / 111 | 1.29 (1.73) | 30746 | yes |
| 2^11 | 17.9 | 4.3 | 93.1 | 3.58e+06 / 3.57e+06 | 0 | 2.58e+09 / 8.05e+09 | 3.6e+04 / 7.72e+05 | 2.58e+09 | 8.06e+09 | 5313399.7 | 16572286.9 | 3.12 | 138.9 | 122.0 | 88 / 114 | 1.59 (1.73) | 43562 | yes |
| 2^11 | 19.0 | 4.5 | 92.9 | 5.09e+06 / 5.08e+06 | 0 | 2.99e+09 / 1.1e+10 | 5.91e+04 / 8.88e+05 | 3e+09 | 1.1e+10 | 4151761.7 | 15217225.4 | 3.67 | 108.1 | 91.5 | 88 / 115 | 1.69 (1.73) | 45364 | yes |
| 2^10 | 19.2 | 4.0 | 93.0 | 2.11e+06 / 2.11e+06 | 0 | 9.06e+08 / 5.27e+09 | 1.86e+04 / 1.62e+05 | 9.08e+08 | 5.28e+09 | 1171590.0 | 6810054.5 | 5.81 | 124.8 | 59.6 | 87 / 112 | 3.57 (1.73) | 19641 | yes |
| 2^12 | 19.2 | 5.2 | 93.5 | 6.92e+06 / 6.91e+06 | 0 | 4.73e+09 / 1.83e+10 | 1e+05 / 1.86e+06 | 4.74e+09 | 1.83e+10 | 6034763.6 | 23337516.2 | 3.87 | 122.8 | 98.5 | 88 / 116 | 1.77 (1.73) | 61252 | yes |
| 2^12 | 20.2 | 5.6 | 93.5 | 6.99e+06 / 6.98e+06 | 0 | 6.53e+09 / 1.91e+10 | 1.36e+05 / 1.96e+06 | 6.54e+09 | 1.91e+10 | 5893919.5 | 17239831.3 | 2.93 | 105.5 | 102.3 | 88 / 118 | 1.40 (1.73) | 57626 | yes |
| 2^11 | 21.3 | 5.0 | 93.5 | 4.42e+06 / 4.41e+06 | 0 | 3.26e+09 / 1.64e+10 | 5.92e+04 / 8.84e+05 | 3.26e+09 | 1.64e+10 | 2052760.4 | 10299420.6 | 5.02 | 106.9 | 85.5 | 87 / 118 | 1.73 (1.73) | 24022 | yes |
| 2^11 | 21.7 | 5.0 | 93.4 | 5.09e+06 / 5.08e+06 | 0 | 4.35e+09 / 1.33e+10 | 7.37e+04 / 9.61e+05 | 4.36e+09 | 1.33e+10 | 2372929.0 | 7238675.9 | 3.05 | 109.8 | 97.3 | 87 / 119 | 1.53 (1.73) | 24398 | yes |
| 2^12 | 23.7 | 6.3 | 93.5 | 1.19e+07 / 1.19e+07 | 0 | 7.64e+09 / 2.5e+10 | 4.44e+05 / 1.08e+07 | 7.65e+09 | 2.5e+10 | 2056739.3 | 6721324.6 | 3.27 | 93.3 | 108.0 | 88 / 121 | 1.17 (1.73) | 19049 | yes |
| 2^12 | 23.8 | 6.7 | 93.5 | 1.29e+07 / 1.28e+07 | 0 | 9.59e+09 / 3.16e+10 | 5.73e+05 / 1.19e+07 | 9.6e+09 | 3.17e+10 | 2472468.6 | 8153526.3 | 3.30 | 79.9 | 93.8 | 89 / 121 | 1.15 (1.73) | 26350 | yes |
| 2^14 | 24.2 | 7.3 | 93.6 | 3.18e+07 / 3.18e+07 | 0 | 2.66e+10 / 8.32e+10 | 4.44e+06 / 1.2e+08 | 2.67e+10 | 8.34e+10 | 6125678.4 | 19151026.7 | 3.13 | 98.9 | 60.4 | 89 / 120 | 2.41 (1.73) | 101441 | yes |
| 2^13 | 25.0 | 7.2 | 93.4 | 2.05e+07 / 2.04e+07 | 0 | 1.26e+10 / 5.43e+10 | 1.3e+06 / 4.24e+07 | 1.26e+10 | 5.43e+10 | 2200628.7 | 9465212.7 | 4.30 | 111.6 | 72.5 | 89 / 122 | 2.19 (1.73) | 30373 | yes |
| 2^13 | 25.0 | 7.2 | 93.5 | 1.95e+07 / 1.95e+07 | 0 | 1.14e+10 / 5.05e+10 | 1.92e+06 / 3.65e+07 | 1.15e+10 | 5.05e+10 | 1984973.6 | 8743451.2 | 4.40 | 106.5 | 74.5 | 89 / 122 | 2.03 (1.73) | 26639 | yes |
| 2^14 | 27.0 | 7.8 | 93.6 | 4.22e+07 / 4.21e+07 | 0 | 3.68e+10 / 1.2e+11 | 1.38e+07 / 2.6e+08 | 3.69e+10 | 1.21e+11 | 3145232.1 | 10284379.7 | 3.27 | 126.3 | 80.3 | 89 / 124 | 2.19 (1.73) | 39150 | yes |

**Summary**

| arms | instances (of run) | log2 r | column ratio | full-rank ratio mean (min–max) | rank fraction at k = cols, π / negation | S neg / S π mean (min–max) | S π / rho S folded, min–max | rho steps ratio mean (expected) | rho muls per group op, negation / folded | rho S negation / rho S folded, mean | fitted exponent of total: π, negation, rho folded, rho negation (rho: 0.50) | all verified |
|:--|--:|:--|--:|--:|--:|--:|--:|--:|--:|--:|:--|:--|
| π-line fold vs negation, m = 3, the algebraic S₄ oracle | 26 (28) | 11.8–27.0 | 3.0 | 3.98 (2.34–6.73) | 0.93 / 0.93 | 4.17 (2.24–7.24) | 3344–101441 | 1.85 (1.73) | 88 / 113 | 1.25 | 0.53, 0.50, 0.41, 0.42 | yes |

**The solver, per instance**

| p | log2 r | solver calls | F_p muls per call | Macaulay muls per call | unsolved (border unreachable) | retried at 11 / 12 / 13 | root triples | unliftable systems | agreement with the pair table (S₄ only / pair table only) | wall s per call |
|--:|--:|--:|--:|--:|:--|:--|--:|--:|:--|--:|
| 2^7 | 10.7 | 1692 | 830626 | 488503 | 30 (30) | 0 / 0 / 0 | 126 | 24 | 300/300 (0 / 0) | 0.0037 |
| 2^8 | 11.8 | 1118 | 867628 | 499124 | 7 (7) | 0 / 0 / 0 | 181 | 0 | 300/300 (0 / 0) | 0.0038 |
| 2^7 | 11.9 | 3732 | 856630 | 496050 | 42 (42) | 0 / 0 / 0 | 312 | 30 | 300/300 (0 / 0) | 0.0038 |
| 2^7 | 12.1 | 344 | 848029 | 483948 | 9 (9) | 0 / 0 / 0 | 94 | 0 | 300/300 (0 / 0) | 0.0031 |
| 2^8 | 12.4 | 1393 | 876293 | 501507 | 5 (5) | 0 / 0 / 0 | 237 | 8 | 300/300 (0 / 0) | 0.0037 |
| 2^7 | 13.4 | 733 | 862917 | 495715 | 6 (6) | 0 / 0 / 0 | 116 | 0 | 300/300 (0 / 0) | 0.0043 |
| 2^9 | 14.8 | 3041 | 911512 | 504912 | 9 (9) | 0 / 0 / 0 | 672 | 0 | 300/300 (0 / 0) | 0.0041 |
| 2^9 | 15.1 | 3043 | 892869 | 502908 | 22 (22) | 0 / 0 / 0 | 509 | 4 | 300/300 (0 / 0) | 0.0040 |
| 2^8 | 15.5 | 1488 | 892496 | 500635 | 13 (13) | 0 / 0 / 0 | 324 | 7 | 300/300 (0 / 0) | 0.0043 |
| 2^8 | 15.5 | 1563 | 897067 | 499185 | 21 (21) | 0 / 0 / 0 | 393 | 6 | 300/300 (0 / 0) | 0.0035 |
| 2^9 | 15.7 | 2711 | 902830 | 504796 | 8 (8) | 0 / 0 / 0 | 530 | 3 | 300/300 (0 / 0) | 0.0040 |
| 2^10 | 16.2 | 4566 | 888582 | 505412 | 12 (12) | 0 / 0 / 0 | 515 | 6 | — | 0.0038 |
| 2^10 | 16.5 | 3878 | 905095 | 505666 | 9 (9) | 0 / 0 / 0 | 632 | 5 | — | 0.0039 |
| 2^9 | 17.1 | 3290 | 890752 | 503764 | 16 (16) | 0 / 0 / 0 | 451 | 0 | 300/300 (0 / 0) | 0.0041 |
| 2^10 | 17.2 | 3846 | 901266 | 505857 | 11 (11) | 0 / 0 / 0 | 671 | 0 | — | 0.0038 |
| 2^11 | 17.9 | 8777 | 912673 | 506741 | 17 (17) | 0 / 0 / 0 | 1429 | 0 | — | 0.0040 |
| 2^11 | 19.0 | 11733 | 930989 | 506745 | 26 (26) | 0 / 0 / 0 | 2193 | 0 | — | 0.0040 |
| 2^10 | 19.2 | 5784 | 906446 | 505544 | 23 (23) | 0 / 0 / 0 | 1086 | 0 | — | 0.0040 |
| 2^12 | 19.2 | 19841 | 917668 | 507505 | 13 (13) | 0 / 0 / 0 | 2856 | 6 | — | 0.0039 |
| 2^12 | 20.2 | 20527 | 925360 | 507369 | 23 (23) | 0 / 0 / 0 | 2899 | 6 | — | 0.0040 |
| 2^11 | 21.3 | 17710 | 918622 | 507069 | 27 (27) | 0 / 0 / 0 | 2573 | 8 | — | 0.0038 |
| 2^11 | 21.7 | 14325 | 921489 | 507295 | 15 (15) | 0 / 0 / 0 | 2373 | 4 | — | 0.0040 |
| 2^12 | 23.7 | 26271 | 944232 | 507707 | 19 (19) | 0 / 0 / 0 | 3984 | 3 | — | 0.0041 |
| 2^12 | 23.8 | 33251 | 944961 | 507795 | 16 (16) | 0 / 0 / 0 | 5162 | 6 | — | 0.0041 |
| 2^14 | 24.2 | 86822 | 951714 | 508025 | 21 (21) | 0 / 0 / 0 | 12674 | 2 | — | 0.0041 |
| 2^13 | 25.0 | 56915 | 946086 | 507908 | 22 (22) | 0 / 0 / 0 | 9556 | 7 | — | 0.0041 |
| 2^13 | 25.0 | 52962 | 945661 | 507915 | 21 (21) | 0 / 0 / 0 | 7993 | 0 | — | 0.0040 |
| 2^14 | 27.0 | 123747 | 964494 | 508088 | 19 (19) | 0 / 0 / 0 | 18742 | 10 | — | 0.0041 |

**Solver summary**

| calls | F_p muls per call (mean) | unsolved | unsolved fraction | agreement with the pair table | S₄ only | pair table only |
|--:|--:|--:|--:|:--|--:|--:|
| 515103 | 942213 | 482 | 0.0009 | 3600/3600 | 0 | 0 |

**Against the pair table (E8), the same instances, `S` in the same unit**

| p | log2 r | S π, S₄ oracle | S π, pair table | pair / S₄ | S negation, S₄ | S negation, pair table | pair / S₄ |
|--:|--:|--:|--:|--:|--:|--:|--:|
| 2^7 | 12.1 | 6.75e+05 | 1.09e+04 | 0.02 | 4.41e+06 | 4.91e+04 | 0.01 |
| 2^7 | 13.4 | 2.69e+06 | 3.98e+04 | 0.01 | 6.02e+06 | 8.4e+04 | 0.01 |
| 2^8 | 11.8 | 4.05e+06 | 8.32e+04 | 0.02 | 1.64e+07 | 2.95e+05 | 0.02 |
| 2^8 | 12.4 | 2.31e+06 | 6.5e+04 | 0.03 | 1.67e+07 | 3.63e+05 | 0.02 |
| 2^8 | 15.5 | 1.58e+06 | 4.91e+04 | 0.03 | 6.19e+06 | 1.65e+05 | 0.03 |
| 2^8 | 15.5 | 1.72e+06 | 5.61e+04 | 0.03 | 6.48e+06 | 1.81e+05 | 0.03 |
| 2^9 | 14.8 | 2.24e+06 | 1.4e+05 | 0.06 | 1.63e+07 | 7.63e+05 | 0.05 |
| 2^9 | 15.1 | 2.78e+06 | 1.19e+05 | 0.04 | 1.47e+07 | 5.39e+05 | 0.04 |
| 2^9 | 15.7 | 4.5e+06 | 2.21e+05 | 0.05 | 1.08e+07 | 4.94e+05 | 0.05 |
| 2^9 | 17.1 | 1.26e+06 | 5.21e+04 | 0.04 | 7.8e+06 | 2.77e+05 | 0.04 |
| 2^10 | 16.2 | 4.51e+06 | 2.43e+05 | 0.05 | 1.47e+07 | 7.38e+05 | 0.05 |
| 2^10 | 16.5 | 2.44e+06 | 1.68e+05 | 0.07 | 1.15e+07 | 6.74e+05 | 0.06 |
| 2^10 | 17.2 | 3.6e+06 | 2.91e+05 | 0.08 | 9e+06 | 6.67e+05 | 0.07 |
| 2^10 | 19.2 | 1.17e+06 | 1.08e+05 | 0.09 | 6.81e+06 | 5.2e+05 | 0.08 |
| 2^11 | 17.9 | 5.31e+06 | 6.89e+05 | 0.13 | 1.66e+07 | 1.97e+06 | 0.12 |
| 2^11 | 19.0 | 4.15e+06 | 6.16e+05 | 0.15 | 1.52e+07 | 2.04e+06 | 0.13 |
| 2^11 | 21.3 | 2.05e+06 | 3.18e+05 | 0.16 | 1.03e+07 | 1.45e+06 | 0.14 |
| 2^11 | 21.7 | 2.37e+06 | 4.12e+05 | 0.17 | 7.24e+06 | 1.17e+06 | 0.16 |

Reading.  The oracle is the one E11 asked for: `S₄(s t₁, s t₂, s t₃, x_R)` symmetrised
in the `t_i` (the fourth summation polynomial of `gaudry_cubic`, its coefficients
scaled by `s^{a + 2b + 3c}` so that the unknowns are the elementary symmetric
functions of the line parameters), Weil-descended to three `F_p` equations in three
unknowns and solved by the Macaulay matrix at degree `10`–`13`, the eigenvalues of
`e₁`, and the cubic split over `F_p`; the signs are settled by group arithmetic, so
nothing is returned that does not sum to the target.  It agrees with the pair table
on `3600` of `3600` targets at `p ≤ 2^9` (`0` decomposed by `S₄` alone,
`0` by the table alone), every decomposition it returns is checked, and the
relation stream it produces is E8's on the same instances — the same columns,
and the same trials and full-rank counts up to the few targets it leaves
unsolved — because an oracle changes what a target costs and not whether it
decomposes.  Its cost is independent of `p`:
`9.42e+05` `F_p` multiplications a call (`4.0` ms), `482` of
`515103` calls unsolved (no Macaulay degree closed the quotient; those targets are
skipped and reported, never guessed).  Against the pair table at `93`
multiplications an addition that is `10123` additions: the algebraic
oracle is the dearer one while `|F| < 10123` (`p ≲ 2^12`) and
the cheaper one above.  The comparison table holds the sizes both oracles were run
on, `p ≤ 2^11`, all below that crossover, and the pair table wins on every one of
them by the margin the per-target costs predict; above it E8 was not run — its
table at `p = 2^13` would hold `2^24` pairs — and E11 runs on at `2^12`, `2^13` and
`2^14` alone, where the per-target cost is the same `10⁶` multiplications as at
`2^7`.  The
fold's reading is unchanged — columns `÷ 3`, relations `÷ 3.98`, both arms at
`0.93` / `0.93` of full rank after `columns` relations, `S ÷ 4.17` — and
the fitted exponents of the total over `26` instances are fold `0.53`,
control `0.50`, rho `0.41` / `0.42`: with the oracle
flat in `p` the stream is `p · ln` solves at a constant each, ~~so the arms grow as `r^{1/3}`
times the coupon factor, below rho's `1/2` for the first time in this note~~ (**accounting
correction, §8.10:** on these instances `r ≈ p²/k` with `k ≤ 8`, not `p³`, so `p · ln` solves is
`r^{1/2+o(1)}` — rho's exponent, not below it; the measured `0.53` / `0.50` are that), and the
folded arm is still `3344`–`101441×` the matched rho at these sizes
(a constant of `10⁶` multiplications a solve against `10²` a step).  **Frobenius
symmetry breaking** (Galbraith–Granger–Merz–Petit's third lever) in the form the
driver can measure — one solve per `⟨−1, π⟩`-orbit of targets, the conjugate
systems' solutions being the conjugate decompositions — would save exactly the
orbit duplicates, `11056` of `512218` targets here (the birthday count of E7);
on a folded base a conjugate decomposition is the same row, so there is nothing
further to break at the system level, and the paper's own mechanism was not
available to this session to reproduce.  **Class: engineering** — the oracle moves
`S` and the exponent of the relation phase, not a ratio to a floor; the method is
not faster than rho at any size run.

### 8.6 E12 — the pair table over orbit representatives

**Runner:** `--exp e12 --bits 7,8,9,10,11 --seeds 4` (the Frobenius line, three summands) and
`--exp e12p --bits 16,20,24,28 --seeds 6` (`j = 0` and `j = 1728` on `F_p`, two summands).
**Data:** `experiments/23_glv_invariant_e12.{json,log}`, `experiments/23_glv_invariant_e12p.{json,log}`
(2026-10-02).  **Module:** `src/cryptanalysis/orbit_pair_table.rs`.

The pair table of E8 stores `P_i + P_j` for every pair of `⟨−1⟩`-representatives.  On a base
that is a union of `H`-orbits (`|H| = w`), `φ(P_i) + φ(P_j)` is the image of `P_i + P_j`, so one
entry per `H`-orbit of pairs suffices: the table is built from the pairs `(rep_c, P_j)` with the
first summand a column representative and `col_j ≥ c`, `|F|²/(2w)` additions in place of
`|F|²/4`.  It is keyed by an **exact orbit invariant** of the sum — on the subfield line the
minimal polynomial of `x` over `F_p` (`σ₁ = 3a`, `σ₂ = 3(a² − νbc)`,
`σ₃ = a³ + νb³ + ν²c³ − 3νabc`: `12` multiplications, closed form), on `j = 0` `x³`, on
`j = 1728` `x²` — so a probe computes the key of `R − P₃` (or of `R`), and on a hit walks the
orbit of the stored sum to find the `φ` that carries it to the probe, reads the images of the two
stored summands off the column map by their coefficients, and checks the result with one
addition.  Nothing is returned that does not sum to the target.  The negation table and the orbit
table run the same stream on the same instances, every phase counted; the orbit key's
multiplications are not field-counter operations, so they are counted separately
(`build_uncounted_muls`, `stream_uncounted_muls`) and added.  Target by target, both tables agree on
whether a target decomposes on `200` targets per instance at `p ≤ 2^9` (line) and `3000` at
`≤ 20` bits (`F_p`).

**The Frobenius line, every instance**

| family | p | log2 r | pair sums built, negation / orbit | ÷ | orbit entries | table cost, negation / orbit | ÷ | stream cost per target, negation / orbit | orbit / negation (falsifies "unchanged" above 1.20) | keys / hits / maps | key collisions / image mismatches | full-rank rel π, negation table / orbit table | S π, negation table / orbit table | S π ratio | S negation arm, negation table / orbit table | rho S folded | S π (orbit) / rho S folded | agreement (orbit only / negation only) | correct |
|:--|--:|--:|--:|--:|--:|--:|--:|--:|--:|:--|:--|:--|:--|--:|:--|--:|--:|:--|:--|
| subfield | 2^7 | 10.7 | 1122 / 396 | 2.83 | 363 | 9.12e+04 / 3.69e+04 | 2.48 | 3.48e+03 / 3.61e+03 | 1.037 | 106960 / 96 / 1152 | 0 / 0 | — / — | — / — | — | — / — | 220.48 | — | 200/200 (0 / 0) | yes |
| subfield | 2^7 | 11.9 | 3306 / 1140 | 2.90 | 1083 | 2.72e+05 / 1.07e+05 | 2.54 | 4.01e+03 / 4.17e+03 | 1.039 | 402956 / 264 / 3168 | 0 / 0 | — / — | — / — | — | — / — | 209.24 | — | 200/200 (0 / 0) | yes |
| subfield | 2^7 | 12.1 | 1806 / 630 | 2.87 | 570 | 1.47e+05 / 5.89e+04 | 2.50 | 8.46e+03 / 9.27e+03 | 1.096 | 22938 / 94 / 1128 | 0 / 0 | 19 / 19 | 10905.3 / 10232.9 | 1.066 | 48922.6 / 51899.5 | 201.81 | 51 | 200/200 (0 / 0) | yes |
| subfield | 2^7 | 13.4 | 2970 / 1026 | 2.89 | 969 | 2.44e+05 / 9.65e+04 | 2.53 | 1.11e+04 / 1.22e+04 | 1.101 | 69419 / 116 / 1392 | 0 / 0 | 46 / 46 | 39559.6 / 41741.6 | 0.948 | 83558.7 / 90181.4 | 160.68 | 260 | 200/200 (0 / 0) | yes |
| subfield | 2^8 | 11.8 | 6162 / 2106 | 2.93 | 2024 | 5.09e+05 / 1.99e+05 | 2.56 | 1.26e+04 / 1.39e+04 | 1.111 | 152271 / 181 / 2172 | 0 / 0 | 50 / 50 | 83311.0 / 85748.9 | 0.972 | 294774.6 / 320719.4 | 202.52 | 423 | 200/200 (0 / 0) | yes |
| subfield | 2^8 | 12.4 | 9900 / 3366 | 2.94 | 3264 | 8.22e+05 / 3.2e+05 | 2.57 | 1.55e+04 / 1.73e+04 | 1.115 | 242391 / 226 / 2712 | 0 / 0 | 42 / 42 | 65088.3 / 63766.4 | 1.021 | 363108.6 / 396199.8 | 168.64 | 378 | 200/200 (0 / 0) | yes |
| subfield | 2^8 | 15.5 | 17556 / 5940 | 2.96 | 5784 | 1.46e+06 / 5.65e+05 | 2.59 | 2.22e+04 / 2.48e+04 | 1.118 | 329238 / 321 / 3852 | 0 / 0 | 76 / 76 | 49176.4 / 49738.0 | 0.989 | 165635.6 / 179971.3 | 151.40 | 329 | 200/200 (0 / 0) | yes |
| subfield | 2^8 | 15.5 | 20880 / 7056 | 2.96 | 6897 | 1.76e+06 / 6.77e+05 | 2.59 | 2.37e+04 / 2.63e+04 | 1.112 | 329063 / 356 / 4272 | 0 / 0 | 110 / 110 | 56614.4 / 56747.8 | 0.998 | 182983.9 / 179423.2 | 154.42 | 367 | 200/200 (0 / 0) | yes |
| subfield | 2^9 | 14.8 | 68382 / 22968 | 2.98 | 22681 | 5.74e+06 / 2.2e+06 | 2.61 | 3.84e+04 / 4.34e+04 | 1.129 | 1322050 / 672 / 8064 | 0 / 0 | 100 / 100 | 140381.7 / 132564.2 | 1.059 | 762897.8 / 835297.2 | 104.12 | 1273 | 200/200 (0 / 0) | yes |
| subfield | 2^9 | 15.1 | 34782 / 11718 | 2.97 | 11514 | 2.91e+06 / 1.12e+06 | 2.60 | 2.98e+04 / 3.35e+04 | 1.125 | 986578 / 506 / 6072 | 0 / 0 | 99 / 99 | 117758.6 / 120234.5 | 0.979 | 534771.5 / 589269.4 | 144.07 | 835 | 200/200 (0 / 0) | yes |
| subfield | 2^9 | 15.7 | 59292 / 19926 | 2.98 | 19653 | 4.97e+06 / 1.91e+06 | 2.60 | 3.84e+04 / 4.33e+04 | 1.128 | 1123121 / 527 / 6324 | 0 / 0 | 225 / 225 | 220482.2 / 231731.0 | 0.951 | 492825.5 / 538892.6 | 136.13 | 1702 | 200/200 (0 / 0) | yes |
| subfield | 2^9 | 17.1 | 30450 / 10266 | 2.97 | 10080 | 2.54e+06 / 9.79e+05 | 2.59 | 3.04e+04 / 3.42e+04 | 1.122 | 1026086 / 451 / 5412 | 0 / 0 | 67 / 67 | 52009.2 / 53171.5 | 0.978 | 275945.3 / 304529.6 | 95.72 | 555 | 200/200 (0 / 0) | yes |
| subfield | 2^10 | 16.2 | 63756 / 21420 | 2.98 | 21150 | 5.39e+06 / 2.07e+06 | 2.61 | 4.25e+04 / 4.79e+04 | 1.127 | 2111799 / 508 / 6096 | 0 / 0 | 152 / 152 | 244136.0 / 260216.4 | 0.938 | 741149.8 / 820526.9 | 144.22 | 1804 | — | yes |
| subfield | 2^10 | 16.5 | 97656 / 32760 | 2.98 | 32439 | 8.25e+06 / 3.16e+06 | 2.61 | 4.96e+04 / 5.6e+04 | 1.130 | 2117592 / 629 / 7548 | 0 / 0 | 134 / 134 | 167920.6 / 168893.9 | 0.994 | 674017.5 / 740558.4 | 137.87 | 1225 | — | yes |
| subfield | 2^10 | 17.2 | 159600 / 53466 | 2.99 | 53016 | 1.35e+07 / 5.16e+06 | 2.61 | 6.24e+04 / 7.07e+04 | 1.132 | 2658720 / 676 / 8112 | 0 / 0 | 276 / 276 | 290891.1 / 302386.5 | 0.962 | 667511.2 / 728561.6 | 116.75 | 2590 | — | yes |
| subfield | 2^10 | 19.2 | 179352 / 60066 | 2.99 | 59595 | 1.51e+07 / 5.79e+06 | 2.61 | 6.6e+04 / 7.47e+04 | 1.131 | 4188561 / 1091 / 13092 | 0 / 0 | 196 / 196 | 107403.0 / 106509.6 | 1.008 | 518240.8 / 571223.9 | 59.39 | 1793 | — | yes |
| subfield | 2^11 | 17.9 | 444222 / 148518 | 2.99 | 147804 | 3.79e+07 / 1.45e+07 | 2.62 | 1.02e+05 / 1.16e+05 | 1.134 | 10221825 / 1430 / 17160 | 0 / 0 | 462 / 462 | 689078.9 / 721766.2 | 0.955 | 1971350.1 / 2175716.3 | 122.13 | 5910 | — | yes |
| subfield | 2^11 | 19.0 | 628056 / 209880 | 2.99 | 209034 | 5.37e+07 / 2.05e+07 | 2.62 | 1.2e+05 / 1.37e+05 | 1.134 | 15956526 / 2197 / 26364 | 0 / 0 | 612 / 612 | 620055.3 / 646360.0 | 0.959 | 2058898.2 / 2278312.0 | 91.58 | 7058 | — | yes |
| subfield | 2^11 | 21.3 | 628056 / 209880 | 2.99 | 209013 | 5.39e+07 / 2.05e+07 | 2.63 | 1.27e+05 / 1.43e+05 | 1.133 | 24977043 / 2569 / 30828 | 0 / 0 | 545 / 545 | 318265.2 / 334848.4 | 0.950 | 1451418.7 / 1619098.4 | 85.40 | 3921 | — | yes |
| subfield | 2^11 | 21.7 | 849162 / 283668 | 2.99 | 282675 | 7.28e+07 / 2.77e+07 | 2.63 | 1.44e+05 / 1.63e+05 | 1.134 | 23042681 / 2370 / 28440 | 0 / 0 | 776 / 776 | 410423.2 / 435247.6 | 0.943 | 1166259.4 / 1292431.7 | 97.09 | 4483 | — | yes |

**Summary, the Frobenius line**

| family | instances (full rank on both tables) | log2 r | w | pair sums ÷ mean (min–max) | table cost ÷ mean (min–max) | stream cost per target, orbit / negation, mean (max) | table share of total π, negation table, mean (max) | S π ratio, negation / orbit table, mean (min–max) | S π (orbit) / rho S folded, min–max | key collisions | image mismatches | agreement (orbit only / negation only) | all correct |
|:--|--:|:--|--:|--:|--:|--:|--:|--:|--:|--:|--:|:--|:--|
| subfield | 18 (20) | 10.7–21.7 | 6 | 2.95 (2.83–2.99) | 2.59 (2.48–2.63) | 1.114 (1.134) | 0.133 (0.238) | 0.982 (0.938–1.066) | 51–7058 | 0 | 0 | 2400/2400 (0 / 0) | yes |

**Summary, `F_p` (per-instance table: the tables binary, 48 rows)**

| family | instances (full rank on both tables) | log2 r | w | pair sums ÷ mean (min–max) | table cost ÷ mean (min–max) | stream cost per target, orbit / negation, mean (max) | table share of total π, negation table, mean (max) | S π ratio, negation / orbit table, mean (min–max) | S π (orbit) / rho S folded, min–max | key collisions | image mismatches | agreement (orbit only / negation only) | all correct |
|:--|--:|:--|--:|--:|--:|--:|--:|--:|--:|--:|--:|:--|:--|
| j0 | 24 (24) | 12.9–26.0 | 6 | 2.97 (2.90–3.00) | 2.77 (2.67–2.83) | 1.005 (1.017) | 0.263 (0.637) | 1.180 (1.010–1.577) | 50–3971 | 0 | 0 | 36000/36000 (0 / 0) | yes |
| j1728 | 24 (24) | 12.9–26.9 | 4 | 1.98 (1.95–2.00) | 1.91 (1.87–1.95) | 1.002 (1.007) | 0.066 (0.137) | 1.031 (1.007–1.068) | 109–6330 | 0 | 0 | 36000/36000 (0 / 0) | yes |

Reading.  **The table is `w/2` smaller**, as predicted: pair sums `÷ 2.95` on the line and
`÷ 2.97` on `j = 0` (`w = 6`), `÷ 1.98` on `j = 1728` (`w = 4`), approaching `w/2` from below as
the diagonal pairs (stored once by both tables) become a vanishing share; the distinct keys are a
few per cent fewer still.  **It is cheaper to build by a little less than that** — `÷ 2.59`,
`÷ 2.77`, `÷ 1.91` — because every entry pays its key: `12` multiplications against an
addition's `84`–`93` on the line, `2` at `0.04` additions each on `F_p`.  **The probe is not quite
unchanged.**  Per target the stream costs `1.114×` the negation table's on the line (`1.134×` at
worst) and `1.005×` / `1.002×` on `F_p`: the key on every probe, and the orbit walk and image
check on every hit.  The falsification target was a probe above `1.2×`; no instance reaches it,
so "unchanged" survives, at `+11–13 %` on the line.  Zero key collisions and zero image mismatches
on `9.77e+07` probes; the two tables agree on `74400` of `74400` targets.

**`S` moves by the table's share, in both directions.**  On `j = 0` the table is `26 %` of the
folded arm's total on average (`64 %` at most), and the orbit table lowers `S` by `1.18×` on mean
(`1.01`–`1.58`).  Part of that spread is not the table: where a target has more than one
decomposition the two tables can return different ones, the relation streams then differ (`139`
against `115`, `240` against `422` relations to full rank on two rows), and so does the rank they
reach per relation; on the rows where the counts are equal the gain is `1.04`–`1.34`, which is
what a `÷ 2.8` table at those rows' `6`–`40 %` shares predicts (`1 / (1 − t(1 − 1/2.8))`).  On `j = 1728` the table is `7 %` of the total
and `S` falls by `1.03×`.  On the Frobenius line the table is `13 %` and the probe's `+11 %` is
paid on a stream that is most of the rest, so the orbit table is **dearer** end to end: `S` ratio
`0.98` on mean (`0.94`–`1.07`).  The fold's own `S` ratio and every comparison with rho are
unchanged: the orbit table is `50`–`7058×` the matched folded rho on these instances, as the full
table was.

**Class: engineering (memory).**  The table shrinks by `w/2` and on `F_p` `j = 0` `S` falls with it
by the table's share; on the subfield line the same change raises `S` by `2 %`, and quoting its
`÷ 3` table there as a cost gain would be **relabelling**.  No ratio to a floor moves.  What the
measurement does say is where the regime is: E12 matters exactly where the pair table is the
cost, which is two summands on `F_p` at large `p` — the regime §5.2 found worst against rho.

### 8.7 E13 — FGHR's `2`-torsion symmetry and the fold, together

**Runner:** `--exp e13 --bits 7,8,9,10,11,12 --seeds 4` and `--exp e13 --bits 13 --seeds 2`.
**Data:** `experiments/23_glv_invariant_e13.{json,log}`, `experiments/23_glv_invariant_e13_13.{json,log}`
(2026-10-02).  **Module:** `src/cryptanalysis/fghr_line.rs`.

**The object.**  A subfield curve `E/F_p` (E5, E8) with a rational `2`-torsion point
`T = (x₀, 0)` whose `f'(x₀) = 3x₀² + a = c²` is a square.  The Möbius coordinate
`Y = (x − x₀ − c)/(x − x₀ + c)` satisfies `Y(P + T) = −Y(P)` and `Y(−P) = Y(P)`, so the
**`Y`-line** `Y ∈ L·F_p` (`L = s` or `s²`, `s³ = ν`) is stable under `−1`, `π` and the
translation `τ_T`.  `T` lies in `E(F_p)`, the `π`-fixed cofactor, so `τ_T` acts on `⟨G⟩` with
eigenvalue `1`: `P` and `P + T` are one column with one coefficient, and the base folds by
`⟨−1, π, τ_T⟩` — **`12` points a column**, against `6` for `⟨−1, π⟩` on the same line and `2` for the
negation control.  That is the `2`-torsion used on the **base**.  On the **system**, `S₄`
rewritten in `Y` (by the substitution `x = ((c − x₀)Y + (x₀ + c))/(1 − Y)`) is invariant under the
even sign changes of `(Y₁, Y₂, Y₃)` — FGHR's `(Z/2)^{m−1}` at `m = 3` — and under their
permutations: `D₃ = (Z/2)² ⋊ S₃`, with invariants `p₁ = ΣY_i²`, `p₂ = ΣY_i²Y_j²`,
`p₃ = Y₁Y₂Y₃`.  On the line `p₁ = L²q₁`, `p₂ = L⁴q₂`, `p₃ = L³q₃` (weighted degree
`2i + 2j + k ≤ 4`); the Weil descent of `S₄(…, x_R)` is three **conics** in `(q₁, q₂)` with
coefficients in `F_p[q₃]`, their Macaulay resultant (the `15 × 15` determinant over its `3 × 3`
minor, interpolated at `17` points and checked at an `18`th) is a polynomial `R(q₃)`, its roots come
from Cantor–Zassenhaus, and each root gives `q₁, q₂` from the conics, `t_i²` from
`Z³ − q₁Z² + q₂Z − q₃²`, and the signs from `t₁t₂t₃ = q₃`; group arithmetic picks which of the
`τ_T`-variants of a solution to return (one, uniformly, per target).  The `S₃`-symmetrised
presentation of the same `S₄` in `Y` — E11's oracle moved to this line — is solved by the
existing Macaulay solver for comparison.  The driver runs four streams per instance — fold `12`
and fold `6`, each under `D₃` and (at `p ≤ 2^{10}`) under `S₃` — and every stream carries the
negation arm; the oracles are checked against the pair table target by target at `p ≤ 2^9`.

**The negation arm is not priced.**  On the four instances whose streams drew every target of the
group it ended at `40`/`48`, `12`/`24`, `100`/`114` and `169`/`174` of its columns, and in
development runs (not frozen) it stalled below full rank on larger instances too; whether that is a
structural ceiling of the `Y`-line negation base — a block degeneracy of the kind §6.5 found for two
summands under a `π`-fixed cofactor, now with `T` in that cofactor — is not derived here.  The
streams therefore stop when the folded arm is at full rank and both arms have pinned a column
(`StopRule::FoldedSquareBothPinned`), and every ratio below is between folded arms.  The
plan's base lever "columns `÷ 3`" (fold against negation) is consequently not measured; what is
measured is the base lever the `2`-torsion itself adds, `τ_T` on top of `⟨−1, π⟩`.

**Relations and solvers, every instance**

| p | log2 r | h | cols fold 12 / fold 6 / negation | full-rank rel fold 12 / fold 6 (D₃) | full-rank rel fold 12 / fold 6 (S₃) | negation rank at stop, D₃ streams (of cols) | D₃ calls fold 12 / fold 6 | D₃ muls per call | R(q₃) degree max | D₃ unsolved | S₃ muls per call | S₃ quotient per call | S₃ / D₃ muls per call | agreement with the pair table: D₃ / S₃ disagreements (targets, pair-table hits) | correct |
|--:|--:|--:|:--|:--|:--|:--|:--|--:|--:|--:|--:|--:|--:|:--|:--|
| 2^7 | 10.3 | 700 | 8 / 16 / 48 | — / — | — / — | 40 / 40 (48) | 1302 / 1302 | 13608 | 16 | 0 | 537898 | 64.0 | 39.5 | 0 / 0 (200, 14) | yes |
| 2^7 | 11.9 | 76 | 4 / 8 / 24 | — / — | — / — | 12 / 12 (24) | 3942 / 3942 | 12462 | 16 | 0 | 515868 | 63.9 | 41.4 | 0 / 0 (200, 2) | yes |
| 2^8 | 12.8 | 1708 | 19 / 38 / 114 | — / — | — / — | 100 / 100 (114) | 7026 / 7026 | 15405 | 16 | 0 | 572396 | 64.0 | 37.2 | 0 / 0 (200, 13) | yes |
| 2^8 | 12.9 | 1820 | 19 / 38 / 114 | 23 / 75 | 23 / 93 | 88 / 88 (114) | 2004 / 2004 | 15668 | 16 | 0 | 569442 | 64.0 | 36.3 | 0 / 0 (200, 14) | yes |
| 2^7 | 13.2 | 112 | 9 / 18 / 54 | 12 / 31 | 12 / 31 | 29 / 31 (54) | 520 / 557 | 13853 | 16 | 0 | 546165 | 64.0 | 39.4 | 0 / 0 (200, 12) | yes |
| 2^9 | 13.8 | 2464 | 29 / 58 / 174 | — / — | — / — | 169 / 169 (174) | 14712 / 14712 | 15824 | 16 | 0 | 571708 | 64.0 | 36.1 | 0 / 0 (200, 10) | yes |
| 2^7 | 14.2 | 112 | 10 / 20 / 60 | 13 / 43 | 13 / 43 | 45 / 45 (60) | 2505 / 2505 | 14969 | 16 | 0 | 543662 | 64.0 | 36.3 | 0 / 0 (200, 8) | yes |
| 2^8 | 15.4 | 184 | 19 / 38 / 114 | 36 / 46 | 36 / 133 | 82 / 82 (114) | 1280 / 1280 | 14779 | 16 | 0 | 571453 | 64.0 | 38.7 | 0 / 0 (200, 9) | yes |
| 2^8 | 15.5 | 244 | 21 / 42 / 126 | 26 / 68 | 26 / 58 | 93 / 93 (126) | 2020 / 2020 | 15532 | 16 | 0 | 567103 | 64.0 | 36.5 | 0 / 0 (200, 9) | yes |
| 2^9 | 17.4 | 404 | 39 / 78 / 234 | 45 / 144 | 45 / 188 | 214 / 214 (234) | 3394 / 3394 | 16302 | 16 | 0 | 586238 | 64.0 | 36.0 | 0 / 0 (200, 14) | yes |
| 2^9 | 17.8 | 448 | 37 / 74 / 222 | 51 / 231 | 51 / 187 | 193 / 199 (222) | 6077 / 7087 | 16529 | 16 | 0 | 582719 | 64.0 | 35.2 | 0 / 0 (200, 9) | yes |
| 2^9 | 17.9 | 464 | 42 / 84 / 252 | 59 / 156 | 59 / 204 | 223 / 223 (252) | 5321 / 5321 | 16772 | 16 | 0 | 588343 | 64.0 | 35.1 | 0 / 0 (200, 4) | yes |
| 2^11 | 18.3 | 10864 | 116 / 232 / 696 | 234 / 645 | — | 234 / 623 (696) | 6680 / 19688 | 19459 | 16 | 0 | — | — | — | — | yes |
| 2^10 | 19.1 | 776 | 66 / 132 / 396 | 187 / 254 | 187 / 593 | 357 / 357 (396) | 8238 / 8238 | 18252 | 16 | 0 | 597605 | 64.0 | 32.7 | — | yes |
| 2^10 | 19.1 | 700 | 58 / 116 / 348 | 87 / 232 | 87 / 408 | 318 / 318 (348) | 9793 / 9793 | 17725 | 16 | 0 | 590320 | 64.0 | 33.3 | — | yes |
| 2^10 | 19.2 | 688 | 59 / 118 / 354 | 73 / 218 | 73 / 318 | 271 / 271 (354) | 8575 / 8575 | 17118 | 16 | 0 | 593736 | 64.0 | 34.7 | — | yes |
| 2^10 | 19.4 | 892 | 72 / 144 / 432 | 131 / 251 | 119 / 319 | 396 / 396 (432) | 9669 / 9669 | 17362 | 16 | 0 | 593492 | 64.0 | 34.2 | — | yes |
| 2^12 | 19.9 | 17360 | 213 / 426 / 1278 | 651 / 821 | — | 1163 / 1163 (1278) | 29169 / 29169 | 20267 | 16 | 0 | — | — | — | — | yes |
| 2^11 | 20.6 | 1304 | 110 / 220 / 660 | 330 / 622 | — | 597 / 607 (660) | 13539 / 14078 | 19217 | 16 | 0 | — | — | — | — | yes |
| 2^11 | 20.6 | 1288 | 104 / 208 / 624 | 208 / 520 | — | 574 / 574 (624) | 15131 / 15131 | 19043 | 16 | 0 | — | — | — | — | yes |
| 2^11 | 21.4 | 1760 | 135 / 270 / 810 | 306 / 711 | — | 730 / 730 (810) | 20100 / 20100 | 18013 | 16 | 0 | — | — | — | — | yes |
| 2^12 | 22.7 | 2528 | 211 / 422 / 1266 | 367 / 1090 | — | 1162 / 1162 (1266) | 29638 / 29638 | 20242 | 16 | 0 | — | — | — | — | yes |
| 2^13 | 23.1 | 55804 | 639 / 1278 / 3834 | 1560 / 2981 | — | 2877 / 2981 (3834) | 74617 / 77969 | 20436 | 16 | 0 | — | — | — | — | yes |
| 2^12 | 23.6 | 3472 | 302 / 604 / 1812 | 542 / 1151 | — | 1663 / 1663 (1812) | 38421 / 38421 | 20048 | 16 | 0 | — | — | — | — | yes |
| 2^12 | 23.7 | 3760 | 318 / 636 / 1908 | 844 / 1640 | — | 1762 / 1762 (1908) | 41198 / 41198 | 19451 | 16 | 0 | — | — | — | — | yes |
| 2^13 | 24.7 | 5260 | 424 / 848 / 2544 | 834 / 2848 | — | 2340 / 2452 (2544) | 62824 / 76407 | 19825 | 16 | 0 | — | — | — | — | yes |

**Every phase priced (F_p multiplications; inversion = measured factor; each solver's own count; the polynomials' set-up charged to every arm; LA row op = 1); folded arms only**

| p | log2 r | S fold 12 + D₃ | S fold 6 + D₃ | S fold 12 + S₃ | S fold 6 + S₃ | base lever: S(6, D₃) / S(12, D₃) | system lever: S(12, S₃) / S(12, D₃) | product | combined: S(6, S₃) / S(12, D₃) | combined / product (falsifies "multiply" below 0.80) | rho S folded | S(12, D₃) / rho S folded |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 2^7 | 10.3 | not full rank | not full rank | not full rank | not full rank | — | — | — | — | — | 258.3 | — |
| 2^7 | 11.9 | not full rank | not full rank | not full rank | not full rank | — | — | — | — | — | 227.9 | — |
| 2^8 | 12.8 | not full rank | not full rank | not full rank | not full rank | — | — | — | — | — | 184.1 | — |
| 2^8 | 12.9 | 96092.2 | 374345.6 | 2890711.4 | 13512186.9 | 3.90 | 30.08 | 117.19 | 140.62 | 1.20 | 163.8 | 587 |
| 2^7 | 13.2 | 44098.4 | 102127.9 | 1281290.8 | 3095734.2 | 2.32 | 29.06 | 67.29 | 70.20 | 1.04 | 134.9 | 327 |
| 2^9 | 13.8 | not full rank | not full rank | not full rank | not full rank | — | — | — | — | — | 136.6 | — |
| 2^7 | 14.2 | 96459.7 | 343441.2 | 2875207.7 | 9902415.0 | 3.56 | 29.81 | 106.13 | 102.66 | 0.97 | 130.0 | 742 |
| 2^8 | 15.4 | 43160.8 | 62595.6 | 1219000.2 | 6102299.8 | 1.45 | 28.24 | 40.96 | 141.39 | 3.45 | 158.4 | 272 |
| 2^8 | 15.5 | 61216.3 | 147282.1 | 1685340.1 | 3725555.1 | 2.41 | 27.53 | 66.24 | 60.86 | 0.92 | 135.6 | 451 |
| 2^9 | 17.4 | 37871.0 | 116163.8 | 988070.2 | 4395394.4 | 3.07 | 26.09 | 80.03 | 116.06 | 1.45 | 141.9 | 267 |
| 2^9 | 17.8 | 65373.2 | 323941.4 | 1744178.5 | 7095370.7 | 4.96 | 26.68 | 132.21 | 108.54 | 0.82 | 103.5 | 632 |
| 2^9 | 17.9 | 62216.2 | 147759.2 | 1643897.8 | 5837460.1 | 2.37 | 26.42 | 62.75 | 93.83 | 1.50 | 105.7 | 588 |
| 2^11 | 18.3 | 293056.3 | 850444.3 | — | — | 2.90 | — | — | — | — | 91.0 | 3220 |
| 2^10 | 19.1 | 134658.6 | 181276.9 | 3394852.8 | 11522881.7 | 1.35 | 25.21 | 33.94 | 85.57 | 2.52 | 88.8 | 1517 |
| 2^10 | 19.1 | 80293.8 | 226282.8 | 2021437.3 | 9904844.4 | 2.82 | 25.18 | 70.95 | 123.36 | 1.74 | 81.5 | 985 |
| 2^10 | 19.2 | 69454.1 | 206218.3 | 1808211.7 | 7733203.2 | 2.97 | 26.03 | 77.30 | 111.34 | 1.44 | 123.5 | 562 |
| 2^10 | 19.4 | 99983.7 | 171876.6 | 2375986.7 | 5548815.8 | 1.72 | 23.76 | 40.85 | 55.50 | 1.36 | 103.0 | 971 |
| 2^12 | 19.9 | 434231.3 | 546091.0 | — | — | 1.26 | — | — | — | — | 75.6 | 5741 |
| 2^11 | 20.6 | 153079.7 | 279830.9 | — | — | 1.83 | — | — | — | — | 87.2 | 1756 |
| 2^11 | 20.6 | 114411.2 | 268794.3 | — | — | 2.35 | — | — | — | — | 85.5 | 1338 |
| 2^11 | 21.4 | 125913.9 | 283597.2 | — | — | 2.25 | — | — | — | — | 64.1 | 1964 |
| 2^12 | 22.7 | 100636.5 | 288902.4 | — | — | 2.87 | — | — | — | — | 85.9 | 1172 |
| 2^13 | 23.1 | 379369.3 | 724818.7 | — | — | 1.91 | — | — | — | — | 79.6 | 4764 |
| 2^12 | 23.6 | 97272.7 | 194746.3 | — | — | 2.00 | — | — | — | — | 90.4 | 1076 |
| 2^12 | 23.7 | 138787.3 | 273503.3 | — | — | 1.97 | — | — | — | — | 77.1 | 1801 |
| 2^13 | 24.7 | 121355.3 | 397487.2 | — | — | 3.28 | — | — | — | — | 77.5 | 1565 |

**The phases of fold 12 + D₃, rows at full rank (share of the total)**

| p | log2 r | base build | polynomial set-up | stream group arithmetic | D₃ solver | linear algebra | total | dominant phase |
|--:|--:|--:|--:|--:|--:|--:|--:|:--|
| 2^8 | 12.9 | 6e+05 (0.07) | 2.07e+05 (0.02) | 1.5e+06 (0.18) | 6.11e+06 (0.73) | 911 (0.00) | 8.42e+06 | solver |
| 2^7 | 13.2 | 1.91e+05 (0.04) | 2.07e+05 (0.05) | 8.05e+05 (0.19) | 3.15e+06 (0.72) | 298 (0.00) | 4.35e+06 | solver |
| 2^7 | 14.2 | 2.25e+05 (0.02) | 2.07e+05 (0.02) | 2.63e+06 (0.20) | 9.99e+06 (0.77) | 406 (0.00) | 1.31e+07 | solver |
| 2^8 | 15.4 | 4.76e+05 (0.05) | 2.07e+05 (0.02) | 1.81e+06 (0.20) | 6.44e+06 (0.72) | 840 (0.00) | 8.94e+06 | solver |
| 2^8 | 15.5 | 5.45e+05 (0.04) | 2.07e+05 (0.02) | 2.59e+06 (0.20) | 9.7e+06 (0.74) | 905 (0.00) | 1.3e+07 | solver |
| 2^9 | 17.4 | 1.05e+06 (0.07) | 2.07e+05 (0.01) | 3.2e+06 (0.21) | 1.11e+07 (0.71) | 2.53e+03 (0.00) | 1.56e+07 | solver |
| 2^9 | 17.8 | 1.06e+06 (0.03) | 2.07e+05 (0.01) | 6.65e+06 (0.22) | 2.29e+07 (0.74) | 1.9e+03 (0.00) | 3.08e+07 | solver |
| 2^9 | 17.9 | 1.2e+06 (0.04) | 2.07e+05 (0.01) | 6.65e+06 (0.21) | 2.3e+07 (0.74) | 2.55e+03 (0.00) | 3.1e+07 | solver |
| 2^11 | 18.3 | 4.61e+06 (0.03) | 2.07e+05 (0.00) | 3.36e+07 (0.20) | 1.3e+08 (0.77) | 5.69e+03 (0.00) | 1.68e+08 | solver |
| 2^10 | 19.1 | 2e+06 (0.02) | 2.07e+05 (0.00) | 2.18e+07 (0.22) | 7.67e+07 (0.76) | 5.02e+03 (0.00) | 1.01e+08 | solver |
| 2^10 | 19.1 | 1.92e+06 (0.03) | 2.07e+05 (0.00) | 1.32e+07 (0.22) | 4.49e+07 (0.75) | 4.32e+03 (0.00) | 6.02e+07 | solver |
| 2^10 | 19.2 | 1.79e+06 (0.03) | 1.69e+05 (0.00) | 1.19e+07 (0.22) | 3.93e+07 (0.74) | 4.06e+03 (0.00) | 5.32e+07 | solver |
| 2^10 | 19.4 | 2.56e+06 (0.03) | 2.07e+05 (0.00) | 1.89e+07 (0.23) | 6.18e+07 (0.74) | 4.72e+03 (0.00) | 8.34e+07 | solver |
| 2^12 | 19.9 | 9.04e+06 (0.02) | 2.07e+05 (0.00) | 9e+07 (0.21) | 3.27e+08 (0.77) | 3.67e+04 (0.00) | 4.26e+08 | solver |
| 2^11 | 20.6 | 3.72e+06 (0.02) | 2.07e+05 (0.00) | 4.34e+07 (0.22) | 1.47e+08 (0.76) | 7.84e+03 (0.00) | 1.94e+08 | solver |
| 2^11 | 20.6 | 3.46e+06 (0.02) | 2.07e+05 (0.00) | 3.26e+07 (0.22) | 1.1e+08 (0.75) | 1.16e+04 (0.00) | 1.46e+08 | solver |
| 2^11 | 21.4 | 4.77e+06 (0.02) | 2.07e+05 (0.00) | 5.09e+07 (0.24) | 1.54e+08 (0.73) | 1.48e+04 (0.00) | 2.1e+08 | solver |
| 2^12 | 22.7 | 7.95e+06 (0.03) | 2.07e+05 (0.00) | 5.98e+07 (0.23) | 1.91e+08 (0.74) | 2.39e+04 (0.00) | 2.59e+08 | solver |
| 2^13 | 23.1 | 3.22e+07 (0.03) | 2.07e+05 (0.00) | 2.66e+08 (0.24) | 8.23e+08 (0.73) | 4.6e+05 (0.00) | 1.12e+09 | solver |
| 2^12 | 23.6 | 1.13e+07 (0.03) | 2.07e+05 (0.00) | 8.4e+07 (0.24) | 2.52e+08 (0.73) | 7.44e+04 (0.00) | 3.48e+08 | solver |
| 2^12 | 23.7 | 1.22e+07 (0.02) | 2.07e+05 (0.00) | 1.29e+08 (0.25) | 3.75e+08 (0.73) | 9.94e+04 (0.00) | 5.16e+08 | solver |
| 2^13 | 24.7 | 1.69e+07 (0.03) | 2.07e+05 (0.00) | 1.62e+08 (0.26) | 4.54e+08 (0.72) | 1.23e+05 (0.00) | 6.33e+08 | solver |

Fitted exponent of the fold 12 + D₃ total against p (the line's field size, over the 22 rows above; the stream is p·ln p solves at a cost that grows with log p, so 1 + o(1) is expected): 1.23.

**Summary (rows where both D₃ arms reached full rank)**

| instances (of run) | log2 r | column ratio fold 12 / fold 6 | base lever S(6, D₃) / S(12, D₃), mean (min–max) | rows with S₃ on both bases | system lever S(12, S₃) / S(12, D₃), mean (min–max) | combined S(6, S₃) / S(12, D₃), mean (min–max) | combined / product, min | S(12, D₃) / rho S folded, min–max | fitted exponent of total against r: fold 12 + D₃, fold 6 + D₃, rho folded (rho: 0.50) | all correct |
|--:|:--|--:|--:|--:|--:|--:|--:|--:|:--|:--|
| 22 (26) | 10.3–24.7 | 0.5 | 2.52 (1.26–4.96) | 12 | 27.01 (23.76–30.08) | 100.83 (55.50–141.39) | 0.82 | 267–5741 | 0.63, 0.60, 0.41 | yes |

Reading.  **The system lever is FGHR's, measured.**  `R(q₃)` has degree `16` — `64/4`, the
`S₃` quotient divided by `2^{m−1}` — on the calls that reach it (`866919` `D₃` calls, mean degree
`15.999`, maximum `16`, `0` failed degree checks, `0` unsolved, `0` unliftable), while the
`S₃` presentation of the same polynomial on the same line has a quotient of `63.9`–`64.0` a call.
A `D₃` call costs `1.25e+04`–`2.04e+04` multiplications, growing slowly with `p` (the root
finding is `O(log p)` multiplications per degree), against `5.2e+05`–`6.0e+05` for `S₃`
(`÷ 33`–`41`) and against E11's `9.42e+05` on the `x`-line (`÷ 46`–`76`).  Both oracles agree with the
pair table on every one of `2400` targets at `p ≤ 2^9` (`118` of them decomposable), and every
decomposition either returns is checked by group arithmetic.  End to end, on the fold-`12` base,
`S(12, S₃) / S(12, D₃) = 27.0` on mean (`23.8`–`30.1`): the solver is `71`–`77 %` of the
`D₃` arm's total on every row, so the per-call ratio arrives nearly whole.

**The base lever is the `2`-torsion's too.**  `τ_T` halves the columns exactly on every row and,
under `D₃`, `S(6, D₃) / S(12, D₃) = 2.52` on mean (`1.26`–`4.96`), the relations to full rank
falling `÷ 1.26`–`4.53` with them (the column ratio `2`, times the scatter of where full rank
lands).  On the fold-`12` base both oracles produce the same relation stream (a `τ_T`-variant of a
decomposition is the same row), and they do on `11` of `12` rows; on the fold-`6` base the
variants are different rows, and the oracle's choice among them matters: `D₃` picks one uniformly
per target, the Macaulay solver's choice is correlated across targets, and `S₃` needs
`0.81`–`2.89×` `D₃`'s relations to reach full rank there.  That is a property of solution
selection, not of either symmetry.

**"Multiply" survives its test.**  Combined, `S(6, S₃) / S(12, D₃) = 101` on mean (`55`–`141`);
the product of the separate ratios is `34`–`132`, and combined / product is `0.82`–`3.45`, never
below the `0.80` that would have falsified "multiply" (closest: `0.82` at `p = 2^9`).  The excess on
`9` of `12` rows is the selection effect above, which inflates the fold-`6` `S₃` arm; with
uniform selection it should sit at the product, which is not measured here.  So the two uses of `T` — a fold of the base and a
symmetry of the system — act on different objects and compound, as §8.0 predicted for the fold
and the system symmetries in general.

**Against rho, and the phases.**  `S(12, D₃)` is `3.8e+04`–`4.3e+05` against the matched folded
rho's `64`–`164` in the same unit: `267`–`5741×` rho, against E11's `3344`–`101441×` — one to
two orders of magnitude closer, by constants.  The solver dominates (`71`–`77 %`), the stream's
group arithmetic is `18`–`26 %`, the base build `2`–`7 %`, the polynomials' set-up `≤ 5 %`, the linear
algebra `< 0.1 %`.  The total grows as `p^{1.23}` (fitted over `p = 2^7`–`2^{13}`, `22` rows);
against `r` the fits are `0.63` (fold `12`), `0.60` (fold `6`) and `0.41` (rho), ~~but `r ≈ p³/h`
with `h` from `76` to `55804` across these instances, so `r` is a poor size variable here and a
fit against it mixes the growth in `p` with the spread in `h`.  **Extrapolation, not measurement:** at fixed cofactor `p^{1.23}` is
`r^{0.41}`, so `S` falls as `r^{−0.09}` and a `10³` gap to rho would close only after `r` grows by
about `2^{110}` — on an exponent fitted over seven sizes of `p`.~~  **Accounting correction (§8.10):**
the cofactor cannot be held fixed — it contains `E(F_p)`, so `h ≥ #E(F_p) ≈ p` and `r ≈ p²/k`
(`k ≤ 8` here) — and the fit against `r` is the right reading: `0.63` against rho's `1/2`, a gap
that **widens** with size.  E16 measures it to `p = 2^{16}`.  Four small instances
(`r ≤ 2^{13.8}`) drew every target of the group with the folded arms at rank `columns` of
`columns + 1`, as E8 and E12 did at `p = 2^7`; they are reported, not priced.

**Class: engineering.**  `S` falls by `27×` (the system symmetry, against this line's `S₃`
oracle) and by `2.5×` (the `τ_T` fold) on top of E11's, and the ratio to rho falls by the same
constants; nothing moves an exponent (`p^{1.23}`, the same `p^{1+o(1)}` relation phase as E11),
and the method is not faster than rho at any size run.

### 8.8 E15 — does the §6.5 degeneracy reach the ECC2K-130 family?

**Runner:** `--exp e15 --bits 13,…,33 --seeds 4` (`m` from `13` to `33`; degrees with no
instance or no usable invariant subspace are logged and skipped).  **Data:**
`experiments/23_glv_invariant_e15.{json,log}` (2026-10-02).

**The question.**  On a subfield curve `E(F_p)` is cofactor and fixed by `π`, and two-summand
relations on the Frobenius line pair a point with its own orbit: every folded row is a
single-column row and the fold buys columns but not relations (§6.5).  The ECC2K-130 group has
the same shape in miniature — `E_0: y² + xy = x³ + 1` over `GF(2^{131})` has order `4·prime`, and
the cofactor `E_0(F_2) ≅ Z/4` is fixed by the Frobenius `τ`.  E15 runs E2's two-summand stream —
the signed-Frobenius-orbit base on a `τ`-invariant subspace against the abscissa control, the
pair-table oracle, both arms to full rank — on `E_0` itself at every degree the driver can carry,
and reads the structural deficiency `D` (§6.0) and the single-column rows.  Prediction (§8.1):
with `h = 4` there is at most one non-self-negative component class, so `D ≤ 1`, no block
structure, and the fold buys relations; `D > 2` on a `4·prime` subgroup falsifies.

**Which degrees are faithful.**  `#E_0(GF(2^m))` is `4·prime` at `m = 13, 19, 23, 41` in the range
`11`–`61` (and at `83` and `131`), and not at `31`: `#E_0(GF(2^{31})) = 4 · 373 · 1439393`, so every
prime-order subgroup there carries a cofactor of at least `4·373`.  Of the faithful degrees, `13`
(`ord₁₃(2) = 12`) and `19` (`18`) have no proper `τ`-invariant subspace — the base would be the
whole group (`13`, run and reported) or `2.6·10⁵` points (`19`, beyond the pair table) — and `41`
(`r ≈ 2^{39}`, a `20`-dimensional subspace) is beyond this driver.  That leaves **`m = 23`**:
`icv1-f2m23-t5197-69e76b73`, `r = 2095853`, `h = 4`, no intermediate subfield, `ord₂₃(2) = 11`
— so, as at `m = 31` and unlike `53`, `83` and `131`, the nontrivial cyclotomic block splits
(AGENTS.md §8b), which is what gives the `11`-dimensional invariant subspace the base is built on.
`m = 15` (subfields `GF(2^3)`, `GF(2^5)`) is reported separately and supports nothing at a prime
degree.

**Every instance**

| curve | m | ord_m(2) | intermediate subfields | log2 r | h | h = #E_0(F_2) | subspace dim | seed | cols fold / control | D fold / control | single-column rows fold / control (of relations) | chance, 1 / cols fold | full-rank rel fold / control | ratio | column ratio | rank fraction at k = cols, fold / control | correct |
|:--|--:|--:|:--|--:|:--|:--|--:|--:|:--|:--|:--|--:|:--|--:|--:|:--|:--|
| `icv1-f2m13-t181-515ee569` | 13 | 12 | none | 11.0 | 4 = 2·2 | yes | 12 | 1 | 155 / 2003 | 1 / 1 | 0 (0.000) / 0 (0.000) | 0.006 | — / — | — | 12.92 | 0.46 / — | yes |
| `icv1-f2m13-t181-515ee569` | 13 | 12 | none | 11.0 | 4 = 2·2 | yes | 12 | 2 | 155 / 2003 | 1 / 1 | 0 (0.000) / 0 (0.000) | 0.006 | — / — | — | 12.92 | 0.46 / — | yes |
| `icv1-f2m13-t181-515ee569` | 13 | 12 | none | 11.0 | 4 = 2·2 | yes | 12 | 3 | 155 / 2003 | 1 / 1 | 0 (0.000) / 0 (0.000) | 0.006 | — / — | — | 12.92 | 0.43 / — | yes |
| `icv1-f2m13-t181-515ee569` | 13 | 12 | none | 11.0 | 4 = 2·2 | yes | 12 | 4 | 155 / 2003 | 1 / 1 | 0 (0.000) / 0 (0.000) | 0.006 | — / — | — | 12.92 | 0.46 / — | yes |
| `icv1-f2m15-tm275-2d22ff5d` | 15 | 4 | GF(2^3), GF(2^5) | 9.6 | 44 = 2·2·11 | no | 6 | 1 | 2 / 16 | 1 / 6 | 14 (1.000) / 0 (0.000) | 0.500 | 2 / 14 | 7.00 | 8.00 | 1.00 / — | yes |
| `icv1-f2m15-tm275-2d22ff5d` | 15 | 4 | GF(2^3), GF(2^5) | 9.6 | 44 = 2·2·11 | no | 6 | 2 | 2 / 16 | 1 / 6 | 15 (1.000) / 0 (0.000) | 0.500 | 2 / 15 | 7.50 | 8.00 | 1.00 / — | yes |
| `icv1-f2m15-tm275-2d22ff5d` | 15 | 4 | GF(2^3), GF(2^5) | 9.6 | 44 = 2·2·11 | no | 6 | 3 | 2 / 16 | 1 / 6 | 16 (1.000) / 0 (0.000) | 0.500 | 2 / 16 | 8.00 | 8.00 | 1.00 / 1.00 | yes |
| `icv1-f2m15-tm275-2d22ff5d` | 15 | 4 | GF(2^3), GF(2^5) | 9.6 | 44 = 2·2·11 | no | 6 | 4 | 2 / 16 | 1 / 6 | 14 (1.000) / 0 (0.000) | 0.500 | 2 / 14 | 7.00 | 8.00 | 1.00 / — | yes |
| `icv1-f2m23-t5197-69e76b73` | 23 | 11 | none | 21.0 | 4 = 2·2 | yes | 11 | 1 | 45 / 1013 | 1 / 1 | 124 (0.037) / 2 (0.001) | 0.022 | 56 / 3397 | 60.66 | 22.51 | 0.91 / 0.84 | yes |
| `icv1-f2m23-t5197-69e76b73` | 23 | 11 | none | 21.0 | 4 = 2·2 | yes | 11 | 2 | 45 / 1013 | 1 / 1 | 144 (0.040) / 6 (0.002) | 0.022 | 97 / 3566 | 36.76 | 22.51 | 0.80 / 0.84 | yes |
| `icv1-f2m23-t5197-69e76b73` | 23 | 11 | none | 21.0 | 4 = 2·2 | yes | 11 | 3 | 45 / 1013 | 1 / 1 | 145 (0.043) / 4 (0.001) | 0.022 | 182 / 3364 | 18.48 | 22.51 | 0.84 / 0.83 | yes |
| `icv1-f2m23-t5197-69e76b73` | 23 | 11 | none | 21.0 | 4 = 2·2 | yes | 11 | 4 | 45 / 1013 | 1 / 1 | 132 (0.039) / 3 (0.001) | 0.022 | 111 / 3408 | 30.70 | 22.51 | 0.82 / 0.83 | yes |
| `icv1-f2m31-tm90707-c95f16f5` | 31 | 5 | none | 20.5 | 1492 = 2·2·373 | no | 10 | 1 | 20 / 590 | 12 / 342 | 1392 (0.729) / 15 (0.008) | 0.050 | 68 / 1910 | 28.09 | 29.50 | 0.67 / 0.57 | yes |
| `icv1-f2m31-tm90707-c95f16f5` | 31 | 5 | none | 20.5 | 1492 = 2·2·373 | no | 10 | 2 | 20 / 590 | 12 / 342 | 1406 (0.730) / 15 (0.008) | 0.050 | 23 / 1925 | 83.70 | 29.50 | 0.89 / 0.64 | yes |
| `icv1-f2m31-tm90707-c95f16f5` | 31 | 5 | none | 20.5 | 1492 = 2·2·373 | no | 10 | 3 | 20 / 590 | 12 / 342 | 1490 (0.737) / 17 (0.008) | 0.050 | 32 / 2023 | 63.22 | 29.50 | 0.89 / 0.56 | yes |
| `icv1-f2m31-tm90707-c95f16f5` | 31 | 5 | none | 20.5 | 1492 = 2·2·373 | no | 10 | 4 | 20 / 590 | 12 / 342 | 1354 (0.739) / 14 (0.008) | 0.050 | 51 / 1833 | 35.94 | 29.50 | 0.44 / 0.55 | yes |

**Summary by degree**

| curve | m | h | h = #E_0(F_2) | instances | D fold / control | single-column fraction, fold, mean (min–max) | against chance | full-rank ratio mean (min–max) | column ratio | all correct |
|:--|--:|:--|:--|--:|:--|--:|--:|--:|--:|:--|
| `icv1-f2m13-t181-515ee569` | 13 | 4 = 2·2 | yes | 4 | 1 / 1 | 0.000 (0.000–0.000) | 0.0× | — (—–—) | 12.92 | yes |
| `icv1-f2m15-tm275-2d22ff5d` | 15 | 44 = 2·2·11 | no | 4 | 1 / 6 | 1.000 (1.000–1.000) | 2.0× | 7.38 (7.00–8.00) | 8.00 | yes |
| `icv1-f2m23-t5197-69e76b73` | 23 | 4 = 2·2 | yes | 4 | 1 / 1 | 0.040 (0.037–0.043) | 1.8× | 36.65 (18.48–60.66) | 22.51 | yes |
| `icv1-f2m31-tm90707-c95f16f5` | 31 | 1492 = 2·2·373 | no | 4 | 12 / 342 | 0.734 (0.729–0.739) | 14.7× | 52.74 (28.09–83.70) | 29.50 | yes |

Reading.  **At the challenge's shape the prediction holds.**  On `icv1-f2m23-t5197-69e76b73`
`D = 1` on both arms on every seed, `4.0 %` of folded rows are single-column (`1.8×` the `1/45`
chance of two summands landing in one column, against `100 %` on the subfield line), both arms
reach the same rank fraction after `columns` relations (`0.80`–`0.91` / `0.83`–`0.84`), and the
fold's relation ratio, `18.5`–`60.7` (mean `36.7`), sits where the column ratio `22.5` and the
coupon factor put it.  The falsification target, `D > 2` on a `4·prime` subgroup, is not met: a
Frobenius-fixed cofactor of order `4` leaves two-summand relations unconfined, and the fold on
`E_0` buys relations as well as columns.  E2's `E_1` rows (`h = 2` at `m = 17, 23`) read the same
(`D = 1`).

**At `m = 31` it does not, and the cause is the cofactor, not the field.**
`icv1-f2m31-tm90707-c95f16f5` with `r = 1439393` has `h = 4·373`: `D = 12` / `342`, and `73 %` of
folded rows are single-column (`14.7×` chance) — the §6.5 confinement, from a cofactor that
`τ` does **not** fix (`E_0(F_2)` has order `4`; the `373`-part is not `F_2`-rational).  What the
two cases share with the subfield line is a cofactor that is large against the base (`1492`
against `1180` points here, `#E(F_p) ≈ p` against `≈ p` points there), so that a point's
cofactor component has few partners in the base; that reading is not derived here.  The fold
still reaches full rank at `m = 31` (`23`–`68` relations against `1833`–`2023`), because the
single-column rows it collects are not useless, but the relation structure is not the
challenge's.  E2's `m = 31` row was the same curve and subgroup and shows the same `D`.

**Consequence for the ECC2K-130 program.**  AGENTS.md §8b names `m = 31` the primary exploratory
size and asks that its different Frobenius-module structure be disclosed; E15 adds a second
disclosure.  `E_0` over `GF(2^{31})` cannot have the challenge's `4·prime` order, and any
two-summand relation statistic measured there is shaped by the `373` cofactor (here `73 %`
single-column rows against `4 %` at the faithful `m = 23`).  For two-summand relation structure
the faithful sizes this driver can carry stop at `m = 23`; `41`, `83` (the §8a gate) and `131` are
the faithful sizes above it.  Nothing here is an improvement claim, so the §8a `m = 83` gate is
neither required nor discharged.

**Class: accounting.**  No algorithm changed; E15 measures where E2's and §6.5's relation
structure applies, and finds that it follows the cofactor.

### 8.9 E14 — Q-curves of degree 2 and 3 over `F_{p²}`

**Runner:** `--exp e14 --bits 8,9,10,11,12 --seeds 4`.  **Data:**
`experiments/23_glv_invariant_e14.{json,log}` (2026-10-02).  **Module:**
`src/cryptanalysis/q_curve.rs`.

**The object.**  A degree-`d` Q-curve is an `E/F_{p²}` with a `d`-isogeny `φ: E → E^σ` to its
Frobenius conjugate; `ψ = π ∘ ι ∘ φ` is then an endomorphism of `E` over `F_{p²}` of degree
`dp`, with `ψ² = [±d]` on `E(F_{p²})` (Smith's construction of fast GLV curves from Q-curves).
It is the `F_{p²}` analogue of E4's degree-2 and degree-3 CM maps, and the second half of E4b.
The module finds the Q-curve `j`-invariants as the `j ∈ F_{p²} ∖ F_p` with
`Φ_d(j, j^p) = 0` (the repository's tabulated `Φ₂`, `Φ₃`; `Φ_d(j, j^p)` lies in `F_p`, one
equation in the two coordinates of `j`), counts `E_j` and its quadratic twist, finds the
kernel among the `F_{p²}`-roots of the `d`-division polynomial and **checks** it by Vélu's
codomain (`j(φ(E)) = j^p` and `μ⁴a' = a^σ`, independent of `Φ_d`), and reads `ψ`'s eigenvalue
off `ψ(G)` by baby-step giant-step.  Every map is then verified on random points of `⟨G⟩`
(eigenvalue and additivity, as E4's were) and `ψ²(G) = [±d]G` is checked in the unit tests.
The base is the `F_p`-line — every point with `x ∈ F_p`, folded by negation — the set a GLS
map (`d = 1`) would keep whole if it were its line; the prediction (§8.1) is the chance overlap,
`|F| / #E` of the base.

**Every instance**

| d | p | log2 r | h | twist | seed | map | λ² ≡ | ord_r(λ) | images in base | base points | chance fraction | expected at chance | verified |
|--:|--:|--:|--:|:--|--:|:--|:--|--:|--:|--:|--:|--:|:--|
| 2 | 131 | 12.1 | 4 | yes | 1 | `q-curve-psi[d=2,lambda=3570]` | −d | 2148 | 2 | 118 | 0.00687 | 0.81 | True |
| 2 | 397 | 13.5 | 14 | yes | 1 | `q-curve-psi[d=2,lambda=6707]` | +d | 11310 | 0 | 400 | 0.00253 | 1.01 | True |
| 2 | 197 | 14.2 | 2 | no | 2 | `q-curve-psi[d=2,lambda=7459]` | +d | 9716 | 0 | 206 | 0.00530 | 1.09 | True |
| 2 | 281 | 14.3 | 4 | yes | 4 | `q-curve-psi[d=2,lambda=7425]` | +d | 6584 | 0 | 284 | 0.00359 | 1.02 | True |
| 2 | 199 | 14.3 | 2 | no | 3 | `q-curve-psi[d=2,lambda=7032]` | −d | 9945 | 2 | 186 | 0.00468 | 0.87 | True |
| 2 | 359 | 14.4 | 6 | no | 3 | `q-curve-psi[d=2,lambda=1913]` | −d | 4280 | 2 | 360 | 0.00280 | 1.01 | True |
| 2 | 229 | 14.7 | 2 | yes | 4 | `q-curve-psi[d=2,lambda=6142]` | +d | 8720 | 0 | 212 | 0.00405 | 0.86 | True |
| 2 | 691 | 14.9 | 16 | no | 3 | `q-curve-psi[d=2,lambda=17962]` | +d | 29878 | 0 | 674 | 0.00141 | 0.95 | True |
| 2 | 307 | 15.5 | 2 | yes | 2 | `q-curve-psi[d=2,lambda=44388]` | +d | 23571 | 0 | 294 | 0.00312 | 0.92 | True |
| 2 | 659 | 16.1 | 6 | yes | 1 | `q-curve-psi[d=2,lambda=71503]` | −d | 72160 | 0 | 600 | 0.00139 | 0.83 | True |
| 2 | 617 | 16.5 | 4 | no | 4 | `q-curve-psi[d=2,lambda=88319]` | +d | 95088 | 2 | 604 | 0.00159 | 0.96 | True |
| 2 | 661 | 17.7 | 2 | no | 2 | `q-curve-psi[d=2,lambda=186822]` | −d | 108924 | 0 | 670 | 0.00154 | 1.03 | True |
| 2 | 1259 | 18.0 | 6 | no | 1 | `q-curve-psi[d=2,lambda=262503]` | −d | 263760 | 2 | 1238 | 0.00078 | 0.97 | True |
| 2 | 1409 | 18.3 | 6 | no | 3 | `q-curve-psi[d=2,lambda=283022]` | −d | 36714 | 0 | 1390 | 0.00070 | 0.97 | True |
| 2 | 1381 | 18.9 | 4 | yes | 2 | `q-curve-psi[d=2,lambda=226919]` | +d | 238299 | 2 | 1356 | 0.00071 | 0.96 | True |
| 2 | 2243 | 20.3 | 4 | yes | 1 | `q-curve-psi[d=2,lambda=461391]` | −d | 314610 | 2 | 2238 | 0.00044 | 1.00 | True |
| 2 | 1621 | 20.3 | 2 | no | 4 | `q-curve-psi[d=2,lambda=679215]` | −d | 437680 | 0 | 1620 | 0.00062 | 1.00 | True |
| 2 | 2861 | 21.0 | 4 | yes | 2 | `q-curve-psi[d=2,lambda=657756]` | +d | 2046192 | 2 | 2844 | 0.00035 | 0.99 | True |
| 2 | 2269 | 21.3 | 2 | no | 3 | `q-curve-psi[d=2,lambda=2338418]` | −d | 2572032 | 0 | 2238 | 0.00044 | 0.97 | True |
| 2 | 3169 | 22.3 | 2 | yes | 4 | `q-curve-psi[d=2,lambda=3727207]` | +d | 78492 | 0 | 3132 | 0.00031 | 0.98 | True |
| 3 | 197 | 13.2 | 4 | no | 1 | `q-curve-psi[d=3,lambda=3243]` | −d | 9630 | 2 | 200 | 0.00519 | 1.04 | True |
| 3 | 337 | 13.8 | 8 | yes | 3 | `q-curve-psi[d=3,lambda=1592]` | +d | 7079 | 0 | 340 | 0.00300 | 1.02 | True |
| 3 | 241 | 13.8 | 4 | no | 2 | `q-curve-psi[d=3,lambda=3618]` | +d | 7296 | 0 | 252 | 0.00432 | 1.09 | True |
| 3 | 239 | 14.2 | 3 | yes | 3 | `q-curve-psi[d=3,lambda=12163]` | +d | 19078 | 0 | 238 | 0.00416 | 0.99 | True |
| 3 | 151 | 14.5 | 1 | no | 4 | `q-curve-psi[d=3,lambda=10323]` | +d | 22740 | 0 | 144 | 0.00633 | 0.91 | True |
| 3 | 461 | 16.1 | 3 | no | 2 | `q-curve-psi[d=3,lambda=70685]` | +d | 35573 | 2 | 460 | 0.00216 | 0.99 | True |
| 3 | 571 | 16.3 | 4 | no | 2 | `q-curve-psi[d=3,lambda=36214]` | +d | 81552 | 0 | 530 | 0.00162 | 0.86 | True |
| 3 | 641 | 17.1 | 3 | no | 4 | `q-curve-psi[d=3,lambda=39148]` | +d | 68669 | 2 | 664 | 0.00161 | 1.07 | True |
| 3 | 647 | 17.1 | 3 | no | 1 | `q-curve-psi[d=3,lambda=139319]` | +d | 139966 | 4 | 652 | 0.00155 | 1.01 | True |
| 3 | 397 | 17.3 | 1 | yes | 1 | `q-curve-psi[d=3,lambda=87823]` | +d | 79080 | 0 | 400 | 0.00253 | 1.01 | True |
| 3 | 439 | 17.6 | 1 | no | 4 | `q-curve-psi[d=3,lambda=128902]` | +d | 193572 | 0 | 442 | 0.00228 | 1.01 | True |
| 3 | 809 | 17.7 | 3 | yes | 3 | `q-curve-psi[d=3,lambda=178911]` | +d | 109289 | 2 | 758 | 0.00116 | 0.88 | True |
| 3 | 1709 | 17.9 | 12 | no | 1 | `q-curve-psi[d=3,lambda=191366]` | +d | 1817 | 4 | 1692 | 0.00058 | 0.98 | True |
| 3 | 1759 | 18.4 | 9 | no | 4 | `q-curve-psi[d=3,lambda=196394]` | −d | 171771 | 0 | 1786 | 0.00058 | 1.03 | True |
| 3 | 2663 | 18.8 | 16 | no | 4 | `q-curve-psi[d=3,lambda=349934]` | −d | 12310 | 0 | 2734 | 0.00039 | 1.05 | True |
| 3 | 1619 | 19.3 | 4 | yes | 2 | `q-curve-psi[d=3,lambda=36459]` | −d | 109242 | 0 | 1670 | 0.00064 | 1.06 | True |
| 3 | 2273 | 20.3 | 4 | yes | 3 | `q-curve-psi[d=3,lambda=615206]` | −d | 1291818 | 0 | 2332 | 0.00045 | 1.05 | True |
| 3 | 3457 | 20.5 | 8 | no | 2 | `q-curve-psi[d=3,lambda=717085]` | +d | 746891 | 0 | 3436 | 0.00029 | 0.99 | True |
| 3 | 1597 | 21.3 | 1 | no | 3 | `q-curve-psi[d=3,lambda=1598]` | +d | 6080 | 0 | 1632 | 0.00064 | 1.04 | True |
| 3 | 2269 | 22.3 | 1 | no | 1 | `q-curve-psi[d=3,lambda=2480057]` | +d | 37324 | 0 | 2284 | 0.00044 | 1.01 | True |

**Summary**

| d | instances | maps | log2 r | λ² ≡ +d / −d | ord_r(λ) min–max | ord_r(λ) / r, min | images in base, total | expected at chance, total | base points, total | all verified |
|--:|--:|--:|:--|:--|--:|--:|--:|--:|--:|:--|
| 2 | 20 | 20 | 12.1–22.3 | 10 / 10 | 2148–2572032 | 0.016 | 16 | 19.2 | 20664 | yes |
| 3 | 20 | 20 | 13.2–22.3 | 15 / 5 | 1817–1291818 | 0.002 | 16 | 20.1 | 22646 | yes |

Reading.  **The prediction holds: a Q-curve endomorphism keeps no base.**  Over `40`
instances (`20` per degree, `r = 2^{12}`–`2^{22}`) every `ψ` verifies, `λ² ≡ ±d` on every one
(`+d` on `10` / `−d` on `10` at `d = 2`, `15` / `5` at `d = 3`), and the eigenvalue orders run
from `1817` to `2.6·10⁶` — at least `0.2 %` of `r`, against `2` and `4` for the automorphisms
and `n` for a Frobenius — so there is no small orbit to fold by.  The images of the
`43310` base points land back in the base `32` times, against `39.3` expected at chance
(`16` against `19.2` at `d = 2`, `16` against `20.1` at `d = 3`).  The search found `174`
Q-curve `j`-invariants and counted `324` curves to get the `40` with a cofactor at most `16`;
every kernel that reached `E^σ`'s `j` did so over `F_{p²}` (`0` needed `F_{p⁴}` for `ι`), and
`41` roots of the division polynomial led to other `d`-isogenous neighbours and were set aside.
This is E4's type-C reading on the `F_{p²}` group: a degree-`d` isogeny's `x`-map is a rational
function of degree `d`, so no line or small set is stable, and a fast endomorphism is a scalar
multiplication shortcut (GLV), not a factor-base symmetry.

**Class: accounting.**  The lever list of §8.1 is closed on the base side: on these families
the only folds are the automorphisms, the Frobenius-type maps (`π`, `τ`, GLS `ψ`) and, with a
rational `2`-torsion point, the translation `τ_T` (E13).

### 8.10 E16 — the boundary on subfield curves, and the best arm's phases to `p = 2^{16}`

**Runner:** E13's driver, unchanged: `--exp e13 --bits 14 --seeds 3`, `--bits 15 --seeds 3`,
`--bits 16 --seeds 2`.  **Data:** `experiments/23_glv_invariant_e13_p{14,15,16}.{json,log}`
(2026-10-08), beside `…_e13.json` and `…_e13_13.json`.  **Tables:** the tables binary's E16
printer, which reads every E13 row.

**The boundary, derived (it should have been stated in §8.1).**  On a subfield curve
`E/F_p`, `E(F_p) ⊂ E(F_{p³})` and `#E(F_{p³}) = #E(F_p) · #T` with `T` the trace-zero part,
`#T ≈ p²`.  A prime `r > p + 1 + 2√p` divides `#T`, so the cofactor `h` contains all of
`E(F_p)`: **`h ≥ #E(F_p) ≈ p` and `r ≈ p²/k`**, `k = h/#E(F_p)` (bounded by `8` in every driver
here).  The frozen rows agree: `log₂ r − 2·log₂ #E(F_p)` lies in `[−3.14, +0.55]` over all `66`
E8, E11 and E13 rows and the `8` new ones.  Two consequences follow.

- **Rho costs about `p`.**  The matched folded walk takes `√(π r / 2A)` steps, `∝ p / √k`.
- **A line base cannot beat that exponent.**  The Frobenius line carries `≈ 2p` points; a base
  folded `w` a column has `≈ 2p/w` columns and needs at least that many relations (`+1 − D`),
  each costing at least one decomposition attempt at a hit rate `η` independent of `p`.  So the
  relation phase alone is `≥ (2p / wη) · c_attempt`: **`Θ(p)` at best, the same exponent as
  rho**, and the ratio to rho is bounded below by a constant, not by a falling function of `p`.
  The linear algebra adds `(2p/w)² · ρ` row operations, `ρ` the row weight: `Θ(p²)`, which
  eventually makes the ratio grow as `p`.

So on these groups the most a fold, a symmetrised system or a cheaper solve can buy is a
**constant** against rho — the fold divides the columns by `w` and the linear algebra by `w²`,
the system symmetry divides the solve — never an exponent.  E8–E13 were constant-factor work on
a boundary that is itself rho's exponent; the claims of §8.5 and §8.7 that read otherwise
assumed `r ≈ p³` or a fixed cofactor and are corrected there (struck through, not deleted).
This is the opposite of the full-group setting Gaudry and Diem analyse for `E(F_{q^n})` of
prime order `≈ q^n`, where rho costs `q^{n/2}`; a subfield curve's large prime lives in a group
of order `q^{n−1}`.

**The prediction E16 tests.**  On the best arm (fold `12` + `D₃`), every relation-phase cost grows
at least as rho does (exponent `≥ 1/2` against `r`), the linear algebra grows as `p²` (`r¹`), and
the ratio `S / rho S` does not fall with size.  A fitted ratio exponent below `−0.05` over `p =
2^7`–`2^{16}` would falsify it.

**The best arm, every instance at full rank (fold 12 + D₃; F_p multiplications)**

| p bits | #E(F_p) | log2 r | log2 r − 2·log2 #E(F_p) | cols | full-rank rel | solver | group arithmetic | linear algebra | total | LA share | S | rho S folded | S / rho S |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 8 | 260 | 12.9 | -3.14 | 19 | 23 | 6.11e+06 | 1.5e+06 | 911 | 8.42e+06 | 0.0001 | 96092 | 163.8 | 587 |
| 7 | 112 | 13.2 | -0.37 | 9 | 12 | 3.15e+06 | 8.05e+05 | 298 | 4.35e+06 | 0.0001 | 44098 | 134.9 | 327 |
| 7 | 112 | 14.2 | +0.55 | 10 | 13 | 9.99e+06 | 2.63e+06 | 406 | 1.31e+07 | 0.0000 | 96460 | 130.0 | 742 |
| 8 | 184 | 15.4 | +0.34 | 19 | 36 | 6.44e+06 | 1.81e+06 | 840 | 8.94e+06 | 0.0001 | 43161 | 158.4 | 272 |
| 8 | 244 | 15.5 | -0.39 | 21 | 26 | 9.7e+06 | 2.59e+06 | 905 | 1.3e+07 | 0.0001 | 61216 | 135.6 | 451 |
| 9 | 404 | 17.4 | +0.05 | 39 | 45 | 1.11e+07 | 3.2e+06 | 2.53e+03 | 1.56e+07 | 0.0002 | 37871 | 141.9 | 267 |
| 9 | 448 | 17.8 | +0.14 | 37 | 51 | 2.29e+07 | 6.65e+06 | 1.9e+03 | 3.08e+07 | 0.0001 | 65373 | 103.5 | 632 |
| 9 | 464 | 17.9 | +0.21 | 42 | 59 | 2.3e+07 | 6.65e+06 | 2.55e+03 | 3.1e+07 | 0.0001 | 62216 | 105.7 | 588 |
| 11 | 1552 | 18.3 | -2.87 | 116 | 234 | 1.3e+08 | 3.36e+07 | 5.69e+03 | 1.68e+08 | 0.0000 | 293056 | 91.0 | 3220 |
| 10 | 776 | 19.1 | -0.11 | 66 | 187 | 7.67e+07 | 2.18e+07 | 5.02e+03 | 1.01e+08 | 0.0000 | 134659 | 88.8 | 1517 |
| 10 | 700 | 19.1 | +0.20 | 58 | 87 | 4.49e+07 | 1.32e+07 | 4.32e+03 | 6.02e+07 | 0.0001 | 80294 | 81.5 | 985 |
| 10 | 688 | 19.2 | +0.31 | 59 | 73 | 3.93e+07 | 1.19e+07 | 4.06e+03 | 5.32e+07 | 0.0001 | 69454 | 123.5 | 562 |
| 10 | 892 | 19.4 | -0.19 | 72 | 131 | 6.18e+07 | 1.89e+07 | 4.72e+03 | 8.34e+07 | 0.0001 | 99984 | 103.0 | 971 |
| 12 | 2480 | 19.9 | -2.68 | 213 | 651 | 3.27e+08 | 9e+07 | 3.67e+04 | 4.26e+08 | 0.0001 | 434231 | 75.6 | 5741 |
| 11 | 1304 | 20.6 | -0.08 | 110 | 330 | 1.47e+08 | 4.34e+07 | 7.84e+03 | 1.94e+08 | 0.0000 | 153080 | 87.2 | 1756 |
| 11 | 1288 | 20.6 | -0.03 | 104 | 208 | 1.1e+08 | 3.26e+07 | 1.16e+04 | 1.46e+08 | 0.0001 | 114411 | 85.5 | 1338 |
| 11 | 1760 | 21.4 | -0.15 | 135 | 306 | 1.54e+08 | 5.09e+07 | 1.48e+04 | 2.1e+08 | 0.0001 | 125914 | 64.1 | 1964 |
| 12 | 2528 | 22.7 | +0.05 | 211 | 367 | 1.91e+08 | 5.98e+07 | 2.39e+04 | 2.59e+08 | 0.0001 | 100637 | 85.9 | 1172 |
| 13 | 7972 | 23.1 | -2.86 | 639 | 1560 | 8.23e+08 | 2.66e+08 | 4.6e+05 | 1.12e+09 | 0.0004 | 379369 | 79.6 | 4764 |
| 14 | 9212 | 23.6 | -2.78 | 769 | 2741 | 1.31e+09 | 4.36e+08 | 7.12e+05 | 1.78e+09 | 0.0004 | 506680 | 76.6 | 6616 |
| 12 | 3472 | 23.6 | +0.09 | 302 | 542 | 2.52e+08 | 8.4e+07 | 7.44e+04 | 3.48e+08 | 0.0002 | 97273 | 90.4 | 1076 |
| 12 | 3760 | 23.7 | -0.03 | 318 | 844 | 3.75e+08 | 1.29e+08 | 9.94e+04 | 5.16e+08 | 0.0002 | 138787 | 77.1 | 1801 |
| 13 | 5260 | 24.7 | -0.02 | 424 | 834 | 4.54e+08 | 1.62e+08 | 1.23e+05 | 6.33e+08 | 0.0002 | 121355 | 77.5 | 1565 |
| 15 | 17504 | 25.4 | -2.76 | 1496 | 3498 | 1.7e+09 | 5.95e+08 | 3.99e+06 | 2.38e+09 | 0.0017 | 353096 | 76.2 | 4631 |
| 15 | 24136 | 26.3 | -2.83 | 1967 | 4597 | 2.75e+09 | 9.01e+08 | 1.16e+07 | 3.77e+09 | 0.0031 | 416284 | 109.9 | 3788 |
| 14 | 14780 | 27.7 | -0.04 | 1207 | 2598 | 1.38e+09 | 5.13e+08 | 2.71e+06 | 1.96e+09 | 0.0014 | 134245 | 80.6 | 1665 |
| 14 | 15256 | 27.8 | +0.00 | 1240 | 4291 | 2.49e+09 | 9.1e+08 | 2.78e+06 | 3.47e+09 | 0.0008 | 226831 | 75.4 | 3010 |
| 15 | 16964 | 28.0 | -0.06 | 1459 | 3671 | 1.62e+09 | 6.35e+08 | 5.32e+06 | 2.32e+09 | 0.0023 | 139720 | 78.4 | 1781 |
| 16 | 36776 | 30.3 | +0.00 | 3073 | 8542 | 4.91e+09 | 1.82e+09 | 4.83e+07 | 6.94e+09 | 0.0070 | 188637 | 90.5 | 2084 |
| 16 | 37580 | 30.4 | +0.01 | 3096 | 7468 | 4.04e+09 | 1.7e+09 | 4.58e+07 | 5.94e+09 | 0.0077 | 157733 | 55.6 | 2835 |

**Fitted exponents (least squares, log–log)**

| quantity | against #E(F_p) ≈ p | against r | rows |
|:--|--:|--:|--:|
| group arithmetic | 1.29 | 0.65 | 30 |
| linear algebra | 2.01 | 1.01 | 30 |
| rho folded (total) | 0.83 | 0.44 | 30 |
| solver | 1.22 | 0.61 | 30 |
| total | 1.23 | 0.62 | 30 |
| S / rho S | — | 0.18 | 30 |

Reading.  **The prediction holds, and the gap widens.**  Over `30` instances from `p = 2^7` to
`2^{16}` (`r = 2^{12.9}`–`2^{30.4}`), the best arm's total grows as `r^{0.62}` (`p^{1.23}`): the
solver `r^{0.61}`, the stream's group arithmetic `r^{0.65}`, and the **linear algebra `r^{1.01}`
(`p^{2.01}`)**, the `p²` the derivation names, still `< 1 %` of the total at `p = 2^{16}` but
closing on the solver as `p^{0.8}`.  The matched folded rho fits `r^{0.44}`: below its asymptotic
`1/2` because per-walk set-up and verification are still a visible share at these sizes, the
same effect E8 and E11 saw (`0.37`–`0.42`).  The ratio `S / rho S` grows as **`r^{0.18}`**, from
`267×` at `r = 2^{17}` to `2084`–`2835×` at `r = 2^{30}`; the rows with `k ≈ 7`
(`log₂ r − 2·log₂ #E(F_p) ≈ −2.8`) sit at `3220`–`6616×`, as the `√k` in rho's cost predicts.  No
fitted exponent of the relation phase is below rho's, so there is no crossover to extrapolate:
on the subfield line, index calculus with a line base is a constant factor above rho at best and
in practice `10³`–`10⁴` above it, rising.  **Extrapolation, not measurement:** on the fitted
exponents the linear algebra overtakes the solver near `p ≈ 2^{24}`, after which the ratio grows
as `r^{1/2}`.

**Class: accounting.**  E16 changes no algorithm.  It derives the boundary §8.1 should have stated
for the subfield families, corrects two exponent claims that rested on `r ≈ p³` and on a fixed
cofactor, and measures the best arm's phases to `p = 2^{16}` against it.

### 8.4 Verdict after E8–E16

| lever | measured | class |
|:--|:--|:--|
| three summands on the `π`-line (E8) | columns `÷ 3`, relations `÷ 4.2`, `0` single-column rows; `S ÷ 3.8` end to end in one unit | engineering |
| the matched folded rho on `F_{p²}`, `F_{p³}` (E9) | steps `÷ 1.40, 1.74, 2.16` at `A = 4, 6, 12` (expected `1.41, 1.73, 2.45`), verified; in `F_p` multiplications the folded walk on `E(F_{p³})` pays `110` a step against `88` (E8) | accounting (reference) |
| the algebraic `S₄` line oracle (E11) | `9.42e+05` multiplications a target, flat in `p`; same relation stream as E8; exponent of the total `0.53` (fold) against the pair table's `0.88`; cheaper than the pair table from `p ≈ 2^12` | engineering |
| exponents (E10) | fold `0.88`, control `0.86`, rho `0.37` / `0.39` over `18` instances; no arm's exponent below rho's `1/2` | accounting |
| the pair table over orbit representatives (E12) | table `÷ 2.95` / `2.97` / `1.98` at `w = 6, 6, 4`, build `÷ 2.59` / `2.77` / `1.91`; probe `1.11×` (line) and `1.00×` (`F_p`), under the `1.2×` falsification line; `S ÷ 1.18` on `j = 0`, `÷ 1.03` on `j = 1728`, `× 1.02` on the line; `0` collisions in `9.77e+07` probes | engineering (memory); relabelling if quoted as a cost gain on the line |
| FGHR's `2`-torsion symmetry with the fold (E13) | system: `R(q₃)` of degree `16 = 64/4`, `1.25e+04`–`2.04e+04` multiplications a call (`÷ 33`–`41` against `S₃`), `S ÷ 27.0`; base: `τ_T` halves the columns, `S ÷ 2.52`; combined / product `≥ 0.82` (survives); `267`–`5741×` rho, total `p^{1.23}` | engineering |
| the ECC2K-130 family, two summands (E15) | `E_0` at `4·prime` (`m = 23`): `D = 1` / `1`, `4 %` single-column rows, the fold buys relations; at `m = 31` (`h = 4·373`): `D = 12` / `342`, `73 %` single-column rows — the §6.5 confinement follows the cofactor, and `m = 31` cannot carry the challenge's shape | accounting |
| Q-curves of degree 2 and 3 over `F_{p²}` (E14) | `40` instances, every `ψ = π ∘ ι ∘ φ` verified, `λ² ≡ ±d`, `ord_r(λ) = 1817`–`2.6·10⁶`; `32` images of `43310` base points in the base against `39.3` at chance | accounting |
| the boundary on subfield curves (E16) | `r ≈ p²/k` (`h ⊇ E(F_p)`), so rho costs `Θ(p)` and a line base needs `Θ(p)` relations: at best a constant; the best arm to `p = 2^{16}`: total `r^{0.62}`, linear algebra `r^{1.01}`, rho `r^{0.44}`, `S / rho S ∝ r^{0.18}` (`267`–`6616×`) | accounting (corrects E11's and E13's exponent readings) |

With E13 run, the cheapest arm in this note is the `Y`-line of a subfield curve with rational
`2`-torsion, folded `12` a column by `⟨−1, π, τ_T⟩` and decomposed by the `D₃`-symmetrised
conic-resultant oracle: `267×` rho at best, every phase priced, the solver three quarters of
the cost.  Both uses of the `2`-torsion are now taken, on the base and on the system, and they
compound with the Frobenius fold; the relation phase is still `p^{1+o(1)}` solves, so what
remains is the constant of a solve (a degree-`16` univariate root-finding and three conics) and
not a lever on the base.  E12 settles the pair-table regime: the table folds by `w/2`, and that
is worth having only where the table is the cost (`F_p`, two summands).  E15 settles the transfer question for the base: on the challenge's `4·prime` shape the Frobenius-fixed cofactor does not confine two-summand relations, but on `m = 31`, the program's primary exploratory size, the `373` cofactor does.  E14 closes the base side: a Q-curve endomorphism of degree 2 or 3 keeps base points only at chance, as E4's CM maps did, so the folds on these families are the automorphisms, the Frobenius-type maps and `τ_T`, and every experiment of §8.1 has run.  E16 then states the boundary that governs all of them: on a subfield curve the large prime lives in the trace-zero part, `r ≈ p²`, so rho costs `Θ(p)` and no line base can do better than a constant against it; the best arm is `10³`–`10⁴×` rho and the ratio grows as `r^{0.18}`.

## 9. Goal G2 — the cheapest oracle, moved to where the exponent can beat rho

**Why this goal.**  §8.10 closes the subfield families: their large prime lives in a group of
order `≈ p²`, so rho costs `Θ(p)`, and no line base, fold or solver can do better than a constant
against it.  The setting where index calculus has a **sub-rho exponent** is the full group: a curve
over `F_{p³}` that is not a subfield curve, prime `r ≈ p³/h` with a small cofactor `h`, and
Gaudry's base of abscissae in an `F_p`-line, which needs `≈ p ∝ r^{1/3}` relations against rho's
`r^{1/2}` steps.  `RESEARCH_EXTENSION_FIELD_BOUNDARIES.md` prices that setting in full and names
the single object that keeps it from crossing rho — the cost `C₃` of one `S₄` decomposition
solve, `≈ 0.88·10⁶` `F_p` multiplications with the `S₃`-symmetrised Macaulay solver:

- the plain method (base of all `x ∈ F_p`) is bounded away from rho, its minimum `S* ≈ 265`
  (`≈ 200×` rho) at `N* ≈ 2^{50}` (Theorem 2, extrapolated from measured exponents);
- the double-large-prime variant has the sub-rho exponent `N^{4/9}` but crosses rho only at
  `N_× = 2^{33}·1989^{18} ≈ 2^{230}` (Theorem 3), because `C_DLP` is dominated by `C₃`;
- the `ψ`-folded `j = 0` arm (`RESEARCH_GLV_INDEX_CALCULUS.md`) is `220`–`540×` rho at
  `n = 2^{24}`–`2^{33}`.

E13 (§8.7) cut exactly that solve by `33`–`41×` — `S₄` in a coordinate `Y` with
`Y(P + T) = −Y(P)`, symmetrised by `D₃ = (Z/2)² ⋊ S₃`, solved as three conics over `F_p[q₃]` —
but on a subfield curve, where it could buy only a constant.  **G2 moves that oracle and the
`τ_T` fold to the full group and measures how far the crossover moves.**  `N_×` is the 18th power
of the constant, so a cheaper solve is worth `18` bits of `n` per halving of `C_DLP` — the
leverage in this program that is largest per unit of engineering.

**The instance (`src/cryptanalysis/fghr_full.rs`).**  `x₀ ∈ F_{p³} \ F_p` and `c ∈ F_p^*` at
random, `a = c² − 3x₀²`, `b = −x₀³ − a x₀`: `x₀` is a root of `x³ + ax + b` and
`f'(x₀) = c²`, so `T = (x₀, 0)` is rational and `x(P + T) − x₀ = c²/(x − x₀)`.  `j(E) ∉ F_p` is
checked, so no instance is a twist of a subfield curve; `#E(F_{p³}) = h·r` is counted by
baby-step giant-step in the Hasse interval, with `h ≤ 8`.  The coordinate
`Y = (x − x₀ − c)/(x − x₀ + c)` has `Y(P + T) = −Y(P)`; its inverse is a Möbius map with
coefficients `c ∓ x₀ ∈ F_{p³}` (in `F_p` on E13's curves), so E13's oracles carry over through
one generalised substitution (`fghr_line::s4_in_y_ext`, `fghr_polynomials_on`) and nothing else.
The base `{P : Y(P) ∈ F_p \ {0, ±1}}` is Gaudry's affine line `x ∈ x₀ + F_p` in that coordinate;
`τ_T` keeps it and folds with coefficient `1`, so `⟨−1, τ_T⟩` puts `4` points in a column against
`⟨−1⟩`'s `2`.  The two unit tests already show the `D₃` oracle returning a decomposition on
exactly the targets the `S₃` oracle and the exhaustive pair table do (`p = 2^6`, `118` targets).

**Predictions, fixed before E17 runs.**

| id | question | prediction | falsified by |
|:--|:--|:--|:--|
| G2.1 | do the oracles agree on the full group? | `D₃`, `S₃` and the pair table decompose the same targets, every decomposition re-added in the group | any disagreement at `p ≤ 2^8` |
| G2.2 | the oracle constant | `C₃(D₃)` `÷ ≥ 20` against `C₃(S₃)` on the same instance and targets (E13: `÷ 33`–`41`), flat in `p` up to `log p` | a ratio below `10` at any size |
| G2.3 | the `τ_T` fold | columns `÷ 2.0 ± 0.05`, relations to the pinned logarithm `÷ ≈ 2` | a column ratio outside `[1.9, 2.1]` |
| G2.4 | end to end | `S / rho S ≤ 50` at `r ≥ 2^{28}` on the `D₃` + `τ_T` arm, against `220`–`540` for the `ψ`-fold with `S₃` | `S / rho S > 100` at `r ≥ 2^{28}` |
| G2.5 | the ratio's slope | relations `∝ p`, rho `∝ p^{3/2}`: while the relation phase dominates, `S / rho S` falls, fitted exponent in `r` `≤ −0.10` (E16 on the subfield line: `+0.18`) | a fitted exponent `≥ 0` |

**What follows if they hold.**  G2.2 and G2.4 give the measured constants to put into
`RESEARCH_EXTENSION_FIELD_BOUNDARIES.md` Theorems 2–3 in place of `C₃ ≈ 0.88·10⁶`: the plain
method's minimum `S*` moves by `√` of the constant (it is the geometric mean of the two terms),
and the double-large-prime crossover by `18·log₂` of the share of `C_DLP` the solve is.  Neither is
an exponent; both are the constant the boundary note says is the only lever, measured on the
group where it matters.  **Class if they hold: engineering**, on a variant whose exponent is
already sub-rho.  A falsified G2.2 would mean the `D₃` structure that made E13 cheap is an
artefact of `F_p` Möbius coefficients; a falsified G2.5 would mean the full-group arm, too, has a
cost that grows as fast as rho in the measured range.

### 9.1 Amendment, before E18 is built: the frontier is `k = 4`

*Added 2026-10-09 while E17 was running and before any `k = 4` code of this goal existed; §9's
table above is unchanged and binds E17.*

§9 read its boundary from `RESEARCH_EXTENSION_FIELD_BOUNDARIES.md`, which predates
`RESEARCH_RESIDUAL_WALKS.md` §11.15–§11.19.  Those sections close `k = 3` (best measured: the
plain method at `528×` rho at `2^{33.1}`; double large primes at `1,989×`) and move the frontier
to `k = 4` on `E(F_{p⁴})`, where the base has `|F| ∝ n^{1/4}`, so the linear algebra is
`∝ n^{1/2}`, rho's exponent:

- `r∞ = 0.518 ± 0.031` — the linear algebra over the rho walk, measured over eight curves to
  `n = 2^{40.1}` (§11.19); past the handover a plain `k = 4` method beats rho by `1/r∞` and no
  more;
- the handover `n* ≈ 2^{151}`, set by the `S₅` solve `C₄ = 1.213·10¹²` (§11.17), with "every
  factor `f` taken off `C₄` moves `n*` about `4 log₂ f` bits earlier", and the
  Faugère–Gaudry–Huot–Renault symmetries named there as one of two levers.

So G2's two levers have a second, larger target.  The `D_m` symmetry acts on `C₄` (E19, to be
registered separately), and **a fold of the base acts on `r∞` itself**.  `r∞` is the linear
algebra, `∝ N²` in the unknowns, against rho.  A fold by `w` points a column divides `N` by
`w/2`, while the matched rho gains `√` of whatever automorphisms it can use and nothing from
`τ_T`.  §11.15 lists levers on the linear-algebra constant as engineering, which they are, but
at `k = 4` that constant *is* the asymptotic ratio to rho.

One accounting point must be measured, not assumed.  `rho4` (`gaudry_quartic.rs`) walks
without the negation map (`S = 1.319`, the `√(π/2)` of an unfolded walk), while every other panel
in this program uses the negation-folded walk.  E18 measures both.

**E18 — the folds on the `k = 4` linear algebra (registered here, before the code exists).**
On curves over `F_{p⁴}` (`p ≤ 1033`, `n ≤ 2^{40}`, the sizes of §11.19), one relation stream
from §11.19's meet-in-the-middle oracle feeds a control arm (`⟨−1⟩`, one column per abscissa,
exactly §11.19's matrix) and a folded arm.  Each arm is filtered and solved by §11.7's Wiedemann,
its logarithm verified by `[d]G = Q`, and priced against the rho matched to its group:

- **`ψ` arm:** `j = 0`, `p ≡ 1 (mod 12)`, prime order, `ψ(x, y) = (ωx, y)` with `ω ∈ F_p`, which
  keeps the base `x ∈ F_p`.  Columns are the `⟨ψ⟩`-orbits of abscissae, with coefficients
  `±λ^k`.  The matched rho walks the classes `{±ψ^k P}`.
- **`τ_T` arm:** the §9 construction over `F_{p⁴}`, `#E = h·n` with `h = 2` where the search
  finds one (else `h ≤ 4`, recorded), base `x ∈ x₀ + F_p`, `P` and `P + T` in one column.  The
  matched rho is the negation walk, since rho cannot use `τ_T`.

| id | quantity | prediction | falsified by |
|:--|:--|:--|:--|
| G2.6 | negation-folded rho against `rho4`'s unfolded walk at `k = 4` | steps `÷ √2` (`1.41 ± 0.10`), so §11.19's `r∞` reads `≈ 0.73` against the negation walk | a ratio outside `[1.25, 1.6]` |
| G2.7 | the `ψ` fold | columns `÷ 3.0 ± 0.1`; linear algebra `÷ ≈ 9` (with the filtering fraction recorded); `ψ`-folded rho `÷ √3` against the negation walk; `r_ψ` against the `ψ`-folded rho `≈ 0.518·√6/9 ≈ 0.14`, against `≈ 1.27` for the control on the same curves | `r_ψ > 0.30` at every size, or any unverified logarithm |
| G2.8 | the `τ_T` fold | columns `÷ 2.0 ± 0.1`; linear algebra `÷ ≈ 4`; `r_τ` against the negation rho `≈ 0.518·√2·√h / 4` (`≈ 0.26` at `h = 2`), against `≈ 1.04` for the control | `r_τ > 0.50` at every size |

**What this would and would not say.**  `r` is the ratio of the linear algebra to rho, not
`S / rho S`: the relation phase is still the meet-in-the-middle oracle's here, and §11.17's
`4.3·10⁸×` rho for the `S₅` solve is untouched by E18.  If G2.7 holds, the eventual advantage of
a `k = 4` method over the rho an attacker would actually run on that curve moves from
`1/0.73 ≈ 1.4×` (negation walk) to `≈ 7×` (`ψ`-folded base against `ψ`-folded walk).
**Class: engineering**, the constant of an asymptote.  The handover `n*` also moves, through
`1 − r∞` in §11.16's formula and through the halved relation count, but `C₄` still sets it.

### 9.2 E17 — the `D₃` oracle and the `τ_T` fold on the full group, measured

**Runner:** `E17_S3_BITS=10 cargo run --release --example glv_invariant_experiments -- --exp e17
--bits 6,7,8,9,10,11,12,13 --seeds 3 --rho-runs 8 --json experiments/23_glv_invariant_e17.json`.
**Data:** `experiments/23_glv_invariant_e17.{json,log}` (2026-10-09, `24` rows, `294 s`).
**Tables:** `cargo run --release --example glv_invariant_experiment_tables --
experiments/23_glv_invariant_e17.json` (the E17 printer).  **Source:** `src/cryptanalysis/fghr_full.rs`,
`src/cryptanalysis/fghr_line.rs` (`YLine`, `s4_in_y_ext`, `fghr_polynomials_on`).

`24` non-subfield curves over `F_{p³}`, `p = 37`–`4549`, `r = 2^{13.3}`–`2^{33.5}`, cofactor
`h ∈ {4, 8}` (`T` is rational, so `h` is even, and the search took the first `h ≤ 8`).  One
relation stream fed the `⟨−1, τ_T⟩` base and the `⟨−1⟩` control; the `S₃` Macaulay oracle ran the
same base and targets to `p = 2^{10}`; both oracles were checked against the exhaustive pair table
on `200` targets per curve to `p = 2^8`.  **Every planted logarithm was recovered, every
algebraic decomposition re-added to its target, and all `192` rho walks verified.**

**The solve constant and the fold.**

| p | log₂ r | h | cols `⟨−1, τ_T⟩` / `⟨−1⟩` | D₃ muls per call | S₃ muls per call | S₃ / D₃ |
|--:|--:|--:|:--|--:|--:|--:|
| 43 | 13.3 | 8 | 12 / 24 | 12537 | 539418 | 43.0 |
| 67 | 15.2 | 8 | 17 / 34 | 13043 | 535231 | 41.0 |
| 211 | 20.2 | 8 | 58 / 116 | 15451 | 570788 | 36.9 |
| 337 | 23.2 | 4 | 88 / 176 | 15835 | 578450 | 36.5 |
| 823 | 26.1 | 8 | 216 / 432 | 17566 | 591700 | 33.7 |
| 1723 | 29.3 | 8 | 431 / 862 | 18867 | — | — |
| 4549 | 33.5 | 8 | 1188 / 2376 | 19471 | — | — |

(Seven of the `24` rows; the printer gives all.)  Over the `15` rows with both oracles,
**`S₃ / D₃ = 37.99` (`33.68`–`43.03`)**.  `R(q₃)` has degree `16` on every call of every row.  The
`D₃` cost grows from `1.25·10⁴` to `1.95·10⁴` over `p = 2^{5.2}`–`2^{12.2}`, which is the
`log p` of Cantor–Zassenhaus.  The column ratio is **`2.00` on all `24` rows**.  Agreement with
the pair table: **`0` disagreements in `1800` targets** for both oracles once one
degenerate target is set apart.  In that one (`p = 67`, `k = 67`) the target is itself a base
point, the pair table's "decomposition" is `P + (−P) + R`, and `S₄` has a one-dimensional
component there that no zero-dimensional solver returns.  `S₃` misses it exactly as `D₃` does.

**End to end, against the matched negation rho** (`F_p` multiplications, the `D₃` arm's phases):

| p | log₂ r | h | set-up | group arithmetic | solver | linear algebra | S / rho S, D₃ | S / rho S, S₃ |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 43 | 13.3 | 8 | 1.37e+06 | 8.74e+05 | 3.18e+06 | 251 | 285 | 7241 |
| 211 | 20.2 | 8 | 1.37e+06 | 7.36e+06 | 2.08e+07 | 2.09e+03 | 303 | 7898 |
| 823 | 26.1 | 8 | 1.37e+06 | 9.88e+07 | 2.39e+08 | 2.7e+04 | 455 | 7449 |
| 1723 | 29.3 | 8 | 1.37e+06 | 1.81e+08 | 4.15e+08 | 1.86e+05 | 298 | — |
| 3037 | 32.7 | 4 | 1.37e+06 | 3.63e+08 | 7.94e+08 | 8.99e+05 | 180 | — |
| 4549 | 33.5 | 8 | 1.37e+06 | 6.44e+08 | 1.29e+09 | 2.91e+06 | 236 | — |

Fitted against `r` over the `24` rows: solver `r^{0.44}`, group arithmetic `r^{0.47}`, linear
algebra `r^{0.68}`, total `r^{0.43}`, matched rho `r^{0.44}`, **`S / rho S` `r^{−0.01}`** (`S₃`
arm: `r^{+0.03}`).

**Reading, against §9's predictions.**

- **G2.1 holds**, with the degenerate target counted apart and frozen in the file
  (`pair_table_cancelling`), never dropped.
- **G2.2 holds:** the `D₃` structure that made E13 cheap is not an artefact of `F_p` Möbius
  coefficients.  On the full group it divides the `S₄` solve by `38`, E13's `33`–`41`.
- **G2.3 holds exactly:** `τ_T` halves the columns on every curve.
- **G2.4 is falsified.**  `S / rho S` is `180`–`377` at `r ≥ 2^{28}` against the `≤ 50`
  predicted; the falsifier was `> 100`.  Within the harness the two levers buy `÷ 25`–`40` end to
  end (the `S₃` arm on the same base is `5,961`–`14,502`).
- **G2.5 is falsified.**  The fitted slope is `−0.01`, not `≤ −0.10`.  The cause is three costs of
  this harness that the prediction did not price.  None of them is a property of the oracle:
  1. **The stop rule.**  The rows stop at the folded arm's full rank, so every column must
     appear, and a weight-3 stream reaches that after `≈ (N/3) ln N` relations.  Measured, the
     relations at full rank are `1.1×` the columns at `N ≈ 10` and `1.5`–`2.8×` at `N ≥ 100`.  §11's pipeline filters
     to a square core and stops at the pinned logarithm instead.
  2. **Fresh targets.**  Each target is `aG + bQ`, two scalar multiplications.  That is a third of
     the total (`r^{0.47}`), where a walk-generated residual costs one addition.
  3. **The rho fit.**  The matched rho fits `r^{0.44}`, not `1/2`, because set-up is still a
     visible share at these sizes (as in E8, E11 and E16).
  The `ln N` and the scalar multiplications add `≈ 0.1` to the relation phase's exponent and the
  rho fit removes `0.06`, which accounts for the predicted `−1/6` reading as `−0.01`.

The cofactor costs a further constant.  With `h = 8`, rho on `r ≈ p³/8` is `√8` cheaper than on a
prime-order curve with the same base.  A cross-harness comparison is therefore only context: the
best `k = 3` method of `RESEARCH_RESIDUAL_WALKS.md` §11 is the plain method at `528×` rho at
`2^{33.1}`, on prime-order curves with filtering and walk-generated residuals.  This arm is at
`236`–`265×` at `2^{33.1}`–`2^{33.5}`.

**Class: engineering.**  The solve constant is `÷ 38`, and the fold halves the columns exactly
and the linear algebra by four.  No exponent moved, and §11.15's verdict on `k = 3` stands: the
plain method is bounded away from rho at every size.  What E17 establishes for the goal is that
both levers survive the move to a non-subfield curve, which is the precondition for using them at
`k = 4` (§9.1).

### 9.3 E18 — the folds on the `k = 4` linear algebra, measured

**Runner:** `cargo run --release --example quartic_folds -- --family psi --sizes
241,337,541,601,733,769,997,1069 --seeds 1 --rho-runs 16 --json experiments/26_quartic_folds_psi.json`
and `-- --family tau --sizes 269,521,769,1033 --seeds 2 --rho-runs 16 --json
experiments/26_quartic_folds_tau.json`.  **Data:** `experiments/26_quartic_folds_{psi,tau}.{json,log}`
(2026-10-09).  **Tables:** `cargo run --release --example glv_invariant_experiment_tables --
experiments/26_quartic_folds_psi.json experiments/26_quartic_folds_tau.json` (the E18 printer).
**Source:** `src/cryptanalysis/quartic_folds.rs`, `examples/quartic_folds.rs`.  **Registered in
advance:** §9.1, pushed (`c4f29698f`) before the module existed.

**Three changes the smoke runs forced, all made before the frozen runs and all recorded in the
rows:**

1. **Identical rows are skipped.**  On the `ψ` arm, two decompositions of one target that differ
   by `P + ψP + ψ²P = O` fold to one row.  Kept, they made half the folded cores singular (rank
   `13` of `17` at `p = 97`; `19`–`46` solve attempts).  The control keeps such pairs in distinct
   columns, where they are independent.  `duplicate_rows` counts the skipped rows: `0` on every
   frozen row, because of rule 2.
2. **Each target contributes one decomposition, drawn uniformly from its verified ones.**  On a
   curve with rational `T`, each decomposition comes with the seven that add `T` to an even number
   of summands, and the eight rows have rank `5`.  Square cores drawn from such families were
   singular, and the stream never ended (`p = 61`, `97`).  Taking the oracle's first decomposition
   instead would pick the lower index of every `{P, P + T}` pair, half-folding the control: its
   core shrank to `φ ≈ 0.67`, and the linear-algebra ratio read `1.6`–`2.6` where the uniform draw
   gives `3.0`–`4.2`.  §11.19 kept every decomposition, which on its prime-order curves is one per
   target with rare exceptions.  `decompositions_per_target` records the histogram on every row.
3. **A fruitless cycle is escaped by doubling.**  Restarting the walk cost two scalar
   multiplications, about `87` times per negation walk at `n = 2^{32}`, `13 %` of the walk.  A
   test now holds each walk's steps to its class size: unfolded / negation in `[1.15, 1.75]`,
   unfolded / `ψ` in `[1.9, 3.1]`.

**The `ψ` arm: `8` prime-order `j = 0` curves over `F_{p⁴}`**, `p = 241`–`1069`,
`n = 2^{31.7}`–`2^{40.2}`.  One curve per `p`: the six sextic twists fix the possible orders, and
two seeds at one `p` would land on isomorphic curves.  **Every logarithm was recovered on both
arms, and all `384` walks verified.**

| p | log₂ n | cols control / folded | control core (φ) | folded core (φ) | LA ratio | r control / unfolded walk | r control / ψ walk | r folded / ψ walk |
|--:|--:|:--|:--|:--|--:|--:|--:|--:|
| 241 | 31.7 | 120 / 40 | 103 (0.858) | 34 (0.850) | 9.20 | 0.487 | 1.083 | 0.118 |
| 337 | 33.6 | 165 / 55 | 151 (0.915) | 49 (0.891) | 9.49 | 0.535 | 1.189 | 0.125 |
| 541 | 36.3 | 273 / 91 | 239 (0.875) | 86 (0.945) | 7.77 | 0.520 | 1.155 | 0.149 |
| 601 | 36.9 | 291 / 97 | 256 (0.880) | 87 (0.897) | 8.68 | 0.483 | 1.074 | 0.124 |
| 733 | 38.1 | 414 / 138 | 367 (0.886) | 126 (0.913) | 8.49 | 0.667 | 1.483 | 0.175 |
| 769 | 38.3 | 438 / 146 | 387 (0.884) | 130 (0.890) | 8.86 | 0.674 | 1.498 | 0.169 |
| 997 | 39.8 | 477 / 159 | 431 (0.904) | 136 (0.855) | 10.05 | 0.498 | 1.106 | 0.110 |
| 1069 | 40.2 | 561 / 187 | 495 (0.882) | 162 (0.866) | 9.33 | 0.571 | 1.268 | 0.136 |

Rho's `S`, pooled over `128` walks per fold in `F_p` multiplications per `√n`: unfolded
**`120.3 ± 5.5`**, negation **`91.0 ± 4.6`**, `ψ` **`54.1 ± 2.6`**, so unfolded / negation is
`1.32` and negation / `ψ` is `1.68`.  `r` pooled over the curves, against each walk:

| arm | unfolded walk | negation walk | `ψ` walk | fitted exponent in `n` |
|:--|--:|--:|--:|--:|
| control (`⟨−1⟩`) | `0.554 ± 0.027` | `0.733 ± 0.036` | `1.232 ± 0.061` | `0.03` |
| `ψ`-folded | `0.062 ± 0.004` | `0.082 ± 0.005` | **`0.138 ± 0.008`** | `0.02` |

**Reading, against §9.1.**

- **G2.6 holds.**  Unfolded / negation is `1.32` in multiplications, inside `[1.25, 1.6]`; the
  canonical representative and the look-ahead cost the negation walk part of its `√2`.  So
  §11.19's `r∞ = 0.518`, which is against the unfolded walk, reads **`0.73` against the negation
  walk**: past the handover the plain method beats the walk this program uses everywhere else by
  `1.36×`, not `1.9×`.  The control reproduces §11.19 on different curves, `0.554 ± 0.027`
  against `0.518 ± 0.031`.
- **G2.7 holds on every component.**  Columns `÷ 3.00` on every curve; linear algebra `÷ 7.8`–`10.1`
  (`÷ 9` predicted, with the filtering fractions equal to within `±0.07`); the `ψ` walk `÷ 1.68`
  against the negation walk (`√3 = 1.73`); and **`r_ψ = 0.138 ± 0.008`** against the matched
  `ψ`-folded walk (`≈ 0.14` predicted).  The control on the same curves is **`1.23`** against that
  walk (`≈ 1.27` predicted).  Both are flat in `n`.

So on a `j = 0` curve the plain `k = 4` method does **not** beat the rho an attacker would run
there, at any size, because its linear algebra alone costs `1.23×` that walk.  The `ψ`-folded base
does, asymptotically, by **`1/0.138 ≈ 7.2×`**.

**The `τ_T` arm: `8` curves over `F_{p⁴}` with rational `T`**, `p = 269`–`1033` (§11.19's sizes),
two curves per `p`, `n = 2^{30.3}`–`2^{38.1}`.  The two `p = 1033` curves ran one per invocation
(`experiments/26_quartic_folds_tau_1033s{1,2}.{json,log}`), after two restarts of the host had
stopped the run.  **Every logarithm was recovered on both arms, and all `256` walks verified.**

| p | log₂ n | h | cols control / folded | control core (φ) | folded core (φ) | LA ratio | decompositions per decomposed target | residuals per relation | r control / negation walk | r folded / negation walk |
|--:|--:|--:|:--|:--|:--|--:|--:|--:|--:|--:|
| 269 | 30.3 | 4 | 134 / 67 | 125 (0.933) | 57 (0.851) | 4.87 | 9.23 | 91.5 | 1.526 | 0.313 |
| 269 | 30.3 | 4 | 116 / 58 | 104 (0.897) | 54 (0.931) | 3.75 | 7.67 | 198.6 | 1.057 | 0.282 |
| 521 | 34.1 | 4 | 264 / 132 | 241 (0.913) | 121 (0.917) | 3.98 | 7.59 | 82.8 | 1.511 | 0.380 |
| 521 | 34.1 | 4 | 268 / 134 | 234 (0.873) | 117 (0.873) | 4.04 | 7.80 | 88.0 | 1.424 | 0.353 |
| 769 | 36.3 | 4 | 378 / 189 | 342 (0.905) | 170 (0.899) | 4.07 | 7.84 | 108.4 | 1.396 | 0.343 |
| 769 | 36.3 | 4 | 370 / 185 | 329 (0.889) | 175 (0.946) | 3.55 | 7.84 | 105.2 | 1.292 | 0.364 |
| 1033 | 38.1 | 4 | 496 / 248 | 439 (0.885) | 219 (0.883) | 4.02 | 7.87 | 119.6 | 1.275 | 0.317 |
| 1033 | 38.1 | 4 | 524 / 262 | 482 (0.920) | 240 (0.916) | 4.05 | 7.90 | 94.5 | 1.536 | 0.379 |

Rho's `S`, pooled over `128` walks per fold: unfolded `118.9 ± 5.4`, negation `90.8 ± 4.2`
(ratio `1.31`).  Pooled `r`: control `1.051 ± 0.044` (unfolded walk), `1.377 ± 0.058`
(negation walk); folded **`0.261 ± 0.009`** (unfolded), **`0.341 ± 0.012`** (negation); fitted
exponents in `n` `0.01` and `0.03`.

**Reading, against §9.1.**

- **G2.8 holds as registered, at the cofactor the search could reach.**  Columns `÷ 2.00`, linear
  algebra `÷ 3.55`–`4.87` (`÷ 4` predicted), and `r_τ = 0.341 ± 0.012` against the negation walk,
  under the `0.50` falsifier.
- **Deviation: every curve has `h = 4`, never `2`.**  The search took its fallback on all eight,
  and E17's `24` curves over `F_{p³}` all had `h ∈ {4, 8}`.  The construction makes `f'(x₀) = c²` a
  square, which is the necessary condition for `T` to be halvable, i.e. for a rational 4-torsion
  point.  So `h ≥ 4` is very likely structural for this coordinate.  That is not proved here and is
  recorded as measured.  The cofactor costs `√h` against rho, so the registered `≈ 0.26` at `h = 2`
  becomes `0.518·√2·√4/4 ≈ 0.37` at `h = 4`, and the control's `≈ 1.04` becomes `≈ 1.47`.  Measured:
  `0.341` and `1.377`.
- **What the registration did not price: the relation yield.**  Each decomposed target carries
  `7.6`–`9.2` verified decompositions, the family that adds `T` to an even number of summands, and
  the folded arm turns the whole family into one row.  Pooled, a relation costs **`106` residuals**
  on these curves, against **`22.5`** on the `j = 0` curves (`2,559` relations from `270,464`
  residuals, against `2,761` from `62,080`; §11.16 assumes `24`).  `r` does not see this, since it
  prices the linear algebra.  The handover does.

**The handovers, re-derived on the measured yields (extrapolation, not measurement).**  §11.16's
form: relation phase `y·(columns)·C₄`, with `y` the measured residuals per relation, `columns`
from the measured `|F| ≈ p/2`, and `C₄ = 1.213·10¹²` (§11.17).  It is set against rho
`S·√n`, with the matched walk's pooled `S` and `√n = p²/√h`.  Then
`S / rho = y·(cols/p)·C₄·√h / (S_rho·p) + r`, and `p* = y·(cols/p)·C₄·√h / (S_rho·(1 − r))`.

| method, curves | against | `y` | `r` | `p*` | `n*` |
|:--|:--|--:|--:|--:|--:|
| plain, prime order (control of the `ψ` runs) | unfolded walk (§11.19's convention) | `22.5` | `0.554` | `2.5·10¹¹` | `2^{151.6}` (§11.19: `2^{151.1}`) |
| plain, prime order | negation walk | `22.5` | `0.733` | `5.6·10¹¹` | `2^{156.1}` |
| plain, `j = 0` | `ψ` walk | `22.5` | `1.232` | — | never |
| **`ψ`-folded, `j = 0`** | **`ψ` walk** | `22.5` | **`0.138`** | `9.8·10¹⁰` | **`2^{146.0}`** |
| plain, 2-torsion (`h = 4`) | negation walk | `106` | `1.377` | — | never |
| `τ_T`-folded, 2-torsion (`h = 4`) | negation walk | `106` | `0.341` | `1.1·10¹²` | `2^{157.9}` |

**So the two folds part.**  The **`ψ` fold moves the frontier**: on a `j = 0` curve, against the
rho an attacker would run there, the asymptotic edge is `1/0.138 ≈ 7.2×`, against the plain
method's `1/0.733 ≈ 1.36×` over the negation walk on a prime-order curve.  The handover comes
`10` bits earlier than that (`2^{146.0}` against `2^{156.1}`).  The **`τ_T` fold improves only
its own curves**: `r` falls `4×` there, but those curves pay a cofactor of `4` and a `4.7×` poorer
relation yield, so they hand over `1.8` bits *later* than a plain prime-order curve.  A curve
with this `T` is a worse target than one without, even with the fold.

**Class: engineering.**  Both folds move constants of an asymptote that is already rho's exponent
at `k = 4`.  No exponent moved, no end-to-end `S` below rho exists, and every number past `n =
2^{40}` is an extrapolation on §11.16's exponents and §11.17's `C₄`.

### 9.4 Verdict on G2, and the next lever

| stage | what moved | measured | class |
|:--|:--|:--|:--|
| E17, `k = 3`, the `D₃` solve on the full group | `C₃` | `÷ 38.0` (`33.7`–`43.0`) against `S₃` Macaulay, on non-subfield curves with `F_{p³}` Möbius coefficients; end to end still `180`–`377×` rho (G2.4, G2.5 falsified by the harness) | engineering |
| E17, `k = 3`, the `τ_T` fold | columns | `÷ 2.00` exactly | engineering |
| E18, `k = 4`, the `ψ` fold | `r∞` against the matched walk | **`0.138 ± 0.008`** (plain on the same curves: `1.232`); handover `2^{156.1} → 2^{146.0}` against the negation walk's | engineering |
| E18, `k = 4`, the `τ_T` fold | `r∞` on its own curves | `0.341 ± 0.012` (plain: `1.377`), but `h = 4` and a `4.7×` poorer yield: handover `2^{157.9}` | engineering |
| E18, `k = 4`, the reference | the walk `r∞` is quoted against | negation walk `1.31`–`1.32×` cheaper than §11.19's unfolded one, so §11.19's `0.518` reads `0.733` against it | accounting |

**What G2 established.**  The levers this note built on subfield curves, where §8.10 showed they
could buy only a constant against a `Θ(p)` rho, carry over to curves where index calculus has
rho's exponent or better: the `D₃` solve intact (`÷ 38`), and the folds intact and exact.  At
`k = 4`, the one setting in this repository where a plain method has a regime below rho at all,
the `ψ` fold is the largest constant on the linear-algebra side.  It turns a `1.36×` asymptotic
edge into `7.2×` and brings the handover `10` bits earlier.

**What it did not.**  The handover is still `2^{146}`, set by `C₄ = 1.2·10¹²` per `S₅` solve.  No
end-to-end `k = 4` run below rho exists, here or in §11.  On curves with rational 2-torsion the
fold cannot pay for the curve's poorer yield.

**The next lever, named and not yet registered.**  §11.17: "every factor `f` taken off `C₄` moves
`n*` about `4 log₂ f` bits earlier", with `C ≈ D^{3.1}` in this solver.  The two candidates:

1. **`ψ` on the system.**  On a `j = 0` curve, `S₅` is weighted-homogeneous in `x` (weight `2`) and
   `b` (weight `6`), so the descended system carries a `Z/3` grading.  §2 of
   `RESEARCH_GLV_INDEX_CALCULUS.md` found the analogous `k = 3` grading usable only on the
   orbit system, at `12`–`30×` the cost.  At `k = 4` it needs re-deriving, not assuming.
2. **`D₄` on the system.**  The FGHR symmetry, `(Z/2)³ ⋊ S₄`, on curves with rational `T`, cuts
   the Bézout number `4096 → 512`.  That is `≈ 8^{3.1} ≈ 630×` on `C₄` if the generic scaling
   holds, worth `≈ 37` bits of `n*`.  E18 shows those curves pay `≈ 8` bits back through the
   cofactor and the yield.  The `k = 5` analogue in `RESEARCH_K5_TORSION_JOUX_VITSE.md` measured
   `416×` for its four-point test.

Either is a separate build: the symmetrised `S₅` over `F_{p⁴}` and a weighted Macaulay solve.  Each
would be registered with its own falsifier before any code, as E17 and E18 were.

### 9.5 Addendum and E19 pre-registration: the `C₄` lever was taken elsewhere; the method end to end

*Added 2026-10-17, before any E19 code exists.*

**The lever §9.4 named has been built by other lanes; this note does not rebuild it.**  C8
(`research/gaudry_quartic_d4_20261013/`, PR #1785) ran the `D₄`-symmetrised `S₅` on E17/E18's
2-torsion construction: `C₄ = 2.54·10¹⁰`, `48×` below the `S₄` form.  Lane `d4-s5`
(`research/d4_symmetrised_s5_20261015/`) then built the weighted Macaulay solve that §9.4 called
for, with four unknowns of weights `(2, 2, 2, 1)`, weighted Bézout 512 and degree 26.  It measured
**`C_w ≈ 1.2`–`1.4·10⁹`** (`854×` below `S₄` at `p = 269`), past §9.4's `≈ 630×` projection.
Read in E18's frame (`h = 4`, `106` residuals per relation, `r = 0.341`), it puts the handover at
**`n* ≈ 2^{118.9}`**, against the `ψ` arm's `2^{146.0}`.  The `ψ`-on-the-system alternative of
§9.4 stays closed: as at `k = 3` (`RESEARCH_GLV_INDEX_CALCULUS.md`), `S₅` is graded only under
the simultaneous scaling that also moves the target.

**What no lane has done: run the method.**  Every `k = 4` handover in this repository, C8's,
`d4-s5`'s and E18's, is §11.16's model.  It is assembled from per-phase constants measured in
separate harnesses: `C` from solves on planted and uniform targets, the yield and `r` from E18's
meet-in-the-middle stand-in, and rho from E18's walks.  No run has collected relations with the
`D₄` solve itself, filtered and solved the matrix, and recovered a logarithm.  C8's decision rule
names that pipeline as the next step.

**E19 — the first end-to-end `k = 4` index calculus with the `D₄` solve (registered here).**  On
E18's 2-torsion instances (`quartic_folds::generate_translation_instance`, `h ≤ 4`), with E18's
`τ_T`-folded base `x ∈ x₀ + F_p`:

1. draw targets `R = aG + bQ`;
2. decompose each with `d4_weighted_s5::solve_weighted` at weighted degree 26, preceded by
   `weighted_system` and followed by `lift_back`, every multiplication charged;
3. map the lifted `Y`-quadruple to base points and signs by a charged group check;
4. keep one decomposition per target, drawn uniformly (E18's rule 2), as a folded row;
5. filter and solve by §11.7's Wiedemann until `[d]G = Q` verifies;
6. price the whole run in `F_p` multiplications against the matched negation walk on the same
   curve (`16` walks).

The total is the instance's `SymmetrisedD4` precompute, target generation, Weil restriction,
solves, lift-back, the group checks, and the linear algebra at `16` per mod-`n` multiplication.
Unsolvable targets (C8's degenerate `5 %`) are charged and counted, never dropped.  Sizes:
`p = 53, 101, 149, 197, 269`, two curves each (`n ≈ 2^{20.9}`–`2^{30.3}`).

| id | quantity | prediction (from C8, `d4-s5` and E18, before any run) | falsified by |
|:--|:--|:--|:--|
| G2.9 | correctness | every curve's logarithm recovered; every relation re-added in the group; every solve's decompositions are base points summing to `R` | any failure |
| G2.10 | residuals per relation, exact oracle | E18's `106` (`83`–`199` per curve) from the stand-in, now from the solve | pooled `y` outside `[70, 160]` |
| G2.11 | the solve in the pipeline | per-call `C` within `±30 %` of `d4-s5`'s medians at the same `p` (`1.2`–`1.4·10⁹`); the unsolved share `≤ 10 %` | outside the band at any size, or more than `10 %` unsolved |
| G2.12 | end to end | `S / rho S ≈ 0.584·C/p` (the relation phase, `y·(p/4)·C` against `90.8·p²/2`): `≈ 3·10⁶×` at `p = 269`, `≈ 1.4·10⁷×` at `p = 53`; fitted slope in `n` `−1/4 ± 0.1`; every phase other than the solves `< 10 %` of the total | a measured ratio outside `×2` of the prediction at any size, or a slope outside `[−0.35, −0.15]` |
| G2.13 | the handover, re-derived from E19's own constants | within `±3` bits of `d4-s5`'s `2^{118.9}` | outside `[2^{115.9}, 2^{121.9}]` |

**What it would and would not say.**  If G2.9–G2.13 hold, the `2^{118.9}` handover rests on
constants measured inside one pipeline, and only the exponents (`n^{1/4}`, `n^{1/2}`) remain
extrapolated.  The measured rows are **`10⁶`–`10⁷×` slower than rho**: the distance remaining,
reported as such, with no speed claimed.  Under the 2026-10-15 success criterion, E19 is a
**measurement** of the current `k = 4` baseline, not an improvement of it.  **Class:
accounting** for the method's first measured `S`, and engineering for nothing.

### 9.6 E19 measured: the first end-to-end `k = 4` index calculus, `2.6`–`8.8·10⁶×` from rho

*2026-10-18.*  `experiments/27_quartic_pipeline_d4/run_all.sh` (one curve per invocation,
`--rho-runs 16`), binary `4e5810598`; raw `p{53,101,149,197,269}s{1,2}.json`, log `run.log`
(one host restart, p = 101 seed 1 rerun from scratch, recorded there), tables `tables.md`.  Host:
the 4-vCPU container, shared with E20–E24 for part of the run (operation counts only).

| p | seed | log₂ n | cols | targets | y | unsolved | `C` median | `S` | rho (negation) | `S` / rho |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 53 | 1 | 20.9 | 15 | 1,498 | 93.6 | 2 | `1.37·10⁹` | `2.41·10¹²` | `3.40·10⁵` | `7.08·10⁶` |
| 53 | 2 | 20.9 | 13 | 1,926 | 137.6 | 5 | `1.37·10⁹` | `3.00·10¹²` | `3.43·10⁵` | `8.75·10⁶` |
| 101 | 1 | 24.6 | 23 | 2,608 | 108.7 | 0 | `1.39·10⁹` | `3.91·10¹²` | `6.18·10⁵` | `6.33·10⁶` |
| 101 | 2 | 24.6 | 23 | 2,701 | 112.5 | 0 | `1.39·10⁹` | `4.01·10¹²` | `6.86·10⁵` | `5.85·10⁶` |
| 149 | 1 | 26.9 | 37 | 3,525 | 92.8 | 0 | `1.40·10⁹` | `5.26·10¹²` | `1.29·10⁶` | `4.09·10⁶` |
| 149 | 2 | 26.9 | 41 | 2,868 | 68.3 | 0 | `1.40·10⁹` | `4.30·10¹²` | `1.26·10⁶` | `3.40·10⁶` |
| 197 | 1 | 28.5 | 48 | 7,061 | 144.1 | 0 | `1.41·10⁹` | `1.04·10¹³` | `2.11·10⁶` | `4.91·10⁶` |
| 197 | 2 | 28.5 | 53 | 4,306 | 79.7 | 0 | `1.41·10⁹` | `6.29·10¹²` | `2.22·10⁶` | `2.83·10⁶` |
| 269 | 1 | 30.3 | 58 | 11,975 | 203.0 | 0 | `1.41·10⁹` | `1.74·10¹³` | `3.17·10⁶` | `5.51·10⁶` |
| 269 | 2 | 30.3 | 67 | 6,200 | 91.2 | 0 | `1.41·10⁹` | `9.01·10¹²` | `3.50·10⁶` | `2.57·10⁶` |

All `h = 4`.  Every log recovered; every kept row re-added in the group; no lifted quadruple off
the base (`Y = 0` never met) or without an exact sign vector; all 320 walks correct.  The solves
are `100.0 %` of `S` on every curve (everything else `≤ 0.001 %`); unpriced native work at
`0, 0.1, 1` GAE moves `S / rho` by at most `1.5×` and never reverses the order.

| id | result | verdict |
|:--|:--|:--|
| G2.9 | every log, row and decomposition verified | **holds** |
| G2.10 | pooled `y = 115.1` | **holds** (`[70, 160]`) |
| G2.11 | `C` median `1.37`–`1.41·10⁹` (d4-s5's `1.2`–`1.44·10⁹`); unsolved `7` of `44,668` targets | **holds** |
| G2.12 | measured / registered `0.584·C/p`: per size (mean of two curves) `0.55, 0.81, 0.73, 1.00, 1.43` — inside `×2`; one curve (`p = 53` seed 1) at `0.495`, a hair outside; fitted slope in `n` **`−0.13`**; non-solve phases `0.001 %` | **holds per size; slope falsified** (`[−0.35, −0.15]`): rho's `S` per `√n` carries its set-up at small `n` (`242` at `p = 53` against `92` at `p = 269`), which flattens the fit |
| G2.13 | handover from E19's own constants `2^{116.7}` (`2^{120.0}` from `p = 269` alone) | **holds** (`[2^{115.9}, 2^{121.9}]`) |

**What it says.**  The `2^{119}` handover that C8, d4-s5 and E18 assembled from separate
harnesses is reproduced from one pipeline's own constants (`2^{116.7}`).  The method recovers
logarithms at `k = 4` with every phase priced, and is **`2.6`–`8.8·10⁶×` slower than rho**: the
distance remaining.  **Class: accounting** (the baseline measured, not moved).  E24 (§9.14) moves it.

### 9.7 Pre-registration of E20–E23: the four directions after E19

*Added 2026-10-17, while E19 runs (5 of 10 curves in), before any E20–E23 code exists.  Owner
request: pre-register the double-large-prime run and pursue all four directions named in the
session's review of where `k = 4` cost lives.*

**Where the cost is (E19, five curves so far).**  The weighted `D₄` solve is `100.0 %` of `S`
on every curve; generation, Weil restriction, lift-back, group checks and the linear algebra
together are `≤ 0.001 %`.  With the full base the relation phase grows as `p` and the linear
algebra as `p²`, a fixed share `r` of rho (`0.34` on this family, E18), so the full-base method
is at best `1/r` faster than rho, past the handover; it shares rho's exponent.  Four
directions follow, each registered below with its prediction and what would falsify it.

**A structural cost of translation folds, stated before E22 relies on it.**  A translation by
rational torsion preserves the sum `P₁ + … + P₄`, so a target's decompositions arrive in orbits:
`8` for `⟨τ_T⟩` (`T` added to an even number of summands), `64` for the full 2-torsion group
`V₄ = {O, T₁, T₂, T₃}` (tuples in `V₄⁴` summing to `O`).  One row per target survives the fold,
so the fold halves the columns per factor 2 of the group while the orbit grows by the cube of
that factor: E18's `106` residuals per relation against `22.5` on `j = 0` is this effect.  The
automorphism `ψ` does not cluster (`ψ` moves the target).  Derived, to be measured in E22: going
from `⟨τ_T⟩` to `V₄` costs up to `4×` more targets for the same matrix.

#### E20 — double large primes at `k = 4` on the `τ_T`-folded base (direction 1)

The only lever here that changes the exponent: Gaudry/Diem's double-large-prime variation gives
`Õ(q^{2−2/n})`, `N^{3/8}` at `n = 4` against rho's `N^{1/2}`; the repository has it at `k = 3`
(`RESEARCH_RESIDUAL_WALKS.md` §11.7, `N^{4/9}`, `2,599×` rho at `2^{33.1}`) and not at `k = 4`.

- **Protocol.**  On E18/E19's 2-torsion instances (`h ≤ 4`) and `τ_T`-folded base `F`
  (columns = `τ_T`-classes), a small base `F' ⊂ F` of `⌈|F|^{3/4}⌉` columns (Gaudry's
  `|F|^{1−1/n}`), chosen uniformly; a relation may carry at most two columns of `F \ F'`; large
  primes eliminated by §11.7's pivot reduction (one pivot per large prime; cycles in the graph
  give full relations); filtering and Wiedemann on `F'` until `[d]G = Q` verifies.  Every
  combined relation is re-added in the group before the solve.
- **Oracle substitution, declared.**  The decomposition sets come from E18's exact
  meet-in-the-middle oracle (`mitm_signed`), which returns the same decompositions as the
  weighted solve (d4-s5's brute-force and meet-in-the-middle agreement; E19's G2.9).  Every
  target is charged **one weighted solve at E19's pooled per-call `C`** (flat in `p`,
  `1.37`–`1.40·10⁹` median so far), plus the lift-back and checks at E19's measured shares.
  This lets E20 reach `p` where the large-prime graph has cycles, which the solve itself cannot
  reach in this container's budget.  The full-base arm runs on the same stream and the same
  pricing, so the comparison is paired.
- **Sizes.**  `p = 269, 521, 1033, 2053`, two curves each; `16` negation walks per curve.

| id | quantity | prediction | falsified by |
|:--|:--|:--|:--|
| G2.14 | correctness | every curve's log recovered; every combined relation re-adds to `O` | any failure |
| G2.15 | at measured sizes | `S_DLP > S_full` at every `p` (the linear algebra is `≈ 0 %` of `S_full`, so shrinking it buys nothing while the targets per full relation rise) | `S_DLP ≤ S_full` at any measured `p` |
| G2.16 | targets per full relation | `y_DLP / y_full` grows with `p`; fitted exponent in `p` within `[0.25, 1.0]` | outside the band |
| G2.17 | the crossovers, derived from E20's own constants | `S_DLP = S_full` only past `n ≈ 2^{200}`; DLP below rho only past `2^{200}` | either below `2^{160}` |

**What it would say.**  If G2.15–G2.17 hold, double large primes are the right exponent and the
wrong constant: at `C ≈ 1.4·10⁹` the solve, not the matrix, decides every size anyone can run,
and the exponent lever waits on `C`.  That is the reason E21 and E22 exist.  **Class:
measurement** (a constant and an exponent of a known variant, not an improvement).

#### E21 — the solve's repeated structure: trace replay on the weighted solve (direction 2a, 2c)

Every target's system has the same monomial support; only the coefficients (polynomials in
`Y_R`) change.  F4Remake's idea, already built for the unweighted solve
(`gaudry_quartic::solve_system4_traced`, rows reducing to zero skipped; `RESEARCH_COVER_DECOMPOSITION_LEDGER.md`
§12 measured `2.44×` on an F4 there), is not wired into the weighted `D₄` solve.  E21 adds a
shape trace to `d4_weighted_s5::solve_weighted`: recorded on a curve's first target, it skips
the input rows that reduced to zero (`291` of `3,979` at degree 26) and checks leading
monomials, pivots and quotient dimension on every replay, falling back to the full solve (charged)
on any mismatch.  It also records which pivots the normal forms reach (`2,877` of `3,688`) as the
ceiling measurement for direction 2c (the normal forms are the largest phase, `7.4·10⁸` of
`1.41·10⁹`).

| id | quantity | prediction | falsified by |
|:--|:--|:--|:--|
| G2.18 | agreement | replayed and full solves return identical decomposition sets on every target (`p = 53, 101`, `≥ 200` targets each) | any disagreement |
| G2.19 | per-call `C`, replayed / full | `÷ 1.03`–`1.20` (zero rows are `7 %` of the rows; the elimination is `34 %` of `C`) | outside the band |
| G2.20 | fallback rate | `≤ 5 %` of targets at `p ≥ 101` | more |
| G2.21 | 2c ceiling | the share of echelon pivots the normal forms never reach, measured; registered only as a ceiling on what a reached-pivot-only elimination could save | — (measurement) |

**Class: engineering** (a constant on one phase), and named as such if it holds.

#### E22 — the full 2-torsion symmetry `V₄` on a stable line (direction 2b, with direction 3)

The successor of what `D₄` gave (`48×` over C8's form, `854×` with the weighted solve): with
three rational 2-torsion points, the group acting on decompositions is `V₄⁴_0 ⋊ S₄` of order
`64 · 24 = 1536` against `D₄`'s `192`, and the weighted Bézout number would fall from `512` by up
to `8`.  Two facts, derived before any code, decide its form:

1. **A `V₄`-stable `F_p`-line through `x(O) = ∞` is confined.**  If the base line is
   `x ∈ a + b·F_p` (any line containing `∞`), `V₄` moves `∞` to the three 2-torsion abscissas,
   so all three lie on the line: `f(a + bu) = b³·g(u)` with `g ∈ F_p[u]`.  Base points have
   `y² = b³·g(u)` with `g(u) ∈ F_p ⊂ (F_{p⁴}^*)²`: if `b³` is a square, `E` is isomorphic over
   `F_{p⁴}` to a curve over `F_p` and every base point lies in the image of its `F_{p²}`-points
   (order `≈ p²`, no large prime); if not, the base is empty.  E19's `x₀ + F_p` escapes this only
   because a single `T` is used: the other two roots are off the line.
2. **A `V₄`-stable line avoiding `∞` exists and is not confined by this argument.**  Conjugate
   `V₄` into `PGL₂(F_p)` as `⟨z ↦ −z, z ↦ κ/z⟩`, `κ ∈ F_p^*`: the invariant binary quartics are the
   pencil `z⁴ + μz² + κ²`, so the curve is `v² = λ(z⁴ + μz² + κ²)` with `μ ∈ F_{p⁴}` free.  With
   `μ ∉ F_p` the base `z ∈ F_p` is not all-or-nothing.  Whether both involutions are translations
   by rational 2-torsion (and not negations about a point), and what the base density is, is
   Stage A.

- **Stage A (gate).**  Construct such curves over `F_{p⁴}` (a quartic model with a rational
  point, mapped to Weierstrass), with three rational 2-torsion points whose translations induce
  `z ↦ −z` and `z ↦ κ/z`; verify on points that the base `z ∈ F_p` is stable, that the folds are
  translations (coefficient `1`), its density, and that relations reach the large subgroup (a
  meet-in-the-middle stream recovers a logarithm at `p ≈ 101`).  Gate: all four hold.
- **Stage B (only after A).**  `S₅` written in invariants of `V₄⁴_0 ⋊ S₄`, Weil-restricted,
  solved with a weighted Macaulay solve on the invariant ring's generators; brute-force agreement
  on tiny fields; per-call `C` at `p = 101, 269` against `C_w`.
- **Direction 3 inside E22.**  Targets per relation `y` and decompositions per target measured
  on the `V₄` base against E19's on the same `p`.

| id | quantity | prediction | falsified by |
|:--|:--|:--|:--|
| G2.22 | Stage A | such curves exist at every registered `p`; base density `≈ 1/2`; the folds are translations; a log is recovered | any of the four fails (then E22 stops at A, recorded as a negative) |
| G2.23 | Stage B, `C_V₄` | `C_w / C_V₄` in `[4, 30]` | outside |
| G2.24 | direction 3 | decompositions per decomposed target `≈ 64` (against E19's `≈ 8`); `y_V₄ / y_E19` in `[3, 10]` at matched `p` | outside |
| G2.25 | net | the relation phase moves by `(C_w / C_V₄) / (targets ratio)`: a gain only if `C` falls by more than the target penalty (derived above, up to `4×`) | reported either way; a gain below `1` is a negative |

**Class:** engineering if it holds (a constant), recorded as a negative if Stage A fails.

#### E23 — `ψ` and `τ_T` together on `j = 0` (direction 4): closed by derivation, confirmed on points

§9.4 closed `ψ` on the system.  The remaining question was a base stable under both folds.
On `j = 0`, `E : y² = x³ + b`, a rational 2-torsion point `T = (x₀, 0)` forces `b = −x₀³`, and
since `ω ∈ F_p` all three roots `x₀, ωx₀, ω²x₀` are rational.  The line `x ∈ x₀·F_p` **is**
stable under the whole group `⟨−1, ψ, τ_{T₁}, τ_{T₂}⟩` of order `24`: `ψ(x₀t) = x₀(ωt)`, and
translation by `(ω^i x₀, 0)` maps `x₀t ↦ x₀(ω^i + 3ω^{2i}/(t − ω^i))` (`f′(x) = 3x²`).  But it
passes through `∞`, so fact 1 above applies: `y² = x₀³(t³ − 1)`, and either `x₀³` is a square,
`E ≅ v² = t³ − 1` (defined over `F_p`) and every base point lies in its `F_{p²}`-points, or the
base is empty.  The line `x₀ + F_p` (E19) is not `ψ`-stable (`(ω − 1)x₀ ∈ F_p` would force
`x₀ ∈ F_p`).  **Registered result, before the check:** no base on a `j = 0` curve carries both
folds without confinement.  **Confirmation (a unit test, cheap):** on `j = 0` curves over
`F_{p⁴}` with rational 2-torsion at small `p`, build the `x₀·F_p` base, verify the order-24
stability on points, and verify that every base point is killed by `#C(F_{p²})` (or that the base
is empty).

| id | quantity | prediction | falsified by |
|:--|:--|:--|:--|
| G2.26 | E23 | on every tested curve: the base is stable under the order-24 group, and either empty or killed by `#C(F_{p²})` | any non-confined base point |

**Order of work.**  E23 (a test), E21 (a trace on one solver), E20 (a stream reprocessed with
priced solves), E22 Stage A, then Stage B if the gate passes.  E19 finishes first in the
background; none of these change E19's registered protocol.

### 9.8 E23 measured: `ψ` and `τ_T` share no useful base on `j = 0`

*2026-10-17.*  `quartic_folds::e23_tests::psi_and_translations_share_only_a_confined_base_on_j0`:
`p = 37, 61, 73, 97`, six random `x₀ ∈ F_{p⁴} \ F_p` each, `E : y² = x³ − x₀³`.  On every curve
the three 2-torsion points are on `E`; where the `x₀·F_p` base is non-empty, every base point's
`ψ`-image and all three translates stay on the line, and every base point is killed by
`#C(F_{p²})`, `C : v² = t³ − 1`; the other curves have an empty base.  Both branches occur.
**G2.26 holds: direction 4 is closed** — the order-24 group exists on that line, and the line
carries no point of the large subgroup.  Class: a negative, by derivation, confirmed on points.

### 9.9 E20 measured: double large primes at `k = 4` on the folded base — the right exponent, the wrong constant

*2026-10-18.*  Run: `experiments/28_e20_dlp/run.sh` (the ic_next10 ladder, `--arms d4dlp,rho
--c4-d4 1.40e9`, one thread, 4 GB cap); raw `experiments/28_e20_dlp/ladder.jsonl`; tables
`experiments/28_e20_dlp/tables.md` (`glv_invariant_experiment_tables`, `e20`).  **Deviations from
§9.7, stated:** (1) `p = 2053` was dropped, because the ladder's exact three-point oracle table does
not fit in memory there (`≈ 12 GB`); sizes are `p = 269, 521, 1033`, two curves each.  (2) The
instances are the ladder's (`generate_translation_instance(p, seed, 16)`); every curve drew
`h = 4`.  (3) The small base is the first `⌈cols^{3/4}⌉` `τ_T`-classes in base order, as the
ladder's own DLP arm does, not a uniform draw.  **Prior work found after registration:** the
ladder (`research/ic_next10_20261014/01-fp4-ladder/`, 2026-10-14) had already run DLP at
`k = 4` on the *unfolded* `x`-line base, charged at the `S₄` solve's `C₄ = 1.213·10¹²`: DLP
`1.8`–`4.5×` dearer than plain, crossing it at `#E ≈ 2^{219}`.  E20 adds the `τ_T`-folded `D₄`
base charged at the weighted `C`, which that run did not have.

| p | log₂ r | trials full | trials DLP | DLP / full | `S` full / rho-neg | `S` DLP / rho-neg |
|--:|--:|--:|--:|--:|--:|--:|
| 269 | 30.3 | 10,020 / 6,823 | 16,586 / 11,915 | 1.66 / 1.75 | `4.5·10⁶` / `2.8·10⁶` | `7.4·10⁶` / `4.8·10⁶` |
| 521 | 34.1 | 10,717 / 12,459 | 25,905 / 31,499 | 2.42 / 2.53 | `1.2·10⁶` / `1.3·10⁶` | `2.8·10⁶` / `3.2·10⁶` |
| 1033 | 38.1 | 24,319 / 30,321 | 76,761 / 109,255 | 3.16 / 3.60 | `7.2·10⁵` / `9.8·10⁵` | `2.3·10⁶` / `3.5·10⁶` |

Fitted exponents in `p` (theory in brackets): relation full `0.89` (1), linear algebra full `2.12`
(2), relation DLP `1.39` (3/2), linear algebra DLP `1.89` (3/2), rho `1.95` (2).

| id | result | verdict |
|:--|:--|:--|
| G2.14 | every log verified (18 of 18: three arms × six curves); every walk correct; the null arm (`F' = F`) reproduced the full arm's trials and `S` exactly on all six | **holds** |
| G2.15 | `S_DLP / S_full` = `1.66`–`3.60`, above 1 at every `p`, widening | **holds** |
| G2.16 | targets per full relation DLP / full grows as `p^{0.51}` | **holds** (band `[0.25, 1.0]`) |
| G2.17 | DLP below the full base at `2^{164}` (fitted exponents) / `2^{222}` (theory exponents); DLP below rho at `2^{193}` / `2^{209}` | **holds** (falsifier: either below `2^{160}`) |

**A cross-check that comes for free.**  With the theory exponents imposed, the *full* base crosses
rho at `2^{119}` — `d4-s5`'s `2^{118.9}`, re-derived here from a different harness and different
targets.  With the freely fitted exponents it never does: the full-base linear algebra fits
`p^{2.12}` against rho's `p^{1.95}` over three sizes, which a ±0.1 error in either exponent
explains; the theory exponents (both `2`, E18's constant `r`) are the ones to read.

**What it says.**  Double large primes have the sub-rho exponent at `k = 4` and lose by the
constant: at the weighted `C ≈ 1.4·10⁹` the solve is the whole cost at every size anyone can run,
so shrinking the matrix buys nothing until `n ≈ 2^{164}–2^{222}`, long after the full base has
already crossed rho (`2^{119}`).  Cutting `C` moves both crossovers; that is E21's and E22's
question.  **Class: measurement.**

### 9.10 Amendment to E22 Stage B, before its code: the invariant form, derived

*2026-10-18, additive to §9.7; G2.23's registered band stands.*  On each coordinate the
translations act as `z ↦ −z, 1/z, −1/z`.  Their invariant is `u = z² + z⁻²`; the three
non-trivial characters are carried by `v₁ = z − 1/z` (`v₁² = u − 2`), `v₂ = z + 1/z`
(`v₂² = u + 2`) and `v₁v₂ = z² − z⁻²`.  The decomposition group `V₄⁴_0 ⋊ S₄` admits exactly the
four diagonal characters, so `H / Πz_i⁴` (degree `8` in each `z_i`, i.e. `≤ 2` in each `u_i`) lies
in the 30-dimensional span

  `{e_i e_j : 0 ≤ i ≤ j ≤ 4} ∪ {P₁e_k} ∪ {P₂e_k} ∪ {P₁P₂e_k}`,  `e_k = e_k(u)`,
  `P₁ = Πv₁(z_i)`, `P₂ = Πv₂(z_i)`,

(precompute checks this on fresh points, as `SymmetrisedD4::precompute` does; failure ends Stage B
as a negative).  `Π(u_i ∓ 2) = P_{1,2}²` give `e₄ = P₁² + 2e₃ − 4e₂ + 8e₁ − 16` and
`P₂² − P₁² = 16e₁ + 4e₃`, so `e₁ = (P₂² − P₁² − 4e₃)/16` and `e₄` are eliminated: **four unknowns
`(e₂, e₃, P₁, P₂)`, weights `(2, 2, 1, 1)`, four Weil components of weighted degree `4`.**
Weighted Bézout `4⁴/(2·2·1·1) = 64` (`= 98,304 / 1536`, the multihomogeneous count over the group
order, as `512 = 98,304 / 192` for `D₄`); weighted Macaulay bound `16 − 6 + 1 = 11` (`D₄`: `26`),
`882`-odd columns before row selection (`D₄`: `4,200`), quotient `64` (`D₄`: `512`).  Lift-back:
`u_i` from `T⁴ − e₁T³ + e₂T² − e₃T + e₄`, `z_i` from `z⁴ − u_iz² + 1`, the four `V₄`-choices per
coordinate fixed by `P₁, P₂` and a group check.  **Derived expectation, not a prediction:** with
an eighth of the quotient and a fifth of the columns, `C_V₄` could fall below `C_w / 30`, past
G2.23's band, which would then be falsified on the high side.

### 9.11 E21 measured: trace replay on the weighted solve is a net loss

*2026-10-18.*  `examples/d4_replay.rs --sizes 53,101 --seed 1 --targets 201`, two threads, on the
`τ_T` instances (`generate_translation_instance(p, 1, 4)`); raw
`experiments/28_e21_replay/e21.json` (per target: both costs, rows, degrees, outcome), log
`e21.log`.  The trace is recorded on each curve's first target; 200 more targets are solved in
full and by replay.

| p | agree | fell back | `C` full / replayed, median | total `C` full / replayed | zero rows skipped | pivots reached by the normal forms |
|--:|--:|--:|--:|--:|--:|--:|
| 53 | 200 / 200 | 47 (23.5 %) | 1.016× | 0.951× | 291 of 3,979 | 2,877 of 3,688 |
| 101 | 200 / 200 | 26 (13 %) | 1.021× | 0.980× | 291 of 3,979 | 2,877 of 3,688 |

| id | result | verdict |
|:--|:--|:--|
| G2.18 | identical decomposition sets on all 400 targets | **holds** |
| G2.19 | replayed targets save `6.3 %` of the elimination, `1.02×` of `C` (median); with fallbacks charged the total is `0.95`–`0.98×` | **falsified** (band `1.03`–`1.20`) |
| G2.20 | `13 %` fallbacks at `p = 101` (`23.5 %` at `53`): which rows reduce to zero is not stable across targets of one curve | **falsified** (`≤ 5 %`) |
| G2.21 | `22 %` of the echelon's pivots are never reached by the normal forms — the ceiling on what a reached-pivots-only elimination could save on the elimination (`34 %` of `C`), i.e. `≤ 1.08×` of `C` | measured |

**What it says.**  The weighted solve has little repeated structure to exploit: the rows that
vanish are `7 %` of the matrix and not the same rows from one target to the next, and the
elimination work they carry is small.  Unlike F4's pair selection (§12 of the cover ledger,
`2.44×`), a Macaulay matrix at its bound wastes little.  Direction 2a is closed for this solver;
direction 2c's ceiling is `≤ 1.08×`.  **Class: engineering, negative.**

### 9.12 E22 measured: the `V₄` line exists, its solve is `≈ 820×` cheaper, its yield `≈ 10×` worse

*2026-10-18.*  **Stage A** (`examples/klein_stage_a.rs`, `experiments/28_e22_stage_a/`):
`generate_klein_instance(p, seed, 16)`, `p = 53, 101, 149, 197, 269`, seeds 1–2.  **Stage B**
(`examples/klein_stage_b.rs`, `experiments/28_e22_stage_b/`): the `klein_v4` solve on 6 planted
(signed sums of four base points) and 6 uniform (`aG + bQ`) targets per size, `p = 37, 41, 53,
101, 269`, against brute force (`p ≤ 37`) or the exact meet-in-the-middle oracle.  Two
corrections, stated: Stage A ran with the base before the fixed points `z ∈ {0, ±1, ±i}` were
excluded (they are order-4 points, logarithm `0` in `⟨G⟩`, harmless to the solve and at most five
points), and its `τ_T` comparison column divides all residuals by the folded arm's relations
although its loop runs until the unfolded control also solves — it overstates `τ_T`'s `y`, so
E19 (the same `τ_T` construction, folded arm only, exact oracle) is the comparator below.

**Stage A** (all ten curves):

| p | seed | base (density) | cols (pts/col) | `z(O) ∈ F_p` | residuals | y | decompositions per decomposed target | log | E19 residuals, same p and seed |
|--:|--:|:--|:--|:--|--:|--:|:--|:--|--:|
| 53 | 1 | 32 (0.60) | 8 (4.00) | no | 10,733 | 1,192.6 | 64 (×4), 24 (×5) | yes | 1,498 |
| 53 | 2 | 36 (0.68) | 9 (4.00) | no | 2,086 | 208.6 | — | yes | 1,926 |
| 101 | 1 | 36 (0.36) | 9 (4.00) | no | 55,508 | 5,550.8 | — | yes | 2,608 |
| 101 | 2 | 52 (0.52) | 13 (4.00) | no | 14,840 | 1,060.0 | — | yes | 2,701 |
| 149 | 1 | 64 (0.43) | 16 (4.00) | no | 48,795 | 2,870.3 | 64 (×11), 24 (×5) | yes | 3,525 |
| 149 | 2 | 60 (0.40) | 15 (4.00) | no | 37,444 | 2,340.2 | 64 (×12), 24, 12, 4 | yes | 2,868 |
| 197 | 1 | 76 (0.39) | 19 (4.00) | no | 81,149 | 4,057.4 | 64 (×16), 24 (×4) | yes | — |
| 197 | 2 | 84 (0.43) | 21 (4.00) | no | 68,741 | 3,124.6 | 64 (×18), 24 (×4) | yes | — |
| 269 | 1 | 132 (0.49) | 33 (4.00) | no | 59,115 | 1,738.7 | 64 (×27), 24 (×6), 4 | yes | — |
| 269 | 2 | 148 (0.55) | 37 (4.00) | no | 44,751 | 1,177.7 | 64 (×31), 24 (×7) | yes | — |

**Stage B:**

| p | targets | agree | unsolved | planted agree | uniform solved | `C` median | precompute |
|--:|--:|--:|--:|--:|--:|--:|--:|
| 37 | 12 | 11 | 1 | 5 / 6 | 6 / 6 | `1.59·10⁶` | `1.7·10⁶` |
| 41 | 12 | 10 | 2 | 4 / 6 | 6 / 6 | `1.56·10⁶` | `1.7·10⁶` |
| 53 | 12 | 9 | 3 | 3 / 6 | 6 / 6 | `1.55·10⁶` | `1.7·10⁶` |
| 101 | 12 | 10 | 2 | 4 / 6 | 6 / 6 | `1.65·10⁶` | `1.7·10⁶` |
| 269 | 12 | 11 | 1 | 5 / 6 | 6 / 6 | `1.72·10⁶` | `1.7·10⁶` |

The precompute's span check passed on every curve: `H` is invariant under `V₄⁴_0 ⋊ S₄` and lies in
the 30-function span.  Every solve that solved returned exactly the true decomposition set
(64 or 128 sorted quadruples); the quotient is exactly `64` on every one, as derived.  **All nine
failures are planted targets of one kind**: quotient `66`, "border unreachable" at degree 11,
true decomposition counts not divisible by 64 (`56`–`752`).  Derived: a planted sum containing two
points of one translation orbit has `P_a + P_b = T_i`, so `R − T_i = P_c + P_d` and `R` has the
one-parameter family `{P, T_i − P, P_c, P_d}` of decompositions — a positive-dimensional system no
degree resolves.  A uniform target `R ∈ ⟨G⟩` meets this only if `R − T_i` has a two-point
decomposition (probability `≈ cols²/n`); all 30 uniform targets solved.

| id | result | verdict |
|:--|:--|:--|
| G2.22 | curves exist at every registered `p`; the pole is never in `F_p`; every fold checked on points as a translation (4 points per column); every log recovered; density `0.36`–`0.68`, pooled `0.46` | **holds** (density: `≈ 1/2` within the orbit-correlated spread, 4 points share one liftability) |
| G2.23 | `C_w / C_V₄ ≈ 1.40·10⁹ / 1.7·10⁶ ≈ 820` | **falsified high** (band `[4, 30]`; §9.10 anticipated it) |
| G2.24 | decompositions per decomposed target `64` (`24` and smaller on orbits with coincidences) — holds; `y_V₄ / y_E19` at matched `p`: `1.5`–`51`, median `≈ 13`; total targets `1.1`–`21×`, median `≈ 10×` | **half falsified** (`y` ratio band `[3, 10]`): the yield penalty is larger than the orbit factor alone, the lower base density (`≈ 0.45`, a fourth power in the decomposition count) adding the rest |
| G2.25 | net on the relation phase: `≈ 820 / 10 ≈ 80×` cheaper per relation than E19, *derived from per-call and per-target measurements in separate harnesses* | a gain, **not yet measured end to end** — E24 below |

### 9.13 E24 pre-registration: the `k = 4` index calculus end to end on the `V₄` line

*Registered 2026-10-18, after Stage A and Stage B above and before any E24 code.*  The same
protocol as E19 (§9.5), with the `V₄` line and solve: on `generate_klein_instance(p, seed, 16)`,
uniform targets `R = aG + bQ`, each decomposed by `klein_v4::v4_system` → `solve_weighted_traced`
(no trace) at weighted degree 11 → `lift_back_v4`; every lifted quadruple mapped to base points
with exact signs by a charged group check; one decomposition per target drawn uniformly as a
`V₄`-folded row; Wiedemann until `[d]G = Q` verifies; every phase charged in `F_p`
multiplications (precompute, base, generation, Weil restriction, solves, lift-back, checks,
linear algebra at `16` per mod-`n` multiplication); `16` negation walks and `16` unfolded walks
on the same curve; unpriced native work at `0, 0.1, 1` GAE (Macaulay entries; one per walk step).
Sizes `p = 101, 149, 197, 269, 397, 521`, two curves each.

| id | quantity | prediction | falsified by |
|:--|:--|:--|:--|
| G2.27 | correctness | every log recovered; every row re-added in the group; no lifted quadruple without an exact sign vector | any failure |
| G2.28 | the solve in the pipeline | per-call `C` median in `[1.3, 2.2]·10⁶` at every `p`; unsolved `≤ 1 %` of targets | outside, or more |
| G2.29 | end to end against E19 | `S_V₄ / rho` below E19's `S / rho` at matched `p` (`101`–`269`) by a factor in `[20, 300]` | outside the band |
| G2.30 | slope | fitted exponent of `S / rho` in `n` in `[−0.35, −0.15]`; the solves `≥ 90 %` of `S` | outside, or less |
| G2.31 | the handover, from E24's own constants (§11.16's model, the theory exponents) | `n* ∈ [2^{86}, 2^{98}]`, against E19's own `2^{116.7}` | outside |

**What it would say.**  If G2.27–G2.31 hold, the `k = 4` baseline moves by two orders of magnitude
at every measured size and its modelled handover by `≈ 2^{20}`–`2^{30}` in `n`: a verified,
measured improvement of the index-calculus baseline under the 2026-10-15 success criterion
(**class: engineering** — a constant; the exponents are unchanged), and the rows stay
`10⁴`–`10⁵×` slower than rho: the distance remaining, reported as such.

### 9.14 E24 measured: the `k = 4` baseline moves by `35`–`240×`, and its handover from `2^{117}` to `2^{90}`

*2026-10-18.*  `experiments/29_quartic_pipeline_v4/run_all.sh` (one curve per invocation,
`--rho-runs 16`), binary `854acdbc6`; raw `p{101,149,197,269,397,521}s{1,2}.json`, log `run.log`,
tables `tables.md` (`glv_invariant_experiment_tables`, `e24`).  Host: the 4-vCPU container, shared
with E19, E20 and E22 runs (operation counts only; no wall-time claim).

| p | seed | log₂ n | base / cols | targets | y | unsolved | `C` median | solves share | `S` | rho (negation) | `S` / rho |
|--:|--:|--:|:--|--:|--:|--:|--:|--:|--:|--:|--:|
| 101 | 1 | 24.6 | 36 / 9 | 77,960 | 7,796 | 9 | `1.62·10⁶` | 99.5 % | `1.28·10¹¹` | `7.6·10⁵` | `1.69·10⁵` |
| 101 | 2 | 24.6 | 52 / 13 | 15,387 | 1,099 | 5 | `1.62·10⁶` | 99.5 % | `2.55·10¹⁰` | `7.4·10⁵` | `3.43·10⁴` |
| 149 | 1 | 26.9 | 64 / 16 | 31,225 | 1,837 | 4 | `1.63·10⁶` | 99.5 % | `5.20·10¹⁰` | `1.37·10⁶` | `3.80·10⁴` |
| 149 | 2 | 26.9 | 60 / 15 | 45,343 | 2,834 | 2 | `1.63·10⁶` | 99.5 % | `7.51·10¹⁰` | `1.36·10⁶` | `5.54·10⁴` |
| 197 | 1 | 28.5 | 76 / 19 | 142,361 | 7,118 | 10 | `1.65·10⁶` | 99.4 % | `2.37·10¹¹` | `1.70·10⁶` | `1.40·10⁵` |
| 197 | 2 | 28.5 | 84 / 21 | 74,345 | 3,379 | 7 | `1.64·10⁶` | 99.4 % | `1.24·10¹¹` | `2.27·10⁶` | `5.44·10⁴` |
| 269 | 1 | 30.3 | 132 / 33 | 44,281 | 1,302 | 0 | `1.65·10⁶` | 99.4 % | `7.38·10¹⁰` | `3.21·10⁶` | `2.30·10⁴` |
| 269 | 2 | 30.3 | 148 / 37 | 50,040 | 1,317 | 1 | `1.65·10⁶` | 99.4 % | `8.37·10¹⁰` | `3.58·10⁶` | `2.34·10⁴` |
| 397 | 1 | 32.5 | 208 / 52 | 66,981 | 1,264 | 1 | `1.66·10⁶` | 99.3 % | `1.13·10¹¹` | `8.61·10⁶` | `1.31·10⁴` |
| 397 | 2 | 32.5 | 188 / 47 | 89,424 | 1,863 | 2 | `1.66·10⁶` | 99.3 % | `1.50·10¹¹` | `9.48·10⁶` | `1.58·10⁴` |
| 521 | 1 | 34.1 | 240 / 60 | 131,952 | 2,163 | 1 | `1.66·10⁶` | 99.3 % | `2.21·10¹¹` | `1.18·10⁷` | `1.88·10⁴` |
| 521 | 2 | 34.1 | 232 / 58 | 142,487 | 2,415 | 1 | `1.66·10⁶` | 99.3 % | `2.39·10¹¹` | `1.11·10⁷` | `2.15·10⁴` |

All `h = 4`.  Every log recovered; every one of the 381 kept rows re-added to `O` in the group;
no lifted quadruple off the base or without an exact sign vector; all 384 walks correct.  The
unsolved targets (`0`–`10` per curve, `≤ 0.012 %`) all end "border unreachable" — the same
signature as Stage B's planted failures — and are charged.  Fitted exponent of `S / rho` in `n`:
`−0.25`.  Handover by §11.16's model from E24's own constants (`y = 2,326` pooled,
`cols / p = 0.114`, `C = 1.66·10⁶`, `S_rho = 110.9`, `r = 0.072`): **`n* = 2^{90.1}`** (from
`p = 521` alone, where rho's set-up weighs least, `S_rho = 84.2`, `r = 0.090`: `2^{91.7}`).
Unpriced native work (one GAE per Macaulay entry; one per walk step) at `0, 0.1, 1` GAE raises
`S / rho` by up to `7×` at `1` GAE and never reverses the order: rho is ahead on every row
across the band.

**Against E19 at matched `p`** (E19's `τ_T`/`D₄` pipeline, same unit, same walks; different curves
of the same `p`): `p = 101`: `6.33·10⁶`, `5.85·10⁶` → `37×`, `171×`; `p = 149`: `4.09·10⁶`,
`3.40·10⁶` → `108×`, `61×`; `p = 197`: `4.91·10⁶`, `2.83·10⁶` → `35×`, `52×`; `p = 269`:
`5.51·10⁶`, `2.57·10⁶` → `240×`, `110×` (pairing by seed; any pairing stays in `[20, 240]`).

| id | result | verdict |
|:--|:--|:--|
| G2.27 | every log, row and quadruple verified | **holds** |
| G2.28 | `C` median `1.62`–`1.66·10⁶`; unsolved `≤ 0.012 %` | **holds** |
| G2.29 | below E19 at every matched `p`, by seed: `37×`, `171×` (`101`), `108×`, `61×` (`149`), `35×`, `52×` (`197`), `240×`, `110×` (`269`); by size (means) `60×, 80×, 40×, 174×` | **holds** (band `[20, 300]`) |
| G2.30 | slope `−0.25`; solves `99.3`–`99.5 %` | **holds** |
| G2.31 | `n* = 2^{90.1}` (`2^{91.7}` from the largest `p`) | **holds** (band `[2^{86}, 2^{98}]`; E19's frame `2^{119}`) |

**What it says.**  The full rational 2-torsion symmetry, on a base line it keeps, cuts one `k = 4`
decomposition from `1.4·10⁹` to `1.66·10⁶` `F_p` multiplications (quotient `512 → 64`, Macaulay
degree `26 → 11`), and pays `≈ 10×` in targets per relation for the larger translation orbits and
the thinner base.  Net, measured end to end: **the `k = 4` index-calculus baseline is `35`–`240×`
cheaper at every size measured, and the modelled handover with rho moves from `2^{116.7}` (E19, §9.6) to
`2^{90}`.**  The exponents are unchanged (`S / rho ∝ n^{−1/4}`, fitted `−0.25`): **class:
engineering**, a constant — and under the 2026-10-15 success criterion a verified, measured
improvement of the baseline.  **Distance remaining:** every row is still `1.3·10⁴`–`1.7·10⁵×`
slower than the matched rho (`1.3·10⁴×` at `p = 397`, the best), and the handover is a model
extrapolation of measured constants with the theory exponents, not a measured crossing.  **Scope:**
curves over `F_{p⁴}` with full rational 2-torsion and rational halves of two of the `T`'s (the
construction's condition), `h = 4`, `p ≤ 521` (`n ≤ 2^{34.1}`); a curve without rational 2-torsion,
such as a prime-order curve, has no such symmetry.

### 9.15 E25 pre-registration: the GLS endomorphism on the `V₄` line

*Registered 2026-10-18, after E24 and before any E25 code.*  Owner request: pursue GLS-enabled
index calculus.  E18's `ψ` was the `j = 0` automorphism of order 3 on prime-order curves, which
have no rational 2-torsion, and E23 closed `ψ` with `τ_T` on `j = 0`.  The GLS endomorphism is a
different map on a different family, and nothing below rules it out on the `V₄` line.  Four facts,
derived before code:

1. **The curves.**  Take `e₂ ∈ F_{p⁴} ∖ F_{p²}`, `e₃ = e₂^{p²}`, `e₁ = −e₂ − e₃ ∈ F_{p²}`, so
   `E₀ : y² = Π(x − e_i)` is defined over `F_{p²}` with one root there.  With `μ` a non-square of
   `F_{p⁴}`, `E : y² = Π(x − μe_i) = x³ + μ²a x + μ³b` is the quadratic twist of `E₀` over
   `F_{p⁴}`, of order `(p² − 1)² + t²` (`t` the trace of `E₀/F_{p²}`), with the GLS endomorphism
   `ψ(x, y) = (μ^{1−p²} x^{p²}, μ^{3(1−p²)/2} y^{p²})`, `ψ² = −1`, so `ψ = [λ]` on `⟨G⟩` with
   `λ² ≡ −1 (mod n)`.  All of `E[2]` is rational over `F_{p⁴}`; `ψ` fixes `T₁` and swaps `T₂, T₃`.
   E24's condition, rational abscissae of the halves of `T₁` and `T₂`, reads: `f′(x₁) =
   μ²·N(e₁ − e₂)` is a square always, and `f′(x₂)` with probability `1/2`.  E24's generator
   excluded `j ∈ F_{p²}`; these curves are exactly what it excluded.
2. **`ψ` keeps the line and acts on it as `z ↦ ±i/z`.**  For `z ∈ F_p`, `z(ψP) = h(z(P))` with
   `h = g⁻¹ ∘ [μ^{1−p²}] ∘ g^{σ}` a Möbius map (`σ` the `p²`-power).  `ψ(P + T) = ψP + ψT`, so `h`
   normalises `V₄ = ⟨z ↦ −z, z ↦ 1/z⟩`, commutes with `z ↦ −z` (`T₁` fixed) and swaps `1/z` with
   `−1/z` (`T₂ ↔ T₃`): `h ∈ {±iz, ±i/z}`.  `ψ² = −1` fixes every abscissa, so `h² = id` and
   `h = ±i/z`.  With `p ≡ 1 (mod 4)`, `i ∈ F_p` and `h` keeps `z ∈ F_p`: the fold group on the line
   is `⟨−z, 1/z, i/z⟩ ≅ D₄` of order `8`, so **`8` points a column against E24's `4`**.  The points
   with `z⁴ = −1` are fixed by `±i/z`, so `ψP = ±P` and their `⟨G⟩`-component is `0`; they are
   excluded with `z ∈ {0, ±1, ±i}`.
3. **The base is not confined.**  In `z` the curve is `v² = κ(z⁴ + νz² + 1)` with `ν = −(w² + w⁻²)`
   and `w = z(O)`.  `ψ(O) = O` gives `w·w^{σ} = ±i`, hence `ν^{σ} = −ν`: `ν ∉ F_{p²}` unless
   `ν = 0`.  Were `ν ∈ F_{p²}`, every `z ∈ F_p` would lift or none would, and the base would lie in
   one coset of a group of order `≈ p²` (E22's fact 1 in another form).  So the density should be
   `≈ 1/2`, as on E24's curves.
4. **What it buys, and what it does not.**  `ψ` acts on the base, not on the decomposition system
   (§8.0): decompositions per target and targets per relation `y` are unchanged, the columns halve,
   so the relation phase (`99.3 %` of `S` in E24) halves and the linear algebra quarters.  The
   matched rho on these curves is the `⟨±ψ⟩` walk, `√2` fewer steps than the negation walk (E9
   measured `1.40` on `F_{p²}` GLS twists).  **Net against the matched rho: `√2`**, a constant.
   In §11.16's model `p* ∝ (cols/p) / S_rho`: `cols/p ÷ 2`, `S_rho ÷ √2`, so `p* ÷ √2` and
   `n* = p*⁴/h ÷ 4`, two bits.

**Protocol.**  `generate_gls_klein_instance(p, seed, 16)`; on each curve two arms on one target
stream (same seed, uniform `R = aG + bQ`, the E24 oracle unchanged): **A**, E24's `V₄`-folded
base (the control, `4` points a column), and **B**, the same base folded by `⟨V₄, ψ⟩` (`8` a
column, coefficients `±λ^e` each checked on points).  Each runs until `[d]G = Q` verifies, every
phase charged as in E24.  On each curve `16` `⟨±ψ⟩` walks, `16` negation walks and `16` unfolded
walks.  Sizes `p = 101, 149, 197, 269, 397`, two curves each where time allows (at least one).
Unpriced native work at `0, 0.1, 1` GAE as in E24.

| id | quantity | prediction | falsified by |
|:--|:--|:--|:--|
| G2.32 | Stage A (gate) | at every `p` a curve exists with `h ≤ 16`; `ψ² = −1` and `ψ(G) = [λ]G`, `λ² ≡ −1`, on points; every base point's `ψ`-image is in the base with `z ↦ ±i/z`; points per column `≥ 7.8`; density in `[0.3, 0.7]`; fold coefficients equal true logarithms (unit test); a log recovered | any (then E25 stops here, recorded as a negative) |
| G2.33 | columns | `cols_A / cols_B` in `[1.9, 2.0]` | outside |
| G2.34 | yield | `y_B / y_A` in `[0.8, 1.25]` per curve | outside |
| G2.35 | end to end, same curve | `S_A / S_B` in `[1.6, 2.4]` on every curve | outside |
| G2.36 | against the matched rho | `⟨±ψ⟩` / negation steps, mean over curves, in `[1.2, 1.6]`; `(S_A / rho_neg) / (S_B / rho_ψ)` in `[1.1, 2.0]` on every curve | outside |
| G2.37 | handover, from arm B's own constants (§11.16's model) | `n* ∈ [2^{84}, 2^{92}]` (E24's own `2^{90.1}`, less two bits, with curve scatter) | outside |

Against E24's rows at matched `p` the comparison crosses curve families (and E24's per-curve `y`
spans `1,099`–`7,796`), so it is reported, not gated.  **Class if G2.32–G2.37 hold:** engineering
(a constant, `√2` against the matched rho), on GLS twists over `F_{p⁴}` with the split 2-torsion
above only.  The exponents do not move.

### 9.16 E25 measured: the GLS fold halves the `k = 4` cost on its own curves, a constant

*2026-10-18.*  **Stage A:** `examples/gls_stage_a.rs --sizes 29,37,41,53,61,89,97,101,109,113,149,197,269,397
--seeds 4` (`experiments/30_gls_v4/stage_a.{json,log}`).  **End to end:**
`experiments/30_gls_v4/run_all.sh` (one curve per invocation, both arms, `--rho-runs 16`), binary
`46202dc97`; raw `p{101,149,197,269,397}s{1,2}.json`, log `run.log`, tables `tables.md`
(`glv_invariant_experiment_tables`, `e25`).  Host: the 4-vCPU container (operation counts only; no
wall-time claim).

**Stage A (56 curves).**  Every `(p, seed)` gave a curve; `ψ² = −1`, `ψ(G) = [λ]G` with `λ² ≡ −1`,
and `ψ` fixing `T₁` and swapping `T₂, T₃` were checked on points on every one.  Every non-empty
base has exactly `8.00` points a column under `⟨V₄, ψ⟩` (`4.00` under `V₄`), and the two arms hold
the same points.  `ν² ∉ F_p` on all 56 (fact 3), and wherever logarithms were computed (`n ≤ 2^{26}`,
`p ≤ 149`) the number of distinct column logarithms equals the number of columns: no base is
confined.  Density pooled over all 56: `0.49` (`0.52` at `p ≥ 149`); per curve `0`–`0.75`, 14 of 56
outside `[0.3, 0.7]`, all but one at `p ≤ 113`, where a base is `3`–`13` orbits of `8` points that
lift together (two `p = 41` curves have no base at all).  **Two facts the derivation did not
predict:** every curve has `h = 16`, never `4` (the construction forces `4 | t`; E24's curves had
`h = 4`), and the unit test confirms the fold coefficients against true logarithms at `p = 37, 41`.

**End to end (10 curves, both arms, `h = 16` throughout).**  Every logarithm recovered on both
arms; every kept row re-added to `O`; no lifted quadruple off the base or without a sign vector;
all `960` walks correct; `198` unsolved of `1.15·10⁶` targets (`0.017 %`, "border unreachable",
charged).  `C` median `1.62`–`1.66·10⁶` on every arm.

| p | seed | log₂ n | cols A / B | y A | y B | `S_A` | `S_B` | `S_A / S_B` | neg / `±ψ` steps | `S_A` / rho neg | `S_B` / rho `±ψ` | improvement |
|--:|--:|--:|:--|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 101 | 1 | 22.6 | 10 / 5 | 3,731 | 3,528 | `6.76·10¹⁰` | `3.48·10¹⁰` | 1.94 | 1.03 | `1.53·10⁵` | `6.77·10⁴` | 2.26 |
| 101 | 2 | 22.6 | 14 / 7 | 1,001 | 888 | `2.47·10¹⁰` | `1.17·10¹⁰` | 2.11 | 1.49 | `6.02·10⁴` | `2.93·10⁴` | 2.05 |
| 149 | 1 | 24.9 | 10 / 5 | 32,013 | 22,006 | `5.83·10¹¹` | `2.19·10¹¹` | 2.67 | 1.09 | `8.31·10⁵` | `2.65·10⁵` | 3.13 |
| 149 | 2 | 24.9 | 14 / 7 | 4,039 | 4,272 | `1.01·10¹¹` | `5.67·10¹⁰` | 1.77 | 0.96 | `1.58·10⁵` | `7.01·10⁴` | 2.25 |
| 197 | 1 | 26.5 | 22 / 11 | 2,218 | 3,150 | `8.53·10¹⁰` | `6.32·10¹⁰` | 1.35 | 1.52 | `7.71·10⁴` | `6.11·10⁴` | 1.26 |
| 197 | 2 | 26.5 | 30 / 15 | 846 | 969 | `4.39·10¹⁰` | `2.59·10¹⁰` | 1.69 | 1.62 | `3.97·10⁴` | `2.62·10⁴` | 1.51 |
| 269 | 1 | 28.3 | 30 / 15 | 2,645 | 2,950 | `1.37·10¹¹` | `7.88·10¹⁰` | 1.74 | 1.61 | `7.45·10⁴` | `4.84·10⁴` | 1.54 |
| 269 | 2 | 28.3 | 40 / 20 | 744 | 718 | `5.10·10¹⁰` | `2.52·10¹⁰` | 2.02 | 2.08 | `2.24·10⁴` | `1.56·10⁴` | 1.44 |
| 397 | 1 | 30.5 | 60 / 30 | 700 | 800 | `7.19·10¹⁰` | `4.17·10¹⁰` | 1.72 | 1.36 | `1.99·10⁴` | `1.13·10⁴` | 1.76 |
| 397 | 2 | 30.5 | 48 / 24 | 1,600 | 1,349 | `1.31·10¹¹` | `5.65·10¹⁰` | 2.32 | 1.67 | `3.38·10⁴` | `1.71·10⁴` | 1.97 |

"Improvement" is `(S_A / rho_neg) / (S_B / rho_±ψ)`: the ψ arm against its matched walk over the
control against the negation walk, on one curve.  Handover by §11.16's model from arm B's own
constants (`y = 2,474` pooled, `cols / p = 0.060`, `C = 1.66·10⁶`, `h = 16`, `S_rho = 121.7` for the
`±ψ` walk, `r = 0.045`): **`n* = 2^{88.0}`** (from `p = 397` alone, `2^{85.7}`); arm A on the same
curves and against the same walk: `2^{93.2}` (`2^{91.1}`).  Fitted exponent of `S / rho` in `n`:
`−0.29` (B), `−0.30` (A).  Unpriced native work at `0, 0.1, 1` GAE never reverses the order: rho is
ahead on every row of both arms across the band.

| id | result | verdict |
|:--|:--|:--|
| G2.32 | curves at every `p`; `ψ` verified on points; `ψ`-image of every base point in the base; `8.00` points a column; coefficients equal true logarithms; logs recovered; density `0.49` pooled, `14` of `56` curves outside `[0.3, 0.7]` (all small `p` but one) | **holds** on everything but per-curve density, which the band did not specify; pooled it holds |
| G2.33 | `cols_A / cols_B = 2.00` on every curve | **holds** |
| G2.34 | `y_B / y_A` `0.69`–`1.42` (two curves outside `[0.8, 1.25]`); pooled `2,474 / 2,707 = 0.91` | **falsified** per curve: with `5`–`60` relations a curve, `y` is a ratio of small counts and the band was narrower than its noise |
| G2.35 | `S_A / S_B` `1.35`–`2.67`, median `1.86` (the same two curves outside `[1.6, 2.4]`, tracking their `y`) | **falsified** per curve (8 of 10 in band) |
| G2.36 | steps ratio mean `1.44` (in `[1.2, 1.6]`); improvement `1.26`–`3.13`, median `1.87`: in `[1.1, 2.0]` on all six curves at `p ≥ 197`, above it on all four at `p ≤ 149` | steps **hold**; improvement **falsified high** at `p ≤ 149`: below `n ≈ 2^{25}` a walk is `≤ 3,000` steps, its set-up dominates and the `±ψ` walk costs more multiplications than the negation walk (`5.1` against `4.4·10⁵` at `p = 101` seed 1), which flatters arm B's ratio |
| G2.37 | `n* = 2^{88.0}` (`2^{85.7}` from the largest `p`) | **holds** (band `[2^{84}, 2^{92}]`) |

**What it says.**  On GLS twists over `F_{p⁴}` with the split 2-torsion of §9.15, `ψ` keeps E24's
`V₄` line and acts on it as `z ↦ ±i/z`, so the base folds by `⟨V₄, ψ⟩` with `8` points a column.
Measured end to end on the same curve, base and target stream, **the fold cuts the `k = 4` index
calculus by `1.35`–`2.67×` (median `1.86×`), and against the matched `⟨±ψ⟩` walk the ψ arm gains
`1.26`–`1.97×` over the control at every `p ≥ 197`**; the modelled handover on these curves moves
from `2^{93.2}` to `2^{88.0}`.  **Class: engineering**, a constant: the exponent is unchanged
(`−0.29` against `−0.30`), and the per-curve spread is the relation count's (`5`–`60` relations).
Under the 2026-10-15 criterion this is a verified, measured improvement of the baseline *on this
family*.  **What it does not say.**  It does not move E24's rows: these curves have `h = 16`, which
makes rho `2×` cheaper at a given `p` than on E24's `h = 4` curves and absorbs most of the fold at
matched `n` (arm B's handover `2^{88.0}` against E24's own `2^{90.1}`, two bits, from different
curve families).  **Distance remaining:** every row is still `1.1·10⁴`–`2.7·10⁵×` slower than the
matched rho (`1.13·10⁴×` at `p = 397` seed 1, the best), and the handover is a model extrapolation,
not a measured crossing.  **Scope:** GLS twists over `F_{p⁴}` of curves over `F_{p²}` with one root
there, the halves condition of E24, `p ≡ 1 (mod 4)`, `p ≤ 397` (`n ≤ 2^{30.5}`).  The next lever on
this family is the cofactor, not the fold: a GLS construction with the `V₄` line and `h = 4` would
keep the fold and give back the `2×` that `h = 16` costs against rho.

### 9.17 E26 pre-registration: can a GLS curve carry the `V₄` line with `h = 4`?

*Registered 2026-10-18, after E25 and before any E26 code.*  Owner request: try a GLS construction
with `h = 4`, the lever §9.16 named.  A derivation says the `V₄` line cannot have one; E26 checks it
on curves.  Notation as §9.15: `q = p²`, `E₀ / F_q` with one root `e₁ ∈ F_q`, `E` its quadratic twist
over `F_{q²}`, `ψ² = −1`, `#E = (q − 1)² + t²`.

1. **The cofactor is set by `t mod 4`.**  `ψ` satisfies `ψ² − tψ + q = 0` and `ψ² = −1` on
   `E(F_{q²})`, so `α = (q − 1) − tψ` kills `E(F_{q²})`, a `Z[ψ] ≅ Z[i]`-module of order `N(α)`.  With
   `8 | q − 1`, the `(1 + i)`-adic valuation of `α` is `2` when `t ≡ 2 (mod 4)` (2-part `Z[i]/(2) ≅
   (Z/2)²`, `h = 4` possible) and `≥ 4` when `4 | t` (2-part of order `≥ 16`).  `h = 8` cannot occur.
2. **`t ≡ 2 (mod 4)` rules out the `V₄` line.**  The line needs the halves of `T₁` and `T₂` to have
   abscissae in `F_{q²}` (they are `g(0), g(∞), g(±1)`, and `g` is defined over `F_{q²}` whenever `z ∈ F_p`
   gives rational `x`).  A half of `T_j` has a rational abscissa iff `T_j ∈ 2E(F_{q²})` or
   `T_j ∈ 2E₀(F_{q²})`.  When `h = 4`, `E(F_{q²})` has no point of order 4, so both `T₁` and `T₂` must be
   halved in `E₀(F_{q²})`, which with `E₀[2] ⊂ E₀(F_{q²})` puts all of `E₀[4]` in `E₀(F_{q²})`: the
   `q`-Frobenius `π` of `E₀` satisfies `π² ≡ 1 (mod 4)` on `E₀[4]`.  But `π` swaps `T₂, T₃` (`π ≡
   [[0,1],[1,0]] (mod 2)`) and `det π = q ≡ 1 (mod 4)`; writing `π = [[2a, 1+2b], [1+2c, 2d]]`, the
   determinant forces `b + c` odd and then `(π²)₁₁ ≡ 1 + 2(b + c) ≡ 3 (mod 4)`.  Contradiction.
3. **The other `ψ`-actions do not escape.**  If `ψ` fixes all three `T`'s (`E₀` with full 2-torsion over
   `F_q`), §9.15 fact 2 gives `h ∈ V₄` and a confined line.  If it fixes none, `E[2]` is not rational
   over `F_{q²}`.  So **on GLS twists the `V₄` line forces `h ≥ 16`**; E25's `h = 16` is the minimum, not
   a sampling accident.

**E26 (an exact census, no cost measured).**  `gls_cofactor_census(p, seed)`: E25's generator without
the halves condition (`f′(x₂)` a square) and without a cofactor bound; for each curve record `t mod 4`,
`v₂(#E)`, whether `f′(x₂)` is a square in `F_{q²}`, and whether `E₀(F_q)` has a point of order 4.  Sizes
`p ≡ 1 (mod 4)` from `29` to `397`, `40` curves each.

| id | quantity | prediction | falsified by |
|:--|:--|:--|:--|
| G2.38 | `t ≡ 2 (mod 4)` curves exist, each with `v₂(#E) = 2` and a point of order 4 in `E₀(F_q)` | about half of all curves | none at some `p`, or any with `v₂(#E) ≠ 2` |
| G2.39 | the obstruction | `f′(x₂)` a non-square on **every** `t ≡ 2 (mod 4)` curve | any `t ≡ 2 (mod 4)` curve with `f′(x₂)` a square (then a GLS `V₄` curve with `h = 4` exists, and E25 is re-run on it) |
| G2.40 | the converse | every curve with `f′(x₂)` a square has `4 | t` and `v₂(#E) ≥ 4` | any exception |

**Class:** accounting (an exact statement about a family; no algorithm changes).  If G2.39 holds, the
GLS lever on the `V₄` line is closed at `h = 16`, and what is left is §9.16's measured `√2`-class
constant against the matched walk.

### 9.18 E26 measured: no GLS curve carries the `V₄` line with `h = 4`

*2026-10-18.*  `examples/gls_cofactor_census.rs --sizes 29,37,41,53,61,89,97,101,109,113,149,197,269,397
--seeds 40` (`experiments/31_gls_cofactor/census.{json,log}`), 560 GLS twists, every order computed
and checked to be `(p² − 1)² + t²`.

| class | curves | `v₂(#E)` | `E₀(F_{p²})` has a point of order 4 | halves of `T₂` have rational abscissae |
|:--|--:|:--|:--|:--|
| `t ≡ 2 (mod 4)` | 310 | `2` on all 310 | yes, on all 310 | **no, on all 310** |
| `4 | t` | 250 | `4` (131), `6` (60), `7` (45), `≥ 8` (14) | no, on all 250 | yes, on all 250 |

At every one of the 14 sizes both classes occur (`t ≡ 2 (mod 4)`: `18`–`28` of `40`).

| id | result | verdict |
|:--|:--|:--|
| G2.38 | `t ≡ 2 (mod 4)` on `310 / 560`, at every `p`; `v₂(#E) = 2` and an order-4 point in `E₀(F_{p²})` on every one | **holds** |
| G2.39 | the halves condition fails on all `310` | **holds**: no GLS twist of this shape has the `V₄` line with `h = 4` |
| G2.40 | the halves condition holds on all `250` with `4 | t`, each with `v₂(#E) ≥ 4` | **holds** |

**What it says.**  The census is sharper than the derivation: on these twists the three conditions
coincide exactly.  `4 | t` holds iff the halves of `T₂` have rational abscissae iff `E₀(F_{p²})` has
no point of order 4, and each fails together with the others.  So E25's `h = 16` is the smallest
cofactor any GLS twist with the `V₄` line can have, as §9.17 derived.  **Class: accounting**, an
exact statement about the family.  The GLS lever on the `V₄` line ends at §9.16's measured constant
(median `1.86×` on the same curve, `1.26`–`1.97×` against the matched walk at `p ≥ 197`), with the
`h = 16` penalty that absorbs most of it at matched `n`.  **What remains open:** the `ψ` fold without
the full `V₄` symmetry (E19's single-`τ_T` line, where `h = 4` is not excluded by this argument but
the solve costs `≈ 820×` more per call), and GLS-type endomorphisms of higher order (quartic or sextic
twists of `j = 1728` and `j = 0`), which act on the 2-torsion differently and are not covered here.

### 9.19 Errata found by the validation gate (§9.6, §9.14, §9.16)

*2026-10-21.*  The validation gate (`docs/VALIDATION_GATE.md`) had an independent validator replay
every recorded run of E19, E24 and E25 and recompute the claimed numbers from the raw JSON
(`benchmarks/20261018-k4-v4-line/` and `benchmarks/20261018-k4-gls-v4/`: `replay-certificate.json`,
`validator-report.json`, `validate.py`).  Every run replayed field-identical (wall time aside)
and every logarithm was confirmed; no result, verdict or ranking changes.  The note's text had
seven errors, corrected here.  The sections above are left as written; where they differ, this
section is the record.

| where | written | raw data gives | effect |
|:--|:--|:--|:--|
| §9.14 table, `p = 521` seed 1, matched rho | `1.18·10⁷` | `1.1749·10⁷` → **`1.17·10⁷`** | none (the ratio column, `1.88·10⁴`, was computed from the unrounded value) |
| §9.14 and G2.29, E19 / E24 at `p = 101` seed 2 | `171×` | **`170×`** (`170.4`); `171` came from dividing the rounded `5.85·10⁶` by `3.43·10⁴` | none: the range stays `35`–`240×`, G2.29 holds |
| §9.14, kept rows re-added to `O` | `381` | **`392`** (all twelve curves; `0` failures) | none |
| §9.14 and G2.28, unsolved targets per curve | `≤ 0.012 %` | **`≤ 0.033 %`**: `5 / 15,387` at `p = 101` seed 2 and `0.0128 %` at `p = 149` seed 1; `0.0047 %` pooled | none: G2.28 registered `≤ 1 %` and holds |
| §9.6 G2.12, `p = 53` seed 1, measured / registered | `0.495` | **`0.494`** (`0.49448`, with the registered `C = 1.3·10⁹`) | none: still a hair outside `×2`; per size holds |
| §9.6 G2.12, rho's `S` per `√n` (explanatory) | "`242` at `p = 53` against `92` at `p = 269`" | **`243`** against **`92`**, both the mean of the two curves (seed 1 alone: `241.9` and `87.5`); the written pair mixed a single curve with a mean | none: the explanation stands |
| §9.16 G2.36, why the improvement exceeds the band at `p ≤ 149` (explanatory) | "below `n ≈ 2^{25}` a walk is `≤ 3,000` steps" | false at `p = 149` (`n ≈ 2^{24.9}`): the `⟨±ψ⟩` walk averages **`4,057`** and **`3,965`** steps; at `p = 101` (`n ≈ 2^{22.6}`) it averages `2,007` and `1,192` | the mechanism stands (walk set-up and the `±ψ` step's extra multiplications dominate at these sizes), but the `3,000`-step threshold is wrong; the measured improvement and the "falsified high" verdict are unchanged |

The same `171` and `0.495` appear in `research/glv_k4_pipeline_d4_20261017/REPORT.typ`, which now
carries an errata section with these values; its PDF is rebuilt there and in
`benchmarks/20261018-k4-v4-line/REPORT.pdf`.

The validator also recorded these limitations of the evidence, which the earlier sections did not
state:

- **Thread count.**  Each round draws `2 ×` (worker threads) targets from one seeded stream, so a
  replay must fix `RAYON_NUM_THREADS=4`, the recording host's count.  Another thread count gives
  different, still valid, runs.
- **`⟨±ψ⟩` walk overhead.**  The walk applies `ψ` to both coordinates on every step (`152`–`154`
  against `109` multiplications per step at `p = 397`).  Comparing abscissae first would save about
  `6 %` (the validator's estimate, not measured), so E25's distance to rho leans about that much in
  the index calculus's favour.
- **Partial pairing in E25.**  Arms A and B share the target stream only until B's first
  linear-algebra attempt.  This adds noise, not bias, to the per-curve `S_A / S_B`.
- **Price band.**  The `35`–`240×` E19-to-E24 factor holds at `0` GAE per unpriced native
  operation.  Pricing one GAE (`97` `F_p` multiplications, the measured cost of an affine addition)
  per Macaulay-matrix entry and per walk step, it is `19`–`132×` at `0.1` GAE and `6.5`–`45×` at
  `1` GAE (recomputed from the raw JSON); E24 stays below E19 on every curve at every price.

### 9.22 E31 pre-registration: the solve itself, by its `P₂`-chains

*Registered 2026-10-24, after the profile below and before any E31 solver code.*  **Numbering:**
§9.20–§9.21 and G2.41–G2.84 are claimed by unregistered drafts (E27–E30) on other branches, so
E31 takes §9.22 and starts at G2.85.  The red team's finding stands as the motivation: the solve
is `99.0`–`99.4 %` of `S` on every E25 run and nothing proposed so far touches it.  The baseline is
E25 (§9.16) as validated (§9.19), both arms, its binary `46202dc97`.

**The profile (step 1).**  `examples/e31_solve_profile.rs` (commit `a2de57077a`) rebuilds four E25
curves and replays the first 400 targets of arm A's stream exactly as `collect_and_solve` draws
them with four workers, through the unchanged `v4_system` → `solve_weighted_traced` (degree 11) →
`lift_back_v4`, summing the solver's own phase counters (`experiments/31_solve_cost/profile*.{json,log}`).
Per-call medians are `1.650`, `1.652`, `1.663`, `1.660·10⁶`, E25's `C`.

| phase, per call (mean) | p = 269 s1 | p = 269 s2 | p = 397 s1 | p = 397 s2 | share |
|:--|--:|--:|--:|--:|--:|
| Weil restriction | `5,691` | `5,691` | `5,691` | `5,691` | `0.3 %` |
| triangularise | `175` | `175` | `175` | `175` | `0.0 %` |
| echelon (393 × 448 Macaulay matrix, 384 pivots) | `427,590` | `430,028` | `431,110` | `426,309` | `25.6`–`25.9 %` |
| normal forms (border of `M_{P₂}`, 240 pivots reached) | `859,113` | `857,751` | `864,187` | `851,998` | `51.4`–`51.9 %` |
| characteristic polynomial of the `64 × 64` `M_{P₂}` | `268,238` | `268,746` | `269,168` | `267,733` | `16.0`–`16.3 %` |
| rational roots | `32,577` | `32,648` | `40,998` | `40,743` | `2.0`–`2.5 %` |
| eigenvectors | `61,153` | `70,561` | `69,349` | `53,666` | `3.3`–`4.2 %` |
| solution checks | `270` | `310` | `305` | `236` | `0.0 %` |
| lift-back (outside `C`, in `S`) | `454` | `535` | `584` | `515` | — |

Wall time per call (one thread): echelon `10.2`–`11.9` ms of `17.7`–`21.1` ms; the echelon's
dense row updates cost far more time than the `F_p` multiplications it is charged.

**Two structural facts, measured on the same 1,600 calls.**  (1) The 64 standard monomials fall
into exactly **12 `P₂`-chains** `h, P₂h, …, P₂^{k−1}h` on every call.  `M_{P₂}` is the unit shift
inside each chain; only the 12 chain ends map to non-standard monomials, so only 12 of its 64
columns need a normal form, and they are what the existing normal-form phase computes — by back
substitution of a dense 64-vector through all 240 reached pivots.  (2) A *left-looking* normal form
of each of the 12 targets separately (substitute pivot rows left to right into one sparse
accumulator) costs at most `Σ_t Σ_{c reached from t} nnz(row c)`, measured at `≈ 6.7·10⁴` per call,
against `8.6·10⁵`.  Moreover `F_p[e₂, e₃, P₁, P₂]/I` is presented over `F_p[x]`, `x = P₂`, by the
12 chain heads with the relation matrix `R(x) = diag(x^{k_i}) − C(x)` (`C` read off the 12 normal
forms), so `charpoly(M_{P₂}) = det R(x)`, monic of degree `Σk_i = 64`, and a dual eigenvector at a
rational root `λ` is a kernel vector of the `12 × 12` matrix `R(λ)`.

**What is not proposed, and why.**  A shape trace or elimination template (Joux–Vitse, F4Remake)
on this matrix: E21 (§9.11) measured `1.02×` on the `D₄` solve because a Macaulay matrix at its
bound has little to skip, and the counted echelon already charges only nonzero pivot entries.  A
symbolic eliminant precomputed per curve: the coefficients depend on `z_R ∈ F_{p⁴}`, four `F_p`
parameters, through degree 8; an eliminant in them is far larger than the system.  More symmetry:
`V₄⁴_0 ⋊ S₄` is already quotiented (§9.10), and `ψ` acts on the base, not the system (§8.0).

**The solvers (E31).**  Beside `solve_weighted_traced`, never replacing it, selected by a flag:

- **A, the chain solve.**  The same rows, triangularisation and echelon (the same code path and
  count); the 12 border targets by left-looking normal forms; `det R(x)` by evaluation at 65 points
  of `F_p` and interpolation (each inversion charged `2⌈log₂ p⌉`, which the existing echelon and
  Hessenberg do not charge — conservative against A); the same root finder on the same polynomial;
  at each rational root a kernel of `R(λ)`; the same solution check.  Anything other than one
  clean solve at degree 11 — an eigenspace of dimension above 1, an unresolved vector, an
  unreachable border, a non-standard unknown — reruns the existing solver from the same RNG state,
  and both runs are charged.
- **B, A with sparsest pivots.**  `echelon_by_lead_with(…, sparsest = true)`: the same row space and
  pivot columns, hence the same standard monomials and the same (unique) normal forms, with less
  fill.  Exploratory.

**Correctness bar.**  On every call, the same decomposition set, outcome and degree as the
existing solver; in the agreement test also the same `M_{P₂}` and the same characteristic
polynomial.  Because the outputs and the RNG state after each call coincide, an end-to-end rerun of
E25 with A or B must reproduce every E25 count except the solve's multiplications — itself a
replay check.

| id | quantity | prediction | falsified by |
|:--|:--|:--|:--|
| G2.85 | agreement | A and B return the existing solver's decomposition set, outcome and degree on every call of Stage A and of the end-to-end runs; `M_{P₂}` and `charpoly` identical on every call of the test; the negative control (one coefficient of one equation perturbed) changes the set | any disagreement, or a control that agrees |
| G2.86 | normal forms, existing / A, per-call mean | `[12, 40]` (the bound gives `≥ 12.8`) | outside |
| G2.87 | characteristic polynomial, existing / A (with A's evaluations, interpolation and inversions) | `[1.8, 3.2]` (derived `≈ 2.4`) | outside |
| G2.88 | eigenvectors, existing / A | `[15, 80]` | outside |
| G2.89 | per-call `C` median, existing / A, every curve of Stage A (`p = 101`–`397`, two seeds each) | `[2.2, 3.0]` (derived `≈ 2.55`, the echelon becoming `≈ 2/3` of `C`) | outside; **kill** below `1.5` on any curve (then E31 is recorded as a negative and stops before end to end) |
| G2.90 | fallbacks of A (charged reruns of the existing solver) | `≤ 1 %` of calls | more |
| G2.91 | per-call wall time, existing / A (practicality note) | `[1.2, 2.0]` | outside |
| G2.92 | B against A: echelon multiplications, A / B | `[1.0, 1.6]`; B is carried to end to end only if its per-call `C` median is `≥ 2 %` below A's on every curve | — (exploratory; reported either way) |
| G2.93 | end to end, E25's ten curves, both arms, rerun with A (and B if carried) | every E25 count other than the solve's reproduced exactly; `S_E25 / S_A` in `[2.1, 2.9]` on every curve and arm at `0` GAE; the gap to the matched `⟨±ψ⟩` rho shrinks by the same factor | any count differs, or outside the band |
| G2.94 | §5a band | unpriced native work as in E25 (one GAE per Macaulay entry, unchanged by A; one per walk step): `S_E25 / S_A` `[1.3, 1.6]` at `0.1` GAE and `[1.03, 1.10]` at `1` GAE; A below E25 at every weight | A above E25 at any weight |

**What it would say.**  If G2.85–G2.89 and G2.93 hold, the `k = 4` index calculus on the `V₄` line
costs `≈ 2.5×` less end to end on E25's curves with the same relations, logs and walks: **class
engineering** (a constant on the dominant phase; exponents unchanged), a verified, measured
improvement of the baseline under the 2026-10-15 criterion.  The band (G2.94) says how much of it
survives once the Macaulay matrix's own entries are priced: most of it at `0.1` GAE, a few percent
at `1` GAE, where the matrix A still builds dominates.  The rows would stay `≈ 4·10³`–`10⁵×` from
the matched rho: the distance remaining.  After A the echelon is the next lever.

### 9.23 E31 measured: the chain solve cuts the `k = 4` solve `2.6×` (A) and `5.1×` (B) for the same output, and `S` by `2.5×` and `4.7×`

*2026-10-24.*  Code: `klein_v4::solve_weighted_chain` beside the unchanged `solve_weighted_traced`
(commit `f6ab6b7253`), selected by `V4Solver` / `quartic_pipeline_gls --solver`.  **Stage A:**
`examples/e31_stage_a.rs`, binary `a8f52422d5` (`experiments/31_solve_cost/binary_commit.txt`), one
thread, the first 1,000 targets of E25 arm A's stream on each of E25's ten curves, the three solvers
interleaved per call from the same per-target RNG seed (`stage_a.{json,log}`).  **End to end:**
`experiments/31_solve_cost/run_e2e.sh` (E25's `run_all.sh` plus `--solver`, `RAYON_NUM_THREADS=4`), same
binary, both solvers, both arms, all ten curves (`e2e_{chain,chain-sparsest}_p*s*.json`, logs).
Tables: `experiments/31_solve_cost/tables.md` (`examples/e31_tables.rs`).  Host: the 4-vCPU container;
wall times are a practicality note.

**Stage A** (per call; existing = E25's solver; all calls, fallbacks charged):

| p | seed | `C` median existing | A | B | existing / A | existing / B | NF existing / A | charpoly existing / A | eigvec existing / A | echelon A / B | fallbacks A | wall existing / A / B (µs, median) |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|:--|
| 101 | 1 | `1,617,013` | `600,981` | `304,257` | 2.69 | 5.31 | 9.2 | 2.85 | 16.4 | 2.52 | 12 | `15,381` / `9,804` / `10,008` |
| 101 | 2 | `1,616,974` | `600,890` | `304,103` | 2.69 | 5.32 | 9.7 | 2.87 | 15.7 | 2.54 | 11 | `17,333` / `10,884` / `11,332` |
| 149 | 1 | `1,632,660` | `612,064` | `313,237` | 2.67 | 5.21 | 10.6 | 2.87 | 18.7 | 2.58 | 7 | `16,666` / `10,569` / `10,795` |
| 149 | 2 | `1,633,472` | `612,068` | `313,320` | 2.67 | 5.21 | 9.5 | 2.82 | 15.4 | 2.54 | 10 | `15,494` / `9,838` / `9,984` |
| 197 | 1 | `1,645,617` | `620,058` | `320,425` | 2.65 | 5.14 | 10.9 | 2.88 | 20.4 | 2.60 | 5 | `15,757` / `9,755` / `9,943` |
| 197 | 2 | `1,646,416` | `620,152` | `320,741` | 2.65 | 5.13 | 10.0 | 2.84 | 15.6 | 2.56 | 8 | `17,060` / `10,422` / `10,477` |
| 269 | 1 | `1,650,218` | `623,969` | `323,602` | 2.64 | 5.10 | 10.9 | 2.84 | 20.5 | 2.60 | 5 | `15,215` / `9,747` / `9,926` |
| 269 | 2 | `1,651,643` | `624,544` | `324,142` | 2.64 | 5.10 | 11.2 | 2.85 | 25.9 | 2.61 | 4 | `16,602` / `10,308` / `10,540` |
| 397 | 1 | `1,663,086` | `633,960` | `333,134` | 2.62 | 4.99 | 11.6 | 2.86 | 26.5 | 2.62 | 3 | `19,819` / `11,849` / `11,388` |
| 397 | 2 | `1,661,986` | `633,419` | `332,624` | 2.62 | 5.00 | 12.4 | 2.89 | 30.0 | 2.65 | 1 | `18,678` / `11,614` / `11,202` |

Agreement: A and B returned the existing solver's solutions, degree, outcome and decomposition set
on all 10,000 calls each; on the 9,996 calls the existing solver does not refuse, `M_{P₂}` and the
characteristic polynomial were identical too.  On the calls that did not fall back, A's normal forms
cost `65.6`–`66.8·10³` (the registered bound, `≈ 6.7·10⁴`, holds) and B's `31.6`–`32.2·10³`.
A's per-call `C` is now echelon `≈ 70 %`, charpoly `≈ 15 %`; B's echelon `≈ 170·10³`, charpoly
`≈ 94·10³`.  The unit test (`e31_chain_solve_agrees_with_the_existing_solver`: E25 `p = 101, 149`,
E24 `p = 101, 53`, both pivot rules) agrees on every call, and its negative control (one coefficient
of one equation moved) changes every non-empty solution set.

**What the fallbacks are.**  In all 40 end-to-end runs the number of fallbacks equals E25's
`reruns + unsolved` exactly (e.g. `488 = 467 + 21` at `p = 101` seed 1, arm A): the chain solve hands
back precisely the calls that the existing solver itself cannot finish cleanly at degree 11
(degenerate eigenspaces, unresolved vectors, unreachable borders), and no others.  Those calls cost
the existing solver `4.5`–`4.8·10⁶` each, and the chain attempt adds `≈ 13 %` to them.

**End to end** (E25's ten curves, both arms, `S` in `F_p` multiplications; the matched walk is the
`⟨±ψ⟩` walk; E25's numbers from its records):

| p | seed | arm | `S` E25 | `S` A | `S` B | E25 / A | E25 / B | `S`/rho E25 | `S`/rho A | `S`/rho B |
|--:|--:|:--|--:|--:|--:|--:|--:|--:|--:|--:|
| 101 | 1 | A | `6.76·10¹⁰` | `2.71·10¹⁰` | `1.49·10¹⁰` | 2.50 | 4.53 | `1.31·10⁵` | `5.27·10⁴` | `2.90·10⁴` |
| 101 | 1 | B | `3.48·10¹⁰` | `1.39·10¹⁰` | `7.63·10⁹` | 2.51 | 4.57 | `6.77·10⁴` | `2.70·10⁴` | `1.48·10⁴` |
| 101 | 2 | A | `2.47·10¹⁰` | `9.87·10⁹` | `5.42·10⁹` | 2.50 | 4.56 | `6.19·10⁴` | `2.47·10⁴` | `1.36·10⁴` |
| 101 | 2 | B | `1.17·10¹⁰` | `4.68·10⁹` | `2.58·10⁹` | 2.50 | 4.54 | `2.93·10⁴` | `1.17·10⁴` | `6.46·10³` |
| 149 | 1 | A | `5.83·10¹¹` | `2.30·10¹¹` | `1.25·10¹¹` | 2.53 | 4.66 | `7.07·10⁵` | `2.79·10⁵` | `1.52·10⁵` |
| 149 | 1 | B | `2.19·10¹¹` | `8.64·10¹⁰` | `4.70·10¹⁰` | 2.53 | 4.65 | `2.65·10⁵` | `1.05·10⁵` | `5.71·10⁴` |
| 149 | 2 | A | `1.01·10¹¹` | `3.98·10¹⁰` | `2.17·10¹⁰` | 2.53 | 4.64 | `1.24·10⁵` | `4.92·10⁴` | `2.68·10⁴` |
| 149 | 2 | B | `5.67·10¹⁰` | `2.25·10¹⁰` | `1.23·10¹⁰` | 2.52 | 4.62 | `7.01·10⁴` | `2.78·10⁴` | `1.52·10⁴` |
| 197 | 1 | A | `8.53·10¹⁰` | `3.35·10¹⁰` | `1.83·10¹⁰` | 2.54 | 4.67 | `8.24·10⁴` | `3.24·10⁴` | `1.76·10⁴` |
| 197 | 1 | B | `6.32·10¹⁰` | `2.49·10¹⁰` | `1.36·10¹⁰` | 2.54 | 4.66 | `6.11·10⁴` | `2.40·10⁴` | `1.31·10⁴` |
| 197 | 2 | A | `4.39·10¹⁰` | `1.72·10¹⁰` | `9.37·10⁹` | 2.55 | 4.68 | `4.44·10⁴` | `1.74·10⁴` | `9.48·10³` |
| 197 | 2 | B | `2.59·10¹⁰` | `1.02·10¹⁰` | `5.53·10⁹` | 2.55 | 4.68 | `2.62·10⁴` | `1.03·10⁴` | `5.60·10³` |
| 269 | 1 | A | `1.37·10¹¹` | `5.36·10¹⁰` | `2.90·10¹⁰` | 2.56 | 4.73 | `8.41·10⁴` | `3.29·10⁴` | `1.78·10⁴` |
| 269 | 1 | B | `7.88·10¹⁰` | `3.09·10¹⁰` | `1.67·10¹⁰` | 2.55 | 4.72 | `4.84·10⁴` | `1.90·10⁴` | `1.02·10⁴` |
| 269 | 2 | A | `5.10·10¹⁰` | `1.99·10¹⁰` | `1.07·10¹⁰` | 2.56 | 4.76 | `3.16·10⁴` | `1.23·10⁴` | `6.65·10³` |
| 269 | 2 | B | `2.52·10¹⁰` | `9.83·10⁹` | `5.30·10⁹` | 2.56 | 4.75 | `1.56·10⁴` | `6.09·10³` | `3.29·10³` |
| 397 | 1 | A | `7.19·10¹⁰` | `2.81·10¹⁰` | `1.52·10¹⁰` | 2.56 | 4.72 | `1.95·10⁴` | `7.62·10³` | `4.13·10³` |
| 397 | 1 | B | `4.17·10¹⁰` | `1.63·10¹⁰` | `8.82·10⁹` | 2.56 | 4.73 | `1.13·10⁴` | `4.42·10³` | `2.39·10³` |
| 397 | 2 | A | `1.31·10¹¹` | `5.15·10¹⁰` | `2.79·10¹⁰` | 2.55 | 4.70 | `3.97·10⁴` | `1.56·10⁴` | `8.45·10³` |
| 397 | 2 | B | `5.65·10¹⁰` | `2.22·10¹⁰` | `1.20·10¹⁰` | 2.55 | 4.70 | `1.71·10⁴` | `6.71·10³` | `3.64·10³` |

Every record reproduces E25's field for field apart from the solve's multiplications, what is
derived from them, wall time, `macaulay_entries` (a fallback builds the matrix twice) and the solver
tags: the same residuals, unsolved targets, reruns, decompositions, relations, linear algebra, logs
and walks.  Every log recovered, every walk correct, every kept row re-added (as in E25).  The solves
are now `98.4`–`98.9 %` of `S` (A) and `97.0`–`97.9 %` (B).

**§5a band** (one GAE, `97` `F_p` multiplications, per Macaulay entry and per walk step, as E25 priced
it): `S_E25 / S_A` is `2.50`–`2.56` at `0`, `1.39`–`1.43` at `0.1`, `1.034`–`1.053` at `1` GAE;
`S_E25 / S_B` is `4.53`–`4.76`, `1.59`–`1.64`, `1.051`–`1.071`.  Both stay below E25 on every row at every
weight, and the matched rho stays ahead of every row of both at every weight.  At `1` GAE the matrix
the chain solve still builds (`393 × 448` entries per call) is almost all of the cost: what E31 cut is
the arithmetic, not the matrix.

| id | result | verdict |
|:--|:--|:--|
| G2.85 | identical outputs on all 10,000 Stage A calls (A and B) and all 40 end-to-end records; `M_{P₂}` and charpoly identical on 9,996 of 9,996 applicable calls; the negative control differs on every non-empty set | **holds** |
| G2.86 | NF existing / A, all calls with fallbacks charged: `9.2`–`12.4` (in band on one curve of ten); on calls without fallback `≈ 13×` (A's NF `65.6`–`66.8·10³`) | **falsified low**: the band was set from the clean-call bound and did not price the charged fallbacks, which carry the existing `≈ 8.5·10⁵` normal forms on top |
| G2.87 | charpoly existing / A `2.82`–`2.89` | **holds** |
| G2.88 | eigenvectors existing / A `15.4`–`30.0` | **holds** |
| G2.89 | `C` median existing / A `2.62`–`2.69` on every curve | **holds**; kill (`< 1.5`) not reached |
| G2.90 | fallbacks `0.1`–`1.2 %` per curve in Stage A (`1.2 %`, `1.1 %` at `p = 101`), `0.2`–`1.1 %` per end-to-end run | **falsified at `p = 101`**, holds at `p ≥ 149`; the fallbacks are exactly E25's reruns and unsolved targets |
| G2.91 | wall existing / A `1.56`–`1.67` | **holds** |
| G2.92 | echelon A / B `2.52`–`2.65`; B's `C` median `49`–`50 %` below A's on every curve | **falsified high** (band `[1.0, 1.6]`); B carried to end to end by the registered rule |
| G2.93 | every E25 count reproduced; `S_E25 / S_A` `2.50`–`2.56` on every curve and arm; gap to the matched rho shrinks by the same factor | **holds** |
| G2.94 | `1.39`–`1.43` at `0.1` GAE, `1.034`–`1.053` at `1` GAE; A below E25 at every weight | **holds** |

**What it says.**  The `k = 4` decomposition on the `V₄` line was paying for a dense 64-wide normal
form at each of 240 pivots, a dense Hessenberg reduction and a dense 64-dimensional kernel, while
its quotient is a 12-generator module over `F_p[P₂]`.  Working in that module — left-looking normal
forms of the 12 chain ends, `det R(x)` of a `12 × 12` polynomial matrix, kernels of `R(λ)` — and
choosing the sparsest pivot rows cuts one call from `1.62`–`1.66·10⁶` to `3.04`–`3.33·10⁵` `F_p`
multiplications (B), with the same decompositions on every call.  **End to end, on E25's curves and
target streams, `S` falls `4.53`–`4.76×` (B; `2.50`–`2.56×` with A alone), with every relation, log and
walk unchanged.**  **Class: engineering** (a constant on the dominant phase; the exponents and the
matrix are unchanged) — a verified, measured improvement of the baseline under the 2026-10-15
criterion, and B is the new baseline on this family.  It is an operation-count result at `0` GAE;
at `1` GAE per Macaulay entry it is `5`–`7 %`.  **Distance remaining:** every row is still
`2.39·10³`–`1.52·10⁵×` slower than the matched `⟨±ψ⟩` rho (`2.39·10³×` at `p = 397` seed 1, arm B, the
best; E25's best was `1.13·10⁴×`).  The §11.16 handover was not recomputed here.  **Scope:** E25's
GLS twists, `p ≤ 397`, the degree-11 solve; E24's curves use the same solver and the unit test covers
two of them, but E24 was not rerun end to end.  **Next levers, in order of measured weight:** B's
echelon (`≈ 1.7·10⁵`, wall time `≈ 10` ms of `≈ 11` ms per call), `det R(x)` (`≈ 9.4·10⁴`, `64`
evaluations of a `12 × 12` determinant), and the `0.2`–`1.1 %` fallbacks (`≈ 4.6·10⁶` each, up to
`≈ 16 %` of B's mean `C` at `p = 101`), which a chain-side treatment of degenerate eigenspaces would
remove.  Not done here: the validation gate (independent replay certificate and validator report),
the leaderboard, `benchmarks/` and the visual report.

### 9.24 E31 validated, with errata and a §5a qualification on solver B

*2026-10-24.*  Independent validation `VAL-20261024-24ef8b` (`benchmarks/20261024-e31-chain-solve/`:
`VALIDATION.json`, `replay-certificate.json`, `validator-report.json`, `validate.py`, `replay/`) passed.
It rebuilt from a tree whose build inputs equal the recorded binary `a8f52422d5` and replayed all 40
end-to-end records and all ten Stage A curves.  Every deterministic field was identical (only wall
times differ).  It also ran a control of the existing solver through the new path, which reproduces
E25 `p = 101` seed 2 exactly.  An out-of-sample agreement check on three new GLS curves
(`(113, 4)`, `(229, 3)`, `(353, 3)`) agreed on all 3,000 calls.  It found no uncounted
multiplication, cache or per-curve precomputation in the chain solve; fallbacks are charged in full.
All 361 deciding figures of §9.23 and the ten G2.85–G2.94 verdicts recompute.  §9.21–§9.23 stand as
recorded; the following are additive corrections.

**Errata** (none changes a table cell or a verdict):

1. "B's `C` median `49`–`50 %` below A's" (G2.92) is `47.45`–`49.39 %`.
2. "`0.2`–`1.1 %` fallbacks per end-to-end run" (G2.90 and "Next levers") is `0.266`–`1.196 %`.
3. "Those calls cost the existing solver `4.5`–`4.8·10⁶` each" holds for the 62 Stage A reruns
   (`4.54`–`4.84·10⁶`).  The 4 calls the existing solver itself refuses cost `0.44`–`0.52·10⁶`.
4. A's echelon share "≈ 70 %" is `66.1`–`67.4 %` of the mean `C`; only the median over calls without
   a fallback reaches `≈ 68`–`70 %`.  B's fallback weight at `p = 101`, "up to ≈ 16 %", is at most
   `≈ 15 %` (4 fallbacks unidentified in Stage A, so approximate).
5. "Every row is still `2.39·10³`–`1.52·10⁵×`" and "at 1 GAE it is 5–7 %" are solver B's ranges; A's
   rows reach `2.79·10⁵×` and its 1-GAE saving is `3.4`–`5.3 %`.
6. §9.22's quoted "the solve is `99.0`–`99.4 %` of `S`" fits neither definition: solve/`S` is
   `99.03`–`99.20 %`, (Weil + solve)/`S` is `99.37`–`99.54 %`.
7. The E31 JSON records carry `"experiment": "e25"`; only the `solver` field distinguishes them.

**§5a qualification on solver B.**  The sparsest-pivot rule counts the nonzeros of every row it
builds and recounts each updated row.  That work is neither counted nor emitted, and the registered
band model (one native operation per Macaulay entry) does not see it.  The validator priced it at the
band's rate from an exact lower bound (`2 E_E25 − E_E31` entry reads per record).  On that basis B
falls *behind* E25 on all 20 rows at `0.1` GAE (`S_E25/S_B = 0.885`–`0.899`) and at `1` GAE
(`0.542`–`0.544`).  So **B beats E25 and A only at 0 GAE; across the band its rank is unresolved**,
and B is not a new baseline.  On wall clock B is no faster than A on this host (`B/A = 0.96`–`1.04`).
**Solver A** is ahead of E25 at every weight under the registered model: `S_E25/S_A = 2.50`–`2.56`
(0), `1.39`–`1.43` (0.1), `1.034`–`1.053` (1 GAE).  A's 1-GAE margin is smaller than the native
traffic both solvers leave unpriced, so at 1 GAE it holds only under that model.

**Distance remaining to the matched `⟨±ψ⟩` rho** (`S`/rho over the 20 rows):

| GAE | E25 | A | B (registered model) | B, pivot pass priced |
|--:|:--|:--|:--|:--|
| 0 | `1.13·10⁴`–`7.07·10⁵` | `4.42·10³`–`2.79·10⁵` | `2.39·10³`–`1.52·10⁵` | `2.39·10³`–`1.52·10⁵` |
| 0.1 | `2.15·10⁴`–`1.38·10⁶` | `1.50·10⁴`–`9.77·10⁵` | `1.31·10⁴`–`8.55·10⁵` | `2.39·10⁴`–`1.55·10⁶` |
| 1 | `7.72·10⁴`–`5.45·10⁶` | `7.33·10⁴`–`5.22·10⁶` | `7.21·10⁴`–`5.14·10⁶` | `1.42·10⁵`–`1.0·10⁷` |

Rho is ahead of every row at every weight.  **What E31 establishes:** a validated engineering
(phase) speed-up of the `k = 4` solve, solver A, ranked ahead of E25 across the band under the
registered model.  It is not an exponent change.  Next levers are unchanged from §9.23.  B's
pivot-selection pass should be counted, or replaced by a counted sparsity estimate, before B is
ranked.

### 9.25 E27 pre-registration: the matched walk, strengthened (Karatsuba tower products, batched norm inversions)

*Registered 2026-10-30, before any E27 code.*  **Numbering:** the 2026-10-22 drafts of E27–E30 are
lost; §9.22 recorded that they claimed §9.20–§9.21 and G2.41–G2.84, which stay unused.  E27 takes
§9.25 and G2.95 onward.  **Motivation** (an independent red team's surviving finding, not re-derived
here except where stated): every distance to rho in §9.14–§9.24 is measured against a walk whose
arithmetic is weaker than it needs to be.  `Fp4::mul` is charged `19` (schoolbook: `16` products and
`3` reductions by `c`) and `Fp4::inv` `40` (`24` products and `16` for one `inv_mod`), so an affine
addition is `40 + 3·19 = 97`, the GAE used in every band above.  With E31 the solve is a smaller share
of `S`, so a stronger reference matters more than it did.  **Class, fixed in advance:** the rho-side
change is a *reference correction* (accounting), not an improvement: if it holds, every row moves
further from rho.

**The arithmetic (derived before code).**  `F_{p⁴} = F_{p²}[t]/(t² − s)`, `F_{p²} = F_p[s]/(s² − c)`.
A Karatsuba product in `F_{p²}` costs `3` products and `1` reduction by `c`; in `F_{p⁴}` it takes three
`F_{p²}` products (`AC`, `BD`, `(A + B)(C + D)`) and one more reduction by `c` for `BD·s`: **`13`**,
every one a real `F_p` multiplication, counted as performed.  The inversion keeps its `24` products
(norm to `F_{p²}`, norm to `F_p`, two scalings, two `F_{p²}` products) and replaces its one `F_p`
inversion (`16`) by a share of Montgomery's simultaneous inversion over `m` concurrent walks:
`3(m − 1) + 16` for the batch, so `3.81` per inversion at `m = 16`.  An affine addition then costs
`24 + 3.81 + 3·13 ≈ 66.8` (red team: `≈ 66`), a doubling `≈ 79.8` (was `116`), the GLS map `ψ`
`2·13` (was `2·19`).  Every component falls by `1.45`–`1.46×`.  Batching the whole `F_{p⁴}` inversion
instead of its norm is worse (red team) and is not built.

**Protocol (Stage W).**  On E25's ten curves (`generate_gls_klein_instance(p, seed, 16)`,
`p ∈ {101, 149, 197, 269, 397}`, seeds `1, 2`), the `⟨±ψ⟩` walks and the negation walks with E25's
seeds (`1000·seed + k`, `k < 16`), each walk run three ways: **(i)** E25's arithmetic, serial (a
replay of E25's records); **(ii)** Karatsuba products, serial; **(iii)** Karatsuba products with the
norm inversions batched over a pool of `16` walks run in lockstep, set-up included (the `66` scalar
multiplications of each walk's set-up run in lockstep too).  When a recorded walk finishes, a filler
walk with a fresh seed takes its slot so the batch stays at `m = 16`; fillers are not recorded and pay
their own share.  A batch's cost is split among its members in integers (rotating remainder), so the
walk totals are exact.  Each walk also records a digest of every class representative it visits.
Then E31's solver-A pipeline (`experiments/31_solve_cost/run_e2e.sh` with `--solver chain`) is rerun
on the same ten curves and both arms with Karatsuba products in all of its `F_{p⁴}` work (generation,
base, precompute, Weil restriction where it uses `F_{p⁴}`, lift-back, checks; the solve is over `F_p`
and is untouched); the IC side does not get batched inversions, and the most they could save is
reported from a new counter of `F_p` inversions.  Distances to rho use E31 A's `S` (and the
Karatsuba `S`) against walk (iii).  §5a band as in E25/E31: one GAE per Macaulay entry and per walk
step, GAE `= 97` (primary, comparable with §9.24); with GAE re-measured under (iii) as a sensitivity.
Host: the 4-vCPU container (isolab-eks is not reachable from it); operation counts only.

| id | quantity | prediction | falsified by |
|:--|:--|:--|:--|
| G2.95 | bit identity (**kill**) | arithmetic (i) reproduces every E25 walk record field for field (steps, group operations, `F_p` multiplications, cycle escapes, recovered log, correctness); (ii) and (iii) give the same steps, group operations, cycle escapes, recovered logs and visit digests as (i) on all `320` walks; unit tests: Karatsuba `=` schoolbook on random products, batched `=` unbatched inverses and additions, and a negative control (one perturbed norm inverse in a batch) is detected | any difference (E27 then stops and is recorded as a failure) |
| G2.96 | per-operation costs | `Fp4::mul` `13` exactly; addition of distinct points in (iii) `[64, 70]`; `F_p` multiplications per group operation over a whole walk, (i) / (iii), `[1.38, 1.50]` | outside |
| G2.97 | the matched walk | `rho_E25 / rho_E27` (mean `F_p` multiplications, (i) over (iii)) in `[1.38, 1.50]` on every curve for the `⟨±ψ⟩` walk and for the negation walk (red team's estimate of the row shift: `1.03`–`1.3`; this band is derived from the component costs above) | outside |
| G2.98 | the IC side | `S_A / S_{A,K}` in `[1.001, 1.008]` on every row (non-solve `F_{p⁴}` work `≤ 2.5 %` of `S_A`); every other field of E31 A's records reproduced exactly | outside, or any other field differs |
| G2.99 | distance to rho | `(S/rho)` grows by `[1.37, 1.50]` per row at `0` GAE, `[1.30, 1.48]` at `0.1`, `[1.12, 1.30]` at `1` GAE (GAE `= 97`); rho ahead of every row at every weight | outside; any row below rho |

**What it would say.**  If G2.95–G2.99 hold, the matched rho of §9.24 was `≈ 1.45×` too expensive, and
E31's distances read `≈ 1.45×` larger at `0` GAE: a correction of the reference, reported as such and
not as a change of the index calculus.  The IC side gains a few tenths of a percent (engineering,
negligible), because its `F_{p⁴}` work is `≈ 2 %` of `S`.

### 9.26 E28 pre-registration: the GLS fold on the single-`τ_T` line (stages 0, A and A2 only)

*Registered 2026-10-30, before any E28 code.*  **What E28 is, reconstructed.**  The 2026-10-22 draft is
lost; what survives is the red team's reading that this `τ_T` variant "predicts itself `≥ 10×` behind
E24, so it can improve the E19 row but not the baseline", and that only its cheap stages 0, A and A2
should run, plus one new cheap control on solve cost.  §9.18 named the variant: the `ψ` fold without the
full `V₄` symmetry, on E19's single-`τ_T` line, where §9.17's obstruction to `h = 4` does not apply but
the solve is E19's weighted `D₄` solve (`≈ 1.4·10⁹` a call, `≈ 820×` E24's).  The derivation below is new
and is checked by Stage 0 before anything is measured.

1. **The curves.**  Over `F_p[t]/(t⁴ − c)` the `p²`-power map is `σ: t ↦ −t`.  Take `x₀` with
   `σ(x₀) = −x₀` (coordinates on `t` and `t³` only, not both zero), `c ∈ F_p^*`, and E19's
   `a = c² − 3x₀²`, `b = −x₀³ − a x₀`, so `T = (x₀, 0)` is on `E` and `f′(x₀) = c²`.  Then `x₀² ∈ F_{p²}`,
   `σ(a) = a`, `σ(b) = −b`, and `E` is the quadratic twist over `F_{p⁴}` by `x₀` (a square root of a
   non-square of `F_{p²}`) of a curve over `F_{p²}`: a GLS twist, `j ∈ F_{p²}`.
2. **`ψ` keeps `T` and the line.**  `ψ(x, y) = (−σ(x), i·σ(y))`, `i² = −1` in `F_p` (`p ≡ 1 mod 4`): on
   `E` because `σ(f(x)) = f(−σ(x))·(−1)` and `i² = −1`; `ψ² = −1`; `ψ(T) = T`.  On E19's line
   `x = x₀ + u`, `u ∈ F_p`: `ψ(x₀ + u) = x₀ − u`, so `ψ` acts as `u ↦ −u`, `τ_T` as `u ↦ c²/u`, `τ_T ψ` as
   `u ↦ −c²/u`: **`8` points a column** (`4` abscissae, `±`) against E19's `4`.  Points with `u² = −c²`
   have `ψP = ±(P + T)`, so `λ·log P = ±log P` and `log P = 0`; they are excluded with `u ∈ {0, ±c}`.
3. **The cofactor.**  All of `E[2]` is rational (the other two roots lie in `F_{p²}(√(9x₀² − 4c²)) ⊆ F_{p⁴}`),
   so `4 | h`; §9.17's argument excludes `h = 4` only for the `V₄` line, which needs the halves of a second
   `T`.  Here only `T`'s halves matter, and `f′(x₀) = c²` is already a square.  So `h = 4` is not excluded.
4. **What it can buy.**  As in E25, `ψ` acts on the base, not on the system: decompositions per target
   (`≈ 8`, E19's `τ_T` orbit) and `y` unchanged, columns halved, so the relation phase halves; the matched
   walk is the `⟨±ψ⟩` walk, `√2` fewer steps than E19's negation walk.  Net against the matched rho
   `≈ √2`, on a line whose rows are `2.6`–`8.8·10⁶×` from rho (E19) against E24's `1.3·10⁴`–`1.7·10⁵×`:
   the variant can improve E19's row and cannot approach the baseline (E31 A on E25's curves).

**Protocol.**  Sizes E19's: `p = 53, 101, 149, 197, 269`.  **Stage 0** (gate): seeds `1`–`8` per `p`,
the generator keeping the least `h` (`≤ 16`) its budget meets; on every curve check on points `ψ² = −1`,
`ψ(G) = [λ]G` with `λ² ≡ −1 (mod n)`, `ψ(T) = T`, and that every base point's `ψ`-, `τ_T`- and
`τ_Tψ`-images are base points with the derived `u`-action; record `h`, density and points per column.
**Stage A** (the fold, measured): seeds `1, 2` per `p`; one target stream `R = aG + bQ` decomposed by
E18's exact meet-in-the-middle oracle (`mitm_signed`, every decomposition re-added in the group), one
decomposition per target drawn uniformly, feeding two arms on the same points: **τ**, E19's `τ_T`-folded
base (`4` points a column), and **τψ**, the `⟨τ_T, ψ⟩`-folded base (`8`); each filtered and solved by
Wiedemann until `[d]G = Q` verifies; `16` `⟨±ψ⟩` and `16` negation walks per curve, priced both with
E25's arithmetic and with E27's (§9.25).  **Solve-cost control** (new, cheap): E19's weighted `D₄` solve,
unchanged (`weighted_system` → `solve_weighted` → `lift_back`), on `3` uniform targets of each Stage A
curve, compared with `mitm_signed` on the same base.  **Stage A2** (a model, labelled as such):
`S_model = (residuals to the τψ log) × (control's median C) + LA + generation`, against the matched
walk, compared with E19's measured `S/rho` at the same `p` and with E31 A's.  **No Stage B** (no
end-to-end run with the `D₄` solve).  Host: the 4-vCPU container, operation counts only.

| id | quantity | prediction | falsified by |
|:--|:--|:--|:--|
| G2.100 | Stage 0 (gate; **kill**) | at every `p` curves exist; `ψ² = −1`, `ψ(G) = [λ]G`, `λ² ≡ −1`, `ψ(T) = T` on points; every image in the base with the derived action; `8.00` / `4.00` points a column on the same points; density pooled in `[0.3, 0.7]`; `h = 4` on at least one of `8` seeds at every `p` | a `ψ`/line check fails or no curve (E28 stops, recorded as a negative); `h = 4` never met at some `p` (recorded; Stage A runs on the least `h`) |
| G2.101 | Stage A (fold) | `cols_τ / cols_τψ = 2.00` on every curve; decompositions per decomposed target median `8`; `y_τψ / y_τ` pooled in `[0.8, 1.25]`; residuals to the log `τ / τψ` pooled in `[1.6, 2.4]`; both arms recover the log on every curve | outside, or a log not recovered |
| G2.102 | solve-cost control | every solved call's decomposition set equals `mitm_signed`'s; per-call `C` median within `±30 %` of E19's `1.37`–`1.41·10⁹` at the same `p` | any disagreement, or outside |
| G2.103 | Stage A2 (model, extrapolation) | `S_model / rho_⟨±ψ⟩` below E19's `S / rho_neg` at the same `p` by `[1.4, 4.0]` (E25 arithmetic on both); `≥ 10×` above E31 A's `S/rho` range on E25's curves at every `p` | outside the first band; within `10×` of E31 A |

**What it would say.**  If G2.100–G2.103 hold, the `ψ` fold carries over to E19's line with `h = 4`, cuts
that line's modelled cost by `≈ 2×` against its own matched walk, and leaves it two orders of magnitude
behind the `V₄` line: **class engineering on the `D₄`/`τ_T` line, not an improvement of the baseline**,
and since A2 is a model, not a measured `S` either.  A Stage B would be justified only if A2 put the line
within reach of E31 A, which the derivation says it cannot.

### 9.27 E29 pre-registration: the order-24 twist action on E18's `j = 0` line (kill test, Stage R, Stage A only)

*Registered 2026-10-30, before any E29 code.*  **What E29 is, reconstructed.**  The draft is lost; the red
team's surviving findings: a twist construction whose draft predicted a handover `n* ≈ 2^{139}`–`2^{144}`
(against E24's `2^{90.1}`), so it cannot beat the baseline; only Stage R and Stage A should run, after a cheap
kill test — does `ψ_p`, the endomorphism derived from the `p`-power Frobenius, have order `24` on E18's
curves?  The red team noted that E18's eight prime orders are `≡ 1 (mod 24)` (also mod `16` and `48`;
`n mod 96 ∈ {1, 49}`), which allows it but does not prove it.  The lane
`research/twist_index_calculus_20261020/` built the object on canonical quartic twists
`y² = x³ + b₀t³` (`Twist24Instance`, `rho_twist24`, `run_k4_twist24`): the transported Frobenius
`ψ(x, y) = (h²x^p, h³y^p)`, `ψ⁴ = −1`, with the cubic automorphism `ι` a signed action of order `24`.
E29 measures what that action does to **E18's `ψ` arm** (§9.3: `r_ψ = 0.138` against the order-6 walk,
handover `2^{146.0}`).  Derived before code:

1. **`ψ_p` exists on every E18 curve.**  `E_b : y² = x³ + b` over `F_{p⁴}`, `p ≡ 1 (mod 12)`; Frobenius maps
   `E_b` to `E_{b^p}`, isomorphic to `E_b` because `b^{p−1}` is a sixth power (`12 | p − 1`).  With
   `u = b^{−(p−1)/6}`, `ψ_p(x, y) = (u²x^p, u³y^p)` is an endomorphism of `E_b` over `F_{p⁴}` and
   `ψ_p⁴ = (ζ²x, ζ³y)`, `ζ = N(u) = χ₆(b)⁻¹`: **`ord ψ_p = 4·ord χ₆(b)`**, `24` exactly when `b` is
   neither a square nor a cube.  The group `⟨ψ_p, Aut E⟩` has order `24` on every curve (`ψ_p⁴ ∈ Aut E`,
   `ψ_p^j ∉ Aut E` for `j = 1, 2, 3`), and it acts faithfully on `⟨G⟩` (a non-trivial element minus `1` has
   degree `≤ (1 + p^{3/2})² < n`), so as a cyclic group of order `24`.
2. **It does not keep E18's base** unless `u² ∈ F_p`; but each E18 curve is `F_{p⁴}`-isomorphic to a
   canonical model `y² = x³ + b₀t³` with the same prime order (E18's `p = 241, 337` orders equal the lane's
   `(241, 49)`, `(337, 100)`), where `h² ∈ F_p` and the line `x ∈ F_p` is kept.  Rho and the base can both
   use the order-24 action on that model, and an attacker maps to it at the cost of one isomorphism.
3. **What it moves.**  Against E18's order-6 `⟨±ψ⟩` walk, an order-24 walk takes `√4 = 2×` fewer steps
   but pays a costlier class representative (three Frobenius images and twelve candidates): derived
   `F_p`-multiplication ratio `[1.3, 2.0]`.  On the base, columns `÷ 4` (`12` abscissae a column against
   `3`), the linear algebra `÷ ≈ 16`, so `r_24 ≈ r_ψ · (rho ratio)/16`, and §11.16's handover moves by
   `(cols ÷ 4)` against `(S_rho ÷ ratio)`: `n* ÷ (4/ratio)⁴`, i.e. `2^{146.0} → ≈ 2^{140}`–`2^{144}` (the
   draft's band).  The order-24 walk alone (no base fold) moves E18's `ψ` handover *later* by
   `4·log₂(ratio)` bits.

**Protocol.**  E18's eight `ψ` curves (`generate_psi_instance(p, 1)`, `p = 241, 337, 541, 601, 733, 769, 997,
1069`).  **Kill test (first, per curve):** on E18's own model compute `ψ_p` as above; check on points that it
maps `E` to `E`; find `λ_p` with `ψ_p(G) = [λ_p]G` among the 24th roots of unity mod `n`; record `ord λ_p`,
`ord χ₆(b)`, `|⟨λ_p, λ_ω, −1⟩|`, and whether `u² ∈ F_p`; find `b₀` with `#(y² = x³ + b₀t³) = n`.
**Kill:** `|⟨λ_p, λ_ω, −1⟩| < 24` on a curve (that curve leaves E29), or no curve passes (E29 stops).
**Stage R** (the matched reference, retrospective): on the canonical model of each passing curve, `16`
order-6 `⟨±ψ⟩` walks (`rho4_folded`, `Psi`) and `16` order-24 walks (`rho_twist24`), the same seeds, both in
E18's arithmetic (schoolbook) and in E27's products (Karatsuba; unbatched, as `rho_twist24` is serial),
the `[d]G` check excluded from both.  First on `p = 241` alone (the red team's one-curve kill budget),
then on the rest.  **Stage A** (the base fold): the lane's `run_k4_twist24` (E18's exact oracle, one
stream, three arms flat / cubic `= E18's ψ fold` / twist24, Wiedemann to the verified log) on the canonical
models at `p = 241, 337, 541`, `16` walks each fold; `r = LA / rho` per arm; §11.16's handover from the
twist24 arm's own constants (`y`, `cols/p`, `r`, `S_rho`) with §11.17's `C₄ = 1.213·10¹²`.  **No Stage B**
(no `S₅` solve).  Host: the 4-vCPU container, operation counts only.

| id | quantity | prediction | falsified by |
|:--|:--|:--|:--|
| G2.104 | kill test | `|⟨λ_p, λ_ω, −1⟩| = 24` on all eight curves; `ord λ_p = 4·ord χ₆(b)`; a canonical `b₀` of the same order exists for every curve; `u² ∉ F_p` on E18's model (E18's base not kept) | a group of order `< 24` (**kill** for that curve), or any other item |
| G2.105 | Stage R steps | order-6 / order-24 walk steps, pooled per curve, in `[1.7, 2.3]` | outside |
| G2.106 | Stage R cost | order-6 / order-24 walk `F_p` multiplications in `[1.3, 2.0]` on every curve, in both arithmetics | outside |
| G2.107 | E18's `ψ` arm re-read | `r_ψ` against the order-24 walk `= 0.138 × (G2.106 ratio)` and its handover later by `4·log₂(ratio)` bits — a reference correction, reported as such | — (derived from G2.106) |
| G2.108 | Stage A | cubic / twist24 columns `4.00`; LA ratio in `[10, 22]`; `r_24` (twist24 arm against the order-24 walk) in `[0.01, 0.06]`; log recovered on every arm | outside, or any log not recovered |
| G2.109 | handover (model) | from Stage A's twist24 constants, `n* ∈ [2^{139}, 2^{144}]`; above E24's `2^{90.1}` by `≥ 2^{45}` | outside the band |

**What it would say.**  If G2.104–G2.109 hold, E18's `j = 0` rows were measured against a walk weaker than
the one the curve allows: the order-24 walk is the matched reference, and against it E18's `ψ` arm stands
further from rho (a reference correction).  The order-24 base fold then brings the modelled `S₅`-based
handover from `2^{146}` to `≈ 2^{140}`–`2^{144}`: **engineering on the `j = 0` / `S₅` line, a model,
not an improvement of the baseline**, which (E24, E25, E31 on the `V₄` line) sits `≈ 2^{50}` earlier.

### 9.28 E30 pre-registration: "invariants", lite — an accounting audit of trivial `F_{p⁴}` products

*Registered 2026-10-30, before any E30 code.*  **What survives of the draft:** E30 ("invariants") had a full
version with a Stage B and a "lite" version that changed only counters or accounting; its draft touched
`gaudry_quartic.rs` and predicted that the lite version would move E25's distance to rho by only `≈ 0.6 %`.
Only the lite version is to run, as an accounting correction.  **What cannot be reconstructed:** which
counter the draft changed.  The note does not say, and no draft file survives.  So E30 is pre-registered here
as a minimal, honest accounting audit in the same place (`gaudry_quartic.rs`'s `F_{p⁴}` counters) and on the
same object (the `V₄` invariant system and the walks), not as the lost draft.

**The audit.**  `Fp4::mul` charges a full product (`19`, or `13` with E27's products) whatever its operands.
Some operands are structurally trivial, and a counted implementation would not pay for them: the factor `1`
(`v4_system` evaluates the `30` invariant coefficients as `Σ_j c_j z_R^j` from `z_R^0 = 1`, and `Fp4::pow`
starts from `1`), the factor `0`, and factors in `F_p` (a scaling, `4`, as `Fp4::scale` is charged).  E30
re-prices exactly these products: `0` when an operand is `0` or `1`, `4` when an operand lies in `F_p`, the
full product otherwise.  Nothing is computed differently; recorded counts are unchanged (the audit is a
separate counter of the difference).  **Class, fixed in advance: accounting.**  Whatever it moves is not an
improvement of the index calculus.

**Protocol.**  E25's ten curves, both arms, E31's solver A with E27's products (the §9.25 baseline): the
pipeline rerun with the audit counter on, every other field required to equal E27's records; the matched
`⟨±ψ⟩` and negation walks (E25's seeds) re-priced the same way, serially.  Report `S`, the walk, and the
distance to rho at `0`, `0.1` and `1` GAE, before and after the audit.

| id | quantity | prediction | falsified by |
|:--|:--|:--|:--|
| G2.110 | replay | every field of the §9.25 Karatsuba records reproduced; the audit changes only its own counter | any other field differs |
| G2.111 | IC side | `S` falls by `[0.01 %, 1 %]` (trivial products in the invariant system's evaluation and in powering) | outside |
| G2.112 | walk | the `⟨±ψ⟩` walk's cost falls by `[0 %, 1 %]` on E25's curves (its `ψ` multipliers are not expected in `F_p`) | outside |
| G2.113 | distance | `S/rho` moves by less than `1 %` at every weight on every row (red team's estimate for the lost draft: `≈ 0.6 %`) | `≥ 1 %` on any row |

### 9.29 E27 measured: the matched walk was `1.45×` too expensive; every row moves `1.45×` further from rho

*2026-10-30.*  Code `6490d0fac` (with the compile repairs `af2504ef1`), binary `af2504ef1`
(`experiments/32_walk_karatsuba/binary_commit.txt`).  **Stage W:** `examples/e27_walk.rs` on E25's ten
curves, `16` `⟨±ψ⟩` and `16` negation walks each, three arithmetics (`stage_w.{json,log}`).  **IC side:**
`experiments/32_walk_karatsuba/run_e2e.sh` (E31's solver-A command plus `--karatsuba`,
`RAYON_NUM_THREADS=4`), ten curves, both arms (`e2e_chain_karatsuba_p*s*.json`, log).  Tables:
`examples/e27_tables.rs` → `tables.md`, `summary.json`.  Host: the 4-vCPU container (isolab-eks is not
reachable from it); operation counts only, no wall-time claim.

**A finding about the reference before E27 touched it.**  Replaying E25's walks with today's code
reproduced every step, group operation and collision but charged `+31` group operations and `+3,309`
`F_p` multiplications per walk at `p = 101` seed 1.  The difference is the `[d]G = Q` check after the
collision: main's `652238c03` (2026-10-24, the twist lane) began charging it inside `rho4_folded`, which
E25's and E31's records did not.  It is `0.13`–`0.78 %` of a `⟨±ψ⟩` walk on these curves, and it raises
the matched rho of every pipeline run since that commit.  E27 records it per walk (`verify_fp_muls`) and
prices every walk without it, as E25 did.

**Per operation** (`F_p` multiplications): `Fp4::mul` `19 → 13`; affine addition of distinct points
`97` → `79` (Karatsuba, unbatched) → `66.3`–`66.7` (batched: the norm-inversion share is `3.27`–`3.75`
per inversion over the twenty walk pools); doubling `116 → 92` unbatched; the GLS map `ψ` `38 → 26`.

**Stage W.**  All `320` walks gave the same steps, group operations, cycle escapes, recovered logarithms,
correctness and visit digests under the three arithmetics; arithmetic (i) reproduced all `320` E25 walk
records field for field (E25's convention).  The walk's `F_p` cost, (i) over (iii): **`1.4533`–`1.4634`**
(`⟨±ψ⟩` `1.4559`–`1.4634`, negation `1.4533`–`1.4621`); (i)/(ii) (Karatsuba alone) `≈ 1.27`.

**IC side.**  Every field of E31 A's twenty records was reproduced except the multiplication counts and what
derives from them (`call` includes the Weil restriction, which uses `F_{p⁴}` products; `solve_muls` is
unchanged, as it is over `F_p`), wall times, and the walks.  `S_A / S_{A,K}` = **`1.00475`–`1.00566`**: the
non-solve `F_{p⁴}` work is `2.00`–`2.48 %` of `S_A`.  Batching the IC side's `F_p` inversions too
(`4.7·10⁵`–`2.6·10⁷` per run) could save at most a further `0.127`–`0.170 %` of `S` (bound, not built).

**Distance to the matched `⟨±ψ⟩` rho** (`S/rho` over the twenty rows; E31 A against E25's walk, E27 against
walk (iii); band as §9.24, one GAE `= 97` per Macaulay entry and per walk step):

| GAE | E31 A (§9.24) | E27 (A with Karatsuba, walk (iii)) | growth per row |
|--:|:--|:--|:--|
| 0 | `4.42·10³`–`2.79·10⁵` | `6.39·10³`–`4.06·10⁵` | `1.4477`–`1.4564` |
| 0.1 | `1.50·10⁴`–`9.77·10⁵` | `2.13·10⁴`–`1.39·10⁶` | `1.4150`–`1.4427` |
| 1 | `7.33·10⁴`–`5.22·10⁶` | `9.07·10⁴`–`6.64·10⁶` | `1.2364`–`1.3252` |

With the GAE itself re-measured under (iii) (`66.3`–`66.7` instead of `97`) on the E27 side, the `1`-GAE
growth is `1.016`–`1.017`: at that weight the band's price per step dominates both sides.  The best row,
`p = 397` seed 1 arm B (E31's solver A): `4.42·10³ → 6.39·10³×` at `0` GAE.  Rho is ahead of every row
at every weight.

| id | result | verdict |
|:--|:--|:--|
| G2.95 | all `320` walks identical under (i), (ii), (iii); (i) = E25's records; unit tests `e27_tests` (Karatsuba = schoolbook with a perturbed-factor control; batched additions = serial with a perturbed-inverse control; batched walks = serial walks; digests distinguish walks) | **holds** for the runs; the unit tests are run with the session's final test build (§9.32 records the result) |
| G2.96 | `13` exactly; distinct addition `66.3`–`66.7`; per group operation (i)/(iii) `1.453`–`1.463` | **holds** |
| G2.97 | `1.4533`–`1.4634` on every curve, both walks | **holds** (red team's `1.03`–`1.3` was low: every component, not only the addition, falls `1.45×`) |
| G2.98 | `1.00475`–`1.00566`; non-solve share `2.00`–`2.48 %`; every other field reproduced | **holds** |
| G2.99 | `1.448`–`1.456` (0), `1.415`–`1.443` (0.1), `1.236`–`1.325` (1 GAE); rho ahead everywhere | **holds** at `0` and `0.1`; **falsified high at `1` GAE on the two `p = 101` seed 2 rows** (`1.3252`, band `≤ 1.30`): their walks are the shortest (`1,192` steps), so the per-step price is a smaller share |

**What it says.**  The rho reference of §9.14–§9.24 used `19`-multiplication products and one `F_p`
inversion per addition.  A Karatsuba tower and Montgomery-batched norm inversions make the same walks
(same points, collisions and logarithms) `1.45×` cheaper.  **This is a correction of the reference, not a
change to the index calculus: every row is `1.45×` further from rho at `0` GAE than §9.24 said**
(`1.24`–`1.33×` at `1` GAE).  The same products make the IC side `0.5 %` cheaper (engineering, below the
noise of anything else in this note), because its `F_{p⁴}` work is `≈ 2 %` of `S`.  **Distance remaining:**
`6.39·10³`–`4.06·10⁵×` at `0` GAE.  The rho walk is now the stronger reference for E28–E30 and for any
later row on these curves.

### 9.30 E28 measured: the `ψ` fold carries over to E19's line with `h = 4`, and the line stays two orders behind the `V₄` line

*2026-10-30.*  Code `789dc2ff6` (Stage 0, Stage A, registered control; binary `789dc2ff6`,
`experiments/33_tau_stage_a/binary_commit.txt`), supplementary control and tables `0e4fccce1`.  **Stage 0:**
`examples/e28_tau.rs --stage 0`, `40` curves (`stage0.{json,log}`).  **Stage A:** `run_stage_a.sh`, `10` curves
(`stage_a_p*s*.json`, `stage_a.log`).  **Supplementary control:** `--stage c` (`control_decomposed.{json,log}`).
Tables `examples/e28_tables.rs` → `tables.md`, `summary.json`.  Host: the 4-vCPU container; operation counts only.

**One deviation, recorded.**  The registered control drew `3` uniform targets per curve; none had a
decomposition, so its agreement check (`0 = 0` on all `30` calls) was vacuous.  A supplementary control was added
after the run: `3` targets per curve that the exact oracle decomposes (`30` calls), with the same solver.

**Stage 0.**  All `40` curves (`8` seeds × `5` sizes) built; `ψ² = −1`, `ψ(G) = [λ]G`, `λ² ≡ −1`, `ψ(T) = T`
on points on every one; every base point's `ψ`-, `τ_T`- and `τ_Tψ`-image is a base point with the derived action
`u ↦ −u, c²/u, −c²/u`; `4` points a column under `τ_T` and `8` under `⟨τ_T, ψ⟩` (with `±`) on the same points;
`j ∈ F_{p²}` throughout; **`h = 4` on all `40`**; density pooled `0.50`–`0.52` per size.

**Stage A** (same stream, both arms, exact oracle):

| p | seed | cols τ / τψ | residuals to the log τ / τψ | ratio | y τ / τψ | control `C` median (uniform / decomposed) |
|--:|--:|:--|:--|--:|:--|:--|
| 53 | 1 | 8 / 4 | 5,519 / 4,055 | 1.36 | 613 / 811 | `1.36` / `1.37·10⁹` |
| 53 | 2 | 16 / 8 | 748 / 292 | 2.56 | 44 / 32 | `1.39` / `1.38·10⁹` |
| 101 | 1 | 24 / 12 | 2,273 / 1,261 | 1.80 | 91 / 97 | `1.40` / `1.42·10⁹` |
| 101 | 2 | 32 / 16 | 1,183 / 481 | 2.46 | 36 / 28 | `1.37` / `1.40·10⁹` |
| 149 | 1 | 36 / 18 | 4,288 / 2,306 | 1.86 | 116 / 121 | `1.38` / `1.40·10⁹` |
| 149 | 2 | 48 / 24 | 1,570 / 696 | 2.26 | 32 / 28 | `1.40` / `1.48·10⁹` |
| 197 | 1 | 42 / 21 | 8,804 / 4,738 | 1.86 | 205 / 215 | `1.38` / `1.41·10⁹` |
| 197 | 2 | 52 / 26 | 4,307 / 2,166 | 1.99 | 81 / 80 | `1.41` / `1.46·10⁹` |
| 269 | 1 | 80 / 40 | 3,493 / 1,986 | 1.76 | 43 / 48 | `1.44` / `1.44·10⁹` |
| 269 | 2 | 66 / 33 | 8,564 / 5,364 | 1.60 | 128 / 158 | `1.38` / `1.41·10⁹` |

Every log recovered on both arms; all `640` walks correct; decompositions per decomposed target: median `8`
on every curve.  Pooled: residuals `40,749 / 23,345` (`1.75`), `y` `98.4 / 110.1` (`1.12`).  The `30`
supplementary calls returned exactly the oracle's decompositions (`2` or `8` each).

**Stage A2 (a model, not a measured `S`).**  `S_model = R·C + R·g + 16·LA` (`R` the arm's residuals to its log,
`C` the control's median call, `g` the measured generation per target), against the matched walk in E25's
arithmetic: the τψ arm `6.99·10⁵`–`1.74·10⁷×` the `⟨±ψ⟩` walk at `0` GAE (`7.32·10⁵`–`1.92·10⁷` at `0.1`,
`9.09·10⁵`–`3.19·10⁷` at `1`); against E27's walk (iii) `1.02·10⁶`–`2.55·10⁷` at `0` GAE.  On the same curve,
the τψ arm against the `⟨±ψ⟩` walk is `1.30`–`2.82×` below the τ arm against the negation walk.  Against E19's
measured `S/rho` at the same `p` (E19's other curves, mean of its two seeds): `0.45`–`6.98×` per curve; by size
(geometric mean of the two seeds) `1.77, 4.29, 2.39, 1.83, 3.65` at `p = 53 … 269`.

| id | result | verdict |
|:--|:--|:--|
| G2.100 | every check on all `40` curves; `h = 4` on all `40`; density `0.50`–`0.52` | **holds** (gate passed) |
| G2.101 | columns `2.00`; decompositions per target median `8`; `y` ratio pooled `1.12`; residuals ratio pooled `1.75`; every log recovered | **holds** (pooled, as registered; per curve the residuals ratio is `1.36`–`2.56`, the noise of `8`–`80` columns) |
| G2.102 | `C` median `1.36`–`1.44·10⁹` (uniform) and `1.37`–`1.48·10⁹` (decomposed); agreement on all `30` decomposed calls (the registered uniform calls agree vacuously) | **holds**, with the deviation above |
| G2.103 | E19/E28 `0.45`–`6.98` per curve, by size `1.77`–`4.29`; E28's best row (`6.99·10⁵`) against E31 A's rows at the same `p`: `≥ 16.6×` at `p = 101`, `≥ 52×` at `197`, `≥ 21×` at `269`, but only `3.3×` above E31 A's worst row at `p = 149` (`2.79·10⁵`, a high-`y` curve) | **falsified** per curve and at `p = 101` by size (`4.29 > 4.0`); holds by size at the other four; the `≥ 10×` gap to E31 A **fails at `p = 149`** under the strict reading (best E28 row against worst E31 row), holds at the other matched sizes and by a factor `≈ 100` between medians |

**What it says.**  On GLS twists over `F_{p⁴}` built on E19's line (`σ(x₀) = −x₀`, `c ∈ F_p`), `ψ` keeps `T` and
the line and acts as `u ↦ −u`, and every one of `40` curves has `h = 4`: the cofactor penalty §9.17 found on
the `V₄` line does not arise here.  The fold halves the columns exactly and cuts the residuals to the log
`1.75×` (pooled) with an unchanged solve (`C ≈ 1.4·10⁹`, the same decompositions).  Modelled with that solve, the
line moves by `≈ 2`–`4×` against its own matched walks relative to E19.  **Class: engineering on the `D₄`/`τ_T`
line, and a model (Stage A2), not a measured end-to-end `S`; not an improvement of the baseline.**  **Distance
remaining (model):** `≥ 7·10⁵×` the matched rho in E25's arithmetic, `≥ 1.0·10⁶×` against E27's walk; E31 A on
E25's curves is two orders of magnitude closer.  A Stage B is not justified: what this line needs is a cheaper
solve, which the `V₄` line already has.

### 9.31 E29 measured: `ψ_p` has order `24` on all of E18's curves; the order-24 walk as built is not cheaper; the order-24 base fold moves the `S₅` handover by `≈ 10` bits, to `2^{135}`–`2^{142}`

*2026-10-30.*  Code and binary `0e4fccce1` (`experiments/34_twist24_stage_r/binary_commit.txt`):
`examples/e29_twist.rs` (`kill.{json,log}`, `stage_r_*.{json,log}`, `stage_a_p{241,337,541}.{json,log}`), tables
`examples/e29_tables.rs` → `tables.md`, `summary.json`.  Host: the 4-vCPU container; operation counts only.
Total compute: under `10` core-minutes (the red team budgeted `≈ 0.3` core-hours for one curve's Stage R).

**Kill test** (E18's own models, `generate_psi_instance(p, 1)`): on all eight curves `ψ_p = (u²x^p, u³y^p)`,
`u = b^{−(p−1)/6}`, maps `E` to `E`; `λ_p` is a 24th root of unity mod `n` of **order `24`**, `χ₆(b)` has order
`6` (so `ord λ_p = 4·ord χ₆(b)`, as derived), `λ_ω` order `3`, and `|⟨λ_p, λ_ω, −1⟩| = 24`.  `u² ∉ F_p` on every
E18 model (its base is not kept).  Each curve has a canonical model `y² = x³ + b₀t³` of the same order (least
`b₀`: `3, 3, 3, 9, 9, 2, 17, 3`).  **Not killed.**

**Stage R** (canonical models, `16` walks per order, both arithmetics, all `256` walks correct): steps order 6 /
order 24 per curve `1.29`–`2.10`, pooled per `√n` **`1.701`**; but the order-24 walk's class representative (three
Frobenius images, twelve candidates) costs `191`–`210` `F_p` multiplications a step against `109`–`122` (schoolbook),
so its `F_p` cost ratio, order 6 / order 24, is `0.743`–`1.202` per curve, **`0.962` pooled** (Karatsuba: `0.684`–
`1.107`, pooled `0.884`).  `S_rho` per `√n`: `52.9` (order 6) and `55.0` (order 24), schoolbook; E18's `ψ` walk
measured `54.1`.

**Stage A** (the lane's `run_k4_twist24`, E18's exact oracle, one stream, flat / cubic / twist24 arms, logs
recovered on all nine arms, all walks correct):

| p | log₂ n | cols cubic / twist24 | LA cubic / twist24 | y cubic / twist24 | `r` twist24 (order-24 walk) | `n*` cubic | `n*` twist24 (order-6 / order-24 walk) |
|--:|--:|:--|--:|:--|--:|:--|:--|
| 241 | 31.7 | 32 / 8 | 13.3 | 79.9 / 65.2 | 0.0058 | `2^{151.9}` | `2^{142.2}` / `2^{141.8}` |
| 337 | 33.6 | 60 / 15 | 15.2 | 19.8 / 15.6 | 0.0103 | `2^{145.7}` | `2^{135.3}` / `2^{135.0}` |
| 541 | 36.3 | 88 / 22 | 13.9 | 29.8 / 27.8 | 0.0083 | `2^{147.9}` | `2^{138.8}` / `2^{137.9}` |

(`n*` is §11.16's model with §11.17's `C₄ = 1.213·10¹²` and each arm's own `y`, `cols/p`, `r` and walk; an
extrapolation, driven mostly by `y`, which here rests on `9`–`89` relations.)

| id | result | verdict |
|:--|:--|:--|
| G2.104 | group of order `24` on all eight; `ord λ_p = 24 = 4·ord χ₆(b)`; canonical `b₀` for every curve; `u² ∉ F_p` everywhere | **holds** (not killed) |
| G2.105 | pooled `1.701`; per curve `1.29`–`2.10` (five of eight below `1.7`) | **holds pooled** (at the band's edge); per curve falsified (16 walks a curve) |
| G2.106 | `F_p` ratio `0.74`–`1.20` (schoolbook), `0.68`–`1.11` (Karatsuba); pooled `0.96` / `0.88` | **falsified**: as built, the order-24 walk costs more than the order-6 walk; its representative costs `1.8×` a step |
| G2.107 | the matched reference stays the order-6 walk (cheaper pooled); on three curves (`p = 601, 997, 1069`, schoolbook) the order-24 walk is `1.04`–`1.20×` cheaper, which would move E18's handover there later by `≤ 1.1` bits | no correction of E18's `ψ` rows at the pooled level |
| G2.108 | columns `4.00`; LA `13.3`–`15.2`; `r_24` `0.0058`–`0.0103`; every log recovered | **holds** except `r_24`, **falsified low** at `p = 241, 541` (below `0.01`) |
| G2.109 | `n*` twist24 `2^{135.0}`–`2^{142.2}`; `2^{44.9}`–`2^{52.1}` above E24's `2^{90.1}` | **holds at `p = 241`**, **falsified low** at `337`, `541`; the `≥ 2^{45}` clause misses by `0.1` bit at `p = 337` |

**What it says.**  The red team's question has a clean answer: on every one of E18's `j = 0` curves the
Frobenius-derived `ψ_p` has order `24` and, with the automorphisms, acts as a cyclic group of order `24` on
`⟨G⟩` (proved in §9.27 and checked on points).  Two consequences differ.  **The walk:** an order-24 walk takes
`1.70×` fewer steps, but with the lane's class representative it pays more than that per step, so it is not a
stronger reference; E18's `ψ`-arm distances stand.  A cheaper representative (comparing abscissae first, `x`-only
images) is the obvious next step and is not built here.  **The base:** on the canonical model the order-24 fold
divides the columns by `4.00` and the linear algebra by `13`–`15` against E18's cubic fold, and moves the modelled
`S₅` handover by `≈ 9`–`10` bits on the same curves, to `2^{135}`–`2^{142}`.  **Class: engineering on the `j = 0`
/ `S₅` line, and a model (no `S₅` solve was run); not an improvement of the baseline**, whose modelled handover
(E24, `2^{90.1}`; E25 `2^{88.0}`) is `≥ 2^{45}` earlier.

### 9.32 E30 measured: trivial products are `0.31 %` of `S`; the distance to rho moves `−0.31 %` at most — an accounting correction

*2026-10-30.*  Code and binary `0e4fccce1` (`experiments/35_invariants_lite/binary_commit.txt`).  Walk audit
`examples/e30_audit.rs` (`walk_audit.{json,log}`); IC side `run_e2e.sh` (E27's Karatsuba solver-A runs plus
`--audit`; `e2e_audit_p*s*.json`, log, finished `2026-10-30T22:23:38Z`); tables `examples/e30_tables.rs` →
`tables.md`, `summary.json`.  Host: the 4-vCPU container; operation counts only.  As §9.28 states, this is a
minimal accounting audit registered in place of the lost "invariants-lite" draft, not that draft.

**Results.**  All twenty audited records reproduce E27's Karatsuba records in every field except wall times and
the new counter (the walks are re-run and not compared here; E27 established them).  The audit re-prices
`0.3087`–`0.3131 %` of `S` away on every row (`1.4·10⁷`–`7.1·10⁸` `F_p` multiplications per run): products by
`0`, `1` or an element of `F_p` in the IC side's `F_{p⁴}` work, which here is `≈ 2 %` of `S`, so about one in seven
of its counted products is trivial.  The `⟨±ψ⟩` walk's saving is `0`–`1.1` multiplications per walk
(`≤ 0.0002 %`): its `ψ` multipliers are not in `F_p` on any of the ten curves.  The distance to the matched walk
(E27's (iii)) moves by **`−0.309`–`−0.313 %`** at `0` GAE, `−0.084`–`−0.086 %` at `0.1` and `−0.011 %` at `1` GAE.

| id | result | verdict |
|:--|:--|:--|
| G2.110 | every non-count field of E27's records reproduced on all `20` | **holds** |
| G2.111 | `S` falls `0.309`–`0.313 %` | **holds** |
| G2.112 | walk `≤ 0.0002 %` | **holds** |
| G2.113 | largest change `0.313 %` (red team's estimate for the lost draft: `≈ 0.6 %`) | **holds** |

**What it says.**  Pricing a product by `0`, `1` or an `F_p` scalar as such moves `S` by `0.31 %` and the walk
by nothing measurable.  **Class: accounting** — no algorithm changed, and it is not an improvement.  It is
recorded so that a later counter change of this kind is not mistaken for progress.  Of comparable size, and
also an accounting effect, is main's change of 2026-10-24 (§9.29): charging the walks' `[d]G` check moved
every matched rho of these curves by `0.13`–`0.78 %` in the other direction.

**Unit tests (E27–E30), run in one build at the end of the session:** recorded in the session log and the PR.

### 9.33 E27–E30 validated: errata, and a §5a qualification on E29's reference

*2026-10-31.*  Each experiment was replayed by an independent validator from a fresh build of its recorded binary
commit, every non-timing field compared, and its headline figures recomputed by a tool that shares no producer
code (operation counts only; local container, no wall time admitted).  §9.29–§9.32 stand as recorded; the
following are additive corrections.

| | evidence | replay | headline figures | gate |
|:--|:--|:--|:--|:--|
| E27 | `research/e27_walk_20261030/` (`VAL-20261031-e27a1c`) | 11/11 runs, 23,144 fields identical | 59 agree | **valid** |
| E28 | `research/e28_tau_20261030/` | 12 records, 8,690 fields identical | 224/226 agree | **refused**: end-to-end `S` is a model |
| E29 | `research/e29_twists_20261030/` | 6 records, 26,124 fields identical | 265/265 agree | not submitted: no index-calculus cost |
| E30 | `research/e30_invariants_lite_20261030/` | 11 records, 12,281 fields identical | 174/174 agree | **valid** |

E27 and E30 are on the IC leaderboard as a reference correction and an accounting audit.  E28 and E29 are named
there as not shown.  Their measured stages are verified; their deciding quantities are models.

**E27 errata** (none changes a verdict):

1. §9.29 Stage W, "(i)/(ii) ≈ 1.27" holds for the `⟨±ψ⟩` walk (`1.265`–`1.283`); the negation walk is
   `1.230`–`1.241`.  The registered quantity (i)/(iii) reproduces exactly.
2. §9.29, "batching the IC side's inversions could save at most `0.127`–`0.170 %`" is the saving at the walk's
   measured batch share.  The upper bound under the batch cost `3(m−1)+16` is `0.130`–`0.180 %` of `S`.
3. In `experiments/32_walk_karatsuba/e2e_chain_karatsuba_p*s*.json`, `rho_matched_muls` is walk (ii)
   (Karatsuba, unbatched) *with* the `[d]G` check, and `ratio` is `S` over that.  Neither is E27's distance,
   which uses walk (iii) without the check.  Read from those records, the distance is understated at `0` GAE by
   `1.137`–`1.166×`.  The records are not changed; this note and the leaderboard use walk (iii).

**E28 errata** (none changes a verdict):

1. §9.30 A2, by size: `1.77`, `2.39` are `1.78`, `2.40` (geometric means of rounded per-curve values were
   printed).  All stay in `[1.4, 4.0]`, and `p = 101` stays above `4.0` (`4.287`).
2. §9.30 G2.103, "`≥ 52×` at 197" is `51.6×`.
3. §9.30, "`≥ 7·10⁵×` the matched rho in E25's arithmetic": the least row is `6.99·10⁵×`.  Against the
   strongest walk (iii) every row is `≥ 1.02·10⁶×`.
4. §9.30 G2.103, "≈ 100 between medians" is `60`–`112×` depending on which E31 solver-A rows are pooled (arm A
   `59.9`, all 20 rows `79.6`, arm B `111.5`).
5. The registered A2 model prices each call at the control's *median* `C`; E19's measured `S` is priced at the
   mean.  Applied to E19's own rows the median formula under-prices them by `1.03`–`1.17×`, so E28's modelled
   `S` is optimistic.  With E28's 3-call mean, E19/E28 by size becomes `1.48`, `4.20`, `2.40`, `1.82`, `3.02`.
   G2.103's falsifications, including the `≥ 10×` failure at `p = 149`, stand under either pricing.

The control deviation of §9.30 (30 uniform targets had no decomposition, so the registered agreement check was
vacuous; 30 decomposable targets added) is disclosed and adequate.  The uniform calls are a negative control
(≈ 0.45 decomposable expected), and the added calls match the oracle's decomposition sets exactly; three calls
per curve is a small sample.  The validator confirmed independently that `⟨±ψ⟩` is the largest fold on all 40
curves (`j ∈ F_{p²} \ F_p`, `Aut(E) = {±1}`, `ψ² = −1`).  So walk (iii) is the strongest reference; §9.30's
headline uses walk (i).

**E29 erratum and §5a qualification.**  §9.31's "the matched reference stays the order-6 walk" and "E18's
`ψ`-arm distances stand" hold **at 0 GAE only**.  With one native operation per walk step priced (E27's
convention), the pooled order-6/order-24 cost ratio is:

| arithmetic | 0 GAE | 0.1 GAE | 1 GAE |
|:--|--:|--:|--:|
| schoolbook | `0.962` | `0.996` | `1.205` |
| Karatsuba | `0.884` | `0.927` | `1.172` |

At `1` GAE the order-24 walk is cheaper on 7 of 8 curves.  The order of the two walks changes inside the band,
so neither is the stronger reference across it.  The order-24 walk also has unemitted per-step work (12
candidate comparisons) that the band does not price, so the ranking is **unresolved, not reversed**.  E18's
`ψ`-arm distances are lower bounds at `1` GAE: a cheaper rho there lengthens them.  The kill
test was re-derived on the canonical models `y² = x³ + b₀t³`: `⟨λ, λ_ω, −1⟩` has order 24 on all 8 curves and
`n` equals E18's.  E18's own coefficients are not in the records, so "order 24 in E18's normalisation" rests on
the replayed binary check.

**E30 erratum.**  §9.32's "about one in seven of its counted products is trivial" cannot be checked: product
counts are not emitted.  The `F_{p⁴}`-priced phases are `1.53`–`1.90 %` of `S` ("≈ 2 %"), and the saving is
`0.164`–`0.202` of their multiplications.

**Where the line stands.**  Against the strengthened walk (iii), the `k = 4` `V₄`/GLS line with E31's solver A
is `6.39·10³`–`4.06·10⁵×` rho at `0` GAE, rho ahead at every weight.  The modelled handover (`2^{88}`–`2^{90}`) is
an extrapolation from `p = 101`–`521` (`22`–`32`-bit `n`), and every `S` above is `≥ 99 %` solve.

### 9.34 E32 pre-registration: the handover ladder — the crossover order fitted from many sizes, with intervals

*Registered 2026-10-31, before any E32 run.*  **Numbering:** E32 owns §9.34 (this registration), §9.36
(results) and G2.114–G2.123; E33 owns §9.35, §9.37 and G2.124–G2.133.  **Question.**  §9.33's handover
(`2^{88}`–`2^{90}`) is §11.16's model applied to constants from four or five sizes (`n = 2^{22}`–`2^{32}`).
The lead atlas (`LEAD-ICE-charged-toy-ic-exponent-vs-rho`) records that a four-size ratio exponent was
unidentifiable and asks instead for a fit of the crossover order `log l*`, the size at which
`S_IC = S_rho`.  E32 measures the line's current best configuration and its matched rho on a ladder of
sizes, two decades wider in `n` than any run so far, and fits `log₂ n*` directly, with intervals, at
`0`, `0.1` and `1` GAE.  **Class, fixed in advance:** a *measurement of distance* (and a fit of where it
reaches `1`); nothing in E32 changes either arm, so no outcome is an improvement, and a fitted `n*`
beyond the measured range is an extrapolation, reported as such.

**The arms (unchanged code, pinned binaries).**  *IC:* the `k = 4` `V₄`/GLS index calculus on E25's
curves, arm **B** (the `⟨V₄, ψ⟩`-folded base, the line's best arm), E31's solver **A** (`chain`), E27's
Karatsuba `F_{p⁴}` products: exactly `quartic_pipeline_gls --solver chain --karatsuba` restricted to arm
B (`run_k4_gls_pipeline_with`, unchanged), every phase charged as in E25/E31.  *Matched rho:* E27's walk
(iii) — the `⟨±ψ⟩` walk, Karatsuba products, norm inversions batched over a pool of `16` walks with
fillers — `16` walks per curve, seeds `1000·seed + k`, cost per walk **without** the `[d]G = Q` check
(E25's and E27's convention), `rho` = mean over the `16` walks.  Both arms run through
`examples/e32_ladder.rs` (modes `ic` and `walk`), which calls only those two unchanged library
functions.  Binaries: built from `e75771052` (build inputs identical at `45378560dd`), copied to the
session scratchpad and run only from there; sha256 in `experiments/36_handover_ladder/binary_commit.txt`.

**Sizes, curves, seeds.**  `p ∈ {101, 149, 197, 269, 397, 557, 773, 1093, 1549, 2213, 3089}` (`log₂ n ≈
22.6`–`42.4`, eleven sizes, `≈ 2` bits apart above `p = 397`), and `p = 4337` (`2^{44.3}`) only under the
budget rule below.  At each size three curves, `generate_gls_klein_instance(p, seed, 16)` with seeds
`1, 2, 3`; a seed that yields no curve is replaced by the next unused seed (`4, 5, …`), recorded.  At the
five E25 sizes seeds `1, 2` are E25's curves, and their runs (with `--rho-runs 16`) are also the identity
check against E27's records; elsewhere `--rho-runs 1` (the pipeline's own walks are not used by E32).
The target stream is the pipeline's (seeded by the curve seed); the replicate unit for intervals is the
curve.  Per curve: `S` (`total_muls`), Macaulay entries, `C`, `y`, columns, phase shares; per walk: steps,
`F_p` multiplications, correctness, visit digest.

**Distance and band (§5a).**  `S(g) = S + 97 g · E` (`E` = Macaulay entries), `rho(g) = rho + 97 g ·
steps` (mean steps), `D(g) = S(g) / rho(g)`, `g ∈ {0, 0.1, 1}`, one GAE `= 97` (as §9.24 and §9.29).
Rho's own unpriced native work is priced by the same rule (one per step).  A row is "ahead of rho" only
if `D < 1` at all three weights.

**Fit (frozen).**  Per curve `c`: `x_c = log₂ n_c − 32`, `L_c(g) = log₂ D_c(g)`.
- **M1 (primary):** OLS `L = α + β x` on the curves with `p ≥ 269` (where the walk's set-up — `66`
  scalar multiplications, `≈ 13 %` of a walk at `2^{30}` and falling — no longer bends `rho`); the
  crossover order `log₂ n* = 32 − α/β` (defined for `β < 0`).
- **M1-all (sensitivity):** M1 on every curve.
- **M2 (sensitivity, theory slope):** `β` fixed at `−1/4` (`S ∝ p`, `rho ∝ p²`), `α` the mean of
  `L + x/4` over `p ≥ 269`; `log₂ n* = 32 + 4α`.
- **Intervals:** cluster bootstrap over curves, stratified by size (resample each size's curves with
  replacement), `10,000` replicates, fixed RNG seed `32`; 95 % percentile intervals for `β`, `α` and
  `log₂ n*`; the fraction of replicates with `β ≥ 0` (no crossover) is reported, and those replicates
  count as `n* = ∞` in the percentiles.  Also reported: the OLS standard-error interval for `β`.
- **Measured bound:** if no row has `D(g) ≤ 1`, the crossover is *empty in the reachable range* and is
  recorded as `n* > 2^{x_max}` with `x_max` the largest `log₂ n` measured, beside the fit.  The linear
  algebra is `≤ 0.002 %` of `S` at `p ≤ 397`; its share is reported per curve and its `∝ p²` term
  is not modelled (sensitivity: its extrapolated share at the fitted `n*`).

**Route, budget, stopping.**  isolab-eks is the default and was checked: unreachable from this container
(no `aws` or `kubectl`; the GitHub `isolab-experiment.yml` dispatch is refused by the session proxy for
both repository paths).  So: local, **operation counts only, no wall-time claim**, at most `3` of the `4`
cores (`taskset -c 1-3`; `RAYON_NUM_THREADS=4`, which fixes the pipeline's target stream as in §9.19), one
curve at a time.  Cost estimate from E27's arm-B walls (`≈ 2` ms per target on three cores, `≈ 0.07 p`
columns, median `y ≈ 1,500`): `≈ 0.2 p` seconds per curve at the median, `≈ 1.6` h for the six new sizes,
`×2`–`×10` on a heavy-tailed `y` curve.  **Budget:** `14` h of wall time for all E32 runs; a curve still
running after `4` h is stopped and recorded as censored (not in the fits; reported).  Sizes run in
ascending order, all three curves of a size before the next; a size is not started if the budget left is
under `3×` the previous size's median curve time `× p_next / p_prev`.  `p = 4337` runs only if, after
`p = 3089`, `≥ 3` h of budget remain and `≥ 8` GB of memory are free (its 16-walk pool holds `≈ 6` GB).
No size, curve or seed is dropped or added for any reason other than these rules.

| id | quantity | prediction | falsified by |
|:--|:--|:--|:--|
| G2.114 | identity (**kill**) | `ic` reproduces every field of E27's ten arm-B records (`experiments/32_walk_karatsuba/e2e_chain_karatsuba_p*s*.json`) apart from wall times; `walk` reproduces E27's `rho_e27` and `walk_steps` (`summary.json`) on the ten curves exactly; at every size every log recovered, every kept row re-added, every walk correct | any difference or failure (E32 then stops) |
| G2.115 | curves and base | every `(p, seed)` yields a curve with `h = 16`; columns / `p` in `[0.04, 0.10]` on every curve with `p ≥ 557` | a size without three curves, or outside |
| G2.116 | per-call cost | `C` median in `[6.2, 6.9]·10⁵` and Macaulay entries per residual in `[1.70, 1.85]·10⁵` on every curve with `p ≥ 557` | outside |
| G2.117 | phase shares | (Weil + solve) / `S` `≥ 98 %` and linear algebra `≤ 0.1 %` of `S` on every curve | outside |
| G2.118 | yield | OLS exponent of `y` in `p`, all curves, in `[−0.5, 0.5]`; per-size median `y` in `[400, 6,000]` | outside |
| G2.119 | matched rho | steps / `√n` in `[0.4, 0.9]` per curve (mean of `16`); `F_p` multiplications per step (set-up included) falling with `p`, in `[66, 80]` at every curve with `p ≥ 1549` | outside |
| G2.120 | distance reached | per-size median `D(0)` falls with `n`; at the largest size reached the median `D(0)` is in `[3·10², 5·10³]`; rho ahead of every row at every weight (**the crossover is empty in the reachable range**) | outside; any row with `D ≤ 1` at any weight |
| G2.121 | slope (M1, `0` GAE) | `β` in `[−0.30, −0.15]`; its 95 % interval contains `−0.25` | outside, or `−0.25` excluded |
| G2.122 | crossover order at `0` GAE | M1: `log₂ n*` in `[74, 96]`, 95 % interval wider than `8` bits; M2: `log₂ n*` in `[76, 90]`, 95 % interval at most `6` bits wide | any of the four outside |
| G2.123 | crossover across the band | M2 `log₂ n*` later than at `0` GAE by `[5, 9]` bits at `0.1` GAE and `[11, 17]` bits at `1` GAE (derived: per call `E/C ≈ 1.78·10⁵ / 6.45·10⁵`, per step `≈ 70`; `D` grows `≈ 3.2×` and `≈ 11.6×`); `D(0) < D(0.1) < D(1)` on every row | outside, or the order breaks on a row |

**What it would say.**  If G2.114–G2.123 hold, the line's distance to the matched rho is measured over
`2^{22.6}`–`2^{42.4}` (or `2^{44.3}`), falls as `≈ n^{−1/4}` with an interval on the exponent, and still
exceeds `≈ 10³` at the largest size: the crossover is empty in the reachable range, recorded as a bound
`n* > 2^{42}`, and the fitted crossover order with its interval replaces §9.33's single-point
extrapolation.  A free-slope interval spanning more than a decade of bits says the slope, not the
constant, limits what the ladder can tell; the theory-slope fit says where the constants put it.
Neither moves the line toward rho: the distance remaining is reported as measured.

### 9.35 E33 pre-registration: solver B's pivot selection, counted (and two cheaper selections)

*Registered 2026-10-31, before any E33 code.*  **Numbering:** E32 (a parallel experiment in this
checkout) owns §9.34, §9.36 and G2.114–G2.123; E33 takes §9.35 (this registration), §9.37 (results) and
G2.124–G2.133.  **Motivation:** §9.24.  Solver B (`echelon_by_lead_with(…, sparsest = true)`,
`gaudry_quartic.rs`) counts the nonzeros of every row of the built matrix before eliminating, compares
the counts of every bucket's candidates, and recounts each updated row's tail after every row update.
None of that is counted or emitted; the validator priced the first pass alone from a lower bound
(`2 E_E25 − E_E31` entry reads) and found B behind E25 at `0.1` and `1` GAE, so B's rank is unresolved
and B is not a baseline.  E33 counts the whole pass and, separately, builds two cheaper selections.
**Class, fixed in advance:** counting B's work is *accounting*; only a variant that is cheaper than A
with its own selection work counted, at weights where the order holds across the band, would be an
improvement.

**Counting rule.**  One native operation per Macaulay-matrix entry examined for its zero status and
one per comparison of two candidates' keys (the convention of §9.16–§9.24: one per Macaulay entry,
priced at `g · 97` `F_p` multiplications for `g ∈ {0, 0.1, 1}`).  Index bookkeeping that A also does
(the bucket sort, the lead scan `row[c+1..].position(≠ 0)`, the swap) is common to every arm and is not
pivot selection; it is emitted separately as `lead_scan_reads` and used only in the sensitivity G2.133.

- **B, counted** (`chain-sparsest-counted`; the same code as B and the same choices): (i) the initial
  pass, `rows · cols` per echelon built; (ii) `|bucket| − 1` comparisons in each column whose bucket
  holds more than one row; (iii) the recount, `cols − l` reads per updated row with new lead `l`.
- **B′, incremental** (`chain-sparsest-inc`; the same choices, by construction and checked): (i) each
  row's count is inherited from its equation (a monomial shift is injective on terms), at one operation
  per coefficient of each triangularised equation examined plus one per row; (ii) as B; (iii) during
  each row update, the zero status of each updated entry before and after (two operations per entry the
  pivot row touches) plus one per updated row.
- **B″, estimate** (`chain-sparsest-est`; possibly different choices, so its `C` is measured): (i) as
  B′; (ii) as B; (iii) the Markowitz fill bound `min(nnz − 1 + |pivot| − 1, cols − l)`, two operations
  per updated row and no entry read.  Any pivot rule spans the same row space with the same pivot
  columns, so the standard monomials, normal forms and decompositions must not change.
- **A, counted** (`chain-counted`): A with `lead_scan_reads` emitted (no selection work), for G2.133.

A fallback (the existing solver rerun) adds no selection work; the chain attempt's selection work is
charged.  All new counting sits behind these new solver names; `macaulay`, `chain` and
`chain-sparsest` are unchanged, and a test shows E25, A, B and E27 outputs bit-identical.

**Protocol.**  *Stage A:* E31's setting exactly (`examples/e31_stage_a.rs`): E25's ten curves, the first
1,000 targets of E25 arm A's stream per curve, one thread, every solver interleaved per call from the
same per-target RNG seed: existing, A, A counted, B, B counted, B′, B″.  Per call: outputs, degree,
outcome, decomposition set, every phase count, the pivot-selection work by part, `lead_scan_reads`, and
a digest of the pivot row chosen in every column.  *End to end:* `quartic_pipeline_gls` on E25's ten
curves, both arms, with `--karatsuba` (E27), `RAYON_NUM_THREADS=4` (the target stream), for B counted
and B′; B″ only if, in Stage A, its per-call `S` at `0.1` GAE (`C + 9.7·(entries + selection work)`) is
below A's on every curve.  *Reference:* A is E27's records (A with Karatsuba); E25 is its own records
(schoolbook), compared with B's schoolbook `S` (E31 B's records plus the selection work measured here,
which does not depend on the `F_{p⁴}` product); the matched rho is walk (iii) of E27 without the
`[d]G` check (`experiments/32_walk_karatsuba/summary.json`, `rho_e27`, `walk_steps`).  Band:
`S(g) = total_muls + 97 g (macaulay_entries + selection work)` and `rho(g) = rho + 97 g · steps`.  A rank
is stated only where the order holds at `0`, `0.1` and `1` GAE on every row.  Host: the 4-vCPU
container, at most 2 cores (isolab-eks is not reachable from it: no AWS CLI, the GitHub route refused);
operation counts only, no wall-time claim.

| id | quantity | prediction | falsified by |
|:--|:--|:--|:--|
| G2.124 | identity (**kill**) | B counted, B′, B″ and A counted return the existing solver's decomposition set, outcome and degree on every Stage A call and every end-to-end record; B counted and B′ choose B's pivot in every column (equal digests) and reproduce B's phase counts; A counted reproduces A's; the end-to-end B counted and B′ records reproduce every field of each other apart from the selection fields and the solver tag, and every non-product field of E31 B's records; rerunning `macaulay`, `chain`, `chain-sparsest` and E27's `chain --karatsuba` at `p = 101` seed 2 reproduces the stored records byte for byte (wall times aside); unit tests: B′'s inherited and incremental counts equal a recount on every row of real E25 matrices.  Negative controls: a pivot moved to the second-sparsest candidate in one column changes the digest and is flagged; an incremental count off by one on one row is flagged by the recount check; E31's perturbed-coefficient control still changes non-empty sets | any difference, or a control not detected (E33 then stops) |
| G2.125 | B's selection work per call (Stage A mean, all calls) | initial pass exactly `rows · cols` per echelon (`393 · 448 = 176,064`); comparisons `≤ 2 %` of the total; recount `>` initial pass; total in `[3·10⁵, 2·10⁶]` | outside |
| G2.126 | B′'s selection work | B′ / B per call in `[0.15, 0.8]` | outside |
| G2.127 | B″ | selection work `≤ 2·10⁴` per call; `C` median B″ / B in `[1.0, 1.3]` on every curve | outside |
| G2.128 | B against A, counted (end to end, Karatsuba) | `S_A / S_B > 1` at `0` GAE and `< 1` at `0.1` and `1` GAE on every row: the order flips inside the band, so B is not ranked ahead of A | B ahead of A at `0.1` or `1` GAE on any row |
| G2.129 | B against E25 | `S_E25 / S_B > 1` at `0`, `≤ 0.899` at `0.1` and `≤ 0.544` at `1` GAE on every row (the validator's lower bound; the recount only adds) | any row above those bounds |
| G2.130 | B′ against A and E25 | behind A on every row at `0.1` and `1` GAE; behind E25 on every row at `0.1` and `1` GAE | B′ ahead of A or E25 at `0.1` or `1` GAE on any row |
| G2.131 | B″ against A and E25 (Stage A per call; end to end if carried) | ahead of A at `0` and `0.1` GAE and behind at `1` GAE (unranked against A); ahead of E25 at every weight | any other order |
| G2.132 | distance to the matched rho (walk (iii)), B counted | `S_B / rho` at `0` GAE in `[3.0·10³, 2.5·10⁵]`; rho ahead of every row of B, B′ and B″ at every weight | outside; any row below rho |
| G2.133 | sensitivity: `lead_scan_reads` priced for every arm (Stage A) | changes no rank decided by G2.128–G2.131 | a rank changes |

**What it would say.**  If G2.124–G2.130 hold, B's advantage over A and E25 exists at `0` GAE only:
its pivot selection reads the matrix again, and any operation per entry costs more at `0.1` GAE than
the `≈ 3·10⁵` multiplications per call that sparser pivots save.  A stays the ranked solver.  A
selection that costs `O(1)` per row update (B″) could keep part of B's saving at `0.1` GAE, but at
`1` GAE its comparisons and updates (thousands per call, `97` multiplications each) should exceed the
saving; if so, no sparsest-pivot rule here ranks ahead of A across the band, and the next lever is the
echelon's arithmetic itself, not its pivot order.

### 9.36 E32 measured: the distance to rho over `2^{22.6}`–`2^{42.4}`, falling as `n^{−0.24}`; no crossover reached; fitted crossover order `2^{85.5}` `[2^{79.2}, 2^{94.3}]` at `0` GAE

*2026-10-31 to 2026-11-01.*  Binaries pinned from `e757710521` (`experiments/36_handover_ladder/binary_commit.txt`),
run only from the scratchpad copies (sha256 re-verified after a container restart).  Runner
`experiments/36_handover_ladder/run_ladder.sh` (it replaced the pre-registration's `run_ladder.py` after the three
`p = 101` curves, with the same logic, because AGENTS.md forbids Python orchestration); records `ic_p*s*.json`,
`walk_p*s*.json`, log `run.log`; tables and fits `examples/e32_tables.rs` → `tables.md`, `summary.json`, `fit.json`.
Report: `research/e32_handover_ladder_20261031/`.  **Route:** local 4-vCPU container, `taskset -c 1-3`,
operation counts only, **no wall-time claim** (isolab-eks unreachable, §9.34).  Compute used: `8,952` s of the
`14` h budget.

**What ran.**  Eleven sizes `p = 101`–`3089`, three curves each (seeds `1`–`3`; every seed gave an `h = 16` curve,
no replacement), **33 paired curves**: arm B with solver A and Karatsuba, and `16` walk-(iii) `⟨±ψ⟩` walks per
curve (`528` walks).  Every logarithm recovered, every kept row re-added, every walk correct.
**Deviations, both disclosed:** (1) the container restarted during the `p = 2213` seed 2 walk; the restart-safe
runner reran that walk only (its interrupted wall time is not in the budget count). (2) `p = 4337` qualified
under the budget rule, and its seed 1 index calculus completed (`S = 2.484·10¹¹`, `y = 1,252`, `282` columns), but
its 16-walk pool was killed by the session's memory-cgroup OOM killer at `4.8` GB. `/proc/meminfo` had passed the
registered `8` GB gate, but the cgroup limit is lower. Shrinking the pool would change the matched reference, so
`p = 4337` is not completed. Its record is kept unpaired, outside every fit. **The ladder reached
`p = 3089`, `log₂ n = 42.37`.**

**Identity (G2.114).**  The ten E25-curve runs reproduce E27's arm-B records in all `604` compared fields each
(`6,040`, wall times aside), and the walks reproduce E27's `rho_e27` and mean steps exactly on all ten curves.

| p | log₂ n | curves | y median | `D(0)` median [min, max] | `D(0.1)` median | `D(1)` median |
|--:|--:|--:|--:|:--|--:|--:|
| 101 | 22.63 | 3 | 1,366 | `2.07·10⁴` [`1.71·10⁴`, `3.93·10⁴`] | `7.21·10⁴` | `3.83·10⁵` |
| 149 | 24.88 | 3 | 4,272 | `4.03·10⁴` [`8.26·10³`, `1.52·10⁵`] | `1.38·10⁵` | `6.59·10⁵` |
| 197 | 26.49 | 3 | 969 | `1.49·10⁴` [`1.07·10⁴`, `3.49·10⁴`] | `5.09·10⁴` | `2.38·10⁵` |
| 269 | 28.29 | 3 | 1,918 | `1.75·10⁴` [`8.83·10³`, `2.75·10⁴`] | `5.90·10⁴` | `2.61·10⁵` |
| 397 | 30.53 | 3 | 1,349 | `8.75·10³` [`6.39·10³`, `9.72·10³`] | `2.91·10⁴` | `1.24·10⁵` |
| 557 | 32.49 | 3 | 1,180 | `6.46·10³` [`4.98·10³`, `7.43·10³`] | `2.13·10⁴` | `8.94·10⁴` |
| 773 | 34.38 | 3 | 2,268 | `7.63·10³` [`6.52·10³`, `1.15·10⁴`] | `2.50·10⁴` | `1.04·10⁵` |
| 1093 | 36.38 | 3 | 1,563 | `4.37·10³` [`3.67·10³`, `5.06·10³`] | `1.42·10⁴` | `5.84·10⁴` |
| 1549 | 38.39 | 3 | 1,392 | `2.37·10³` [`2.07·10³`, `3.72·10³`] | `7.66·10³` | `3.14·10⁴` |
| 2213 | 40.45 | 3 | 1,369 | `1.61·10³` [`1.34·10³`, `2.05·10³`] | `5.15·10³` | `2.10·10⁴` |
| 3089 | 42.37 | 3 | 1,496 | `1.54·10³` [`1.14·10³`, `1.70·10³`] | `4.93·10³` | `2.01·10⁴` |

`D(g) = (S + 97g·E) / (rho + 97g·steps)`, `rho` = walk (iii) without the `[d]G` check.  Per-curve rows (with `S`, `rho`,
`C`, shares, steps) are in `tables.md`.  `C` median rises slowly, `5.99·10⁵` (`p = 101`) to `6.65·10⁵` (`p = 3089`);
Macaulay entries per residual `1.76`–`1.82·10⁵`; Weil restriction plus solve is `98.3`–`99.1 %` of `S` (the solve alone `97.69`–`98.45 %`), the linear algebra `≤ 0.006 %`.
The walk's cost per step falls from `175`–`229` (`p = 101`, set-up dominated) to `95.9` (`p = 3089`).

**Fits** (`log₂ D = α + β(log₂ n − 32)`; stratified cluster bootstrap over curves, `10,000` replicates, seed `32`;
no replicate had `β ≥ 0`):

| GAE | model | curves | `β` [95 %] | `log₂ n*` [95 %] |
|--:|:--|--:|:--|:--|
| 0 | **M1** (`p ≥ 269`) | 24 | `−0.243` [`−0.279`, `−0.205`] | **`85.5` [`79.2`, `94.3`]** |
| 0 | M1-all | 33 | `−0.232` [`−0.272`, `−0.191`] | `87.7` [`80.2`, `98.6`] |
| 0 | M2 (`β = −1/4`) | 24 | — | `84.0` [`83.4`, `84.6`] |
| 0.1 | **M1** | 24 | `−0.248` [`−0.284`, `−0.211`] | **`91.2` [`84.4`, `100.7`]** |
| 0.1 | M1-all | 33 | `−0.239` [`−0.278`, `−0.197`] | `93.5` [`85.4`, `105.4`] |
| 0.1 | M2 | 24 | — | `90.8` [`90.3`, `91.4`] |
| 1 | **M1** | 24 | `−0.256` [`−0.292`, `−0.219`] | **`97.6` [`90.1`, `107.9`]** |
| 1 | M1-all | 33 | `−0.256` [`−0.295`, `−0.216`] | `97.7` [`89.6`, `108.9`] |
| 1 | M2 | 24 | — | `99.1` [`98.5`, `99.6`] |

Yield exponent in `p`: `−0.066` (OLS 95 % `[−0.285, 0.154]`).  Linear-algebra sensitivity: `0.0044 %` of `S` at
`p = 3089` (`∝ p²` against the rest `∝ p`) extrapolates to `≈ 5.6 %` at M2's `n*` (`p ≈ 4.2·10⁶`); it would move
`n*` by well under a bit.

| id | result | verdict |
|:--|:--|:--|
| G2.114 | `6,040` of `6,040` fields; ten walk references exact; every log, row and walk correct at every size | **holds** |
| G2.115 | `33` of `33` seeds gave `h = 16` curves; columns / `p` `0.0517`–`0.0687` at `p ≥ 557` | **holds** (for the sizes reached) |
| G2.116 | `C` median `6.39`–`6.67·10⁵`; entries per residual `1.762`–`1.770·10⁵` at `p ≥ 557` | **holds** |
| G2.117 | Weil + solve `≥ 98.28 %`; linear algebra `≤ 0.0057 %` | **holds** |
| G2.118 | exponent `−0.066`; per-size median `y` `969`–`4,272` | **holds** |
| G2.119 | steps / `√n` `0.468`–`0.787`; per step at `p ≥ 1549` `95.9`–`96.5` | steps **hold**; per step **falsified high**: the band `[66, 80]` priced the batched addition alone and missed the `ψ` map (`26`) the class representative costs at every step |
| G2.120 | largest size `D(0)` median `1.54·10³` (in band); no row at or below rho at any weight; medians fall from `2^{14}`–`2^{15}` to `2^{10.6}`, but not monotonically (up at `149`, `269`, `773`, within the per-curve scatter) | level and "rho ahead" **hold**; "falls with `n`" **holds as a trend** (`β < 0` in every replicate) and is **falsified read as monotone**, at 3 of 10 steps |
| G2.121 | `β = −0.243` `[−0.279, −0.205]` | **holds** (contains `−0.25`) |
| G2.122 | M1 `85.5`, interval `15.1` bits wide; M2 `84.0`, interval `1.1` bits wide | **holds** |
| G2.123 | M2 shift `+6.8` bits (`0.1` GAE), `+15.1` (`1` GAE); `D(0) < D(0.1) < D(1)` on all 33 rows | **holds** |

**What it says.**  On the line's best configuration (arm B, solver A, Karatsuba) and against its strongest
matched walk, **the distance to rho was measured over twenty bits of `n`, from `2^{22.6}` to `2^{42.4}`: per-size
median `2.1·10⁴×` → `1.5·10³×` at `0` GAE (best row `1.14·10³×`, `p = 3089` seed 2), `4.9·10³×` at `0.1` GAE and
`2.0·10⁴×` at `1` GAE at the largest size.  The crossover is empty in the reachable range: `n* > 2^{42.4}`, measured.**
The free-slope fit gives `β = −0.243` with a `±0.04` interval that contains the theory's `−1/4`, so with eleven
sizes and three curves each the slope is identified, unlike the four-size ratio exponent of
`LEAD-ICE-charged-toy-ic-exponent-vs-rho`. The crossover order it implies is
**`2^{85.5}` [`2^{79.2}`, `2^{94.3}`] at `0` GAE, `2^{91.2}` [`2^{84.4}`, `2^{100.7}`] at `0.1`, and `2^{97.6}`
[`2^{90.1}`, `2^{107.9}`] at `1` GAE**. The theory-slope fit puts it at `2^{84.0}`, `2^{90.8}` and `2^{99.1}`. The
`15`-bit width of the free-slope interval is what the ladder can say about an extrapolation over `43` bits beyond
its top size. **Class:** a measurement of distance, and a fitted extrapolation of where it would reach `1`. It is not
an improvement, and the fitted `n*` is not a measured crossing. It replaces §9.33's single-point model handover
(`2^{88}`–`2^{90}`, before E31 and E27) with an interval that contains it. **Distance remaining:** `≈ 1.5·10³×`
at `2^{42.4}` and `0` GAE, and `≈ 43` bits of `n` to the fitted crossover. **Scope:** E25's GLS twists
(`h = 16`, `p ≡ 1 (mod 4)`), the degree-11 `V₄` solve, `p ≤ 3089`, operation counts. The validation gate is not run;
the owner arranges it.

**Erratum (2026-11-01, from the independent validator's replay of all 66 runs from a fresh build and its recompute, `research/e32_handover_ladder_20261031/`).** Four statements in this section disagreed with the records and are corrected in place above; no verdict changes (G2.114–G2.118 and G2.121–G2.123 hold; G2.119's per-step cost is falsified high; G2.120 holds with the median rising at `p = 149, 269, 773`).

1. The build-input commit is `e757710521b…`; `binary_commit.txt` dropped a digit (`e75771052b…`). The record keeps its text and carries a correction line.
2. The share printed as "the solve" (`98.3`–`99.1 %`, and `≥ 98.28 %` in G2.117) is Weil restriction plus solve; the solve alone is `97.69`–`98.45 %`. G2.117's registered quantity is Weil plus solve, so its verdict stands.
3. G2.115: columns / `p` is `0.0517`–`0.0687`, not `0.057`–`0.068`; the registered band still holds.
4. M2's `log₂ n*` at `0.1` GAE is `90.847`, printed `90.9`; now `90.8`.

### 9.37 E33 measured: solver B's pivot selection costs `1.05·10⁶` native operations per call — B is ahead of A and E25 at `0` GAE only; a counted estimate (B″) stays ahead of A through `0.1` GAE but not at `1`

*2026-11-01.*  Code `c443138920` (compile fix of the unit test and the checks example `c689d19821`; the
library build is unchanged), binaries pinned in `experiments/37_solver_b_pivot_count/binary_sha256.txt`.
**Stage A:** `examples/e33_stage_a.rs`, one thread, E31's calls exactly (the first 1,000 targets of
E25 arm A's stream on each of E25's ten curves), seven solvers per call from the same per-target seed:
existing, A, A counted, B, B counted, B′, B″ (`stage_a.{json,log}`; the JSON is in the artifact store,
sha256 `7bc6e00b…`).  **End to end:** `run_e2e.sh` (E27's command with `--solver chain-sparsest-counted`,
`chain-sparsest-inc`, `chain-sparsest-est`; `--karatsuba`; `RAYON_NUM_THREADS=4` on two cores), E25's ten
curves, both arms, 30 runs, 0 failures.  Tables: `examples/e33_tables.rs` → `tables.md`, `summary.json`.
Host: the 4-vCPU container; isolab-eks is not reachable from it (no AWS CLI; the GitHub route refused
through the proxy), so operation counts only, no wall-time claim.  Report:
`research/e33_solver_b_pivot_count_20261031/`.

**Identity.**  On all 10,000 Stage A calls every solver returned the existing solver's solutions,
degree, outcome and decomposition set.  A counted reproduced A's phase counts, and B counted and B′
reproduced B's, on every call.  B′ chose B's pivot in every column of every call (equal digests), and its
maintained counts never differed from a recount.  B's initial pass was exactly `rows · cols` on every
call: `393 · 448 = 176,064` on 9,999 calls, and `396 · 448` on one.  End to end, all 30 records
reproduce every non-product field of E31 B's records.  The one exception is `folded.recovered`, a key
added to the records after E31 (E27's records carry it too).  B′'s records equal B counted's apart from
the selection fields and the tag.  B″'s also differ in its solve multiplications and what derives from
them, and in nothing else: every relation, log and walk is the same.  The pre-E33 binary (E32's pinned
build) and E33's give identical records for `macaulay`, `chain`, `chain-sparsest` and
`chain --karatsuba` at `p = 101` seed 2 (1,206–1,214 leaves each, timing aside).  The new
`chain --karatsuba` record equals E27's stored one, and no uncounted solver emits an E33 field
(`identity_existing_solvers.json`).  Negative controls (`e33_checks`, 32 calls): a second-best pivot in
one column changed the digest on 32/32 calls and the output on none; a count off by one was flagged on
32/32.

**Selection work per call** (Stage A means over 1,000 calls per curve; end to end in brackets):

| solver | initial counts | comparisons | count updates | total | end to end |
|:--|--:|--:|--:|--:|--:|
| B (as built, counted) | `176,064` | `3,681`–`3,713` | `8.69`–`8.77·10⁵` | `1.049`–`1.057·10⁶` | `1.049`–`1.057·10⁶` |
| B′ (incremental) | `506`–`507` | as B | `2.89`–`2.94·10⁵` | `2.93`–`2.98·10⁵` | `2.93`–`2.98·10⁵` |
| B″ (Markowitz estimate) | `506`–`507` | `4,070`–`4,105` | `8,121`–`8,192` | `12,696`–`12,804` | `12,694`–`12,802` |

B's selection pass reads the matrix about six times per call: once in the initial count (`176,064`
entries, the validator's lower bound) and about five times more in recounts.  The recounts are 83 % of the
pass.  B′ removes the initial pass and replaces recounts with two status tests per touched entry,
`0.279`–`0.282` of B's work.  B″ does `O(1)` work per row update.  Its estimate picks other pivots, so its
`C` median is `1.205`–`1.222 ×` B's (`3.71`–`4.02·10⁵` against `3.04`–`3.33·10⁵`; A's is `6.01`–`6.34·10⁵`).
Lead-scan reads (`79,114`–`79,119` per call) are identical for A, B, B′ and B″ on every call: a row's
lead-scan reads telescope to its final lead, which the shared pivot columns fix.

**End to end** (20 rows; `S_E25/S_X` in schoolbook arithmetic against E25's records, X's schoolbook `S`
being E31 B's total re-priced by X's own solve; `S_A/S_X` with Karatsuba products against E27's
solver-A records; `S_X/rho` against walk (iii) of E27 without the `[d]G` check; one GAE `= 97` per
Macaulay entry, per selection operation and per walk step):

| X | quantity | `0` GAE | `0.1` GAE | `1` GAE | rank across the band |
|:--|:--|:--|:--|:--|:--|
| B | `S_E25 / S_B` | `4.53`–`4.76` | `0.274`–`0.276` | `0.157`–`0.158` | unresolved: the order flips between `0` and `0.1` |
| B | `S_A / S_B` | `1.82`–`1.86` | `0.193`–`0.197` | `0.149`–`0.152` | unresolved: the order flips between `0` and `0.1` |
| B | `S_B / rho` | `3.45·10³`–`2.19·10⁵` | `1.10·10⁵`–`7.17·10⁶` | `6.10·10⁵`–`4.40·10⁷` | rho ahead at every weight |
| B′ | `S_E25 / S_B′` | `4.53`–`4.76` | `0.678`–`0.683` | `0.404`–`0.407` | unresolved: the order flips between `0` and `0.1` |
| B′ | `S_A / S_B′` | `1.82`–`1.86` | `0.478`–`0.487` | `0.384`–`0.393` | unresolved: the order flips between `0` and `0.1` |
| B′ | `S_B′ / rho` | `3.45·10³`–`2.19·10⁵` | `4.45·10⁴`–`2.89·10⁶` | `2.36·10⁵`–`1.71·10⁷` | rho ahead at every weight |
| B″ | `S_E25 / S_B″` | `3.83`–`3.98` | `1.457`–`1.497` | `0.980`–`0.997` | unresolved: B″ ahead at `0` and `0.1`, behind at `1` on all 20 rows |
| B″ | `S_A / S_B″` | `1.538`–`1.558` | `1.046`–`1.048` | `0.9465`–`0.9480` | unresolved: B″ ahead at `0` and `0.1`, behind at `1` on all 20 rows |
| B″ | `S_B″ / rho` | `4.12·10³`–`2.61·10⁵` | `2.03·10⁴`–`1.33·10⁶` | `9.58·10⁴`–`7.01·10⁶` | rho ahead at every weight |

For comparison, solver A (E27, walk (iii)) is at `6.39·10³`–`4.06·10⁵` (`0`), `2.13·10⁴`–`1.39·10⁶`
(`0.1`) and `9.07·10⁴`–`6.64·10⁶` (`1` GAE).  **Sensitivity (G2.133):** pricing every arm's lead-scan
reads too (E25's taken as A's: the same echelon on the same rows) moves the Stage A per-call ratios but
reverses none.  B″/A becomes `1.035`–`1.036` at `0.1` and `0.962`–`0.963` at `1` GAE; E25/B″ becomes
`0.986`–`0.999` at `1` GAE.

| id | result | verdict |
|:--|:--|:--|
| G2.124 | identical outputs on all 10,000 Stage A calls and all 30 end-to-end records, with the field rule above; phase counts and digests as registered; old binary = new for the four existing configurations; both negative controls detected 32/32. The in-crate unit tests type-check (`cargo check --release --lib --tests`) but were not executed: a release test build of the library needs about 9 GB of RSS, which the shared container could not provide beside E32. Their solver-level content runs as `e33_checks`. | **holds** (unit tests not executed; disclosed) |
| G2.125 | initial pass `rows · cols` exactly; comparisons `0.35 %` of the total; recount `8.7·10⁵` > initial `1.76·10⁵`; total `1.049`–`1.057·10⁶` | **holds** |
| G2.126 | B′/B `0.279`–`0.282` | **holds** |
| G2.127 | B″ `12,696`–`12,804` per call (≤ `2·10⁴`); `C` median B″/B `1.205`–`1.222` | **holds** |
| G2.128 | `S_A/S_B` `1.82`–`1.86` (0), `0.193`–`0.197` (0.1), `0.149`–`0.152` (1) on every row | **holds**: the order flips; B is not ranked ahead of A |
| G2.129 | `S_E25/S_B` `4.53`–`4.76`, `0.274`–`0.276`, `0.157`–`0.158` | **holds** (far below the validator's lower-bound values `0.885`–`0.899` and `0.542`–`0.544`: the recounts were the larger part) |
| G2.130 | B′ behind A (`0.478`–`0.487`, `0.384`–`0.393`) and E25 (`0.678`–`0.683`, `0.404`–`0.407`) at `0.1` and `1` GAE on every row | **holds** |
| G2.131 | against A: ahead at `0` and `0.1`, behind at `1` GAE on every row, as registered; against E25: ahead at `0` and `0.1`, **behind at `1` GAE on all 20 rows** (`0.980`–`0.997`) | **falsified** for E25 at `1` GAE; holds against A |
| G2.132 | `S_B/rho` `3.45·10³`–`2.19·10⁵` at `0` GAE; rho ahead of every row of B, B′, B″ at every weight | **holds** |
| G2.133 | lead scans identical across rules; no rank reverses | **holds** |

**What it says.**  Counting B's selection work is accounting, not an improvement.  The count settles
what §9.24 left open: **B as built is ahead of A and E25 at `0` GAE only.**  Its pass costs about six
matrix reads per call, and at `0.1` GAE each read costs more than the multiplications sparser pivots
save, so B falls to `0.19×` A.  Maintaining the counts incrementally (B′) keeps B's choices for `28 %` of
the work, which is still behind A and E25 at `0.1` and `1` GAE.  A counted estimate (B″), with `O(1)` work
per row update, gives up `22.5`–`22.7 %` of B's saving in mean `C` (worse pivots).  With its `1.27·10⁴` selection
operations counted, it is ahead of A at `0` GAE (`1.54`–`1.56×`) and at `0.1` GAE (`1.046`–`1.048×`).  At
`1` GAE it is behind A (`0.946`–`0.948`) and E25 (`0.980`–`0.997`): at that weight its selection
operations cost `1.2·10⁶` multiplications per call, more than the `2.30`–`2.33·10⁵` it saves against A's mean `C`.  **No pivot rule
tested here ranks ahead of A across the band; A remains the ranked solver.**  B″ is a measured
improvement of the baseline at `0` and `0.1` GAE only.  Under the §5a rule it is not ranked, and it is
not claimed as an improvement across the band.  **Distance remaining:** the best row (`p = 397` seed 1,
arm B) is `4.12·10³×` the matched rho for B″ and `6.39·10³×` for A at `0` GAE.  Rho is ahead of every row
at every weight.  **Next lever:** the band is set by the Macaulay entries (`176,064` per call, `1.7·10⁷`
multiplications at `1` GAE).  No pivot rule removes them; a smaller matrix would.  Not done here: the
validation gate, the leaderboard and `benchmarks/`.


**Erratum (2026-11-01, from the independent validator's recompute of `stage_a.json` and the end-to-end records).** Fourteen printed figures in this section disagreed with the raw records and are corrected in place above; no verdict or rank changes (G2.124–G2.130, G2.132, G2.133 hold; G2.131 is falsified against E25 at `1` GAE; no variant is ranked).

1. Transcription: B′'s count updates were printed `2.92`–`2.98·10⁵`; the records give `288,884.5`–`294,201.3`, i.e. `2.89`–`2.94·10⁵` (`stage_a.log`, p = 101 seed 1: `289,121`).
2. Truncated means, now rounded: B's comparisons minimum `3,680.79` → `3,681`; B′ and B″ initial counts `505.85`–`506.74` → `506`–`507`; B″ comparisons minimum `4,069.53` → `4,070`; B″ total maximum `12,803.90` → `12,804`; `C` median B″/B maximum `1.22156` → `1.222`.
3. Double rounding (four places, then three), now rounded once: `S_A/S_B` at `1` GAE maximum `0.152475` → `0.152`; `S_E25/S_B′` at `0.1` GAE minimum `0.678485` → `0.678`; `S_E25/S_B″` at `1` GAE minimum `0.980463` → `0.980`. The prose summary's `S_A/S_B″` at `1` GAE (`0.946478`–`0.947987`) was printed `0.947`–`0.948` (rounded twice); now `0.946`–`0.948`.

### 9.38 E34 pre-registration: a cheaper order-24 class representative, and which walk is the matched rho on E18's `j = 0` curves across the §5a band

*Registered 2026-10-31, before any E34 code.*  **Question.**  §9.31 and §9.33 left the matched reference on
E18's eight `j = 0` curves unresolved: E29's order-24 walk takes `1.701×` fewer steps (pooled) but its class
representative costs `≈ 90` `F_p` multiplications a call (three full Frobenius images, twelve `ψ` candidates),
so at `0` GAE the order-6 walk is cheaper (`0.962` schoolbook, `0.884` Karatsuba), at `1` GAE the order-24 walk
(`1.205`, `1.172`), and the order-24 walk's `12` candidate comparisons were not emitted.  E34 builds the cheaper
representative §9.31 names, counts and emits every per-step operation of every arm, and re-ranks.

**Design (derived before code).**  On the canonical model `y² = x³ + b₀t³` (`t⁴ = c`), E29's representative of
`P` is: among the twelve points `ι^k ∘ Φ^j (P)` (`j < 4`, `k < 3`, `Φ` the transported Frobenius, `ι` the cubic
automorphism) the one with least abscissa in the lexicographic order of `E4`'s coordinates (first in the order
`j`-major, `k`-minor on a tie), then the least of `±y`; multiplier `λ_Φ^j λ_ω^k`, negated with `y`.  Both maps are
coordinate-wise on `x`: `Φ(x)_i = h² zⁱ x_i` with `z = t^{p−1} ∈ F_p` and `h² = z^{−1}` (`h = t^{−(p−1)/2}`), and
`ι(x) = ωx`; so candidate `(j, k)` has `x_i·c_{ijk}`, `c_{ijk} = h^{2j} z^{ij} ω^k ∈ F_p`, a table of `48`
constants computed once per walk (charged).  On `y`, `Φ^j` is a monomial map (a coordinate permutation with
`F_p` scalars), `ι` fixes `y`.  The new representative:
1. computes coordinate `0` of the twelve candidate abscissae, `x₀·c_{0jk}` (`c = ±1` taken as a copy or a
   negation, every other constant as one `F_p` multiplication), and keeps the minimum by `11` comparisons;
   only on a tie (`x₀ = 0`, probability `≈ 1/p`) does it go on to coordinate `1`, `2`, `3` over the tied set;
2. completes the winner's abscissa (coordinates `1`–`3`, `≤ 3` multiplications) and computes only the
   winner's ordinate (`Φ^j` on `y`, `≤ 4` multiplications);
3. resolves the sign by comparing, at the first non-zero coordinate `y_i`, `y_i` against `(p − 1)/2` (equal to
   E29's `−y < y`), and negates `y` only when needed; the multiplier comes from a `24`-entry table.
The same construction gives a cheap order-6 representative (three candidates `ω^k x`, the arm E29 did not have),
run as a fourth arm so that the order-24 walk is ranked against the strongest order-6 walk too.  A mul-lean
variant (sign classes `c_{0,j+2,k} = −c_{0jk}`: `5` multiplications and `6` negations instead of `10` and `1`)
is registered as a fifth arm, producing the same walk.

**Counting rule.**  Every arm is one serial walk loop (a copy of E29's, which this experiment does not modify)
with a pluggable representative.  Counted unit: `F_p` multiplications (`F_{p⁴}` products at `19` / `13`, as
E27), the per-walk constants included, the `[d]G` check excluded.  **Emitted native operations**, each one
counter, applied the same way to every arm: `cmp`, a comparison of two `F_p` residues in the representative
(an `E4` comparison counts one per coordinate it examines; a test against `0` or `(p−1)/2` counts one); `neg`,
an `F_p` negation or addition in the representative (outside the group law; `F_p` additions inside an affine
addition are part of the counted group operation, as the GAE that prices native operations is itself a group
addition); `modn`, a multiplication, addition, subtraction or negation mod `n` in the logarithm bookkeeping
(`pow_mod(λ, k)` counted by its products: `0, 2, 3` for `k = 0, 1, 2`; the collision's inversion as one);
`hash`, a partition-hash evaluation; `table`, a hash-table lookup or insert.  E27's visit digest is an
instrument, not part of the walk, and is not counted.  The band (§5a): `cost(w) = F_p multiplications + w·97·
(native operations)`, `w ∈ {0, 0.1, 1}`, one GAE `= 97` in both arithmetics (as §9.33; the Karatsuba
arithmetic's own `79` is a sensitivity column).  The validator's convention (one native operation per step)
is reported beside it for continuity.  A ratio is order 6 over order 24 (`> 1`: the order-24 walk is cheaper),
pooled as §9.33 (sum over curves of cost`/√n`).  **Ranking rule:** two walks are ranked only where the pooled
ratio is on one side of `1` at all three weights, in both arithmetics; per curve, the same test per curve.

**Identity and controls.**  (a) The counted copies of E29's two walks must reproduce E29's `256` Stage R
records field for field (`rho4_folded_with`, `rho_twist24_with` unchanged; E29's outputs bit-identical, shown
by a unit test and by the rerun).  (b) The new representatives must equal E29's on every one of the `24`
(resp. `6`) class members of `≥ 200` random points per curve, including forced `x₀ = 0` and `x = 0` points, in
both point and multiplier, and `rep = [m]P` checked by scalar multiplication on a sample.  (c) The new walks
must equal E29's walks per seed: steps, group operations, cycle escapes, recovered log, correctness, and the
visit digest of every representative visited.  **Negative controls:** a perturbed representative that omits
candidate `(j, k) = (3, 2)`, and one that takes `Φ^{j−1}` for the ordinate; both must fail (b), and the first
must change the visit digest of every one of `16` Stage R walks at `p = 241`.  **Positive control:** the
counters must reproduce the analytic counts of the order-6 arm (`8` multiplications per E29 representative).

**Protocol.**  E29's eight curves and canonical models (`b₀ = 3, 3, 3, 9, 9, 2, 17, 3`), seeds `1000`–`1015`,
five arms (order 6 as built; order 6 cheap; order 24 as built = E29; order 24 cheap; order 24 cheap mul-lean)
× schoolbook and Karatsuba.  Route: isolab-eks is the default but unreachable from this container (no `aws`
CLI, no `kubectl`); so local, operation counts only, one core (`RAYON_NUM_THREADS=1`), no wall-time claim.
Records `experiments/38_twist24_cheap_rep/`; report `research/e34_twist24_cheap_rep_20261031/`.

| id | quantity | prediction | falsified by |
|:--|:--|:--|:--|
| G2.134 | identity (b) | both cheap representatives equal E29's on every class member tested, all eight curves; both negative controls caught | any mismatch, or a control not caught |
| G2.135 | E29 replay (a) | counted copies reproduce all `256` E29 Stage R records in every field; E29's functions untouched | any field differs |
| G2.136 | walk identity (c) | all `128` cheap order-24 walks (and their mul-lean twins) per arithmetic equal E29's per seed, digests included; cheap order 6 equals order 6 likewise | any walk differs |
| G2.137 | negative control in the walk | the omitted-candidate walk's digest differs from E29's on `16/16` seeds at `p = 241` | fewer than `16` |
| G2.138 | representative cost | `F_p` multiplications per call: order 6 as built `8.0`; order 6 cheap `[3, 5]`; order 24 as built `[85, 100]`; order 24 cheap `[12, 20]`; mul-lean `[7, 15]` (equal in both arithmetics) | outside |
| G2.139 | per-step multiplications | cheap order 24 / order 6 as built, per step, `[1.00, 1.15]` on every curve with `p ≥ 541`, both arithmetics | outside |
| G2.140 | native operations per step | order 6 as built `[14, 19]`; order 6 cheap `[12, 17]`; order 24 as built `[40, 70]`; order 24 cheap `[22, 30]` (of which `cmp` `[11, 14]` per representative) | outside |
| G2.141 | ranking, pooled | order 6 as built / cheap order 24: `0` GAE `[1.40, 1.80]`, `0.1` GAE `[1.10, 1.45]`, `1` GAE `[1.00, 1.30]`, both arithmetics: **the cheap order-24 walk is cheaper across the band**; per curve, the same holds on `≥ 6` of `8` | a pooled ratio `≤ 1` at any weight, or `< 6` curves |
| G2.142 | against the strongest order-6 walk | cheap order 6 / cheap order 24 pooled: `> 1.3` at `0` GAE, `> 1.0` at `0.1`, and in `[0.90, 1.10]` at `1` GAE (a crossing inside the band is possible) | outside |
| G2.143 | consequence for E18 | the `ψ`-arm distances to rho lengthen by the pooled ratio against the strongest walk at each weight where the ranking holds (`[1.4, 1.8]×` at `0` GAE, handover later by `4·log₂` of it, `[1.9, 3.4]` bits); a **reference correction, not an improvement** | — (derived from G2.141–G2.142) |

**What it would say.**  If G2.141 holds, the order-24 walk with the cheap representative is the stronger rho
on E18's curves at every weight of the band against E29's order-6 walk, and E18's `ψ` rows stand further from
rho by the pooled ratio — a reference correction.  If G2.142's `1` GAE ratio straddles `1`, the matched
reference is the cheap order-24 walk at `0` and `0.1` GAE and unresolved at `1` GAE against the cheap order-6
walk; the deciding counters are the `11` comparisons a minimum of twelve needs, which no representative equal
to E29's can avoid.

### 9.39 E34 measured: the cheap order-24 walk is the matched rho on E18's `j = 0` curves at every weight of the band (pooled); E18's `ψ` rows move `1.13`–`1.63×` further from rho — a reference correction

*2026-11-01.*  Code and binary `bd0118f39` (`experiments/38_twist24_cheap_rep/binary_commit.txt`, sha256 recorded):
`src/cryptanalysis/twist24_rep.rs` (new module; E29's functions untouched), `examples/e34_cheap_rep.rs`
(`identity.{json,log}`, `stage_r_p*.{json,log}`, `negative_walks.{json,log}`); tables `examples/e34_tables.rs` →
`tables.md`, `summary.json`.  Report `research/e34_twist24_cheap_rep_20261031/`.  Route: isolab-eks unreachable
from this container (no `aws`, no `kubectl`), so local, serial, one core; operation counts only, no wall-time claim.
All runs together took under a minute of CPU.  E33's uncommitted solver edits were in the worktree when the
binary was built (no walk path); the E29 replay below is the check.

**Identity.**  (a) The counted copies of E29's walks reproduce **all `512`** E29 Stage R records (8 curves × 2 orders ×
16 seeds × 2 arithmetics; §9.38 said `256`, the count per arithmetic) in every field, except the visit digest of the order-24 records, which E29 stored as `0`.  (b) Every one of the `24`
(resp. `6`) class members of `200` random points of `⟨G⟩` plus `8` forced `x₀ = 0` points per curve gets E29's
representative and multiplier from both cheap order-24 representatives (`39,936` members) and the cheap order-6 one
(`9,984`), `0` mismatches, `rep = [m]P` on every scalar check.  No curve has a point with `x = 0` (`b₀t³` is not a
square), so that case is untested by points (the code keeps E29's first-candidate rule).  (c) All `768` cheap walks
equal their built walk per seed in steps, group operations, cycle escapes, recovered log, correctness and the
digest of every representative visited; digests also agree across the two arithmetics.  **Negative controls:**
the omitted-candidate representative mismatches `416/4,992` members per curve, the wrong-ordinate one `4,992/4,992`
(and fails every scalar check); the omitted-candidate walk's digest differs from E29's on `16/16` seeds at
`p = 241` (all `16` still recover the log: an inconsistent class map is still a valid walk, so a digest, not
correctness, is the detector).  **Positive control:** E29's order-6 representative costs exactly `8.000`.

**Per step** (schoolbook, `p = 541`–`1069`; Karatsuba in `tables.md`):

| arm | rep `F_p` mults / call | mults / step | native ops / step | of which `cmp` | `neg` | `modn` | `hash` | `table` |
|:--|--:|--:|--:|--:|--:|--:|--:|--:|
| order 6, E29 | `8.00` | `109.1`–`111.2` | `16.77` | `3.10` | `4.13` | `5.55` | `2.00` | `2.00` |
| order 6, cheap | `4.00` | `105.0`–`107.1` | `15.22` | `4.65` | `2.06` | `4.52` | `2.00` | `2.00` |
| order 24, E29 | `86`–`92` | `191.0`–`199.4` | `65.25` | `16.52` | `4.13` | `40.61` | `2.00` | `2.00` |
| order 24, cheap | `15.17` | `117.2`–`120.2` | `25.81` | `13.95` | `3.35` | `4.52` | `2.00` | `2.00` |
| order 24, cheap mul-lean | `10.17` | `112.1`–`115.0` | `30.96` | `13.95` | `8.50` | `4.52` | `2.00` | `2.00` |

The cheap representative needs `13.5` comparisons a call (`11` for the minimum of twelve, one zero test, `≈ 1.5` for
the sign).  E29's walk as built also spent `40.6` mod-`n` operations a step on its multipliers (`pow_mod` per
candidate), which no earlier pricing counted; with every operation priced, E29's order-24 walk loses to the
order-6 walk at **every** weight (`0.962`, `0.562`, `0.453`, schoolbook), so §9.33's crossing at `1` GAE was an
artefact of the one-op-per-step convention.

**The band** (pooled, order 6 / order 24, `> 1`: order 24 cheaper; every emitted operation at `w·97`):

| order 6 / order 24 | arithmetic | 0 GAE | 0.1 GAE | 1 GAE | pooled order holds | curves: order 24 / unresolved |
|:--|:--|--:|--:|--:|:--|:--|
| E29 / E29 | schoolbook | `0.962` | `0.562` | `0.453` | yes (order 6) | `0` / `3` (order 6 on 5; `p = 601, 997, 1069` unresolved: order 24 cheaper at `0` GAE) |
| E29 / cheap | schoolbook | `1.563` | `1.254` | `1.126` | **yes** | `7` / `1` (`p = 769`) |
| E29 / cheap | Karatsuba | `1.544` | `1.232` | `1.122` | **yes** | `7` / `1` |
| cheap / cheap | schoolbook | `1.505` | `1.167` | `1.026` | **yes** | `3` / `5` |
| cheap / cheap | Karatsuba | `1.476` | `1.140` | `1.021` | **yes** | `3` / `5` |
| cheap / mul-lean | schoolbook | `1.572` | `1.042` | `0.864` | no | — |

With the Karatsuba arithmetic's own GAE (`79`) the cheap/cheap row is `1.476`, `1.160`, `1.025`; under the
validator's one-op-per-step convention every cheap order-24 row is `1.48`–`1.66` at every weight.  The mul-lean
variant is cheapest at `0` GAE and loses its lead by `0.1` (its six negations); the strongest order-24 walk is the
mul-lean one at `0` GAE and the op-lean one at `0.1` and `1`, the strongest order-6 walk the cheap one throughout.
Per curve against the cheap order-6 walk, the cheap order-24 walk is cheaper at every weight on `p = 601, 997,
1069`; on the other five it is cheaper at `0` (`1.16`–`1.46`) and the order changes at `0.1` (`p = 769`) or `1` GAE
(`0.78`–`0.99`): there the ranking is unresolved.  The per-curve spread is the step count of `16` walks (steps
ratio `1.29`–`2.10` per curve, `1.701` pooled), not the representative.

| id | result | verdict |
|:--|:--|:--|
| G2.134 | `0` mismatches on `49,920` members; both controls caught on all eight curves | **holds** (the `x = 0` case has no point to test) |
| G2.135 | `512/512` E29 records identical; E29's functions untouched; unit test `e34_built_arms_reproduce_e29_walks` passes | **holds** (registered count `256` was per arithmetic) |
| G2.136 | `768/768` | **holds** |
| G2.137 | `16/16` | **holds** |
| G2.138 | `8.00`; `4.00`; `86`–`92`; `15.16`–`15.17`; `10.17`–`10.18`; equal in both arithmetics | **holds** |
| G2.139 | `1.072`–`1.086` (schoolbook), `1.086`–`1.100` (Karatsuba) | **holds** |
| G2.140 | `16.77`; `15.22`; `65.25`–`65.29`; `25.81`–`25.87` (`cmp` `13.52`–`13.59` a call) | **holds** |
| G2.141 | `1.563 / 1.254 / 1.126` (S), `1.544 / 1.232 / 1.122` (K); `7/8` curves | **holds** |
| G2.142 | `1.505 / 1.167 / 1.026` (S), `1.476 / 1.140 / 1.021` (K) | **holds**: above `1` at every weight, pooled, by `2 %` at `1` GAE |
| G2.143 | see below | derived |

**Consequence for E18.**  E18's `ψ` arm was measured against E29's order-6 walk.  The strongest walk now costs
`1.632×` (`0` GAE), `1.254×` (`0.1`) and `1.126×` (`1` GAE) less than that walk, pooled, schoolbook (Karatsuba
`1.627`, `1.232`, `1.122`): **every E18 `ψ`-arm distance to rho lengthens by that factor, and the modelled handover
(§9.3, `2^{146.0}`) moves later by `4·log₂` of it, `2.83` / `1.31` / `0.68` bits (Karatsuba `2.81` / `1.20` / `0.66`).**  Of this, the order-6 walk's own
cheaper representative is `1.035`–`1.039×` at `0` GAE; the rest is the order-24 action.  This is a **reference
correction, not an improvement**: no index-calculus cost changed, the rho reference got stronger.  §9.33's "lower
bounds at `1` GAE" now has a value.

**Unit tests.**  `cargo test --release --lib twist24_rep`: `3` passed (`e34_built_arms_reproduce_e29_walks`: the counted copies equal `rho4_folded_with` / `rho_twist24_with` on `p = 241`, both arithmetics, with the positive control of `8` products per order-6 representative; `e34_cheap_arms_give_e29s_walks`; `e34_cheap_representatives_equal_e29s_and_controls_are_caught`), run after E33's compile fix of the crate's test build (`c689d1982`).  Clippy (no warning in E34's files; one `is_multiple_of` rewrite after the runs, behaviour-identical) and rustfmt pass.

**What it says.**  The order-24 action is the stronger rho on E18's `j = 0` curves once its class representative is
built as the coordinate-wise structure allows: `15` multiplications instead of `86`–`92`, so a step costs `7–10 %`
more than an order-6 step instead of `80 %`, against `1.70×` fewer steps.  That holds pooled at every weight of the
band with every operation of both walks emitted.  The margin at `1` GAE is thin (`2 %` over the cheap order-6 walk)
and rests on the `11` comparisons that a minimum of twelve needs.  No representative equal to E29's avoids them.


**Erratum (2026-11-01, from the independent validator's recompute; replay byte-identical from a fresh build of `bd0118f39`).** Six statements in this section disagreed with the records and are corrected in place above; no verdict changes (G2.134–G2.143 hold).

1. G2.138: the cheap representative costs `15.16`–`15.17` F_p multiplications a call, not `15.16`–`15.18`.
2. Mul-lean multiplications per step: top `115.047` → `115.0` (was rounded twice to `115.1`).
3. Cheap order-24 native operations per step, pooled: `25.81`, not `25.82`.
4. E29 / E29 per curve (schoolbook): order 6 is cheaper across the band on 5 of 8 curves, not all 8; on `p = 601, 997, 1069` E29's order-24 walk is cheaper at `0` GAE, so those three are unresolved. The pooled correction to §9.33 is unaffected.
5. The five curves unresolved against the cheap order-6 walk are cheaper at `0` GAE by `1.16`–`1.46`, not `1.16`–`1.86` (`1.86` is `p = 601`, a resolved curve).
6. Wording: the E29 replay matches every field except the order-24 visit digest, which E29 stored as `0` (disclosed in `tables.md`).

**Two cautions from the validator.** The cheap order-24 lead over the cheap order-6 walk at `1` GAE (`1.026`, schoolbook) is `0.4` standard errors from `1` with `16` walks per curve and arm (steps ratio `1.701 ± 0.111` against the expected `2.0`); the registered rule ranks on point estimates, so G2.142 stands, but the data do not resolve that margin statistically, and the per-curve ranks are within sampling noise. The `1.632` factor at `0` GAE for E18 uses the mul-lean walk, which is not ranked across the band; against the band-ranked cheap walk it is `1.563` (`2.58` bits) schoolbook and `1.544` (`2.51` bits) Karatsuba.

### 9.40 E35 pre-registration: solver B's pivot schedule, learned once per curve and replayed with a counted check

*Registered 2026-11-02, before any E35 code.*  **Numbering:** E35 takes §9.40 (this registration), §9.42
(results) and G2.144–G2.153; §9.41 is left to the parallel E36.  **Motivation:** §9.37.  Solver B
(`chain-sparsest`) multiplies `1.82`–`1.86×` less than A (E31's ranked chain solve) at `0` GAE, but its
counted pivot selection (`1.049`–`1.057·10⁶` native operations a call, `chain-sparsest-counted`) puts it
behind A at `0.1` and `1` GAE; the incremental B′ (`2.9·10⁵`) is still behind, and the estimate B″
(`1.27·10⁴`, with `1.21`–`1.22×` B's multiplications) is ahead of A at `0` and `0.1` but behind at `1` GAE.
No B variant is ranked.  The selection is the same question asked afresh on every call; if its answer
depends mostly on the curve (the system's shape: the same monomial supports, the same row order), it can
be learned once and replayed.  **Literature and prior work:** Joux–Vitse's F4 trace reuse across many
similar systems (`KR-IC-f3da82` in the lead atlas: one system's reduction trace replayed on many); on
the binary Boolean `S₃` line, learn/apply trace reuse gave `1.4`–`2.3×` engine gains and that lead is
closed dominated (`LEAD-ICE-boolean-f4-f5-f6-engines`); in this note E21 (§9.11) replayed a *shape* trace
(skipping rows that reduce to zero) on the weighted `D₄` solve and lost (`0.95`–`0.98×`, `13`–`23 %` full
fallbacks), because which rows vanish changes from target to target.  E35 differs from E21 in what is
replayed and how a miss is handled: only the *choice* among valid pivot candidates is replayed (any
candidate gives the same row space and pivot columns, so a wrong choice costs fill, never correctness),
and a miss falls back per column to A's rule at no selection cost, not to a full re-solve.

**Stage 0 (measured first, design question).**  Does B's pivot sequence depend only on the curve, or
also on the target?  On E31/E33's Stage A calls (E25's ten curves, the first 1,000 targets of arm A's
stream), the pivot row B chooses in every column (B′'s elimination, which E33 showed chooses B's pivot
in every column) is recorded per call and compared with the curve's learned trace (below): the fraction
of calls whose whole sequence equals it, the number of distinct sequences per curve, the per-column
agreement over contested columns (more than one candidate), the fraction of calls with B's matrix shape
(`rows × cols`) equal to the trace's, and, across curves, the agreement of one curve's sequences with
another curve's trace.  Stage 0 informs the reading of the replay; it changes no rule below.

**The trace solver T (`chain-sparsest-trace`).**  E31 A's chain solve with one change, in the echelon's
pivot choice, behind a new solver name in a new module (`src/cryptanalysis/chain_trace.rs`) with a
minimal dispatch hook; `macaulay`, `chain`, `chain-sparsest` and E33's four solvers are unchanged.

- **Learning**, once per curve and arm (each `run_k4_gls_pipeline_with` call): `K = 3` learning targets
  `R = aG + bQ` from a stream of their own (`StdRng::seed_from_u64(seed ^ 0xE357ACE)`, disjoint from the
  target stream, so Stage A is out of sample), each decomposed only as far as B′'s echelon (Weil
  restriction, triangularisation, matrix, echelon with B′'s counted selection).  The trace is the
  per-column mode of the three pivot sequences (ties to the first learning target), and the shape of
  the first.  **Charged:** every `F_p` multiplication of learning (target generation, Weil, triangularise,
  echelon) into `S`; B′'s selection operations, the `rows · cols` Macaulay entries and one operation per
  column per learning sequence for the mode, as native operations.  Itemised in a new record field and
  also added to `total_muls`, `macaulay_entries` and `pivot_selection_ops`, so E33's band formula prices it.
- **Replay**, per call: one native operation to compare the call's shape with the trace's (a mismatch
  runs the whole echelon with A's rule); in each column whose bucket holds more than one candidate, three
  operations: the trace lookup, the test that the traced row's current lead is this column (it is an
  unused candidate), and the test that its entry in this column is nonzero.  If both tests pass the traced
  row is the pivot; otherwise the column falls back to A's rule (the lowest-indexed candidate), with no
  further selection work, and the miss is counted.  Columns with one candidate need no choice (B does no
  selection there either).  The bucket sort, lead scans and swaps are common to every rule and are not
  selection (E33's convention; they enter only the sensitivity check).
- **Storage:** the trace is `cols` row indices (`448 · 4` bytes per curve and arm), listed as a
  memory-size counter and not charged (§5a, owner 2026-11-01: the band prices operations, not bytes).
- A chain-solve fallback (the existing solver rerun) is charged in full as in E31/E33.
- **Not built:** a replay of A's schedule.  A's rule (lowest index) has no selection work to save, so a
  replay of it could only add the checks.

**Arms and protocol.**  *Stage A:* `examples/e35_stage_a.rs`, E33's setting exactly (one thread, the same
1,000 calls per curve, every solver from the same per-target seed): existing (E25), A counted
(`chain-counted`), B counted (`chain-sparsest-counted`), B″ (`chain-sparsest-est`) and T, with per call
outputs, degree, outcome, decomposition set, phase counts, selection work, entries, lead-scan reads,
contested columns, column misses, and whether T's pivot digest equals B's.  *Checks:* `examples/e35_checks.rs`
on 32 calls of E25 `p = 101` seed 2 and `p = 397` seed 1: corrupted traces of three kinds — (i) one contested
column's entry pointed at a row already used as an earlier pivot, (ii) at a row whose lead is a later
column, (iii) a trace of another shape — must each be detected on every call where the corrupted column is
contested (a column miss, or the shape fallback), with outputs unchanged; and a positive control that the
check is load-bearing: the same corrupted traces replayed *without* the check must produce a wrong
echelon (pivot columns or standard monomials not B's), a chain-solve fallback or an abort on at least one
call.  *End to end:* E33's command (`quartic_pipeline_gls --karatsuba`, `RAYON_NUM_THREADS=4` for the
target stream, two cores) with `--solver chain-sparsest-trace` on E25's ten curves, both arms (20 rows).
*Identity of existing solvers:* the new binary's `macaulay`, `chain`, `chain-sparsest`, `chain --karatsuba`,
`chain-sparsest-counted --karatsuba` and `chain-sparsest-est --karatsuba` records at `p = 101` seed 2 equal
the stored E25, E31, E27 and E33 records leaf for leaf (wall times aside), and carry no E35 field.
*References:* A is E27's records (A with Karatsuba); B, B′, B″ are E33's records; E25 is its own records
(schoolbook), compared with T's schoolbook `S` built as E33 built B's (E31 B's total re-priced by T's
own solve and learning); the matched rho is walk (iii) of E27 without the `[d]G` check.  *Band:*
`S(g) = total_muls + 97 g (macaulay_entries + pivot_selection_ops)`, `rho(g) = rho + 97 g · steps`,
`g ∈ {0, 0.1, 1}`; a rank is stated only where the order holds at all three weights on every row.  Host:
this 4-vCPU container, at most two cores; isolab-eks is not reachable from it (no AWS CLI or kubectl), so
operation counts only, no wall-time claim.  Tables are computed in Rust (`examples/e35_tables.rs`);
Python only plots.  **Class, fixed in advance:** if G2.150 holds, T is an engineering improvement (a
constant on the solve's selection work; exponents unchanged); counting alone would be accounting.

| id | quantity | prediction | falsified by |
|:--|:--|:--|:--|
| G2.144 | Stage 0: does B's sequence depend on the target? | shape equal to the trace's on `≥ 99.5 %` of calls per curve; per-column agreement with the trace over contested columns `≥ 90 %` on every curve; whole-sequence equality below `50 %` of calls on at least one curve (the sequence depends on the target through accidental zeros, but mostly on the curve) | outside |
| G2.145 | identity (**kill**) | T returns the existing solver's decomposition set, outcome and degree on every Stage A call and every end-to-end record; the end-to-end T records equal E33's B counted records in every field except the solve's multiplications and what derives from them, the selection and learning fields, `macaulay_entries` (learning) and the solver tag — every relation, log and walk the same; the existing solvers' records at `p = 101` seed 2 equal the stored ones and carry no E35 field | any difference (E35 then stops) |
| G2.146 | controls | each corrupted trace detected on every call where its column is contested, outputs unchanged; the unchecked replay of a corrupted trace gives a wrong echelon, a chain fallback or an abort on `≥ 1` of the 32 calls | a corruption not detected; no unchecked failure |
| G2.147 | T's selection work | per call (Stage A mean) `≤ 1,200` (at most `3 · 384 + 1`); learning `≤ 2·10⁶` native operations per curve and arm | outside |
| G2.148 | T's multiplications | `C` median T / B in `[1.00, 1.10]` on every curve; A / T in `[1.65, 1.95]` | outside |
| G2.149 | Stage A, per-call mean `S` | T below A at `0`, `0.1` and `1` GAE on every curve, `S_A / S_T` at `1` GAE in `[1.004, 1.020]`; T below B counted and B″ at `0.1` and `1` GAE on every curve | T above A at any weight on any curve |
| G2.150 | end to end against A (E27, Karatsuba) | `S_A / S_T` in `[1.65, 1.90]` (`0`), `[1.08, 1.20]` (`0.1`), `[1.004, 1.020]` (`1` GAE) on all 20 rows: **T ranked ahead of A across the band** | any row `≤ 1` at any weight |
| G2.151 | against E25 (schoolbook) | `S_E25 / S_T` in `[3.9, 4.8]`, `[1.45, 1.75]`, `[1.04, 1.07]` on all 20 rows | any row `≤ 1` |
| G2.152 | against B, B′, B″ (E33) | T below each at `0.1` and `1` GAE on every row; at `0` GAE T at or above B (`S_T / S_B` in `[1.00, 1.10]`) and below B″ | any other order |
| G2.153 | distance to the matched rho (walk (iii)) | `S_T / rho` at `0` GAE in `[3.4·10³, 2.5·10⁵]`; rho ahead of every row at every weight; pricing every arm's lead-scan reads (E33's G2.133) reverses no rank | outside; a row below rho; a rank reversed |

**What it would say.**  If G2.145–G2.150 hold, B's pivot order is mostly a property of the curve: one
learned schedule per curve, replayed at three operations per contested column, keeps nearly all of B's
`≈ 3·10⁵` saved multiplications a call for `≈ 10³` native operations, so the saving survives `1` GAE, by
about `1 %`, because the `176,064` Macaulay entries every arm still builds dominate there.  T would be the
first B variant ranked ahead of A across the band, and the new baseline on this family.  If G2.144's
per-column agreement is low, the replay misses often, each miss costs fill toward A's, and T lands
between B and A at `0` GAE.  Either way the next lever after E35 is the matrix itself (§9.37).

### 9.41 E36 pre-registration: pivot rules whose selection work fits the band

*Registered 2026-11-02, before any E36 code.*  **Numbering:** E35 (a parallel experiment on this
branch, a trace replay of B's pivot schedule) takes §9.40/§9.42; E36 takes §9.41 (this registration),
§9.43 (results) and G2.154–G2.163.  **Question.**  §9.37 left no pivot rule ranked ahead of A across the
band: B″ is ahead of A at `0` and `0.1` GAE and behind at `1` (`0.946`–`0.948`).  Is there a rule, with
every native operation of its selection counted, that is ahead of A (and E25) at `0`, `0.1` **and**
`1` GAE on every row?  **Class, fixed in advance:** a rule ahead of A at every weight on every row is a
measured improvement of the baseline (engineering: a constant on the echelon); anything else is
reported, not ranked.

**The budget (derived from E33's records, before any code).**  Every rule spans the same row space
with the same pivot columns, so on every call it builds the same matrix (the same `macaulay_entries`,
fallbacks included) and returns the same normal forms and decompositions.  Hence, per call,
`S_A(g) − S_X(g) = (C_A − C_X) − 97 g · sel_X`.  At `1` GAE a rule beats A only if
`sel_X < (C_A − C_X)/97`.  Taking B (exact sparsest) as the best measured pivot quality,
`(C_A − C_B)/97 = 2.96·10⁵/97 ≈ 3,050` native operations per call (Stage A means; `3,054`–`3,103`
over E33's ten curves).  In the elimination each non-pivot candidate of a bucket is updated once and
moved, so `Σ_c (|bucket_c| − 1)` equals the number of row updates, `≈ 4,060` per call (B″'s `4,070`
comparisons, `8,121`/2 updates).  **Consequence:** any rule that pays at least one counted operation
per row update or per non-pivot candidate — B, B′, B″, and B″'s estimate maintained in any way
(candidate (a)) — costs `≥ 4·10³` operations, `≥ 3.9·10⁵` multiplications at `1` GAE, more than any
rule can save.  Only rules whose selection work is `O(rows)` or `O(columns)` per call (`≤ 393` rows,
`≤ 384` pivot columns) can rank.  The rules below are chosen to that budget; one rule that is
predicted to lose at `1` GAE (K) tests the bound itself.

**Counting rule** (§9.35's, extended): one native operation per matrix or equation entry examined
for its zero status, per comparison of two candidates' keys, per maintained count or key update,
per per-column branch decision that A does not make, and per row moved by a permutation.  As in
§9.35, the bookkeeping A itself does (bucketing by lead, the lead scan, A's ordering of a bucket by
row index, the swap) is common and not selection work.  **Arrival order** — the order in which rows
enter a bucket (the initial scan in row order, then pushes in elimination order) — is maintained by
that common bookkeeping and costs nothing to read; A discards it by sorting.  All rules: one new
module, `src/cryptanalysis/chain_pivot_rules.rs`, a counted echelon beside E33's
(`echelon_by_lead_counted` and every existing solver unchanged), new solver names, and a minimal
dispatch hook in `klein_v4` (a small separate commit).

- **F — arrival order** (`chain-fifo`): each column's pivot is the bucket's first row in arrival
  order.  Rows still at their initial lead (never updated, as sparse as their equation) come first;
  otherwise the row that arrived earliest.  **Selection work `0`.**  (Candidate (d), the degenerate
  static order: the build order itself.)
- **S — static order from the initial structure** (`chain-static`): once per call, each
  triangularised equation's nonzero count (one op per coefficient examined), the equations ordered by
  that count (comparisons counted; ties by equation index), and the built rows permuted into
  `(equation count, equation, build order)` blocks (one op per row moved; inverse-permuted after the
  echelon, bookkeeping); then A's rule (lowest index) on the permuted rows.  Work `≈ 113 + ≤ 45 + 393
  ≈ 5·10²`.  (Candidate (d).)
- **H — hybrid by a structural criterion** (`chain-fifo-static`): in a column whose bucket still
  holds a never-updated row, F's choice (the first such row); in a column holding only updated rows,
  S's choice (lowest permuted index).  The criterion is read from the bucket's initial length (one op
  per contested column: the branch).  Work = S's + contested columns `≈ 5·10² + ≤ 384`.  (Candidate (b):
  the estimate-free branch decided by a pre-registered, counted criterion.)
- **K — bounded Markowitz search, `k = 2`** (`chain-est-k2`): B″'s maintained estimate (inherited
  counts as B′/B″, two ops per updated row), but only the first two rows in arrival order are
  compared (one comparison per contested column).  Work `≈ 506 + 8.1·10³ + ≤ 384 ≈ 9·10³`.
  (Candidates (a)/(c); the test of the bound.)
- **Negative control L — last arrival** (`chain-lifo`): the bucket's most recently arrived row (the
  most recently updated, most filled).  Work `0`.  Must return the same decompositions and should cost
  more than A.

A fallback (the existing solver rerun) adds no selection work; the chain attempt's is charged.

**Protocol.**  *Stage A:* E33's harness setting exactly (`examples/e33_stage_a.rs` → `examples/e36_stage_a.rs`):
E25's ten curves, the first 1,000 targets of E25 arm A's stream, one thread, every solver per call from
the same per-target seed: existing, A, A counted, B″, F, S, H, K, L.  Per call: outputs, degree,
outcome, decomposition set, `C`, echelon multiplications, entries, selection work by part, lead-scan
reads, pivot digest.  *Carry rule:* a rule goes end to end if its Stage A per-call mean `S(0.1)` is below
A's on every curve; L is never carried.  *End to end:* `quartic_pipeline_gls` with `--karatsuba` (E27's
command, E33's `run_e2e.sh` pattern), `RAYON_NUM_THREADS=4` on at most two cores, E25's ten curves, both
arms.  *References:* A = E27's solver-A Karatsuba records; E25 = its own records (schoolbook), against X's
schoolbook `S` = X's Karatsuba total plus the schoolbook re-pricing of the non-solve `F_{p⁴}` work measured
on A (E31 A's total − E27 A's total, the same calls); rho = walk (iii) of E27 without the `[d]G` check
(`experiments/32_walk_karatsuba/summary.json`).  Band: `S(g) = total_muls + 97 g (macaulay_entries +
selection)`, `rho(g) = rho + 97 g · steps`.  A rank is stated only where the order holds at `0`, `0.1`
and `1` GAE on every row.  *Sensitivities* (not used for ranks): (σ1) lead-scan reads priced for every arm;
(σ2) every arm that orders a bucket by a key or an index (A, S, H's second branch) charged `|b| − 1`
comparisons per contested column; (σ3) one op per contested column charged to every rule, F and L
included.  Host: this container, at most two cores; isolab-eks is unreachable from it (no `aws`, no
`kubectl`): operation counts only, no wall-time claim.

| id | quantity | prediction | falsified by |
|:--|:--|:--|:--|
| G2.154 | identity (**kill**) | F, S, H, K, L return the existing solver's decomposition set, outcome and degree on every Stage A call and every end-to-end record; the end-to-end records equal E27's A records in every field except the solve's multiplications and what derives from them, the selection fields and the tag; A, A counted, B″ and the existing solver reproduce E33's Stage A per-call `C`, echelon and selection counts on every call; rerunning `macaulay`, `chain`, `chain-sparsest`, `chain-sparsest-est` and `chain --karatsuba` at `p = 101` seed 2 with the new binary reproduces the stored records (wall times aside).  Negative controls: E31's perturbed coefficient changes every non-empty set under each new rule; F's and L's digests differ from A's on calls with a contested column; S's permutation inverse-mapped wrong by one row is detected | any difference, or a control not detected (E36 then stops) |
| G2.155 | selection work per call (Stage A mean) | F, L `0` exactly; S in `[400, 700]`; H − S in `[1, 384]`; K in `[8.0·10³, 1.0·10⁴]` | outside |
| G2.156 | F's pivot quality | `C_A / C_F` (means) in `[1.2, 1.7]` and `C_F / C_B″` in `[0.85, 1.3]` on every curve | outside |
| G2.157 | S | `C_A / C_S` in `[0.9, 1.3]` on every curve (a static key distinguishes only equations) | outside |
| G2.158 | H against F | `C_H / C_F` in `[0.9, 1.1]` on every curve | outside |
| G2.159 | K | `C_K / C_B″` in `[0.95, 1.25]`; K behind A at `1` GAE on every curve (the bound) | outside, or K ahead of A at `1` GAE on any curve |
| G2.160 | negative control L | `C_L > C_A` on every curve | `C_L ≤ C_A` on any curve |
| G2.161 | Stage A band against A | F ahead of A at every weight on every curve; S and H ahead of A at every weight on every curve iff `C_A − C_X > 97 · sel_X` (predicted: H yes, S no); no rank decided here changes under σ1–σ3 | F behind A at any weight on any curve; a rank changes under σ1–σ3 |
| G2.162 | end to end (carried rules, 20 rows) | F: `S_A / S_F` in `[1.2, 1.7]` (`0`), `[1.02, 1.15]` (`0.1`), `[1.002, 1.02]` (`1` GAE) on every row; `S_E25 / S_F > 1` at every weight on every row — **F ranked ahead of A and E25 across the band** | F behind A or E25 at any weight on any row (F then unranked) |
| G2.163 | distance to the matched rho (walk (iii)) | rho ahead of every row of every rule at every weight; F's `S / rho` at `0` GAE in `[3.5·10³, 3.4·10⁵]` | any row below rho, or outside |

**What it would say.**  If G2.154 and G2.162 hold, a pivot rule that reads nothing — the order rows
arrive in a bucket — keeps most of B's saving with no selection work, so it is ahead of A at every
weight of the band: a ranked (small at `1` GAE, `≈ 1 %`) engineering improvement of the `k = 4` solve,
with the same relations, logs and walks.  It would also say why B″ lost: its saving was real, but it
paid for it per row update, which the band prices above any saving.  At `1` GAE the matrix entries
(`≈ 1.8·10⁵` per call) remain almost all of `S`; no pivot rule changes that, and the rows stay
`≳ 10⁴×` from the matched rho at `1` GAE.  If F does not beat A, the arrival order is not informative and
the result is a negative for candidate (d).

### 9.42 E35 measured: B's pivot schedule is not a property of the curve — the replay keeps a fifth to a third of B's saving, is ahead of A at `0` and `0.1` GAE and splits 14 / 6 at `1` GAE; unranked

*2026-11-02.*  Code `b1f5df9a45` (module `chain_trace.rs`, examples) and `2308fa4664` (dispatch hook), binaries
built from `git archive 2308fa4664` and pinned by sha256 (`experiments/39_chain_trace/binary_sha256.txt`; the
checks example rebuilt once, see below).  **Stage 0 + A:** `e35_stage_a`, one thread, E31/E33's calls exactly
(the first 1,000 targets of E25 arm A's stream on each of E25's ten curves), five solvers per call from the same
per-target seed: existing (E25), A counted, B counted, B″, T (`stage_a.log`; `stage_a.json` sha256 `6c7c8a44…`, not
in Git: the artifact store is not reachable without the AWS CLI).  **Controls:** `e35_checks`, 32 calls each of
`p = 101` seed 2 and `p = 397` seed 1 (`checks.{json,log}`).  **Identity:** `identity.sh`.  **End to end:**
`run_e2e.sh` (E33's command with `--solver chain-sparsest-trace`), ten curves, both arms, 10 runs, 0 failures.
Tables: `e35_tables` → `tables.md`, `summary.json`.  Host: the 4-vCPU container, two cores; isolab-eks is not
reachable from it (no AWS CLI, no kubectl), so operation counts only, no wall-time claim.  Report:
`research/e35_chain_trace_20261102/`.

**Stage 0: B's choices depend on the target.**  On every curve, *no* call's pivot sequence equals the learned
trace, and 997–1,000 of the 1,000 calls have pairwise distinct sequences; none of the three learning sequences
equals their per-column mode.  The shape is the trace's on 9,999 of 10,000 calls.  Per contested column (334
a call):

| p | seed | B's row = trace | (row, position) = trace | all columns, row = trace | best other curve's trace |
|--:|--:|--:|--:|--:|--:|
| 101 | 1 | `81.4 %` | `63.5 %` | `83.8 %` | `81.8 %` |
| 101 | 2 | `69.8 %` | `50.2 %` | `73.7 %` | `82.2 %` |
| 149 | 1 | `81.7 %` | `68.4 %` | `84.1 %` | `84.9 %` |
| 149 | 2 | `80.9 %` | `67.0 %` | `83.4 %` | `84.4 %` |
| 197 | 1 | `86.0 %` | `74.0 %` | `87.9 %` | `86.2 %` |
| 197 | 2 | `86.4 %` | `74.0 %` | `88.2 %` | `86.4 %` |
| 269 | 1 | `80.5 %` | `70.5 %` | `83.0 %` | `88.5 %` |
| 269 | 2 | `83.4 %` | `74.5 %` | `85.6 %` | `88.3 %` |
| 397 | 1 | `88.4 %` | `80.3 %` | `89.9 %` | `90.2 %` |
| 397 | 2 | `87.3 %` | `78.8 %` | `89.0 %` | `89.9 %` |

Agreement rises with `p` (accidental zeros, probability `≈ 1/p` per entry, move B's counts), and another curve's
trace agrees as well as the curve's own, often better (`69.3`–`90.2 %` across all pairs).  What is stable is the
generic structure of the Macaulay matrix, shared by all ten curves; what varies from call to call is the target.
The schedule is not a per-curve object.

**Stage A** (per call; means over all calls, fallbacks charged; T's learning amortised over the curve's 1,000
calls): T agrees with the existing solver on all 10,000 calls, as do A counted, B counted and B″.  T's selection
work is `744`–`817` operations a call (trace lookups `≈ 335`, tests `≈ 410–480`).  It misses on `186`–`259` of the
`334` contested columns (`55.6`–`77.5 %`; about two thirds of the misses are rows still among the candidates at
another position): one changed choice changes later columns' candidates, so the misses cascade, and every call
misses at least once.  T's `C` median is `5.23`–`5.51·10⁵`, against B's `3.04`–`3.33·10⁵` and A's `6.01`–`6.34·10⁵`:
T / B `1.571`–`1.791`, A / T `1.103`–`1.211`.  Per-call `S_A / S_T` is `1.092`–`1.254` (`0`), `1.014`–`1.049`
(`0.1`) and `0.9916`–`0.9952` (`1` GAE) on the ten curves: behind A at `1` GAE on every curve.  `S_{B″} / S_T` is
`0.705`–`0.804`, `0.969`–`1.0006` and `1.046`–`1.051`.  Learning costs `5.08`–`5.28·10⁵` multiplications and
`1.40`–`1.43·10⁶` native operations per curve (528,192 entries, B′'s selection `≈ 8.8·10⁵`, the mode 1,344).

**Controls.**  A corrupted trace entry — a row already used as an earlier pivot, or another call's pivot row of a
later column, in the first contested column where the clean replay used the trace — was rejected by the check
on 63 of 63 calls for each kind, and a trace of another shape fell back on 63 of 63; outputs were the existing
solver's on every call.  Replayed without the check, every corrupted call aborted on a zero pivot.  The abort
also follows from the trace's own invalid entries, so this shows the check is necessary, not that each corruption
alone breaks the echelon.  A first version of the controls (`checks_v1.*`) counted one corruption out of 63 as
undetected.  There the corrupted row was the trace's own entry, a no-op.  The example now excludes no-op
corruptions; only `e35_checks` was rebuilt (sha256 `ed21fbd0…`).

**Identity.**  E33's pinned binary and E35's give identical records (canonical JSON without timings) for `macaulay`,
`chain`, `chain-sparsest`, and `chain`, `chain-sparsest-counted`, `chain-sparsest-inc`, `chain-sparsest-est` with
`--karatsuba` at `p = 101` seed 2.  The four Karatsuba records also equal the stored E27 and E33 ones, and none
carries `trace_learning`.  End to end, all 20 T records equal E33's B counted records in every field except
the solve's multiplications (and `call`, `total_muls`, `ratio`), the selection, learning and lead-scan fields,
`macaulay_entries` (learning's `528,192`) and the tag.  Every relation, log and walk is the same.

**End to end** (20 rows; `S(g) = total_muls + 97 g (macaulay_entries + pivot_selection_ops)` with T's learning
inside its totals; A = E27's records, B, B′, B″ = E33's, all Karatsuba; E25 schoolbook against T shifted by the
schoolbook re-pricing measured on B; rho = walk (iii) of E27):

| X | `S_X / S_T`, `0` GAE | `0.1` GAE | `1` GAE | rank across the band |
|:--|:--|:--|:--|:--|
| A | `1.093`–`1.252` | `1.020`–`1.055` | `0.9981`–`1.0028` (T behind on 6 rows: `p = 101` both seeds, `p = 149` seed 2, both arms) | **unresolved**: T ahead at `0` and `0.1` on every row; at `1` GAE ahead on 14, behind on 6 |
| E25 | `2.727`–`3.204` | `1.421`–`1.507` | `1.0325`–`1.0557` | T ahead at every weight on every row (as A is: §9.24) |
| B | `0.598`–`0.676` | `5.19`–`5.48` | `6.55`–`6.74` | unresolved (B ahead at `0`) |
| B′ | `0.598`–`0.676` | `2.10`–`2.21` | `2.54`–`2.61` | unresolved (B′ ahead at `0`) |
| B″ | `0.709`–`0.807` | `0.975`–`1.007` | `1.053`–`1.059` | unresolved (B″ ahead at `0` on every row, at `0.1` on 18 of 20; T ahead at `1` GAE on every row) |

T's selection is `744`–`817` operations a call and learning adds `4`–`197` a call (`1.4·10⁶` per run over
`7,106`–`352,146` calls).  Its solve multiplications are `1.50`–`1.69×` B's.  **Distance to the matched rho**
(`S_T / rho`): `5.11·10³`–`3.53·10⁵` (`0`), `2.02·10⁴`–`1.35·10⁶` (`0.1`), `9.04·10⁴`–`6.63·10⁶` (`1` GAE); rho is ahead
of every row at every weight.  For comparison A is at `6.39·10³`–`4.06·10⁵`, `2.13·10⁴`–`1.39·10⁶`, `9.07·10⁴`–`6.64·10⁶`
and B″ at `4.12·10³`–`2.61·10⁵`, `2.03·10⁴`–`1.33·10⁶`, `9.58·10⁴`–`7.01·10⁶`.  **Sensitivity:** lead-scan reads are
identical for A, B and T on every Stage A call (`79,114`–`79,119`; they telescope to the shared final leads), so
pricing them for every arm moves the ratios toward 1 and reverses nothing (Stage A `S_A / S_T` `1.010`–`1.035` at
`0.1`, `0.9932`–`0.9957` at `1` GAE).

| id | result | verdict |
|:--|:--|:--|
| G2.144 | shape `99.9`–`100 %`; whole sequence `0 %` on every curve; contested-column agreement `69.8`–`88.4 %`, below `90 %` on every curve | **falsified** (agreement); the sequence depends on the target, and the curve explains less than the generic structure does |
| G2.145 | Stage A 10,000 / 10,000; end to end 20 / 20 with the registered field rule; existing solvers identical old = new (7 / 7) and = stored (4 / 4), no E35 field | **holds** |
| G2.146 | corruptions rejected 63 / 63 per kind, outputs unchanged; unchecked replay aborted 126 / 126 | **holds** (controls v2; v1's one undetected case was a no-op corruption; disclosed) |
| G2.147 | `744`–`817` a call; learning `1.40`–`1.43·10⁶` | **holds** |
| G2.148 | `C` median T / B `1.571`–`1.791`; A / T `1.103`–`1.211` | **falsified** (the replay misses `56`–`77 %` of contested columns) |
| G2.149 | T below A at `0` and `0.1`, **above at `1` GAE on every curve** (`0.992`–`0.995`); below B at `0.1` and `1`; below B″ at `1` but above at `0.1` on 9 of 10 | **falsified** |
| G2.150 | `1.093`–`1.252`, `1.020`–`1.055`, `0.998`–`1.003` (6 rows `< 1`) | **falsified**: T is not ranked ahead of A |
| G2.151 | `2.727`–`3.204`, `1.421`–`1.507`, `1.0325`–`1.0557`: ahead of E25 at every weight, below every registered band at `0` and partly below at `0.1` and `1` | **falsified** (bands); the order holds |
| G2.152 | below B and B′ at `0.1` and `1`; above B″ at `0` and at `0.1` (18 / 20), below at `1`; `S_T / S_B` at `0` is `1.48`–`1.67` | **falsified** |
| G2.153 | `S_T / rho` at `0` GAE `5.11·10³`–`3.53·10⁵` (above `2.5·10⁵` on one row); rho ahead everywhere; sensitivity reverses nothing | **falsified** (upper end); the rest holds |

**What it says.**  Replaying solver B's pivot order does not work on this line, because there is no
per-curve order to replay.  B's choice among candidates with nearly equal counts is decided by accidental zeros
of the target's coefficients and fill: no two calls of a curve share a sequence.  The 70–88 % of contested columns
that do agree are generic structure, which the other nine curves' traces predict as well.  A counted replay with an
`O(1)` check therefore keeps B's choice in only `23`–`44 %` of contested columns.  It keeps `18.8`–`36.8 %` of the multiplications B saves over A (`C` medians; A / T `1.10`–`1.21`
against A / B `1.90`–`1.98`), for `≈ 800` native operations a call.  At `1` GAE that is break-even with A: ahead on 14 rows, behind on 6, by at most `0.3 %`
either way.  **Under the 2026-10-15 criterion T is not an improvement**: it is ahead of the baseline A at `0`
and `0.1` GAE only, so it is unranked.  At those weights it is behind B″ (E33), which also stays unranked.  It
does have the smallest 1-GAE cost of any sparsest-pivot variant tested (`S_{B″} / S_T` `1.053`–`1.059`).
**Class:** engineering, negative.  A stays the ranked solver, and the distance to rho is unchanged:
`6.39·10³×` at `0` GAE on the best row (`p = 397` seed 1, arm B; T `5.11·10³×` there, unranked).
This agrees with E21 (§9.11) on a different object.  The per-call structure of the `V₄` systems differs from
target to target in exactly the places a trace would need to fix: E21's vanishing rows, and here the tie-breaks
among nearly equal rows.  Joux–Vitse trace reuse (`KR-IC-f3da82`) presupposes systems whose reduction pattern is
generic; at `p ≤ 397` with `≈ 450` columns, accidental zeros are too frequent for that.
**Not done:** a membership test by a row-to-lead map instead of the position test.  It would recover up to the
`≈ 2/3` of misses whose row was still a candidate (an upper bound, exploratory, not registered).  Its bookkeeping
would have to be priced, and it could not exceed the `70`–`88 %` per-column agreement above.  Also not done: the
validation gate, the leaderboard and `benchmarks/`.  **Next lever:** unchanged from §9.37, the Macaulay matrix
itself (`176,064` entries a call, all of the `1`-GAE cost).
