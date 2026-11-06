#!/usr/bin/env bash
# Preserve native candidates and validate exact roots before merging a checkpoint.
set -euo pipefail
here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
out=$(cd -- "$1" && pwd)
number=$2
[[ $number =~ ^[0-9]+$ && $(cat "$out/input.txt") == "$number" ]]
exec 9> "$out.lock"
flock -n 9 || { echo 'Native search is active; refusing checkpoint mutation' >&2; exit 1; }
index=1
while [[ -e $out/polynomial-recovery-$index ]]; do index=$((index+1)); done
rec="$out/polynomial-recovery-$index"
mkdir -p "$rec"
gp=${GP_BIN:-gp}
validator="$here/../R03-20261022/recover_polynomials.gp"
validate() {
  NFS_INPUT="$number" POLY_INPUT="$1" POLY_OUTPUT="$2" "$gp" -q -f "$validator" \
    > "$3.json" 2> "$3.stderr"
  jq -e '.verified and .candidate_count>0' "$3.json" >/dev/null
}
if [[ -s $out/state/nfs.dat.p ]]; then
  cp "$out/state/nfs.dat.p" "$rec/original-master.p"
  validate "$rec/original-master.p" "$rec/base.p" "$rec/base-verdict"
else
  printf 'n: %s\n' "$number" > "$rec/base.p"
fi
cp "$rec/base.p" "$rec/merged.p"
shopt -s nullglob
threads=("$out/state/nfs.dat."*.p)
for file in "${threads[@]}"; do
  name=$(basename -- "$file")
  cp "$file" "$rec/original-$name"
  # Framing only. All integer arithmetic and common-root checks are native GP.
  awk '
    /^# norm / {if(buffer!="") {print "incomplete internal block" > "/dev/stderr"; exit 1}; buffer=$0 ORS; next}
    {buffer=buffer $0 ORS}
    /^Y1: *-?[0-9]+[[:space:]]*$/ {printf "%s",buffer; buffer=""}
    END {if(buffer!="") print "retained trailing incomplete block in original file" > "/dev/stderr"}
  ' "$file" > "$rec/framed-$name" 2> "$rec/framing-$name.stderr"
  if [[ -s $rec/framed-$name ]]; then
    validate "$rec/framed-$name" "$rec/validated-$name" "$rec/verdict-$name"
    tail -n +2 "$rec/validated-$name" >> "$rec/merged.p"
  fi
done
validate "$rec/merged.p" "$rec/validated-master.p" "$rec/final-verdict"
cp "$rec/validated-master.p" "$out/state/nfs.dat.p"
# Consumed thread files remain in evidence and cannot be merged twice.
for file in "${threads[@]}"; do
  name=$(basename -- "$file")
  mv "$file" "$rec/consumed-$name"
done
cp "$rec/final-verdict.json" "$out/polynomial-verification.json"
sha256sum "$out/state/nfs.dat.p" > "$rec/master-sha256.txt"
cat "$rec/final-verdict.json"
