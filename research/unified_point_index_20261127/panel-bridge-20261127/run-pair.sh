#!/usr/bin/env bash
# Proposed native L2 payload. This script never submits a job; the launcher
# must separately admit the checkpoint, image digest, source and L2 request.
set -euo pipefail
image_root=/opt/isolab/point-index
if [[ $# == 2 && $1 == --check-only ]]; then
  root=$(realpath -- "$2")
  check_only=true
elif [[ $# == 2 && $1 == --run && $2 =~ ^[012]$ ]]; then
  root=$image_root
  schedule_index=$2
  check_only=false
else
  echo 'usage: run-pair.sh --check-only ROOT | --run SCHEDULE_INDEX_0_TO_2' >&2
  exit 2
fi
[[ -d $root && ! -L $root ]] || { echo 'missing image payload' >&2; exit 2; }
for name in baseline candidate panel optimized-options.json build-info.json SHA256SUMS inputs.sha256; do
  [[ -f $root/$name && ! -L $root/$name ]] || { echo "missing payload file: $name" >&2; exit 2; }
done
[[ -d $root/inputs && ! -L $root/inputs ]] || { echo 'missing public inputs' >&2; exit 2; }
[[ -z $(find "$root/inputs" -type l -print -quit) ]] || { echo 'symlinked public input' >&2; exit 2; }
(cd "$root" && sha256sum -c SHA256SUMS >/dev/null)
(cd "$root/inputs" && sha256sum -c ../inputs.sha256 >/dev/null)
[[ $(wc -l < "$root/inputs.sha256") == 86 ]] || { echo 'incomplete public input list' >&2; exit 2; }
for name in baseline candidate panel; do
  [[ -x $root/$name ]] || { echo "arm or driver is not executable: $name" >&2; exit 2; }
done
# The seed list is a byte-pinned JSON file. Read decimal digits as text: jq's
# floating-point parser can round these 64-bit schedule seeds.
mapfile -t seeds < <(sed -n '/"schedule_seeds": \[/,/^[[:space:]]*\]/{s/^[[:space:]]*\([0-9][0-9]*\),\{0,1\}[[:space:]]*$/\1/p;}' "$root/inputs/seed-manifest.json")
[[ ${#seeds[@]} == 3 ]] || { echo 'invalid schedule seed list' >&2; exit 2; }
for seed in "${seeds[@]}"; do [[ $seed =~ ^[0-9]{1,20}$ ]] || exit 2; done
baseline_revision=$(sed -n 's/^  "baseline_revision": "\([a-f0-9]*\)",$/\1/p' "$root/build-info.json")
candidate_revision=$(sed -n 's/^  "candidate_revision": "\([a-f0-9]*\)",$/\1/p' "$root/build-info.json")
[[ $baseline_revision =~ ^[a-f0-9]{40}$ && $candidate_revision =~ ^[a-f0-9]{40}$ && $baseline_revision != "$candidate_revision" ]] \
  || { echo 'invalid paired source identities' >&2; exit 2; }
grep -Fq '"schema": "hashed-ecdl-panel/v2"' "$root/inputs/manifest.json" \
  || { echo 'wrong panel schema' >&2; exit 2; }
grep -Fq "\"baseline\": \"$baseline_revision\"" "$root/inputs/manifest.json" \
  || { echo 'panel baseline source mismatch' >&2; exit 2; }
grep -Fq "\"paired_baseline\": \"$candidate_revision\"" "$root/inputs/manifest.json" \
  || { echo 'panel candidate source mismatch' >&2; exit 2; }
if [[ $check_only == true ]]; then
  echo 'Native pair payload and frozen public inputs verified; no solver executed.'
  exit 0
fi
[[ -n ${ISOLAB_JOB_ID:-} && $ISOLAB_JOB_ID =~ ^[a-z0-9]([-a-z0-9]*[a-z0-9])?$ ]] \
  || { echo 'missing native job identity' >&2; exit 2; }
[[ ${ISOLAB_RESULTS_URI:-} == "s3://setec-astronomy-artifacts/runs/$ISOLAB_JOB_ID/"* ]] \
  || { echo 'unexpected result destination' >&2; exit 2; }
[[ -n ${POD_UID:-} && -n ${NODE_NAME:-} ]] || { echo 'missing pod/node identity' >&2; exit 2; }
[[ ! -e /work/results ]] || { echo 'result path already exists' >&2; exit 2; }
mkdir -p /work/results
export RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1
printf '%s\n' "$ISOLAB_JOB_ID" > /work/results/job-id.txt
printf '%s\n' "$POD_UID" > /work/results/pod-uid.txt
printf '%s\n' "$NODE_NAME" > /work/results/node-name.txt
printf '%s\n' "$schedule_index" > /work/results/schedule-index.txt
cp -- "$root/build-info.json" "$root/inputs.sha256" /work/results/
set +e
"$root/panel" run --panel "$root/inputs" --inputs-sha256 "$root/inputs.sha256" \
  --baseline "$root/baseline" --candidate "$root/candidate" \
  --optimized-options "$root/optimized-options.json" --out /work/results/panel-run \
  --seed "${seeds[$schedule_index]}" --case-limit-secs 30 \
  --deadline-secs 1800 --aa-rounds 5 --ab-rounds 5 \
  > /work/results/panel.stdout 2> /work/results/panel.stderr
panel_exit=$?
set -e
printf '%s\n' "$panel_exit" > /work/results/panel.exit
(cd /work/results && find . -type f ! -name SHA256SUMS -printf '%P\n' | LC_ALL=C sort | xargs -r sha256sum > SHA256SUMS)
aws s3 cp /work/results/ "$ISOLAB_RESULTS_URI" --recursive --only-show-errors
[[ $panel_exit == 0 ]] || exit "$panel_exit"
echo 'Native panel result uploaded; independent remote readback and replay remain required.'
