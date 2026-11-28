#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
: "${S3_VALIDATOR_ARM:?set S3_VALIDATOR_ARM to local or github-hosted}"
case "$S3_VALIDATOR_ARM" in local|github-hosted) ;; *) echo 'Unknown arm' >&2; exit 2;; esac
output=${1:?set output directory}
if [ -e "$output" ]; then echo 'Refusing to overwrite existing output.' >&2; exit 2; fi
sha256sum --check frozen-files.sha256
node check-protocol.mjs
mkdir -p "$output"
output=$(cd "$output" && pwd)
date -u +%s > "$output/start-epoch"
set +e
timeout 60s rustc --edition=2021 -C opt-level=3 -C overflow-checks=yes -D warnings \
  src/main.rs -o "$output/validator" > "$output/build.stdout" 2> "$output/build.stderr"
build_rc=$?
set -e
printf '%s\n' "$build_rc" > "$output/build.exit"
export S3_VALIDATOR_OUTPUT="$output"
export S3_VALIDATOR_RUST_VERSION
S3_VALIDATOR_RUST_VERSION=$(rustc --version)
export S3_VALIDATOR_GIT_SHA
S3_VALIDATOR_GIT_SHA=$(git rev-parse HEAD)
node - <<'JS'
const fs=require('fs'),os=require('os'),crypto=require('crypto');
const sha=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const dir=process.env.S3_VALIDATOR_OUTPUT,p=JSON.parse(fs.readFileSync('protocol.json'));
const manifest={schema:'independent-s3-host/v1',arm:process.env.S3_VALIDATOR_ARM,
  created:new Date().toISOString(),git_sha:process.env.S3_VALIDATOR_GIT_SHA,
  source_sha256:sha('src/main.rs'),compiler:process.env.S3_VALIDATOR_RUST_VERSION,
  binary_sha256:fs.existsSync(dir+'/validator')?sha(dir+'/validator'):null,
  protocol_sha256:p.freeze.protocol_sha256,os:os.type(),release:os.release(),arch:os.arch(),
  cpu:os.cpus()[0]?.model,threads:1,clock_valid:false};
fs.writeFileSync(dir+'/host.json',JSON.stringify(manifest,null,2)+'\n');
JS
run_rc=125
compare_rc=125
if [ "$build_rc" -eq 0 ]; then
  set +e
  timeout 30s "$output/validator" --prime-s3 > "$output/actual.jsonl" 2> "$output/actual.stderr"
  run_rc=$?
  if [ "$run_rc" -eq 0 ]; then
    node compare.mjs "$output/actual.jsonl" ../theorem_census_20261109/evidence/semaev-native.jsonl \
      > "$output/comparison.json" 2> "$output/comparison.stderr"
    compare_rc=$?
  fi
  set -e
fi
printf '%s\n' "$run_rc" > "$output/run.exit"
printf '%s\n' "$compare_rc" > "$output/comparison.exit"
date -u +%s > "$output/end-epoch"
# Logs are the remote evidence channel; no paid artifact/cache upload.
for part in host.json build.exit build.stdout build.stderr actual.jsonl actual.stderr \
  comparison.json comparison.stderr run.exit comparison.exit start-epoch end-epoch; do
  printf '\nS3_VALIDATOR_BEGIN %s\n' "$part"
  if [ -f "$output/$part" ]; then cat "$output/$part"; fi
  printf '\nS3_VALIDATOR_END %s\n' "$part"
done
if [ "$build_rc" -ne 0 ] || [ "$run_rc" -ne 0 ] || [ "$compare_rc" -ne 0 ]; then exit 1; fi
