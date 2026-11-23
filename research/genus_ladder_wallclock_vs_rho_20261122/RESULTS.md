# Wall-clock ratio of the 2LP + prefilter + SGE(8,3) arm to negation rho, genus 3 `[pcc:unverified]`

Protocol frozen before data (`PROTOCOL.md`, `frozen.sha256`). Binary: copy of `target/release/genus_ladder` built from
HEAD f6acc4c2e4, `dirty_src: []` (hash in `binary.sha256`, equal to the build marker's). Same instances as R2
(`genus_ladder_g3_highp_20261121`): its first six admitted curves per prime, 2 seeds, 1 thread, host otherwise idle
(`raw/load.log`: load average 1.00 before and after every invocation, i.e. this process only; runs one prime at a
time). Instance search and order computation are timed separately (`order_search_wall_s`) and are not inside any
run's `wall_ns`, so they are excluded for both arms. All figures are single measurements on one shared VM, 4 vCPU,
no PMU. Numbers: `tables.md`, `analysis.json`, figure `fig_ratio.svg`.

## Result

| p | mean wall IC+SGE (s) | mean wall rho (s) | **IC / rho, wall** | 95 % interval (curves, then seeds) | R8 count re-pricing |
|--:|--:|--:|--:|:--|--:|
| 65537 | 2.89 | 14.28 | **0.202** | [0.158, 0.271] | 0.176 |
| 131101 | 8.48 | 32.51 | **0.261** | [0.220, 0.324] | 0.228 |
| 262147 | 22.19 | 118.76 | **0.187** | [0.121, 0.325] | 0.198 |

In end-to-end wall time the arm costs about a fifth to a quarter of negation rho on these instances (a 4 to 5x
advantage, at all three primes), and the measured values sit 10 to 15 % above the R8 count-based re-pricing at 65537
and 131101 and 6 % below it at 262147. The arm without SGE is 0.272 / 0.335 / 0.226 of rho.

## Scorecard

| id | prediction | outcome | verdict |
|---|---|---|---|
| V1 | all runs certify, all count fields equal R2's | 108 of 108 runs certified (36 per prime); 0 differing non-`wall_ns` fields over all 108 runs; instance lines (curve seed, order, r) equal | **pass** |
| V2 | ratio within +-30 % of 0.176 / 0.228 / 0.198 | 0.202 in [0.123, 0.229]; 0.261 in [0.160, 0.296]; 0.187 in [0.139, 0.257] | **pass** (all three; the 131101 point is 14 % above the re-pricing, inside the band) |
| V3 | flat or falling 65537 to 262147 (flat within 15 %) | 0.202 to 0.187, a 7 % fall; 131101 is the highest point (0.261) so the line is not monotone | **pass** (flat/slightly falling; the intervals overlap, no trend is established) |
| V4 | old-binary wall ratio >= 2 x new at 262147 | old 0.500 vs new 0.187, 2.67x | **pass, but confounded** (below) |

## Old-binary cross-check (R2 rows, same 6 curves per prime)

Old-binary ratio IC/rho: 0.499 [0.393, 0.694], 0.682 [0.532, 0.869], 0.500 [0.368, 0.806]. Mean wall of the IC+SGE arm
fell from 12.9 / 32.1 / 91.5 s to 2.89 / 8.48 / 22.2 s (4.5 / 3.8 / 4.1x); counts are unchanged (V1). That is the
Wiedemann change at the whole-arm level. **Confound:** R2 ran `--threads 4` on a 4-vCPU VM, four runs at once, and its
rho rows are 1.8 / 1.4 / 1.5x slower in wall time than the same counted walks here (25.9 / 47.1 / 183.2 s against
14.3 / 32.5 / 118.8 s), although nothing in rho's code changed (the changed `mulmod` works mod the group order, not in
the Cantor field). So R2's walls carry a loaded-host penalty of unknown size on both arms, and the old ratios are not a
clean measure of the binary change. If rho's wall had been equal in both runs, the old ratio would have been
0.90 / 0.99 / 0.77 and the old/new factor 4.5 / 3.8 / 4.1x. V4 passes at either reading (2.67x as measured, about 4x
if the load penalty is taken out of rho only), so the verdict does not depend on the confound; the size does.

## Exponents of mean wall in p (three primes, 65537 to 262147, descriptive, no interval)

IC+SGE 1.47 (old binary 1.41); negation rho 1.53 (old 1.41). Two octaves only; rho's wall is dominated by which
walks were lucky (mean rho steps per run: 8.1M, 15.9M, 60.1M; the ratio of the last two is 3.8 against 2.8 expected
from p^1.5, the first two 2.0, so the non-monotone ratio line above is rho's sampling noise on 12 runs per prime,
not a trend of the arm). Per-curve ratios range from 0.09 to 0.39 at 262147 (T4 in `tables.md`) because rho wall
varies by a factor of 4 between curves while IC wall varies by 10 %.

## Is rho under-engineered? ns per counted operation

Counted operation = Cantor field multiplication + 10 x inversion (rho's counted work is exactly this; the IC
relation phase adds `rel_mul`).

| p | rho ns per counted op | rho ns per walk step | IC relation phase ns per counted op (estimated) | est. LA share of IC wall |
|--:|--:|--:|--:|--:|
| 65537 | 6.35 | 1768 | 6.57 | 0.30 |
| 131101 | 7.34 | 2043 | 7.15 | 0.36 |
| 262147 | 7.10 | 1977 | 7.63 | 0.36 |

The driver records only totals, not the relation phase's wall. The IC relation-phase figure is therefore an estimate:
the IC+SGE arm's wall minus `la_mul` times R8's measured ns per `la_mul` (16.6 / 20.3 / 19.0), divided by its counted
Cantor and relation operations; it carries R8's uncertainty and ignores SGE time and uncounted additions. With that
caveat the rho walk and the IC relation phase cost the same 6.4 to 7.6 ns per counted field operation: both run on the
same Cantor arithmetic, and the rho walk is not slower per counted operation than the code it is compared with. A rho
step is 226.7 field multiplications and 5.17 inversions (about 278 counted operations), so about 1.8 to 2.0 us per
step. This says the counted-operation price is matched; it does not say that a better-engineered genus-3 rho would not
be faster per operation (for example fixed-degree register arithmetic specialised for the Jacobian), because both
share one generic Cantor implementation at 6 to 7 ns per field multiplication, which is probably not at the
machine's limit for 18-bit moduli. A faster Cantor would speed both arms in proportion on the relation phase and rho,
leaving the ratio near 0.2 to 0.26 as long as the linear-algebra share (about a third) does not move. No PMU, so no
cycle counts; that statement is wall-time only.

## What this says

On this host, one thread, same binary, same instances and seeds, the 2LP + prefilter + SGE(8,3) arm finishes in 0.19
to 0.26 of negation rho's wall time at genus 3, p = 65537 to 262147, with every logarithm certified and all counts
identical to R2. The count-based re-pricing of R8 (0.18 to 0.23) was a good predictor of the wall ratio. The matched
test the R8 note asked for is met at these sizes, not beyond them: the ratio shows no trend over a factor of 4 in p, and
its interval at 262147 is wide because rho's runtime is random. The gap to rho is a factor of 4 to 5 in the arm's favour
at these sizes, not a claim about larger p or other genera.

## Limits

One shared VM, one thread, no PMU, walls not isolated from the hypervisor; six curves and two seeds per prime (12 paired
runs), rho wall heavy-tailed; the IC relation-phase split is an estimate; the old-binary comparison is confounded by R2's
load; the rho implementation shares the Cantor code with the IC arm and is not separately tuned; no independent replay.
All figures `[pcc:unverified]`.

## Files
`PROTOCOL.md`, `frozen.sha256`, `run_all.sh`, `raw/run_*.jsonl` (driver output), `raw/load.log`, `analyze.py`,
`analysis.json`, `tables.py`, `tables.md`, `fig_ratio.svg`, `data.sha256`, `binary.sha256`.

## Deviation
`raw/load.log` was deleted by mistake during cleanup (a `rm` of the empty `raw/*.log` stderr files also matched it) and
re-typed from the lines printed earlier in the session (full text visible there); treat the load log as reconstructed, not original.
