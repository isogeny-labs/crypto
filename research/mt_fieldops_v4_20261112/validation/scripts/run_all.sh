#!/bin/sh
# Validator command sequence for research/mt_fieldops_v4_20261112 (validated commit bb3d2bc40e).
# S = a scratch directory; V = this validation directory; nothing here writes outside S and V.
set -e
REPO=/home/user/crypto; V=$REPO/research/mt_fieldops_v4_20261112/validation; S=${S:?set S to a scratch directory}
NONCE=${NONCE:-6f8349fedd9cf3d9041ce040e08eb849}        # drawn with os.urandom(16) before any sampling
mkdir -p $S/frozen $S/frozenrepo $S/pristine/src $S/instr/src $S/sens/src $S/sens2/src
cd $REPO
git archive bb3d2bc40e research/mt_fieldops_v4_20261112 | tar -x -C $S/frozen
git archive bb3d2bc40e research/mt_fieldops_v4_20261112 examples/mt_fieldops.rs | tar -x -C $S/frozenrepo
git show bb3d2bc40e:examples/mt_fieldops.rs > $S/pristine/src/main.rs     # sha256 c6b694bc...
FZ=$S/frozen/research/mt_fieldops_v4_20261112
# standalone crates: rand =0.8.8, serde_json =1.0.151 (arbitrary_precision, raw_value), cargo build --release --offline
for c in pristine instr sens sens2; do
  printf '[package]\nname = "mtfo_%s"\nversion = "0.1.0"\nedition = "2021"\n[dependencies]\nrand = { version = "=0.8.8", features = ["small_rng"] }\nserde_json = { version = "=1.0.151", features = ["arbitrary_precision", "raw_value"] }\n' $c > $S/$c/Cargo.toml
done
python3 $V/scripts/patch_instr.py $S/pristine/src/main.rs $S/instr/src/main.rs
python3 $V/scripts/patch_ic5rw.py $S/pristine/src/main.rs $S/sens/src/main.rs
python3 $V/scripts/patch_ic5rw.py $S/pristine/src/main.rs $S/sens2/src/main.tmp && python3 $V/scripts/patch_rhodpu.py $S/sens2/src/main.tmp $S/sens2/src/main.rs
for c in pristine instr sens sens2; do (cd $S/$c && cargo build --release --offline); done
# instances: validity, freshness, regeneration from the stated command
python3 $V/scripts/instances_check.py $REPO $FZ $S/instances_check.json
git show bb3d2bc40e:research/ic_e222_m221_20261106/gen_instances.py > $S/regen_gen.py
python3 -I $S/regen_gen.py --out $S/regen --families W --bits 24,28,32 --per 11 --seed mtfo-v4-20261112
for f in $FZ/instances/*.json; do cmp $f $S/regen/$(basename $f); done
# replay
python3 $V/scripts/replay.py all $S/instr/target/release/mtfo_instr $FZ $S/replay_all.json $NONCE
python3 $V/scripts/replay.py sample $S/pristine/target/release/mtfo_pristine $FZ $S/replay_sample.json $NONCE
python3 $V/scripts/replay.py producer $REPO/target/release/examples/mt_fieldops $FZ $S/replay_producer.json $NONCE
python3 $V/scripts/replay.py dump $S/instr/target/release/mtfo_instr $FZ $S/replay_dump.json $NONCE
python3 $V/scripts/ecverify.py $FZ $S/replay_dump.json $S/ecverify.json
python3 $V/scripts/verify_samples.py $FZ $S/replay_all.json $S/verify_samples.json
# recompute
python3 $V/scripts/recompute.py $FZ $S/recompute.json $NONCE
python3 $V/scripts/results_check.py $FZ $S/recompute.json $S/results_check.json
python3 $V/scripts/coverage.py > $S/coverage.txt
# sensitivity panels (not pre-registered)
JOBS=3 python3 $V/scripts/run_sens.py $S/sens/target/release/mtfo_sens $FZ $S/sens.json
SENS_GRID=rho python3 $V/scripts/run_sens.py $S/pristine/target/release/mtfo_pristine $FZ $S/sens_rho.json
SENS_GRID=dpu python3 $V/scripts/run_sens.py $S/sens2/target/release/mtfo_sens2 $FZ $S/sens_dpu.json
python3 $V/scripts/compact_evidence.py $S $V
python3 $V/scripts/sens_analysis.py $FZ $V/native_work.json $V/sens_ic_runs.json $V/sens_rho_runs.json $V/sens_dpu_runs.json $V/sensitivity.json
python3 $V/scripts/make_artifacts.py $S $S/frozenrepo $V
