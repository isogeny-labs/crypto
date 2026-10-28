#!/usr/bin/env bash
# Submit exactly one Amendment 13 part. No retry and no mutation of frozen source.
set -euo pipefail
HERE=$(cd -- "$(dirname -- "$0")" && pwd)
ROOT=$(git -C "$HERE" rev-parse --show-toplevel)
part=${1:?usage: launch.sh a|c}
case "$part" in
  a) instance_type=r7i.8xlarge; ttl=48h ;;
  c) instance_type=r7i.4xlarge; ttl=48h ;;
  *) echo 'Only Amendment 13 parts a and c are admitted; E12b is pending.' >&2; exit 2 ;;
esac
cd "$ROOT"
protocol=research/fall_degree_bounds_20261008/PREREGISTRATION.md
launcher=taskq/deploy/ephemeral/ephemeral
runner=taskq/deploy/ephemeral/runner.sh
relative_here=research/fall_degree_bounds_20261008/e12/continuation-20261028
git diff --quiet HEAD -- "$protocol" "$launcher" "$runner" "$relative_here/launch.sh" || {
  echo 'Refusing uncommitted changes to launch-bound files.' >&2; exit 2;
}
git cat-file -e "HEAD:$relative_here/launch.sh"
protocol_sha=$(sha256sum "$protocol" | cut -d' ' -f1)
protocol_commit=$(git rev-parse HEAD)
mkdir -p "$HERE/receipts"
receipt="$HERE/receipts/$part"
mkdir "$receipt" || { echo 'Receipt already exists; reconcile it instead of retrying.' >&2; exit 2; }
jq -n --arg part "$part" --arg type "$instance_type" --arg ttl "$ttl" \
  --arg commit "$protocol_commit" --arg protocol_sha "$protocol_sha" \
  '{part:$part,instance_type:$type,ttl:$ttl,market:"spot",protocol_commit:$commit,
    protocol_sha256:$protocol_sha,created_utc:(now|todate),new_attempts_allowed:1,
    source_commit:"371035216942660c4615dfbb88d85a14afaf5c16",
    source_sha256:"4ae808ac57920d93f599936c34513167befc93d865495defcae29fb16a18fb8f"}' \
  >"$receipt/request.json"
sha256sum "$protocol" "$launcher" "$runner" "$relative_here/launch.sh" >"$receipt/inputs.sha256"
command=$(cat <<'REMOTE'
set -euo pipefail
mkdir -p /work/fall-degree-source
cd /work/fall-degree-source
aws s3 cp --region us-east-1 s3://ephemeral-runs-104186811093-us-east-1/jobs/inputs/fd-src-371035216942.tgz source.tgz --only-show-errors
echo '4ae808ac57920d93f599936c34513167befc93d865495defcae29fb16a18fb8f  source.tgz' | sha256sum -c -
tar xzf source.tgz
rm source.tgz
printf '%s\n' 371035216942660c4615dfbb88d85a14afaf5c16 >"$EPHEMERAL_OUT/source_commit"
printf '%s\n' "$E12_PROTOCOL_SHA256" >"$EPHEMERAL_OUT/protocol_sha256"
printf '%s\n' "$E12_PROTOCOL_COMMIT" >"$EPHEMERAL_OUT/protocol_commit"
case "$E12_PART" in
  a) KEEP=2 CMAX=5 bash research/fall_degree_bounds_20261008/e12/run_par.sh "$EPHEMERAL_OUT" 3:21:7 ;;
  c) bash research/fall_degree_bounds_20261008/e12/run_window.sh "$EPHEMERAL_OUT" 4:28:7 ;;
  *) exit 2 ;;
esac
REMOTE
)
set +e
EPHEMERAL_REGION=us-east-1 bash "$launcher" run --detach --no-source --spot \
  --type "$instance_type" --toolchain rust --ttl "$ttl" \
  --name "fd-e12${part}-20261028-r3" \
  --env "E12_PART=$part" --env "E12_PROTOCOL_SHA256=$protocol_sha" \
  --env "E12_PROTOCOL_COMMIT=$protocol_commit" \
  -- "$command" >"$receipt/launch.stdout" 2>"$receipt/launch.stderr"
rc=$?
set -e
jq -n --argjson rc "$rc" '{launcher_exit_code:$rc,recorded_utc:(now|todate),scientific_result:null}' \
  >"$receipt/submission.json"
cat "$receipt/launch.stdout"
cat "$receipt/launch.stderr" >&2
exit "$rc"
