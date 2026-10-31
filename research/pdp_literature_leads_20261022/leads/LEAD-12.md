# LEAD-12 — Theory gate: structured-GGM and non-adaptive time–space audit of every lead

**Verdict.** unverifiable-from-here, for the one statement the gate most wants: the elliptic-curve theorem of ePrint 2026/384 could not be read (§8). The 14-row classification is delivered anyway; it rests on theorems read at source (Dinur–Keller–Marmor, Gorla–Massierer) and on the repository's own derivations for the rest. Tally: 3 can move the exponent in principle (01, 06, 10 conditional), 9 constant only, 0 excluded outright, 2 not oracles (05, 12).
**Confidence.** medium for the generic-oracle rows; low for the line between "exponent in principle" and "constant only" on algebraic rows, because none of the three bounds constrains the cost of an oracle that reads coordinates.
**Expected gain if it works.** none: a gate, it moves no leaderboard row. It routes the first discriminating test to LEAD-05 (it decides 01, 03, 07, 14) and withdraws exponent expectations from nine cards.
**Cost of the discriminating test.** none; about 0.5 person-day and 0 CPU to close the reading gap if a copy of 2026/384 with its elliptic-curve section is found.

## 1. Mechanism (precise)

**Notation.** N = prime order of the target subgroup (CGHW's q, CGK's and DKM's N). F = signed factor base, |F| = N^b. m = summands. δ_m = fraction of labels that are m-sums of F, `δ_m ≈ |F|^m/(m!·N)` times the torsion-shift count c if relations may absorb cosets (W3 mapping). KN-FIND-007 proves the mean count `C(|F|+m−1, m)/N` exactly for every factor-base geometry, so δ_m is a function of (|F|, m, N) alone and cannot separate two leads of equal |F| and m. Regimes: binary subspace `2^{mℓ}/(m!N)`, dense at `ℓ = (n+log₂m!)/m`; Gaudry on E/F_{q^k}: `1/k!`; trace-zero T_n (m = n−1, N ≈ q^{n−1}): `1/(n−1)!`; prime small-x base, B = N^b: `2^m N^{mb−1}/m!`.

**The gate, three questions in order.**
- **Qa, generic coverage.** Is every answer computable from group-law queries and equality or membership tests on labels? If yes, Shoup (`T = Ω(√N)`) and CGK (`S·T² = Ω̃(εN)`) bind it; if its group queries are also fixed before it sees answers and `S < √N`, DKM binds it (`S·T ≳ N`).
- **Qb, structured.** If not, δ is the fraction of labels on which the structure bites and SGGM gives `T = Ω(min(√N, 1/δ))`. Beating rho needs `δ ≫ N^{-1/2}`, i.e. `b > 1/(2m)` (necessary, under the mapping). An index calculus with k = |F| relations costs `k/δ` (SGGM Thm 4.2 as an algorithm), i.e. `b > 1/(2(m−1))`, which is the repository's own §2.1 condition. Dense δ makes the bound vacuous.
- **Qc, oracle cost.** Outside all three models. SGGM charges group queries only and evaluates ⋆ for free, so for a dense-δ algebraic oracle the exponent is decided by the oracle's own cost slope against the repository's G3 (below the pair table's `⌊m/2⌋/m` bits per tower variable, polynomial memory). Measured: solving degree 7 at m = 3, 4 against admissible 2–3 at m = 4 (ECC2K-130, 256-bit prime).

**What the bounds literally say.**
- **Shoup 1997** (recalled; text via DKM and a course report): m queries give success `O(m²/p)`, p the largest prime divisor.
- **CGK 2018** (abstract-only): `S·T² = Ω̃(εN)`, tight up to logs.
- **DKM Thm 1.1** (read at source): non-adaptive `(S,T)`, success at most `3T²/N + 4 ln2·S·T/N`. Non-adaptive means the pre-translated pairs (a,b) in `σ(a·d+b)` depend on the advice, not on the challenge. Challenge-dependent pre-translated queries are the authors' own open problem; their r-round bound `Õ(T²/N + rST/N)` is a conjecture.
- **CGHW** (abstract-only here; formulas via the W3 README, which read the authors' PDF): Thm 3.2 `DLAdv ≤ δn(3T+2)/q + (3T+1)²/q + 1/n`; Thm B.1 `Õ(ST²/q + δT)`; Thm 4.2 index calculus `O(k log²k log n/γ)`.

**Codes.** E = can move the exponent in principle; C = constant only; X = already excluded by theorem; M = not an oracle. "T" = theorem; "I" = interpretation.

| Lead | Oracle type | Group ops + membership only? | δ exploited | Adaptive? Advice S | Verdict | Theorem vs interpretation |
|---|---|---|---|---|---|---|
| 01 MinRank / SM | algebraic elimination (reformulated chain) | no | `2^{mℓ}/(m!N)`, dense | outside DKM; S = shape-only prepare | **E**, open: needs a D-slope below the pair table; chain D = 4, S₄ D = 7 measured | T: none covers it. I: that the generic bilinear bound `min(n_x,n_y)+1` (recalled) transfers to the descended chain |
| 02 parametric eliminant | algebraic elimination, amortised | no | Gaudry `1/k!`, dense | S = eliminant coefficients, size independent of q | **C** | T: none. I: per-decomposition cost is q^{o(1)} at fixed k, so the q-exponent is set by relation count and linear algebra (Gaudry heuristic) |
| 03 ℓ-torsion / Kummer symmetry | algebraic elimination (quotient) | no | unchanged (KN-FIND-007) | S = 0 | **C**, E only if the D(n) slope moves (§8) | I: "symmetry cannot change the exponent" is a repository inference from n-independent group order |
| 04 class pre-partition | filter on table or enumeration oracles (hybrid) | the class map is not generic, but only prunes a membership-tested table | unchanged: the class sum is implied by landing in the coset (KN-FIND-14706d pattern) | non-adaptive scan; S = table + labels | **C**, at most \|A\|, plausibly 1× for hash lookup (accounting) | T: DKM headroom ≤ 2–4× (§4). I: the class is trivial on ⟨P⟩ when ℓ∤N, so it carries no information about d |
| 05 d_reg growth law | measurement | n/a | n/a | n/a | **M**; decides E vs C for 01, 03, 07, 14 | I: the 2025 preprint is a GitHub release, no venue |
| 06 trace-zero coordinates | algebraic elimination (representation) | no | `1/(n−1)!`, dense | S = 0 | **E**: published `Õ(q^{2−2/(n−1)})` (heuristic); ties rho at n = 3, wins by large primes at n = 5; instance class absent from the leaderboard | T: none; G–M Thm 4.2 is labelled heuristic by its authors. I: comparability with the Gaudry baseline |
| 07 Crossbred | algebraic solver, hybrid with enumeration | no | dense | S = 0 | **C**, medium-low (§4) | I: Crossbred exponents are recalled, ePrint 2024/992 not read |
| 08 learned orderings | solver heuristic (ML) | no | unchanged | S = trained model | **C** (null: WDSat is an enumerator) | I |
| 09 Ore elimination | algebraic elimination, enumeration scale `Õ(2^ℓ)` | no | m = 2 residual, `2^{2ℓ}/(2N)` | S = 0 | **C** (§8) | T: none binds a coordinate-reading oracle. I: "on the enumeration line by the generic bounds" is a cost count, not a theorem |
| 10 chord small-root | lattice (Coppersmith) oracle, prime | no | `2^m N^{mb−1}/m!` | S = B logs | **E**, conditional: S_{m+1} reach 1/80 at m = 4 is 10× under the necessary 1/8; new family needs b* ≥ 1/8 | T: SGGM Thm 3.2. I: EC mapping; the card's trigger 1/10 sits below the necessary 1/8 |
| 11 McGuire–Mueller | a priori enumeration / table (S_{m+1} evaluation as the test); unread | probably yes: evaluating S_{m+1} at known abscissas is simulable by group law + comparison | `\|F\|^m/(m!N)` | a priori non-adaptive | **C**, conditional | I: abstract-only; classification pending the full text |
| 12 this gate | gate | n/a | n/a | n/a | **M** | n/a |
| 13 summation ideals on covers | transfer + algebraic elimination | no | `1/3!`, dense (Jacobian) | S = 0 | **C**: the cover regime's q^{1.30} vs q^{1.5} comes from dimension and double large primes, already in the baseline | I: solution count falls by a q-independent factor |
| 14 digit/tower blocks | algebraic (degree bound) | no | `\|V\|^m/(m!N)`, dense | S = 0 | **C** (explanatory); E only with a t-independent degree bound | I: the multilinear bound `Σn_i−max n_i+1` is linear in t |

## 2. Prior art: literature

- Shoup, *Lower Bounds for Discrete Logarithms and Related Problems*, EUROCRYPT 1997, LNCS 1233, doi 10.1007/3-540-69053-0_18. **recalled**; statement read second-hand in DKM §1 and in Philipson's course report (web.stanford.edu/class/cs259c/finalpapers/genericgroups.pdf, read, secondary). Claim taken: success `O(m²/p)`.
- Corrigan-Gibbs, Kogan, *The Discrete-Logarithm Problem with Preprocessing*, EUROCRYPT 2018, ePrint 2017/1113. **abstract-only**. Claim: `S·T² = Ω̃(εN)`, tight up to logs.
- Corrigan-Gibbs, Henzinger, Wu, *The Structured Generic-Group Model*, EUROCRYPT 2026, ePrint 2026/384. **abstract-only** (ePrint page, HTTP 200). Claim: `Ω(min(√q, 1/δ))`; the abstract also says similar bounds are proved for "elliptic-curve points". Theorem formulas are taken from the W3 README, which read the authors' PDF on 2026-10-11 and found no EC section in that copy.
- Dinur, Keller, Marmor, *Non-Adaptive Cryptanalytic Time-Space Lower Bounds via a Shearer-like Inequality for Permutations*, arXiv 2505.00894v3 (STOC 2026). **read-at-source** (PDF, `pdftotext`). Claims: Thm 1.1; the non-adaptivity definition; the open problem on challenge-dependent queries.
- Gorla, Massierer, *Index Calculus in the Trace Zero Variety*, Adv. Math. Commun. 9(4), 2015, arXiv 1405.1059. **read-at-source** (intro, §4, §6). Claim: Thm 4.2, heuristic `Õ(q^{2−2/(n−1)})` in T_n against rho `q^{(n−1)/2}`; tie at n = 3; exponential dependence on n through the Gröbner step.
- McGuire, Mueller, ePrint 2017/1262. **abstract-only**: a Gröbner-free algorithm with fast S_{m+1} evaluation and one with neither; no complexity stated on the page.
- Gaudry, *Index calculus for abelian varieties of small dimension…*, J. Symb. Comput. 2009. **recalled**; the `Õ(q^{2−2/d})` exponent is confirmed in G–M's intro (read).
- Faugère–Safey El Din–Spaenlehauer (arXiv 1001.4004), Vidal–Ionica–Delaplace (ePrint 2024/992), Solovyev's preprint, Faugère–Wallet (HAL access denied): **recalled via the cards only**, not fetched.

## 3. Prior art: these repositories

- `/home/user/crypto/research/lit2026_w3_ggm_audit_20261011/README.md`: supports. The SGGM mapping, "interpretation, not a theorem of the paper", the DKM arm classification, 208 rows none below any bound (min ratio 38.9×), 122 with δ_upper = 1, the EC section missing from the copy read.
- `/home/user/crypto/RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md` §0–2.4: supports. The generic ceiling table, `b ≥ 1/(2(m−1))`, lattice reach 1/6, 1/24, 1/80, m = 4 the first arity that could win.
- `/home/user/crypto/RESEARCH_IC_LEADS_AND_CONNECTIONS_20261010.md` §0 and Q1–Q3 (lines 79–89): supports, with one caveat. The file has no heading "0.1"; the card means numbered item 1 of §0. It calls the generic ceiling "a theorem, not a heuristic" while applying CGK with the `|Aut|` divisor, which none of the four papers states.
- `/home/user/crypto/research/literature_search_20261011/README.md` §1 and `/home/user/crypto/research/ic_leads_review_20261018/README.md` §2.5: supports; they restate the two bounds, and §2.5 adds "every table-lookup IC hybrid is non-adaptive".
- `/home/user/crypto/research/pdp_root_finder_requirements_20261012/README.md`: supports Qc. Gate G3, admissible degree 2–3 against measured 7, F4 slopes 1.6–2.4 bits per tower variable against 0.33–0.75 for enumeration and pair table.
- `/home/user/crypto-autoresearcher/knowledge/findings/KN-FIND-007.md`, `KN-FIND-14706d.md`: supports. Yield conservation; membership collapse of a class-type filter.
- `/home/user/crypto/docs/ic/LEADERBOARD.md` (grep): no Gaudry or trace-zero rows.

## 4. Why it could lower the Gröbner cost or the degree of regularity

A gate lowers nothing. It fixes what each lead must still show.

| m | SGGM necessary b (`>1/(2m)`) | IC-level b (`>1/(2(m−1))`) | repository reach (S_{m+1}) |
|---|---|---|---|
| 2 | 1/4 | 1/2 | 1/6 |
| 3 | 1/6 | 1/4 | 1/24 |
| 4 | 1/8 | 1/6 | 1/80 |

Tally by what each lead must show: **E** 01 needs a D(n) slope below the pair table; 06 is already published, needs a baseline row; 10 needs b* ≥ 1/8 at m = 4. **C** 02, 03, 04, 07, 08, 09, 11, 13, 14. **M** 05, 12. **X** none whole.

Desk arithmetic, unverified, script not committed:
- **DKM headroom.** At `T² ≪ N`, success ε needs `S·T/N ≥ ε/(4 ln 2)` = 0.36 (ε = 1) or 0.18 (ε = ½). The W3 multiplier-closure arms sit at 0.739–1.432, so no non-adaptive table arm can gain more than about 2.1–4.0× (ε = 1) or 4.1–7.9× (ε = ½). That bounds gains from a theorem whose constant need not be tight; it promises none.
- **Slope sensitivity.** With D = s·n and about n variables, a dense Macaulay width gives `log₂ cost / n` of 0.81 at s = 1/20 and 0.48 at s = 1/40 (ω = 2.8). Rho is 0.5. So a constant-factor change in the D slope moves the exponent, which is why 03, 01, 07 and 14 cannot be called constant on group-order arguments alone.

## 5. Obstructions

- **Table and enumeration oracles** (04, the table half of 09, 11 if confirmed): CGK and Shoup fix the exponent. DKM puts the non-adaptive ones at `S·T ≳ N`, above Bernstein–Lange's adaptive `S·T² ≈ N` at equal advice. Constants only.
- **Structured oracles:** SGGM excludes sparse structure (δ < N^{-1/2}) and nothing dense. For prime fields the repository's reach (`b ≤ 1/80` at m = 4) is below the necessary `1/8`; that exclusion rests on the EC mapping (interpretation).
- **Algebraic oracles:** no theorem applies. The measured negatives bind instead: D = 7 at m = 3, 4; F4 slopes above the pair table; the W3 audit found no row below any bound.

## 6. Minimal discriminating test (frozen protocol)

Gate, no test.

## 7. Ranking inputs

Gate, no test. | expected gain: none | confidence: medium | cost: about 0.5 person-day to close §8 | tier: gate | depends on: nothing; 01, 03, 07, 14 depend on LEAD-05's slope fit.

Recommended order from the gate: LEAD-05 first; then 01 and 06; 10 only with its trigger raised to b* ≥ 1/8; the C rows as engineering.

## 8. Open questions

### The uncertain classifications, and what would settle them

1. **LEAD-03, and with it 01, 07 and 14: constant or slope?** The symmetry group has order `ℓ^{m−1}m!`, independent of n, which is why the repository calls it a constant. But F4 cost is exponential in D, so if symmetrisation lowers the slope of D(n) rather than its offset, the exponent moves (§4). Settled by LEAD-05's fit of D(n) at four or more sizes with and without the reformulation, read against G3 with disjoint intervals.
2. **LEAD-09: is the Ore elimination dense?** `Res_{x₂}(S₃, L_V)` has degree about `2^{ℓ+1}` in x₁; if it is dense the cost is at least `2^ℓ`, the enumeration scale, and the verdict is C. If linearised structure gives a poly(ℓ) representation, it is not. Settled by counting terms of the resultant at ℓ = 4…8 before any timing. CGK and Shoup do not decide it, since the oracle reads coordinates.
3. **LEAD-06: comparability, not exponent.** Gorla–Massierer already give `Õ(q^{2−2/(n−1)})`, so the card's "constant" understates the literature. The open point is the baseline: trace-zero groups arise only for base-changed curves, and the leaderboard has no such row. Settled by freezing a base-changed baseline row, then reading G–M's heuristic assumptions (hyperplane-section factor base, semi-regularity) against it.

LEAD-11 is unread rather than uncertain.

### Not settled from here

- The EC theorem of 2026/384 is unread. The abstract promises "elliptic-curve points"; the copy the W3 audit read had only small integers, smooth integers and smooth polynomials. Either the ePrint version added it, or the EC claim covers a case the W3 copy lacked. Until read, every EC reading of δ is the repository's derivation, and unique factorization fails for EC sums.
- The `S·T² ≳ N/|Aut|` form used throughout is in none of the four papers.
- DKM covers only challenge-independent pre-translated queries and the pure generic group; whether an IC online phase that adapts between rounds escapes it is the authors' open problem.
- Crossbred's exponent on non-quadratic, square descended systems (07), and whether G–M's assumptions hold at n = 3, q ≈ 2^20 (06).

## 9. Sources

Fetched (outcome):
- `https://eprint.iacr.org/2026/384` 200 (abstract); `…/2026/384.pdf` 403 (Cloudflare, WebFetch and curl); `https://ia.cr/2026/384` 302 to the abstract.
- `https://www.cs.utexas.edu/~dwu4/papers/SGGM.pdf` 503 twice via WebFetch; curl with `/root/.ccr/ca-bundle.crt` fails TLS ("unable to get local issuer certificate"), verification not disabled. `…/sggm.html` and `…/talks/StructuredGGM0526.pdf` 503.
- `https://people.eecs.berkeley.edu/~henrycg/pubs/structured-generic-groups/` 200, only the ePrint link; `https://cryptology.city/References/CHW26---The-Structured-Generic-Group-Model` 200, abstract; `https://iacr.org/news/index.php?next=27853` 200, abstract; `https://projects.cwi.nl/crypto/studentseminar.php` 200, no abstract; `https://par.nsf.gov//servlets/purl/10655064` 404; `web.archive.org` blocked by tool policy; Semantic Scholar API 429.
- `https://arxiv.org/abs/2505.00894` 200; `https://arxiv.org/pdf/2505.00894` 200 (read).
- `https://eprint.iacr.org/2017/1113` 200 (abstract); `…/2017/1113.pdf` 403; `https://people.csail.mit.edu/henrycg/files/academic/papers/eurocrypt18discrete.pdf` 404 (guessed path).
- `https://cs259c.stanford.edu/finalpapers/genericgroups.pdf` 301 then `http://web.stanford.edu/class/cs259c/finalpapers/genericgroups.pdf` 200 (secondary).
- `https://eprint.iacr.org/2014/318` 200 (abstract); `https://arxiv.org/pdf/1405.1059` 200 (read).
- `https://eprint.iacr.org/2017/1262` 200 (abstract); `https://hal.science/hal-01658573` access denied (Anubis).
- Eight WebSearch queries (2026/384 and its EC section, three of them; CGK; Shoup; G–M; Gaudry), used only to locate the above.

Repository paths read: `research/pdp_literature_leads_20261022/{BRIEF_SONNET.md,LEADS.md}`; the paths in §3. Scratch (not committed): `/tmp/claude-0/-home-user/cf89363d-ad6e-5bef-bde6-195cbd206c93/scratchpad/dreg_slope.py`.
