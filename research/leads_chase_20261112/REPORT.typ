#set page(paper: "a4", margin: 1.7cm)
#set text(size: 9.3pt)
#set heading(numbering: none)
= Chasing the five top-tier index-calculus leads (2026-11-12)

Study `research/leads_chase_20261112/`. Protocol frozen at commit `dccf37b6`, amended before data at `e50d4f23` (S1) and `0056aced` (J2 held-out). Local container, 4 vCPU; one engine per cell unless stated; every number `[pcc:unverified]`. Full text and tables: `RESULTS.md`.

== J2: factor-base constant of the plane-quartic DL1 solver: measured gain, held out
Lowering c_D from the frozen 2.00 to 1.60 cuts the charged DL1 cost to *0.655–0.685×* at k = 21 (size not in the grid) and k = 22 on fresh seeds 4–8, first-attempt success 10/10, and the per-seed order 1.60 < 1.75 < 2.00 holds at w = 0, 0.1 and 1. The success threshold lies between 1.50 and 1.60 at every k = 16…22 (predicted c_50 ∈ [1.40, 1.75]). The preregistered cost band at c = 1.75 is falsified at k = 16 and holds at k ≥ 18. Engineering class on the weak GHS quartic family; not validated. Run report: `reports/run-20261112T0348Z/REPORT.pdf`.
#figure(image("figures/j2_threshold_cost.svg", width: 96%), caption: [J2 grid (seeds 1–3): success and paired charged-cost ratio against c = 2.00 (derived).])

== S1: what carries the prime degree law: the number of equations
The record's small-characteristic cells were void: over F_17, F_97, F_257 the chain ideal is proper for essentially every instance (0 of 60 draws unit), because F_l^× sits inside μ_N. Rescoped before data to l ∈ {19, 43, 107, 227}. With one equation the law holds at every scored cell for every l (288 small cells, 8 at k = 10). With two equations over F_12289 ({g, random h} and {g, σ_257 g}), 1 ∈ V_5(10), one degree below the law; the negative control {g, x_1 g} gives the generic closure. Verdict: the equation-count hypothesis survives; the characteristic-free and long-range-element hypotheses are falsified; the large-characteristic half of the locality hypothesis is not supported at k ≤ 10.
#figure(image("figures/s1_departures.svg", width: 96%), caption: [Exploratory, post hoc: about 1 in 10 small-characteristic unit instances has a non-generic closure (classes 602/826, 603/828 and full 794/857), none among 14 large-characteristic small-order targets apart from the known order-2 case.])

== P1: penultimate-degree quotient: reproduces exactly out of sample
Nine fresh instances on three primes (unit and proper): all 72 plateau cells equal the committed q/2^k to the digit (2.5–5.7 times the base size). This confirms the instance independence of these closures and the band at j ≤ 3; the mechanism test is stage 2 (D = 6, k = 15, predicted q = 167,665; GPU-class).
#figure(image("figures/p1_quotient_law.svg", width: 62%), caption: [q/2^k at the penultimate degree against plateau position: committed cells (lines) and fresh instances (crosses).])

== M1 and J1
*M1 closes* at the measured depths from existing exact three-summand records: LFD 7 at k ≤ 5 and ≥ 8 at k = 6, far above the entropy bar (LFD/k < 0.184 needed). *J1 is blocked* on the instance side: the solver can take a generic quartic with an adapter of about 140 lines, but the lab has no genus-3 point counter and ℓ < 2^64 caps prime-order instances at q ≈ 2^21.

== Next steps
J2: validation gate (replay certificate, validator report) on the c = 1.60 rule, then the same threshold scan on the S family. S1: identify the exceptional loci (what algebraic condition on (a, b, t) gives the full 794 closure), and a k = 14 cell with two equations on the GPU. P1: stage 2 on the rented GPU (owner's call). J1: a genus-3 point counter or a construction with known \#J.
