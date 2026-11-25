# New relation paradigm — cross-lane synthesis for ECDLP index calculus

Date 2026-11-24. Idea-Synthesist memo. Desk synthesis, nothing run. Every
figure is `[pcc:unverified]`, quoted from the named record, not re-measured.
research-kb unavailable in-container; grounding by direct `research/`,
`problems/`, `docs/` reads. Two `idea` records follow the memo.

Scope note: I do not re-score any recorded result. Where two lanes disagree I
say so and name the test that resolves it rather than choosing a side.

## The portfolio picture, in one paragraph

Three consecutive IC levers have each landed on the same place: yield is an
exact invariant (`cost_per_relation_analysis_20261123`, KN-FIND-007), the
biased-sampler escape reduces to the constant `|Aut|` fold; per-attempt cost is
toy-measured engineering; and the solving-degree/tower lever is **OPEN,
strongly leaning KILL** (`solving_degree_tower_frontier_20261124`). The lanes'
own verdict: "closing [the exponent gap] needs either a new relation paradigm
or the degree-slope lower-bound proof." This memo finds that the degree-slope
lower-bound proof **is itself a connection no single lane names**, and that two
lanes have independently built the two halves of its resolution while
disagreeing about the answer.

---

## Connection A (developed): the PKM degree-slope question is an open problem in *algebraic proof complexity*, and two IC lanes have each built half of its resolution without citing the other

### The mechanism, stated once

The quantity the whole tower lever turns on — the solving degree `D(m,t)` of the
split-Semaev tower presentation — is, by the lanes' own reductions, a
**polynomial-calculus (PC) refutation degree**:

- `research/chain_sd_bounds_20261115/REPORT.md` §1 defines
  `ref(I_k) := min{c : 1 ∈ V_{F,c}}` as "the Clegg–Edmonds–Impagliazzo
  polynomial-calculus refutation degree with the generators as axioms, over
  `F_p`", and §3 Theorem 2 proves `sd = ref = d_F = LFD` for a unit instance [pcc:unverified]
  (Lean-checked, standard axioms). So "solving degree" and "PC degree" are the
  **same number** for these systems.
- `research/prime_leads_chase_20261113/L1-pkm/THEORY.md` casts the m-block chain
  version as `Deg_PC(m,t)`, proves the **one-free-block barrier** (Prop R: [pcc:unverified]
  restrict all but one block ⇒ `Deg_PC ≤ C(m) = O_m(1)`, t-independent), and —
  this is the load-bearing sentence nobody connected to the literature — names
  the exact uncarried-out technique: *"a two-block expansion/'design' argument in
  the Fourier basis of `μ_{2^t}` (as in Buss–Grigoriev–Impagliazzo–Pitassi for
  Tseitin mod q), where the tower encoding makes the degree the popcount of the
  character index."*

The tower normal-form degree **is** the popcount of the character index
(`chain_sd_bounds` §2 Theorem 1: `deg(NF m) = wt(NF m) = popcount`). That is
exactly the degree measure BGIP use for Tseitin-mod-`q` PC lower bounds. The
one-free-block barrier is the proof-complexity statement that a single Semaev
block has **no expansion**; a growing lower bound needs the **joint expansion of
two coupled 2-power chains**, which is the Tseitin-on-an-expander pattern. So
the repo's "missing theorem that would close PKM" is a named open problem in
algebraic proof complexity, and the transferable method (BGIP two-source
character-design dual functionals) has a precise address.

### Why it is invisible lane-by-lane

- The `chain_sd_bounds` lane built the **dual-functional machinery** for the
  simplest coupling (m=2, two 2-power chains): §7.3 "Dual (co-closed) form", the
  boundary functionals, Theorem A/A′, the cofactor reduction to **Missing Lemma
  Π**, and a counting heuristic (§13.4). It never used the word "BGIP" or
  connected its co-closed functionals to the expander-design construction that
  is known to work where its *naive* boundary evaluations fail (§7.3: the
  evaluation annihilator fails at `(k=6,c=4)`, 188/605 rows, yet `1 ∉ U_4`).
- The `solving_degree_tower` lane works at the **decisive arity** (m≥8) and
  derives the KILL from the **slope** `σ = dD/dN ≈ 0.125–0.2`, treated as
  constant, through a dense-multilinear column-count model (s = 3.25, 3.90, 4.35
  for m = 4, 6, 8). It never reads the `chain_sd` lane's asymptotic prediction.
- Neither names that the object is one PC-degree problem, so neither sees that
  **their two measurements disagree about the asymptotic slope** (Connection B).

### The exact claim (falsifiable)

A BGIP-style two-source character-design — a family `Λ_c ⊆ Q_{≤c}^*` of
co-closed dual functionals built from characters of `μ_{2^t} × μ_{2^t}`,
weighted by the joint 2-adic expansion of the two free blocks — annihilates
`U_c(k)` and separates `1` **exactly at the measured LFD staircase**
(`4,4,4,4,5,5,5,5` for k = 2..9 on F1/F2, `chain_sd` §8), i.e. succeeds at
`(k=6,c=4)` where the naive boundary-evaluation functional of §7.3 fails. Its
design-degree is governed by `Π(w)` (§13.2 Corollary A), whose growth decides
linear vs sub-linear.

### Pre-registered prediction

The character-design's attainable degree tracks `Π(w)`, measured `Π(3)=6`,
`Π(4)≥10` (`chain_sd` §13.3, two independent solvers agree). Prediction, stated
in the direction that discriminates: the design **succeeds at `(k=6,c=4)`**
(unlike the naive functional) **and** its degree grows like the counting
heuristic `m(w) ~ w·log₂ w` — i.e. **sub-linear**, matching `Π(3)=6, Π(4)=10,
predicted Π(5)=14`. If so, **Missing Lemma Π is false** and the degree-slope
route **cannot** close PKM: `LFD` is `Ω(k/log k)`, not `Ω(k)`.

### The cheapest discriminating test

Desk + the existing native `chainsd` engine (Rust, already written,
`research/chain_sd_bounds_20261115/native`), no cloud:
1. Construct the character-design dual functional at m=2 for `(k,c) ∈
   {(6,4),(7,4),(10,5),(11,5)}` — the four cells where truth, reachable and
   Theorem A′ already disagree by known constant offsets (`chain_sd` §13.8
   ladder table).
2. Check annihilation of `U_c(k)` (reuse `bnd`/`cofactor` closures already
   computed) and whether `λ(1)≠0`.
3. Read off the design-degree slope in `c` and compare with both `Π(w)` (§13.3)
   and the `solving_degree` lane's `σ`.

One desk construction + replays of closures that are already cached. This is the
"cheapest experiment that closes two open questions at once": it decides
`chain_sd`'s Missing Lemma Π **and** reconciles it with `solving_degree`'s
constant-slope KILL.

### Falsification target, declared in advance

The connection is **killed** if the character-design functional **fails at
`(k=6,c=4)` exactly as the naive boundary evaluation does** (same rank
deficiency, no better separation) — that would mean the BGIP technique carries
no further than the evaluation functionals already tried, the two-free-block
expansion is not exploitable in this basis, and L1-pkm's named route is a dead
end for this system. Separately, if the design-degree grows **linearly**
(`Π(w) ≳ αw`, contradicting the `w·log₂w` heuristic), then Missing Lemma Π is
*true*, the degree-slope lower bound closes, and the PKM/tower lever is cleanly
**KILLed** (high portfolio value, but not an exponent route).

### Prior art, with paths

- `research/chain_sd_bounds_20261115/REPORT.md` (PC/`ref`/`d_F` identity, dual
  form, Theorem A/A′, Missing Lemma Π, Π measurements, Lean proof map).
- `research/prime_leads_chase_20261113/L1-pkm/THEORY.md` (Prop R one-free-block
  barrier; names BGIP Tseitin-mod-q as the uncarried-out two-block route).
- `research/solving_degree_tower_frontier_20261124/README.md` (σ slope, derived
  s, OPEN-leaning-KILL verdict, "the repo's own missing theorem").
- Published: Clegg–Edmonds–Impagliazzo (PC refutation degree); Buss–Grigoriev–
  Impagliazzo–Pitassi, *Linear gaps between degrees for the polynomial calculus
  modulo distinct primes* (Tseitin-mod-q character-design degree lower bounds);
  Ben-Sasson–Impagliazzo (near-optimal PC degree via expansion). Confirm each
  `source_id` via research-kb before any novelty claim is written into a PR —
  research-kb was unavailable here, so "BGIP never applied to the Semaev tower"
  is an `rg`-only novelty statement, not a KB-checked one.

### Lean obligation this would create

If the design succeeds: a Lean statement `Λ_c annihilates U_c(k)` co-closed,
i.e. the dual of `chain_sd`'s existing `closure_le_F`/`not_mem_closure_of_*`
family — a direct extension of `Cofactor.lean`. If it refutes (Π sub-linear):
Lean-formalize `Π(w) = o(w)·` upper bound, closing T9 negatively.

### Honest probability

- P(the character-design separates where naive evaluation failed, at m=2): ~0.4.
- P(it resolves the Missing-Lemma-Π / constant-σ disagreement either way): ~0.7
  — this is cheap and the two lanes genuinely disagree, so a measurement is
  informative regardless of direction.
- **P(it opens an exponent-changing route for ECDLP): ~0.08.** The most likely
  resolution (sub-linear degree) removes the KILL's *confidence* but runs into
  the column-count wall (Connection B): `chain_sd` §13.6 shows `1` enters the
  closure near full rank (0.84–0.997), and k=15 already needs 2.6M columns /
  ~25 TB. A sub-linear *degree* with a super-polynomial *matrix width* is still
  not a win. The exponent route needs **both** a sub-linear degree **and** a
  sparse/structured solver that beats dense Macaulay width — the second half is [pcc:unverified]
  untouched by this test.

---

## Connection B (measured refute/rescue): the two lanes' slope measurements contradict, and one measurement resolves it

This is the "measured result in one lane that refutes an assumption in another"
the task asks for.

- **Refutation.** `solving_degree_tower` derives its KILL (s = 3.25–4.35) by
  extrapolating a **constant** slope `σ ≈ 0.125–0.2` from reachable cells
  (m≤4) to the decisive m≥8 scale. `chain_sd_bounds` §13.4 measures the
  cofactor growth `Π(3)=6, Π(4)≥10` and fits the counting heuristic `m(w) ~
  w·log₂ w`, stating explicitly: *"asymptotically ... only `Ω(k/log k)` ... if
  the heuristic is sharp, Missing Lemma Π is false."* A slope that **decays**
  like `1/log t` means the KILL's constant-σ extrapolation **overstates** the
  asymptotic solving degree, so the derived `s` is an **upper** estimate, not
  the asymptotic value. The KILL is therefore not as secure as the [pcc:unverified]
  `solving_degree` memo's "strongly leaning" reads — exactly the caution that
  memo's own §"Correction to this session's interim framing" already half-states
  for a different reason.
- **Rescue.** The same `chain_sd` lane **rescues** the KILL on the other axis:
  §13.6 shows the constant `1` enters the closure at 84–99.7% of full rank, and
  the k=15 upper certificate needs `C(30,≤7)=2.6M` columns. The cost is the
  **column count** `C(2k,≤D)`, which is the `solving_degree` lane's
  "dense-multilinear column-count model" — the very model that produced the
  KILL. So the degree may decay while the width does not, and the whole-solve
  cost stays above rho parity.
- **The single measurement that resolves it.** Does `D(m,t)`'s slope decay
  (favoring a lower asymptotic exponent, reopening) or does `C(2k,≤D)` dominate
  regardless (confirming KILL)? Connection A's test measures the degree slope;
  pairing it with the already-recorded `curve25519_exponent_study` /
  `pkm_odd_tower_f4` whole-solve fits (`s≈2.77–3.6`, F4 `~S^{3.1–3.6}`) at
  matched cells settles which term dominates. Neither lane ran both halves.

I do **not** choose a side here. The contradiction is real and the resolving
test is in Connection A's record.

---

## Connection C (ruled out, with a cross-lane reason): no isogeny/cover/correspondence carries the ECC2K-130 or a prime-field elliptic DLP into a genus-3 (or any lower-exponent) setting

### Why the genus-3 exponent-1 result cannot be borrowed by transfer

The genus-3 "exponent 1" advantage (Diem/Smith plane-quartic, `Õ(q)` vs rho
`q^{3/2}`; `docs/ic/SUCCESS_CRITERIA.md` §5 table) is a property of a DLP in a
group of **dimension 3 / order `~q^3`**. An elliptic DLP is dimension 1, order
`~q`. Three lanes together prove transfer cannot manufacture the missing [pcc:unverified]
dimension:

1. **Order is preserved, so hardness is preserved.**
   `problems/prime-field-chained-transfer-no-advantage.md` (T7 collision
   equivalence, Tate): a DLP-preserving `E→E'` exists iff `#E=#E'`; every
   genus-2/3 destination reached by a verified chain embeds `⟨P⟩` as a strict [pcc:unverified]
   subvariety, and **all IC relations on the transferred subgroup degenerate to
   fibres** = the matched collision search on `E`. Measured across 189,330
   genus-2 relations and 28,815 genus-3 relations; `φ2_*φ1^* = 0` on 300/300.
2. **The strict-subvariety screen, measured.**
   `research/n83_genus3_image_solver_20261031/DESTINATION_CALCULUS_SCREEN.md`:
   pushing an order-`r≈q` (81-bit) elliptic subgroup into a genus-3 Jacobian of
   ambient scale `q^3` lands it where **Gaudry's relation analysis fails**
   (ePrint 2004/073 Remark 2 — strict abelian subvariety), and the `q^{4/3}`
   genus-3 IC cost is **70.17 bits ABOVE rho** on the actual order. The
   exponent-1 comparison is against rho in a `q^3` group and cannot be reused.
3. **The only dimension-raising mechanism needs a composite extension degree
   with a small factor — which the targets do not have.**
   Weil restriction / GHS of `E/F_{q^n}` gives a dim-`n` AV over `F_q`, genus
   `~2^{n-1}` (`docs/ic/SUCCESS_CRITERIA.md` §5; `problems/ghs-higher-magic-
   descent-not-executable.md`). For **ECC2K-130, n=131 is PRIME** — the only
   subfield is `F_2`, forcing genus `≥ 2^{130}`; the dedicated modular/CM
   carrier lane (`problems/koblitz-modular-carrier-transfer.md`, 29 rounds)
   reaches genus **45,589** at n=131 and still has **no evaluated
   subgroup-preserving map**, let alone a sub-rho cost. For **prime-field
   curves, n=1** — there is no extension and no Weil restriction at all.
   `SUCCESS_CRITERIA.md` lines 284–286 state it directly: "For ECC2K-130,
   m=131 is prime, so descents that need an intermediate subfield do not apply
   directly."

### The exact claim that is falsified (i.e. ruled out)

"There exists a computable homomorphism from `⟨P⟩ ⊂ E_0(F_{2^131})` (or a
prime-field `E(F_p)`) onto an order-`r` subgroup of `Jac(C)` for a genus-`g`
curve `C/F_{2^m}` with `m | 131`, `m < 131`, and `g` small enough that the
destination IC exponent beats rho on `E`." This is **false by arithmetic**: [pcc:unverified]
`131` prime ⇒ `m=1` ⇒ `g ≥ 2^{130}`; prime field ⇒ no `m<n`. The dimension the
genus-3 result needs is unreachable from a dimension-1 source over a prime
extension degree. Falsifier (what would REOPEN it): a curve family where `n` has
a small factor and the instance is ECC2K-130-fidelity under the §8 gate — none
exists for the pinned targets.

### Honest probability

P(an exponent route for ECC2K-130 or prime-field curves via a genus-3/cover
transfer) ≈ **0** for the pinned targets; the obstruction is the prime
extension degree, not a missing construction. This rules the candidate out and
sharpens *why* the pinned **hyperelliptic genus≥2 goal** is the live one: its
source is already dimension ≥2, which no elliptic transfer can synthesize.

---

## Single most promising connection

**Connection A** (with B as its measured motivation): the PKM/tower
solving-degree lever is an open algebraic-proof-complexity degree problem; the
`chain_sd_bounds` and `solving_degree_tower` lanes have independently built the
two halves of its resolution and **disagree about the asymptotic slope**; the
cheapest desk test — a BGIP character-design dual functional at m=2 on closures
already cached — decides `chain_sd`'s Missing Lemma Π and reconciles it with
`solving_degree`'s KILL in one shot. Low probability of an exponent win (~0.08),
moderate probability (~0.7) of permanently resolving the portfolio's #1 open
theorem either way.
