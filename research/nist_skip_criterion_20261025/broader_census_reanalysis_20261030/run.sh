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
cp "$here/inputs.tsv" "$here/prior21.tsv" "$here/SOURCES-SHA256SUMS" "$run/"
cp "$build/sha256.txt" "$run/binaries-and-sources.sha256"
phase() {
 name=$1
 seconds=$2
 expected=$3
 shift 3
 mkdir "$run/phases/$name"
 printf '%s\n' "$@" > "$run/phases/$name/argv.txt"
 if (ulimit -v 1572864; exec env -i PATH=/usr/bin:/bin LANG=C LC_ALL=C \
  CHECKPOINT_SKIP=exact_reanalysis_of_existing_certified_results timeout -k 2 "$seconds" "$@") \
   > "$run/phases/$name/stdout" 2> "$run/phases/$name/stderr"; then status=0;else status=$?;fi
 printf '%s\n' "$status" > "$run/phases/$name/native-status"
 printf '%s\n' "$expected" > "$run/phases/$name/expected-status"
 test "$status" -eq "$expected"
}
phase producer-control 30 0 "$build/producer" --self-test
phase verifier-control 30 0 "$build/verify" --self-test
phase producer 180 0 "$build/producer" "$run/inputs.tsv" "$run/prior21.tsv" "$run/results" ACK_LOCAL_EXACT_REANALYSIS
phase verifier 300 0 "$build/verify" "$run/inputs.tsv" "$run/prior21.tsv" "$run/results" "$run/verification.json"
for kind in character profile cutoff;do
 phase "prepare-$kind" 30 0 "$build/tamper" "$kind" "$run/results" "$run/bad-$kind"
 phase "refuse-$kind" 300 1 "$build/verify" "$run/inputs.tsv" "$run/prior21.tsv" "$run/bad-$kind" "$run/bad-$kind-receipt.json"
 test ! -e "$run/bad-$kind-receipt.json"
done
printf '%s\n' '{"kind":"broader-existing-results-reanalysis/v1","completed":true,"phases":10,"expected_native_exits_matched":true,"S3_checkpoint_claimed":false,"new_point_counts":0,"new_curves":0,"timing_admitted":false}' > "$run/completion.json"
printf '%s\n' 'Completed all broader-census phases and three expected corruption refusals.'
