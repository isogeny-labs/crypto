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
export CARGO_TARGET_DIR=/tmp/g3census-target          # outside the tree: the freeze needs an empty git status
export BIN=$CARGO_TARGET_DIR/release/g3census
mkdir -p $RUN/rows $RUN/pilot $RUN/summary            # reports/run-*/ is gitignored in this cell
cargo build --release 2>&1 | tail -3                  # ~5 s, std-only
cargo test --release -- --skip satlib 2>&1 | tail -3  # expect: 38 passed (about 21 s); satlib = the repo solver's own tests, skipped
                                                      # (tests write to the OS temp dir, not the tree)
```

## 1. Freeze (before any data)

The frozen set is committed first, so that the commit binds the sources
(`Cargo.lock` is ignored by the root `.gitignore` (`**/Cargo.lock`) and is NOT committed; the crate has no dependencies, and its sha256 is recorded in FREEZE.txt below). Commit and PR handling follow AGENTS.md "Default workflow"; this
card only fixes what must be true before round 0.

```sh
# 1a. protocol.json (experiment.protocol/v1): author it beside PROTOCOL.md, then validate and freeze it
#     with the sibling checker. If `validate` fails or the schema is missing, STOP: no data.
A=../../../../crypto-autoresearcher/tools/experiment_protocol.py     # sibling checkout at origin/main
python3 $A validate protocol.json || { echo "protocol.json invalid: do not run"; exit 1; }
# 1b. commit the frozen set (this also records the removal of the old tracked target/test-* fixtures)
git rm -r -q --cached target 2>/dev/null || true
git add .gitignore PROTOCOL.md AMENDMENT.md RUN_CARD.md README.md protocol.json Cargo.toml src
git commit -m "g3-c3-census: freeze protocol, run card and sources"
python3 $A freeze protocol.json --commit auto --filed-before-affected-data
git add protocol.json && git commit -m "g3-c3-census: frozen protocol.json" && git rev-parse HEAD
# 1c. the tree must now be clean for the cell and for the solver source
git status --porcelain -- . ../../../src/cryptanalysis/sat.rs > /tmp/g3_freeze_status.txt
test ! -s /tmp/g3_freeze_status.txt || { cat /tmp/g3_freeze_status.txt; echo "dirty tree: do not run"; exit 1; }
cp /tmp/g3_freeze_status.txt $RUN/FREEZE_status.txt
cargo build --release 2>&1 | tail -3
{
  echo "commit $(git rev-parse HEAD)"; echo "branch $(git rev-parse --abbrev-ref HEAD)"
  echo "cargo $(cargo -V)"; echo "RUSTFLAGS=[${RUSTFLAGS}]"; rustc -Vv
  echo "binary_sha256 $(sha256sum $BIN)"
  sha256sum src/*.rs Cargo.toml Cargo.lock .gitignore ../../../src/cryptanalysis/sat.rs PROTOCOL.md AMENDMENT.md RUN_CARD.md README.md protocol.json
} > $RUN/FREEZE.txt
$BIN manifest | tee $RUN/manifest.tsv                 # fnv64 of sources and of src/cryptanalysis/sat.rs
lscpu > $RUN/host.txt; cat /proc/loadavg > $RUN/load_before.txt
```
`FREEZE.txt` is a separate file: PROTOCOL.md is not edited at the freeze (it is
itself hashed). A change to any frozen file after this point is an amendment.

## 2. Pilot (round 0, excluded from inference), ~10 min, core 0

Hard cap `--conflicts 500000 --deadline-s 300` so no arm can run away; pilot
targets come from `--seed-offset 1000`, a stream disjoint from every round-1
stream (round-1 uses offsets 0 and 1 to 4), so no pilot target is later reused
as an inference target.

```sh
PIL="--round 0 --conflicts 500000 --deadline-s 300 --seed-offset 1000 --out $RUN/pilot"
taskset -c 0 $BIN run --n 17 --l 8  --m 2 --curve-a 0 --unsat 5 --sat-keep 2 --planted 3 --arms C,C@2,Cpoly,Cnobreak,A,B,Bnogauge,G --planted-only-arms U --unsat-only-arms Chalf $PIL
taskset -c 0 $BIN run --n 23 --l 11 --m 2 --curve-a 0 --unsat 4 --sat-keep 1 --planted 2 --arms C,C@2,Cnobreak,A,B,G --planted-only-arms U --unsat-only-arms Chalf $PIL
taskset -c 0 $BIN run --n 31 --l 15 --m 2 --curve-a 0 --unsat 3 --sat-keep 0 --planted 1 --arms C,C@2,A,B $PIL
taskset -c 0 $BIN run --n 31 --l 5  --m 3 --curve-a 0 --unsat 4 --sat-keep 0 --planted 1 --arms C,C@2,Cnobreak,A,B,G --unsat-only-arms Chalf $PIL
taskset -c 0 $BIN run --n 31 --l 6  --m 3 --curve-a 0 --unsat 3 --sat-keep 0 --planted 1 --arms C,C@2,B $PIL
$BIN gate $RUN/pilot | tee $RUN/pilot/gate.tsv         # per cell and arm: censored count, conflicts, ns/conflict, max RSS, deadline-censor marker
$BIN summarize $RUN/pilot --out $RUN/pilot/summary     # sanity only; pilot labels are never reported
```
The review observed one correctness-only scratch run at (31,3,5) at 10
CPU-minutes and 1.7 GB with 5M-conflict cap, far above the table in PROTOCOL.md
section 8. If any pilot row is censored or any arm is slow, the estimate is
wrong and G5 applies.

## 3. Gates (decide from `gate.tsv` only, before step 4, 5 or 6)

Let `c1(arm)` = ns per conflict from the pilot and K(n,m,l) the mean C
conflicts, extrapolated as K(l+1) = K(l) x 2^{m-1}. Seconds per target for a
cell = K x sum over its arms of `c1(arm)`. CPU-hours = that x N / 3600.

* **G4 (memory):** projected RSS = `max_rss_mb` x (real cap / `max_conflicts`) per arm. At most 3 GB per process (four run at once). If larger, lower that cell's `--conflicts` until it fits, and record the cap as the censor rule.
* **G5 (re-estimate):** if any arm is more than 2x off the PROTOCOL.md section 8 table, or any pilot row is censored, recompute the whole plan from the pilot numbers (including SAT, U and Chalf rows) before launching the ladder or anything at n = 31.
* G2: n = 31, m = 2: N per curve = min(25, floor(4 x 3600 / seconds-per-target)). If N < 10 per curve: do not launch that cell; record O-IMPEDIMENT(budget).
* G3: (31,3,10): arms C, C@2, B; pooled N = min(10, floor(12 x 3600 / seconds-per-target)); if N < 6 do not launch. (31,3,11): launch only if N = 6 fits in 12 CPU-hours.
* If the pilot sd(log) of the A/C and B/C ratios (read from the pilot SUMMARY, not only C@2/C@1) is above 0.2 in any cell, that cell is *resolution-limited*: report its MDE (PROTOCOL.md section 5) as is. Slots 3 and 4 are already in the step-4 arm list `$SM`; they are replicate slots (PROTOCOL.md rule 3) and are not pooled into the primary ratio.
* **P3 numeric gate:** pooled over the pilot's Chalf cells (at least 3 UNSAT targets), the median Chalf/C must be <= 0.8 (PROTOCOL.md section 4). Above 0.8, or any pilot soundness failure (a `verdict_ok = 0` on a sound arm, a failed certificate): stop and review the instrument. A censored pilot row is only a lower bound on cost: do not use it for a projection; it triggers G5. `Cnobreak` showing no effect is an informative miss (PROTOCOL.md section 4), not a stop.

**Abort rule (n = 17 and 23 are not exempt):** if `$BIN gate $RUN/rows` shows any censored UNSAT row in a cell, stop launching that cell's remaining chunks (the 1% tolerance already makes it O-IMPEDIMENT) and re-estimate under G5 before the next cell.

## 4. Round 1, cheap cells first, ~1.5 CPU-h, core 0 (about 1.5 h wall)

The controls run first (cheap cells); inspect `SUMMARY.md` for P3 before step 5.

```sh
SM="C,C@2,C@3,C@4,Cpoly,Cnobreak,A,A@2,A@3,A@4,B,B@2,B@3,B@4,G"
for a in 0 1; do
  for l in 8 9; do
    taskset -c 0 $BIN run --n 17 --l $l --m 2 --curve-a $a --unsat 25 --sat-keep 5 --planted 10 --arms $SM,Bnogauge --planted-only-arms U --unsat-only-arms Chalf --round 1 --out $RUN/rows
  done
  taskset -c 0 $BIN run --n 23 --l 11 --m 2 --curve-a $a --unsat 25 --sat-keep 5 --planted 10 --arms $SM,Bnogauge --planted-only-arms U --unsat-only-arms Chalf --round 1 --out $RUN/rows
  taskset -c 0 $BIN run --n 23 --l 12 --m 2 --curve-a $a --unsat 25 --sat-keep 5 --planted 10 --arms $SM --planted-only-arms U --unsat-only-arms Chalf --round 1 --out $RUN/rows
done
# V sensitivity (second sigma-stable V of the same dimension), one curve
for l in 8 9; do taskset -c 0 $BIN run --n 17 --l $l --m 2 --vsel 1 --curve-a 0 --unsat 25 --sat-keep 5 --planted 10 --arms $SM --planted-only-arms U --unsat-only-arms Chalf --round 1 --out $RUN/rows; done
for l in 11 12; do taskset -c 0 $BIN run --n 23 --l $l --m 2 --vsel 1 --curve-a 0 --unsat 25 --sat-keep 5 --planted 10 --arms $SM --planted-only-arms U --unsat-only-arms Chalf --round 1 --out $RUN/rows; done
# ordinary-curve unsoundness detector (b = t, not in F_2): C, A must agree with the oracle, B must not
taskset -c 0 $BIN run --n 17 --l 8 --m 2 --b 2 --curve-a 0 --unsat 25 --sat-keep 5 --planted 10 --arms C,A,B --round 1 --out $RUN/rows
# no-priority sensitivity is NOT in this build; do not improvise it (it would be an amendment)
$BIN summarize $RUN/rows --out $RUN/summary
```
Gate before continuing: `SUMMARY.md` must show P3 (`Chalf@1` over `C@1`, role `control`, expected 0.125 to 0.5) with upper bound below 0.95 in every n = 17 and 23 cell, no `O-ARTIFACT`, and the ordinary-curve line "detector WORKS". Otherwise stop. P1 (`C@1` over `Cnobreak@1`) and P2 are read and recorded as informative; a miss there does not stop the run.

## 5. Round 1, n = 31, m = 2, ~3.6 + 7.0 CPU-h per curve pair at N = 50 (central estimate, SAT/U/Chalf included), cores 1 and 2

One lane per curve; N per curve from gate G2 (shown here at 25).

```sh
S31="C,C@2,Cpoly,Cnobreak,A,A@2,B,B@2,G"
X31="--unsat 25 --sat-keep 2 --planted 4 --planted-only-arms U --unsat-only-arms Chalf --round 1 --out $RUN/rows"   # SAT targets capped at n = 31 (PROTOCOL.md section 8)
# core 1
taskset -c 1 $BIN run --n 31 --l 15 --m 2 --curve-a 0 --arms $S31 $X31
taskset -c 1 $BIN run --n 31 --l 16 --m 2 --curve-a 0 --arms $S31 $X31
# core 2
taskset -c 2 $BIN run --n 31 --l 15 --m 2 --curve-a 1 --arms $S31 $X31
taskset -c 2 $BIN run --n 31 --l 16 --m 2 --curve-a 1 --arms $S31 $X31
# replicate slots (same targets: the target stream is a pure function of the arguments), only where PROTOCOL.md section 8 G2 allows
taskset -c 1 $BIN run --n 31 --l 15 --m 2 --curve-a 0 --unsat 25 --sat-keep 2 --planted 4 --arms C@3,C@4,A@3,A@4,B@3,B@4 --round 1 --tag rep --out $RUN/rows
# ... likewise --l 16 and --curve-a 1 (one command each), by the same rule
```
A killed or timed-out process leaves a partial `rows_*.tsv`; delete that file and rerun the whole command into a fresh `$RUN` subdirectory. Do not splice.

## 6. Round 1, m = 3

Cores are shared with steps 4 and 5, so nothing here starts on a core that
step 4 (core 0) or step 5 (cores 1, 2) still holds. Lanes: the ladder and
chunk c4 on core 3; chunk c1 on core 0 once step 4 has exited; chunks c2 and
c3 on cores 1 and 2 once their step-5 lanes have exited. Check with
`pgrep -a g3census` (once, not in a loop) before each launch. The ladder uses
the `run` defaults (cap 20 000 000, deadline 900 s); only the (31,3,10) chunks
use the m = 3 budgets (PROTOCOL.md section 2).

```sh
# ladder: all arms, N = 25 pooled over one curve, ~0.6 CPU-h (re-estimate from the pilot first: G5)
taskset -c 3 $BIN run --n 31 --l 5 --m 3 --curve-a 0 --unsat 25 --sat-keep 3 --planted 6 --arms C,C@2,C@3,C@4,Cpoly,Cnobreak,A,A@2,A@3,A@4,B,B@2,B@3,B@4,G --planted-only-arms U --unsat-only-arms Chalf --round 1 --out $RUN/rows
taskset -c 3 $BIN run --n 31 --l 6 --m 3 --curve-a 0 --unsat 25 --sat-keep 3 --planted 6 --arms C,C@2,C@3,C@4,Cpoly,Cnobreak,A,A@2,A@3,A@4,B,B@2,B@3,B@4,G --planted-only-arms U --unsat-only-arms Chalf --round 1 --out $RUN/rows
# spec cell (31,3,10): arms C, C@2, B only, no SAT or planted targets, no Chalf; N from gate G3,
# in chunks of 3 targets (4 chunks = 12, trim to the gate's N). Each chunk draws its own targets
# (--seed-offset c) and has a distinct --tag; the analysis keys targets by xr, so chunks never collide.
CH="--n 31 --l 10 --m 3 --curve-a 0 --unsat 3 --sat-keep 0 --planted 0 --arms C,C@2,B --round 1 --conflicts 40000000 --deadline-s 14400 --out $RUN/rows"
taskset -c 3 $BIN run $CH --seed-offset 4 --tag c4 &        # core 3, after the ladder
taskset -c 0 $BIN run $CH --seed-offset 1 --tag c1 &        # core 0, after step 4
taskset -c 1 $BIN run $CH --seed-offset 2 --tag c2 &        # core 1, after its step-5 lane
taskset -c 2 $BIN run $CH --seed-offset 3 --tag c3 &        # core 2, after its step-5 lane
wait
# (31,3,11): only if gate G3 allows, same form with --l 11
```
(Each `taskset ... &` line above is launched when its core is free, not all at
once.)
Estimated: ladder 0.7 CPU-h; (31,3,10) about 16 CPU-h at N = 10 (4 h wall on 4
cores), uncertain by a factor of ten either way. (31,3,10) carries no U arm, so
the tripwire count excludes it.

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
| 5 (n = 31, m = 2, slots 1 and 2; SAT/U/Chalf included) | 10.6 | 5.3 h on two cores |
| 5 replicate (conditional) | up to 7.6 | up to 3.8 h |
| 6 ladder + (31,3,10) at N = 10 | 17 | 4.5 h on four cores after steps 4 and 5 free them (lanes in step 6) |
| total | about 40 | about 10 h; gates cut it to fit 8 h |

Output layout: `$RUN/{FREEZE.txt,FREEZE_status.txt,manifest.tsv,host.txt,load_*.txt}`,
`$RUN/pilot/`, `$RUN/rows/rows_n<N>_l<L>_m<M>_v<V>_b<B>_a<A>_r<R>[_tag].tsv`,
`$RUN/summary/{SUMMARY.md,summary.json,forest.svg,gate_final.tsv}`.
