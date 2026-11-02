# Sub-exponential ECDLP over prime fields: a desk evaluation against the lead atlas (2026-11-01)

> **Historical snapshot, not the current pursuit queue.** The counts,
> dispositions, PDF, and generated data below preserve the atlas at
> `cfc1dae2` (154 leads). The canonical reconciliation later catalogued 160
> leads, keeps 139 under active pursuit, and excludes only 21 exactly scoped
> mathematical obstructions. The kernel-checked JKSST audit also showed that
> the classic xedni end-to-end obstruction is conditional: selected-lift,
> height, good-reduction/useful-relation, cofactor, and adaptive-trial bridges
> remain open. Accordingly, `LEAD-TRF-global-lift-small-height` and
> `LEAD-ICP-char0-lifting-xedni-smoothness` are active next-step leads; broad
> “xedni is closed” statements below are superseded. The scoped fixed-arity CM
> obstruction remains closed only within its stated model.

**What this is.** The owner asked for "drastic changes to solve ECDLP in
sub-exponential time" with "exotic ideas". This note is the desk evaluation
that AGENTS.md ("Pointers": read the lead atlas and run `--match` before
proposing an ECDLP mechanism) requires before any such proposal: sixteen
candidate mechanisms, each matched against the 154-lead atlas of
`aburan28/crypto-autoresearcher` (`knowledge/leads/ecdlp/`, `main`
cfc1dae2, reviewed 2026-10-30), the theorem-level obstructions the atlas
already records for prime fields, and the open leads whose next step is
cheapest and most decisive.

**Classification.** Desk evaluation. **No improvement is claimed and no
research state changes.** No mechanism below is new to the program: every
one of the sixteen lands on an existing lead (figure 2). The measured
distance to rho on the prime-field ladder is unchanged (Table B of
`docs/ic/LEADERBOARD.md`: 5.5–13.5× at 10–24 bits; the 20-bit engineering
baseline of 2026-10-29 at 7.75× at w = 0).

Report and figures: [`REPORT.pdf`](REPORT.pdf) (source `REPORT.typ`),
[`figures/atlas_tally.svg`](figures/atlas_tally.svg),
[`figures/candidate_matches.svg`](figures/candidate_matches.svg).
Data: [`data/atlas_tally.json`](data/atlas_tally.json) (every lead's family
and disposition, read from the YAML files) and
[`data/candidate_matches.json`](data/candidate_matches.json) (the raw
`--match` output for each candidate). `plot.py` lays the two out; it
computes nothing.

## 1. The shape of the question

A sub-exponential algorithm for the ECDLP on a prime-order subgroup of
`E(F_p)` cannot be generic: Shoup's bound (`LEAD-RHO-generic-sqrt-lower-bound`,
closed, theorem) puts every algorithm that uses only the group law and
equality at `Ω(√N)`, and with advice `S` at `S·T² ≳ N`. So it must read the
encoding: coordinates, the curve equation, the field. The atlas sorts what
the program has tried into the routes by which an encoding can be read, and
for prime fields each route ends at a named obstruction:

| route | lead (disposition) | the obstruction, as the atlas states it |
|:--|:--|:--|
| a factor base with cheap membership, so that point decomposition becomes an algebraic system (Semaev) | `LEAD-ICP-algebraic-membership-factor-base` (closed, theorem) | a target-independent algebraic locus of degree `d` yields `O(d)` usable points, so a size-`B` algebraic base over `F_p` has description degree `≥ B/3`; mean yield is `C(B+m−1, m)/N` for every base of size `B`, measured ratio 1.000000 to a random base at `N ~ 2^14..2^18` |
| small-`x` bases solved as a small-root problem (Coppersmith) | `LEAD-ICP-coppersmith-smallroot-semaev` (closed, proof) | every Coppersmith lattice on a convex monomial set reaches at most `0.3293` per `S_3` block in three unknowns (`0.3536` in two); the index-calculus demand at `(m, w) = (4, 2)` is `0.8889` |
| the base-`B` digit / subset-sum form with representations | `LEAD-ICP-representation-subset-sum` (closed, theorem) | uses only group operations and equality: generic, so `T ≳ √N` |
| enumerative, pair-table and MITM decomposition oracles at any arity | `LEAD-ICE-generic-oracle-arity-cost-floor` (closed, cost model) | the replicated cost model has no sub-rho cell for ENUM/MITM oracles; a free oracle at `m = 3` still costs `2^68.58` against rho's `2^60.81` on ECC2K-130 |
| p-adic lifts: `Z/p²`, jets, canonical and Teichmüller lifts, formal-group digits | `LEAD-EXO-padic-jet-canonical-lift`, `LEAD-TRF-padic-canonical-section` (closed, proof); `LEAD-TRF-anomalous-padic-lift` (closed, known) | for `gcd(n, p) = 1` the prime-to-`p` torsion lifts uniquely with zero formal defect and multiplication by `ℓ` is invertible on every jet target, so every additive order-`ℓ` image is zero; jet oracles are GGM-simulable with `O(1)` overhead |
| multiplicative transports: pairings, self-pairings, distortion maps | `LEAD-TRF-mov-small-embedding-degree` (closed, known), `LEAD-ICP-self-pairing-prime-order`, `LEAD-TRF-one-point-self-pairing` (closed) | a homomorphic image in a multiplicative group factors through the embedding degree `k = ord_r(p)`; no nondegenerate self-pairing on a prime-order subgroup |
| covers and Jacobians over the same field | `LEAD-TRF-same-field-genus-g-embedding` (closed) | index calculus in a genus-`g` Jacobian over `F_p` costs `p^{2−2/g} ≥ p` for `g ≥ 2` against rho's `p^{1/2}`; the small-genus regime never reaches `L(1/2)` |
| the isogeny class, volcanoes, endomorphisms | `LEAD-STR-isogeny-class-generic-invariance`, `LEAD-STR-small-cm-disc-nonunit-endo`, `LEAD-STR-endo-scalar-action-generic` (closed) | generic cost is an isogeny-class invariant; automorphisms give `√|Aut|` and nothing beyond |
| global lifts: xedni, number fields, function fields | `LEAD-ICP-char0-lifting-xedni-smoothness`, `LEAD-TRF-global-lift-small-height` (closed, known); `LEAD-EXO-global-lift-height` (open) | the Q-lift variant is closed by the literature (xedni exponent 3 for the named family); the function-field section timing is the open next step |

Nothing in this table is new; it is the atlas read in one direction. The
point of writing it down is that the "exotic" candidates below are
instances of these routes, and the reader can see which obstruction each
one meets.

## 2. Sixteen candidates, each against the atlas

`python3 tools/lead_atlas.py --match "<idea>"` for each candidate; the best
hit, its disposition and the reading (raw output: `data/candidate_matches.json`).

| # | candidate mechanism | best-matching lead | disposition | reading |
|--:|:--|:--|:--|:--|
| 1 | tag tracing / partial-product rho steps (Cheon–Hong–Kim) | `LEAD-RHO-cheaper-step-primitives` | open, unexamined | engineering of the step constant, not an exponent; the lead's next step is a ported tag-tracing walk on a 64-bit toy curve |
| 2 | polynomial-interpolation degree bounds for the EC discrete log (Lange–Winterhof 2003; WCC 2005) | `LEAD-EXO-invertible-hint-subset` | open, next step | the lead's next step measures the maximal agreement `A(d)` of degree-`d` polynomials with the log graph; the published lower bounds on interpolation degree bear directly on it and may close the low-degree variant by literature (statements not retrieved here: unverified pointer, see §4) |
| 3 | Tate-curve / multiplicative uniformisation transport | `LEAD-TRF-padic-canonical-section` | closed, obstructed | a multiplicative image of the prime-order subgroup factors through the embedding degree; the `p`-adic side lifts torsion to torsion |
| 4 | quadratic-twist trace map `E(F_p²) → E(F_p)` as a decomposition | `LEAD-TRF-cover-catalog-jacobian-transfer` | open, blocked | the trace map splits `E(F_p²) ≅ E(F_p) ⊕ E'(F_p)` when the orders are coprime; the `F_p` part is the original problem; the catalogued pullbacks are the executable form and are blocked on their constructors |
| 5 | sum–product / additive-combinatorics structure of `x`-coordinate sets | `LEAD-EXO-dl-character-sum-pseudorandom` | exhausted at scale | character bias on structured bases read as chance at the reachable scale |
| 6 | special-form primes (Solinas, Mersenne) as algorithmic structure | `LEAD-ICB-nonlinear-frob-stable-bases` | open, next step | a weak match: the form of `p` speeds field arithmetic (engineering) and gives the group no algebraic structure; the SNFS-style lift is `LEAD-TRF-global-lift-small-height` (closed, known) |
| 7 | Weil restriction of `E/F_p²` to an abelian surface over `F_p` | `LEAD-ICP-extension-field-embedding` | closed, obstructed | embedding `E(F_p)` into `E(F_{p^n})` buys a subfield factor base for a larger group, not for the original one |
| 8 | `x`-only Montgomery-ladder walks | `LEAD-EXO-curve-model-representation` | closed, obstructed | models and representations change constants only |
| 9 | SIMD / vectorised field arithmetic and batched inversion | `LEAD-ICP-prime-ic-constant-engineering` | open, next step | engineering; this session's C-library step change (cryptanalysis PR, `experiments/rho-ec-step-20261101`) is in this lane |
| 10 | black-box fields, the algebraic group model (Boneh–Lipton; Maurer) | `LEAD-TRF-same-field-genus-g-embedding` | closed, obstructed | a weak match; Boneh–Lipton's sub-exponential black-box *field* algorithm needs a field, and `E(F_p)` has one operation |
| 11 | Maurer–Wolf auxiliary curves of smooth order | `LEAD-RHO-cheon-deployed-applicability` | open, blocked | a reduction between DH and DL, not an algorithm for DL; the auxiliary-input lane (Cheon) is where the program keeps it |
| 12 | Coleman integration / Chabauty-style `p`-adic abelian integrals | `LEAD-TRF-padic-canonical-section` | closed, obstructed | the `p`-adic elliptic logarithm is Smart's channel; it needs `r = p` |
| 13 | Wagner `k`-tree / generalised-birthday joins | `LEAD-EXO-four-list-ktree-join` | open, next step | the next step is the bucket-variance class `W` of the O2 agreement kernel; the `j = 2` four-tree is closed at exponent `0.4167` for character filters |
| 14 | `Z/p²` Hensel lift of the prime-order subgroup (Smart's channel for `r ≠ p`) | `LEAD-TRF-padic-canonical-section` | closed, obstructed | the lift defect is multiplied by `r`, which vanishes mod `p` only when `r = p`; with a canonical lift the torsion lifts to torsion and the image is zero |
| 15 | split genus-2 Jacobian / gluing `E × E'` transfer | `LEAD-TRF-same-field-genus-g-embedding` (also `LEAD-EXO-split-jacobian-projection`) | closed, obstructed | genus-2 index calculus over the same field costs `p`; the program measured `ic.hyperelliptic` at `1.47×` rho at 2^17–2^20 (2026-10-25) with exponent `q¹` |
| 16 | class-group action / ideal-class vectorisation bridge | `LEAD-STR-class-group-ecdlp-bridge` | open, next step | the one structural lead whose closure would be a theorem: a simulation lemma for a curve-aware generic group model with free class-group action |

Sixteen of sixteen collide. The program's rule for this case (atlas README,
rule 4) is to advance the matched lead's `next_action`, not to file a new
lead.

## 3. What a "drastic change" can honestly mean here

1. **For prime fields, the exponent is `1/2` on every route the program
   has measured, and the routes that could lower it are the open algebraic
   ones.** The open leads with a decided next step and a plausible bearing on
   the exponent are: `LEAD-ICP-digit-presentation-solving-degree` (an `F_p`
   analogue of Weil descent; its measured first-fall degree so far is
   curve-independent and reproduced by a curve-free null), `LEAD-ICP-pkm-tower-algebraic-oracle`
   and `LEAD-ICP-lattice-ic-relation-construction` (blocked on constructors),
   `LEAD-STR-class-group-ecdlp-bridge` (a theorem would close it),
   `LEAD-EXO-invertible-hint-subset`, `LEAD-EXO-miller-pairing-cocycle` /
   `LEAD-TRF-miller-cocycle-nonhom-transfer`, `LEAD-EXO-four-list-ktree-join`
   and `LEAD-EXO-global-lift-height`. None of them has a measured excess over
   the matched null; each has a cheap, decisive next step (§4).
2. **Where sub-rho exponents are known is extension fields**, `E(F_{p^n})`
   with `n = 3, 5` (Gaudry, Diem, Joux–Vitse). The program's open leads there
   (`LEAD-ICP-gfp5-symmetrized-gaudry`, `LEAD-ICP-gfp5-oa-sdhp-jv`,
   `LEAD-TRF-jv-sieve-fp6-weak-class`) are the only places where a
   below-`√N` exponent is a measurement away rather than a conjecture away.
   Moving effort there is a change of target, not a prime-field result.
3. **The ratchet is the right frame.** The owner's success criterion
   (AGENTS.md, 2026-10-15) counts every measured improvement over the current
   index-calculus baseline; the prime-field baseline moved on 2026-10-29
   (`N = 45`, `0.818 [0.773, 0.866]` at w = 0, 20 bits) and the relation
   phase is 68–93% of `S` on Table B. Constant-factor and oracle work there is
   what moves the distance to rho today.

## 4. Open next steps, ranked by cost and by what each outcome decides

| rank | lead | next step (from the atlas) | cost | what each outcome decides |
|--:|:--|:--|:--|:--|
| 1 | `LEAD-ICP-digit-presentation-solving-degree` | the certified finite degree-growth ladder with blocking controls on the digit system at `m = 2, 3` over `s = 2..8`, independent primes per `s`, support-matched and block-factored nulls; then the adaptive staircase arm on the 12/16/20-bit panel | toy sizes, CPU-hours | `d_lf` growing at the semi-regular rate closes the lane as exhausted at scale; a `d_lf` bounded in `s` strictly below the null is the first elliptic early fall over `F_p` |
| 2 | `LEAD-STR-class-group-ecdlp-bridge` | write and review the simulation lemma: in a curve-aware generic group model where the adversary also gets free class-group action, ECDLP still needs `Ω(√N)`; state quantifier order and the size-gap argument (`h ~ p^{1/2}` vs `N ~ p`) | desk / Lean, no compute | a checked simulator closes the lane at exponent `1/2` with a theorem; a place where the simulation fails names the one bridge a reduction must use |
| 3 | `LEAD-EXO-invertible-hint-subset` | first the literature: the interpolation-degree lower bounds for the EC discrete log (Lange–Winterhof, "Interpolation of the elliptic curve Diffie-Hellman mapping", 2003; "On degrees of polynomial interpolations related to elliptic curve cryptography", WCC 2005) against the lead's `A(d)` question; then the measurement on the toy ladder of H-ECDLP-f6a43a if the bounds leave room | desk, then toy | bounds that force degree `≥ c·|S|` on any set of size `|S|` close the low-degree variant as `closed_known`; otherwise the measurement decides as the lead states |
| 4 | `LEAD-TRF-miller-cocycle-nonhom-transfer` / `LEAD-EXO-miller-pairing-cocycle` | verify K1/K2 of IDEA-20261017-e3a6f1 on toy prime-order curves with `gcd(N, p−1) = 1`; equidistribution of `ℓ(k) = log_g U(kP)` with a planted-structure positive control | toy, hours | equidistribution at the `1/p` rate closes both (a non-homomorphic map of this kind is a hash); if `r_k` needs `k` the primitive collapses into the closed elliptic-net lead |
| 5 | `LEAD-EXO-four-list-ktree-join` | measure the relative bucket variance `W` of the O2 agreement kernel across `N` and fit its decay against `1/√N` | toy, CPU-hours | slower decay reopens a bucket-concentrated filter; `1/√N` extends the bucket-averaged barrier to a bucket-resolved one |
| 6 | `LEAD-EXO-global-lift-height` | time `T_section(d, χ, p)` for a bounded-height section through a prescribed fibre point on `E1` surfaces at the Gaussian-heuristic degree, `≥ 3` primes, random-system Gröbner control | Gröbner runs, CPU-days | `T > p^{1/2}·poly(log p)` at the random `d_reg` closes the function-field variant; `T ≤ p^{0.45}` on `≥ 3` primes is a signal |
| 7 | `LEAD-RHO-cheaper-step-primitives` | tag-tracing r-adding walk on a 64-bit prime toy curve, ns per useful step paired against the plain step at 8–1024 walks per inversion | engineering, hours | `< 0.95` opens an engineering lead for the pool and GPU kernels; `≥ 1.0` closes it (cost model) |

## 5. What this note does not do

It proposes no new lead, measures nothing, and changes no disposition
(atlas rule 2: dispositions move by Coordinator decision). The literature
pointer in row 3 of §4 is a pointer: the theorem statements were not
retrieved in this session, so it is recorded as unverified. The sixteen
candidates were chosen by the author of this note from the standard
cryptanalytic toolbox and from the routes in §1; a different author would
pick others, and the atlas's `--match` is a word-overlap score, so the
"best lead" column is the author's reading of the hits, not the tool's
verdict.

## 6. Reproduce

```sh
# in a checkout of aburan28/crypto-autoresearcher at main (cfc1dae2 or later)
python3 tools/lead_atlas.py --brief
python3 tools/lead_atlas.py --match "class group action vectorization bridge Cl(O) ECDLP"
# this directory
/tmp/report-venv/bin/python plot.py          # figures/ from data/
/tmp/report-venv/bin/python -c "import typst; typst.compile('REPORT.typ', output='REPORT.pdf')"
```
