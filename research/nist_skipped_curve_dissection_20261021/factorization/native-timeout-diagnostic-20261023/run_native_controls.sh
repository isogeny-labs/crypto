#!/usr/bin/env bash
# Two isolated bounded diagnostics; never reads or modifies R05 relation data.
set -euo pipefail
here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
out=$1
mkdir -p "$out"
[[ -z $(find "$out" -mindepth 1 -maxdepth 1 -print -quit) ]]
out=$(cd -- "$out" && pwd)
original=${YAFU_ORIGINAL_BIN:?}
patched=${YAFU_PATCHED_BIN:?}
data=${YAFU_DATA_DIR:?}
siever=${GNFS_BIN_DIR:?}
n=33492266050798185653682380062483799213318454801653963493134064721059735936944520907295931496614455858076048437184499090478420204888353
for arm in original patched; do
  state="$out/$arm/state"
  mkdir -p "$state"
  bin=$original
  if [[ $arm == patched ]]; then bin=$patched; fi
  cp "$bin" "$state/yafu"
  for file in ggnfs-params-table.txt ggnfs-params-Gimarel.txt docfile.txt; do
    cp "$data/$file" "$state/$file"
  done
  cp "$here/../rounds/R05-20261022/selected-polynomial.job" "$state/nfs.job"
  cp "$here/../rounds/R05-20261022/selected-polynomial.fb" "$state/nfs.fb"
  sha256sum "$state/yafu" "$state/nfs.job" "$state/nfs.fb" > "$out/$arm/input-sha256.txt"
  printf 'nfs(%s)\n' "$n" > "$out/$arm/expression.txt"
  argv=(./yafu -threads 2 -lathreads 2 -seed 20261022 -gnfs -xover 81
    -ggnfsT 1 -ns 2935000,2935500 -ggnfs_dir "$siever/" -job nfs.job -o nfs.dat
    -logfile factor.log -session session.log -R)
  printf '%q ' timeout --verbose --signal=INT --kill-after=5s 30 "${argv[@]}" > "$out/$arm/command.txt"
  printf '< %q\n' "$out/$arm/expression.txt" >> "$out/$arm/command.txt"
  date -u +%FT%TZ > "$out/$arm/started.txt"
  set +e
  (cd "$state" && ulimit -c 0 && ulimit -v 4194304 && ulimit -f 65536 &&
    LC_ALL=C timeout --verbose --signal=INT --kill-after=5s 30 "${argv[@]}" < "$out/$arm/expression.txt") \
    > "$out/$arm/stdout.txt" 2> "$out/$arm/stderr.txt"
  rc=$?
  set -e
  printf '%s\n' "$rc" > "$out/$arm/exit.txt"
  date -u +%FT%TZ > "$out/$arm/finished.txt"
done
fixed_timeout=false
if rg -q 'nfs timeout of 1 sec exceeded' "$out/patched/stdout.txt"; then fixed_timeout=true; fi
original_sha=$(sha256sum "$out/original/state/yafu"); original_sha=${original_sha%% *}
patched_sha=$(sha256sum "$out/patched/state/yafu"); patched_sha=${patched_sha%% *}
jq -n --argjson original "$(cat "$out/original/exit.txt")" \
  --argjson patched "$(cat "$out/patched/exit.txt")" --argjson printed "$fixed_timeout" \
  --arg original_sha "$original_sha" --arg patched_sha "$patched_sha" \
  '{original_exit:$original,patched_exit:$patched,patched_integer_timeout_logged:$printed,
    original_binary_sha256:$original_sha,patched_binary_sha256:$patched_sha,
    passed:($original==139 and $patched==0 and $printed),threads:2,native_limit_seconds:1,
    outer_limit_seconds:30,kill_grace_seconds:5,q_start:2935000,q_stop:2935500,
    original_r05_relations_imported:false,production_backtrace_obtained:false,
    factorization_completion_claim:false}' > "$out/summary.json"
cat "$out/summary.json"
jq -e .passed "$out/summary.json" >/dev/null
