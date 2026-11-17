#!/bin/bash
# Restartable counted-S v2 pipeline. All durable state lives in S3 (the host's /dev/shm is wiped by restarts).
# Usage: pipeline.sh  (idempotent; skips stages whose S3 marker exists)
export AWS_SECRET_ACCESS_KEY="$AWS_SECRET_KEY" TMPDIR=/dev/shm
S3=s3://setec-astronomy-artifacts/validation/hashed-ecdl-counts/20261031/v2
SNAP=s3://setec-astronomy-artifacts/validation/hashed-ecdl-counts/20261031/snapshot-1730
R=/home/user/wt-counts/research/hashed_ecdl_counts_20261031
CC=4df337e64d59bbe0145c204048ab1dc69a43c187; BC=3ae1ac2bd63d3fb0e86b7e8c0af2b5ec0f1bd09d
L=/dev/shm/logs; mkdir -p $L /dev/shm/sessions /dev/shm/restore /dev/shm/receipts /dev/shm/bin
exists() { aws s3 ls "$1" >/dev/null 2>&1; }
log() { echo "$(date -u +%H:%M:%S) $*" | tee -a $L/pipeline.log; }
mkwt() { # commit dir
  [ -d /dev/shm/src-c/.git ] || { git clone -q --filter=blob:none --no-checkout https://github.com/isogeny-labs/crypto /dev/shm/src-c; }
  ( cd /dev/shm/src-c && git fetch -q origin $1 2>&1 | tail -1; rm -rf $2; git worktree prune; git worktree add -q --no-checkout --detach $2 $1 )
  ( cd $2; { for p in /Cargo.toml /Cargo.lock /src/ /crates/ /.cargo/ /rust-toolchain.toml /build.rs /docs/ /data/ /tools/factor-base/ /tools/volcano-cli/ /isogeny_algos/ /research/aps_measurement_20261029/ /research/f6_ic_ecbench_ladder_20261005/sessions/n31-killed-02/ /research/ic_candidate_tournament_20260915/goal_20260924/ /research/hashed_ecdl_ecbench_20261030/; do echo $p; done; git show $1:Cargo.toml | grep -oE 'path *= *"[^"]+"' | sed -E 's/path *= *"([^"]+)"/\/\1/'; } | sort -u > .sparse-list
    git sparse-checkout init --no-cone >/dev/null 2>&1; git sparse-checkout set --no-cone --stdin < .sparse-list >/dev/null 2>&1; git checkout -q 2>&1 | tail -1 )
}
# ---- stage 1: restore finished sessions
exists $SNAP/sessions/B/records.jsonl && aws s3 cp --only-show-errors --recursive $SNAP/sessions /dev/shm/restore/sessions
# ---- stage 2: binary C
if exists $S3/bin/ecbench-C; then aws s3 cp --only-show-errors $S3/bin/ecbench-C /dev/shm/bin/ecbench-C; log "C binary from S3";
else
  log "building C"; mkwt $CC /dev/shm/wt-c; ( cd /dev/shm/wt-c; rustc -vV > /dev/shm/rustc-c.txt; CARGO_TARGET_DIR=/dev/shm/target-c cargo build --locked --release --bin ecbench > $L/build-c.log 2>&1 ) || { log "BUILD C FAILED"; exit 1; }
  cp /dev/shm/target-c/release/ecbench /dev/shm/bin/ecbench-C; aws s3 cp --only-show-errors /dev/shm/bin/ecbench-C $S3/bin/ecbench-C; aws s3 cp --only-show-errors /dev/shm/rustc-c.txt $S3/bin/rustc-c.txt; log "C built+uploaded"
fi
chmod +x /dev/shm/bin/ecbench-C; sha256sum /dev/shm/bin/ecbench-C | tee $L/sha-C.txt
# ---- stage 3: C-16 run (pinned CPU 3) in background, unless done
if ! exists $S3/sessions/C-16/records.jsonl; then
  rm -rf /dev/shm/sessions/C-16
  ( taskset -c 3 /dev/shm/bin/ecbench-C run --spec $R/spec-C-tensor-counts-16.json --out /dev/shm/sessions/C-16 --cpus none --lock /dev/shm/lock-c16 --quiet > $L/run-c16.log 2>&1; rc=$?; log "C-16 run rc=$rc"
    [ $rc = 0 ] && aws s3 cp --only-show-errors --recursive /dev/shm/sessions/C-16 $S3/sessions/C-16 && log "C-16 uploaded" ) &
  log "C-16 started"
else aws s3 cp --only-show-errors --recursive $S3/sessions/C-16 /dev/shm/sessions/C-16; fi
# ---- stage 4: binary B (CPUs 0-2)
if exists $S3/bin/ecbench-B; then aws s3 cp --only-show-errors $S3/bin/ecbench-B /dev/shm/bin/ecbench-B; log "B binary from S3";
else
  log "building B"; export RUSTUP_HOME=/dev/shm/rustup; [ -d $RUSTUP_HOME ] || rustup toolchain install 1.99.1 --profile minimal -t x86_64-unknown-linux-musl --no-self-update >$L/rustup.log 2>&1
  mkwt $BC /dev/shm/wt-b; ( cd /dev/shm/wt-b; RUSTUP_HOME=/dev/shm/rustup rustc +1.99.1 -vV > /dev/shm/rustc-b.txt; CC_x86_64_unknown_linux_musl=gcc CARGO_TARGET_DIR=/dev/shm/target-b taskset -c 0-2 cargo +1.99.1 build --locked --release --no-default-features --target x86_64-unknown-linux-musl --bin ecbench > $L/build-b.log 2>&1 ) || { log "BUILD B FAILED"; exit 1; }
  cp /dev/shm/target-b/x86_64-unknown-linux-musl/release/ecbench /dev/shm/bin/ecbench-B; aws s3 cp --only-show-errors /dev/shm/bin/ecbench-B $S3/bin/ecbench-B; aws s3 cp --only-show-errors /dev/shm/rustc-b.txt $S3/bin/rustc-b.txt; log "B built+uploaded"
fi
chmod +x /dev/shm/bin/ecbench-B; sha256sum /dev/shm/bin/ecbench-B | tee $L/sha-B.txt
# ---- stage 5: replay audits (CPUs 0-2)
for t in C-10-14:C B:B; do n=${t%%:*}; b=${t##*:}
  if exists $S3/receipts/$n.json; then log "receipt $n exists"; continue; fi
  taskset -c 0-2 /dev/shm/bin/ecbench-$b verify --dir /dev/shm/restore/sessions/$n --replay-all --out /dev/shm/receipts/$n.json > $L/verify-$n.log 2>&1; rc=$?; log "verify $n rc=$rc"
  aws s3 cp --only-show-errors /dev/shm/receipts/$n.json $S3/receipts/$n.json; aws s3 cp --only-show-errors $L/verify-$n.log $S3/receipts/verify-$n.log
done
wait; log "stages done (C-16 verify pending its completion)"
