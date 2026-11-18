#!/usr/bin/env bash
# Python-versus-Rust equivalence of the optlevel tool on the six controls, run before the Python was deleted.
# Both tools measure the SAME control binaries (the C kernels the Python tool built) and score against the SAME
# calibration (the Python tool's), so any difference is the tool's.  Per control and round the two tools run back
# to back, the order alternating; the load average is logged before every measurement.
# usage: equivalence.sh PYTHON_TOOL RUST_BIN OUTDIR [ROUNDS [REPEAT [SECONDS]]]
set -euo pipefail
PY=$1; RS=$2; O=$3; ROUNDS=${4:-2}; REPEAT=${5:-5}; SEC=${6:-1.0}
mkdir -p "$O"; export OPTLEVEL_CACHE=$O/cache
log() { echo "$(date -u +%T) load $(cut -d' ' -f1-3 /proc/loadavg) $*" | tee -a "$O/run.log"; }
log "python calibrate"; python3 "$PY" calibrate --out "$O/py.calibration.json" --repeat 3 2>> "$O/py.calibrate.log"
log "rust calibrate --c-kernels (same C builds plus the Rust kernels)"; "$RS" calibrate --c-kernels --out "$O/rs.calibration.json" --repeat 3 2>> "$O/rs.calibrate.log"
CAL=$O/py.calibration.json
best=$(jq -r '.compute.modmuladd_p31.best_variant | split("@")[1]' "$CAL")
bin() { jq -r --arg b "$1" '.build.builds[$b].path' "$CAL"; }
big=$(jq -r '.memory.triad_array_bytes' "$CAL"); n=$(nproc)
declare -A CMD SYM BIN
CMD[positive-simd-incache]="$(bin $best) modmuladd_p31";  SYM[positive-simd-incache]=pass_modmuladd_p31; BIN[positive-simd-incache]=$(bin $best)
CMD[scalar-incache]="$(bin scalar) modmuladd_p31";        SYM[scalar-incache]=pass_modmuladd_p31;        BIN[scalar-incache]=$(bin scalar)
CMD[serial-dependency]="$(bin $best) horner_p31";         SYM[serial-dependency]=pass_horner_p31;        BIN[serial-dependency]=$(bin $best)
CMD[stream-daxpy]="$(bin native) fma_f64 --n $((big / 8))"
CMD[pointer-chase]="$(bin native) chase --n $((big / 64))"
for r in $(seq 1 "$ROUNDS"); do
  for c in positive-simd-incache scalar-incache serial-dependency stream-daxpy pointer-chase; do
    order="py rs"; [ $((r % 2)) = 0 ] && order="rs py"
    for t in $order; do
      m=$O/$c.r$r.$t.measurement.json; rec=$O/$c.r$r.$t.record.json; vs=()
      log "round $r $c $t"
      if [ $t = py ]; then tool=(python3 "$PY"); else tool=("$RS"); fi
      "${tool[@]}" measure --label "$c" --repeat "$REPEAT" --scale "copies:1,$n" --out "$m" -- ${CMD[$c]} --seconds "$SEC" 2>> "$O/measure.log"
      if [ -n "${SYM[$c]:-}" ]; then vs=(--vector-share "$("${tool[@]}" vector-share "${BIN[$c]}" "${SYM[$c]}" | jq -r .vector_share_static)"); fi
      "${tool[@]}" score --calibration "$CAL" --measurement "$m" --out "$rec" "${vs[@]}" > /dev/null
    done
  done
done
log done
