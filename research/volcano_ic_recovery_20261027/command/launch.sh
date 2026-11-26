#!/usr/bin/env bash
# Image-local thin orchestration. Inputs remain the frozen public instance.
set -euo pipefail
umask 077
[[ $# == 1 && ( $1 == plan || $1 == run ) ]] || { echo 'usage: launch.sh plan|run' >&2; exit 64; }
image_root=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)
pilot=research/volcano_ic_recovery_20261027
[[ $(cat "$image_root/SOURCE_REVISION") =~ ^[a-f0-9]{40}$ ]] || { echo 'missing source revision' >&2; exit 65; }
cd "$image_root/repo"
[[ $(sha256sum "$pilot/SOURCES.sha256" | cut -d' ' -f1) == ff0c1804287eb3f281bbe2908d6ba149f2b33013f4abd6ecfaa2ff8f96c8e162 ]] || { echo 'frozen source manifest changed' >&2; exit 65; }
sha256sum -c "$pilot/SOURCES.sha256" >&2
(cd "$image_root" && sha256sum -c binaries.sha256) >&2
input=$image_root/repo/$pilot/public_source_case.json
config=$image_root/repo/$pilot/pilot_config.json
if [[ $1 == plan ]]; then
  exec "$image_root/bin/runner" plan "$input" "$config" --explicit
fi
[[ ${ISOLAB_CRIU:-} == 1 ]] || { echo 'run requires the recovery supervisor; no local production fallback' >&2; exit 66; }
output=${ISOLAB_OUTPUT_DIR:-}
[[ $output == /work/compute/* && $output != /work/compute/ && -d $output && ! -L $output ]] || { echo 'missing supervisor output directory' >&2; exit 66; }
[[ $(realpath -e -- "$output") == "$output" ]] || { echo 'output path must be canonical and contain no symlinks' >&2; exit 66; }
[[ ! -e $output/recovery && ! -e $output/wrapper-result.json && ! -e $output/collector.stdout && ! -e $output/collector.stderr ]] || { echo 'output already used; reconcile the original run' >&2; exit 66; }
collector_exit=0
"$image_root/bin/runner" run "$input" "$config" "$output/recovery" --private-output --explicit > "$output/collector.stdout" 2> "$output/collector.stderr" || collector_exit=$?
replay_exit=null
if [[ $collector_exit == 0 ]]; then
  replay_exit=0
  "$image_root/bin/verify" "$input" "$config" "$output/recovery/records.private.jsonl" > "$output/independent-replay.json" 2> "$output/independent-replay.stderr" || replay_exit=$?
fi
printf '{"schema":"volcano.ic.command-wrapper/v1","collector_exit_code":%s,"independent_replay_exit_code":%s,"timing_valid":false,"clock_admission":"L0 command computation only"}\n' "$collector_exit" "$replay_exit" > "$output/wrapper-result.json"
[[ $collector_exit == 0 ]] || exit "$collector_exit"
[[ $replay_exit == 0 ]] || exit 67
