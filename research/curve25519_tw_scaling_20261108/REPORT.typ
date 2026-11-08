#set page(paper: "a4", margin: 1.8cm, numbering: "1")
#set text(size: 9.5pt)
#set table(inset: 4pt, stroke: 0.4pt + gray)
#show table: set text(size: 7.5pt)
#show figure: set block(breakable: true)
= The F \= 1 target-in-walk arm at k \= 28 and 32, single-target ecbench (2026-11-08)
<the-f-1-target-in-walk-arm-at-k-28-and-32-single-target-ecbench-2026-11-08>
#strong[Class.] Measured, single-target ecbench sessions under a protocol frozen before data (#link("PROTOCOL.md"); hashes in `data/frozen.sha256`, freeze commit `e6a759cc72`). Counted units only (ecbench GAE, S \= cost/sqrt(r)); every comparison at w \= 0, 0.1 and 1 under both band readings: the pre-2026-11-08 reading (table inserts unpriced; this README’s earlier "committed band") and the priced reading (`ecbench band --price-table-work`, the default since the owner ruling of 2026-11-08, landing in PR \#3726). Public synthetic Curve25519-family miniatures; no security claim. No pcc claim has been filed; every number is an unverified assertion until it is reviewed \[pcc:unverified\]. No code change; binary `/home/user/ctarget-f1/release/ecbench` (SHA-256 `d23fab99...`, `data/binary-k{28,32}.sha256`).

#strong[Question.] The target-in-walk arm had been measured at k \= 20 and 24 only. Does its distance to rho-neg hold near 1.4x at k \= 28 and 32, where rho-neg’s set-up overhead vanishes and both arms sit near their walk floors, and do 32 or 64 jumps pay there, where the set-up they cost is 0.01 sqrt(r) or less?

#strong[Answer.] The distance holds. The 16-jump arm costs 1.395x \[1.345, 1.446\] rho-neg at k \= 28 and 1.366x \[1.267, 1.473\] at k \= 32 at w \= 0, and its lower end stays above 1 at every weight under the priced reading (table inserts priced like rho’s canonicalisations); under the pre-2026-11-08 reading the verdict against rho-neg is unresolved in all 16 sessions and the order flips at w \= 1 \[pcc:unverified\]. Against the table-then-probe F \= 1 arm it costs 0.906x \[0.877, 0.935\] and 0.857x \[0.811, 0.905\] \[pcc:unverified\]. Its mean S at k \= 32 is 1.387 with a walk of 1.368 sqrt(r), 3 % above the second-collision floor of 1.329; rho-neg’s search is 0.999 sqrt(r) there, 13 % above its first-collision floor of 0.886, which is why the measured ratio is 1.37x (frozen model 1.38x at k \= 28, 1.375x at k \= 32; data within 1.1 %) and not the floor ratio 1.5 \[pcc:unverified\]. More jumps do not pay: 32 jumps give 0.997x \[0.954, 1.041\] at k \= 28 and 0.985x \[0.930, 1.044\] at k \= 32, 64 jumps 0.998x and 0.990x, all intervals containing 1 \[pcc:unverified\]. Six of seven frozen predictions held; the only failure is the risky P4b (32 jumps strictly below 16 at k \= 32) \[pcc:unverified\]. "Rho-neg is ahead at every weight" holds under the priced reading only \[pcc:unverified\].

#figure([#image("figures/scaling.png", width: 100%)],
  caption: [
    Scaling of the distance to rho
  ]
)

== 1. Design (as frozen)
<design-as-frozen>
8 sessions per size, `targets_per_curve = 1`, 64 measured rounds plus 1 warm-up per arm, `order = alternate`; arms `rho-neg`, `f1-x07` (baseline, `wwalk:x=0.7,g=1,grow=2,seed=run`), `twcj-j16` (primary), `twcj-j32`, `twcj-j64` (`wwalk:x=1.5,g=1,grow=2,tw=1,cj=1,jumps=J,seed=run`), all pipeline arms on `prime-projected-abscissa:size=1,scan_cap=32` with `targets=stride-generator`. Curves: the family miniatures of the autolab holdouts at k \= 28 (`r = 33,557,809`) and k \= 32 (`r = 536,884,639`). Audit: `ecbench verify --replay 1` per session. Pooling as in the sibling studies. Runtime: 50 s at k \= 28, 76 s at k \= 32 (`data/run-*.log`).

== 2. Results (measured)
<results-measured>
All 16 sessions completed with 325 of 325 records verified each; every audit reproduced its replay with 0 problems (`data/verify-k*-s*.txt`) \[pcc:unverified\]. Pooled figures use the 2,560 measured records per size (`data/pooled_measured.json`).

Mean S per arm (pre-2026-11-08 reading, inserts unpriced; the last two columns price the table inserts at w):

#align(center)[#table(
  columns: 7,
  align: (col, row) => (right,left,right,right,right,right,right,).at(col),
  inset: 6pt,
  [k], [arm], [S w\=0], [S w\=0.1 (unpriced)], [S w\=1 (unpriced)], [w\=0.1, inserts priced], [w\=1, inserts priced],
  [28],
  [rho-neg],
  [1.029],
  [1.125],
  [1.993],
  [1.125],
  [1.993],
  [28],
  [f1-x07],
  [1.584],
  [1.639],
  [2.131],
  [1.735],
  [3.100],
  [28],
  [twcj-j16],
  [1.435],
  [1.435],
  [1.438],
  [1.572],
  [2.805],
  [28],
  [twcj-j32],
  [1.430],
  [1.430],
  [1.433],
  [1.565],
  [2.784],
  [28],
  [twcj-j64],
  [1.432],
  [1.433],
  [1.436],
  [1.566],
  [2.765],
  [32],
  [rho-neg],
  [1.015],
  [1.115],
  [2.014],
  [1.115],
  [2.014],
  [32],
  [f1-x07],
  [1.619],
  [1.677],
  [2.201],
  [1.779],
  [3.218],
  [32],
  [twcj-j16],
  [1.387],
  [1.387],
  [1.388],
  [1.524],
  [2.756],
  [32],
  [twcj-j32],
  [1.367],
  [1.367],
  [1.368],
  [1.501],
  [2.711],
  [32],
  [twcj-j64],
  [1.373],
  [1.373],
  [1.374],
  [1.507],
  [2.717],
)
]

Pooled ratios with 95 % intervals over 8 single-target sessions:

#align(center)[#table(
  columns: 5,
  align: (col, row) => (right,left,left,left,left,).at(col),
  inset: 6pt,
  [k], [ratio], [w \= 0], [w \= 0.1 (pre-2026-11-08 / inserts priced)], [w \= 1 (pre-2026-11-08 / inserts priced)],
  [28],
  [twcj-j16 / rho-neg],
  [#strong[1.395] \[1.345, 1.446\]],
  [1.275 \[1.230, 1.322\] / 1.397 \[1.347, 1.448\]],
  [0.722 \[0.695, 0.749\] \* / 1.407 \[1.356, 1.461\]],
  [28],
  [f1-x07 / rho-neg],
  [1.540 \[1.484, 1.598\]],
  [1.456 / 1.542],
  [1.070 \* / 1.556],
  [28],
  [twcj-j16 / f1-x07],
  [#strong[0.906] \[0.877, 0.935\]],
  [0.876 / 0.906],
  [0.675 \* / 0.905],
  [28],
  [twcj-j32 / twcj-j16],
  [0.997 \[0.954, 1.041\]],
  [0.997 / 0.996],
  [0.997 / 0.993],
  [28],
  [twcj-j64 / twcj-j16],
  [0.998 \[0.947, 1.053\]],
  [0.998 / 0.996],
  [0.998 / 0.986],
  [32],
  [twcj-j16 / rho-neg],
  [#strong[1.366] \[1.267, 1.473\]],
  [1.244 \[1.154, 1.341\] / 1.367 \[1.267, 1.473\]],
  [0.689 \[0.639, 0.743\] \* / 1.368 \[1.268, 1.476\]],
  [32],
  [f1-x07 / rho-neg],
  [1.595 \[1.508, 1.687\]],
  [1.504 / 1.595],
  [1.093 \* / 1.598],
  [32],
  [twcj-j16 / f1-x07],
  [#strong[0.857] \[0.811, 0.905\]],
  [0.827 / 0.857],
  [0.631 \* / 0.856],
  [32],
  [twcj-j32 / twcj-j16],
  [0.985 \[0.930, 1.044\]],
  [0.985 / 0.985],
  [0.985 / 0.984],
  [32],
  [twcj-j64 / twcj-j16],
  [0.990 \[0.934, 1.049\]],
  [0.990 / 0.989],
  [0.990 / 0.986],
)
]

\* The pre-2026-11-08 reading prices rho-neg’s per-step canonicalisation and not the walk arms’ per-step table inserts. Its ecbench verdict (`data/band-k*.txt`) for rho-neg against twcj-j16, j32 and j64 is "unresolved inside the band" in all 16 sessions each, so no rank is admitted against the twcj arms under that reading; against f1-x07 it is "rho-neg \< f1-x07" in 15 of 16 sessions and unresolved in 1 (the f1-x07 inserts are priced at w \= 1 through the probe lookups, so its asymmetry is smaller) \[pcc:unverified\]. Under the priced reading (`--price-table-work`, `data/band-tw-k*.txt`) rho-neg is ahead of every arm at every weight in 16 of 16 sessions \[pcc:unverified\]. That statement holds under the priced reading only \[pcc:unverified\].

=== Phases and walk diagnostics (mean over 512 measured runs per arm, units of sqrt(r))
<phases-and-walk-diagnostics-mean-over-512-measured-runs-per-arm-units-of-sqrtr>
#align(center)[#table(
  columns: 11,
  align: (col, row) => (right,left,right,right,right,right,right,right,left,right,right,).at(col),
  inset: 6pt,
  [k], [arm], [fixed], [set-up], [walk], [collision verify], [probes], [total], [walk steps / sqrt(r) (sd)], [collisions per run], [set-up GAE],
  [28],
  [rho-neg],
  [-],
  [0.059],
  [0.964],
  [0.006],
  [-],
  [1.029],
  [],
  [],
  [],
  [28],
  [f1-x07],
  [0.012],
  [0.055 (incl. verify)],
  [0.969],
  [],
  [0.548],
  [1.584],
  [],
  [],
  [],
  [28],
  [twcj-j16],
  [0.012],
  [0.032],
  [1.367],
  [0.024],
  [0],
  [1.435],
  [1.37 (0.48)],
  [2.06],
  [186],
  [28],
  [twcj-j32],
  [0.012],
  [0.044],
  [1.351],
  [0.023],
  [0],
  [1.430],
  [1.35 (0.47)],
  [2.03],
  [255],
  [28],
  [twcj-j64],
  [0.012],
  [0.068],
  [1.329],
  [0.023],
  [0],
  [1.432],
  [1.33 (0.49)],
  [2.01],
  [393],
  [32],
  [rho-neg],
  [-],
  [0.015],
  [0.999],
  [0.002],
  [-],
  [1.015],
  [],
  [],
  [],
  [32],
  [f1-x07],
  [0.004],
  [0.016 (incl. verify)],
  [1.017],
  [],
  [0.582],
  [1.619],
  [],
  [],
  [],
  [32],
  [twcj-j16],
  [0.004],
  [0.009],
  [1.368],
  [0.007],
  [0],
  [1.387],
  [1.37 (0.52)],
  [2.06],
  [212],
  [32],
  [twcj-j32],
  [0.004],
  [0.012],
  [1.344],
  [0.007],
  [0],
  [1.367],
  [1.34 (0.48)],
  [2.04],
  [290],
  [32],
  [twcj-j64],
  [0.004],
  [0.019],
  [1.343],
  [0.007],
  [0],
  [1.373],
  [1.34 (0.49)],
  [2.01],
  [443],
)
]

Every twcj record at every jump count solved at rank 2 with zero probe hits on the first trial (3,072 of 3,072) \[pcc:unverified\]. The set-up is above the frozen model (`data/model.json`): 178.0 GAE at k \= 28 and 203.0 at k \= 32 for 16 jumps against measured 185.8 and 212.5 (+4.4 %, +4.7 %); the measured set-up cost per extra jump is 4.31 GAE at k \= 28 (16 to 32 and 32 to 64 jumps) and 4.82 / 4.79 GAE at k \= 32, against 4.5 and 5.0 in the model (the 3.5 to 3.8 GAE of the k \= 20 / 24 study does not carry over, and PROTOCOL.md’s "about 3.8 GAE per jump" disagrees with its own model.json) \[pcc:unverified\].

== 3. Predictions (PROTOCOL.md section 5)
<predictions-protocol.md-section-5>
#align(center)[#table(
  columns: 4,
  align: (col, row) => (left,left,left,left,).at(col),
  inset: 6pt,
  [id], [prediction], [outcome], [status],
  [P1],
  [twcj-j16/rho-neg point in \[1.30, 1.50\] at w \= 0, lower end \> 1 at w \= 0 and (inserts priced) at every weight, both sizes],
  [1.395, 1.366; lower ends 1.345 / 1.347 / 1.356 and 1.267 / 1.267 / 1.268],
  [#strong[held] \[pcc:unverified\]],
  [P2],
  [twcj-j16/f1-x07 upper end \< 1, point in \[0.82, 0.93\], both sizes],
  [0.906 \[0.877, 0.935\], 0.857 \[0.811, 0.905\]],
  [#strong[held] \[pcc:unverified\]],
  [P3],
  [j16 walk steps in \[1.30, 1.42\] both sizes; mean S at k \= 32 in \[1.36, 1.50\]],
  [1.367, 1.368; 1.387],
  [#strong[held] \[pcc:unverified\]],
  [P4],
  [j32/j16 point in \[0.95, 1.00\] both sizes],
  [0.997, 0.985],
  [#strong[held] \[pcc:unverified\]],
  [P4b],
  [k \= 32 j32/j16 upper end \< 1],
  [1.044],
  [#strong[failed] \[pcc:unverified\]],
  [P5],
  [j64/j16 point in \[0.95, 1.01\] both sizes],
  [0.998, 0.990],
  [#strong[held] \[pcc:unverified\]],
  [P6],
  [\>\= 99.5 % of twcj records at rank 2, no probe, first trial],
  [100 % at every jump count],
  [#strong[held] \[pcc:unverified\]],
)
]

== 4. Reading
<reading>
- #strong[The distance to rho is flat in k.] With the sibling studies’ k \= 20 and 24 points (1.358 and 1.343 on the same arm, `research/curve25519_tw_jumps_20261108`), the 16-jump arm is 1.34x to 1.40x rho-neg at w \= 0 from k \= 20 to 32, and the table-then-probe arm 1.54x to 1.60x \[pcc:unverified\]. Neither moves toward parity with size.
- #strong[Why 1.37x and not 1.5x.] The derived F \= 1 floor ratio 1.5 \= 1.329 / 0.886 compares the two walks’ model floors; it is not the frozen model, which predicted 1.38 (k \= 28) and 1.375 (k \= 32) for twcj-j16 / rho-neg, and the data (1.395, 1.366) agree with it within 1.1 % \[pcc:unverified\]. Measured, the twcj walk runs 3 % above 1.329 at every size, while rho-neg’s search runs 9 % to 13 % above 0.886 (0.964 and 0.999 sqrt(r) at k \= 28, 32; its look-ahead and cycle escapes are counted) \[pcc:unverified\]. The ratio of the measured walks is 1.37 to 1.42, and the remaining set-up and verification (0.03 to 0.07 sqrt(r) at these sizes) add little. A better rho-neg walk (closer to its own floor) would widen the gap; nothing here changes the reference.
- #strong[Jumps.] The walk shortens with more jumps (1.367 to 1.344 to 1.343 sqrt(r) at k \= 32), by about the amount the 20- and 24-bit study saw, but the gain of 0.02 sqrt(r) is inside the session noise (interval half-width 0.05) and is not established \[pcc:unverified\]. Sixteen jumps remains the default; 32 is not worse at k \>\= 28.

== 5. Distance to matched rho (rho-neg, same sessions)
<distance-to-matched-rho-rho-neg-same-sessions>
#align(center)[#table(
  columns: 5,
  align: (col, row) => (right,left,right,left,left,).at(col),
  inset: 6pt,
  [k], [arm], [w \= 0], [w \= 0.1 (pre-2026-11-08 / inserts priced)], [w \= 1 (pre-2026-11-08 / inserts priced)],
  [28],
  [twcj-j16],
  [1.40x \[1.35, 1.45\]],
  [1.28x / 1.40x],
  [0.72x (artifact) / 1.41x],
  [28],
  [f1-x07],
  [1.54x],
  [1.46x / 1.54x],
  [1.07x / 1.56x],
  [32],
  [twcj-j16],
  [1.37x \[1.27, 1.47\]],
  [1.24x / 1.37x],
  [0.69x (artifact) / 1.37x],
  [32],
  [twcj-j32],
  [1.35x \[1.25, 1.45\]],
  [1.23x / 1.35x],
  [0.68x (artifact) / 1.35x],
  [32],
  [f1-x07],
  [1.60x],
  [1.50x / 1.60x],
  [1.09x / 1.60x],
)
]

=== Unpriced counters per arm (AGENTS.md section 5a listing)
<unpriced-counters-per-arm-agents.md-section-5a-listing>
Mean per measured run, in units of sqrt(r), k \= 28 / k \= 32 (validator’s recomputation from the records) \[pcc:unverified\]. U is the sum of the counters in `cost.unpriced`; the last column is `work_native.table_insert_attempts`, counted but priced only by the priced reading.

#align(center)[#table(
  columns: 4,
  align: (col, row) => (left,left,left,left,).at(col),
  inset: 6pt,
  [arm], [U (cost.unpriced)], [components], [table inserts (counted, priced only in the priced reading)],
  [rho-neg],
  [0.964 / 0.999],
  [`canonicalisations_uncharged` only],
  [none],
  [f1-x07],
  [0.547 / 0.582],
  [`lookups_uncharged` 0.545 / 0.582, row\_ops and legendre\_symbols about 0.001],
  [0.969 / 1.017],
  [twcj-j16],
  [0.003 / 0.001],
  [row\_ops, legendre\_symbols],
  [1.367 / 1.368],
  [twcj-j32],
  [0.003 / 0.001],
  [row\_ops, legendre\_symbols],
  [1.351 / 1.344],
  [twcj-j64],
  [0.003 / 0.001],
  [row\_ops, legendre\_symbols],
  [1.329 / 1.343],
)
]

Each is priced at 0, 0.1 and 1 GAE per operation in the band files, and two arms are ranked only where the order holds across that band \[pcc:unverified\].

== 6. Scope and limits
<scope-and-limits>
As in the sibling studies: the F \= 1 target-in-walk arm is one unknown away from rho (P1 :\= G makes it a rho walk with a stored table), does no decomposition work, is not decomposition-based index calculus, and proposes no leaderboard or `benchmarks/README.md` row; no VALIDATION.json or replay certificate exists (a validator report now does, `validator-report.json`) \[pcc:unverified\]. Single target only. Local L0 counted units, deterministic and replayed; no wall-clock claim. No A/A arm (it would replicate rho-neg exactly). research-kb and ecdb were not reachable; no novelty claim.

== Validation (independent validator, 2026-11-08; status pass\_with\_errata)
<validation-independent-validator-2026-11-08-status-pass_with_errata>
Source: `validator-report.json` in this directory (not edited). Each erratum and how it was applied here \[pcc:unverified\]:

- #strong[E1] (count): "five of seven" corrected to six of seven held, P4b the only failure, in the Answer; the coverage report carries the same correction \[pcc:unverified\].
- #strong[E2] (band-verdict footnote): replaced by "unresolved against every twcj arm in all 16 sessions; against f1-x07 rho-neg is ahead in 15 of 16 and unresolved in 1", with the pre-2026-11-08 label on the reading \[pcc:unverified\].
- #strong[E3] (5a listing): the table of every arm’s unpriced counters per run was added to section 5 \[pcc:unverified\].
- #strong[E4] (set-up numbers and wording): 178 / 203 GAE (model) against 185.8 / 212.5 (measured); 4.3 (k \= 28) and 4.8 (k \= 32) GAE per extra jump against 4.5 and 5.0 in the model; the 1.37x is compared with the frozen model 1.38 / 1.375 and not with the floor ratio 1.5 \[pcc:unverified\]. PROTOCOL.md’s "about 3.8 GAE per jump" is not edited (frozen) \[pcc:unverified\].
- #strong[E5] (record hygiene): disclosed below; closed records are not edited and the raw archives are not uploaded by this commit \[pcc:unverified\].
- Also noted, from check 9: the "distance is flat in k" statement rests on four points (1.358, 1.343, 1.395, 1.366) with a least-squares slope of ln ratio on k of +0.0013 per bit (+1.6 % over 12 bits, inside the intervals); no fit was printed in the original README \[pcc:unverified\].

Disclosures \[pcc:unverified\]:

- The measuring binary `d23fab99...` (`/home/user/ctarget-f1/release/ecbench`) was rebuilt on disk at 2026-11-08T21:08:01Z during validation (now `fcb16d9c...`). The validator’s replays used `fcb16d9c` and reproduced every count in all 16 sessions (`audit OK: 325 records, 320 replays   reproduced, 0 problems`), so the replays establish behavioural identity only, not byte identity of source or binary \[pcc:unverified\].
- The benchrec records `20261108T2051Z-...-k28` and `-k32` store 4-decimal metric values that differ from the validator’s recomputation by at most 0.04 % (the 3-decimal README values agree), for example k \= 28 twcj / rho 1.3951 against 1.39458; the difference comes from rounding in the producer’s pipeline. All raw entries have `uploaded: false` (raw archives not uploaded to S3), and `session.json` `git_commit` and `git_dirty` are null \[pcc:unverified\].
- For its replays the validator exported `CHECKPOINT_SKIP` as well as `BENCHREC_SKIP`; no ledger bypass entry resulted \[pcc:unverified\].
- Validation gate: validator report only; records have `validation: null` and no `VALIDATION.json` exists, so no leaderboard or `benchmarks/README.md` row follows \[pcc:unverified\].

== Files
<files>
`PROTOCOL.md`, `specs/`, `data/frozen.sha256`, `data/frozen-at.txt`, `scripts/` (`model.py`, `make_specs.py`, `run_sessions.sh`, `pooled_compare.py`, `analyse.py`, `make_figure.py`, `render_pdf.py`), `sessions/k*-s*/{session,spec,plan,host}.json` (`records.jsonl` in `data/sessions-k{28,32}-records.tar.gz`, hashed in the benchmark records), `data/` (model, plans, run logs, both band readings, verify reports, `pooled_measured.json`, binary hashes), `figures/scaling.{svg,png}`, `REPORT.pdf`. Benchmark records: `benchmarks/records/20261108T2051Z-curve25519-tw-scaling-k{28,32}.json`; copies in `benchmarks/20261108-curve25519-tw-scaling/`.
