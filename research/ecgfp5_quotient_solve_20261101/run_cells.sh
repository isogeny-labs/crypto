#!/usr/bin/env bash
# Frozen commands of PROTOCOL.md §7.
#   run_cells.sh host            record the host, solver and binary identity
#   run_cells.sh m3 [P...]       the m = 3 validation solves and both controls (default: all primes)
#   run_cells.sh m5 P            one m = 5 quotient cell at prime P (tier L caps)
#   run_cells.sh m5all           the three m = 5 cells in order 4111, 262231, 33554371
#   run_cells.sh launch-m5all    start m5all detached; writes results/m5all.pid and m5all.log
# Amendment A1 (PROTOCOL.md §10): 65551 -> 262231; summary lines no longer stop the script.
# Outputs go to results/<cell>/; nothing is overwritten (a present cell directory aborts).
set -euo pipefail
HERE=$(cd "$(dirname "$0")" && pwd)
BIN="$HERE/bin/qsolve"
R="$HERE/results"
NPROC=$(nproc)
RSS_CAP_GIB=${RSS_CAP_GIB:-12}
WALL_CAP_S=${WALL_CAP_S:-14400}
mkdir -p "$R"

host() {
  {
    echo "{"
    echo "  \"date_utc\": \"$(date -u +%Y-%m-%dT%H:%M:%SZ)\","
    echo "  \"cpu_model\": \"$(grep -m1 'model name' /proc/cpuinfo | cut -d: -f2 | sed 's/^ //')\","
    echo "  \"cpu_mhz_reported\": $(grep -m1 'cpu MHz' /proc/cpuinfo | cut -d: -f2 | tr -d ' '),"
    echo "  \"nproc\": $NPROC,"
    echo "  \"mem_total_kib\": $(grep MemTotal /proc/meminfo | awk '{print $2}'),"
    echo "  \"cpu_flags_avx2\": $(grep -m1 -c -w avx2 /proc/cpuinfo),"
    echo "  \"cpu_flags_avx512f\": $(grep -m1 -c -w avx512f /proc/cpuinfo),"
    echo "  \"msolve_package\": \"$(dpkg-query -W -f='${Package} ${Version}' msolve)\","
    echo "  \"libneogb_sha256\": \"$(sha256sum /lib/x86_64-linux-gnu/libneogb-0.6.5.so | cut -d' ' -f1)\","
    echo "  \"msolve_sha256\": \"$(sha256sum /usr/bin/msolve | cut -d' ' -f1)\","
    echo "  \"qsolve_src_sha256\": \"$(sha256sum "$HERE/src/qsolve.rs" | cut -d' ' -f1)\","
    echo "  \"qsolve_bin_sha256\": \"$(sha256sum "$BIN" | cut -d' ' -f1)\","
    echo "  \"rustc\": \"$(rustc --version)\""
    echo "}"
  } > "$R/host-$(date -u +%Y%m%dT%H%M%SZ).json"
  cat "$R"/host-*.json | tail -16
}

solve() { # dir arm wallcap
  local d=$1 a=$2 wc=$3
  OMP_WAIT_POLICY=PASSIVE "$BIN" run --rss-cap-gib "$RSS_CAP_GIB" --wall-cap-s "$wc" --record "$d/$a.run.json" -- \
    msolve -v 2 -t "$NPROC" -P 1 -f "$d/$a.ms" -o "$d/$a.out" > "$d/$a.log" 2>&1 || true
  local pl=()
  [ -f "$d/$a.planted" ] && pl=(--planted "$d/$a.planted")
  if [ -s "$d/$a.out" ]; then
    "$BIN" check --msolve-out "$d/$a.out" "${pl[@]}" --msolve-log "$d/$a.log" > "$d/$a.check.json" 2>&1 || true
  else
    echo '{"check": "no msolve output (solve did not finish)"}' > "$d/$a.check.json"
  fi
}

PRIMES=(4111 262231 33554371)

m3() {
  local primes=("$@")
  [ ${#primes[@]} -eq 0 ] && primes=("${PRIMES[@]}")
  for p in "${primes[@]}"; do
    for tg in planted:1 random:2; do
      t=${tg%%:*}; s=${tg##*:}
      d="$R/m3_p${p}_${t}"
      [ -e "$d" ] && { echo "refusing to overwrite $d"; exit 1; }
      mkdir -p "$d"
      "$BIN" generate --m 3 --p "$p" --seed "$s" --target "$t" --out "$d" > "$d/generate.out" 2> "$d/generate.err"
      for a in raw_x sym_x raw_u sym_u quot; do solve "$d" "$a" 0; done
      beta=$(grep -o '"beta": [0-9]*' "$d/meta.json" | grep -o '[0-9]*$')
      "$BIN" orbits --m 3 --beta "$beta" --raw "$d/raw_u.out" --quot "$d/quot.out" --sym "$d/sym_u.out" > "$d/orbits.json" 2>&1 || true
      echo "== $d"
      { grep -h -o '"param_degree": [0-9]*\|"planted_coordinates_recovered": [a-z]*\|"check": "[^"]*"' "$d"/*.check.json | paste -sd' '; } || true
      cat "$d/orbits.json" || true
    done
  done
}

m5() {
  local p=$1
  d="$R/m5_p${p}_planted"
  [ -e "$d" ] && { echo "refusing to overwrite $d"; exit 1; }
  mkdir -p "$d"
  "$BIN" generate --m 5 --p "$p" --seed 1 --target planted --out "$d" > "$d/generate.out" 2> "$d/generate.err"
  solve "$d" quot "$WALL_CAP_S"
  echo "== $d"; cat "$d/quot.check.json" || true
  { grep -o '"signal": [0-9a-z]*\|"killed_by_[a-z_]*": [a-z]*\|"wall_seconds": [0-9.]*\|"user_seconds": [0-9.]*\|"maxrss_kib": [0-9]*' "$d/quot.run.json" | paste -sd' '; } || true
}

case "${1:-}" in
  host) host ;;
  m3) shift; m3 "$@" ;;
  m5) m5 "${2:?prime}" ;;
  m5all) for p in "${PRIMES[@]}"; do m5 "$p"; done ;;
  launch-m5all)
    [ -e "$R/m5all.pid" ] && { echo "refusing: $R/m5all.pid exists"; exit 1; }
    nohup "$HERE/run_cells.sh" m5all > "$R/m5all.log" 2>&1 < /dev/null &
    echo $! > "$R/m5all.pid"; echo "launched m5all pid $(cat "$R/m5all.pid")" ;;
  *) echo "usage: $0 host | m3 [P...] | m5 P | m5all | launch-m5all"; exit 2 ;;
esac
