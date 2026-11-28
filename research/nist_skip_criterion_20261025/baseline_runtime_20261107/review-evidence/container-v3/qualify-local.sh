#!/usr/bin/env bash
# Local, finite development controls only. Never pushes an image or samples a field.
set -euo pipefail
[[ $# == 2 && $1 == /* && $2 == /* ]] || {
    echo 'Usage: qualify-local.sh /ABS/STAGED/CONTEXT /ABS/FRESH/EVIDENCE' >&2; exit 2;
}
nist_context=$1
nist_evidence=$2
nist_here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
nist_repo=$(cd -- "$nist_here/../../.." && pwd)
cmp "$nist_here/Dockerfile" "$nist_context/Dockerfile"
(cd "$nist_context/root" && sha256sum -c /dev/stdin < "$nist_context/receipts/PAYLOAD.sha256")
mkdir -- "$nist_evidence"
cp -a "$nist_context/receipts" "$nist_evidence/staging"
(
    cd "$nist_repo"
    sha256sum research/nist_skip_criterion_20261025/baseline_runtime_20261107/{known-orders.cpp,run-field,test-launcher.sh,prepare-container.sh,qualify-local.sh,Dockerfile}
) > "$nist_evidence/CONTROL-SOURCE.sha256"
# The managed daemon is already running; preserve authentication configuration.
unset DOCKER_HOST DOCKER_CONTEXT DOCKER_TLS DOCKER_TLS_VERIFY DOCKER_CERT_PATH
nist_docker=(docker --host=unix:///var/run/docker.sock)
"${nist_docker[@]}" version --format '{{json .Client.Version}} {{json .Server.Version}}' > "$nist_evidence/docker-version.txt"
uname -sm > "$nist_evidence/host-architecture.txt"
set +e
timeout --kill-after=5s 300s "${nist_docker[@]}" build --network=none \
    --iidfile "$nist_evidence/image-id.txt" "$nist_context" \
    > "$nist_evidence/build.stdout" 2> "$nist_evidence/build.stderr"
nist_build_status=$?
set -e
printf '%s\n' "$nist_build_status" > "$nist_evidence/build.exit"
[[ $nist_build_status == 0 ]] || exit "$nist_build_status"
nist_image=$(cat "$nist_evidence/image-id.txt")
"${nist_docker[@]}" image inspect "$nist_image" \
    --format '{"image_config_digest":{{json .Id}},"repo_digests":{{json .RepoDigests}},"architecture":{{json .Architecture}},"os":{{json .Os}}}' \
    > "$nist_evidence/image.json"
nist_active_cid=
cleanup() {
    if [[ -n $nist_active_cid ]]; then
        "${nist_docker[@]}" kill "$nist_active_cid" >/dev/null 2>&1 || true
        "${nist_docker[@]}" logs "$nist_active_cid" > "$nist_case_dir/stdout" 2> "$nist_case_dir/stderr" || true
        "${nist_docker[@]}" rm "$nist_active_cid" >/dev/null 2>&1 || true
    fi
}
trap cleanup EXIT
nist_any_failed=0
for nist_case in journal-controls known-orders; do
    nist_case_dir=$nist_evidence/$nist_case
    mkdir "$nist_case_dir" "$nist_case_dir/output"
    if [[ $nist_case == journal-controls ]]; then
        nist_command='/opt/nist/bin/journal_tests /opt/nist/source/strict_elkies_20261029/inputs.tsv /work/compute/toy-fixtures && bash /opt/nist/source/journal_qualification_20261104/test-cli.sh /opt/nist/bin && bash /opt/nist/source/baseline_runtime_20261107/test-launcher.sh'
    else
        nist_command='/opt/nist/bin/known-orders /opt/nist/source/strict_elkies_20261029/inputs.tsv'
    fi
    nist_command='cd / && sha256sum -c /opt/nist/PAYLOAD.sha256 && '"$nist_command"
    printf '%s\n' "$nist_command" > "$nist_case_dir/command.txt"
    "${nist_docker[@]}" create --network=none --read-only --cap-drop=ALL \
        --label=org.isogeny.nist60000.qualification=local-development \
        --security-opt=no-new-privileges --pids-limit=128 --cpus=2 --memory=2g \
        --user=1000:1000 --tmpfs=/tmp:rw,noexec,nosuid,nodev,size=128m,uid=1000,gid=1000 \
        --mount "type=bind,src=$nist_case_dir/output,dst=/work/compute" \
        "$nist_image" /bin/bash -c "$nist_command" > "$nist_case_dir/container-id.txt"
    nist_active_cid=$(cat "$nist_case_dir/container-id.txt")
    "${nist_docker[@]}" start "$nist_active_cid" > "$nist_case_dir/start.stdout"
    set +e
    timeout --kill-after=5s 180s "${nist_docker[@]}" wait "$nist_active_cid" \
        > "$nist_case_dir/wait.stdout" 2> "$nist_case_dir/wait.stderr"
    nist_wait_status=$?
    set -e
    printf '%s\n' "$nist_wait_status" > "$nist_case_dir/wait.exit"
    if [[ $nist_wait_status != 0 ]]; then
        "${nist_docker[@]}" kill "$nist_active_cid" > "$nist_case_dir/kill.stdout" 2> "$nist_case_dir/kill.stderr" || true
        "${nist_docker[@]}" wait "$nist_active_cid" > "$nist_case_dir/killed-exit.txt" || true
        nist_any_failed=1
    fi
    "${nist_docker[@]}" logs "$nist_active_cid" > "$nist_case_dir/stdout" 2> "$nist_case_dir/stderr"
    "${nist_docker[@]}" inspect "$nist_active_cid" \
        --format '{"status":{{json .State.Status}},"exit_code":{{json .State.ExitCode}},"oom_killed":{{json .State.OOMKilled}},"error":{{json .State.Error}},"started_at":{{json .State.StartedAt}},"finished_at":{{json .State.FinishedAt}}}' \
        > "$nist_case_dir/state.json"
    [[ $(jq -r '.exit_code' "$nist_case_dir/state.json") == 0 ]] || nist_any_failed=1
    (
        cd "$nist_case_dir/output"
        find . -type f -print0 | sort -z | xargs -0 -r sha256sum
    ) > "$nist_case_dir/OUTPUT.sha256"
    "${nist_docker[@]}" rm "$nist_active_cid" > "$nist_case_dir/remove.stdout"
    nist_active_cid=
    printf '%s: ' "$nist_case"
    cat "$nist_case_dir/state.json"
done
exit "$nist_any_failed"
