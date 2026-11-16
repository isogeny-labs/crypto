#!/usr/bin/env bash
# Offline contract test: four exact specs, unique requests, and fail-closed continuation.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
mkdir -p "$work/bin/remote" "$work/bin/resume" "$work/fakebin"
cp "$here/run-ic-sweep" "$work/bin/run-ic-sweep"
cat > "$work/bin/remote/validate-request" <<'SH'
#!/usr/bin/env bash
jq -e '.schema=="isolab.eks.request/v1" and .job.build.source_revision==.provenance.source_revision and .job.spec.workloads.targets_per_curve==1' "$1" >/dev/null
cat "$1"
SH
cat > "$work/bin/remote/launch-remote" <<'SH'
#!/usr/bin/env bash
bits=$(jq -r '.job.spec.workloads.curves[0].bits' "$1")
if [[ "$bits" == "${FAKE_FAIL_BITS:-}" ]]; then exit 42; fi
mkdir -p "$ISOLAB_CLIENT_OUTPUT"
jq -n --arg name "$(jq -r .job.job_name "$1")" '{job_name:$name,state:"succeeded"}' > "$ISOLAB_CLIENT_OUTPUT/result.json"
SH
cat > "$work/fakebin/curl" <<'SH'
#!/usr/bin/env bash
url=${!#}
[[ "$url" =~ /n(20|24|28|32)(-resumable)?\.json\?ref= ]] || exit 22
jq -n --rawfile spec "$FAKE_SPEC_DIR/n${BASH_REMATCH[1]}${BASH_REMATCH[2]}.json" '{content:($spec|@base64)}'
SH
cat > "$work/bin/resume/fetch-client" <<'SH'
#!/usr/bin/env bash
mkdir -p "$1"
cat > "$1/isolab-eks-recovery" <<'CLIENT'
#!/usr/bin/env bash
case "$1" in
  validate) jq -e '.schema=="isolab.eks.resumable/v1" and .instance_type=="c8a.2xlarge"' "$2" >/dev/null ;;
  submit) mkdir -p "$3"; cp "$2" "$3/submitted.json" ;;
  wait) jq -n '{state:{phase:"succeeded"}}' > "$3/result.json" ;;
  *) exit 2 ;;
esac
CLIENT
chmod +x "$1/isolab-eks-recovery"
SH
chmod +x "$work/bin/run-ic-sweep" "$work/bin/remote/validate-request" "$work/bin/remote/launch-remote" "$work/bin/resume/fetch-client" "$work/fakebin/curl"
export PATH="$work/fakebin:$PATH" FAKE_SPEC_DIR="$here/specs/ic-size-sweep"
export GH_TOKEN=fake GITHUB_REPOSITORY=example/crypto GITHUB_RUN_ID=123456789 GITHUB_RUN_ATTEMPT=1
export GITHUB_SHA=aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa GITHUB_SERVER_URL=https://github.example.test
export GITHUB_API_URL=https://api.github.example.test ISOLAB_SWEEP_INSTANCE_TYPE=c8a.2xlarge
export ISOLAB_SWEEP_SECONDS=1800 ISOLAB_SWEEP_RESUMABLE=false ISOLAB_SWEEP_CHECKPOINT_SECONDS=120 ISOLAB_SWEEP_MAX_RESTORES=2
build="$work/build.json"
jq -n '{image_uri:"example.invalid@sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",source_revision:"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",ecbench_sha256:"bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"}' > "$build"
export RUNNER_TEMP="$work/success" GITHUB_STEP_SUMMARY="$work/success-summary.md"
mkdir -p "$RUNNER_TEMP"
"$work/bin/run-ic-sweep" "$GITHUB_SHA" "$build"
for bits in 20 24 28 32; do
  cmp "$here/specs/ic-size-sweep/n$bits.json" "$RUNNER_TEMP/isolab-sweep/n$bits/spec.json"
  jq -e --arg name "gh-$GITHUB_RUN_ID-$GITHUB_RUN_ATTEMPT-n$bits" '.job.job_name==$name' "$RUNNER_TEMP/isolab-sweep/n$bits/request.json" >/dev/null
done
[[ $(grep -c 'succeeded' "$GITHUB_STEP_SUMMARY") == 4 ]]
export RUNNER_TEMP="$work/failure" GITHUB_STEP_SUMMARY="$work/failure-summary.md" FAKE_FAIL_BITS=28
mkdir -p "$RUNNER_TEMP"
if "$work/bin/run-ic-sweep" "$GITHUB_SHA" "$build"; then
  echo 'sweep accepted a failed size' >&2
  exit 1
fi
[[ -f "$RUNNER_TEMP/isolab-sweep/n32/request.json" ]]
grep -q '| 28 | failed |' "$GITHUB_STEP_SUMMARY"
export RUNNER_TEMP="$work/recovery" GITHUB_STEP_SUMMARY="$work/recovery-summary.md" ISOLAB_SWEEP_RESUMABLE=true
unset FAKE_FAIL_BITS
mkdir -p "$RUNNER_TEMP"
jq -n '{runtime_image_uri:"example.invalid@sha256:cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc"}' > "$RUNNER_TEMP/isolab-recovery-image.json"
"$work/bin/run-ic-sweep" "$GITHUB_SHA" "$build"
for bits in 20 24 28 32; do
  jq -e --arg name "gh-$GITHUB_RUN_ID-$GITHUB_RUN_ATTEMPT-n$bits" '.schema=="isolab.eks.resumable/v1" and .run_name==$name and .max_restores==2' "$RUNNER_TEMP/isolab-sweep/n$bits/request.json" >/dev/null
  cmp "$RUNNER_TEMP/isolab-sweep/n$bits/request.json" "$RUNNER_TEMP/isolab-sweep/n$bits/results/submitted.json"
done
cmp "$here/specs/ic-size-sweep/n32-resumable.json" "$RUNNER_TEMP/isolab-sweep/n32/spec.json"
echo 'IC sweep offline contract passed'
