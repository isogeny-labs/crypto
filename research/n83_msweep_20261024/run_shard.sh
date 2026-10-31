#!/usr/bin/env bash
# Thin orchestration only: run every cell of grid.tsv whose m is in the
# shard's list, one fresh single-threaded process per cell under a cap.
# Usage: run_shard.sh <binary> <out_dir> <m,m,...>
set -euo pipefail
bin=$1; out=$2; ms=$3
here=$(cd "$(dirname "$0")" && pwd)
mkdir -p "$out"
seed=8609517
grep -v '^#' "$here/grid.tsv" | while IFS=$'\t' read -r family curve engines bases mvals trials qms cap probes; do
  for m in ${mvals//,/ }; do
    case ",$ms," in *",$m,"*) ;; *) continue ;; esac
    for fb in ${bases//,/ }; do
      for e in ${engines//,/ }; do
        f="$out/${curve}__m${m}__${fb/:/-}__${e}__${probes}.json"
        [ -e "$f" ] && continue
        start=$(date +%s.%N)
        set +e
        RAYON_NUM_THREADS=1 timeout --kill-after=5 "$cap" "$bin" cell "$m" "$fb" "$e" "$trials" "$seed" "$qms" "$probes" "$curve" > "$f.tmp" 2> "$f.err"
        rc=$?
        set -e
        end=$(date +%s.%N)
        if [ "$rc" -eq 0 ] && [ -s "$f.tmp" ]; then
          mv "$f.tmp" "$f"
        else
          printf '{"schema":"n83-msweep-cell/v2","status":"process_cap","exit":%d,"m":%d,"factor_base":"%s","engine":"%s","family":"%s","curve_spec":"%s","probes":"%s","cap_seconds":%d,"wall_seconds":%s}\n' \
            "$rc" "$m" "$fb" "$e" "$family" "$curve" "$probes" "$cap" "$(echo "$end - $start" | bc)" > "$f"
          rm -f "$f.tmp"
        fi
        [ -s "$f.err" ] || rm -f "$f.err"
        echo "m=$m $fb $e rc=$rc"
      done
    done
  done
done
