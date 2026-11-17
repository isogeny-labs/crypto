#set page(paper:"a4",margin:15mm)
#set text(font:"Helvetica",size:9.5pt,fill:rgb("233746"))
#set par(leading:.55em)
#text(size:21pt,weight:"bold")[Double-large-prime graph calibration]

18 real-curve targets · 589,824 fixed trials · 15 November 2026 · count study

*M1 exactness passes. M2 forecast criterion fails in one of 36 holdouts.* Both rooted
and full weighted forests recover all 18 targets. Raw outputs, unique equations, core rank,
cycle depth and time-to-target counts are recorded separately. No runtime gain is measured.

#image("ROWS.svg",width:100%)

#table(columns:(1.25fr,1fr,1fr,1fr,1fr),align:right,
 [Genus / prime],[Root rows],[Full rows],[Root weight],[Full weight],
 [2 / 1,009],[15,721],[16,130],[3.75],[3.67],
 [2 / 16,381],[673],[8,442],[3.91],[3.76],
 [3 / 127],[4,638],[4,645],[6.69],[7.07],
 [3 / 1,009],[2,090],[2,407],[8.95],[11.23],
 [4 / 31],[1,563],[1,563],[5.93],[5.93],
 [4 / 127],[979],[986],[8.08],[9.54])

Table: three-target medians at 32,768 trials. Weight is total nonzero coefficients divided
by all emitted rows, including empty rows. Row counts are not rank or speed.

*Frozen forecasts.* Fit Full/FP/PP rates only from the first 2,048 trials. Integrate the
known GTTD rooted-tree equations from zero; 8,192 and 32,768 are holdouts. Compare rooted
known-node count, raw Full outputs and depth sum at the registered factor-two plus five
count tolerance. The failed case is g2, p16,381, seed16 at 32,768: observed rows 744,
forecast 367.595. Known nodes 3,807 vs 2,902.622 and depth 13,739 vs 9,639.259 pass.
No post-run refit, omission or tolerance adjustment was made.

*Scope.* Six reused family-0 fixtures, new target indices 14–16; H is the classical
ceil(p^(1−1/g)) canonical-place count. Rooted simplification drops PP edges disconnected
from root zero; the full coefficient-aware forest retains them. Every split decomposition
is checked in the whole Jacobian, public labels are checked at snapshots, and every returned
scalar is group-checked and matched separately by the harness. No stopping-time benchmark.

*Genus-2 boundary.* Each raw genus-2 relation contains at most two unknown place logs plus
two public coefficients. Exact elimination may therefore put all places in the off-base
forest, leaving only the target unknown. Preserving all equations implies target-determination
prefix invariance under this partition change. The rooted ODE cannot describe the zero-seed
H=0 boundary; a full weighted graph can pin an unbalanced component. This is a known-method
algebraic specialization requiring a separate implementation gate, not a new 10× result.

*Standing.* Native RK4 agrees with closed forms in 48 checks plus a zero-FP control.
Source 353a85552e and protocol 3331b61aba were checkpointed before execution.
Known source: GTTD Sections 3.3 and 4.1–4.4. Count calibration does not establish independent
target rank, statistical confidence, a complete cost fit or mathematical novelty.
