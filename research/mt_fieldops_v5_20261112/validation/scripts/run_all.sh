#!/bin/bash
# Validator command sequence for research/mt_fieldops_v5_20261112 (validated commit 2729e252e2, protocol commit 5aab996e55).
# S = a scratch directory; V = this validation directory; nothing here writes outside S and V.
set -e
REPO=/home/user/crypto; V=$REPO/research/mt_fieldops_v5_20261112/validation; S=${S:?set S to a scratch directory}; W=$S/v5val
NONCE=${NONCE:-fc12f5fd8a83efeadcac54dc5d6c889f}        # os.urandom(16) at 2026-11-12T03:31:42Z, before any sampling
mkdir -p $S/frozen $S/pristine/src $S/instr/src $S/v4h/src $S/v4sens/src $W/dumps $W/regen $W/regen_code
cd $REPO
git archive 2729e252e2 research/mt_fieldops_v5_20261112 research/mt_fieldops_v4_20261112 examples/mt_fieldops.rs research/ic_e222_m221_20261106/gen_instances.py | tar -x -C $S/frozen
git show 5aab996e55:examples/mt_fieldops.rs > $S/pristine/src/main.rs       # sha256 1dfdd511..., identical at 2729e252e2
git show bb3d2bc40e:examples/mt_fieldops.rs > $S/v4h/src/main.rs           # v4 harness, sha256 c6b694bc...
FZ=$S/frozen/research/mt_fieldops_v5_20261112
# harness provenance: v4 harness + the v4 validator's patches, compared with the v5 harness (whitespace-normalised)
python3 -I research/mt_fieldops_v4_20261112/validation/scripts/patch_ic5rw.py $S/v4h/src/main.rs $S/v4sens/src/main.tmp
python3 -I research/mt_fieldops_v4_20261112/validation/scripts/patch_rhodpu.py $S/v4sens/src/main.tmp $S/v4sens/src/main.rs
diff <(sed 's/^ *//' $S/v4sens/src/main.rs) <(sed 's/^ *//' $S/pristine/src/main.rs) || true
# standalone crates: rand =0.8.8, serde_json =1.0.151 (arbitrary_precision, raw_value), cargo build --release --offline
python3 -I $V/scripts/patch_instr.py $S/pristine/src/main.rs $S/instr/src/main.rs
for c in pristine instr; do
  printf '[package]\nname = "mtfo_%s"\nversion = "0.1.0"\nedition = "2021"\n[dependencies]\nrand = { version = "=0.8.8", features = ["small_rng"] }\nserde_json = { version = "=1.0.151", features = ["arbitrary_precision", "raw_value"] }\n[profile.release]\nopt-level = 3\n' $c > $S/$c/Cargo.toml
  (cd $S/$c && cargo build --release --offline)
done
P=$S/pristine/target/release/mtfo_pristine; I=$S/instr/target/release/mtfo_instr
# instances: validity, freshness, regeneration from the stated command
python3 -I $V/scripts/instances_check.py $REPO $FZ $W/instances_check.json
cp $S/frozen/research/ic_e222_m221_20261106/gen_instances.py $W/regen_code/
python3 -I $W/regen_code/gen_instances.py --out $W/regen --families W --bits 24,28,32 --per 11 --seed mtfo-v5-20261112
for f in $FZ/instances/*.json; do cmp $f $W/regen/$(basename $f); done
# replay
python3 -I $V/scripts/replay.py producer $REPO/target/release/examples/mt_fieldops $FZ $W/replay_producer.json $NONCE
python3 -I $V/scripts/replay.py sample $P $FZ $W/replay_sample.json $NONCE
JOBS=4 python3 -I $V/scripts/replay.py all $I $FZ $W/replay_all.json $NONCE
python3 -I $V/scripts/replay.py dump $I $FZ $W/replay_dump.json $NONCE $W/dumps
python3 -I $V/scripts/ecverify.py $FZ $W/replay_dump.json $W/scalar_check.json
python3 -I $V/scripts/v4cross.py $P $REPO $W/v4cross.json $NONCE 3
# recompute
python3 -I $V/scripts/recompute.py $FZ $W/recompute.json $NONCE
python3 -I $V/scripts/results_check.py $FZ $W/recompute.json $W/results_check.json
# sensitivity panels (not pre-registered) and analysis
JOBS=2 python3 -I $V/scripts/run_sens.py $P $FZ $W/sens_kappa.json
SENS_ARM=rho2x SENS_KAPPAS=0.5,1.0 JOBS=2 python3 -I $V/scripts/run_sens.py $P $FZ $W/sens_rho2x.json
python3 -I $V/scripts/sens_analysis.py $FZ $W/replay_all.json $W/sens_kappa.json,$W/sens_rho2x.json $W/sensitivity.json
python3 -I $V/scripts/claim_check.py $FZ $W/recompute.json $W/instances_check.json $W/sensitivity.json $W/claim_check.json
# evidence and reports ($W/findings.json and $W/claim.json hold the validator's written judgement)
cp $V/scripts/findings.json $V/scripts/claim.json $W/
python3 -I $V/scripts/compact_evidence.py $W $V $FZ
python3 -I $V/scripts/make_artifacts.py $W $V $FZ $REPO $S/pristine/src/main.rs $P $I
