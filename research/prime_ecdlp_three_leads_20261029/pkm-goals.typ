#set page(paper: "a4", margin: 18mm, footer: context align(center)[#counter(page).display()])
#set text(size: 10pt)
#set heading(numbering: "1.")
= PKM: goals and novelty audit
29 October 2026 · proposed work program · no new measured result

The immediate question is whether symmetric PKM lowers the complete cost of a
verified decomposition on actual curve targets. The later question is whether
that moves a cold, single-target ECDLP. The detailed proposal and references are
in `PKM_GOALS.md`; the blocked execution card is
`backlog/XP-20261029-pkm-verified-oracle-program/`.

#image("pkm-goals.svg", width: 100%)

== What is different
PKM's low-degree-map factor bases and summation-polynomial framework were
published in 2016. Algorithm 1 explicitly accounts for permutations. This lab's
elementary-symmetric tower and m=4 S5 instrument already appear in PR2632 and
PR2650. PR2868 adds native point extraction and verification to that reviewed
driver path, with explicit field-branch diagnostics and structural Lean proofs.
It supplies an implementation bridge; no new ECDLP algorithm or complexity
bound has been established.

The useful next contribution would be a complete, reproducible cost/yield
comparison, followed by a verified improvement or a precise structural bound.
Global novelty of the particular presentation has not been established.

== What the older experiments already decided
The raw plain N=20 study reached productive degree-8 steps before a memory cap;
the symmetric N=20 attempt aborted under 6 GB. Merely attempting N=20 is not a
new experiment question. For symmetric m=4, degree 8 is already an input floor,
so equal reported degree can conceal large cost growth. The old timer starts
after constructing the equations. Construction, point extraction, failed
queries and useful relation yield must be brought into the next comparison.

#pagebreak()
= Measurable goals

#table(columns: (0.65fr, 1.5fr, 2.65fr), inset: 6pt,
  table.header([Goal], [Measure], [Completion criterion]),
  [G1], [Faithful point oracle], [Zero lost/invalid supported decompositions against a native exhaustive tiny-curve control; equal domains and explicit unknowns.],
  [G2], [Complete cost and yield], [Setup and every query phase priced or visibly unpriced; all successes verify; misses, caps, branches and valid yield retained.],
  [G3], [Presentation and solver ablation], [Same E,V,Q across chain, plain S5 and symmetric S5; separate qualified external equal-system comparison.],
  [G4], [One measured improvement], [Proposed: at least 20% less full cold oracle-batch cost on held-out data, with a paired interval excluding no improvement across the pricing band.],
  [G5], [Depth and field-size growth], [m=4 first; predeclared completed points plus held-out check, or an explicit resource boundary. Separate fixed-p depth from field-size scaling.],
  [G6], [Cold ECDLP and the proof gap], [Verified useful rows, factor-base logs and target recovery with every phase charged; faithful structural cost bound for any complexity claim.],
)

== First implementation candidates
+ Reuse validated curve/base algebra. Test a target-parameter representation of
  S5 against rebuilding the interpolation grid for every query. Charge cache
  generation, storage, cold setup and the declared reuse count.
+ Reuse triangular/quotient information for exact extraction. Make every
  full-field branch visible; an algebraic-only arm must report unsupported or
  capped extraction when it cannot proceed algebraically.
+ Reduce the largest measured matrix block and live basis. Change layout,
  ordering or tower-aware representation only with an equal-output ablation.

The memory objective is proposed as 2x lower peak memory with at most 10% extra
complete cost, enabling another declared cell under the same cap. These are
planning targets to freeze before data, not observed gains. Earlier F5/signature
criteria already lost on degree and memory; repeating them needs a new reason.
For the cost target, require point ratio at most 0.8, paired 95% upper ratio
below 1, and unchanged or better completion and verified coverage.

== Avoid the two main measurement traps
Planted positives and random field abscissae do not replace an unconditional
public curve-target stream. Report natural valid yield and separate certified
misses from resource-censored calls. At fixed p, increasing tower depth changes
factor-base size and target consistency; that ladder cannot establish an
ECDLP exponent. Matrix width, actual cost and output coverage accompany degree.

#pagebreak()
= Decision boundary and execution

Under the existing rough model with quadratic-scale sparse LA, expected
relation/rank yield and non-dominating setup, per-query cost $r^gamma$ gives
balanced exponent $2(1+gamma)/(m+1)$. At m=4, the arithmetic condition for that
exponent to be below $1/2$ is exactly $gamma < 1/4$.

`lean/ThreeLeads/PKMGoalModel.lean` checks this arithmetic implication for rational
model parameters only.
It does not establish an oracle exponent, the LA/yield assumptions, curve
coverage, a Rust cost bound or sub-rho ECDLP. The real proof goal is the precise
symmetric ideal/filtration, its point-decomposition bridge and a cost bound on a
quantified growing family. Small fixtures and degree plateaus are dependencies
or observations, not that theorem.

== Sequence and stop rules
G1–G3 first, then one justified G4 change, then G5. Attempt G6 once measured
useful relations and accounting justify it. A correctness/verifier discrepancy
stops the affected comparison. Report a null, loss or cap without changing the
held-out set or creating a finite ratio from a censored baseline.
Reject a candidate that misses its frozen target. Repeated caps trigger a
specific memory/representation task. Finite negative scaling constrains that
implementation/model extrapolation, not all PKM presentations.

The initial m=4 depth candidates are t=2,3,4, with t=5 conditional on validated
sizing within an approved cap. Multiple primes and fresh curves/seeds need a
frozen roster. The current driver is restricted to small primes below $2^32$.
Historical plain-arm 256–512 GB projections exceed the default pool's 128 GiB
cap; this proposal requests no capacity increase.

== Status
All scientific goals are pending. The native full-cost panel and replay
verifier, qualified external arm, frozen protocol and supported execution/resume
route are missing. Checkpoint/artifact access was unavailable. The backlog card
is blocked and unfunded; no task, lease, objective or run is posted. The existing
execution gates remain in force. Canonical performance and curve graphs are
unchanged because the proposal adds no new measured value or curve.

== Sources and limits
PKM primary text, sections 1,3–5:
https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf .
PR2632; PR2650 at `adde64fdc22d51dc180fe0b9f30a2d28adf992fe`;
PR2868 at `8baeee2d446968936882adc503570fb957bb1429`;
the PKM tower research note, sections 3.3–3.7 and rounds 1–8;
`research/ic_next10_20261014/03-pkm-degree/README.md`;
`research/prime_fall_degree_20261027/PREREGISTRATION.md`.
APS ePrint 2017/609 returned HTTP403 in this audit. Full Yokoyama text remains
unavailable. No exhaustive global-priority or universal negative claim is made.
