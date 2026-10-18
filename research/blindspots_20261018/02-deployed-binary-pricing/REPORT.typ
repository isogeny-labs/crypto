#set page(margin: 1.8cm, numbering: "1")
#set text(size: 10pt)
#set heading(numbering: none)
= Item 02: pricing binary curves at deployed sizes under the measured degree model

_Assessed 2026-10-18. All numbers are *derived* (closed-form model evaluation), none measured. Protocol: PROTOCOL.md (frozen). Cards CARD-BLINDSPOT-02-01 (sha256 96f35c67...) and CARD-BLINDSPOT-02-02 (sha256 f98dd5c6...)._

== Verdict
*ADVANCES (derived, model level), conditional on a constant-degree model; item-07 gate OPEN by the frozen rule; not a measured improvement.* Level: exponent (derived; arities m >= 4 have floors 2n/5 and n/3, below rho's n/2).

By the frozen rule some design arm flips (gap to matched rho <= 0) at K = 1: 100 design cells at K = 1, 31 of them at n = 571.
Qualifications. (1) The flips come from the constant-D arms (D = 3, 4, 6, 7), which are hypotheses at deployed n; the measured degree evidence stops at n <= 15, l <= 5. The measured growth fit D = l + 4 never flips: gap +105.1 bits at n = 131 to +336.7 at n = 571 (Koblitz, w = 1, K = 1), a loss at every deployed n. (2) m = 3 never flips (non-floor gaps +14 to +72 bits), and the m = 2 negative control never crosses (minimum +24.50). (3) Both sides are lower bounds (unpriced counters), so no gap is a ranking, and a derived crossing is not an improvement over the index-calculus baseline.

#figure(image("gap_vs_n.svg", width: 100%),
  caption: [Gap to matched rho (log2 group operations, K = 1, w = 1) against field degree n, from raw/cells.tsv. Derived. Source: plot_gap.py. Run ids CARD-BLINDSPOT-02-01 / -02.])

== Controls (all passed)
Positive: 30/30 and 16/16 committed rows identical (descent_degree_cost scale table; cost_d4 table); named rows 2^76.31 and 2^88.05; ECC2K-130 rho 60.8090. Range check: minimiser unchanged. Negative: m = 2 minimum gap +24.50 bits, never crosses. Monotonicity in w, K, D: 1392 checks, 0 violations.

== Gap to matched rho, w = 1, m = 4 (log2 group operations)
K = 1 is 1 GAE per column touch, K = 10 is 0.1, floor is 0 (AGENTS.md section 5a band). Negative: at or below rho.

#table(columns: 7, align: (left, right, right, right, right, right, right),
  [*Koblitz*], [*131*], [*163*], [*233*], [*283*], [*409*], [*571*],
  [floor], [-4.35], [-7.83], [-14.07], [-18.93], [-30.99], [-47.29],
  [D=4, K=1], [+7.62], [+4.89], [-0.92], [-5.38], [-16.70], [-31.67],
  [D=4, K=10], [+6.24], [+3.24], [-2.24], [-6.41], [-18.23], [-33.35],
  [D=6, K=1], [+15.50], [+9.94], [+4.87], [+0.44], [-10.58], [-25.38],
  [D=6, K=10], [+12.34], [+8.69], [+3.31], [-0.71], [-11.97], [-26.82],
  [D=7, K=1], [+21.30], [+15.17], [+7.33], [+3.49], [-7.52], [-22.10],
  [D=l+4, K=1], [+105.1], [+122.3], [+160.7], [+187.2], [+253.1], [+336.7])

#table(columns: 7, align: (left, right, right, right, right, right, right),
  [*Random*], [*131*], [*163*], [*233*], [*283*], [*409*], [*571*],
  [floor], [-8.37], [-11.50], [-18.50], [-23.50], [-35.83], [-52.37],
  [D=4, K=1], [+3.61], [+1.22], [-5.35], [-9.96], [-21.54], [-36.75],
  [D=4, K=10], [+2.23], [-0.43], [-6.67], [-10.98], [-23.07], [-38.42],
  [D=6, K=1], [+11.49], [+6.26], [+0.44], [-4.13], [-15.42], [-30.46],
  [D=6, K=10], [+8.32], [+5.01], [-1.12], [-5.28], [-16.81], [-31.90],
  [D=7, K=1], [+17.28], [+11.49], [+2.90], [-1.08], [-12.36], [-27.18],
  [D=l+4, K=1], [+101.1], [+118.7], [+156.2], [+182.6], [+248.3], [+331.7])

Rho (log2 group operations), Koblitz: 60.81, 77.15, 111.39, 136.25, 198.99, 279.75; random: 64.83, 80.83, 115.83, 140.83, 203.83, 284.83.
Sign unresolved inside the band: 5 of 480 design cells change sign between K = 1 and K = 10, 16 between K = 1 and K = 256; they are reported as unresolved, not as wins.

== Crossover n (scan, m = 4, w = 1, K = 1; above 571 is extrapolation)
#table(columns: 3, [*model*], [*Koblitz*], [*random*],
  [floor], [86], [64 (grid edge)], [D=3], [191], [143], [D=4], [223], [175],
  [D=6], [292], [238], [D=7], [324], [270], [D=6, w=2], [533], [475],
  [D=l+4], [none to 4096], [none to 4096])

== Pricing and caveats
Unit: log2 group operations; oracle term = column touches / K. Unpriced on the attack side: Macaulay construction and memory traffic, root extraction, membership tests; on the rho side: orbit canonicalisation, partition hash, distinguished-point table. Both are lower bounds. Memory of the D=6, m=4, w=1 flips: 2^95 words at n = 409, 2^128 at n = 571 (rho O(1), not charged).
The relay check (tools/task_card.py check-relay) reported both relays as not verbatim (missing lines listed in README); it is reported, not repaired; on-disk sha256 values equal the relayed digests and numbers were read from the files. Host L0 (shared container; irrelevant to a closed-form evaluation). Toy scale: degree evidence is measured to n <= 15, l <= 5 only. The model's free-oracle floor already crosses at n = 86, so it is optimistic for m >= 4. The crypto library does not compile at the branch base, so the instrument is std-only and built with rustc (deviation 8).
