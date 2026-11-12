#!/bin/bash
# Validator command sequence for research/mt_timed_20261112 (validated commit 544f832063, protocol commit a992ed6c80).
# W = a scratch directory; V = this validation directory; nothing here writes outside W and V.
# Timing steps (timing.py, timing_round2.py) must run with the machine otherwise idle; they pin to CPU 2 like run.py.
set -e
REPO=/home/user/crypto; V=$REPO/research/mt_timed_20261112/validation; W=${W:?set W to a scratch directory}
mkdir -p $W/frozen $W/pristine/src $W/dumpb/src $W/repobuild
python3 -c "import os,datetime;print(os.urandom(16).hex(),datetime.datetime.now(datetime.timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ'))" > $W/nonce.txt
# drawn: 3f09704503c71d99b8cdd885e6f42941 2026-11-12T04:56:27Z, before any sampling
N=$(cut -d' ' -f1 $W/nonce.txt)
cd $REPO
git archive 544f832063 research/mt_timed_20261112 research/mt_fieldops_v5_20261112 examples/mt_fieldops.rs | tar -x -C $W/frozen
git diff 5aab996e55 a992ed6c80 -- examples/mt_fieldops.rs        # harness change since v5: opcost arm + ms_arm only
# 1. binary provenance: rebuild the committed source in a separate target dir; sha256 must equal the rows' bin_sha256
CARGO_TARGET_DIR=$W/repobuild/target cargo build --release --example mt_fieldops --offline
sha256sum $W/repobuild/target/release/examples/mt_fieldops target/release/examples/mt_fieldops
# 2. validator standalone builds of the frozen source (pristine, and dump-only instrumentation)
git show a992ed6c80:examples/mt_fieldops.rs > $W/pristine/src/main.rs
python3 -I $V/scripts/patch_dump.py $W/pristine/src/main.rs $W/dumpb/src/main.rs
for c in pristine dumpb; do
  printf '[package]\nname = "mtfo_%s"\nversion = "0.1.0"\nedition = "2021"\n[dependencies]\nrand = { version = "=0.8.8", features = ["small_rng"] }\nserde_json = { version = "=1.0.151", features = ["arbitrary_precision", "raw_value"] }\n[profile.release]\nopt-level = 3\n' $c > $W/$c/Cargo.toml
  (cd $W/$c && cargo build --release --offline)
done
P=$W/pristine/target/release/mtfo_pristine; D=$W/dumpb/target/release/mtfo_dumpb
# 3. recompute from the raw rows (validator's code)
python3 -I $V/scripts/recompute.py $W/frozen/research/mt_timed_20261112 $W/recompute.json $N
python3 -I $V/scripts/results_check.py $W/frozen/research/mt_timed_20261112 $W/recompute.json $W/results_check.json
python3 -I $V/scripts/price_model.py $W/frozen/research/mt_timed_20261112 $W/recompute.json $W/price_model.json
# 4. counter replay (CPUs 0,1,3) and EC certificates
JOBS=3 python3 -I $V/scripts/replay.py timed $P $W/frozen $W/replay_timed_all.json $N
JOBS=3 python3 -I $V/scripts/replay.py v5 $P $W/frozen $W/replay_v5.json $N 35
JOBS=3 python3 -I $V/scripts/replay.py timed $REPO/target/release/examples/mt_fieldops $W/frozen $W/replay_producer.json $N 17
JOBS=3 python3 -I $V/scripts/replay.py dump $D $W/frozen $W/replay_dump.json $N 54 $W/dumps
python3 -I $V/scripts/ec_check.py $W/frozen $W/replay_dump.json $W/ec_check.json
# 5. timing reproduction on CPU 2 (machine otherwise idle): round 1, then round 2 (added after round 1)
PIN=2 REPEATS=3 python3 -I $V/scripts/timing.py $REPO/target/release/examples/mt_fieldops $W/frozen $W/recompute.json $W/timing.jsonl $N
python3 -I $V/scripts/timing_analysis.py $W/timing.jsonl $W/frozen/research/mt_timed_20261112 $W/recompute.json $W/timing_analysis.json
PIN=2 REPEATS=5 python3 -I $V/scripts/timing_round2.py $REPO/target/release/examples/mt_fieldops $W/frozen $W/recompute.json $W/timing_r2.jsonl $N
python3 -I $V/scripts/timing_pooled.py $W/timing.jsonl $W/timing_r2.jsonl $W/recompute.json $W/timing_pooled.json
# 6. evidence and reports (scripts/findings.json and scripts/claim.json hold the validator's written judgement)
python3 -I $V/scripts/compact_evidence.py $W $V
python3 -I $V/scripts/make_artifacts.py $V $W/frozen/research/mt_timed_20261112 $W/nonce.txt
# 7. re-issue after the producer's amendment commit fd91337cce (adopted wording = scripts/claim.json proposed_wording)
python3 -I $V/scripts/reissue.py $V $REPO 4e92ffe8a92df02886a5a826e2f1b1df039aa3a75a1b6cdd7e4d90f377c6e09c 2ea2b24ed82c308d9f3f50135a953c114f5d1ffdef185b2df878f2b19576b47b
