#set page(margin: 1.6cm, numbering: "1")
#set text(size: 9.5pt)
#show raw: set text(size: 8pt)
#let pu = text(size: 7pt)[`[pcc:unverified]`]
= F5: ROBDD size of the decomposable set X₂ and the Legendre set X_E against random sets

Run F5-RUN-01 of study `compiled_advice_robdd_20261110`. Class: boundary diagnostic; no S, no rho row, no leaderboard change. All measured numbers below are #pu (no pcc claim id exists for this run yet).

== Question
Is the set of decomposable abscissae X₂ (or the Legendre set X_E) more compressible as a reduced ordered BDD than a uniformly random set of the same size, by the margin frozen in PROTOCOL.md (ratio ≤ 0.9, decreasing in n)? The preprocessing bound S·T² ≳ n charges advice by its size; it does not forbid advice that is more compressible than random with a fast membership query (RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md §1; idea E1).

== Mechanism
The indicator of each set, as a function of the n bits of x, is built bottom-up as an ROBDD with a unique table. Its node count is the size of a small-advice representation; if X₂ had structure the count would fall below that of random sets of equal density.

== Method
p = largest prime below 2^n, n ∈ {16, 18, 20, 22, 24}, one prime-order curve per n. F = small-x points with x ≤ B = ⌈p^(1/3)⌉; X₂ = signed two-sums of F; X_E = abscissae with Legendre symbol 1 (density ≈ 1/2); INT = interval [0, |X₂|) as positive control. Orders: msb, lsb, and one budgeted Rudell sifting pass from each (sift_msb, sift_lsb; 2n² swaps, 1.2× growth abort). Null: RAND(X) = 10 uniformly random subsets of [0, p) with |RAND| = |X|, seeds `F5/rand/<n>/<set>/<j>`, same instrument and verification. Every ROBDD is verified by re-enumeration. Native Rust tool, local host (4 vCPU), exact counts, no timing taken.

== Results #pu
Ratio = nodes(X) / mean nodes(RAND); relsd = sd/mean of the 10 RAND counts. Node counts are listed in the order msb / lsb / sift_msb / sift_lsb.

#include "figures/table.typ"

#figure(image("figures/ratio_vs_n.svg", width: 100%),
  caption: [Left: node-count ratio X/mean(RAND) vs n for X₂ and X_E, four orders as markers (open = sifted), bars = RAND ±3·relsd, grey = P1 band [0.9, 1.1]. Right: absolute node counts (log scale), msb order, with RAND means, INT and the 2n bound. Run F5-RUN-01, 10 RAND sets per row, exact counts, local host. Source: `results/run.json`; `figures/make_figure.py`.])

Extremes: X₂ ratio 0.994 to 1.029; X_E ratio 0.998 to 1.001 (all 40 cells in [0.9, 1.1]). Largest z-scores: +3.44 (n = 20, X₂, msb), +2.37, +2.35, −2.34; no multiplicity correction preregistered. Costs: 346,541,069 unique-table operations and 21,415 sifting swaps over the X and INT builds; peak RSS 610,432 kB. #pu

== Reading (PROTOCOL.md §4)
+ Instrument failure: not triggered. INT = 12, 17, 20, 22, 24 nodes against 2n = 32, 36, 40, 44, 48; all 60 ROBDD verifications passed. #pu
+ Lead alive: not met; no ratio ≤ 0.9 (minimum 0.994). #pu
+ Rule 3 applies, verbatim: "Closed: otherwise: the decomposable and Legendre sets are as incompressible as random sets under ROBDDs at these sizes. Says nothing about other compressed representations." #pu

P1 (ratio in [0.9, 1.1] at every n and order, X₂ and X_E): held. P2 (INT ≤ 2n): held. The prior of 0.003 on a downward, decreasing trend was not realised. #pu

== Scope
Says nothing about other compressed representations, X₃, or n > 24. The §9a isolab-eks confirmatory run is open; the host was shared; no timing is a result. The research-kb server was unavailable; no novelty claim is made.

== Provenance
#table(columns: (auto, 1fr), stroke: 0.4pt, inset: 3pt,
  [Card], [`CARD-compiled-advice-robdd-20261110-F5-RUN-01`, sha256 `e946af9134b86caed6dd735ad8904f262f88cb65f0aae12825de425357fe4a79`],
  [Commit], [`da7ac2dbb8dc920652be41efb0be3ee14eaa17a7` (da7ac2dbb8)],
  [Receipt], [sha256 `dd28fdb8f727c92a644ce34878d92c0d3b33860c1595e455c26f74a51bdeed05`; status completed; tree dirty before and after (card file, other studies)],
  [Binary], [sha256 `7bb1a710a6ae3d3df15a78cde927275195f5cf3c521337d058c6bcbf339f8bb2`],
  [run.json], [sha256 `1ab1b4fded683de2fd55b9e53e5f3830a47b9a7229d2f5b2c99d6e18631f0f9a`],
  [FREEZE.sha256], [PROTOCOL.md `e57b8ef0449a0353244770e439fc1c1ee7798f9a915e0d8d3cf49513fe4f76d1`; tool/src/main.rs `c0f41c930679fc016f29864e541bac351272dd49976dd8a23fd2a579e015d019`; tool/Cargo.toml `01cec37983a65158e913ab81b4bc6d8ede694b1fb4050e526f6723d243155278`; tool/Cargo.lock `2912a6c720234c63c2c8ed0622fc99d8ff566ee0b80f873b8df3e8aea0c08c29`],
  [Report build], [typst Python wheel 0.15.2 (scratch venv); figure by matplotlib 3.11.2],
)
