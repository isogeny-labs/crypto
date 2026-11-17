#!/usr/bin/env bash
set -euo pipefail
umask 077
cd /home/ec2-user/ecbench-e2e-20261030-v16
api=https://api.runpod.io/v2
pod_id=
api_get() { curl -sS --fail-with-body --max-time 30 --config private/auth.curl "$api/$1"; }
cleanup() {
    local prior=$? code=0
    trap - EXIT INT TERM
    if [[ -n "$pod_id" ]]; then
        for attempt in 1 2 3; do
            code=$(curl -sS --max-time 30 --config private/auth.curl -X DELETE -o delete-response.json -w '%{http_code}' "$api/pods/$pod_id") || code=0
            if [[ "$code" == 200 || "$code" == 204 || "$code" == 404 ]]; then break; fi
            sleep 5
        done
        api_get pods > private/pods-after.json || true
        if jq -e --arg id "$pod_id" '.pagination.hasNextPage == false and all(.pods[]; .id != $id)' private/pods-after.json >/dev/null; then
            printf 'DELETION_VERIFIED %s\n' "$pod_id" | tee deletion.txt
        else
            printf 'DELETION_UNVERIFIED %s HTTP=%s\n' "$pod_id" "$code" | tee deletion.txt
            prior=90
        fi
    fi
    printf '%s\n' "$prior" > controller.exit
    exit "$prior"
}
trap cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM
name=$(jq -r .name request.json)
api_get pods > private/pods-before-create.json
jq -e --arg name "$name" '.pagination.hasNextPage == false and all(.pods[]; .name != $name)' private/pods-before-create.json >/dev/null
started=$(date +%s)
printf '%s\n' "$started" > launch-epoch.txt
create_code=$(curl -sS --max-time 45 --config private/auth.curl -H 'Content-Type: application/json' -X POST --data-binary @request.json -o private/create-response.json -w '%{http_code}' "$api/pods") || create_code=0
pod_id=$(jq -r '.id // .pod.id // empty' private/create-response.json 2>/dev/null || true)
if [[ -z "$pod_id" ]]; then
    # Reconcile an ambiguous response; never blindly retry the POST.
    api_get pods > private/pods-reconcile.json
    matches=$(jq --arg name "$name" '[.pods[]|select(.name==$name)]|length' private/pods-reconcile.json)
    if [[ "$matches" == 1 ]]; then
        pod_id=$(jq -r --arg name "$name" '.pods[]|select(.name==$name)|.id' private/pods-reconcile.json)
    else
        printf 'CREATE_FAILED_OR_AMBIGUOUS HTTP=%s matches=%s; inspect private response\n' "$create_code" "$matches"
        exit 91
    fi
fi
[[ "$pod_id" =~ ^[a-zA-Z0-9_-]+$ ]]
printf '%s\n' "$pod_id" > pod-id
printf 'POD_CREATED %s\n' "$pod_id"
api_get "pods/$pod_id" > private/pod-initial.json
jq '{id,name,status,gpu,image,cudaVersion,cost,dataCenterId,createdAt}' private/pod-initial.json > receipt.json
cat receipt.json
jq -e '.cost <= 2.50 and .gpu.count == 1 and .gpu.id == "NVIDIA RTX PRO 6000 Blackwell Server Edition"' private/pod-initial.json >/dev/null
while (( $(date +%s) - started < 18300 )); do
    # SSE remains open after backfill; curl timeout is expected here.
    curl -sS --max-time 20 --config private/auth.curl "$api/pods/$pod_id/logs?tail=5000" > logs-latest.sse 2>logs-curl.stderr || true
    sed -n 's/^data: //p' logs-latest.sse | jq -Rr 'fromjson? | .line // empty' > logs-latest.txt
    if rg -q '^ECBENCH_DONE ' logs-latest.txt; then
        cp logs-latest.sse logs-final.sse
        cp logs-latest.txt logs-final.txt
        (eval "$(aws configure export-credentials --format env)"; aws s3 cp s3://setec-astronomy-artifacts/validation/ecbench-explicit-x/e2e-20261030-v16/results.tgz results.tgz --only-show-errors)
        expected=$(sed -n 's/^ECBENCH_ARCHIVE_SHA256 //p' logs-final.txt | tail -1)
        actual=$(sha256sum results.tgz | cut -d ' ' -f1)
        test "$expected" = "$actual"
        mkdir -p results
        tar -xzf results.tgz -C results
        sha256sum results.tgz logs-final.sse > evidence.sha256
        cat results/bootstrap.exit
        rg '^ECBENCH_DONE ' logs-final.txt
        test "$(cat results/bootstrap.exit)" = 0
        test "$(cat results/validation.exit)" = 0
        printf 'VALIDATION_PASSED\n'
        exit 0
    fi
    api_get "pods/$pod_id" > private/pod-current.json
    jq -r '[.id,.status,(.runtime.uptime // "")] | @tsv' private/pod-current.json
    status=$(jq -r .status private/pod-current.json)
    if [[ "$status" == EXITED || "$status" == TERMINATED ]]; then
        cp logs-latest.txt logs-final.txt
        printf 'POD_STOPPED_WITHOUT_COMPLETION\n'
        exit 92
    fi
    sleep 10
done
cp logs-latest.txt logs-final.txt
printf 'CONTROLLER_DEADLINE\n'
exit 124
