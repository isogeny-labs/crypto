#!/usr/bin/env bash
# Artifact transport only. The frozen native binaries produce and verify data.
set -euo pipefail

[[ ${GITHUB_RUN_ID:-} =~ ^[0-9]+$ ]]
[[ ${GITHUB_RUN_ATTEMPT:-} =~ ^[0-9]+$ ]]
delivery_root=artifacts/tr04-delivery
output="$delivery_root/run-$GITHUB_RUN_ID-attempt-$GITHUB_RUN_ATTEMPT"
mkdir -p "$delivery_root"
mkdir "$output" # Every invocation retains its own evidence; never replace it.
command -v jq >/dev/null
stage=source_check
archive_exit=null
push_exit=null
readback_exit=null
compare_exit=null
archive_sha=
archive_bytes=null
remote=
readback_directory=
readback_cache=
started=$(date -u +%FT%TZ)
finish() {
  local delivery_exit=$?
  trap - EXIT
  jq -n --arg started "$started" --arg finished "$(date -u +%FT%TZ)" \
    --arg source "${TR04_SOURCE_REVISION:-}" --arg stage "$stage" \
    --arg remote "$remote" --arg sha "$archive_sha" --argjson bytes "$archive_bytes" \
    --argjson delivery_exit "$delivery_exit" --argjson archive_exit "$archive_exit" \
    --argjson push_exit "$push_exit" --argjson readback_exit "$readback_exit" \
    --argjson compare_exit "$compare_exit" --arg readback_directory "$readback_directory" \
    --arg readback_cache "$readback_cache" \
    --arg aggregate "${TR04_AGGREGATE_OUTCOME:-unknown}" \
    --arg checkpoint "${TR04_CHECKPOINT_OUTCOME:-unknown}" \
    '{schema:"twist-tr04.delivery/v1",started_at:$started,finished_at:$finished,
      source_revision:$source,stage:$stage,remote:$remote,
      archive:{path:"runs.tar.xz",sha256:(if $sha=="" then null else $sha end),bytes:$bytes},
      delivery_exit_code:$delivery_exit,archive_exit_code:$archive_exit,
      push_exit_code:$push_exit,readback_exit_code:$readback_exit,
      byte_compare_exit_code:$compare_exit,
      independent_remote_download_verified:($push_exit==0 and $readback_exit==0 and $compare_exit==0),
      readback_directory:$readback_directory,readback_cache:$readback_cache,
      aggregate_step_outcome:$aggregate,aggregate_checkpoint_outcome:$checkpoint,
      status:(if $delivery_exit==0 then "uploaded_and_download_verified" else "failed" end),
      scope:"Artifact delivery only; prior job failures and finite-census verification retain their own status"}' \
    > "$output/delivery.json.tmp"
  mv "$output/delivery.json.tmp" "$output/delivery.json"
  exit "$delivery_exit"
}
trap finish EXIT

[[ ${TR04_SOURCE_REVISION:-} =~ ^[a-f0-9]{40}$ ]]
[[ $(git rev-parse HEAD) == "$TR04_SOURCE_REVISION" ]]
[[ -z $(git status --porcelain --untracked-files=no) ]]
protocol=research/twist_relations_tr04_20261029/protocol.json
tool=tools/artifacts/artifacts.py
policy=.artifacts/config.json
test -f "$protocol"
test -f "$tool"
test -f "$policy"
remote=$(jq -er .remote "$policy")
[[ "$remote" == s3://setec-astronomy-artifacts/store ]]
export ARTIFACTS_REMOTE="$remote"
sha256sum "$protocol" "$tool" "$policy" tools/twist-relations/run-tr04-delivery.sh \
  > "$output/delivery-source.sha256"

stage=archive
inputs=()
inventory='[]'
for input in artifacts/tr04-preflight-input artifacts/tr04-inputs artifacts/tr04-summary; do
  present=false
  if [[ -d "$input" ]]; then inputs+=("$input"); present=true; fi
  inventory=$(jq -c --arg path "$input" --argjson present "$present" \
    '. + [{path:$path,present:$present}]' <<< "$inventory")
done
jq -n --arg source "$TR04_SOURCE_REVISION" --argjson inputs "$inventory" \
  --arg run "$GITHUB_RUN_ID" --arg attempt "$GITHUB_RUN_ATTEMPT" \
  '{schema:"twist-tr04.archive-inputs/v1",source_revision:$source,run_id:$run,
    run_attempt:$attempt,inputs:$inputs,
    scope:"Actual retained directories only. Absent directories and primary attempts are not invented."}' \
  > "$output/archive-inputs.json"
inputs+=("$protocol" "$output/archive-inputs.json" "$output/delivery-source.sha256")
archive="$output/runs.tar.xz"
printf '%q ' tar --sort=name --format=gnu --mtime=@0 --owner=0 --group=0 --numeric-owner \
  '--mode=a=rX,u+w' -cf - "${inputs[@]}" > "$output/archive-command.txt"
printf ' | xz -T1 -6 > runs.tar.xz\n' >> "$output/archive-command.txt"
if LC_ALL=C tar --sort=name --format=gnu --mtime=@0 --owner=0 --group=0 --numeric-owner \
    '--mode=a=rX,u+w' -cf - "${inputs[@]}" | xz -T1 -6 > "$archive.tmp"; then
  archive_exit=0
else
  archive_exit=$?
  exit "$archive_exit"
fi
mv "$archive.tmp" "$archive"
archive_sha=$(sha256sum "$archive" | cut -d ' ' -f 1)
archive_bytes=$(stat -c %s "$archive")
printf '%s  runs.tar.xz\n' "$archive_sha" > "$output/runs.tar.xz.sha256"
cat > "$output/README.md" <<'EOF'
# TR-04 immutable run archive

`delivery.json` records actual archive, upload, fresh-download and comparison
exits. Its infrastructure status does not replace the aggregate or field outcomes.
The archive includes retained preflight, field and aggregate records, even when
partial, plus the frozen protocol and archive input/source fingerprints. An
absent directory or missing execution is explicitly recorded, never fabricated.

The configured destination is the content-addressed artifact store. Storage
and fresh-download success are established only by the actual `delivery.json`
exits. Retain this directory's `.artifacts.json` and managed `.gitignore` when
created, together with its checksum and receipts, in the result PR. After a
successful delivery, hydrate with the maintained artifact-store pull command
and configured read access before extracting:

```sh
python3 tools/artifacts/artifacts.py pull PATH/TO/THIS/DIRECTORY
(cd PATH/TO/THIS/DIRECTORY && sha256sum -c runs.tar.xz.sha256)
tar -xJf PATH/TO/THIS/DIRECTORY/runs.tar.xz
```

`archive-command.txt` describes deterministic GNU tar/xz packing (mtime/owner
zero, normalized modes, sorted names, one xz thread). Do not rewrite raw bytes
to relocate absolute host paths. Portable report/export files retain relative
compact evidence paths; full raw manifests retain their original references.
Record any relocation map separately before a new aggregate/replay. Rebuild
the native verifier from the recorded source, preserve every failure and inspect
every PDF page before publishing a result. No valid clock or solving gain is
established by successful storage.
EOF

stage=credentials
[[ -n ${AWS_ACCESS_KEY_ID:-} && -n ${AWS_SECRET_ACCESS_KEY:-} ]]
export AWS_DEFAULT_REGION="${AWS_REGION:-us-west-2}"
if ! command -v aws >/dev/null; then
  python3 -m venv "$RUNNER_TEMP/tr04-artifacts-awscli"
  "$RUNNER_TEMP/tr04-artifacts-awscli/bin/python" -m pip install \
    --disable-pip-version-check awscli==1.46.2 \
    > "$output/awscli-install.log" 2>&1
  export ARTIFACTS_AWS="$RUNNER_TEMP/tr04-artifacts-awscli/bin/aws"
fi

stage=push
# Explicit files include newly produced, untracked archives. A directory push
# would only examine tracked data and could silently miss this new archive.
if python3 "$tool" push "$archive" --jobs 1 > "$output/push.stdout.log" 2> "$output/push.stderr.log"; then
  push_exit=0
else
  push_exit=$?
  exit "$push_exit"
fi
jq -e --arg sha "$archive_sha" --argjson bytes "$archive_bytes" \
  '.files["runs.tar.xz"].sha256==$sha and .files["runs.tar.xz"].bytes==$bytes' \
  "$output/.artifacts.json" >/dev/null

stage=readback
# Both destination and cache are fresh. Pulling the original hydrated file,
# or merely HEAD-verifying S3, would not establish an independent download.
readback_directory=$(mktemp -d "$PWD/artifacts/tr04-readback.XXXXXX")
readback_cache=$(mktemp -d "$RUNNER_TEMP/tr04-artifacts-cache.XXXXXX")
cp "$output/.artifacts.json" "$readback_directory/.artifacts.json"
# The maintained tool discovers tracked/staged manifests, including when its
# requested directory is explicit. Stage only this ephemeral pointer, never
# the downloaded archive. This checkout is not committed or pushed by delivery.
git add -f --sparse -- "$readback_directory/.artifacts.json"
if ARTIFACTS_CACHE="$readback_cache" ARTIFACTS_NO_READ_URL=1 \
    python3 "$tool" pull "$readback_directory" --jobs 1 \
    > "$output/readback.stdout.log" 2> "$output/readback.stderr.log"; then
  readback_exit=0
else
  readback_exit=$?
  exit "$readback_exit"
fi
stage=compare
sha256sum "$readback_directory/runs.tar.xz" > "$output/readback.sha256"
if cmp -s "$archive" "$readback_directory/runs.tar.xz"; then
  compare_exit=0
else
  compare_exit=$?
  exit "$compare_exit"
fi
stage=complete
