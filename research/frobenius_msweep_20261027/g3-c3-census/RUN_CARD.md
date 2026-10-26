# RUN CARD: cell G3, C3 census

**Do not run anything below until:** (a) the resume checkpoint is uploaded
(`scripts/checkpoint.sh init`, fill `.resume/RESUME.md`; the hooks refuse
launchers until the S3 push succeeds; do not use `CHECKPOINT_SKIP`), (b) a
Conductor task covers this directory, (c) the Stage-2 authorisation named in
AMENDMENT.md exists, (d) PROTOCOL.md is frozen by step 1.

All paths are relative to `/home/user/crypto/research/frobenius_msweep_20261027/g3-c3-census`.
Four CPUs are shared with other agents: each command below is one process on
one core (`taskset -c K`). Wall times are **estimates from code reasoning**
(PROTOCOL.md section 8), unmeasured; the gates in step 3 replace them with
pilot numbers before anything expensive launches.

Arms are interleaved *within* each command, per target, in an order shuffled
from the master seed. That is required by the design (same targets, same
instrument, no order effect), so the card has one command per (cell, curve)
rather than one per arm; the arm list in each command is the per-arm coverage.
Seed slots `@k` select independent renamings and phases of the same encoding.

## 0. Environment

```sh
cd /home/user/crypto/research/frobenius_msweep_20261027/g3-c3-census
export RUN=reports/run-$(date -u +%Y%m%dT%H%MZ)      # fix once, reuse in every command
export BIN=$PWD/target/release/g3census
mkdir -p $RUN/rows $RUN/pilot $RUN/summary
cargo build --release 2>&1 | tail -3                  # ~5 s, std-only
cargo test --release -- --skip satlib 2>&1 | tail -3  # expect: 30 passed (9 s); satlib = the repo solver's own tests, skipped
```

## 1. Freeze (before any data)

```sh
$BIN manifest | tee $RUN/manifest.tsv                 # fnv64 of sources and of src/cryptanalysis/sat.rs
sha256sum src/*.rs Cargo.toml ../../../src/cryptanalysis/sat.rs PROTOCOL.md AMENDMENT.md RUN_CARD.md > $RUN/sha256.txt
rustc -Vv > $RUN/rustc.txt; lscpu > $RUN/host.txt; cat /proc/loadavg > $RUN/load_before.txt; git rev-parse HEAD > $RUN/commit.txt
```
Record `$RUN/sha256.txt` in PROTOCOL.md as the freeze. A change to any `src/`
file after this point is an amendment.

## 2. Pilot (round 0, excluded from inference), ~10 min, core 0

```sh
taskset -c 0 $BIN run --n 17 --l 8  --m 2 --curve-a 0 --unsat 5 --sat-keep 2 --planted 3 --arms C,C@2,Cpoly,Cnobreak,A,B,Bnogauge,G --planted-only-arms U --round 0 --out $RUN/pilot
taskset -c 0 $BIN run --n 23 --l 11 --m 2 --curve-a 0 --unsat 4 --sat-keep 1 --planted 2 --arms C,C@2,Cnobreak,A,B,G --planted-only-arms U --round 0 --out $RUN/pilot
taskset -c 0 $BIN run --n 31 --l 15 --m 2 --curve-a 0 --unsat 3 --sat-keep 0 --planted 1 --arms C,C@2,A,B --round 0 --deadline-s 3600 --out $RUN/pilot
taskset -c 0 $BIN run --n 31 --l 5  --m 3 --curve-a 0 --unsat 4 --sat-keep 0 --planted 1 --arms C,C@2,Cnobreak,A,B,G --round 0 --out $RUN/pilot
taskset -c 0 $BIN run --n 31 --l 6  --m 3 --curve-a 0 --unsat 3 --sat-keep 0 --planted 1 --arms C,C@2,B --round 0 --out $RUN/pilot
$BIN gate $RUN/pilot | tee $RUN/pilot/gate.tsv         # mean conflicts and ns per conflict per cell and arm
$BIN summarize $RUN/pilot --out $RUN/pilot/summary     # sanity only; pilot labels are never reported
```

## 3. Gates (decide from `gate.tsv` only, before step 5 or 6)

Let `c1(arm)` = ns per conflict from the pilot and K(n,m,l) the mean C
conflicts, extrapolated as K(l+1) = K(l) x 2^{m-1}. Seconds per target for a
cell = K x sum over its arms of `c1(arm)`. CPU-hours = that x N / 3600.

* G2: n = 31, m = 2: N per curve = min(25, floor(4 x 3600 / seconds-per-target)). If N < 10 per curve: do not launch that cell; record O-IMPEDIMENT(budget).
* G3: (31,3,10): arms C, C@2, B; pooled N = min(10, floor(12 x 3600 / seconds-per-target)); if N < 6 do not launch. (31,3,11): launch only if N = 6 fits in 12 CPU-hours.
* If the pilot A/A (`C@2/C@1`) sd(log) is above 0.2 in any cell, add slots 3 and 4 for that cell's primary arms (the MDE table of PROTOCOL.md section 5) before reading anything.
* If any pilot soundness check fails (a `verdict_ok = 0` on a sound arm, a failed certificate, or `Cnobreak` showing no effect), stop and fix the instrument. Do not proceed.

## 4. Round 1, cheap cells first, ~1.5 CPU-h, core 0 (about 1.5 h wall)

Positive controls run first (cheap cells); inspect `SUMMARY.md` for P1 before step 5.

```sh
SM="C,C@2,C@3,C@4,Cpoly,Cnobreak,A,A@2,A@3,A@4,B,B@2,B@3,B@4,G"
for a in 0 1; do
  for l in 8 9; do
    taskset -c 0 $BIN run --n 17 --l $l --m 2 --curve-a $a --unsat 25 --sat-keep 5 --planted 10 --arms $SM,Bnogauge --planted-only-arms U --round 1 --out $RUN/rows
  done
  taskset -c 0 $BIN run --n 23 --l 11 --m 2 --curve-a $a --unsat 25 --sat-keep 5 --planted 10 --arms $SM,Bnogauge --planted-only-arms U --round 1 --out $RUN/rows
  taskset -c 0 $BIN run --n 23 --l 12 --m 2 --curve-a $a --unsat 25 --sat-keep 5 --planted 10 --arms $SM --planted-only-arms U --round 1 --out $RUN/rows
done
# V sensitivity (second sigma-stable V of the same dimension), one curve
for l in 8 9; do taskset -c 0 $BIN run --n 17 --l $l --m 2 --vsel 1 --curve-a 0 --unsat 25 --sat-keep 5 --planted 10 --arms $SM --planted-only-arms U --round 1 --out $RUN/rows; done
for l in 11 12; do taskset -c 0 $BIN run --n 23 --l $l --m 2 --vsel 1 --curve-a 0 --unsat 25 --sat-keep 5 --planted 10 --arms $SM --planted-only-arms U --round 1 --out $RUN/rows; done
# ordinary-curve unsoundness detector (b = t, not in F_2): C, A must agree with the oracle, B must not
taskset -c 0 $BIN run --n 17 --l 8 --m 2 --b 2 --curve-a 0 --unsat 25 --sat-keep 5 --planted 10 --arms C,A,B --round 1 --out $RUN/rows
# no-priority sensitivity is NOT in this build; do not improvise it (it would be an amendment)
$BIN summarize $RUN/rows --out $RUN/summary
```
Gate before continuing: `SUMMARY.md` must show P1 (`C@1+2` over `Cnobreak@1`, role `control`) with upper bound below 0.95 in every n = 17 and 23 cell, no `O-ARTIFACT`, and the ordinary-curve line "detector WORKS". Otherwise stop.

## 5. Round 1, n = 31, m = 2, ~3.1 + 6.1 CPU-h per curve pair at N = 50 (central estimate), cores 1 and 2

One lane per curve; N per curve from gate G2 (shown here at 25).

```sh
S31="C,C@2,Cpoly,Cnobreak,A,A@2,B,B@2,G"
# core 1
taskset -c 1 $BIN run --n 31 --l 15 --m 2 --curve-a 0 --unsat 25 --sat-keep 5 --planted 10 --arms $S31 --planted-only-arms U --round 1 --out $RUN/rows
taskset -c 1 $BIN run --n 31 --l 16 --m 2 --curve-a 0 --unsat 25 --sat-keep 5 --planted 10 --arms $S31 --planted-only-arms U --round 1 --out $RUN/rows
# core 2
taskset -c 2 $BIN run --n 31 --l 15 --m 2 --curve-a 1 --unsat 25 --sat-keep 5 --planted 10 --arms $S31 --planted-only-arms U --round 1 --out $RUN/rows
taskset -c 2 $BIN run --n 31 --l 16 --m 2 --curve-a 1 --unsat 25 --sat-keep 5 --planted 10 --arms $S31 --planted-only-arms U --round 1 --out $RUN/rows
# replicate slots (same targets: the target stream is a pure function of the arguments), only where PROTOCOL.md section 8 G2 allows
taskset -c 1 $BIN run --n 31 --l 15 --m 2 --curve-a 0 --unsat 25 --sat-keep 5 --planted 10 --arms C@3,C@4,A@3,A@4,B@3,B@4 --round 1 --tag rep --out $RUN/rows
# ... likewise --l 16 and --curve-a 1 (one command each), by the same rule
```
A killed or timed-out process leaves a partial `rows_*.tsv`; delete that file and rerun the whole command into a fresh `$RUN` subdirectory. Do not splice.

## 6. Round 1, m = 3, core 3 (and 0, 1, 2 as they free up)

```sh
# ladder: all arms, N = 25 pooled over one curve, ~0.6 CPU-h
taskset -c 3 $BIN run --n 31 --l 5 --m 3 --curve-a 0 --unsat 25 --sat-keep 3 --planted 10 --arms C,C@2,C@3,C@4,Cpoly,Cnobreak,A,A@2,A@3,A@4,B,B@2,B@3,B@4,G --planted-only-arms U --round 1 --out $RUN/rows
taskset -c 3 $BIN run --n 31 --l 6 --m 3 --curve-a 0 --unsat 25 --sat-keep 3 --planted 10 --arms C,C@2,C@3,C@4,Cpoly,Cnobreak,A,A@2,A@3,A@4,B,B@2,B@3,B@4,G --planted-only-arms U --round 1 --out $RUN/rows
# spec cell (31,3,10): arms C, C@2, B; N from gate G3, in chunks that can run on idle cores (here 4 chunks of 3 = 12, trim to the gate's N).
# Each chunk draws its own targets (--seed-offset) and must use a distinct --tag.
for c in 1 2 3 4; do
  taskset -c $((c-1)) $BIN run --n 31 --l 10 --m 3 --curve-a 0 --unsat 3 --sat-keep 0 --planted 1 --arms C,C@2,B --planted-only-arms U --round 1 --conflicts 40000000 --deadline-s 14400 --seed-offset $c --tag c$c --out $RUN/rows &
done; wait
# (31,3,11): only if gate G3 allows, same form with --l 11
```
Estimated: ladder 0.6 CPU-h; (31,3,10) about 16 CPU-h at N = 10 (4 h wall on 4 cores), uncertain by a factor of ten either way.

## 7. Analysis and report

```sh
$BIN summarize $RUN/rows --out $RUN/summary     # SUMMARY.md, summary.json, forest.svg; seconds
$BIN gate $RUN/rows > $RUN/summary/gate_final.tsv
cat /proc/loadavg > $RUN/load_after.txt
```
Then, per `.claude/skills/research-visuals/SKILL.md`, write the run report and
compile a PDF from it, keep `forest.svg`, and render a PNG. The PR copies them
into `benchmarks/` **only if** a validation-gate record (replay certificate,
validator report, matched rho) exists; this is a toy-tier control census with no
rho reference, so it is not an index-calculus leaderboard or `benchmarks/` row.
Wall time and RSS are in the rows (practicality notes). Report each cell's label
as `SUMMARY.md` prints it; the verdict wording is fixed by PROTOCOL.md section 7.

## 8. Clean up

```sh
rm -rf target                                   # std-only crate, ~60 MB; reports/ stays
```

## Expected wall time (central estimate, four cores, not measured)

| step | CPU-h | wall |
|:--|--:|--:|
| 0, 1, 2 | 0.2 | 15 min |
| 4 (n = 17, 23, V sensitivity, ordinary) | 1.5 | 1.5 h on core 0, in parallel with 5 and 6 |
| 5 (n = 31, m = 2, slots 1 and 2) | 9.2 | 4.6 h on two cores |
| 5 replicate (conditional) | up to 7.6 | up to 3.8 h |
| 6 ladder + (31,3,10) at N = 10 | 17 | 4.5 h on four cores after steps 4 and 5 free them |
| total | about 35 | about 9 h; gates cut it to fit 8 h |

Output layout: `$RUN/{manifest.tsv,sha256.txt,rustc.txt,host.txt,commit.txt,load_*.txt}`,
`$RUN/pilot/`, `$RUN/rows/rows_n<N>_l<L>_m<M>_v<V>_b<B>_a<A>_r<R>[_tag].tsv`,
`$RUN/summary/{SUMMARY.md,summary.json,forest.svg,gate_final.tsv}`.
