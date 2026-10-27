# Native N83 indexed XOR propagation and natural continuation

The native indexed-word optimization passed differential and full library
validation. Seven dense A and seven indexed B controls independently verified
actual selector models and original group sums, with identical search counters.
Five matched dense/indexed solve-wall ratios are **12.1636332262, 9.99446883302,
9.95866828830, 10.1695267031, 10.1932494096**. These describe this exact known-witness
SAT component under the recorded conditions; they are not full-cold IC gains.

The study uses the public synthetic a=0 curve
`icv1-f2m83-tm6151469093347-debefd74`, polynomial
x^83+x^45+x^2+x+1 and subgroup order2417851639230796216685689. It is distinct
from the positive-trace a=1 N83 cold leaderboard holdout. EC1 mapping remains
unresolved; no new identity or registry equivalence is invented.

## Recorded results

| arm / scope | n | recorded milliseconds | status |
|---|---:|---|---|
| Dense native solve wall | 7 | median182074.425421; range177459.286074–216393.804510 | all actual models/domain/group sums verified |
| Indexed native solve wall | 7 | median17767.144183; range17731.849428–17862.255509 | all actual models/domain/group sums verified |
| CryptoMiniSat5.17.0 solve/simplify CPU | 7 | median2830; range2810–2840 | all original exported constraints and original-group models independently verified |
| K2048 natural,1000 conflicts | 1 | solver4861.868809; outer10319.216838 | zero models/relations/rank; bounded unknown, native replay PASS |
| K4096 natural,1000 conflicts | 1 | solver7814.674415; outer18151.190391 | zero models/relations/rank; bounded unknown, native replay PASS |
| K2048 natural,10000 conflicts | 1 | solver42413.639201; outer47750.513728 | zero models/relations/rank; bounded unknown, native replay PASS |
| K4096 natural,10000 conflicts | 1 | solver50433.062636; outer60818.306138 | zero models/relations/rank; bounded unknown, native replay PASS |

Natural solver intervals are derived in Rust as `solver_return.elapsed_ms -
solver_enter.elapsed_ms`. The earlier progress update's5.15/8.39seconds were
phase-return timestamps; the exclusive solver intervals are4.862/7.815seconds.
No comparison with unreserved earlier-host natural runs is inferred.

Every native witness solve uses787conflicts,35307decisions,5restarts,
36757XORpasses,94879418repivots,3493683row operations,87094507row scans and
5153715reason literals. Original model representative indices are
401/780/1059/193/62, all phase0, from full1182/866domains and eight fixed
coordinate blocks. No selector assumptions were added.

Dense word updates are derived natively as3493683×1589=5551462287; indexed
updates are counted14064862 with15997241support visits. Ratio394.704355222.
These count matrix-word XOR updates, not every instruction or reason scan.
The exact sorted support list removes zero words after cancellation and is
rebuilt after matrix reset/variable growth. Pivot, row and reason order stay
fixed. No architecture-specific implementation or cross-host claim.

**CryptoMiniSat is faster.** Candidate wall / external algorithm CPU ratios
are6.28629953357,6.27411646961,6.34929032028,6.27814282085,6.28952658768.
They explicitly compare different reported timer kinds. Process/parse costs
are separately retained, not charged as CMS algorithm time. CMS uses the
same original714736CNF+4980native-XOR formula, threads1, maxtime360,
maxconfl1000000; official5.17.0 Git03954f7f7815ea3984774c95a66e7c224cd25952.
The complete formula and model are replayed against all original constraints,
selected representatives/phases, fixed coordinates and original group sum.
See the [official release](https://github.com/msoos/cryptominisat/releases/tag/release/v5.17.0).
No fastest-tool or better-exponent claim is made; other hardware/configurations
remain unmeasured.

## Protocol, conditions and preserved failures

- Original frozen [protocol](../../verification/native-xor-indexed-20261019/PROTOCOL.md),
  separate [external-reference correction](../../verification/native-xor-indexed-20261019/REFERENCE-V3.md),
  [quiet-readiness v4](../../verification/native-xor-indexed-20261019/PREFLIGHT-V4.md),
  and separately frozen [natural10K v5](../../verification/native-xor-indexed-20261019/NATURAL-10K-V5.md).
- Baseline source4122fddca6; optimized library6634ce0955. v4card freeze40d554ab57;
  v5freeze2ccbc3da6b. Worker ELF hashes, exact commands, specs and custody are
  in [compact records](../../verification/native-xor-indexed-20261019/records/)
  and the complete raw archive. No measured worker or old driver was overwritten.
- Managed Linux x86-64 Xeon6975P-C,32vCPU/128GiB; CPU15 and sibling31 reserved,
  exclusive native lock, movable user threads evicted with restoration, quiet
  two-second samples before launch. Some fixed kernel threads remain. Docker
  image541096ae9b05…,1CPU quota/4GiB/no extra swap/networknone/read-only
  root/input/source/worker,capdropALL,no-new-privileges,64PIDs. No heavy work
  during timing. All launched workers cleaned up; no OOM.
- A/A and five interleaved ABE/BAE rounds are preserved. A/A wall drift and
  missing worker context-switch/fault accounting preclude an automatic L2 or
  full-cold wall admission. Counts are primary; wall observations descriptive.
- Original180-second profile is wall censored, final counters null. Separate
  one-conflict profile records971224218ns XOR versus977.691845ms solve wall.
  Initial nonzero census108026/7913220words does not bound later fill-in.
- Original external5000 cap returns15/INDETERMINATE at5001reported conflicts;
  its failed EOF audit remains unchanged. v3refuses before worker launch on
  busy sibling preflight. v4only waits for quiet readiness before one launch;
  no timed retry. Unattempted superseded arms remain unattempted.
- Director compiled/validated v4; a usage limit interrupted its assessments.
  Primary performed identical read-only native checks at the frozen HEAD,
  saved `assessment-primary.json`. Director returned for v5compile and assess.
  Mechanical execution was exact and stdout captured verbatim. This is a
  disclosed staffing deviation, not an invented director assessment.

## Validation and deliverables

Full serial release library:4398passed,118ignored,0failed in588.86seconds;
unchanged final library blob. Preserve the earlier managed-runtime interruption
and restarted run separately. Dense/indexed differential tests cover twelve
planted systems each at widths1/63/64/65/127/130, repeated blocking solves,
variable growth and exact supports/search/model comparisons. Export example19,
external model checker2, and final native summary/card tests14+7 pass. No
Python research, harness, replay or calculation path ran. Python only rendered
recorded native results and inspected the PDF.

[Native generated data](figure-data.json), [source of native summary](../../../../tools/n83-sweep-runner/src/xor_summary.rs),
[rendering source](render.py), [Typst source](report.typ), [inspected four-page PDF](report.pdf).
Graphs: [solve clocks](solver-times.svg), [word updates](word-updates.svg),
[natural trials](natural-trials.svg), [failed/diagnostic attempts](diagnostics.svg).
Every graph has a same-source150dpiPNG. The PDF covers every planned/preserved
observation, including failures/censors, and each arm's disposition.
[Benchmark copies](../../../../benchmarks/20261019-n83-xor-indexed/).

| requirement | status / evidence |
|---|---|
| Profile XOR bottleneck | verified; one-conflict diagnostic and retained180s censor |
| Optimize and validate | verified for this component; differential/full tests plus matched models/counts |
| Continue native factor-base sweep | verified1000 and separately10000 conflicts per K2048/K4096; bounded zero yield |
| Raw S3 preservation | see [publication receipt](../../verification/native-xor-indexed-20261019/PUBLICATION.md); complete only after fresh GET/hash checks |
| Full cold winner / useful natural rank | unresolved; no models/relations/rank in these bounds |

The canonical leaderboard HTML/JSON/Markdown includes this engineering component
outside tablesA–C. The render-only path bypasses legacy Python research calculations.
The cold scoreboard, progress timeline, curve registry and rho values remain
unchanged because no full cold pipeline, curve identity or matched-rho result moved.

Next discriminating performance experiment: skip repeated decided-row scans until
backjump, requiring exact support invalidation, unchanged differential search/model
checks, fresh A/A+five interleaved matched rounds and a CMS comparison. It is a
proposed next-round goal, not an implemented or scheduled run. Natural positive
relation/rank/scalar replay and higher-yield workloads remain separate obligations.
