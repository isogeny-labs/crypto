# Weak points in the ECDLP hardness corpus: triage, 2026-11-03

**Question.** Where is the claim graph that supports ECDLP hardness weakest, and which
weak points can be broken cheaply enough to attack first?

**Scope.** The assembled corpus (version 2026-11-01: 541 claims, 909 edges, 233 sources),
read through `weakness.mjs`, plus the repository sources the corpus cites. Nothing here
changes a claim's status or edges; every proposed change is listed as an action. The
generated companion is [`WEAKNESSES.md`](WEAKNESSES.md) (all rows) with
[`weakness.json`](weakness.json) and [`weakness-queue.svg`](weakness-queue.svg).

**Method.** `attack-graph.mjs` already ranks unestablished claims by impact (how much
rests on them) and propagates taint from refuted claims. `weakness.mjs` adds what that
pass does not see:

| check | what it finds | count |
|---|---|---|
| status inflation | proved / algorithm / measured claims whose support closure reaches an open, heuristic or assumed claim | 17 |
| unwired dependence | a claim's own assumption text invokes a heuristic node with no edge to it | 11 |
| live obstruction | an established claim `obstructs` / `qualifies` / `does_not_cover` a claim that still supports others | 49 (17 `obstructs`) |
| hidden shared assumption | the same assumption text on 3+ nodes, never a node itself | 17 clusters |
| source contagion | a primary source with a refuted claim and other live claims | 19 sources |
| model-scope leap | generic-model or measured claim supporting a broader one without `model_bridge` | 9 |

It then ranks the 219 attackable claims by impact × breakability, where breakability is
the expected cost of a decisive result: a heuristic with a stated formula in an
algebraic model can be checked exactly on small fields; an open literature conjecture
cannot. Established `obstructs` edges multiply the score. Reading a primary text or
auditing a kernel-checked statement goes in a separate hygiene queue (91 claims): cheap,
but it rarely overturns anything. **The scores are a triage heuristic, not evidence.**

![Attack queue, top 20 (weakness-queue.svg)](weakness-queue.svg)

Evidence labels below: **exact** = a count or graph fact over the corpus files;
**read** = taken from a cited repository source, quoted path given; **derived** = an
argument written here, not yet reviewed or kernel-checked; **scratch** = a toy check run
in this session's scratch space, not committed and not evidence for the corpus.

## W1. The first-fall-degree pillar is obstructed and still load-bearing

**Exact.** `HEUR-FIRST-FALL-DEGREE` has four incoming `obstructs` edges from established
claims (CG21 incomparability, Kosters–Yeo's first fall degree 2 on descended S₃ at every
n, and two Caminata–Gorla summation measurements), and it still conditions seven claims:
`COND-ECC2K130-FFDA-PROJECTION` (the 2^86 projection), `COND-NAGAO-2015-COMPLEXITY`,
`COND-PKM-PRIME-ALGEBRAIC`, `COND-BINARY-WEIL-DESCENT`,
`COND-GB-HFE-FIRST-FALL-ESTIMATES`, `COND-SP-HPST-SYMMETRIZED-PRIME-DEGREE`,
`COND-SP-KARABINA-PDP`. Its siblings are in the same state:
`HEUR-GB-SEMIREG-ASSUMPTION` has six `obstructs` edges, `HEUR-GB-PQ-ASSUMPTION-2` four.
These three take the top of the attack queue. `HEUR-GB-PQ-ASSUMPTION-2` also states the
first-fall assumption in its own text with no edge to `HEUR-FIRST-FALL-DEGREE`.

**Judgment.** As a general principle the first-fall-degree assumption is already dead in
the corpus's own terms. What survives is a family-specific version for each estimate
that uses it. The graph keeps it at `heuristic`, so the seven conditional estimates read
as one discharged condition away from usable, which overstates them.

**Attack.** Petit–Quisquater Assumption 2 is asymptotic (`D_reg = D_ff + o(1)`), so no
finite table refutes it; the repository's measurements (D_ff = 2, degree 4→5 at n = 36→37,
5 at n = 45) only obstruct it. A **lower-bound proof** settles it: show that the solving
degree of the descended S₃ system at m = 2 is ≥ 3 for all n ≥ n₀. Degrees are integers,
so `+ o(1)` means eventual equality, and a uniform lower bound of 3 with D_ff = 2 refutes
the assumption for that family. Candidate route: exhibit, for every n, a degree-2
Macaulay span that provably misses a polynomial the reduced Gröbner basis needs. A
census of the degree-2 Macaulay rank against n over F₂ would show whether the gap is
uniform before the proof is attempted.

**Effect on the baseline.** None directly. This retracts projected IC costs that were
never the measured baseline. It matters because it removes a whole family of
"conditional subexponential" rows from the menu of routes worth funding.

## W2. The relation-rank heuristic is load-bearing only by a modelling choice

**Exact.** `HEUR-RELATION-RANK` conditions `ALG-SPARSE-LINEAR-ALGEBRA`, and through it the
corpus marks `THM-WD-GTTD-THM1`, `THM-WD-DIEM-SMALL-GENUS` and `ATTACK-GHS-WEIL-DESCENT`
(all `proved_external`) as resting on a heuristic: 4 of the 17 status-inflation rows.
Five more claims invoke rank in their assumption text with no edge (W5).

**Read.** `HEUR-BK-GG-RIGHT-KERNEL-RANK` (Galbraith–Gaudry 2016, §6.2) already records
that left-kernel (target-dependent) relations "are much easier to analyze rigorously",
while right-kernel relations need rank #F − 1. No node carries the left-kernel argument
and no edge discharges the rank heuristic with it.

**Derived (needs review and a Lean statement).** Let r be prime and Q = xP. Draw (aᵢ, bᵢ)
uniformly, set Rᵢ = aᵢP + bᵢQ, and keep the Rᵢ that decompose over the factor base F, giving
rows Mᵢ ∈ (Z/r)^F. Take B = |F| + 1 rows. Some nonzero λ satisfies λM = 0, whatever the
rank of M, and then Σλᵢaᵢ + x·Σλᵢbᵢ ≡ 0 (mod r). Fix the points Rᵢ: the decomposition, and
so M and λ, depend only on the Rᵢ. Given Rᵢ, bᵢ is still uniform on Z/r (the r pairs with
aᵢ + bᵢx = log Rᵢ are equally likely). So Pr[Σλᵢbᵢ ≡ 0] = 1/r and x is recovered with
probability 1 − 1/r. Signed/automorphism-folded columns change nothing. **No rank
condition is used.** That is why GTTD and Diem can be rigorous, and the four inflation
rows are edge errors, not fragile theorems.

**Scratch.** Toy check (p = 10007, prime-order curves, factor base x < 60, two-term
decompositions, |F| + 1 relations, 12 seeds): every matrix was rank-deficient (2–7 short
of |F|), so the right-kernel formulation fails on every one, and the left-kernel recovery
returned the correct log in 12/12. Illustration only.

**Where the heuristic is truly load-bearing.** Only where relations are reused across
targets (right-kernel amortisation): multi-target batches such as the
`ic.residual_batch` vs `rho.multitarget_pooled` parity work, and PKM's Δ = 10 surplus
(`HEUR-PF-PKM-RANK-DELTA`). There the measurable question is the rank deficit as a
function of |F| and the column-folding convention, which a native census can answer
exactly.

**Actions.** (a) Add a `THM-LEFT-KERNEL-RELATIONS` node (derived above; formalise the
finite core in Lean: uniformity of bᵢ given Rᵢ and the 1/r bound). (b) Re-wire the
rigorous Weil-descent chain to it, which removes 4 inflation rows. (c) Narrow
`HEUR-RELATION-RANK` to the multi-target / right-kernel regime and add the missing edges
from W5. (d) Measure rank deficit on the multi-target factor bases the repository
already uses.

## W3. The decomposition-yield heuristic conflates two regimes

**Exact.** `HEUR-DECOMPOSITION-PROBABILITY` has the largest impact on the frontier (13),
and four more claims invoke it in text without an edge (W5). Its statement already says
the mean B^m/r "follows by counting": only the distribution shape (Poisson, variance,
independence, rationality, solver visibility) is heuristic.

**Judgment.** Two different claims share this node. Over F_{q^n} with n fixed, Diem's
rigorous results (`THM-WD-DIEM-SMALL-GENUS`; its scope records the rigorous
q^(2−2/n) ECDLP bound cited by KMPS) bypass the heuristic. Over prime fields it is
genuinely heuristic, and that is where this repository's programme lives. Under the
left-kernel formulation (W2), the only quantity an index-calculus cost needs is
Pr[a uniform point decomposes] = |{R : R decomposes}| / r, which is a **set size, not a
heuristic**, and exact on small fields.

**Attack.** Split the node into an extension-field version (cite Diem; status follows the
read) and a prime-field version. For the prime-field one, run an exact census over every
target on a scaling family of small p. Record the full histogram of representation counts,
not just the mean, and report the observed decomposable fraction against 1 − exp(−μ) with
its trend in p. A systematic deviation is a correction to every prime-field yield estimate;
agreement is a measured bound. Either outcome is decisive at the measured scale.

## W4. The Lemma 4 refutation rests on a reading choice, and two internal papers disagree

**Exact.** Eleven nodes in the Nagao Lemma 4 family carry the free-text assumption
"Equality reading of Definitions 5 and 6", the largest hidden-assumption cluster.
`CLAIM-NAGAO-LEMMA4` is `refuted`, and its scope says the refutation is reading-specific.

**Read.** `papers/ecdlp-boundaries-20261018/sections/fall-degree.tex` tabulates all four
(reading × placement) cells. The equality reading is refuted both outside and inside;
literal `≥` outside is refuted (d_F = 3, d'_F = 2); literal `≥` inside gives d_F = d'_F = 2.
It also states that the wording "the literal reading always voids the counterexample is too
broad". But `papers/ic-negative-results/ic_negative_results.tex` (≈ line 470), the
repository source for the refutation (`S-IC-NEGATIVE`), still says "under that literal
reading the lemma holds only vacuously". The boundaries section also notes that a final
publication "still owes a fresh PDF-level transcription of both definitions".

**Actions.** (a) Correct the ic-negative wording to the boundaries table. (b) Record the
reading as a parameter of `CLAIM-NAGAO-LEMMA4` (refuted in 3 of 4 cells) instead of a
footnote. (c) Run the existing exact census on the one open cell (literal `≥`, field
equations inside). (d) Do the PDF transcription, because the refutation is only as good
as the reading.

## W5. Eleven unwired dependences understate the top frontier nodes

**Exact.** These claims name a heuristic in their own assumptions but have no support path
to its node, so refuting the heuristic would not reach them:

- decomposition yield → `ALG-GAUDRY-FIXED-EXTENSION`, `COND-BINARY-WEIL-DESCENT`,
  `COND-GB-PQ-SUBEXP-BINARY-ECDLP`, `COND-SP-KARABINA-PDP`
- relation rank → `COND-PF-PKM-COST-THRESHOLD`, `ALG-GAUDRY-FIXED-EXTENSION`,
  `COND-BINARY-WEIL-DESCENT`, `COND-GB-PQ-SUBEXP-BINARY-ECDLP`,
  `COND-PKM-PRIME-ALGEBRAIC`, `HEUR-XT-NFS-SMOOTHNESS` (NFS rank is the same
  phenomenon in a finite-field setting; wire it or record why not)
- first fall degree → `HEUR-GB-PQ-ASSUMPTION-2`

**Action.** Add the `conditions` edges, then re-run `attack-graph.mjs`. `weakness.mjs`
now reports any new occurrence.

## W6. Smaller structural findings

- **Conditional obstruction labelled proved (exact).** `OBSTRUCTION-XEDNI-HEIGHT` is
  `proved_external`, but it rests on Lang's height conjecture (`CONJ-XT-LANG-HEIGHT`,
  open) and six more hypotheses its own scope lists. The xedni line is closed only
  conditionally. Relabel it `conditional`. As an attack route it is still poor (it needs a
  uniform failure of Lang's conjecture on the selected lifts).
- **In-model theorems resting on an assumption (exact).** Five rho theorems (KMPT, KMT,
  MV mixing and nondegeneracy, three-partition) are `proved_external` given
  `ASM-GG-RANDOM-PARTITION`. That is correct inside the model, but implemented r-adding
  walks are not random partitions. This affects the **rho reference**, not IC: the
  repository already matches rho empirically, which is the right control.
- **Source contagion (exact).** Among primary sources, Semaev 2015 has 2 of its claims
  refuted while `HEUR-SP-SEMAEV15-ASSUMPTION1` (d_F4 ≤ 4) is still live and already
  obstructed once (Kosters–Yeo: degree 5 at n = 45). `RT-SP-SEMAEV-A1-N45` is the
  decisive test and is cheap. Kosters–Yeo, Euler–Petit/HKPYY and BGJT each have one
  refuted claim and live siblings (listed in `WEAKNESSES.md` §4).
- **Model-scope leaps (exact).** All nine are generic-model rho heuristics feeding
  standards or the concrete rho algorithm. That is the honest, well-known generic-to-concrete
  step, and the repository's matched-rho measurements control it.

## Recommended order of attack

| # | target | kind of result | cost | what changes if it lands |
|---|---|---|---|---|
| 1 | W2: left-kernel lemma + Lean core; re-wire; narrow rank heuristic | proof | low | 4 inflation rows resolved; rank heuristic confined to the multi-target regime |
| 2 | W5: add the 11 missing edges | corpus fix | trivial | true impact of the top frontier nodes becomes visible |
| 3 | W4: fix the ic-negative wording; census the open cell | correction + exact census | low | the Lemma 4 refutation becomes reading-complete |
| 4 | W3: prime-field yield census, full histogram, scaling in p | exact measurement | low–medium | every prime-field yield estimate is checked or corrected |
| 5 | W1: degree-2 Macaulay rank census, then the lower-bound proof | census, then proof | medium | PQ Assumption 2 refuted for m = 2; the FFDA rows retire |
| 6 | `RT-SP-SEMAEV-A1-N45` | exact F4 run | low | Semaev 2015 Assumption 1 settled at the measured scale |

Items 1, 2 and 3 are corpus and proof work, not experiment runs. Items 4–6 are runs and go
through the repository's experiment route (protocol, resume checkpoint, native code,
validation gate) before any number enters the corpus.

## Reproduce

```sh
node docs/ecdlp-hardness-corpus/weakness.mjs          # writes WEAKNESSES.md, weakness.json, weakness-queue.svg
node docs/ecdlp-hardness-corpus/weakness.mjs --check  # fails if they are stale
```
