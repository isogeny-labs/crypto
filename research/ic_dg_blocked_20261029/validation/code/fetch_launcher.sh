#!/bin/bash
# Read-only copy of the durable S3 launcher receipts of the six jobs to
# /dev/shm/dgb-val/s3-launcher/<P>-<s>/ (the evidence directory's launch/
# folders are untracked at bf899979a6 and are not used).  Then place request.json
# and result.json in /dev/shm/dgb-validation/launcher-receipts/ (status.json and
# submission.json stay in scratch; their pod-binding fields are extracted by
# dgbval.py into work/recompute.json).  Needs AWS_DEFAULT_REGION=us-east-2 and
# the awsvenv on PATH; issues only s3 ls/cp reads.
set -eu
export AWS_DEFAULT_REGION=us-east-2 PATH=/tmp/claude-0/awsvenv/bin:$PATH
B=s3://isolab-results-104186811093-us-east-2/runs
D=/dev/shm/dgb-val/s3-launcher
V=/dev/shm/dgb-validation/launcher-receipts
declare -A L=(
  [P24-wall]=ic-dgb-p24-20261029/launcher/cbe06109-04d6-4437-9489-4eba30d8f5e0
  [P24-counted]=ic-dgb-rho-p24-20261029/launcher/cb50bfe8-6cca-42a8-b922-4526c887d7ee
  [P28-wall]=ic-dgb-p28-20261029/launcher/489f1695-77dd-4b0b-bf32-0e050dc91e49
  [P28-counted]=ic-dgb-rho-p28-20261029/launcher/b5c2c990-0e5c-4b3d-91f7-720a02c8eac0
  [P32-wall]=ic-dgb-p32-20261029/launcher/11f7feea-a538-4dc8-83fd-c874ae603784
  [P32-counted]=ic-dgb-rho-p32-20261029/launcher/9c48332a-2626-4037-b843-9c0bd7ca0faf )
for k in "${!L[@]}"; do
  job=${L[$k]%%/*}
  n=$(aws s3 ls "$B/$job/launcher/" | wc -l)
  [ "$n" -eq 1 ] || { echo "$k: expected one launcher prefix, found $n" >&2; exit 1; }
  mkdir -p "$D/$k" "$V/$k"; echo "${L[$k]}" > "$D/$k/PREFIX"
  for f in request.json original-request.json result.json status.json submission.json; do
    aws s3 cp --only-show-errors "$B/${L[$k]}/$f" "$D/$k/$f"
  done
  cp "$D/$k/request.json" "$D/$k/result.json" "$D/$k/PREFIX" "$V/$k/"
done
(cd "$V" && find . -type f | sort | xargs sha256sum)
