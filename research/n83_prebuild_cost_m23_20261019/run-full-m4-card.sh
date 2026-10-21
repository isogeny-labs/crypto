#!/bin/sh
set -eu
test "$#" -eq 1
root=$(realpath "$1")
cd "$root"
test ! -e runs
sha256sum -c inputs.sha256 > input-check.log
jq -e '.schema=="n83.xor-cache-card-audit/v1" and .status=="PASS_RECEIPT_AUDIT" and (.natural_attempts|length)==16 and (.comparisons|length)==4 and all(.comparisons[]; .status=="PASS_MATCHED_SEARCH_ENDPOINT")' cache-audit.json > /dev/null
revision=$(cat source-commit.txt)
mkdir runs
date -u +%FT%TZ > card-start.utc
uname -a > host.txt
uptime > host-uptime.txt
lscpu > host-cpu.txt
sudo docker ps --no-trunc > host-containers.txt
trap 'result=$?; if [ "$result" -ne 0 ]; then printf "FAILED_OR_INTERRUPTED exit=%s\n" "$result" > card-status.txt; fi' EXIT
run() (
    mode=$1 cache=$2 arm=$3 target=$4 label=$5
    shift 5
    if [ "$mode" = replay ]; then binary="$root/replayer"; else binary="$root/worker"; fi
    sh "$root/run-full-m4-docker.sh" "$mode" "$cache" "$arm" "$target" "$root/runs/$label" "$binary" \
        "/home/ec2-user/n83-m23-20261019/inputs/$arm" \
        /home/ec2-user/n83-m23-20261019/inputs/public/public-targets.json "$revision" "$label" "$@"
)
for arm in U D; do
    for cache in off on; do
        label="positive-$arm-$cache"
        run positive "$cache" "$arm" 0 "$label"
        jq -e '.status=="PASS_CONSTRUCTED_PENDING_REPLAY"' "runs/$label/worker.json" > /dev/null
        run replay "$cache" "$arm" 0 "$label-replay" "$root/runs/$label/worker.json"
        jq -e '.status=="PASS"' "runs/$label-replay/worker.json" > /dev/null
    done
    "$root/worker" full-m4-control-check \
        "runs/positive-$arm-off/worker.json" "runs/positive-$arm-on/worker.json" \
        "runs/positive-$arm-off-replay/worker.json" "runs/positive-$arm-on-replay/worker.json" \
        "control-pair-$arm.json"
    jq -e '.status=="PASS"' "control-pair-$arm.json" > /dev/null
done
natural() (
    arm=$1 target=$2 label=$3 result=0
    run natural on "$arm" "$target" "$label" || result=$?
    test -s "runs/$label/worker.json.progress.jsonl"
    jq -s -e --arg arm "$arm" --argjson target "$target" 'length>0 and all(.[];
        .schema=="n83.small-m4-progress/v1" and .identity.arm==$arm and .identity.target_index==$target and
        .identity.experiment_policy=="full-m4-common-public/v1" and .identity.xor_decided_cache==true and
        .identity.conflict_budget==10000 and .identity.mode=="natural" and
        (.counters.completed_conflicts|type)=="number")' "runs/$label/worker.json.progress.jsonl" > /dev/null
    if [ "$result" = 124 ]; then
        jq -e '.status=="UNKNOWN_budget"' "runs/$label/worker.json.cap.json" > /dev/null
    elif [ "$result" = 0 ]; then
        jq -e '(.status=="UNKNOWN_search_cap" and .counters.completed_conflicts==10000) or .status=="UNVERIFIED_MODEL_UNSAT" or .status=="PASS_NATURAL_PENDING_REPLAY"' "runs/$label/worker.json" > /dev/null
        if jq -e '.status=="PASS_NATURAL_PENDING_REPLAY"' "runs/$label/worker.json" > /dev/null; then
            run replay on "$arm" "$target" "$label-replay" "$root/runs/$label/worker.json"
            jq -e '.status=="PASS"' "runs/$label-replay/worker.json" > /dev/null
        fi
    else
        exit "$result"
    fi
)
natural U 0 t0-u1
natural D 0 t0-d1
natural D 0 t0-d2
natural U 0 t0-u2
for target in 1 2 3 4 5 6 7; do
    if [ "$((target % 2))" = 1 ]; then order='D U'; else order='U D'; fi
    for arm in $order; do
        natural "$arm" "$target" "t$target-$arm"
    done
done
date -u +%FT%TZ > card-end.utc
printf '%s\n' COMPLETE_BOUNDED > card-status.txt
