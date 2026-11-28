#!/usr/bin/env bash
set -u -o pipefail
panel=${1:?preflight or main}
out=${2:?new output directory}
qualifier=${3:?immutable binary}
[[ $panel == preflight || $panel == main ]] || exit 2
[[ ! -e $out ]] || { echo 'refuse existing run directory' >&2; exit 2; }
mkdir -p "$out"
ulimit -v 8388608
export VIETA_ISOLATED_RUN=1
rounds=6
warmups=2
cap=600
if [[ $panel == preflight ]]; then rounds=2; warmups=0; cap=300; fi
any=0
while read -r representation field m; do
  directory="$out/$representation-$field-m$m"
  mkdir "$directory"
  if timeout --signal=TERM --kill-after=5s "$cap" env VIETA_PANEL="next-$panel" VIETA_OUTPUT_DIR="$directory" "$qualifier" bench "$representation" "$field" "$m" "$rounds" "$warmups" 30 v1 > /dev/null 2> "$directory/stderr.txt"; then
    code=0
  else
    code=$?
    any=1
  fi
  printf '{"panel":"%s","representation":"%s","field":"%s","m":%s,"exit":%s}\n' "$panel" "$representation" "$field" "$m" "$code" >> "$out/driver.jsonl"
done <<'MATRIX'
coset word62 8192
coset word62 16384
coset p224 2048
rational word62 2048
rational p224 128
rational p224 512
MATRIX
exit "$any"
