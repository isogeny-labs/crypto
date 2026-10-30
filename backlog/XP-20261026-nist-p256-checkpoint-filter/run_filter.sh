#!/usr/bin/env bash
# Thin orchestration only; pinned YAFU/msieve filters, Rust inventories its log.
set -euo pipefail
here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
preflight=false
if [[ ${1:-} == --preflight-only ]]; then preflight=true; shift; fi
[[ $# == 3 ]] || { echo 'usage: run_filter.sh [--preflight-only] reference|candidate CHECKPOINT_STATE NEW_OUTPUT' >&2; exit 2; }
arm=$1
input=$(cd -- "$2" && pwd)
out=$3
case $arm in
  reference) manifest=$here/reference-inputs.sha256; lines=4592399 ;;
  candidate) manifest=$here/candidate-inputs.sha256; lines=10441117 ;;
  *) echo 'Unknown arm' >&2; exit 2 ;;
esac
(cd "$input" && sha256sum --check "$manifest")
[[ $(wc -l < "$input/nfs.dat") == "$lines" ]]
[[ $(sha256sum "$input/yafu" | cut -d' ' -f1) == bc7680adbdd7450a68120e73ef8aa88d0336c8ffa141343f50966c595ca6bae9 ]]
if $preflight; then printf 'Verified %s input; no experiment executed.\n' "$arm"; exit 0; fi
repo=$(git -C "$here" rev-parse --show-toplevel)
# Required by AGENTS.md: a durable resume checkpoint must precede execution.
bash "$repo/scripts/checkpoint.sh" gate
: "${GNFS_BIN_DIR:?Set the pinned native siever directory}"
: "${NIST_FILTER_CHECKER:?Build inspect_filter.rs and set its executable path}"
[[ -x $NIST_FILTER_CHECKER && -d $GNFS_BIN_DIR ]]
[[ ! -e $out ]] || { echo 'Output already exists; preserve it and use a new run id' >&2; exit 2; }
mkdir -p -- "$out/state"
out=$(cd -- "$out" && pwd)
available_kib=$(df -Pk "$out" | awk 'NR==2 {print $4}')
[[ $available_kib -ge 5242880 ]] || { echo 'Require at least 5 GiB free before copying' >&2; exit 2; }
exec 9>"$out/run.lock"
flock -n 9
for file in yafu nfs.dat nfs.job nfs.fb nfs.dat.ranges ggnfs-params-table.txt ggnfs-params-Gimarel.txt docfile.txt; do
  cp --reflink=auto -- "$input/$file" "$out/state/$file"
done
(cd "$out/state" && sha256sum --check "$manifest") > "$out/copy-verification.txt"
cp -- "$manifest" "$out/input-sha256.txt"
sha256sum "$here/run_filter.sh" "$here/inspect_filter.rs" "$NIST_FILTER_CHECKER" > "$out/instrument-sha256.txt"
uname -a > "$out/host.txt"
lscpu >> "$out/host.txt"
printf '%s\n' 'L0; no isolation claim; all clock values descriptive' >> "$out/host.txt"
printf '%s\n' 'nfs(33492266050798185653682380062483799213318454801653963493134064721059735936944520907295931496614455858076048437184499090478420204888353)' > "$out/expression.txt"
argv=(./yafu -threads 2 -lathreads 2 -seed 20261025 -gnfs -xover 81 -ggnfsT 600
  -nc1 -minrels 1 -ggnfs_dir "$GNFS_BIN_DIR/" -job nfs.job -o nfs.dat -logfile factor.log -session session.log -R)
printf '%q ' timeout --verbose --signal=INT --kill-after=15s 660 "${argv[@]}" > "$out/command.txt"
printf '\n' >> "$out/command.txt"
date -u +%FT%TZ > "$out/started.txt"
set +e
(cd "$out/state" && ulimit -c 0 && ulimit -v 4194304 && ulimit -f 1048576 &&
 LC_ALL=C timeout --verbose --signal=INT --kill-after=15s 660 "${argv[@]}" < "$out/expression.txt") > "$out/stdout.txt" 2> "$out/stderr.txt"
rc=$?
set -e
printf '%s\n' "$rc" > "$out/exit-status.txt"
date -u +%FT%TZ > "$out/finished.txt"
# Filtering may append free relations to its copy. The collection source must stay unchanged.
(cd "$input" && sha256sum --check "$manifest") > "$out/source-after-verification.txt"
head -n "$lines" "$out/state/nfs.dat" | sha256sum > "$out/original-prefix-after.sha256"
[[ $(cut -d' ' -f1 "$out/original-prefix-after.sha256") == $(awk '$2=="nfs.dat" {print $1}' "$manifest") ]]
shopt -s nullglob
native_log=
for log in "$out"/state/*.log; do
  if rg -q 'commencing relation filtering' "$log"; then
    [[ -z $native_log ]] || { echo 'Ambiguous native filter logs' >&2; exit 2; }
    native_log=$log
  fi
done
[[ -n $native_log ]] || { echo "No native filtering log; preserve native exit $rc" >&2; exit 2; }
cp -- "$native_log" "$out/native-filter.log"
"$NIST_FILTER_CHECKER" "$out/native-filter.log" "$out/exit-status.txt" "$arm" "$lines" > "$out/counts.json"
rg -n 'raising min_rels|filtering wants|commencing msieve' "$out/state/factor.log" "$out/stdout.txt" > "$out/controller-status.txt" || true
find "$out/state" -maxdepth 1 -type f -printf '%f %s\n' | sort > "$out/output-files.txt"
cat "$out/counts.json"
