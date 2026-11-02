# Brief for the 2026-10-31 ECDLP index-calculus mega pass

You are one agent in a research swarm. The goal of the swarm: find **genuinely
new routes** to the elliptic-curve discrete logarithm (ECDLP) through index
calculus or anything adjacent (algebraic geometry, number theory, commutative
algebra, additive combinatorics, coding theory, lattices, representation
theory, ML, "random ideas"), **positioned against everything this programme
has already closed**, with a **falsifier experiment** at toy scale for each.

Read this file first. Then read only what your task needs (paths below).

## 0. Rules that bind every agent

1. **No security claims.** Nothing you write asserts a speed-up on, or the
   insecurity of, any deployed curve. No key recovery. Toy evidence is toy.
2. **Classify honestly** (crypto `AGENTS.md` §3): *advance* (exponent or
   relation shape changes), *engineering* (constant factor), *relabelling*,
   *accounting*. A stage ratio is not an end-to-end gain.
3. **Name the obstruction.** When an idea dies, say exactly which theorem,
   measurement or bound kills it, and the scope of that kill (where it does
   *not* apply).
4. **Novelty is a claim about the literature.** Mark every "new" as
   `novelty_unverified` unless you actually searched and found nothing, and
   say what you searched.
5. **The non-genericity certificate.** Any index calculus with factor base of
   `B` points and per-target online cost `D` is non-generic exactly when
   `B·D² ≪ N` (Corrigan-Gibbs–Kogan preprocessing bound; `N = r/|Aut|`).
   Every proposal must say how it could ever get below that line, or admit it
   is a constant-factor device.
6. **Write only under** `research/ic_mega_pass_20261031/` (Conductor task
   T-5736 holds that directory; do not run `conductor check` yourself, it is
   done). Never edit anything else in any repository.
7. **Experiments**: python3 (numpy, sympy, cypari2), `gp` (PARI), `msolve`
   (Gröbner over F_p and F_2), `cryptominisat5`, `kissat`, `cargo` are on
   PATH. Sage is at
   `/Applications/SageMath-10-9.app/Contents/Frameworks/Sage.framework/Versions/10.9/venv/bin/sage`
   but HANGS in this sandbox (5-minute timeout with no output): do not use it.
   Use PARI (`gp -q`, or `cypari2` from python3), sympy, numpy and `msolve`
   (`msolve -f in.ms -o out`; input: variables line, characteristic line,
   comma-separated polynomials). Use
   `$TMPDIR` for scratch. Keep any single run under ~20 CPU-minutes.
8. **Return raw data.** Your final text is consumed by a program, not a human.
   When told to write a file, write it and return its path plus a short
   summary.

## 1. The programme in one page

Four repositories (all local, read-only for you):

| repo | path | what it holds |
|---|---|---|
| crypto | `/Users/adamburan/ic-work/crypto` (sparse checkout, this tree) and full tree at `/Volumes/SSD990-2/crypto` | Rust IC engines, ladders, ledgers, `docs/ic/*`, `papers/ecdlp-index-calculus-treatise/` (87-page treatise, 2026-10-29) |
| crypto-autoresearcher | `/Volumes/SSD990-2/crypto-autoresearcher` | knowledge base (`knowledge/{literature,findings,open-problems,frontiers,leads}`), `ideas/` (hypothesis cards), `research/THM_*.md` (theory notes with verification scripts), ledger of 1,400+ decisions |
| ml-cryptanalysis | `/Volumes/SSD990-2/ml-cryptanalysis` | "Curve Structural Discovery": learned/mined laws from curve data; `csd/semaev.py`, `csd/fpn_degrees.py` (Semaev S3/S4, Weil descent, fall degrees over F_{p^n}), hardness predictor M7 |
| cryptanalysis | `/Users/adamburan/cryptanalysis` | C11 reference library: rho, BSGS, kangaroo, BL precomputation, Z_p^* index calculus, GPU rho (64-bit) |

Pre-extracted reading in this directory (`brief/`):

- `treatise/*.tex` — the treatise chapters (plain LaTeX). Most important:
  `obstructions.tex`, `cover.tex`, `falldegree.tex`, `prog_openproblems.tex`,
  `conclusion.tex`, `prog_prime.tex`, `prog_binary.tex`, `prog_koblitz.tex`,
  `extension.tex`, `prog_transfer.tex`, `distance.tex`, `summer.tex`.
- `RESEARCH_IC_LEADS_AND_CONNECTIONS_20261010.md` — the "step back" analysis:
  why everything built so far is either generic or has a measured solving
  degree; the three questions Q1–Q3; symmetry inventory; what to stop.
- `LEAD_ATLAS_DISPOSITIONS.tsv` — 154 leads with disposition
  (`closed_obstructed`, `closed_known`, `closed_dominated`,
  `exhausted_at_scale`, `open_blocked`, `open_next_step`, `open_unexamined`),
  title and mechanism. Full YAML cards in
  `/Volumes/SSD990-2/crypto-autoresearcher/knowledge/leads/ecdlp/<family>/`.
- `FRONTIER_KNOWN_RESULTS_IC.txt` — titles of the known-results map rows
  (YAML in `crypto-autoresearcher/knowledge/frontiers/ecdlp/index-calculus/`),
  each with a `forecloses:` phrase list.

Other key programme documents (crypto tree): `docs/ic/CERTICOM131_IC_PROGRAM.md`
(cost model and gates at n=131 / p≈2^131), `docs/ic/RESEARCH_IC_NOVEL_DIRECTIONS_20261007.md`
(G1–G4, A1–A2, N1 leads), `docs/ic/LEAD_LADDER_METHODOLOGY_20261010.md`,
`docs/ic/ECDLP_RESEARCH_TECHNIQUES.md` (what is implemented),
`docs/ic/FRAMEWORK.md`, `RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md`,
`RESEARCH_KOBLITZ_INDEX_CALCULUS.md`, `research/notes/index-calculus/RESEARCH_*.md`
(full tree only).

## 2. What is closed, and the exact scope of each closure

Treat these as walls. A new idea must say which wall it goes around and why
the wall's hypotheses fail for it.

| closure | statement | scope / where it does NOT apply |
|---|---|---|
| **Generic oracle bound** | Any decomposition oracle that decides "is R an m-sum of base points" by group operations + membership tests (tables, MITM, enumeration, root tables) is a generic-group algorithm with advice; `S·T² ≳ r/|Aut|`. All compact-orbit k-sum pipelines sit 2^25–2^27 above that line. | Does not bound oracles that use the **field representation** of coordinates non-generically (Weil descent, Coppersmith on integer lifts of x, bit-level structure). That is the only door. |
| **GGM-simulable augmented oracles** | Jet / p-adic lift, endomorphism image, halving, incidence, elliptic-net evaluations are simulable in the generic group model (KN-FIND-002, -b7e091, -194294). | Applies to oracles computable from group elements alone. Not to representation-dependent maps. |
| **Summation-cover monodromy** | Galois group of Semaev's cover is `(Z/2)^{m-2}` for every curve; fibres split completely on the factor-base locus; mean yield at matched base size is a double count. | Closes: curve selection by monodromy, fibre censuses, "better geometry raises mean yield". Leaves free: coverage (which targets decompose at all), oracle cost, solving difficulty, *variance* of yield. |
| **Transport conservation** | A homomorphic transfer `φ: E→E'` with kernel κ changes the number of ordered m-decompositions by exactly `|κ|^{m-1}`. Isogenies buy only kernel symmetry (trivial at prime order), destination geometry, destination arithmetic cost. Chains compose; loops are scalars. | Non-homomorphic transfers (covers, correspondences, Miller cocycles) are not covered by this lemma; they are covered by the GHS/cover literature, which needs an intermediate field or a weak class. |
| **No subfield, no low-degree membership** | Prime `p`: no subset of F_p has membership degree below its size (RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM §2.3). Prime extension degree `n`: only F_2 is a proper subfield; no Frobenius-stable subspace when `ord_n(2)=n−1` (n=131); quasi-subfield polynomials closed. | Non-invariant `F_2`-subspaces still exist at prime n (Weil descent works; the cost is the solving degree). Over F_p the only algebraic handle is "x as an integer mod p" (lattices). |
| **Lattice reach** | Jochemsz–May small roots of `S_{m+1}` reach `δ_max = 1/(m(m+1)2^{m−2})`; needed `δ ≥ 0.169` at m=4 against reach 0.0125 (13.5× short), 6× at m=3. m=2,3 cannot beat √p even with a free oracle. | A new small-root method, or a different lattice-describable base shape, is the only lever. The D₄-symmetrised S₅ lattice at m=4 has never been run. |
| **Heights** | `ĥ(P+Q)+ĥ(P−Q)=2ĥ(P)+2ĥ(Q)`: sums of small points are not small; xedni lift probability exponent α=3 on a frozen family (not a general theorem). | Family-specific; a dimension count over all section shapes is missing. Height-compressing lifts (CM canonical lifts, function fields) are `open_next_step` in the atlas. |
| **Twist** | Twist points carry no relations (Frobenius acts as −1 on the twist part, so 2B=0); 0 of 3.6·10^8 mixed sums rational. | Twist-*avoiding* subspaces at prime degree are unresolved (toy evidence only). |
| **First fall degree is a constant of the presentation** | Binary S_3 descent: d_ff=2 always; last fall degree `d_F ≤ k+2`, certified `≥5` at n=33–45; Semaev's Assumption 1 fails as stated. PKM towers: d_ff=4 constant; d_F depends on presentation, not ideal. | **Whether d_F grows without bound is open** — the single most leveraged open question for binary curves. Measured D at m=3: fits `⌈ℓ/2⌉+4`; at m=4 fits `ℓ+4`; cells (m,ℓ)=(4,4),(4,5),(4,6) unmeasured. |
| **Frontier measurement** | 4,848 priced executions: no IC below rho on any prime or prime-degree binary curve. Koblitz 4.7–18× rho at 37–47 bits; prime 5.3–102× at 20–40 bits; random binary 10.6–38×. Online wins all retired by equal-advice Bernstein–Lange audits. | Weak extension-field families cross rho (genus-3 GHS, plane quartics at exponent ≈1, Joux–Vitse covers, k=4 char 2 near m*≈17.6). These are known algorithms, fairly measured. |
| **Wagner / k-list** | Relation collection is a k-list sum problem but `E(F_q)` has no small visible quotient, so Wagner's `2^{n/3}` does not apply; sort-and-match at `n⁴K⁴/r` is the best generic rate. Residual/large-prime walks lose above `r^{1/4}` bases. | A *non-group* filter correlated with sums (an "x-functional filter" whose single-bucket gain grows) is `KN-OPEN-91d4f6`, open. |
| **Symmetry inventory** | Negation, Frobenius (÷n relations, ÷n² LA, Faugère–Svartz blocks 31–128× on rank), CM automorphisms, 2-torsion translation (FGHR, 30–416× on solve), S_m, D₄ on S₅ (computed on paper, never run), isogenies (nothing at h(O_K)=1). All polynomial in n or constant. | Symmetry cannot change an exponent; it can make the decisive solving-degree cells affordable. |

Also closed or exhausted in the atlas (do not re-propose without a new
mechanism): factor-base subspace shape at m=2; free relations by
self-collision; orbit linear algebra; coordinate-statistic bias hunts
(exhausted at scale); DL character-sum pseudorandomness; naive Semaev over
F_p; char-2 summation algorithms vs rho; xedni; MOV/anomalous transfers on
deployed curves; isogeny walks to weak neighbours (Tate's theorem: group
order is an isogeny invariant); GLV/Frobenius-invariant bases (GGMP 2020,
known).

## 3. What is explicitly open (the levers that are free)

From the treatise's open-problems chapter and KN-OPEN entries:

1. **Growth of the last fall degree / solving degree** of Weil-descended
   `S_{m+1}` restricted to ℓ-dim subspaces, grid over (n, ℓ, m≥3). A proved
   lower bound growing with n would kill the subexponential heuristics; a
   bound f(m) independent of n would revive them. Proof route suggested:
   degree-c pseudo-solutions (linear functionals on `B_{≤c}` killing
   `W_c(F)` and not 1), generalising the closure certificates. (KN-OPEN-8f3d27,
   -3c8f51, -d218ec, -7f0511, -02200b.)
2. **Root finding beyond two summands on prime curves**: is there a sub-`N²`
   structure for `S_4` membership over a prime-field base (e.g. from
   `D_m`-invariant coordinates of 2-torsion curves)? Would it change the
   `r^{2/3}` cost of two-summand prime IC?
3. **A small-root method for S₅ at δ ≈ 1/5** (m=4), against a measured reach
   of 1/80. D₄-symmetrised S₅ lattice: never run.
4. **PKM-tower plateau**: measured D = 6,6,7,7 at N=9–18 (m=3), contradicts
   the naive XL bound `≥ d+δ`; unexplained. Re-measure t=1..14 with closure
   certificates; prediction `d_F = 4 + ⌊(t−2)/4⌋`.
5. **Frobenius-equivariant F4** on the full barrel system (character blocks
   over `F_{2^130}`): projected 10–300× dense; enabler for (1).
6. **Non-subspace Frobenius-stable bases at prime n** (trace-spectrum,
   Hamming ball — has a bounded-degree Hamming-ideal membership
   (KN-OPEN-5e3dd0), multiplicative subgroup μ_263, τ-adic, Couveignes–Lercier
   algebraic-group points). Cyclotomic-sparse base A2 never measured.
7. **Twist-avoiding subspaces at prime degree**; all-lift subspaces exist at
   small n (0.92× null, p=2·10⁻⁵).
8. **x-functional filters** whose gain grows (KN-OPEN-91d4f6); **invertible
   hint subsets** (dense, recognizable, invertible subsets of E(F_p)).
9. **Digit (base-B) presentation of prime-field Semaev systems** as a
   Weil-descent analogue: first/last fall degree, staircase (LEAD-ICP-digit-…).
10. **Miller cocycles / non-homomorphic transfers into F_p^*** when the
    pairing is trivial (unexamined); **Prym / same-field genus-g embeddings**
    (E as isogeny factor of a Jacobian over the same field; five filed
    follow-ups 2026-10-27, unexecuted); **second N-torsion direction** via
    Kani/Richelot gluing.
11. **Height-compressing global lifts** (CM canonical lifts; function-field
    sections) — open_next_step.
12. **ML**: learned decomposability predictors, learned cost models for base
    selection, hardness predictor (M7: planted law recovered 0.995 of oracle;
    no real skill on plain rho control, R² 0.0003).
13. **Hashed-factor-base ECDL with tensor contraction** (LEAD-EXO-hashed-…,
    conditional on an ω ≤ 9/4 question over GF(2^k); `crates/hashed-ecdl-tensor`).
14. **Drinfeld-module / function-field analogies**: the DLP on Drinfeld
    modules *is* broken by index calculus; what structural difference makes
    E(F_q) resist? (not in the atlas — check literature.)

## 4. The target quantities and sizes

- ECC2K-130: `K_0: y²+xy=x³+1` over `F_{2^131}`, `r ≈ 2^129`, `τ²+τ+2=0`,
  `End=Z[τ]`, class number 1, ONB type II, `μ_263 ⊂ F_{2^131}^×`,
  `ord_131(2)=130` (no Frobenius-stable subspace). Rho ref `√(πr/(4·131))`.
- ECC2-131 (random binary, prime n), ECCp-131 (`p<2^131` prime, h=1).
  Rho with negation `2^64.9` GAE.
- Toy ladders where the programme measures: binary n=13–45, Koblitz n=41–83,
  prime 20–63 bits, `E(F_{p^k})` k=3..6.
- Unit: group-addition equivalents (GAE), `S = ops/√r`, price band
  0 / 0.1 / 1 GAE per native op.

## 5. Output conventions for this pass

All outputs live under `research/ic_mega_pass_20261031/`:

- `literature/<lane>.json` — literature sweep results (schema in prompt).
- `ideas/<lens>.json` — generated route hypotheses (schema in prompt).
- `verdicts/<idea_id>.json` — skeptic verdicts.
- `experiments/<idea_id>/` — code, `RESULTS.json`, `README.md` per experiment.
- `report/REPORT.md` — synthesis.

Dates: today is 2026-10-31. "Past few weeks" means 2026-09-15 to 2026-10-31.
