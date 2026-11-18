#!/usr/bin/env bash
set -u
panel=${1:?panel}
raw_root=${2:?new raw directory}
qualifier=${3:?immutable qualifier}
study=research/vieta_coset_rust_20261115/speed-20261117
case "$panel" in
  preflight) class_cap=400 ;;
  main) class_cap=1400 ;;
  *) exit 2 ;;
esac
[[ ! -e "$raw_root" ]] || exit 2
mkdir "$raw_root" || exit 2
export RAYON_NUM_THREADS=1
ulimit -v 8388608
overall=0
for cell in coset-word62-m8192 coset-word62-m16384 coset-p224-m2048 rational-word62-m2048 rational-p224-m128 rational-p224-m512; do
  timeout --signal=TERM --kill-after=5s "$class_cap" "$qualifier" panel "$study/configs/$panel/$cell.json" "$raw_root/$cell"
  code=$?
  printf '{"configuration":"%s","exit":%s,"retry":false}\n' "$cell" "$code" >> "$raw_root/driver.jsonl"
  if [[ $code -eq 101 ]]; then exit "$code"; fi
  if [[ $code -ne 0 ]]; then overall=$code; fi
done
exit "$overall"
