#!/bin/bash
set -euo pipefail
study=$(cd "$(dirname "$0")" && pwd)
root=$1
[[ ! -e "$root" ]]
mkdir "$root"
for mode in success failure signal; do
 mkdir "$root/$mode"
 set +e
 bash -c '
 set -euo pipefail
 out=$1;started=$(date -u +%FT%TZ);mode=$2
 export LEFSCHETZ_TEST_CONTROLLER=$$
 upload(){ :; }
 source "$3/source-functional-journal.sh"
 case "$mode" in
 success) lefschetz_phase producer true;;
 failure) lefschetz_phase producer bash -c "exit 7";;
 signal) lefschetz_phase producer bash -c '\''kill -TERM "$LEFSCHETZ_TEST_CONTROLLER"; sleep 30'\'';;
 esac
 ' _ "$root/$mode" "$mode" "$study"
 code=$?
 set -e
 printf '%s\n' "$code" > "$root/$mode-controller-exit.txt"
 expected=0;[[ $mode != failure ]] || expected=7;[[ $mode != signal ]] || expected=143
 [[ $code == "$expected" ]]
 jq -e --argjson expected "$expected" '.exit_code==$expected and .phases["producer"].exit_code==$expected and .phases["replay"].started==false and .phases["replay"].exit_code==125' "$root/$mode/terminal.json"
 (cd "$root/$mode" && sha256sum -c manifest.sha256)
done
