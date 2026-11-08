#set page(paper: "a4", margin: 1.8cm, numbering: "1")
#set text(size: 9.5pt)
#set table(inset: 4pt, stroke: 0.4pt + gray)
#show table: set text(size: 7.5pt)
#show figure: set block(breakable: true)
= Jump count of the F \= 1 target-in-walk arm, single-target ecbench (2026-11-08)
<jump-count-of-the-f-1-target-in-walk-arm-single-target-ecbench-2026-11-08>
#strong[Class.] Measured, single-target ecbench sessions under a protocol frozen before data (#link("PROTOCOL.md"); hashes in `data/frozen.sha256`, freeze commit `a1ce0db2f9`). Counted units only (ecbench GAE, S \= cost/sqrt(r)); every comparison at w \= 0, 0.1 and 1, under the committed band and under `ecbench band --price-table-work`. Public synthetic Curve25519-family miniatures; no security claim. No pcc claim has been filed; every number is an unverified assertion until it is reviewed \[pcc:unverified\]. No code change: the existing `jumps` option of the `wwalk` oracle on the `tw=1,cj=1` arm, binary `/home/user/ctarget-f1/release/ecbench` (SHA-256 `d23fab99...` in `data/binary-k{20,24}.sha256`).

#strong[Question.] The current best single-target arm (`twcj`, 16 jumps) spends 17 % of its k \= 20 cost on building jumps. Its jump indices are drawn independently of the point, so the frozen model predicted that 4 or 8 jumps save that set-up with no walk penalty, and that 2 jumps fail.

#strong[Answer.] The set-up saving is real but the walk penalty is not zero, and the frozen primary prediction failed. With 4 jumps the arm costs 1.220x \[1.186, 1.256\] the 16-jump arm at k \= 20 and 1.268x \[1.193, 1.348\] at k \= 24 (w \= 0), because the walk lengthens from 1.39 to 1.86 sqrt(r) and needs 2.93 collisions instead of 2.08 for two independent rows \[pcc:unverified\]. With 8 jumps the ratio is 1.008 \[0.967, 1.051\] at k \= 20 and 1.050 \[1.005, 1.097\] at k \= 24 \[pcc:unverified\]. With 32 jumps it is 1.051 \[0.996, 1.110\] and 1.014 \[0.978, 1.051\]: the walk is 2.5 % and 1.5 % shorter, the set-up 54 and 61 GAE dearer \[pcc:unverified\]. Two jumps fail as predicted: 3.25x and 3.46x, with the walk reaching its cap below rank 2 in 94 % and 98 % of runs \[pcc:unverified\]. Sixteen jumps is at the optimum of this knob at these sizes; the only direction left is more jumps at larger k, where the set-up vanishes (measured in `research/curve25519_tw_scaling_20261108`). Rho-neg remains ahead: the 16-jump arm is 1.36x \[1.23, 1.50\] rho-neg at k \= 20 and 1.34x \[1.22, 1.48\] at k \= 24 at w \= 0 on these fresh seeds \[pcc:unverified\].

#figure([#image("figures/jumps.png", width: 100%)],
  caption: [
    Jump count: pooled ratios and phases
  ]
)

== 1. Design (as frozen)
<design-as-frozen>
8 sessions per size, `targets_per_curve = 1`, 64 measured rounds plus 1 warm-up per arm, `order = alternate`; arms `rho-neg`, `twcj-j16` (baseline), `twcj-j2`, `twcj-j4` (primary), `twcj-j8`, `twcj-j32`, all pipeline arms on `prime-projected-abscissa:size=1,scan_cap=32` with `targets=stride-generator` and `wwalk:x=1.5,g=1,grow=2,tw=1,cj=1,jumps=J,seed=run`. Audit: `ecbench verify --replay 1` per session. Pooling: `scripts/pooled_compare.py` (unchanged copy of the earlier study’s, warm-up records skipped), ratio of per-session sums, linearised t interval, df \= 7. Runtime: 90 s of sessions at k \= 20 and 2 min at k \= 24 (`data/run-*.log`).

== 2. Results (measured)
<results-measured>
All 16 sessions completed with 390 of 390 records verified each, and every audit reproduced its replay with 0 problems (`data/verify-k*-s*.txt`) \[pcc:unverified\]. Pooled figures use the 3,072 measured records per size (`data/pooled_measured.json`).

Mean S per arm (committed band, then with table inserts also priced):

#align(center)[#table(
  columns: 7,
  align: (col, row) => (right,left,right,right,right,right,right,).at(col),
  inset: 6pt,
  [k], [arm], [S w\=0], [S w\=0.1], [S w\=1], [w\=0.1, inserts priced], [w\=1, inserts priced],
  [20],
  [rho-neg],
  [1.573],
  [1.686],
  [2.711],
  [1.686],
  [2.711],
  [20],
  [twcj-j16],
  [2.136],
  [2.140],
  [2.181],
  [2.279],
  [3.567],
  [20],
  [twcj-j2],
  [6.946],
  [6.976],
  [7.246],
  [7.419],
  [11.676],
  [20],
  [twcj-j4],
  [2.606],
  [2.611],
  [2.658],
  [2.797],
  [4.518],
  [20],
  [twcj-j8],
  [2.153],
  [2.158],
  [2.199],
  [2.304],
  [3.662],
  [20],
  [twcj-j32],
  [2.245],
  [2.250],
  [2.290],
  [2.385],
  [3.642],
  [24],
  [rho-neg],
  [1.179],
  [1.285],
  [2.240],
  [1.285],
  [2.240],
  [24],
  [twcj-j16],
  [1.583],
  [1.584],
  [1.595],
  [1.720],
  [2.950],
  [24],
  [twcj-j2],
  [5.475],
  [5.498],
  [5.705],
  [5.946],
  [10.180],
  [24],
  [twcj-j4],
  [2.008],
  [2.010],
  [2.022],
  [2.187],
  [3.797],
  [24],
  [twcj-j8],
  [1.663],
  [1.664],
  [1.674],
  [1.809],
  [3.124],
  [24],
  [twcj-j32],
  [1.606],
  [1.607],
  [1.617],
  [1.740],
  [2.952],
)
]

Pooled ratios with 95 % intervals over 8 single-target sessions (committed band):

#align(center)[#table(
  columns: 5,
  align: (col, row) => (right,left,left,left,left,).at(col),
  inset: 6pt,
  [k], [ratio], [w \= 0], [w \= 0.1], [w \= 1],
  [20],
  [j4 / j16],
  [#strong[1.220] \[1.186, 1.256\]],
  [1.220 \[1.186, 1.255\]],
  [1.219 \[1.186, 1.253\]],
  [20],
  [j8 / j16],
  [1.008 \[0.967, 1.051\]],
  [1.008 \[0.967, 1.051\]],
  [1.008 \[0.968, 1.050\]],
  [20],
  [j32 / j16],
  [1.051 \[0.996, 1.110\]],
  [1.051 \[0.996, 1.110\]],
  [1.050 \[0.996, 1.108\]],
  [20],
  [j2 / j16],
  [3.252 \[3.055, 3.463\]],
  [3.259 \[3.061, 3.471\]],
  [3.323 \[3.120, 3.539\]],
  [20],
  [j16 / rho-neg],
  [1.358 \[1.232, 1.497\]],
  [1.269 \[1.149, 1.401\]],
  [0.804 \[0.722, 0.896\] \*],
  [20],
  [j4 / rho-neg],
  [1.657 \[1.522, 1.805\]],
  [1.548 \[1.419, 1.690\]],
  [0.981 \[0.890, 1.081\] \*],
  [24],
  [j4 / j16],
  [#strong[1.268] \[1.193, 1.348\]],
  [1.268 \[1.193, 1.348\]],
  [1.268 \[1.192, 1.349\]],
  [24],
  [j8 / j16],
  [1.050 \[1.005, 1.097\]],
  [1.050 \[1.005, 1.097\]],
  [1.050 \[1.005, 1.097\]],
  [24],
  [j32 / j16],
  [1.014 \[0.978, 1.051\]],
  [1.014 \[0.978, 1.051\]],
  [1.014 \[0.978, 1.051\]],
  [24],
  [j2 / j16],
  [3.458 \[3.331, 3.590\]],
  [3.470 \[3.342, 3.603\]],
  [3.578 \[3.445, 3.717\]],
  [24],
  [j16 / rho-neg],
  [1.343 \[1.219, 1.479\]],
  [1.233 \[1.118, 1.359\]],
  [0.712 \[0.643, 0.787\] \*],
  [24],
  [j4 / rho-neg],
  [1.703 \[1.566, 1.852\]],
  [1.564 \[1.437, 1.701\]],
  [0.903 \[0.827, 0.985\] \*],
)
]

\* The committed band prices rho-neg’s per-step canonicalisation and not the walk arms’ per-step table inserts (the asymmetry found in `research/curve25519_f1_targetwalk_20261108` section 5). With `--price-table-work` (PR \#3700’s opt-in reading; `data/band-tw-*.txt`, and `ratios_table_work_priced` in `data/pooled_measured.json`) j16 / rho-neg is 1.358 / 1.351 / 1.316 at k \= 20 and 1.343 / 1.338 / 1.317 at k \= 24 at w \= 0 / 0.1 / 1, and j4 / rho-neg is 1.657 / 1.659 / 1.667 and 1.703 / 1.702 / 1.695, so rho-neg is ahead at every weight under that reading \[pcc:unverified\]. ecbench’s own committed band verdict for rho-neg against every twcj arm is "unresolved inside the band" in every session (`data/band-k*.txt`), so no rank between them is admitted under the committed reading \[pcc:unverified\].

=== Phases and walk diagnostics (mean over 512 measured runs per arm, units of sqrt(r))
<phases-and-walk-diagnostics-mean-over-512-measured-runs-per-arm-units-of-sqrtr>
#align(center)[#table(
  columns: 11,
  align: (col, row) => (right,left,right,right,right,right,right,right,left,right,right,).at(col),
  inset: 6pt,
  [k], [arm], [fixed], [set-up], [walk], [collision verify], [probes], [total], [walk steps / sqrt(r) (sd)], [collisions per run], [cap reached below rank 2],
  [20],
  [twcj-j2],
  [0.125],
  [0.237],
  [4.430],
  [1.936],
  [0.218],
  [6.946],
  [4.43 (0.36)],
  [15.19],
  [93.6 %],
  [20],
  [twcj-j4],
  [0.125],
  [0.256],
  [1.860],
  [0.365],
  [0.000],
  [2.606],
  [1.86 (0.66)],
  [2.93],
  [0.2 %],
  [20],
  [twcj-j8],
  [0.125],
  [0.293],
  [1.464],
  [0.271],
  [0.000],
  [2.153],
  [1.46 (0.52)],
  [2.19],
  [0],
  [20],
  [twcj-j16],
  [0.125],
  [0.368],
  [1.386],
  [0.256],
  [0.000],
  [2.136],
  [1.39 (0.49)],
  [2.08],
  [0],
  [20],
  [twcj-j32],
  [0.125],
  [0.518],
  [1.352],
  [0.250],
  [0.000],
  [2.245],
  [1.35 (0.49)],
  [2.03],
  [0],
  [24],
  [twcj-j2],
  [0.040],
  [0.073],
  [4.474],
  [0.673],
  [0.216],
  [5.475],
  [4.47 (0.21)],
  [17.00],
  [98.1 %],
  [24],
  [twcj-j4],
  [0.040],
  [0.078],
  [1.775],
  [0.114],
  [0.001],
  [2.008],
  [1.77 (0.70)],
  [2.97],
  [0.4 %],
  [24],
  [twcj-j8],
  [0.040],
  [0.089],
  [1.450],
  [0.085],
  [0.000],
  [1.663],
  [1.45 (0.51)],
  [2.19],
  [0],
  [24],
  [twcj-j16],
  [0.040],
  [0.110],
  [1.355],
  [0.079],
  [0.000],
  [1.583],
  [1.36 (0.47)],
  [2.05],
  [0],
  [24],
  [twcj-j32],
  [0.040],
  [0.152],
  [1.335],
  [0.079],
  [0.000],
  [1.606],
  [1.34 (0.47)],
  [2.03],
  [0],
)
]

The set-up follows the frozen model to within 5 % (measured 85.8 / 92.6 / 106.2 / 133.4 / 187.8 GAE for 2 / 4 / 8 / 16 / 32 jumps at k \= 20, about 3.5 GAE per jump plus 80 fixed) \[pcc:unverified\]. The walk does not: every reduction in jumps lengthens it, and every collision beyond the second is a dependent row \[pcc:unverified\].

== 3. Predictions (PROTOCOL.md section 5)
<predictions-protocol.md-section-5>
#align(center)[#table(
  columns: 4,
  align: (col, row) => (left,left,left,left,).at(col),
  inset: 6pt,
  [id], [prediction], [outcome], [status],
  [P1 (primary)],
  [k\=20 j4/j16 upper end \< 1 at w \= 0, 0.1, 1],
  [1.256, 1.255, 1.253],
  [#strong[failed] \[pcc:unverified\]],
  [P1b],
  [k\=20 j4/j16 point in \[0.92, 0.98\]],
  [1.220],
  [#strong[failed] \[pcc:unverified\]],
  [P2],
  [k\=20 j8/j16 point in \[0.93, 0.99\]],
  [1.008],
  [#strong[failed] \[pcc:unverified\]],
  [P3],
  [j4, j8 walk steps within 3 % of j16, both sizes],
  [j4 +34 % / +31 %, j8 +5.6 % / +7.0 %],
  [#strong[failed] \[pcc:unverified\]],
  [P4],
  [k\=20 j32/j16 lower end \> 1],
  [0.996],
  [#strong[failed] (by 0.004) \[pcc:unverified\]],
  [P5],
  [j2/j16 lower end \> 1.5 and cap below rank 2 in \>\= 90 % of runs, both sizes],
  [3.055 / 3.331; 93.6 % / 98.1 %],
  [#strong[held] \[pcc:unverified\]],
  [P6],
  [k\=24 j4/j16 point in \[0.96, 1.00\]],
  [1.268],
  [#strong[failed] \[pcc:unverified\]],
  [P7],
  [j4/rho-neg lower end \> 1 at w \= 0 and, inserts priced, at w \= 0, 0.1, 1, both sizes],
  [1.522, 1.520, 1.512; 1.566, 1.564, 1.552],
  [#strong[held] \[pcc:unverified\]],
  [P7b],
  [k\=20 j4/rho-neg point in \[1.25, 1.40\]],
  [1.657],
  [#strong[failed] \[pcc:unverified\]],
)
]

Two of nine held. The failed predictions stay failed; nothing was re-scored.

== 4. Why the model failed (exploratory, derived from the counters, not frozen)
<why-the-model-failed-exploratory-derived-from-the-counters-not-frozen>
The model assumed that with m \>\= 4 jumps the collision count stays n^2/r and that every collision is an independent row. The counters show two effects it missed \[pcc:unverified\]:

- #strong[Dependent re-collisions.] After a collision E\_t \= E\_s the walk continues from a point it has already stored, and because the jump index is drawn independently of the point, the next steps from E\_t repeat the stored path from E\_s whenever the two short jump sequences have the same count vector. With m jumps that happens with probability about 1/m per step for a one-step repeat (and more for longer matches), and every such re-collision is the earlier row again, so rank 2 needs a third, fourth, … collision. Measured collisions per run: 2.93 (m \= 4), 2.19 (m \= 8), 2.08 (m \= 16), 2.03 (m \= 32), against the two that rank 2 needs \[pcc:unverified\].
- #strong[A slower approach to uniformity.] Even the first independent row arrives later with fewer jumps (the per-collision rate falls below 2/r because the difference vectors of nearby positions repeat); the walk-step means 1.86 / 1.46 / 1.39 / 1.35 sqrt(r) for 4 / 8 / 16 / 32 jumps at k \= 20 fall monotonically toward the 1.329 floor \[pcc:unverified\].

Both effects favour more jumps. At these sizes each jump costs about 3.5 GAE of set-up, which cancels the gain already at 32 jumps (walk 0.034 sqrt(r) shorter, set-up 0.150 sqrt(r) dearer at k \= 20). At k \= 28 and 32 the set-up is 0.01 sqrt(r) or less, which is the follow-on measured in `research/curve25519_tw_scaling_20261108` \[pcc:unverified\].

== 5. Distance to matched rho (rho-neg, same sessions)
<distance-to-matched-rho-rho-neg-same-sessions>
#align(center)[#table(
  columns: 5,
  align: (col, row) => (right,left,right,left,left,).at(col),
  inset: 6pt,
  [k], [arm], [w \= 0], [w \= 0.1 (committed / inserts priced)], [w \= 1 (committed / inserts priced)],
  [20],
  [twcj-j16 (current best)],
  [1.36x \[1.23, 1.50\]],
  [1.27x / 1.35x],
  [0.80x (artifact) / 1.32x],
  [20],
  [twcj-j32],
  [1.43x],
  [1.33x / 1.41x],
  [0.85x (artifact) / 1.34x],
  [24],
  [twcj-j16],
  [1.34x \[1.22, 1.48\]],
  [1.23x / 1.34x],
  [0.71x (artifact) / 1.32x],
  [24],
  [twcj-j32],
  [1.36x],
  [1.25x / 1.35x],
  [0.72x (artifact) / 1.32x],
)
]

No arm of this study moves the distance remaining; the 16-jump arm stays the best, at 1.34x to 1.36x rho-neg at w \= 0, with the earlier studies’ 1.40x and 1.41x inside these intervals \[pcc:unverified\]. Rho-neg’s own set-up (0.376 sqrt(r) at k \= 20, 0.098 at k \= 24) is part of the reference, as in the earlier studies.

== 6. Scope and limits
<scope-and-limits>
- The F \= 1 target-in-walk arm is one unknown away from rho: with P1 :\= G it is a random walk on \[c\]G + \[b\]Q whose first collision gives log Q (rho with a stored walk). It does no decomposition work and is not decomposition-based index calculus; no leaderboard or `benchmarks/README.md` row is proposed, and none could be added without the validation gate (no VALIDATION.json, no replay certificate, no validator report) \[pcc:unverified\].
- Single target only; the walk is built from the target.
- Local L0 counted units on a shared host; deterministic and replayed; no wall-clock claim.
- No A/A arm (ecbench seeds runs by workload and round; an A/A arm replicates rho-neg exactly, as the earlier studies found). Noise is carried by the between-session t intervals.
- research-kb and ecdb were not reachable from this machine; no novelty claim is made.

== Files
<files>
`PROTOCOL.md`, `specs/`, `data/frozen.sha256`, `data/frozen-at.txt`, `scripts/` (`model.py`, `make_specs.py`, `run_sessions.sh`, `pooled_compare.py`, `analyse.py`, `make_figure.py`, `render_pdf.py`), `sessions/k*-s*/{session,spec,plan,host}.json` (the `records.jsonl` files are in `data/sessions-k{20,24}-records.tar.gz`, hashed in the benchmark records), `data/` (model, plans, run logs, both band readings, verify reports, `pooled_measured.json`, binary hashes), `figures/jumps.{svg,png}`, `REPORT.pdf`. Benchmark records: `benchmarks/records/20261108T2045Z-curve25519-tw-jumps-k20.json`, `benchmarks/records/20261108T2047Z-curve25519-tw-jumps-k24.json`; figure and report copies in `benchmarks/20261108-curve25519-tw-jumps/`.
