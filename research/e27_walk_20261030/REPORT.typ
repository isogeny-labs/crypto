#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set par(justify: true)
= E27: the matched walk, strengthened (Karatsuba tower products, batched norm inversions)

*Run report and results, 2026-10-30.* Research note: `research/notes/index-calculus/RESEARCH_GLV_INVARIANT_FACTOR_BASES.md` §9.25 (pre-registration, commit `b3a0617ad`) and §9.29 (results). Code `6490d0fac`; binary `af2504ef1`. Records: `experiments/32_walk_karatsuba/` (`stage_w.json`, `e2e_chain_karatsuba_p*s*.json`, `tables.md`, `summary.json`). Host: the 4-vCPU Claude Code container; isolab-eks not reachable from it, so no isolation tier is earned; operation counts only, wall times not admitted.

== What ran
- *Stage W:* E25's ten GLS twists ($p in {101,149,197,269,397}$, seeds 1, 2), 16 $chevron.l plus.minus psi chevron.r$ walks and 16 negation walks each (E25's seeds), each walk run three ways: (i) E25's arithmetic (schoolbook products, 19; one $FF_p$ inversion charged 16 per addition), (ii) Karatsuba tower products (13), (iii) Karatsuba plus Montgomery-batched norm inversions over a lockstep pool of 16 walks (fillers keep the pool full; they are not recorded). 320 recorded walks, 0 failures.
- *IC side:* E31's solver-A pipeline rerun on the same ten curves, both arms (20 runs, 0 failures), with every $FF_(p^4)$ product priced by the Karatsuba tower.

== Measured (all values recorded; derived values say how)
- Per operation ($FF_p$ mults): $FF_(p^4)$ product 19 → 13; distinct addition 97 → 79 (unbatched) → 66.3–66.7 (batched; norm-inversion share 3.27–3.75); doubling 116 → 92; GLS map 38 → 26.
- Identity: all 320 walks equal under (i), (ii), (iii) in steps, group operations, cycle escapes, recovered logarithm, correctness and visit digest; (i) equals all 320 of E25's walk records.
- Walk cost (i)/(iii): 1.4533–1.4634 (derived: ratio of recorded means).
- IC side: $S_A \/ S_(A,K)$ = 1.00475–1.00566; non-solve $FF_(p^4)$ work 2.00–2.48 % of $S_A$; every non-count field of E31 A reproduced. Batching the IC inversions could save at most a further 0.127–0.170 % (bound from the recorded inversion count).
- Distance to the matched $chevron.l plus.minus psi chevron.r$ rho (20 rows): 0 GAE $4.42 dot 10^3$–$2.79 dot 10^5$ → $6.39 dot 10^3$–$4.06 dot 10^5$ (×1.448–1.456); 0.1 GAE ×1.415–1.443; 1 GAE ×1.236–1.325. Rho is ahead of every row at every weight.
- A finding about the reference: main's commit `652238c03` (2026-10-24) began charging the post-collision $[d]G = Q$ check inside every walk (0.13–0.78 % of a walk here). E27 prices walks without it, as E25 did, and records it separately.

#figure(image("figures/walk_cost.svg", width: 100%), caption: [Mean $FF_p$ multiplications per $chevron.l plus.minus psi chevron.r$ walk (16 walks per curve) under the three arithmetics. Source: `stage_w.json`, binary `af2504ef1`.])
#figure(image("figures/distance.svg", width: 100%), caption: [$S \/ "rho"$ per row before (§9.24) and after E27, at 0, 0.1 and 1 GAE (GAE = 97). Source: `summary.json`.])

== Verdicts (pre-registered §9.25)
G2.95 holds (runs; unit tests in the final test build). G2.96 holds. G2.97 holds (1.4533–1.4634; the red team's 1.03–1.3 was low). G2.98 holds. G2.99 holds at 0 and 0.1 GAE, falsified high at 1 GAE on the two $p = 101$ seed 2 rows (1.3252 > 1.30).

== What it says, and what it does not
This is a *correction of the reference*, not an improvement of the index calculus: the same walks cost 1.45× less, so every row is 1.45× further from rho than §9.24 reported at 0 GAE. The IC side gains 0.5 % from the same products (engineering, negligible). No leaderboard or `benchmarks/` row is added: the rows lack a validation-gate `VALIDATION.json` (replay certificate, validator report), which this session cannot produce independently.
