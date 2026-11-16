#!/bin/bash
# Thin orchestration of the unchanged native worker and native prefix auditor.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
out=${1:?usage: continue.sh NEW_OUTPUT_DIRECTORY INPUT_DB INPUT_DUMP}
input_db=${2:?}
input_dump=${3:?}
worker=${WORKER:?set absolute pinned WORKER path}
auditor=${AUDITOR:?set absolute native immutable-prefix AUDITOR path}
study=research/prime61_disk_extend_20261018
spec=$study/spec.json
mkdir "$out"
cp "$input_db" "$out/walk.db"
run() {
    local name=$1 cap=$2
    shift 2
    for attempt in 1 2 3; do
        local log=$out/$name
        if [ "$attempt" -gt 1 ]; then log=$out/$name-retry-$attempt; fi
        test ! -e "$log.stdout"
        test "$(df --output=avail -B1 "$out" | tail -1)" -ge 2147483648
        test "$(du -sb "$out" | cut -f1)" -le 2147483648
        printf '%q ' timeout "$cap" "$worker" "$@" > "$log.command"
        printf '\n' >> "$log.command"
        set +e
        timeout "$cap" "$worker" "$@" > "$log.stdout" 2> "$log.stderr"
        local result=$?
        set -e
        printf '%s\n' "$result" > "$log.exit"
        if [ "$result" -eq 0 ]; then
            if [ "$attempt" -gt 1 ]; then cp "$log.stdout" "$out/$name.completed"; fi
            return 0
        fi
        # Only transactional advance/measure work can resume after an outer cap.
        if [ "$result" -ne 124 ] || { [ "$1" != advance ] && [ "$1" != measure ]; }; then return "$result"; fi
    done
    return 124
}
run restore-verify 900 verify "$spec" "$out/walk.db" 16
run restore-dump 300 dump "$spec" "$out/walk.db"
cmp "$input_dump" "$out/restore-dump.stdout"
printf 'PASS: input dump matched exactly after verification and 192-stream replay\n' > "$out/restore-comparison.txt"
expanded=256
measured=512
for batch in 512 1024; do
    while [ "$expanded" -lt "$batch" ]; do
        expanded=$((expanded+32))
        run "advance-$expanded" 300 advance "$spec" "$out/walk.db" "$expanded"
    done
    if [ "$batch" -eq 512 ]; then wanted=768; else wanted=1024; fi
    while [ "$measured" -lt "$wanted" ]; do
        measured=$((measured+64))
        run "measure-$measured" 300 measure "$spec" "$out/walk.db" "$measured"
    done
    run "verify-$batch" 900 verify "$spec" "$out/walk.db" 0
    run "snapshot-$batch" 300 snapshot "$spec" "$out/walk.db" "$out/checkpoint-$batch.db"
    run "dump-$batch" 300 dump "$spec" "$out/checkpoint-$batch.db"
    "$auditor" "$input_dump" "$out/dump-$batch.stdout" > "$out/prefix-$batch.json"
    gzip -n -c "$out/checkpoint-$batch.db" > "$out/checkpoint-$batch.db.gz"
done
run final-status 300 status "$spec" "$out/walk.db"
