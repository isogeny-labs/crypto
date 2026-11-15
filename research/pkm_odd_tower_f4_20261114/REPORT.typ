#set page(paper: "a4", margin: 1.8cm, numbering: "1")
#set text(size: 10pt)
#set heading(numbering: "1.")
#set par(justify: true)

= Odd, mixed and isogeny PKM towers under F4, and the algebraic relation oracle against enumeration

EXP-PKM-ODD-001 · 2026-11-14 · isogeny-labs/crypto PR \#4091 · protocol `PROTOCOL.md` (frozen in `fff5b8f` before any
study row; Amendment 1 run order, Amendment 2 analysis bug fix, Amendment 3 censoring by memory) · rows `runs/` ·
analysis `analyze.py` → `results.json`. *Final:* the run ended 2026-11-14T11:46Z. Of the `m = 3` cells with `S ≥ 27`,
`M3-A3-k3` and `B-M3-A3-k3` completed; `M3-A2-t5`, `M3-A23-n4`, `B-M3-A2-t5` and `B-M3-A23-n4` were killed by the
kernel for memory on the 15 GiB host and are censored (Amendment 3). Counted metrics, unisolated local host,
4 threads [pcc:unverified].

== Results

#table(columns: 3, align: left,
  [question], [verdict (frozen rule)], [what was measured],
  [Validity (C1, C2, A/A)], [*valid*], [274 part A rows, 262 not censored: F4's verdict equals the exhaustive one on all 262. 16 planted rows check. A/A bit-identical (2,537,218,131 ×3). 12 rows censored (memory).],
  [Positive control], [*detects one bit*], [Doubling `|V|` on the degree-2 tower multiplies F4 work by 8.7–13.9 at every step.],
  [Degree 3 vs degree 2 (`m = 2`)], [*gain*, constant], [`r = 0.24, 0.29, 0.30` at `S = 27, 81, 243`. Null: gain. Replicated at `P2`.],
  [Degree 5 vs degree 2 (`m = 2`)], [*gain*], [`r = 0.16, 0.12` at `S = 25, 125`. Null: gain. Replicated at `P2`.],
  [Mixed 2,3,… (`m = 2`)], [*no material effect*], [`r = 0.55, 0.55, 0.35, 0.69` at `S = 36, 72, 216, 432`. Null: same.],
  [Mixed 3,2,… (`m = 2`)], [*gain*, growing by the rule], [`r = 0.49, 0.48, 0.38` at `S = 36, 108, 216`. Null: gain.],
  [Order 3,2 vs 2,3 at equal `S`], [*no material effect*], [`0.89` at `S = 36`, `1.07` at `S = 216`, same targets.],
  [Isogeny vs Kummer, same degrees], [*no material effect*, all three], [I2/A2 `0.76–1.47`; I3/A3 `1.17–1.32`; I23/A23 `0.88–1.22`. Null: same.],
  [`m = 3`], [A23 *no material effect*; A3 *no material effect* (tower) vs *gain* (null), so "elliptic-specific" by the rule, on one borderline cell], [A3 `0.54` at `S = 9`; its `S = 27` cell completed but lies above the `m = 3` degree-2 reference, whose `S = 32` cell was censored, so it cannot be read. A5 and A32 have no in-range cell.],
  [Part B validity (C7)], [*valid*], [88 targets, 80 decided: on all 80 the algebraic oracle returns exactly enumeration's relation set. 8 censored (memory).],
  [Part B: algebraic vs enumeration], [*disadvantage* in every cell], [`m = 2`: 213 → 1.8·10⁵ (A2, `S = 16 → 256`), 220 → 5.2·10⁴ (A3), 317 → 5.9·10³ (I3). `m = 3`: 317 → 7.7·10⁵ (A2, `S = 4 → 16`), 3.1·10⁴ → 1.2·10⁷ (A3, `S = 9 → 27`). Trend: widening (A2) or flat by the rule; never closing.],
)

*Growth.* The least-squares slope of `log₂ field_ops` against `log₂ S` at `m = 2` is 3.11–3.60 for every arm, Kummer and
isogeny (descriptive, no interval). All three draws of each cell gave identical counts, so the F4 work here is set by
the system's shape, not the target.

#figure(image("figures/work_vs_S.svg", width: 100%),
  caption: [Part A, `m = 2`. F4 work against `log₂|V|` for each degree mix and form, tower (left) and the null control
  (right). The lines are parallel: the degree mix moves the constant, not the slope.])

#figure(image("figures/partB_ratio.svg", width: 100%),
  caption: [Part B. Algebraic oracle (F4 plus linear stage) over enumeration, in `F_p` multiplications, on the same
  targets (2 planted, 2 random per cell). Above 0 the algebraic oracle costs more. The dotted line is the 0.5 band.])

== What this says about the questions asked

- *Odd degrees.* Writing `V` with degree-3 or degree-5 steps cut F4's work by a constant factor of 3–8 at equal
  `|V|`. The prediction (no effect) is *falsified* for A3 and A5. The null control shows the same gain, so it belongs to
  the system's shape: fewer, higher-degree tower variables. No arm changed the growth rate, which is the barrier
  note's reading. A constant factor is engineering, not an exponent.
- *The auxiliary curve.* The isogeny form was never cheaper than the Kummer form beyond the band. Its F4 work was
  0.76–1.47× Kummer's for the same degree sequence, and *the auxiliary-curve search was not charged*. The pre-registered
  "isogeny is not cheaper" prediction stands.
- *The slides' algebraic relation search.* Implemented, verified exact (80/80 decided targets), and measured. On these
  sizes it costs between 2·10² and 1.2·10⁷ times plain enumeration of the factor base (5·10⁷ at `m = 3`,
  `S = 32`–`36` in the extension below), and the gap grows with `S` in
  every arm with more than one cell, at `m = 2` and at `m = 3`. The slides' hope that solving beats enumerating at
  `m ≥ 3` is not seen on any measured cell (`m = 3`, `S ≤ 27`). The larger `m = 3` systems did not fit in 15 GiB.

== Extension: the `m = 3` cells on a 128 GiB host (Amendments 4–5)

At the user's request, the cells censored for memory were rerun on a RunPod CPU pod (16 vCPU, 128 GiB; EKS was not
reachable from this session). The binaries were hash-checked on the pod, with the same seeds and 4 threads. A
host-equivalence gate passed: three committed cells (`M2-A3-k5`, `M3-A2-t4`, `B-M3-A2-t4`) reproduced
bit-identically. With memory, the `m = 3` systems at `S = 32` still hit the frozen 1,800 s budget, so they stay
censored for the frozen verdicts (`runs/highmem/`). A separate 24 h-budget extension (`runs/xb/`, Amendment 5;
descriptive, not a frozen verdict) completed all six systems in 44–54 min each, at a peak RSS of 16.4–17.9 GiB, just
above the first host's 15 GiB:

#table(columns: 3, align: left,
  [system], [result], [note],
  [`m = 3`, degree 2, `S = 32` (16 unknowns)], [`5.10·10¹²` F4 mults, `D = 7`, refuted; agrees with exhaustive], [`S = 16` was `1.59·10¹⁰`: about ×320 per doubling of `|V|` (×9–14 at `m = 2`)],
  [`m = 3`, mixed 2,3,2,3, `S = 36` (13 unknowns)], [`5.10·10¹²`, `D = 8`, refuted; agrees], [Equal to the `S = 32` count within 0.03%; observed, not explained],
  [Degree 3 vs degree 2 at `m = 3`, `S = 27`], [`r = 0.45` (gain band)], [Against the reference extended to `S = 32`; the frozen verdict, read at `S = 9`, was 0.54],
  [Part B, `m = 3`, `S = 32` and `36`], [algebraic ÷ enumeration `5.0·10⁷`], [4/4 targets agree with enumeration; up from `7.7·10⁵` (`S = 16`) and `2.9·10⁵` (`S = 12`)],
)

Each random part B target is the same system as the part A refutation draw at its size, and their F4 counts are
identical (`5,097,695,134,360` and `5,096,514,454,550`). The pod ran about 6.9 h (about \$7) and was deleted.

=== Why `S = 32` and `S = 36` cost the same (diagnostic, not study data)

The two `m = 3` refutation counts, `5,097,695,134,360` (degree 2, `S = 32`) and `5,096,514,454,550` (mixed
2,3,2,3, `S = 36`), differ by 0.023%. Two diagnostics (`runs/trace/`, `runs/coinc/`):

- *It is a property of the shapes, not of the instance.* Rerunning both shapes with a different curve, coset `g`
  and target (seed 77), and at the second prime, gives the same counts to 8–9 significant figures. The remaining
  differences of about 10⁻⁹ come from the parallel schedule, and the ratio is 0.999768 every time. F4's work on
  these systems depends on the degree sequence and `m` alone, which fits the null controls matching the tower
  throughout.
- *It is not the same computation.* Per-step logs (`--trace 1`, which reproduces the totals exactly) show
  different runs: 24 against 34 steps, a top degree of 7 against 8, largest matrices of about 117k against 105k
  rows. By degree, the work is 3.76·10¹² at degree 7 plus 1.24·10¹² at degree 6 for the degree-2 tower, and
  3.10·10¹² at degree 8 plus 1.65·10¹² at degree 7 for the mixed one. The totals agree although the parts do not.
- *Reading.* The match is reproducible because each count is a fixed number for its shape. Nothing found here
  forces the two fixed numbers to be equal. They differ in every component, and the match is best read as a
  coincidence between two shape constants. With about 40 shape counts in this study, a pair this close was
  spotted after the fact; it is surprising but not extraordinary.

== Scope and deviations

Kummer and isogeny towers over two primes near `2^30` and `2^31.6`; `m = 2` to `log₂ S = 9`, `m = 3` small.
Nothing here is about cryptographic sizes or Pollard rho, and no claim is made beyond the ladder.
- *Backend:* local, because there is no EKS route.
- *Checkpoint:* `CHECKPOINT_SKIP=no-s3-credentials`, recorded in the pcc ledger.
- *Model tiers:* the director/executor relay was not used, because it needs an S3 benchmark record.
- *Knowledge base:* not available. Searched with `git grep`.
- *Root finding:* the univariate root finding in part B is not counted, which favours the algebraic oracle.
- *Memory:* the `m = 3` systems with 16 (`S = 32`) or 13 (`S = 36`) unknowns were killed by the kernel on the 15 GiB
  host. They are censored, not imputed, and cap the `m = 3` reading at `S ≤ 27`.

Details are in `PROTOCOL.md` §7–8.
