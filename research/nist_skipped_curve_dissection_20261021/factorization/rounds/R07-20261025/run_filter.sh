#!/usr/bin/env bash
set -euo pipefail
out=$1
input=${NFS_RELATION_SOURCE:?Set the completed R06 state directory}
siever=${GNFS_BIN_DIR:?}
[[ -f $input/nfs.dat && -x $input/yafu ]]
mkdir -p "$out"
[[ -z $(find "$out" -mindepth 1 -maxdepth 1 -print -quit) ]]
out=$(cd -- "$out" && pwd)
mkdir "$out/state"
for file in yafu nfs.dat nfs.job nfs.fb nfs.dat.ranges ggnfs-params-table.txt ggnfs-params-Gimarel.txt docfile.txt; do
  cp --reflink=auto "$input/$file" "$out/state/$file"
done
(cd "$input" && sha256sum yafu nfs.dat nfs.job nfs.fb nfs.dat.ranges) > "$out/input-sha256.txt"
(cd "$out/state" && sha256sum -c "$out/input-sha256.txt") > "$out/copy-verification.txt"
n=33492266050798185653682380062483799213318454801653963493134064721059735936944520907295931496614455858076048437184499090478420204888353
printf 'nfs(%s)\n' "$n" > "$out/expression.txt"
argv=(./yafu -threads 2 -lathreads 2 -seed 20261025 -gnfs -xover 81
  -ggnfsT 300 -nc1 -minrels 1 -ggnfs_dir "$siever/" -job nfs.job
  -o nfs.dat -logfile factor.log -session session.log -R)
printf '%q ' timeout --verbose --signal=INT --kill-after=15s 360 "${argv[@]}" > "$out/command.txt"
printf '< %q\n' "$out/expression.txt" >> "$out/command.txt"
sha256sum "${BASH_SOURCE[0]}" > "$out/source-sha256.txt"
date -u +%FT%TZ > "$out/started.txt"
set +e
(cd "$out/state" && ulimit -c 0 && ulimit -v 4194304 && ulimit -f 1048576 &&
 LC_ALL=C timeout --verbose --signal=INT --kill-after=15s 360 "${argv[@]}" < "$out/expression.txt") \
 > "$out/stdout.txt" 2> "$out/stderr.txt"
rc=$?
set -e
date -u +%FT%TZ > "$out/finished.txt"
(cd "$out/state" && sha256sum nfs.dat) > "$out/relation-file-after.sha256"
original=$(awk '$2=="nfs.dat" {print $1}' "$out/input-sha256.txt")
after=$(awk '{print $1}' "$out/relation-file-after.sha256")
unchanged=false
if [[ $original == "$after" ]]; then unchanged=true; fi
filter_started=false
if [[ -f $out/state/factor.log ]] && rg -q 'commencing msieve filtering' "$out/state/factor.log"; then filter_started=true; fi
jq -n --argjson rc "$rc" --argjson unchanged "$unchanged" --argjson started "$filter_started" '{exit_status:$rc,
  native_filtering_started:$started,
  input_relation_file_unchanged:$unchanged,native_limit_seconds:300,outer_limit_seconds:360,
  kill_grace_seconds:15,phase_selection:"filter_only",dispatch_min_relations:1,
  raw_relation_lines_supplied:4592399,complete_factorization_claim:false}' > "$out/receipt.json"
awk '($1 ~ /^(P|PRP|C)[0-9]+$/ && $2=="=" && $3 ~ /^[0-9]+$/) {print $3}
     ($1 ~ /^(p|prp|c)[0-9]+:$/ && $NF ~ /^[0-9]+$/) {print $NF}' "$out/stdout.txt" |
  awk -v n="$n" '"x" $0!="x" n && "x" $0!="x1"' | sort -u > "$out/candidate-factors.txt"
cat "$out/receipt.json"
