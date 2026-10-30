#!/bin/sh
set -eu
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
build=${1:?usage: run.sh BUILD FRESH_RUN}
run=${2:?usage: run.sh BUILD FRESH_RUN}
test ! -e "$run"
mkdir -p "$run/phases"
(cd "$here" && sha256sum --check --status SOURCES-SHA256SUMS)
sha256sum --check --status "$build/sha256.txt"
git -C "$here" rev-parse HEAD > "$run/source-revision.txt"
cp "$here/inputs/verified-branches.tsv" "$here/inputs/external-score-rows.tsv" "$here/SOURCES-SHA256SUMS" "$run/"
cp "$build/sha256.txt" "$run/binaries-and-sources.sha256"
phase() {
 name=$1
 expected=$2
 shift 2
 mkdir "$run/phases/$name"
 printf '%s\n' "$@" > "$run/phases/$name/argv.txt"
 if (ulimit -v 262144;exec env -i PATH=/usr/bin:/bin LANG=C LC_ALL=C \
  CHECKPOINT_SKIP=exact_reanalysis_of_existing_verified_results timeout -k 2 30 "$@") \
   > "$run/phases/$name/stdout" 2> "$run/phases/$name/stderr";then status=0;else status=$?;fi
 printf '%s\n' "$status" > "$run/phases/$name/native-status"
 printf '%s\n' "$expected" > "$run/phases/$name/expected-status"
 test "$status" -eq "$expected"
}
phase producer-control 0 "$build/producer" --self-test
phase verifier-control 0 "$build/verify" --self-test
phase producer 0 "$build/producer" "$run/verified-branches.tsv" "$run/external-score-rows.tsv" "$run/results" ACK_EXISTING_RESULT_AUDIT
phase verifier 0 "$build/verify" "$run/verified-branches.tsv" "$run/external-score-rows.tsv" "$run/results" "$run/verification.json"
for kind in field frequency summary;do
 phase "prepare-$kind" 0 "$build/tamper" "$kind" "$run/results" "$run/bad-$kind"
 phase "refuse-$kind" 1 "$build/verify" "$run/verified-branches.tsv" "$run/external-score-rows.tsv" "$run/bad-$kind" "$run/bad-$kind-receipt.json"
 test ! -e "$run/bad-$kind-receipt.json"
done
printf '%s\n' '{"kind":"exact-existing-score-audit-completion/v1","completed":true,"phases":10,"expected_native_exits_matched":true,"S3_checkpoint_claimed":false,"new_curves":0,"new_point_counts":0,"new_random_draws":0,"DLP_attempted":false,"timing_admitted":false}' > "$run/completion.json"
printf '%s\n' 'Completed exact audit, independent rank-DP verification and three corrupt-output refusals.'
