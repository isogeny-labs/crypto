#!/bin/sh
set -eu
test "$#" -eq 1
root=$(realpath "$1")
cd "$root"
test ! -e runs
sha256sum -c inputs.sha256 > input-check.log
revision=$(cat source-commit.txt)
mkdir runs
date -u +%FT%TZ > card-start.utc
uname -a > host.txt
uptime > host-uptime.txt
lscpu > host-cpu.txt
sudo docker ps --no-trunc > host-containers.txt
trap 'result=$?; if [ "$result" -ne 0 ]; then printf "FAILED_OR_INTERRUPTED exit=%s\n" "$result" > card-status.txt; fi' EXIT
run() {
    mode=$1 cache=$2 arm=$3 target=$4 label=$5
    shift 5
    if [ "$mode" = replay ]; then binary="$root/replayer"; else binary="$root/worker"; fi
    sh "$root/run-xor-cache-docker.sh" "$mode" "$cache" "$arm" "$target" "$root/runs/$label" "$binary" \
        "/home/ec2-user/n83-m23-20261019/inputs/$arm" \
        /home/ec2-user/n83-m23-20261019/inputs/public/public-targets.json "$revision" "$label" "$@"
}
for arm in K2048 D12; do
    for cache in off on; do
        label="positive-$arm-$cache"
        run positive "$cache" "$arm" 0 "$label"
        jq -e '.status=="PASS_CONSTRUCTED_PENDING_REPLAY"' "runs/$label/worker.json" > /dev/null
        run replay "$cache" "$arm" 0 "$label-replay" "$root/runs/$label/worker.json"
        jq -e '.status=="PASS"' "runs/$label-replay/worker.json" > /dev/null
    done
done
for cell in K2048:0 D12:0 K2048:1 D12:1; do
    arm=${cell%:*} target=${cell#*:}
    for pair in off:a1 on:b1 on:b2 off:a2; do
        cache=${pair%:*} repeat=${pair#*:}
        label="$arm-t$target-$repeat"
        result=0
        run natural "$cache" "$arm" "$target" "$label" || result=$?
        test -s "runs/$label/worker.json.progress.jsonl"
        jq -s -e --arg arm "$arm" --argjson target "$target" --arg cache "$cache" 'length>0 and all(.[];
            .schema=="n83.small-m4-progress/v1" and .identity.arm==$arm and .identity.target_index==$target and
            .identity.experiment_policy=="small-m4-xor-cache/v1" and .identity.xor_decided_cache==($cache=="on") and
            .identity.conflict_budget==10000 and .identity.mode=="natural" and
            (.counters.completed_conflicts|type)=="number")' "runs/$label/worker.json.progress.jsonl" > /dev/null
        if [ "$result" = 124 ]; then
            jq -e '.status=="UNKNOWN_budget"' "runs/$label/worker.json.cap.json" > /dev/null
        elif [ "$result" = 0 ]; then
            jq -e '(.status=="UNKNOWN_search_cap" and .counters.completed_conflicts==10000) or .status=="UNVERIFIED_MODEL_UNSAT" or .status=="PASS_NATURAL_PENDING_REPLAY"' "runs/$label/worker.json" > /dev/null
            if jq -e '.status=="PASS_NATURAL_PENDING_REPLAY"' "runs/$label/worker.json" > /dev/null; then
                run replay "$cache" "$arm" "$target" "$label-replay" "$root/runs/$label/worker.json"
                jq -e '.status=="PASS"' "runs/$label-replay/worker.json" > /dev/null
            fi
        else
            exit "$result"
        fi
    done
done
date -u +%FT%TZ > card-end.utc
printf '%s\n' COMPLETE_BOUNDED > card-status.txt
