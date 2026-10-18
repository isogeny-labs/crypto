#set page(margin: 2cm)
#set text(size: 9.5pt)
#show table: set text(size: 7.5pt)
#set table(stroke: 0.4pt + gray, inset: 4pt)
#show table: it => { show raw.where(block: false): r => text(font: "DejaVu Sans Mono", size: 6.3pt, r.text.replace("_", "_\u{200b}").replace("/", "/\u{200b}")); it }
= Index-calculus leads: status review and missed opportunities
(2026-10-18)
<index-calculus-leads-status-review-and-missed-opportunities-2026-10-18>
#strong[Status.] Desk review across the four repositories (`crypto`,
`cryptanalysis`, `crypto-autoresearcher`, `ml-cryptanalysis`) plus a
literature scan. No new measurement; every number is quoted from the
file named beside it, and each statement is labelled measured, derived,
or proposed. No security claim about a deployed curve is made or
withdrawn. The previous synthesis is
#link("../../reports/Index%20calculus%20leads%20next%20steps.md")[`reports/Index calculus leads next steps.md`]
(2026-10-14); this note records what changed in the four days since and
what the program is still not doing.

#strong[Question.] Which index-calculus (IC) leads are open today, what
did each measure, and which large opportunities is the program missing?

#strong[Method.] Four parallel sweeps (crypto research directories dated
2026-10-10 to 10-17, backlog cards, problems and journal; the
autoresearcher ledger, goals, decisions and open problems; the
cryptanalysis and ml-cryptanalysis experiment reports; a 30-query
ePrint, arXiv and web scan for 2023–2026), then a cross-check of the
pipeline code paths named in the findings.

== 1. Bottom line
<1-bottom-line>
+ #strong[The baseline is unchanged and still trails rho everywhere it
  is cold.] Table A of
  #link("../../docs/ic/LEADERBOARD.md")[`docs/ic/LEADERBOARD.md`]:
  Koblitz one-target IC at 5.23× to 21× the matched rho for log₂ r
  36.6–47.2; Table B prime field 5.47× to 13.5× at 9.7–23.4 bits. The
  autoresearcher\'s best complete Koblitz solve is 3.29× \[2.80, 4.37\]
  a strengthened rho at N53
  (`crypto-autoresearcher/research/cold_arm_composition_ic_20260921/REPORT.md`).
  No record in any repository claims an end-to-end IC result below rho.
+ #strong[The ten items ranked on 2026-10-14 all ran the same day]
  (#link("../ic_next10_20261014/README.md")[`ic_next10_20261014`]), on
  one shared 4-vCPU, 15 GB container with a 3 GB cap per item. That
  campaign, plus four improvement rounds and the `dreg_closure_*` series
  on 10-16/17, closed or narrowed most of the ranked list. The ranked
  report\'s statement that all ten are unexecuted is stale.
+ #strong[Two exponent-level questions remain, and both are blocked on
  memory, not on ideas] (figure 2). The deciding Weil-descent cell (m,
  ℓ) \= (4, 6) needs about 70 GB dense at degree 5
  (`dreg_closure_driver_20261016/RESULTS.md`); the PKM tower at m \= 3,
  N \= 21 to m \= 4, N \= 20 needs a 256–512 GB host
  (`backlog/XP-20261015-pkm-m3-n21-m4-n20`). The instruments that would
  answer them (msolve, the dense MutantXL closure) already exist and
  passed their positive controls. #strong[The single largest missed
  opportunity is one large-memory node for about a week.]
+ #strong[The ECC2K-130 m \= 4 price is now bracketed at +10.32 bits
  over rho in the optimistic case] (degree 5 held to n \= 131, 2^71.13
  against rho\'s 2^60.81; figure 1). At that distance the unrun symmetry
  arms (FGHR 2-torsion, measured 30–416× elsewhere; the equivariant
  pass) and the unmeasured unit conversion K are no longer rounding
  error. They matter only if the degree-5 row survives the (4, 6) and ℓ
  ≥ 7 measurements; until then they are conditional work.
+ #strong[The program\'s remaining risk is in baselines and
  bookkeeping], exactly as the 10-14 report said, and most of that debt
  is still unpaid: the equal-advice preprocessing-rho control at n \=
  73/83, the `s4_expr` harness audit, the 48 inherited-F4 replay
  mismatches, and the column-touch to group-operation conversion K.

#image("figures/ecc2k130_degree_rows.svg")

#emph[Figure 1. Recorded rows from
`research/notes/ecc2k130/RESEARCH_ECC2K130_DESCENT_DEGREE.md` points
6–8. The m \= 4 rows are in unconverted column-touch units; rho is in
group operations with the measured 2^60.81 constant. Source:
`figures/plot.py`.]

== 2. Status of the leads
<2-status-of-the-leads>
=== 2.1 The five \"do\" items of the 2026-10-10 anchor note
<21-the-five-do-items-of-the-2026-10-10-anchor-note>
#align(center)[#table(
  columns: 5,
  align: (col, row) => (auto,auto,auto,auto,auto,).at(col),
  inset: 6pt,
  [\#], [Lead], [Status], [What was measured], [File],
  [1],
  [Preprocessing-rho control at equal advice (n \= 73, 83)],
  [#strong[Partly open.] Ran at n \= 41, 53, 61 at equal advice; at N83
  a time- and RSS-matched (not equal-bytes) control exists on `main`
  (see correction)],
  [Bernstein–Lange rho at equal or smaller advice is 53–317× faster
  online than compact-orbit IC (317× \[137, 739\] at n \= 41; 53× \[35,
  80\] at n \= 61, K \= 360). The N83 figure (≈50–100×) is a projection:
  the instrument covers n ≤ 63 and the run did not fit a 1 GiB cap. A
  later n \= 61 pairing over 1024 points (BL 3.37 ms vs IC 111.4 ms) is
  `claim_check FAIL / SCHEMA_INCOMPLETE` because BL was replayed from an
  archive, not co-measured. Owner decision 2026-10-15: online rows stay
  on the board. #strong[Correction (2026-10-18, later the same day):]
  `research/20261016-n83-precomputed-rho-control/` (PR \#1987, merge
  `bb023d5d7`) already ran a time-matched and memory-matched
  Bernstein–Lange control at N83 in separate processes; under its frozen
  rule neither N83 online reading survives (the 95% interval of median
  IC/precomputed-rho is above 1 in every matched comparison). What
  remains open is the equal-bytes, one-process-group, co-measured form
  at n \= 73 and 83, which PR \#2163 now provides as a frozen protocol],
  [`ic_next10_20261014/09-equal-advice/README.md`;
  `ic_improve_20261015/README.md:21`;
  `research/20261016-n83-precomputed-rho-control/README.md`],
  [2],
  [Free relations by pair-root self-collision],
  [#strong[Closed.] Mechanism real, yield not],
  [Homogeneous rank 49 of 599 at n \= 61, K \= 600; 6 of 599 at n \= 83.
  The anchor note\'s n⁴K⁴/r prediction was about 60× too high. Residual
  constant landed: 0.960× \[0.946, 0.976\] with free relations on by
  default],
  [`RESEARCH_FREE_RELATIONS_CENSUS_20261012.md`; `ic_improve_20261015`
  item 02],
  [3],
  [Solving-degree cells (4,4), (4,5), (4,6) with FGHR and
  Faugère–Svartz],
  [#strong[Partly measured; deciding cell open.]],
  [msolve F4: (4,4) and (15,5) level at degree 4 where XL read ≥ 7.
  MutantXL closure refutes 2–3 degrees below XL on 20/20 cells. m \= 4
  flat at 4 through N \= 42; m \= 3 rises above 4 at ℓ \= 7 on 8/8
  draws. Two survivors: \"rise at ℓ \= 7\" or \"rise once m·ℓ ≥ 21\";
  (4,6) separates them and is unmeasured (needs ≈70 GB or a sparse
  closure; the sparse engine built on 10-17 is too slow). #strong[The
  FGHR arm was not run; no Faugère–Svartz block run exists anywhere.]],
  [`ic_next10_20261014/06-equivariant-f4-m4/README.md`;
  `dreg_closure_{mutantxl,scale,driver}_20261016/RESULTS.md`;
  `dreg_closure_specialise_20261017/RESULTS.md`],
  [4],
  [Equivariant F4 to full F4],
  [#strong[Done; not a production lever.]],
  [G2 pass bit-identical to full F₂ RREF; 44–73× was against a naive
  kernel, 2–3× against `gf2_elim`. Oracle-level verdict TRADES (0.25× at
  n \= 5, 15.5× at n \= 13), most of it from fall rows, not
  equivariance; production never builds σ-symmetric systems, so the hook
  is dead code. Fall-row gate kept opt-in (`KIC_FALLROW_GATE=1`)],
  [`ic_improve_20261014/02-equivariant-oracle/README.md`;
  `ic_improve_20261015` item 03],
  [5],
  [D₄-symmetrised S₅ lattice at m \= 4],
  [#strong[Lattice route closed generically; the D₄ lattice itself never
  run.]],
  [Multi-polynomial and shared-target lattices: best S₅ δ\* \= 0.0140,
  1.12× over 1/80, against δ ≈ 1/5 needed; kill criterion met. The D₄
  lane instead built a weighted Macaulay solve on E(F\_{p⁴}): C\_w ≈
  1.41·10⁹, 844–863× below plain S₄, 18× below C8, handover n\* \=
  2^118.9; classed engineering],
  [`ic_next10_20261014/05-multipoly-lattice/README.md`;
  `d4_symmetrised_s5_20261015/README.md`],
)
]

=== 2.2 The other ranked items and leads opened since 10-10
<22-the-other-ranked-items-and-leads-opened-since-10-10>
#align(center)[#table(
  columns: 4,
  align: (col, row) => (auto,auto,auto,auto,).at(col),
  inset: 6pt,
  [Lead], [Status], [Measured], [Next step on record],
  [E(F\_{p⁴}) k \= 4 ladder (item 1)],
  [measured, toy],
  [D₄ gives 18.2× \[16.4, 20.8\] on whole-solve S (constant). Double
  large primes: LA exponent 0.495 → 0.420 \[0.405, 0.440\] but relation
  phase 0.238 → 0.333 and whole solve 2.31× \[2.10, 2.62\] costlier,
  crossing only near 2^219. #strong[The stacked D₄ + 2LP arm was not
  run.]],
  [stack them; wire sparse LA (§3.3)],
  [Solution-count census for algebraic bases (item 2)],
  [closed],
  [Kummer, subgroup, Dickson, torus, small-x: 0.93–1.03× decompositions
  per target],
  [none],
  [PKM tower degree (item 3)],
  [#strong[negative for the m \= 4 plateau; m \= 3 inconclusive;
  blocked]],
  [msolve at m \= 4, N \= 20 completes three productive degree-8 rounds
  then hits the 5 GB cap: D ≥ 8, about one degree per eight unknowns
  over N \= 12–20. m \= 3 at N \= 18–24 stopped by the 45-minute wall at
  D ≥ 7. Positive control 16/16],
  [N \= 21–27 on a 256–512 GB host
  (`pkm_lead1_m3n21_20261016/progress.txt`)],
  [Kummer Fourier spectrum (item 4)],
  [closed],
  [0/31 cells above the null; planted bias detected],
  [none],
  [Cubic GHS → genus-3 IC (item 7, rounds 2–4)],
  [#strong[largest measured exponent gain]],
  [double large primes q^1.650 → q^1.300 \[1.292, 1.308\] (q^1.377 over
  2^11–2^15); after tuning the whole solve fell to 0.62–0.65× and the
  exponent to 1.368 against the 4/3 target; linear algebra (local
  q^1.52) is the next lever; memory blocks 2^23],
  [LA engine (`ic_improve4_20261015/README.md`)],
  [Quasi-subfield bases (item 8)],
  [closed],
  [yield 0.214 vs 0.204–0.230; trace-zero prefilter never fires on
  odd-order targets],
  [none],
  [Torelli toy ladder (item 10)],
  [heuristic test],
  [Weil-restriction carriers exist at n \= 7],
  [none registered],
  [Binary MITM pair-table oracle ℓ \= 8–12],
  [engineering],
  [928× (ℓ \= 11), 2,187× (ℓ \= 12) over pairs-and-solve, 0
  disagreements; relation collection stays Θ(2^n), no vs\_rho],
  [ℓ \= 13 at ≈1 GiB
  (`binary_mitm_oracle_l11_l12_20261016/REPORT.typ`)],
  [ECC2K-130 decomposition programme],
  [plan only],
  [best decomposition 42.47 / 55.22 bits above rho; W1–W3 defined,
  nothing run],
  [`decomposition_programme_20261016/PLAN.md`],
  [IC frontier campaign],
  [phase 1 done],
  [no IC arm beats rho at any band weight and none has a fitted exponent
  below rho\'s; phase 2 (RunPod wall + replay) not run],
  [`ic_frontier_campaign_20261015/README.md`],
  [Koblitz linear-row harvest and root-rank gate],
  [landed],
  [1.18–1.89× bench, 1.14–1.20× end to end, defaults],
  [—],
  [Cyclotomic γ-weight base],
  [closed],
  [yield neutral (p \= 0.363), cost up to 12% higher],
  [solver clause untested],
  [N83 online / Tier 1 lane],
  [engineering continues],
  [host routes (AWS, GitHub) blocked; A/A noise open],
  [`problems/n83-online-aa-noise.md`],
  [IC probe latency (N83)],
  [frozen, not run],
  [—],
  [`ic_probe_latency_20261017/README.md`],
)
]

=== 2.3 Autoresearcher ledger (crypto-autoresearcher)
<23-autoresearcher-ledger-crypto-autoresearcher>
Nineteen IC goals are `active`; almost every one is held by
infrastructure, review or custody gates rather than by a negative
result. Items that bear on this review:

- #strong[DEC-20261001-9e675e:] enumerative membership is the
  bottleneck, not IC itself; the approved non-enumerative oracle
  `EXP-ICEX-153c34` is unbuilt.
- #strong[GOAL-DREG-001 item C1] (n \= 15, D \= 6): all runs
  `failed_infrastructure`, peak RSS 11.7–12.2 GiB against an 8 GiB cap.
  A memory failure recorded as an outcome.
- #strong[H-BINSTD-dcc004] (Krylov/Wiedemann vs dense elimination on IC
  matrices): approved, never run. #strong[H-ICEX-5666b2] (\"the second
  half of every IC exponent is cited, not measured\"): proposed only.
- #strong[H-GFPN-9a29be] (EcGFp5 symmetrisation): inconclusive, red team
  broke the construction, no m \= 5 degree measured. Open, not dead.
- #strong[KN-OPEN-5b3a08] (`harness/semaev.py` `s4_expr` bug,
  whole-curve sampling) may contaminate 21 archived runs; no audit
  found.
- Negatives whose ground was process or infrastructure, not measurement:
  H-STR-002 (missing artifacts), H-DTREE-001 (design infeasible), the
  ECTD arity-3 panel (17 of 20 arms lost to infrastructure), H-IC-001
  (criterion revised post hoc).
- `problems/full-width-ic-arity-grid-unmeasured.md`: no complete
  50–63-bit prime or ordinary-binary row exists; msolve refuses the
  63-bit field.

=== 2.4 ml-cryptanalysis and cryptanalysis
<24-ml-cryptanalysis-and-cryptanalysis>
- #strong[No learned component feeds the measured IC pipeline.] M7/M9
  hardness and decomposability predictors are null beyond size and the
  planted |Aut|/2 law (`experiments/hardness/report.md`,
  `experiments/fpn-degrees/report.md`). The F\_{p^n} degree labels agree
  with the Macaulay bound on 5,592/5,592 rows.
- cryptanalysis holds the fastest Boolean F4 (`f4_gf2.rs`, GPU: degree-6
  matrices 426 s → 5.7 s on an RTX 5090) and a fitted solving-degree
  rule D \= 2 + \[e \< −3.5\] + \[e \< −8.5\] (MAE 0.06 over 656 bases),
  with geometric-progression bases 15–40× cheaper per query
  (`experiments/pdp-degree-heuristics/README.md`). None of these exist
  in `crypto` under those names, and the rule is not used as a
  factor-base prior.
- cryptanalysis\'s own ledger: no engine beats exhaustive search at the
  PDP stage, the pair table is still the best whole row, and the learned
  solver selector chose a constant pair-table policy.

=== 2.5 Literature, 2023–2026
<25-literature-20232026>
The summation-polynomial and Weil-descent literature is dormant since
the 2020 subfield-curve (ePrint 2020/1315) and SAT (ePrint 2019/313,
WDSat) papers; targeted searches for new solving-degree, tower,
Artin–Schreier, Z\[τ\], sum-product or volcano-decomposition ideas
returned nothing. No ECDLP record since 2016–2020; ECC2K-130 remains
open. Three external items change how this program should read its own
results:

- #strong[Structured generic-group model] (Corrigan-Gibbs, Henzinger,
  Wu, ePrint 2026/384, EUROCRYPT 2026): DL costs Ω(min(√q, 1/δ)) for
  algorithms exploiting structure on a δ-fraction of elements, with
  analogous bounds for elliptic-curve point structure. An EC index
  calculus must exploit structure in at least a 1/√q fraction of points
  to beat rho: the decomposition probability wall, stated as a theorem
  in-model.
- #strong[Non-adaptive time–space bounds] (Dinur, Keller, Marmor, ePrint
  2025/783, STOC 2026): with S below √N advice, non-adaptive DL needs T
  \= Ω(√N); only adaptive walks reach ST² ≈ N. Every table-lookup IC
  hybrid is non-adaptive. This formally closes further pair-table and
  k-sum engineering at ECC2K-130, independently of the repository\'s CGK
  calculation.
- #strong[Crossbred theory] (Vidal, Ionica, Delaplace, ePrint 2024/992;
  improved preprocessing ePrint 2026/360): current state of the art for
  overdetermined Boolean systems. In this repository Crossbred was tried
  only on the m \= 2 toy oracle at n \= 7, 9
  (`research/crossbred_20260920/e2e_panel.txt`), never on the
  Weil-descent degree cells.

Also surfaced: Galbraith\'s Õ(q^0.4) volcano navigation (ePrint
2024/924), HATSolver learned Gröbner orderings (arXiv 2512.14722), and
the group\'s own Blackwell rho kernel (ePrint 2026/2469, 14.6–15.9 G
iter/s). None narrows the IC-vs-rho gap.

== 3. Opportunities the program is missing
<3-opportunities-the-program-is-missing>
Ranked by exponent leverage first and cost second. Costs are my
estimates from the sizes and timings the notes report, not measurements.

=== 3.1 One large-memory node, one week (the largest)
<31-one-large-memory-node-one-week-the-largest>
Both open exponent questions are answered by instruments that already
exist and already passed positive controls; what is missing is RAM.
Every run since 10-14 has had 3–15 GB (figure 2).

#image("figures/memory_wall.svg")

#emph[Figure 2. Recorded caps and requirements:
`ic_next10_20261014/README.md` and `RULES.md` (3 GB per item, 15 GB
container), `03-pkm-degree/README.md` (5 GB msolve cap, f4\_fp\_tower N
\= 20 \> 14 GB), `dreg_closure_driver_20261016/RESULTS.md` (≈70 GB),
`backlog/XP-20261015-pkm-m3-n21-m4-n20` (256–512 GB). Source:
`figures/plot.py`.]

- #strong[Weil descent, m \= 4.] Run (4, 6) with the dense MutantXL
  closure (≈70 GB at degree 5, N \= 40), then m \= 3 at ℓ \= 8 and m \=
  4 at ℓ \= 7 to measure the slope of the degree past the rise. Three
  cells decide between \"degree 5 holds\" (+10.32 bits at ECC2K-130) and
  \"degree follows ℓ\" (+112 bits). Nothing else in the binary program
  has that leverage. Run the FGHR 2-torsion arm in the same session; it
  was specified on 10-10 and never run.
- #strong[PKM tower, prime fields.] msolve at m \= 3, N \= 21, 24, 27
  and m \= 4, N \= 20, 24 on 256–512 GB. The m \= 4 plateau already
  broke (D ≥ 8 at N \= 20); the m \= 3 kill test (D ≥ 8 at N \= 21, ≥ 9
  at N \= 24) could be evaluated neither way under the 45-minute wall.
  This is the only question in the prime-field line that decides an
  exponent (`elliptic_snfs_survey_20261016` §7).
- #strong[Autoresearcher DREG C1] at n \= 15, D \= 6 (needs about 12
  GiB) and the (15, 5) degree-8 cell (18–50 GB) ride the same node.

A memory-light alternative worth building only if no node can be had: a
GF(2) block Wiedemann (Coppersmith) consistency test on the Macaulay
matrix. The repository\'s block Wiedemann is over Z/r only
(`koblitz_sparse_la.rs`); it would give the plain-XL degree, which the
closure studies show is 2–3 degrees pessimistic, so it is a bound rather
than the measurement.

=== 3.2 Pay the baseline and bookkeeping debt before the next claim
<32-pay-the-baseline-and-bookkeeping-debt-before-the-next-claim>
These are cheap, and each one can flip the sign of a headline row.

- #strong[Equal-advice preprocessing rho at n \= 73 and 83,
  co-measured.] Extend the n ≤ 63 instrument to the wide field and run
  BL and IC in one session under one resource envelope with replay
  certificates, as the n \= 61 diagnostic\'s own next step says.
  (Correction, same day: the N83 time- and RSS-matched control already
  exists on `main`, `research/20261016-n83-precomputed-rho-control/`;
  the equal-bytes co-measured form is what PR \#2163 adds.) Record
  `log₂(S·T²) − log₂(r/|Aut|)` on every online row, as the anchor note
  asked (not verified to be on the board today).
- #strong[Measure K], the column-touch to group-operation conversion.
  Every m \= 4 price row at ECC2K-130 is in unconverted units; at +7.6
  to +10.3 bits the conversion alone can decide the sign. One perf-ipc
  session on the current oracle.
- #strong[Audit `s4_expr`] (KN-OPEN-5b3a08) against EXP-MTIC-001 and
  EXP-FIB-001, and #strong[root-cause the 48 inherited-F4 replay
  mismatches] (120/168 identical, `ecbench_frontier_20261013`).
  Inherited F4 is the default Gröbner engine; its counted cost is not
  reproducible across hosts until this is closed.
- Relabel the autoresearcher negatives whose ground was infrastructure
  (DREG C1, ECTD panel, H-STR-002, H-DTREE-001) so they stop being cited
  as results.

=== 3.3 Measure the linear algebra as an object, and make the good
engine reachable
<33-measure-the-linear-algebra-as-an-object-and-make-the-good-engine-reachable>
The relation matrix has never been measured (REVIEW-UNADDRESSED §4.1
\#6; H-ICEX-5666b2). #strong[Correction (2026-10-18, later the same
day):] the check below was made on the PR base `f85012683`; `main`
already carried `sparse-cado` (CADO-style filter followed by block
Wiedemann inside `ic.pipeline`, PR \#1874, merge `ae8ac7df9`). What was
unreachable from the pipeline was block Lanczos and the unfiltered
Krylov cores; PR \#2162 wires `wiedemann`, `lanczos`, `filter+lanczos`
and `filter+wiedemann` under the full-rank completion rule. Original
text: at `f85012683`, ecbench `ic.pipeline` offered only dense or
structured Gauss (`src/cryptanalysis/ic_framework/linalg.rs:250-279`;
the test at `:379` asserted that `"wiedemann"` is rejected), while block
Wiedemann and block Lanczos with CADO-style filtering sat in
`koblitz_sparse_la.rs`. This matters in exactly the two regimes where IC
has a sub-rho exponent: the E(F\_{p⁴}) ladder (LA is 0.495 of the
exponent plain, 0.420 with large primes) and genus-3 after cubic GHS (LA
local exponent q^1.52 is the named next lever). H-BINSTD-dcc004 is
already approved. Glue work, a few days, and a prerequisite for the
first admissible IC bound.

=== 3.4 Stack the measured gains in the proven-exponent regime
<34-stack-the-measured-gains-in-the-proven-exponent-regime>
- #strong[E(F\_{p⁴}): D₄ (18.2× constant) with double large primes
  (exponent)]. The stacked arm was never run; the plain 2LP arm loses by
  2.31× only because its relation phase worsens, which D₄ attacks
  directly. Run it end to end in ecbench with sparse LA; this is the
  cheapest route to the first IC row on any frontier.
- #strong[Genus-3 after cubic GHS]: double large primes gave −0.35 in
  the exponent; the whole solve is at 0.62–0.65×. Add the sparse LA and
  push past 2^23. Weak class, but it is where the program can build and
  validate an HPC-grade IC pipeline (filtering, block Wiedemann, replay)
  before any real target needs one.

=== 3.5 Conditional on the degree: the symmetry arms at m \= 4
<35-conditional-on-the-degree-the-symmetry-arms-at-m--4>
Only if (4, 6) and the ℓ ≥ 7 cells keep the degree at 5. Then the
remaining +10.32 bits is within reach of measured constants that have
never been applied to this system: FGHR 2-torsion symmetrisation
(30–416× measured in the Gaudry setting, about 5–8.7 bits), the
Faugère–Svartz character blocks (never run; the C ≈ D^3.1 projection
overstated D₄ by 13×, so discount any projected figure), and the
equivariant pass (2–3× honest, plus memory reach). Stack, do not
compare; and convert K first (§3.2).

=== 3.6 Literature imports worth a measured trial
<36-literature-imports-worth-a-measured-trial>
- #strong[Crossbred on Weil-descended Semaev systems], same PDP
  instances as F4 and WDSat, with the 2026/360 preprocessing. Never
  tried beyond n \= 9, m \= 2.
- #strong[Last fall degree] (Huang–Kosters–Yang–Yeo, arXiv 1505.02532)
  as an order-independent and cheaper instrument than Macaulay
  refutation for §3.1.
- #strong[Learned orderings] (HATSolver-style) on Semaev and PKM
  systems: a constant-factor lever, but a cheap one given the
  ml-cryptanalysis tooling.

=== 3.7 Close the ML and code loops
<37-close-the-ml-and-code-loops>
- Use the cryptanalysis fitted degree rule and the geometric-progression
  base result as a factor-base prior in `fb-search`; today no learned
  component touches a measured IC run. Running the ready
  `fpn_decomposable` config (about 10 CPU-minutes) turns a null into a
  citable one.
- Port `f4_gf2.rs` / `f4_gpu` (25–75× on degree-6 matrices) into
  `crypto`, and crypto\'s newer Koblitz core back into `cryptanalysis`.
  No GPU IC kernel in `crypto` has ever run on a device.
- Build the non-enumerative membership oracle `EXP-ICEX-153c34`; the
  ledger\'s own decision names enumeration as the bottleneck.

=== 3.8 Stop, or label as engineering
<38-stop-or-label-as-engineering>
Pair-table, k-sum and 6-sum designs at ECC2K-130 and new rungs at n \=
85, 89 (now formally non-adaptive, Dinur–Keller–Marmor, on top of the
CGK line); first-fall-degree measurements (flat at 3); factor-base shape
at m \= 2; isogeny, volcano, conductor and Kani transport for Koblitz
(h(O\_K) \= 1, identical ranks; Galbraith 2024/924 lowers navigation
cost but there is nothing to navigate to); S₃/S₄ constant-factor work on
prime fields; further algebraic factor-base censuses (0.93–1.03×);
further Kummer spectra (0/31).

== 4. Recommended order
<4-recommended-order>
#align(center)[#table(
  columns: 5,
  align: (col, row) => (auto,auto,auto,auto,auto,).at(col),
  inset: 6pt,
  [\#], [Action], [Regime], [Kind], [Rough cost],
  [1],
  [(4,6), m \= 3 ℓ \= 8, m \= 4 ℓ \= 7 with FGHR arm; PKM N \= 21–27],
  [binary and prime],
  [exponent],
  [one 256–512 GB node, about a week],
  [2],
  [Co-measured equal-advice BL rho at n \= 73/83; record S·T² per online
  row],
  [Koblitz],
  [integrity],
  [1–2 days, needs the wide-field instrument],
  [3],
  [Measure K; audit `s4_expr`; diagnose the 48 replay mismatches],
  [all],
  [integrity],
  [2–4 days],
  [4],
  [Wire block Wiedemann and filtering into `ic.pipeline`; measure the
  relation matrix],
  [k \= 4, genus 3],
  [prerequisite],
  [3–5 days],
  [5],
  [Stacked D₄ + 2LP arm on E(F\_{p⁴}), end to end in ecbench],
  [k \= 4],
  [exponent (proven regime)],
  [1–2 weeks after 4],
  [6],
  [Crossbred and last-fall-degree on the same PDP cells as 1],
  [binary],
  [instrument],
  [3–5 days],
  [7],
  [Port `f4_gpu` into crypto; build the non-enumerative membership
  oracle],
  [all],
  [engineering],
  [1–2 weeks],
  [8],
  [Symmetry arms at m \= 4 (FGHR, Faugère–Svartz, equivariant)],
  [binary],
  [constant, conditional on 1],
  [2–3 weeks],
)
]

== 5. What this review does not establish
<5-what-this-review-does-not-establish>
It does not show that any IC variant beats rho, cold or online. One
statement was corrected after publication: §3.3\'s \"sparse solvers
unreachable\" was true of the PR base but not of `main`, which already
had `sparse-cado` (see the correction there). A second: §2.1 item 1
called the N83 preprocessing-rho control open; a time- and RSS-matched
one was on `main` since 2026-10-16 (see the correction there). It does
not rerun or re-derive any number; two of the anchor note\'s own
calculations (free-relation count, 60× high; the C ≈ D^3.1 symmetry
rule, 13× high) are a reminder that calculations here are not
measurements. The costs in §4 are estimates. The sweeps could not see
uncommitted worktrees or open PRs, so each action should start with a
five-minute check of those places. No canonical graph changes: the
leaderboard, scoreboard and progress timeline were checked and carry no
new value from this review.

== Files
<files>
- `README.md` (this report), `report.typ` (generated by Pandoc from it),
  `report.pdf`.
- `figures/plot.py`, `figures/ecc2k130_degree_rows.{svg,png}`,
  `figures/memory_wall.{svg,png}`.
