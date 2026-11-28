# Red-team of MEMO.md §1a "Lemma (redistribution is non-exploitable beyond MITM)"

Reviewed: `/tmp/.../scratchpad/cost-per-relation/MEMO.md` L44-54 (the Lemma), desk analysis, nothing run.

## VERDICT: STANDS-WITH-CAVEAT

The Lemma's *operational* conclusion — **no cheap, o(|mD|)-memory, known-(a,b)
sampler changes the IC exponent by biasing residuals into mD** — survives every
counterexample I could build. But the Lemma as *written* has a literal
counterexample and a category error, and its stated "proof" is a tautology. It
needs three scope fixes before it is sound. It is not FALSE (no candidate meets
all three bars: super-constant bias, o(|mD|) memory, known (a,b)); it is not a
clean SURVIVES (the written form is contradicted by a concrete cheap sampler and
mislabels MITM).

---

## 1. The stated "proof" is a tautology, not an impossibility argument

The Lemma argues: a cheap, o(|mD|)-memory sampler emitting known (a,b) with
`Pr[[a]G+[b]Q ∈ mD] ≫ |mD|/r` "would itself be a decomposition oracle — it is
the relation." That is a *restatement of what the thing does*, not a proof it
cannot exist. "It would be valuable" is not "it cannot exist." A real
impossibility needs one of:

- **ECDLP hardness (conditional):** a sampler whose bias is *super-constant*
  (exponent-changing) produces relations fast enough that IC collects |F| of
  them below rho and solves the DLP, contradicting ECDLP hardness. This *is*
  valid — but it is conditional on hardness, and it only excludes
  **super-constant** bias. A modest constant bias neither breaks ECDLP nor is
  excluded by this argument, and (see §3) a cheap constant bias demonstrably
  exists. The memo never states the hardness assumption and never restricts to
  super-constant bias.
- **KN-FIND-007** (`docs/ic/factor-base-encyclopedia/README.md` L631-635) bounds
  the mean over **uniform** residuals. A biased sampler draws **non-uniform**
  residuals, so KN-FIND-007 does not directly apply; the memo leans on it
  implicitly but it is out of scope for the biased case.

Finding (unsupported): the load-bearing step is asserted, not proved. Cheapest
fix: replace the one-line tautology with the explicit conditional "assuming
ECDLP hardness, no sampler with super-constant bias exists; constant bias is
bounded by §3."

## 2. Category error: MITM is not a yield-redistribution exploit

The Lemma says "the largest such structure the pipeline can afford is the m=2
pair table, and MITM already does exactly this ... MITM is the maximal
exploitation of the sumset-redistribution degree of freedom." This conflates two
orthogonal levers, which the memo itself separates elsewhere (§1 yield vs §2
cost-per-attempt):

- **Yield / bias** = fraction of known-(a,b) attempts whose residual lands in
  mD. This is the Lemma's actual subject.
- **Per-attempt check cost** = how expensive it is to test `R ∈ mD` once R is
  fixed. This is MITM's actual domain.

For a residual R drawn from random known (a,b), MITM (store {P_i}, test
`R − P_i ∈ D`) makes the *existence check* cheap; it does **not** change the
existence probability, which stays `|2D|/r`. Run MITM the other way — pick two
base points and form `R = P_i + P_j` (guaranteed in 2D) — and the coefficients
(a,b) are **unknown** (base-point logs unknown). So MITM in "oracle" mode does
not beat the uniform known-(a,b) hit rate, and in "sampler" mode does not keep
(a,b) known. In neither mode is it a known-(a,b) yield booster. Calling it "the
maximal exploitation of redistribution" mislabels the lever.

(Aside that cuts the other way for the memo: the m=2 pair table stores ~|D| ≈
√|2D| points, i.e. it is already **o(|mD|)** memory. So "largest affordable
structure = the pair table, beyond which you need |mD|-scale memory" is loose:
the real statement is about *check-cost* scaling, not about a yield structure
living at |mD| memory.)

Finding (unsupported / mislabeled): the identification MITM ⇔ maximal
redistribution exploit is a category error. Cheapest fix: state that
yield-redistribution is exploitable only up to the constant coverage headroom
(≤1.582, KN-FIND-007) plus the constant |Aut| orbit fold (§3), both at O(1)–o(|mD|)
memory, and that MITM is an orthogonal *check-cost* technique, not a redistribution
exploit.

## 3. Literal counterexample: the Frobenius / negation orbit sampler

This is the strongest candidate and it **falsifies the Lemma's literal
probability statement** while confirming its operational spirit.

ECC2K-130 is a Koblitz curve; its natural IC factor base is τ-closed and
negation-closed (the automorphism-orbit base, encyclopedia §A3 L639-659). Let
`R₀ = [a₀]G+[b₀]Q = P_1+…+P_m ∈ mD` be one genuine hit with known (a₀,b₀).
Frobenius acts on the order-r subgroup as `[λ]`. Then:

```
τ^j(R₀) = τ^j(P_1)+…+τ^j(P_m) ∈ mD     (D is τ-closed)
        = [λ^j a₀]G + [λ^j b₀]Q           (coefficients KNOWN, λ known)
```

and τ is a free coordinate squaring. Negation gives `−R₀ = [−a₀]G+[−b₀]Q ∈ mD`.
So from one hit the orbit `{±λ^j R₀}` yields up to ~2·ord(τ) **more** residuals
that are (i) in mD, (ii) emitted with **O(1)** memory and cheap ops, (iii) with
**known** (a,b). All three bars of the counterexample are met for the Lemma's
*literal* metric `Pr[R∈mD] ≫ |mD|/r`.

Why it does not actually break the algorithm — and is the caveat the Lemma
needs: these relations are **linearly dependent**. The cyclotomic falsifier
established (`research/cyclotomic_falsifier_20261015/README.md`, §2 Method and
verdict L11-17) that the mean m-decomposition count is `|B|^m/r` for every base
and that **"every one-column base is a unit multiple of {±λ^j}, so all K=1 bases
have identical metrics"** — the `{±λ^j}` fold is a *unit-multiple* (yield-neutral)
structure (15/15 null, positive control at p=0.001). In matrix terms τ^j(R₀)'s
row is the column-permuted λ^j multiple of R₀'s row; on a folded (orbit-rep) base
it is linearly dependent, and on a full orbit base the ×|Aut| extra rows are paid
for by a ×|Aut| larger base — the "constant |Aut|" verdict (encyclopedia A3
L657-659; E12 table ÷2.95, S ÷1.18 at L650-651).

Resolution: the Lemma's metric must be **rate of rank-contributing (independent)
relations per unit cost**, not raw membership probability. Under the corrected
metric the orbit sampler gives only the constant |Aut| already catalogued, and
the Lemma holds. Under the written metric it is false. This is a genuine scope
restriction, not cosmetic: the orbit sampler is a real cheap O(1)-memory
known-(a,b) sampler that beats `|mD|/r`.

## 4. The other candidates — all fail at least one bar (support the Lemma)

| Candidate | (i) super-const bias? | (ii) o(|mD|) mem? | (iii) (a,b) known? | Breaks Lemma? |
|---|---|---|---|---|
| Small-norm Z[τ]/lattice (a,b) | No — (a,b)→R is pseudo-random; small (a,b) do not bias R into mD | Yes | Yes | No |
| Frobenius/negation orbit (§3) | No — constant |Aut|, dependent rows | Yes (O(1)) | Yes | **Literal metric only**; fixed by caveat |
| Near-miss perturbation | To stay in mD you swap base points (unknown logs); known moves [Δa]G+[Δb]Q re-hit at uniform rate | Yes | Loses (iii) when staying in mD | No |
| Kangaroo / DP on residual | Known jumps = pseudo-random walk, uniform hit rate; DP aids collision detection, not mD enrichment | Yes | Yes | No |
| Generalized-birthday / Wagner | Can find decompositions fast, but memory ~2^{n/(1+log k)} ≫ |mD| | **No** | Yes (R is the known target) | No — confirms memory↔exploitation tradeoff |
| Partial-join / slope buckets (`single_relation_reduction_20261115`) | Measured: first-merge yield decays too fast; interval joins return at ~ambient density (README L27-39) | varies | — | No |
| Selective promotion of cheap residuals (`integer_factoring_transfer_20261101` L33) | Promotion allowed but "matrix width, setup, rank and individual-log cost must be charged" — the ≥√r batch-log setup dominates r^{1/6} relations | — | — | No |
| MITM pair table itself | Check-cost tool, not a yield booster (§2) | Yes (~√|2D|) | — | No (and confirms the Lemma's spirit) |

Key structural reason every known-(a,b) in-mD navigation fails: **tension between
(iii) and staying in mD.** Known moves `[Δa]G+[Δb]Q` are pseudo-random jumps →
uniform re-hit. Moves that keep you in mD (swap/add base points) use base-point
logs you do not know; to learn even one base-point log costs a full/batch DLP
(≥√r), which the "selective promotion" lead already requires be charged. The
Frobenius/negation maps are the *only* cheap operations that are simultaneously
log-known (they act as scalars λ, −1) and mD-preserving — and they are exactly
the constant-|Aut| unit-multiple fold that is yield-neutral. That is why the
operational conclusion holds.

## 5. Hidden-assumption stress

- **"Semi-known / recoverable coefficients" crack:** closed. Recovering any
  base-point log to enable in-mD navigation costs ≥√r (batch rho amortizes but
  the floor ≥√r dominates the ~r^{1/6} relations needed); charged per
  `integer_factoring_transfer` L33.
- **"Amortize one expensive bias-setup across many targets" crack:** closed by
  the same √r floor; the setup is itself rho-order work.
- **"Is m=2 the largest affordable structure?" :** imprecise (see §2 aside); the
  pair table is √|2D|-memory, so the memory argument is really about check-cost
  scaling, and larger affordable structures (m=3 table+scan) exist but do not
  change yield.

## Findings, by severity

1. **(unsupported)** The stated proof is a tautology; needs explicit
   ECDLP-hardness conditioning and restriction to super-constant bias (§1).
   Cheapest test: write the conditional and check a modest constant bias is not
   excluded — it is not (§3).
2. **(unsupported / mislabeled)** MITM is mislabeled as the maximal
   yield-redistribution exploit; it is a check-cost technique (§2). Cheapest
   test: trace the hit probability `|2D|/r` through an MITM attempt — unchanged.
3. **(wrong, literal metric only)** The membership-probability statement is
   falsified by the O(1)-memory Frobenius/negation orbit sampler (§3); fix the
   metric to rank-contributing relations. Cheapest test: the cyclotomic
   falsifier's `{±λ^j}` unit-multiple identity already shows the enrichment is
   yield-neutral.
4. **(supported)** The operational core — no cheap o(|mD|)-memory known-(a,b)
   sampler changes the exponent — survives all candidates (§4), capped at the
   ≤1.582 coverage headroom and the constant |Aut| fold.

Net: **STANDS-WITH-CAVEAT.** Keep the conclusion (no sub-table exponent win;
redistribution of *useful* yield is bounded by two constants), but restate it as:
(a) metric = independent relations, not membership; (b) conditional on ECDLP
hardness for super-constant bias; (c) the O(1)-memory orbit fold is a real but
constant (|Aut|) exploit, and MITM is an orthogonal check-cost technique, not the
redistribution exploit.

Citations:
- Memo Lemma: MEMO.md L44-54.
- KN-FIND-007 / coverage headroom ≤1.582: `docs/ic/factor-base-encyclopedia/README.md` L631-635.
- Constant |Aut| fold, orbit pair table ÷2.95: same file L650-651, L657-659.
- Frobenius `{±λ^j}` unit-multiple yield-neutral + `|B|^m/r` identity:
  `research/cyclotomic_falsifier_20261015/README.md` verdict L11-17, §2 Method.
- Charge individual-log/setup for promoted residuals:
  `research/integer_factoring_transfer_20261101/README.md` L33.
- Partial-join / interval-join yield decay:
  `research/single_relation_reduction_20261115/README.md` L27-39.
