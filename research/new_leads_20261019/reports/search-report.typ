#set page(paper: "a4", margin: (x: 17mm, y: 15mm), numbering: "1")
#set text(font: "DejaVu Sans", size: 10pt, fill: rgb("#17394a"))
#set par(leading: .65em)
#set heading(numbering: none)
#show link: set text(fill: rgb("#176b89"))
= Five ECDLP leads to chase
*Search round • 19 October 2026 • Conductor T-10 / T-11*

The request: set PR maintenance aside, discover additional leads, and make them concrete. This round combines a bounded source audit with an exact finite module screen. The published mechanisms are established research; the new work is the experiment design and native screening result. No new DLP speedup is claimed.

#table(columns: (1fr, 2.3fr, 2.2fr), inset: 5pt,
  [*Lead*], [*Question*], [*Evidence now*],
  [L1], [Cubic odd-torsion quotient to a rational quartic], [32 abstract-module cases screened; actual transfer not run],
  [L2], [Odd-kernel exit from Smith-negative genus-3 inputs], [Published route; holdout comparison proposed],
  [L3], [Mixed PKM fibers for cheaper solving], [Untested local hypothesis; no extra-relation assumption],
  [L4], [Reach target-useful rank sooner], [Existing cohort exposes cycle/rank gap; policy unbuilt],
  [L5], [Projective genus-2 transfer arithmetic], [Existing backend costs known; replacement unmeasured],
)

== Chase order
L1 is the first construction question. L4 is the direct next experiment on the verified genus-2 pipeline; L5 tests a separate arithmetic change. L2 reuses odd-kernel infrastructure on another source variety. L3 tests solving cost on complete small systems. This is judgment about the next discriminating work, not a speed ranking.

#image("../figures/evidence-chain.svg", width: 100%)

== What was not relabeled as new
The repository already tested quasi-subfield bases, Kummer Fourier bias and supersingular auxiliary PKM towers. It also cites Tian’s cubic odd-isogeny mechanism. This round does not claim to discover those mechanisms or prove no other implementation exists.

Five draft cards contain first experiments, controls and acceptance gates. They are not scheduled or funded. Their producer/verifier and checkpoint TODOs remain explicit. The abstract module screen alone has completed.

#pagebreak()
= Exact screen: what actually ran
All rank-three subspaces of a six-dimensional vector space were enumerated by unique RREF bases, then tested for isotropy and invariance under the specified model Frobenius. The coordinate pairs are (0,1), (2,3), (4,5). q and t below are residues, not experimental field/subgroup identities.

#table(columns: (1fr, 2fr, 2fr, 1.4fr), inset: 5pt,
 [*ell*], [*All rank-3 subspaces*], [*Lagrangians*], [*Model cases*],
 [3], [33,880], [1,120], [8],
 [5], [2,558,556], [19,656], [24],
)

#image("../figures/kernel-counts.svg", width: 100%, height: 111mm, fit: "contain")

At ell=3, q=1, t=1, the companion and scalar models have 1 and 16 stable Lagrangians. At ell=5, q=1, t=2, the counts are 1 and 36. Thus trace and determinant do not determine the candidate population. Companion ell=3, q=1, t=0 has zero.

Every membership decision agrees with stacked-matrix rank. All tested multiplier checks pass; identity and non-isotropic controls pass. Optimized and overflow-checked debug builds replay byte-for-byte. This is same-source replay, not independent-host or independent-implementation validation.

*Limits:* no actual elliptic realizability, smooth rational quotient, polarization, twist/descent, executable map, subgroup preservation or solver advantage has been checked. The modules are not an exhaustive symplectic conjugacy classification.

#pagebreak()
= Construction leads
== L1 • Actual odd-torsion cubic transfer
*Tian, arXiv:2012.07173* motivates a cubic Weil-restriction quotient without requiring full rational 2-torsion on the source. Our abstract screen is only the first gate.

First reproduce the published q=523 example. Register the source and subgroup, compute actual torsion and Frobenius, certify kernel isotropy and rationality, reconstruct the quotient and smooth rational quartic, and check map/dual recovery on 100 public known-answer targets. Wrong-coordinate, non-isotropic and non-stable kernels must reject. Check exceptional points and rational descent explicitly.

Only then freeze a disjoint cohort and compare complete source-to-answer cost with source rho and destination generic solving. Include kernel search, construction and every map evaluation. A geometric quotient alone is insufficient.

Card: `XP-20261019-cubic-odd-torsion-transfer`.

== L2 • Odd kernels beyond the current Smith route
*Tian, arXiv:2007.03172* starts from a genus-3 hyperelliptic Jacobian. This tests a different torsion kernel instead of repeating the existing 2-torsion walk.

Reproduce its published F_257 fixture, then freeze 20 Smith-negative odd-characteristic inputs. Screen actual ell=3 and 5 torsion before attempting a quotient. Compare construction reach with the two-torsion route on the same inputs; keep extension-only quotients as rational-descent failures.

The first pass gate is one verified rational non-hyperelliptic destination with an executable subgroup-injective map on a held-out rejected input. That establishes extra coverage only. Complete transport and solving costs still determine usefulness.

Card: `XP-20261019-genus3-odd-kernel-escape`.

== Distinct unresolved obligations
The source torsion module, isotropic subgroup, principally polarized quotient, rational smooth curve, map domain, exact source subgroup and destination solver each need their own evidence. A positive abstract count cannot fill these obligations. There is no automatic extension of these cubic routes to prime fields or prime-degree binary curves.

#pagebreak()
= Solver and implementation leads
== L3 • Mixed-fiber PKM
Give the three summands separate prescribed tower fibers. Compare homogeneous and mixed systems with random sets matched for actual rational point and sign-orbit counts. Separate training and holdout choices. Start at small N=8,12,16 and certify the complete solution sets before larger runs.

Use full F4/msolve and the strongest available external reference on identical systems. Include fiber selection, algebraic preprocessing, every root, rejected lift and failed target. The gate is lower complete oracle cost, followed by a lower complete one-target IC cost. Lower nominal degree or a fast first root alone does not pass. This variant is proposed here; the PKM slides are background, not a result for it.

Card: `XP-20261019-pkm-mixed-fiber-solver`.

== L4 • Useful rank rather than first cycle
The verified exact-order cohort needed a later dependence in 1,244/1,260 solves. Instrument graph closure, sign anchoring, algebraic rank and nonzero target coefficient separately. Train a policy on disjoint public fixtures; it may inspect current relation state but never the unknown scalar or future draws.

Run the frozen candidate over the entire 1,260-target cohort against the incumbent, equal-cost random selection and independent trials. Charge rejection, restarts, rank updates and source answer verification. An IC improvement need not beat rho to be reported, but the remaining gap must remain visible.

Card: `XP-20261019-genus2-target-rank-collector`.

== L5 • Projective arithmetic with normalization charged
Check formula applicability on all 42 exact genus-2 transfer models. Implement a generic projective backend first. A Kummer arm additionally needs verified rationality, signs and full recovery; differential pseudo-addition is not an ordinary arbitrary addition.

Replay all 1,260 exact-order targets with identical relation seeds, and equally optimized projective/batched source rho. Include normalization at splitting/root extraction, conversions, exceptions, full additions and recovery. Report complete counts and instructions separately; qualified wall time still requires L2/A/A controls. No disappearance of the existing cost gap is predicted.

Card: `XP-20261019-genus2-projective-transfer`.

#pagebreak()
= Reproduction, status and sources
Native code and the protocol were committed before execution at
`1f0656cbb9e8ad76fdac2425d775870892cf2bba`, based on
`555eb93adc0d097aa9c2a702effa86110084d927`.

#raw("./research/new_leads_20261019/reproduce.sh /tmp/new-leads-replay", block: true)

The native tool enumerates and verifies all mathematics. Rendering Python only reads retained CSV values and lays out graphs/documents. See `results/validation.json` for exact commands, compiler, host class and exit status. There is no performance measurement, sampling confidence interval, independent-host certificate or new curve registration in this round.

== Requirement status
#table(columns: (2fr, 3fr), inset: 5pt,
 [*Requirement*], [*Status*],
 [Find additional leads], [Five specific hypotheses and discriminating next experiments recorded],
 [Check against existing work], [Bounded audit of named reports/indexes and exact tracked-term searches completed],
 [Execute first screen], [32 abstract modules; all controls and exact debug/optimized replay pass],
 [Actual new transfer or full DLP runs], [Not performed; open follow-up obligations],
 [Schedule experiments], [Five validated draft cards; none running/funded],
)

The canonical scoreboard, progress timeline, leaderboard and curve/cover atlas remain unchanged: no new complete DLP cost, registered curve, certified transfer edge or qualified speed result was produced. PR #2229 remains unchanged. This branch is a local research continuation; no new PR was opened.

== Primary sources and audit trail
#link("https://arxiv.org/html/2012.07173")[Tian: cubic Weil-restriction odd-isogeny construction], introduction, section 3.4 and conclusion.

#link("https://arxiv.org/html/2007.03172")[Tian: genus-3 hyperelliptic-to-quartic odd-isogeny construction], construction outline and section 6.

#link("https://arxiv.org/abs/0806.2995")[Smith: genus-3 two-torsion isogenies], abstract, comparator mechanism.

#link("https://www.iacr.org/workshops/pkc2016/troll.iis.sinica.edu.tw/pkc16/slides/6-1--Algebraic_approaches_for_the_Elliptic_Curve_Discrete_Logarithm_Problem_over_Prime_Fields.pdf")[Petit–Kosters–Messeng: PKC 2016 author slides], background only.

#link("https://arxiv.org/abs/1510.03174")[Chung–Costello–Smith: Kummer-to-Jacobian recovery], abstract.

#link("https://link.springer.com/article/10.1007/s13389-024-00351-7")[Complete genus-2 Jacobian-coordinate formulas], candidate implementation reference.

Local evidence: `three_leads_e2e_20261018`, `ic_leads_review_20261018`, `pkm_supersingular_aux_20261015`, GHS/Smith studies, problem/backlog indexes. Full bounded search scope and prior-work caveats are in `SOURCES.md`; exact native results and hashes are alongside this report.
