#!/usr/bin/env bash
# Thin orchestration only: run every cell of grid.tsv whose m is in the
# shard's list, one fresh single-threaded process per cell under its cap.
# Modelled on research/n83_msweep_20261024/run_shard.sh; differences:
#   - the record file name leads with the grid family, so the A/A duplicate
#     ("aa") of a public cell is a separate file with identical arguments;
#   - exit 124/137 is a process_cap, any other failure a process_error;
#   - the stopping rule (PROTOCOL.md): within Part A, a cell at (curve, m, engine)
#     is skipped when the public cells (S = 2 and 3) at both of the two
#     previous Part A curves are capped or themselves skipped, per engine;
#   - peak RSS and wall seconds go to a <record>.time sidecar.
# Usage: run_shard.sh <binary> <out_dir> <m,m,...>
set -euo pipefail
bin=$1; out=$2; ms=$3
here=$(cd "$(dirname "$0")" && pwd)
mkdir -p "$out"
seed=8609517
curves_order=(k0n23 k1n29 k0n31 k0n37 k0n41)

dead() { # file is a capped or rule-skipped record
  [ -s "$1" ] && grep -Eq '"status":"(process_cap|skipped_stopping_rule)"' "$1"
}
stopped() { # curve m engine -> 0 when the stopping rule skips (curve, m, engine)
  local c=$1 m=$2 eng=$3 i=-1 k fb ok
  for k in "${!curves_order[@]}"; do [ "${curves_order[$k]}" = "$c" ] && i=$k; done
  [ "$i" -ge 2 ] || return 1
  for back in 1 2; do
    for fb in union-2 union-3; do
      dead "$out/koblitzA__${curves_order[$((i-back))]}__m${m}__${fb}__${eng}__public.json" || return 1
    done
  done
  return 0
}

grep -v '^#' "$here/grid.tsv" | while IFS=$'\t' read -r family curve engines bases mvals trials qms cap probes; do
  for m in ${mvals//,/ }; do
    case ",$ms," in *",$m,"*) ;; *) continue ;; esac
    for fb in ${bases//,/ }; do
      for e in ${engines//,/ }; do
        f="$out/${family}__${curve}__m${m}__${fb/:/-}__${e}__${probes}.json"
        [ -e "$f" ] && continue
        if [ "$family" = koblitzA ] || [ "$family" = aa ]; then
          if stopped "$curve" "$m" "$e"; then
            printf '{"schema":"arity-completion-cell/v1","status":"skipped_stopping_rule","m":%d,"factor_base":"%s","engine":"%s","family":"%s","curve_spec":"%s","probes":"%s"}\n' \
              "$m" "$fb" "$e" "$family" "$curve" "$probes" > "$f"
            echo "m=$m $family $curve $fb $e $probes skipped"
            continue
          fi
        fi
        start=$(date +%s.%N)
        set +e
        if [ -x /usr/bin/time ]; then
          RAYON_NUM_THREADS=1 /usr/bin/time -f '%e %M' -o "$f.time" \
            timeout --kill-after=5 "$cap" "$bin" cell "$m" "$fb" "$e" "$trials" "$seed" "$qms" "$probes" "$curve" \
            > "$f.tmp" 2> "$f.err" < /dev/null
        else
          RAYON_NUM_THREADS=1 timeout --kill-after=5 "$cap" "$bin" cell "$m" "$fb" "$e" "$trials" "$seed" "$qms" "$probes" "$curve" \
            > "$f.tmp" 2> "$f.err" < /dev/null
        fi
        rc=$?
        set -e
        end=$(date +%s.%N)
        if [ "$rc" -eq 0 ] && [ -s "$f.tmp" ]; then
          mv "$f.tmp" "$f"
        else
          st=process_error
          { [ "$rc" -eq 124 ] || [ "$rc" -eq 137 ]; } && st=process_cap
          printf '{"schema":"arity-completion-cell/v1","status":"%s","exit":%d,"m":%d,"factor_base":"%s","engine":"%s","family":"%s","curve_spec":"%s","probes":"%s","cap_seconds":%d,"wall_seconds":%s}\n' \
            "$st" "$rc" "$m" "$fb" "$e" "$family" "$curve" "$probes" "$cap" "$(echo "$end - $start" | bc)" > "$f"
          rm -f "$f.tmp"
        fi
        [ -s "$f.err" ] || rm -f "$f.err"
        echo "m=$m $family $curve $fb $e $probes rc=$rc"
      done
    done
  done
done
