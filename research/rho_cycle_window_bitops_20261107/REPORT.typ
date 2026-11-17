#set page(paper: "a4", margin: 1.8cm)
#set text(size: 9.5pt)
#set heading(numbering: "1.")

= Rho cycle windows in bitop/v1, and the rho Ir/gate spread
research/rho\_cycle\_window\_bitops\_20261107 · 2026-11-07 · branch claude/rho-cycle-window-bitops · base a2be660914 · class: accounting. All values are unverified assertions [pcc:unverified].

== The cycle windows are charged
The default hyperelliptic rho reference (DistinguishedPointsNegationFullWidthCollideFirst) keeps a 32-entry recent-hash window whose work was not in the bitop/v1 tally; ecbench's tuned rho walker (ic\_boundary TunedWalker, its lanes, Brent under WalkPolicy::v2) had the same gap with its 16-entry ring. Now charged: one word\_op per hash compare of the scan and per push test; the escape step's RNG draw, branch index and two modr\_add (as the gap-limit escape); the collide-first solve and check moved to linear\_algebra and verification. ecbench carries its counts as the search phase's work\_native (cycle\_window\_word\_ops, brent\_word\_ops), so committed records replay and price unchanged. Memory, clearing (control) and the passive instrument window are not charged (docs/units/BITOPS.md, 2026-11-07).

== Which published ratios used the default walk
None: the hec\_bitop\_20261020 rows and the leaderboard section sourced from them used the full-width walk (no window); rounds 14--15 compare rho with rho in walk group operations; round 16 is ecbench GAE. The leaderboard and dashboard are unchanged. To size the change, the rho arm of all 18 hec\_bitop rows was re-derived with examples/hec\_rho\_window\_reprice.rs (same instances, 40 seeds from 0xC0FFEE, counts only). Control: the full-width re-derivation reproduces all 18 committed rho means exactly. Every run verified.

#figure(image("figures/ratio_and_window_share.svg", width: 100%), caption: [Left: IC/rho per row, committed (full-width rho) and against the default walk with the window uncharged and charged. Right: the window's share of the default walk's bitop/v1 total. 18 rows, 40 rho seeds per row, committed IC means; raw/collide-first.jsonl.])

#table(columns: 7, align: right, stroke: 0.4pt,
  [g], [p], [N], [committed IC/rho (full-width)], [default walk, window uncharged], [default walk, window charged], [window share %],
  [2], [41], [1321], [0.2071], [0.20999], [0.20991], [0.039],
  [2], [61], [1399], [0.2213], [0.22362], [0.22354], [0.037],
  [2], [101], [4663], [0.2987], [0.30587], [0.30574], [0.045],
  [2], [151], [7949], [0.4299], [0.44897], [0.44878], [0.042],
  [2], [211], [11813], [0.5558], [0.60883], [0.60849], [0.055],
  [2], [251], [61667], [0.5238], [0.52938], [0.52872], [0.125],
  [3], [23], [6299], [0.1995], [0.19985], [0.19966], [0.094],
  [3], [31], [7333], [0.2567], [0.27048], [0.2703], [0.064],
  [3], [41], [6679], [0.3052], [0.31836], [0.31822], [0.046],
  [3], [61], [124459], [0.3014], [0.33158], [0.33106], [0.158],
  [3], [101], [364747], [0.3572], [0.39393], [0.39328], [0.165],
  [3], [151], [1180351], [0.4309], [0.5215], [0.5207], [0.154],
  [3], [211], [4620611], [0.454], [0.42782], [0.42692], [0.211],
  [4], [17], [5639], [0.7667], [0.79005], [0.78968], [0.047],
  [4], [23], [30223], [0.5355], [0.56271], [0.56221], [0.089],
  [4], [31], [239753], [0.3849], [0.38994], [0.38916], [0.2],
  [4], [41], [333041], [0.437], [0.47129], [0.47063], [0.14],
  [4], [61], [16790591], [0.1144], [0.14367], [0.14327], [0.277],
)

The window costs 18--31 word operations per walk group operation against 10#super[5]--10#super[6] gates per Jacobian step: 0.04--0.28% of the walk; every IC/rho ratio falls by the same 0.04--0.28%. Aside, not acted on: the default walk is up to 20% cheaper than the full-width walk the published rows used, so the genus-4, p = 61 row reads 0.143 against it instead of the published 0.114.

== The rho Ir/gate spread of ic-acct-7
Hypothesis: fixed per-solve work that bitop/v1 does not charge dominates short rho walks. It holds. Committed audit-ir (session binary bf8196c2, seq 9 and 11): Ir = 2.335e6 + 0.0869·gates. Local ladder, ecbench audit-ir on all 12 rho records of the spec with binary 4ff2214e: Ir = 1.441e6 (±1.9e3) + 0.08704 (±0.0007)·gates, R² = 0.9993. callgrind\_annotate names the fixed work, identical in every record: SignedFrobeniusClasses::new ≈ 605,300 Ir (FrobeniusPowers::new 493,698; FrobeniusCanon::new ≈ 110,800), the per-solve build of the Frobenius tables that canon and as\_solve are priced as if precomputed; and full\_price::reprice ≈ 778,000--786,000 Ir, the harness re-pricing its report (JSON parse and SHA-256 of full\_prices.json) inside the bracketed solve. Without them Ir = 5.7e4 + 0.0859·gates (R² = 0.9997) and Ir per gate is 0.095--0.133 on every record, against 0.31--1.22 before.

#figure(image("figures/ir_vs_gates.svg", width: 100%), caption: [Callgrind Ir against counted bitop/v1 gates, rho.signed\_frobenius\_full on K1/F\_2^17 (spec koblitz-symmetrised-n17). Blue: 12 ladder records, whole bracketed solve; green: the same minus the two named fixed parts; red: the two committed ic-acct-7 records. ir\_spread/fit-\*.json.])

Recorded as known constants, not charged: the re-pricing is instrumentation (move it out of the bracket or subtract it in audit-ir); the classes build is per-curve set-up that bitop/v1 omits for every Frobenius-folded rho, to be charged with its own counter as the next step. Open: the rho marginal, 0.086 Ir per gate, is below the IC arm's 0.30--0.32; either IC phases still miss work or the binary-field canon/as\_solve weights over-price the rho walk. Two IC records do not separate the two.

== Limits
Counts and Callgrind only, shared L0 host, no wall time. No AWS identity here, so nothing ran on isolab. Descriptive; no row promoted, no VALIDATION.json, no leaderboard or benchmarks row. Resume checkpoint not pushed (no upload token on the host).
