#set page(paper: "a4", margin: 16mm)
#set text(size: 9.5pt)
#show raw: set text(size: 8.5pt)

= Round 2: orbit-union τ-slots for the split and hybrid PDP, measured in ecbench

*Study* `research/pdp_split_slots_20261012` · branch `claude/pdp-split-dreg` (PR \#1709) · 2026-10-12/19 · *stage diagnostic plus end-to-end ecbench measurement; no speedup claim.* Protocol `PROTOCOL.md` (registered before any measured run) with amendments 1–3 (three container reclaims; inputs unchanged). Analysis `ANALYSIS.json` (native `pdp_split_analyze`). Full tables: `README.md`. Run reports: `reports/run-*`.

== Question

Round 1 decomposed over the standard subspace `V = span(1, z, …, z^(ℓ−1))` and projected costs. Round 2 runs the same split and hybrid systems as a real `ic.pipeline` oracle (`pdp-split`) in ecbench, on `V` and on its Frobenius closure `∪ σ^j(V)` with τ-slot systems and one column per signed Frobenius orbit, against matched rho on the same public targets.

== Correctness

Every ecbench run recorded verified `[k]G = Q` (Parts A and C); all 576 Part B runs match an exhaustive enumeration exactly. The 20 n = 19 (workload, arm) pairs recorded by both interrupted sessions are bit-identical. Replay audits: `audits/`.

#figure(image("reports/figures/fig-ecbench-s.svg", width: 100%), caption: [Recorded S (left; F4 and SAT arms are lower bounds, solver work uncharged) and priced S (right; F4 word XORs at the pinned ratio, a sensitivity) per arm and curve, against measured strong and signed-Frobenius rho. Medians large, per-workload values small. Sessions in `sessions/`; n = 19 censored as in amendment 3.])

== Results

#table(columns: (auto, 1fr), align: left,
  [Closure vs standard], [On all four curves the closure solved 2.0–3.2× fewer systems per IC run and cost 0.35–0.54× in priced S (R1, R2 held), with 1.5–4.5× the yield per system and the same solving degree (R3, R4 held).],
  [Against rho], [At the pinned XOR ratio the best F4 arm (closure, unsplit, g = 4) is 36× strong rho at n = 11, 60× at 13, 889× at 19 and 25,257× at 17 (ℓ = 5). Under the AGENTS.md §5a band (0 / 0.1 / 1 GAE per unpriced op, every arm), every F4 and SAT arm is *unresolved* against rho: 0.78–0.94× at w = 0, 1,215× and up at w = 0.1. Rankable arms (pair table, standard at n = 13 and 19) are all dearer than rho.],
  [§5a, closure vs standard], [The closure is cheaper at every weight on every workload for unsplit at n = 11, 13, 17 and the chain at n = 11. At n = 13 (chain) and n = 19 (unsplit), one workload reverses at w ≥ 0.1, though the medians favour the closure at every weight.],
  [Part C, n = 17], [On the dimension-9 invariant base the fully charged pair table (`mitm-frobenius:m=2`) costs 4.18× strong rho; `pdp-split` m = 2 prices at 46× (g = 4) and 2,919× (g = 0); the SAT arm's lower bound is 0.86× (R7 held).],
  [Accounting], [Every F4/SAT arm's recorded S is at or below strong rho because solver work is uncharged: a lower bound, not a win.],
)

== Interpretation (labelled)

The Frobenius closure is the better base for split and hybrid systems: same per-system cost, more decompositions per system, by about the ordering factor predicted before the run. It moves the cost by a constant factor of 2–3 and does not change the comparison with rho; the fully charged pair table remains the cheapest complete IC on the curve where both ran. Limits: `r ≤ 2^18`, one round, L0 counts; n = 19 partly censored; XOR pricing omits F4's per-call work.
