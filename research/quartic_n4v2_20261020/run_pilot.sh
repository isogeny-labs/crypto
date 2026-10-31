#!/opt/homebrew/bin/bash
# N4 v2 pilot (seeds 101, 102; not grid data). usage: run_pilot.sh <v2 binary> <outdir>
set -u
BIN=$1; OUT=$2; mkdir -p "$OUT"
jobs=()
for sd in 101 102; do
  for q in 16363 65479 262111; do jobs+=("ic $q $sd B0"); jobs+=("ic $q $sd S 3.0 2.0"); done
  for q in 65479 262111; do for cd in 1.5 2.0 2.5 3.0; do jobs+=("ic $q $sd D 3.0 $cd"); done; done
  jobs+=("ic 16363 $sd D 3.0 2.0")
  jobs+=("ic 1048507 $sd S 3.0 2.0"); jobs+=("ic 1048507 $sd D 3.0 2.0")
  jobs+=("rho 16363 $sd 0 2 plain,neg"); jobs+=("rho 65479 $sd 0 2 plain,neg"); jobs+=("rho 262111 $sd 0 1 plain,neg")
done
printf '%s\n' "${jobs[@]}" | awk '{print NR" "$0}' | xargs -P 4 -L 1 bash -c 'n=$0; shift 0; tag=$(echo "$@" | tr " ," "__"); /usr/bin/time -p nice -n 10 '"$BIN"' "$@" > '"$OUT"'/p_${n}_${tag}.jsonl 2> '"$OUT"'/p_${n}_${tag}.err'
