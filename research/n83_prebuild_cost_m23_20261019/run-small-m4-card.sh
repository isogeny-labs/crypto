#!/bin/sh
set -eu
if [ "$#" -ne 1 ]; then echo 'usage: run-small-m4-card.sh ABSOLUTE_FROZEN_RUN_ROOT' >&2; exit 2; fi
root=$(realpath "$1")
cd "$root"
sha256sum -c inputs.sha256 > input-check.log
revision=$(cat source-commit.txt)
output="$root/runs"
test ! -e "$output"
mkdir "$output"
date -u +%FT%TZ > card-start.utc
uname -a > host.txt
uptime > host-uptime.txt
lscpu > host-cpu.txt
sudo docker ps --no-trunc > host-containers.txt
trap 'result=$?; if [ "$result" -ne 0 ]; then printf "FAILED_OR_INTERRUPTED exit=%s\n" "$result" > card-status.txt; fi' EXIT
run() {
    mode=$1 arm=$2 label=$3
    shift 3
    if [ "$mode" = replay ]; then binary="$root/replayer"; else binary="$root/worker"; fi
    sh "$root/run-small-m4-docker.sh" "$mode" "$arm" "$output/$label" "$binary" \
        "/home/ec2-user/n83-m23-20261019/inputs/$arm" \
        /home/ec2-user/n83-m23-20261019/inputs/public/public-targets.json "$revision" "$label" "$@"
}
for arm in K2048 D12; do
    run preflight "$arm" "preflight-$arm"
    jq -e '.status=="PASS_PREFLIGHT"' "$output/preflight-$arm/worker.json" > /dev/null
done
for arm in K2048 D12; do
    run positive "$arm" "positive-$arm"
    jq -e '.status=="PASS_CONSTRUCTED_PENDING_REPLAY"' "$output/positive-$arm/worker.json" > /dev/null
    run replay "$arm" "positive-$arm-replay" "$output/positive-$arm/worker.json"
    jq -e '.status=="PASS"' "$output/positive-$arm-replay/worker.json" > /dev/null
done
for pair in K2048:k1 D12:d1 D12:d2 K2048:k2; do
    arm=${pair%:*} label=${pair#*:}
    result=0
    run natural "$arm" "$label" || result=$?
    if [ "$result" = 124 ]; then
        jq -e '.status=="UNKNOWN_budget"' "$output/$label/worker.json.cap.json" > /dev/null
    elif [ "$result" = 0 ]; then
        jq -e '.status=="UNKNOWN_search_cap" or .status=="UNVERIFIED_MODEL_UNSAT" or .status=="PASS_NATURAL_PENDING_REPLAY"' "$output/$label/worker.json" > /dev/null
        if jq -e '.status=="PASS_NATURAL_PENDING_REPLAY"' "$output/$label/worker.json" > /dev/null; then
            run replay "$arm" "$label-replay" "$output/$label/worker.json"
            jq -e '.status=="PASS"' "$output/$label-replay/worker.json" > /dev/null
        fi
    else
        exit "$result"
    fi
done
date -u +%FT%TZ > card-end.utc
printf '%s\n' COMPLETE_BOUNDED > card-status.txt
