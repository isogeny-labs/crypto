#!/bin/bash
# Thin native-worker orchestration; each log and exit receipt is write-once.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
out=${1:?usage: continue.sh NEW_OUTPUT_DIR RESTORED_DB ARCHIVED_RECORDS}
restored=${2:?}
archived=${3:?}
mkdir "$out"
spec=research/prime61_disk_continue_20261018/spec.json
worker=${WORKER:-target/release/examples/isogeny_disk_walk}
cp "$restored" "$out/walk.db"
run() {
    local name=$1
    shift
    if [ "$(df --output=avail -B1 "$out" | tail -1)" -lt 2147483648 ]; then
        echo 'disk guard: fewer than 2 GiB free' >&2
        return 90
    fi
    printf '%q ' timeout 300 "$worker" "$@" > "$out/$name.command"
    printf '\n' >> "$out/$name.command"
    set +e
    timeout 300 "$worker" "$@" > "$out/$name.stdout" 2> "$out/$name.stderr"
    local result=$?
    set -e
    printf '%s\n' "$result" > "$out/$name.exit"
    [ "$result" -eq 0 ]
}
run restore-verify verify "$spec" "$out/walk.db" 16
run restore-dump dump "$spec" "$out/walk.db"
cmp "$archived" "$out/restore-dump.stdout"
printf 'PASS: archived logical dump exactly reproduced after native verification/replay\n' > "$out/restore-comparison.txt"
for expanded in 64 96 128 160 192 224 256; do
    run "advance-$expanded" advance "$spec" "$out/walk.db" "$expanded"
    case "$expanded" in
        64) first=128; last=128 ;;
        128) first=192; last=256 ;;
        256) first=320; last=512 ;;
        *) continue ;;
    esac
    for ((measured=first; measured<=last; measured+=64)); do
        run "measure-$measured" measure "$spec" "$out/walk.db" "$measured"
    done
    run "verify-$expanded" verify "$spec" "$out/walk.db" 0
    run "snapshot-$expanded" snapshot "$spec" "$out/walk.db" "$out/checkpoint-$expanded.db"
    run "dump-$expanded" dump "$spec" "$out/checkpoint-$expanded.db"
    gzip -n -c "$out/checkpoint-$expanded.db" > "$out/checkpoint-$expanded.db.gz"
    sha256sum "$out/checkpoint-$expanded.db" "$out/checkpoint-$expanded.db.gz" "$out/dump-$expanded.stdout" > "$out/checkpoint-$expanded.sha256"
done
run final-status status "$spec" "$out/walk.db"
