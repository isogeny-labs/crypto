#!/usr/bin/env bash
# Native YAFU NFS with retained savefiles; all factor labels are untrusted.
set -euo pipefail
number=$1 out=$2 seconds=$3 mode=${4:-fresh}
[[ $number =~ ^[0-9]+$ && $seconds =~ ^[0-9]+$ ]]
[[ $mode == fresh || $mode == resume ]]
((seconds>0))
mkdir -p "$(dirname -- "$out")"
exec 9> "$out.lock"
flock -x 9
if [[ $mode == fresh ]]; then
  mkdir -p "$out"
  [[ -z $(find "$out" -mindepth 1 -maxdepth 1 -print -quit) ]] || exit 1
  mkdir -p "$out/state"
  printf '%s\n' "$number" > "$out/input.txt"
  bin=${YAFU_BIN:?Set YAFU_BIN to the pinned executable}
  data=${YAFU_DATA_DIR:-$(dirname -- "$bin")}
  cp "$bin" "$out/state/yafu"
  for file in ggnfs-params-table.txt ggnfs-params-Gimarel.txt docfile.txt; do
    cp "$data/$file" "$out/state/$file"
  done
else
  [[ $(cat "$out/input.txt") == "$number" && -x $out/state/yafu ]]
fi
out=$(cd -- "$out" && pwd)
siever=${GNFS_BIN_DIR:?Set GNFS_BIN_DIR to the pinned lattice siever directory}
siever=$(cd -- "$siever" && pwd)
attempt=1
while [[ -e $out/attempt-$attempt ]]; do attempt=$((attempt+1)); done
log="$out/attempt-$attempt"
mkdir -p "$log"
date -u +%FT%TZ > "$log/started.txt"
git rev-parse HEAD > "$log/code-commit.txt"
sha256sum "${BASH_SOURCE[0]}" "$out/state/yafu" > "$log/source-binary-sha256.txt"
find "$siever" -maxdepth 1 -type f -name 'gnfs-*' -exec sha256sum {} + > "$log/siever-sha256.txt"
if [[ $mode == resume ]]; then
  tar -C "$out" -cf - state | xz -6 -T1 > "$log/state-before.tar.xz"
fi
argv=(./yafu "nfs($number)" -threads 2 -lathreads 2 -seed 20261022,362436069
  -ggnfsT "$seconds" -ggnfs_dir "$siever/" -job nfs.job -o nfs.dat
  -logfile factor.log -session session.log)
[[ $mode == fresh ]] || argv+=(-R)
outer=$((seconds+60))
printf '%q ' timeout --signal=INT --kill-after=30s "$outer" "${argv[@]}" > "$log/command.txt"
printf '\n' >> "$log/command.txt"
set +e
(cd "$out/state" && ulimit -v 4194304 && timeout --signal=INT --kill-after=30s "$outer" "${argv[@]}") \
  > "$log/stdout.txt" 2> "$log/stderr.txt"
status=$?
set -e
date -u +%FT%TZ > "$log/finished.txt"
awk '($1 ~ /^(P|PRP|C)[0-9]+$/ && $2=="=" && $3 ~ /^[0-9]+$/) {print $3}
     ($1 ~ /^(p|prp|c)[0-9]+:$/ && $NF ~ /^[0-9]+$/) {print $NF}' \
  "$log/stdout.txt" | sort -u > "$log/reported-factors.txt"
awk -v n="$number" '"x" $0!="x" n && "x" $0!="x1"' "$log/reported-factors.txt" > "$log/candidate-factors.txt"
jq -n --arg n "$number" --arg mode "$mode" --argjson seconds "$seconds" \
  --argjson outer "$outer" --argjson status "$status" \
  '{schema:"nist-native-nfs-attempt/v1",input:$n,mode:$mode,native_limit_seconds:$seconds,
    outer_limit_seconds:$outer,threads:2,linear_algebra_threads:2,
    seed:"20261022,362436069",address_space_limit_kib_per_process:4194304,
    exit_status:$status,timed_out:($status==124),factor_labels:"unverified",
    checkpoint_directory:"state"}' > "$log/receipt.json"
cp "$log/candidate-factors.txt" "$out/candidate-factors.txt"
cp "$log/receipt.json" "$out/receipt.json"
echo "NFS recorded: exit $status; state preserved at $out/state"
[[ $status == 0 || $status == 124 || $status == 130 || $status == 137 ]]
