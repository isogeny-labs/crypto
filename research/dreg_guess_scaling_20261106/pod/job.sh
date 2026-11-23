#!/usr/bin/env bash
# Amendment 1: every remaining branch of PREREGISTRATION.md (S5) and of
# PREREG_SUMMAND.md (W.l9.d7) on one RunPod CPU pod, from the uploaded,
# hash-checked files in /workspace/in. Thin orchestration only: every
# computation is dreg_linear_guess --engine pe.
set -u
IN=/workspace/in; OUT=/workspace/out; R=$OUT/runs
mkdir -p "$R"
B=$IN/dreg_linear_guess; chmod +x "$B"
export F4_F2_MAX_ROWS=50000000 F4_F2_MAX_COLS=50000000 RAYON_NUM_THREADS=2
status() { echo "$(date -u +%FT%TZ) $1" >> "$OUT/STATUS"; }
{ echo "date: $(date -u +%FT%TZ)"; echo "pod: ${RUNPOD_POD_ID:-?}"
  sha256sum "$B" "$IN/m3-25-9.jsonl" "$IN/commands.txt"
  grep MemTotal /proc/meminfo; nproc; lscpu | grep 'Model name'; } >> "$R/host.txt"
one() {
  name=$1; shift
  [ -s "$R/$name.jsonl" ] && return 0
  echo "$(date -u +%FT%TZ) $name: start $*" >> "$R/queue.log"
  s=$(date +%s)
  timeout 7200 "$B" --engine pe --batch 64 --to 4 --max-cols 230000 --check "$@" "$IN/m3-25-9.jsonl" \
    > "$R/$name.jsonl" 2> "$R/$name.log"
  echo "$(date -u +%FT%TZ) $name: exit $? after $(( $(date +%s) - s )) s" >> "$R/queue.log"
}
export -f one; export B R IN
status "START parallel=31"
xargs -P 31 -L 1 bash -c 'one "$@"' _ < "$IN/commands.txt"
status "ALLDONE"
