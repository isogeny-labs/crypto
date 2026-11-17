#!/bin/bash
# Validator command sequence for research/mt_timed_runpod_20261112 (validated commit 43fc3b0e61, protocol fde73fb405,
# amendment A1 7934b7e343).  W = a scratch directory; V = this validation directory; nothing writes outside W and V.
# No RunPod call is made: the pod is gone; everything is validated from the committed evidence and local reruns.
set -e
REPO=/home/user/crypto; S=$REPO/research/mt_timed_runpod_20261112; V=$S/validation; W=${W:?set W to a scratch directory}
mkdir -p $W/bins $W/payload
cd $REPO
# 0. frozen inputs: the build inputs and the study must equal 43fc3b0e61 (checked before and after the builds)
git diff --stat 43fc3b0e61 -- Cargo.toml Cargo.lock src crates examples research/mt_timed_runpod_20261112 research/mt_timed_20261112 research/mt_fieldops_v5_20261112/instances
git status --porcelain --untracked-files=all -- Cargo.toml Cargo.lock src crates examples
# 1. binary provenance: rebuild each build in its own fresh target dir; sha256 must equal launch/payload.json
CARGO_TARGET_DIR=$W/t-std  cargo build --release --locked --offline --example mt_fieldops
cp $W/t-std/release/examples/mt_fieldops $W/bins/mt_fieldops_std;  rm -rf $W/t-std     # disk is short on this VM
CARGO_TARGET_DIR=$W/t-fast cargo build --release --locked --offline --features mt-fasthash --example mt_fieldops
cp $W/t-fast/release/examples/mt_fieldops $W/bins/mt_fieldops_fast; rm -rf $W/t-fast
sha256sum $W/bins/*                     # std 7ac0cf63..., fast afc768c5... (launch/payload.json, host.json, rows)
# 2. source diff vs the validated timed study (a992ed6c80) and Cargo.toml vs the protocol commit's parent
python3 -I $V/scripts/source_diff.py $REPO a992ed6c807eee6fa713176412e6475e19db7946 6010b68195 43fc3b0e61 $W/source_diff.json
# 3. payload: the launcher's pinned file (sha256 must equal the pin), members vs committed inputs + rebuilt binaries,
#    bit-for-bit rebuild (gzip mtime from the header and, independently, by window search); attempt 1 by window search
cp /tmp/claude-0/-home-user/aeefaead-5838-5329-9e0a-13b7d93fe240/scratchpad/mtr_payload.tar.gz $W/payload/candidate.tar.gz
git show fde73fb405:research/mt_timed_runpod_20261112/run.py > $W/payload/run_fde73fb405.py
python3 -I $V/scripts/payload_check.py $S $W/payload/candidate.tar.gz $W/bins/mt_fieldops_std $W/bins/mt_fieldops_fast $W/payload_check.json
python3 -I $V/scripts/payload_check.py $S none $W/bins/mt_fieldops_std $W/bins/mt_fieldops_fast $W/payload_check_a1.json a1 $W/payload/run_fde73fb405.py
# 4. recompute from the raw rows (validator's code) and field-by-field check of the producer's summary.json
python3 -I $V/scripts/recompute.py $S $REPO/research/mt_timed_20261112 $W/recompute.json
python3 -I $V/scripts/results_check.py $S $W/recompute.json $W/results_check.json
# 5. counter replay: every key of both builds with the rebuilt binaries (CPUs 0,1,3)
JOBS=3 python3 -I $V/scripts/replay.py full std  $W/bins/mt_fieldops_std  $S $W/replay_full_std.json
JOBS=3 python3 -I $V/scripts/replay.py full fast $W/bins/mt_fieldops_fast $S $W/replay_full_fast.json
# 6. EC certificate: dump builds (standalone crate, frozen source + MTFO_DUMP block), nonce sample, validator EC code
python3 -c "import os,time;print(os.urandom(16).hex(),time.strftime('%Y-%m-%dT%H:%M:%SZ',time.gmtime()))" > $W/nonce.txt
# drawn: 5a7b5fe5a26fbf2bd874f3c10df7e3ad 2026-11-12T17:14:31Z, before any sampling
N=$(cut -d' ' -f1 $W/nonce.txt)
git show 43fc3b0e61:examples/mt_fieldops.rs > $W/mt_fieldops_43fc.rs; git show 43fc3b0e61:Cargo.lock > $W/Cargo.lock.43fc
python3 -I $V/scripts/mk_dump_crate.py $W/mt_fieldops_43fc.rs $W/Cargo.lock.43fc $W/dumpcrate
(cd $W/dumpcrate && CARGO_TARGET_DIR=$W/dt-std cargo build --release --offline && CARGO_TARGET_DIR=$W/dt-fast cargo build --release --offline --features mt-fasthash)
for b in std fast; do JOBS=3 python3 -I $V/scripts/replay.py dump $b $W/dt-$b/release/mtfo_dump $S $W/replay_dump_$b.json $N $W/dumps; done
python3 -I $V/scripts/ec_check.py $S $W/ec_check.json $W/replay_dump_std.json $W/replay_dump_fast.json
# 7. the producer's own analysis rerun (reproducibility of its artifacts, not used for the recompute)
(cd $W && python3 -I $S/analyze.py $S/results/holdout $W/prod_summary.json > $W/prod_analysis.txt && python3 -I $S/mk_runs.py $S/results/holdout/runs.jsonl $W/prod_runs.json)
sha256sum $W/prod_summary.json $W/prod_analysis.txt $W/prod_runs.json $S/results/holdout/{summary.json,analysis.txt,runs.json}
# 8. evidence and reports (scripts/findings.json and scripts/claim.json hold the validator's written judgement)
python3 -I $V/scripts/make_artifacts.py $W $V
python3 $REPO/scripts/validation_gate.py evidence $V || true
# 9. issue 2 of claim-report.json (at dd80a7c4d7, after the producer adopted the proposed wording; reads issue 1 from git)
python3 -I $V/scripts/reissue_claim.py $REPO $V/claim-report.json
