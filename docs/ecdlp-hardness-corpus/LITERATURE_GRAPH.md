# The ECDLP literature graph: lemmas, heuristics, proofs and where they break

**Date:** 2026-11-02. **Scope:** definitions, theorems, lemmas, heuristics, conjectures,
counterexamples and open problems behind ECDLP attacks. Eight literature clusters plus
the repository's own results, each claim linked to what it rests on.

| | before | after |
|---|---:|---:|
| claims | 101 | **525** |
| typed edges | 145 | **888** |
| sources | 50 | **229** (83 primary text read and SHA-256 hashed) |
| research targets | 13 | **51** |
| refuted claims | 0 | **15** |
| claims with a Lean record | 0 | **29** (15 kernel-checked with a committed receipt) |

`node validate.mjs` passes, `node attack-graph.mjs --check` reports 0 unlinked
citations, and three claims rest on a refuted claim.

## How it was built

Each cluster owned a fixed list of works, so no paper was encoded twice. Every claim
records its theorem number or section, its hypotheses, and its status:

- `proved_external` only when the primary text was downloaded, read and hashed.
- `proved_external_unread` when only metadata was available (paywalls, or the eprint
  bot check, which was not worked around).
- `heuristic`, `conditional`, `open` or `refuted`, as the literature supports.

Each fragment validated alone; the integrator then added 25 cross-cluster edges
(`cross-links.json`).

| cluster | works | claims | edges | primary-read | refuted | heuristic | open |
|---|---|---:|---:|---:|---:|---:|---:|
| summation polynomials (`lit-sumpoly`) | Semaev, Gaudry, Diem I/II, Joux–Vitse, FPPR, FGHR, HPST, Semaev 2015, Karabina | 47 | 86 | 9 | 1 | 9 | 3 |
| Gröbner / fall degrees (`lit-groebner`) | BFS, Bardet, HMS, Caminata–Gorla, CCG, GMP, Salizzoni, Bigdeli et al., Steiner, Kousidis–Wiemers, Huang | 48 | 101 | 13 | 1 | 2 | 2 |
| prime fields (`lit-primefield`) | PKM, Amadori–Pintore–Sala, Semaev 2015, Kosters–Yeo, quasi-subfield polynomials, YYTK | 52 | 90 | 5 | 1 | 10 | 6 |
| generic / rho (`lit-generic`) | Miller–Venkatesan, KMPT, Montenegro–Tetali, Bernstein–Lange, BLS negation, AGM, multi-DLP | 42 | 79 | 10 | 2 | 6 | 2 |
| Weil descent / covers (`lit-covers`) | Frey, GHS, MMT, Hess, KMPS, Enge–Gaudry, GTTD, Diem, Diem–Thomé, Smith, Joux–Vitse | 57 | 79 | 11 | 0 | 10 | 2 |
| lifting / transfers (`lit-transfer`) | BGJT, Cheng–Wan–Zhuang, GKZ, Kleinjung–Wesolowski, exTNFS, Barbulescu–Duquesne, Petsche, PPVW | 61 | 91 | 10 | 5 | 10 | 5 |
| books / surveys / standards (`lit-books`) | Galbraith–Gaudry 2016, SafeCurves, SEC 1, RFC 5639, NIST SP 800-186, Koblitz–Menezes, handbooks | 47 | 91 | 5 | 3 | 11 | 7 |
| this repository (`lit-repo`) | 21 Lean projects and ledgers, cryptanalysis `ic-assumption-graph` | 56 | 83 | (repo) | 1 | 4 | 4 |

The Galbraith 2018 book inventory (717 labelled results) and its promoted claims were
already in the corpus.

## What the graph shows

### 1. The degree-to-cost bridge is where binary and prime-field index-calculus estimates break

Every subexponential claim for binary-field ECDLP, and every prime-field projection,
turns a degree invariant into a cost. The graph now records the evidence that this step
fails:

- **First fall degree vs solving degree.** Caminata–Gorla 2021 (Ex. 4.1–4.4) put the
  first fall degree arbitrarily far from the solving degree, last fall degree, `d_reg`
  and regularity, even with field equations (`THM-GB-CG21-FIRST-FALL-INCOMPARABLE`).
  On summation systems specifically:
  - Kosters–Yeo prove first fall degree 2 for descended `S_3` at every `n`.
  - The degree reached grows: Kousidis–Wiemers Table 1 (4 to 5 between `n = 36`
    and `37`), and Kosters–Yeo at `n = 45` (degree 5).
  - PKM's own tables over prime fields climb 4, 5, 6 while the first fall degree stays 4;
    this repository reproduces 5 and 6.
  - Semaev 2015's own `t = 2` rows: first fall 2, solver degree 4. This is
    `CEX-SP-SEMAEV15-T2-TRACE`, an inference from two published results, not yet
    recomputed.
- **Semi-regularity.** Bardet–Faugère–Salvy conjectured that almost all `F_2` systems
  are semi-regular. Hodges–Molina–Schlather (Thm 7.14) prove the share tends to 0
  (`CONJ-GB-BFS-SEMIREG-PROPORTION-F2`, refuted). The binary Weil-descent route and the
  semi-regular assumption rest on it, so the taint rule now flags both.
- **`d_reg` under-predicts the solving degree** on Gaudry-style relation systems:
  5 vs 4 on 150,000 instances, and 15 vs 8 where `d_reg` is undefined.
- **Same name, different invariants.** "Last fall degree ≤ degree of regularity" holds
  only when `d_reg` means a solving degree; Salizzoni Ex. 3.9 gives `d_reg + 1`.
  Several repository files use "degree of regularity" in more than one sense
  (`RT-GB-DREG-READING-AUDIT`).
- **The sound routes are attached to `GAP-FALL-DEGREE-COST-BRIDGE`:**
  - Caminata–Gorla Thm 3.1: `sd = max{last fall, max GB degree}`.
  - Salizzoni: `sd ≤ d_reg + 1`.
  - CG: `sd ≤` the CM regularity in generic coordinates.
  - CCG: regularity of Weil restrictions grows exactly linearly in `n`.

Nagao's Lemma 4 (the starting point of this audit) is one instance of this pattern, not
the only one.

### 2. The rho baseline is itself partly heuristic

- Every rigorous rho theorem assumes a uniformly random partition
  (`ASM-GG-RANDOM-PARTITION`), which implementations do not use.
- No rigorous square-root bound is known for the r-adding walks used in practice
  (`GAP-GG-ADDING-WALK-RIGOROUS`).
- The best proved constant is about 52.5·√|G|, against the heuristic 1.2533.

So "matched rho" references rest on a heuristic constant plus measurement
(`GAP-GG-RHO-CONSTANT qualifies FACT-MEASURED-BOUND`). In addition, Bernstein–Lange show
the Brent–Pollard correction `1/√(1−1/r)` undercounts (r = 6: 1.129 vs 1.095), and
`ecc2k130/WALK-CONSTANT.md` uses the Brent–Pollard form.

### 3. Transfers: heuristic vs proved

- Every L(1/3) NFS, TNFS and exTNFS cost rests on a smoothness heuristic
  (`HEUR-XT-NFS-SMOOTHNESS`, 8 claims downstream). No target-field cost has a proved
  lower bound.
- In fixed characteristic, Kleinjung–Wesolowski prove quasi-polynomial DLP, so MOV on
  small-characteristic supersingular curves is provably quasi-polynomial.
- Several pairing-security claims are refuted by later estimates or breaks:
  BN-3072 at 128 bits, the genus-2 supersingular curve over F_2^367, and AMOR F_2^1223.

### 4. Standards disagree, and none derive their thresholds

| | embedding degree | CM discriminant / class number |
|---|---|---|
| SEC 1 v2 | B ≥ 100 | none |
| NIST SP 800-186 | k ≥ 2^10 | none ("no technical rationale") |
| Brainpool (RFC 5639) | (q−1)/k < 100 | class number > 10^7 |
| SafeCurves | ≥ (ℓ−1)/100 | \|D\| > 2^100 |

SafeCurves misquotes SEC 1 (it says 20) and Brainpool (it says 10^6) against the
current texts. Both are recorded as refuted against SEC 1 v2 and RFC 5639; older
versions may explain the difference.

## Repository findings (to fix in follow-ups, not in this PR)

| file | finding | verified here |
|---|---|---|
| `src/ecc_safety.rs:211-215` | Amortised rho is attributed to "Galbraith-Lin-Scott" (Kuhn–Struik is the usual attribution), and described as helping an attacker who wants *any one* of m keys. No speedup is known for the first of m keys; the √(n/m) is per key when breaking all m. The computed margin is valid under the correct reading. | yes (source read) |
| `src/cryptanalysis/ghs_descent.rs:45`, `problems/ghs-higher-magic-descent-not-executable.md:37` | Cite "Hess 2004, Theorem 5.3". Hess numbers theorems consecutively, so there is no 5.3; the injectivity result is Theorem 14. | yes (Hess text) |
| `src/cryptanalysis/jacobian_transfer/cost.rs:270-278, 376-425` | GTTD `q^(2-2/g)` applied at every genus (proved for g ≥ 3 with conditions); Enge–Gaudry `L(√2)` hypothesis "reported, not enforced"; gGHS covers are non-hyperelliptic, so their cost is open. | lines confirmed |
| `research/transfer_audit_20261011/PROTOCOL.md` | Calls `L[1/3, (32/9)^(1/3)]` an "attacker-favourable lower bound"; it is unproved, and below the proved quasi-polynomial bound in characteristic 2 and 3. | agent report |
| `research/factor_base_arity_review_20261024/lit_degree_reduction.md` vs `papers/ic-negative-results` S6 | One says Semaev's Assumption 1 is refuted at n = 45; the other says the off-diagonal failure is not established. | agent report |
| `src/cryptanalysis/preprocessing_rho.rs` | Table described as N^{2/3}; in Bernstein–Lange the table holds ~ℓ^{1/3} distinguished points covering ~ℓ^{2/3} points. Worth checking the cost model. | agent report |
| five Lean "kernel-checked" claims | No receipt in the tree (four JKSST counterexamples, Koblitz `CyclicWindow`); recorded as unchecked. | yes |

## Where to attack next

The full ranked frontier (318 claims) is in [`ATTACK_GRAPH.md`](ATTACK_GRAPH.md). The
claims carrying the most weight:

| claim | status | rests on it | settle by |
|---|---|---:|---|
| `HEUR-DECOMPOSITION-PROBABILITY` | heuristic | 13 | exact counts vs `1/(ng)!` on the families in use (`RT-BK-DECOMP-MEASURE`) |
| `HEUR-RELATION-RANK` | heuristic | 10 | rank deficiency measured at scale |
| `ALG-WD-GAUDRY-2000` | conditional | 9 | read the primary text (paywalled) |
| `HEUR-XT-NFS-SMOOTHNESS` | heuristic | 8 | not settleable here; keep transfer costs labelled heuristic |
| `COND-ECC2K130-FFDA-PROJECTION` | conditional | 8 | `RT-GB-SUMMATION-INVARIANTS`: measure all five invariants on S_3/S_4 descents as n grows |
| `HEUR-FIRST-FALL-DEGREE` | heuristic | 7 | as above; Caminata–Gorla route for any cost row |
| `HEUR-RHO-RANDOM-MAPPING` | heuristic | 7 | `RT-GG-WALK-NONRANDOMNESS` |

Cheap, decisive targets that are ready now:

- `RT-SP-T2-TRACE-CHECK`: confirm or kill the Semaev 2015 `t = 2` counterexample by
  exact computation.
- `RT-BK-ONE-OUT-OF-L`: correct the multi-target model in `ecc_safety.rs`.
- `RT-WD-HESS-KERNEL-DESCENT`: fix the Hess citation and use Theorem 14 as the
  injectivity certificate.
- `RT-PF-APS-VS-PKM-REMEASURE`: the APS "beats PKM" comparison sets its m = 3 runs
  against PKM's m = 2.
- `RT-LEMMA4-REVERSE`: the outside form of `d'_F ≤ d_F`.

## Limits

- 64 sources are metadata-only and 35 secondary-only, so their 30 theorems are
  `proved_external_unread`. Paywalled books (the Cohen–Frey handbook,
  Hankerson–Menezes–Vanstone, Washington, Blake–Seroussi–Smart) are located to the
  chapter only.
- eprint PDFs were not fetched (bot check). Unread as a result, among others: Diem's
  Math. Comp. paper and Diem 2013 beyond its sample pages, Gaudry 2000,
  Adleman–DeMarrais–Huang, Menezes–Qu, Petit–Quisquater, HKY 2015 and Semaev–Tenti.
- Agent-reported findings marked "agent report" above were not independently re-derived
  by the integrator.
- The first-fall-degree assumption stays `heuristic` rather than `refuted`: the general
  form is refuted (Caminata–Gorla), but the estimates use a family-specific form, for
  which the evidence is strong but not a proof.

## Files

- Fragments: `lit-sumpoly.json`, `lit-groebner.json`, `lit-primefield.json`,
  `lit-generic.json`, `lit-covers.json`, `lit-transfer.json`, `lit-books.json`,
  `lit-repo.json`, `cross-links.json`, plus `fall-degree-lemmas.json` (the Lemma 4 audit,
  [`LEMMA_LINKAGE.md`](LEMMA_LINKAGE.md)).
- Generated: [`ATTACK_GRAPH.md`](ATTACK_GRAPH.md), `attack-graph.{json,dot,svg,pdf}`,
  `graph.{dot,svg}`, `overview.{dot,svg,pdf}`, `claims.tsv`, `sources.tsv`, `stats.tsv`.
- Report PDF: `LITERATURE_GRAPH.pdf` (this file plus the attack-graph figure).
