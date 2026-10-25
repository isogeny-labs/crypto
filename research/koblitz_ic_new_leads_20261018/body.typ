= New leads for Koblitz-curve index calculus, screened against prior work (2026-10-18)
#strong[Status.] This is a literature and corpus search with one native exhaustive check. It measures no index-calculus (IC) cost and moves no leaderboard row. No security claim about ECC2K-130 or any deployed curve is made or withdrawn. Each statement is labelled #strong[derived], #strong[verified] (exhaustive native check), #strong[reported] (quoted from a paper, not reproduced) or #strong[proposed].

#strong[Question.] Which IC leads for Koblitz curves, ECC2K-130 first, have not been examined in `crypto`, `cryptanalysis`, `crypto-autoresearcher` or `ml-cryptanalysis`?

#strong[Target.] ECC2K-130 is `E₀ : y² + xy = x³ + 1` over `GF(2^131)`, with `#E₀ = 2722258935367507707729280517973639940516 = 4·r` and `r = 680564733841876926932320129493409985129` (log₂ r \= 129.0). These are the values in #link("../../RESEARCH_ECC2K130_IC_FEASIBILITY.md")[`RESEARCH_ECC2K130_IC_FEASIBILITY.md`] §1; the trace `t = −22283658519494248867` satisfies Hasse.

#strong[Method.]

+ Read the existing lead inventories:
  - #link("../../RESEARCH_KOBLITZ_INDEX_CALCULUS.md")[`RESEARCH_KOBLITZ_INDEX_CALCULUS.md`]
  - #link("../../RESEARCH_ECC2K130_IC_FEASIBILITY.md")[`RESEARCH_ECC2K130_IC_FEASIBILITY.md`] §4
  - #link("../../RESEARCH_IC_LEADS_AND_CONNECTIONS_20261010.md")[`RESEARCH_IC_LEADS_AND_CONNECTIONS_20261010.md`]
  - #link("../../docs/ic/RESEARCH_IC_NOVEL_DIRECTIONS_20261007.md")[`docs/ic/RESEARCH_IC_NOVEL_DIRECTIONS_20261007.md`]
  - #link("../notes/index-calculus/RESEARCH_PDP_SAT_SMT_NAGAO_SURVEY.md")[`research/notes/index-calculus/RESEARCH_PDP_SAT_SMT_NAGAO_SURVEY.md`]
  - #link("../KOBLITZ_SUBFIELD_INDEX_CALCULUS_QUEUE.md")[`research/KOBLITZ_SUBFIELD_INDEX_CALCULUS_QUEUE.md`]
  - #link("../../research_notes/Index%20calculus%20leads%20next%20steps/")[`research_notes/Index calculus leads next steps/`]
  - today\'s #link("../ic_leads_review_20261018/README.md")[`research/ic_leads_review_20261018/`]
  - the 61-paper #link("../literature_search_20261011/papers.csv")[`research/literature_search_20261011/papers.csv`] and #link("../lit2026_speedup_program/README.md")[`research/lit2026_speedup_program/`]
+ Grep every candidate, by author, arXiv or ePrint id, and mechanism name, across all four repositories. A candidate counts as #strong[new] only when no document in any of them examines it.
+ Run about 25 web searches over ePrint, arXiv and the wider web for 2023–2026 work, plus older work never imported.
+ Check the one algebraic claim natively and exhaustively on toy fields (`check/`).

#image("figures/lead_map.svg")

#emph[Figure 1. Candidates screened, with the pointer that shows each grey one is already covered. Source: `figures/plot.py`.]

== 1. Bottom line
- #strong[Four leads are new], in the sense that no document in the four repositories examines them. None is expected to beat rho by itself.
- #strong[L1 closes on arrival.] λ-coordinate factor bases are affine x-subspace bases in disguise (derived, then verified on eight curves). Recording it saves a lane from spending on it.
- #strong[L2 bears on the open exponent question.] Partial enlargement (MXL3/MGB) and the YAGS memory-efficient F4 attack the memory wall that blocks the degree-5 cells deciding the ECC2K-130 price. Today\'s registered big-node run covers degree 4 at (4, 6) and one degree-5 cell. The degree-5 cells at N \= 44–48 need 191–463 GB as a dense echelon (figure 2).
- #strong[L3 and L4 are cheap desk checks.] They supply proved bounds where the program has only measurements and extrapolations. The prior on both is low, and the reasons are stated below.
- #strong[The distance to rho is unchanged.] The optimistic ECC2K-130 m \= 4 row stays at +10.32 bits (2^71.13 against rho\'s 2^60.81, #link("../ic_leads_review_20261018/README.md")[`ic_leads_review_20261018`] §1). Table A of #link("../../docs/ic/LEADERBOARD.md")[`docs/ic/LEADERBOARD.md`] stays 5.23× to 21× the matched rho.

== 2. The leads
=== L1. λ-coordinate factor bases: closed on arrival (derived; verified)
#strong[Why it looked worth asking.] λ \= x + y/x is the fastest representation for binary-curve arithmetic (Oliveira, López, Aranha, Rodríguez-Henríquez, CHES 2013). It encodes the sign: λ(−P) \= λ(P) + 1. A factor base {P : λ(P) ∈ W} with W an F₂-subspace is therefore a membership-linear base that breaks the ± symmetry. The corpus uses λ only as a scalar-multiplication cost (`crypto-autoresearcher/research/endosweep_lambda_20261008/`). No document treats λ-subspace factor bases.

#strong[Derivation.] On `E_a : y² + xy = x³ + ax² + 1`:

- x(2P) \= λ² + λ + a. This is standard; Knudsen\'s point halving rests on it.
- The map L(w) \= w² + w is F₂-linear with kernel {0, 1}. If 1 ∉ W, L is injective on W.
- So W′ \= L(W) + a is an affine subspace of the same dimension, lying inside the hyperplane Tr(x) \= Tr(a).
- Then {P : x(2P) ∈ W′} \= A ⊔ (−A), where A \= {P : λ(P) ∈ W}.

On ⟨G⟩ (odd order r), doubling is a bijection. A relation Σ Pᵢ \= R over A is therefore exactly a relation Σ ±Qᵢ \= 2R over the affine x-base W′, with Qᵢ \= 2Pᵢ. The descended Boolean systems coincide under the invertible linear substitution x-bits \= L(λ-bits) + a.

A λ-base therefore yields no new relation and no new system. It is an affine x-subspace base at the target 2R. That family is already inside the repo\'s factor-base census (`ic_next10_20261014` item 2; the trace-zero prefilter of item 8).

#strong[Verification.] `check/lambda_check.rs` (standalone Rust, output in `check/output.txt`, seed 20261018) checks three things:

- C1: x(2P) \= λ² + λ + a for every point, against an independent chord-based doubling.
- C2: λ(−P) \= λ(P) + 1.
- C3: B \= A ⊔ (−A) and |W′| \= |W| for 12 random subspaces per curve, with dimensions 3, ⌊n/2⌋ and n − 3.

It runs on both curves (a \= 0, 1) over F\_{2^11}, F\_{2^13}, F\_{2^17} and F\_{2^19}: eight curves in all.

#align(center)[#table(
  columns: 7,
  align: (col, row) => (auto,auto,auto,auto,auto,auto,auto,).at(col),
  inset: 6pt,
  [field], [a], [\#E], [C1], [C2], [C3], [size of A over the 12 subspaces],
  [2^11],
  [0],
  [2116],
  [pass],
  [pass],
  [pass],
  [3–273],
  [2^11],
  [1],
  [1982],
  [pass],
  [pass],
  [pass],
  [6–270],
  [2^13],
  [0],
  [8012],
  [pass],
  [pass],
  [pass],
  [7–1029],
  [2^13],
  [1],
  [8374],
  [pass],
  [pass],
  [pass],
  [2–1084],
  [2^17],
  [0],
  [130972],
  [pass],
  [pass],
  [pass],
  [1–16473],
  [2^17],
  [1],
  [131174],
  [pass],
  [pass],
  [pass],
  [4–16588],
  [2^19],
  [0],
  [523492],
  [pass],
  [pass],
  [pass],
  [5–65687],
  [2^19],
  [1],
  [525086],
  [pass],
  [pass],
  [pass],
  [6–65964],
)
]

#strong[Class.] A closure, not an improvement. λ-coordinates can still lower the field-multiplication cost of a group operation. That change applies equally to rho, so it is accounting under AGENTS.md §5a, not an IC lead.

#strong[Certificate.] No `identity.certificate/v1` record is attached. The new claim is a set bijection in characteristic 2, while the certificate format evaluates polynomial identities over F\_{2^61−1}. The polynomial identity it rests on, x(2P) \= λ² + λ + a, is textbook. The exhaustive native check above is the evidence.

=== L2. Memory-frugal Boolean closure for the degree-5 cells (proposed instrument)
#strong[The blocker it attacks.] The question that decides the ECC2K-130 price is whether the closure degree stays at 5 past ℓ \= 7. #link("../ic_leads_review_20261018/README.md")[`ic_leads_review_20261018`] §1 and #link("../dreg_closure_driver_20261016/RESULTS.md")[`dreg_closure_driver_20261016/RESULTS.md`] record the blocker:

- The dense MutantXL closure at degree 5 and N \= 40 needs 760k columns, about 70 GB.
- The sparse engine built on 10-17 is too slow, because the falls are dense across about 10⁵ columns.
- The registered big-node run (#link("../dreg_closure_m4l6_20261018/README.md")[`dreg_closure_m4l6_20261018`]) buys degree 4 at (4, (15, 6)) and (4, (16, 6)), and C1 (3, (19, 7)) at degree 5. Its B cells and C2 are outside the budget.
- At degree 5, N \= 44 and N \= 48 need 191 GB and 463 GB as a full dense echelon (derived, `figures/columns.csv`).

#strong[Two unexamined instruments.] Neither appears anywhere in the four repositories.

- #strong[Partial enlargement] (Buchmann, Cabarcas, Ding, Mohamed, #emph[Flexible partial enlargement to accelerate Gröbner basis computation over F₂], AFRICACRYPT 2010; the MXL3 and MGB solvers). As reported in its abstract and in the MXL3 papers, at degree D it multiplies only part of the degree-(D − 1) polynomials, a leading-variable class at a time. It stops as soon as enough mutants appear, so the echelon holds the rows actually needed rather than every fall times every variable. The repo\'s closure enlarges fully. The full text was not read here.
- #strong[YAGS] (Huang, Hong, Cheng, Chen, Yang, #emph[A memory efficient variant of an implementation of the F4 algorithm for computing Gröbner bases], Springer LNCS, DOI 10.1007/978-3-319-27998-5\_24). This is a memory-reduced F4 from the group behind the fast-exhaustive-search and mutant line of work.
  - #strong[Reported only.] The paper\'s text was not reachable from this container (Springer returned 403). Its exact technique and numbers must be read before any design depends on them.

#strong[Test (proposed).]

+ Add partial enlargement to the existing native closure, keeping the refutation output identical. Run it on the draws whose dense closure is recorded: m \= 3 at (16, 6), (22, 6) and (24, 6), and m \= 4 at (11, 5) and (12, 5).
+ Record the peak echelon rank, peak RSS, and refutation degree per draw.
+ #strong[Correctness gate.] Every refutation degree must equal the dense closure\'s.
+ #strong[Falsifier.] The lead is dead if the peak rank at refutation is at least 0.5 × columns on 3 of the 4 or more draws.
+ #strong[Payoff threshold.] A peak rank below 0.25 × columns puts degree 5 at N \= 44 under 48 GB (figure 2). That runs the B and C2 cells on an ordinary 64 GB host.

#strong[Class.] Instrument (engineering). It moves no S. It decides whether the +10.32-bit optimistic row survives.

#strong[Cost.] About one week of Rust in `koblitz_bench`\'s closure path.

#image("figures/memory_reach.svg")

#emph[Figure 2. Derived memory of a dense GF(2) echelon over all degree-≤D monomials in N unknowns, with columns \= Σ\_{k≤D} C(N, k) and bytes \= rank × columns / 8 (`check/columns.rs`, `figures/columns.csv`). The square is the recorded N \= 40, degree-5 cell. The dashed orange lines are hypothetical rank fractions that L2 would have to reach; they are not measurements.]

=== L3. Agreeing–Gluing on the S₃ chain (proposed desk derivation)
#strong[Source.]

- The originators: Raddum and Semaev (multiple-right-hand-side equations, 2007).
- The bound: Semaev, #emph[Sparse algebraic equations over finite fields], SIAM J. Comput. 39 (2009), ePrint 2007/280. It gives the first rigorous average-case bound for l-sparse systems.
- Later refinements: Improved Agreeing–Gluing (ePrint 2010/140) and MaxMinMax (ePrint 2014/004).
- #strong[Reported figures.] For q \= 2, l \= 3 and m \= n, about 1.113ⁿ against a worst case of 1.324ⁿ (Crypto 2007 rump slides). For 3-sparse systems, q^{n/5.79 + O(log n)} (ePrint 2014/004 abstract).

None of these papers appears in the corpus.

#strong[Mapping.] The Karabina (ePrint 2015/319) and Semaev (ePrint 2015/310) chain S₃(x₁, x₂, u₁), S₃(u₁, x₃, u₂), …, S₃(u\_{m−2}, x\_m, x\_R) is a 3-sparse system over mixed alphabets. Each xᵢ ranges over V (2^ℓ values) and each u\_j over F\_{2^n}.

- Gluing on u\_j is the pair-table join that the binary MITM oracle measures: 928× and 2,187× over pairs-and-solve at ℓ \= 11 and 12 (#link("../binary_mitm_oracle_l11_l12_20261016/")[`binary_mitm_oracle_l11_l12_20261016`]).
- Agreeing adds projection pruning between equations, which the oracle does not do.

#strong[What it would give.]

+ A proved expected per-target cost to set beside the measured MITM probe counts.
+ The Improved Agreeing–Gluing and MaxMinMax gluing orders as a candidate order for m ≥ 4. The repo\'s #link("../chain_split_order_20260924/")[`chain_split_order_20260924`] chose its order empirically.

#strong[Prior.] Low. Gluing is a table method, so the Dinur–Keller–Marmor non-adaptive bound (ePrint 2025/783) and the repo\'s own genericity argument (#link("../../RESEARCH_IC_LEADS_AND_CONNECTIONS_20261010.md")[`RESEARCH_IC_LEADS_AND_CONNECTIONS_20261010.md`] §0) cap total work at the generic line. The published bounds also assume small alphabets, while u\_j ranges over 2^n values, so the mapping may need a new argument.

#strong[Falsifier.] The derived expected cost is at least the measured MITM cost at every recorded ℓ. Then close.

#strong[Class.] Accounting or engineering. #strong[Cost.] 2–3 days.

=== L4. Solving-degree bounds for iterated systems (proposed desk check)
#strong[Source.] M. J. Steiner, #emph[Solving degree bounds for iterated polynomial systems], IACR ToSC 2024(1), arXiv 2310.03637. It extends Caminata–Gorla (solving degree ≤ Castelnuovo–Mumford regularity in generic coordinates) to iterated systems, the standard model for MiMC-type ciphers.

The corpus holds its parents but not this paper:

- Caminata–Ceria–Gorla 2112.10506, in `crypto-autoresearcher/research/THM_FALLDEG_INVARIANTS1.md`;
- the semi-local last-fall paper 2311.02804, as `KN-LIT-a2f7b3`.

#strong[Mapping.] The S₃ chain with auxiliary u\_j is an iterated system: each equation defines u\_j from u\_{j−1} and x\_{j+1}. A bound in generic coordinates that depends on m and ℓ but not n would be a proved ceiling at (4, 6) and beyond, with no big host.

#strong[Prior.] Low. Bounds of this type are Macaulay-like, Σ(dᵢ − 1) + 1. For the descended Boolean system with N ≈ 40 unknowns that gives about N, against measured closure degrees of 4–5.

#strong[Falsifier.] The bound at (4, 6) is at least ℓ + 4. Then close as uninformative.

#strong[Class.] Accounting. #strong[Cost.] 1 day.

== 3. Screened and already covered
#align(center)[#table(
  columns: 2,
  align: (col, row) => (auto,auto,).at(col),
  inset: 6pt,
  [candidate], [where it is already examined],
  [2-torsion quotient coordinate u \= x + 1/x (\= x(P ± πP) on Koblitz curves)],
  [`crypto-autoresearcher/research/golden_factor_base_sat_20260921/`; `RESEARCH_ISOGENY_FIBERS_ECDLP.md`],
  [(π − 1) transport with kernel E(F₂) ≅ Z/4 on a \= 0],
  [#link("../notes/index-calculus/RESEARCH_EXOTIC_COORDINATES.md")[`RESEARCH_EXOTIC_COORDINATES.md`] §10.5–11, measured],
  [Transformer border-basis oracle (arXiv 2505.23696), HATSolver],
  [`lit2026_speedup_program` W9 (not started); `literature_search_20261011`],
  [HPST symmetrisation (IWSEC 2013); curve-model studies (Kodera, Cheng, Miyaji)],
  [symmetrised Semaev notes; `RESEARCH_EXOTIC_COORDINATES.md`],
  [Gorla–Mueller–Petit HFE solving-degree bounds],
  [`crypto-autoresearcher/ledger/corrections/CORR-20260916-0c9c0a.yaml`],
  [Huang–Kosters–Yang–Yeo zero-dimensional last fall; Kousidis–Wiemers first fall],
  [`literature_frontier.md`; literature review 2026-09-26],
  [2023–2026 solving-degree, Crossbred, F4/F5, structured-GGM and non-adaptive bounds],
  [the 61 rows of `literature_search_20261011/papers.csv`],
  [miniGB, the \"CS challenge\" solver for Gaudry–Diem systems (Cheng, 2016 talk)],
  [not in the corpus, but no paper or code was found, so it is not a lead yet],
  [Out-of-core or larger-memory hosts for (4, 6)],
  [`dreg_closure_m4l6_20261018` (registered, rented host)],
)
]

The searches found no 2025–2026 paper on binary-curve point decomposition or Weil-descent ECDLP beyond the ones already in `papers.csv`. This agrees with §2.5 of `ic_leads_review_20261018`.

== 4. Next improvement to try
L2 comes first. It is the only lead here that can unblock a measurement deciding the ECC2K-130 exponent, and its correctness gate is exact. L3 and L4 are desk work that can run in parallel. L1 needs no further work.

== 5. Graphs and canonical records checked
These were checked and left unchanged, because no operation count, verdict or leaderboard row moved:

- `docs/ic-leaderboard.html`
- `docs/ic/LEADERBOARD.md`
- `docs/ic/leaderboard.json`
- `docs/ic/progress-timeline.json`
- `docs/ic/BOUNDARY_TARGETS.md`

There is no `benchmarks/` entry, because nothing was timed. The check\'s wall time in `check/output.txt` is descriptive only.

== 6. Files
#align(center)[#table(
  columns: 2,
  align: (col, row) => (auto,auto,).at(col),
  inset: 6pt,
  [file], [what],
  [`README.md`, `report.typ`, `report.pdf`],
  [this report],
  [`check/lambda_check.rs`, `check/output.txt`],
  [L1 exhaustive native check and its recorded output],
  [`check/columns.rs`, `figures/columns.csv`],
  [derived column and memory table for L2],
  [`figures/plot.py`],
  [layout only, for both figures],
  [`figures/lead_map.svg`/`.png`, `figures/memory_reach.svg`/`.png`],
  [figures 1 and 2],
)
]

Reproduce:

```sh
cd research/koblitz_ic_new_leads_20261018
rustc -O -o /tmp/lambda_check check/lambda_check.rs && /tmp/lambda_check
rustc -O -o /tmp/columns check/columns.rs && /tmp/columns > figures/columns.csv
(cd figures && python3 plot.py)
python3 -c "import typst; typst.compile('report.typ', output='report.pdf')"
```

== Sources
- Buchmann, Cabarcas, Ding, Mohamed, #link("https://www.researchgate.net/publication/221462150_Flexible_Partial_Enlargement_to_Accelerate_Grobner_Basis_Computation_over_mathbbF_2")[Flexible partial enlargement to accelerate Gröbner basis computation over F₂], AFRICACRYPT 2010.
- Huang, Hong, Cheng, Chen, Yang, #link("https://link.springer.com/chapter/10.1007/978-3-319-27998-5_24")[A memory efficient variant of an implementation of the F4 algorithm] (not read; 403).
- Semaev, #link("https://epubs.siam.org/doi/10.1137/070700371")[Sparse algebraic equations over finite fields]\; #link("https://eprint.iacr.org/2007/280")[ePrint 2007/280]\; #link("https://eprint.iacr.org/2010/140")[ePrint 2010/140]\; #link("https://eprint.iacr.org/2014/004")[ePrint 2014/004]\; #link("https://rump2007.cr.yp.to/17-semaev.pdf")[Crypto 2007 rump slides].
- Steiner, #link("https://arxiv.org/abs/2310.03637")[Solving degree bounds for iterated polynomial systems].
- Karabina, #link("https://eprint.iacr.org/2015/319")[Point decomposition problem in binary elliptic curves]\; Semaev, #link("https://eprint.iacr.org/2015/310")[New algorithm for the DLP on elliptic curves].
- Galbraith, Granger, Merz, Petit, #link("https://eprint.iacr.org/2020/1315")[On index calculus algorithms for subfield curves].
- Gorla, Mueller, Petit, #link("https://arxiv.org/abs/2011.01050")[Stronger bounds on the cost of computing Gröbner bases for HFE systems].
- Caminata, Ceria, Gorla, #link("https://arxiv.org/abs/2112.10506")[The complexity of solving Weil restriction systems].
- Kera et al., #link("https://arxiv.org/abs/2505.23696")[Transformer oracles for border basis algorithms].
- Cheng, #link("https://www.math.kyushu-u.ac.jp/activities/6271")[Solving the \"CS\" challenge with miniGB] (talk, 2016; no paper found).
- Bailey et al., #link("https://cr.yp.to/talks/2009.12.04/slides.pdf")[Breaking ECC2K-130 (slides)].
