#!/bin/sh
# Thin orchestration only; native workers, replay and control gates do the checks.
set -eu
test "$#" -eq 1
root=$(realpath "$1")
cd "$root"
test ! -e runs
sha256sum -c inputs.sha256 > input-check.log
test "$(cat build/lib-exit.txt)" = 0
test "$(cat build/tests-exit.txt)" = 0
test "$(cat build/build-exit.txt)" = 0
test "$(cat build/proof-exit.txt)" = 0
jq -e '.schema=="n83.d18-shared-root-construction/v1" and .status=="PASS"
    and .columns==16627 and (.rows|length)==6 and .search_performed==false' construction.json > /dev/null
# The preceding bounded container must finish before this card starts.
test -z "$(sudo docker ps -q)"
revision=$(cat source-commit.txt)
mkdir runs
date -u +%FT%TZ > card-start.utc
uname -a > host.txt
uptime > host-uptime.txt
lscpu > host-cpu.txt
sudo docker ps --no-trunc > host-containers.txt
trap 'result=$?; if [ "$result" -ne 0 ]; then printf "FAILED_OR_INTERRUPTED exit=%s\n" "$result" > card-status.txt; fi' EXIT
run() (
    mode=$1 encoding=$2 arm=$3 target=$4 label=$5
    shift 5
    if [ "$mode" = replay ]; then binary="$root/replayer"; else binary="$root/worker"; fi
    sh "$root/run-docker.sh" "$mode" "$encoding" "$arm" "$target" "$root/runs/$label" "$binary" \
        "/home/ec2-user/n83-m23-20261019/inputs/$arm" \
        /home/ec2-user/n83-m23-20261019/inputs/public/public-targets.json "$revision" "$label" "$@"
)
positive() (
    encoding=$1 arm=$2 label=$3
    run positive "$encoding" "$arm" 0 "$label"
    jq -e '.status=="PASS_CONSTRUCTED_PENDING_REPLAY"' "runs/$label/worker.json" > /dev/null
    run replay "$encoding" "$arm" 0 "$label-replay" "$root/runs/$label/worker.json"
    jq -e '.status=="PASS"' "runs/$label-replay/worker.json" > /dev/null
    if [ "$arm" = D ]; then
        jq -s -e '.[0].root_representatives_blake3==.[1].root_representatives_blake3
            and .[0].root_shared_bit_count==(.[1].shared_bits|length)' \
            "runs/$label/worker.json" construction.json > /dev/null
    fi
)
pair() (
    kind=$1 a=$2 b=$3 label=$4
    "$root/gate" control-pair "$kind" \
        "runs/$a/worker.json" "runs/$b/worker.json" \
        "runs/$a-replay/worker.json" "runs/$b-replay/worker.json" "pair-$label.json"
    jq -e '.status=="PASS" and .matched_assignment_and_relation==true' "pair-$label.json" > /dev/null
)
for round in 0 1 2 3 4; do
    positive per-representative D "aa-$round-a1"
    positive per-representative D "aa-$round-a2"
    pair aa "aa-$round-a1" "aa-$round-a2" "aa-$round"
done
for round in 0 1 2 3 4; do
    positive per-representative D "ab-$round-a"
    positive shared-bits D "ab-$round-b"
    pair ab "ab-$round-a" "ab-$round-b" "ab-$round"
done
positive per-representative U u-a
positive shared-bits U u-b
pair ab u-a u-b u
natural() (
    encoding=$1 target=$2 label=$3 result=0
    run natural "$encoding" D "$target" "$label" || result=$?
    test -s "runs/$label/worker.json.progress.jsonl"
    jq -s -e --arg encoding "$encoding" --argjson target "$target" 'length>0 and all(.[];
        .schema=="n83.small-m4-progress/v1" and .identity.arm=="D" and .identity.target_index==$target
        and .identity.root_encoding==$encoding and .identity.experiment_policy=="d18-shared-root-m4/v1"
        and .identity.xor_decided_cache==true and .identity.conflict_budget==10000
        and .identity.mode=="natural" and (.counters.completed_conflicts|type)=="number")' \
        "runs/$label/worker.json.progress.jsonl" > /dev/null
    if [ "$result" = 124 ]; then
        jq -e '.status=="UNKNOWN_budget"' "runs/$label/worker.json.cap.json" > /dev/null
    elif [ "$result" = 0 ]; then
        jq -e '(.status=="UNKNOWN_search_cap" and .counters.completed_conflicts==10000)
            or .status=="UNVERIFIED_MODEL_UNSAT" or .status=="PASS_NATURAL_PENDING_REPLAY"' \
            "runs/$label/worker.json" > /dev/null
        if jq -e '.status=="PASS_NATURAL_PENDING_REPLAY"' "runs/$label/worker.json" > /dev/null; then
            run replay "$encoding" D "$target" "$label-replay" "$root/runs/$label/worker.json"
            jq -e '.status=="PASS"' "runs/$label-replay/worker.json" > /dev/null
        fi
    else
        exit "$result"
    fi
)
natural per-representative 0 t0-a1
natural shared-bits 0 t0-b1
natural shared-bits 0 t0-b2
natural per-representative 0 t0-a2
for target in 1 2 3 4 5 6 7; do
    if [ "$((target % 2))" = 1 ]; then order='shared-bits per-representative'; else order='per-representative shared-bits'; fi
    for encoding in $order; do
        natural "$encoding" "$target" "t$target-$encoding"
    done
done
date -u +%FT%TZ > card-end.utc
printf '%s\n' COMPLETE_BOUNDED > card-status.txt
