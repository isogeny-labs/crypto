#!/usr/bin/env bash
# Paired A/B(/A2/B0) cover-IC runs of PROTOCOL.md, one thread, idempotent (skips finished files).
#   run.sh <jv_cover binary> <outdir> [cap_seconds] [phase]      phase: small | p251 | p503
set -uo pipefail
BIN=$1; OUT=$2; CAP=${3:-36000}; PHASE=${4:-small}
mkdir -p "$OUT/raw"
RAW=$OUT/raw
{
  echo "host $(lscpu | sed -n 's/^Model name: *//p') nproc $(nproc)"
  echo "jv_cover sha256 $(sha256sum "$BIN" | cut -c1-64)"
  echo "git $(git rev-parse HEAD 2>/dev/null) (modified source files: $(git status --short -- src examples 2>/dev/null | grep -v '^ D' | wc -l))"
} >> "$RAW/versions.txt"
now() { date +%s; }
spent() { awk '{s+=$NF} END{print s+0}' "$RAW/walls.txt" 2>/dev/null; }
cell() { # p seed arm rho_runs
  local p=$1 s=$2 arm=$3 rr=$4 f="$RAW/p${p}_s${s}_${3}"
  [ -s "$f.json" ] && return 0
  if [ "$(spent)" -ge "$CAP" ]; then echo "CAP reached before p=$p seed=$s arm=$arm" >> "$RAW/notrun.txt"; return 1; fi
  local stop=() envs=(RAYON_NUM_THREADS=1)
  [ "$arm" = B ] && stop=(--stop hilbert:16)
  [ "$arm" = B0 ] && { stop=(--stop hilbert:1); envs+=(F4_STAIR=scan); }
  local cmd=(env "${envs[@]}" "$BIN" --exp dlp --sizes "$p" --seeds 1 --first-seed "$s" --rho-runs "$rr" --rho-ref 1.3 --check-every 64 "${stop[@]}" --json "$f.json")
  echo "${cmd[*]}" >> "$RAW/commands.txt"
  local t0; t0=$(now)
  "${cmd[@]}" > "$f.log" 2>&1
  echo "p$p s$s $arm exit $? wall_s $(( $(now) - t0 ))" >> "$RAW/walls.txt"
}
case $PHASE in
small)
  for s in 1 2 3 4 5 6 7 8; do cell 53 $s A 8; cell 53 $s B 0; done
  for s in 1 2; do cell 53 $s A2 8; cell 53 $s B0 0; done
  for s in 1 2 3 4 5 6 7 8; do cell 101 $s A 8; cell 101 $s B 0; done
  cell 101 1 A2 8; cell 101 1 B0 0 ;;
p251) for s in 1 2 3 4 5 6; do cell 251 $s A 4; cell 251 $s B 0; done ;;
p503) for s in 1 2 3; do cell 503 $s A 0; cell 503 $s B 0; done ;;
esac
