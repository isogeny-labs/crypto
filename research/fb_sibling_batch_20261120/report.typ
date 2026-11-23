#set page(paper: "a4", margin: 1.6cm)
#set text(size: 9.5pt)
#set par(justify: true)

= Batched sibling extraction in the `f4_fp` visitor

*2026-11-20 · isogeny-labs/crypto PR \#4826 · record `20261120T0342Z-fb-sibling-batch`* \
*Standing:* every number is `[pcc:unverified]`. This is a single-host, unisolated (L0) component measurement. There is no replay certificate, validator report or `VALIDATION.json`, so it is not an admitted benchmark row.

== Why
The previous study's Frobenius root step cut counted root-step work. Wall time stayed dominated by per-child machinery: a full F4 run on each specialised child, normalisation, allocation and root-finder setup. At $|D| = 4096$ a child cost about 26 µs of wall time against about 5 µs of counted arithmetic.

== Algorithm (`F4Options::sibling_batch`, opt-in)
At a node that branches over a finite domain, the children are one generating set specialised at each domain value.
- *Once per node:* the generators are rewritten with coefficient univariates in the branch variable.
- *Per sibling:* Horner specialisation, then an F4-free triangular back-substitution:
  - take the gcd of each variable's univariates;
  - extract its roots, in closed form for degree at most two (one counted Tonelli–Shanks square root), otherwise with the counted Frobenius root finder;
  - keep the roots in the domain through the shared domain index;
  - substitute each root and recurse.
- *Fallback:* a child with no univariate goes to the F4 walk.
- *Leaves* are checked against the original input.

The solution set is the scan's. The order is the scan's when every child is one univariate ($m = 2$). The mode is off by default and never used with a trace sink or reuse pool.

== Correctness
- Tests: counted square root and roots against brute force; exact sequence equality at $m = 2$; set equality on $m = 3$ chains and dense systems, with the fallback exercised; stop-early returns a genuine solution. The `f4_fp` suite passes 73/73.
- The example asserted sequence or set equality on every target. Its scan and Frobenius counts are identical to the 2026-11-19 record.

== Results
#figure(image("sibling_batch.svg", width: 100%), caption: [Total counted multiplications (top) and wall time (bottom, L0). Labels: scan ÷ batched wall, as printed. Same solution set in every mode.])

#table(
  columns: 9,
  align: right,
  [m], [bits], [$|D|$], [fmul scan], [fmul Frob], [fmul batched], [wall scan ÷ batched], [wall Frob ÷ batched], [fallbacks],
  [2], [16], [64], [527,504], [731,625], [71,950], [5.6], [12.2], [0 / 1,024],
  [2], [16], [256], [8,401,040], [2,927,059], [287,347], [13.1], [14.6], [0 / 4,096],
  [2], [20], [256], [8,388,608], [3,678,558], [360,588], [9.8], [13.8], [0 / 4,096],
  [2], [20], [1024], [67,108,864], [7,442,442], [721,176], [34.9], [14.1], [0 / 8,192],
  [2], [24], [1024], [67,108,864], [8,942,083], [835,668], [32.1], [15.3], [0 / 8,192],
  [2], [31], [1024], [67,108,864], [11,909,735], [868,408], [24.9], [15.0], [0 / 8,192],
  [2], [31], [4096], [536,870,912], [23,800,840], [1,736,732], [*106.7*], [*17.6*], [0 / 16,384],
  [3], [16], [16], [742,021], [3,056,957], [644,404], [4.5], [6.7], [128 / 2,176],
  [3], [20], [32], [3,609,216], [11,417,615], [2,185,596], [4.6], [7.6], [192 / 6,336],
  [3], [31], [32], [3,609,216], [17,947,407], [2,510,460], [4.6], [8.3], [192 / 6,336],
  [3], [31], [64], [17,935,872], [48,515,931], [6,664,872], [5.1], [9.1], [256 / 16,640],
)

== What this shows and does not show
- The batched path has the lowest total counted multiplications and the lowest wall time in all 11 configurations.
  - $m = 2$: 12–18× less wall time than Frobenius and 5.6–107× less than the scan.
  - $m = 3$: 4.5–5.1× less wall time than the scan.
- The small-$|D|$ crossover of the Frobenius path is gone, and counted and wall-time gains now agree.
- Fallbacks occur only in the $m = 3$ chains, on about 1.5–6% of children.
- Not shown: any relation-collection, linear-algebra or complete-DLP change; any comparison with rho; a scaling law.
- At $m = 3$ only set equality holds, so a stop-at-first visitor may see a different genuine first solution.
- Owed: an L2 host, an independent replay, a validator recompute and a `VALIDATION.json`.
