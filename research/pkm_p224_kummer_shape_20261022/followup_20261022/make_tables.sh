#!/bin/bash
# Tables of the follow-up (PROTOCOL.md 12.3) from the recorded runs; prints Markdown to stdout.
#   make_tables.sh e2     # memory: baseline against fixed binary, per cell
#   make_tables.sh steps ID   # per-step table of a fixed-binary run (runs/ID.steps.jsonl)
set -eu
here=$(cd "$(dirname "$0")" && pwd)
cd "$here/runs"
peak_log() {  # VmHWM at the last traced step of a baseline log, MB
  grep -o 'peak [0-9]* MB' "$1" | tail -1 | awk '{print $2}'
}
peak_steps() { jq -s 'map(.peak_rss_mb) | max' "$1"; }
sampled() { awk 'BEGIN{m=0} {if($2>m)m=$2} END{printf "%d", m/1024}' "$1"; }
wall() { jq -r 'select(.N != null) | (.wall_s // 0 | floor)' "$1" | head -1; }
muladds() { jq -r 'select(.N != null) | .muladds' "$1" | head -1; }
case ${1:?e2 or steps} in
  e2)
    echo "| cell | arm | peak resident, VmHWM at the last step (MB) | peak sampled every 5 s (MB) | resident after the last step (MB) | wall (s, shared host) | muladds |"
    echo "|:--|:--|--:|--:|--:|--:|--:|"
    for n in 20 22; do
      for arm in base fixed fixed-arena2; do
        id=R20e-N$n-$arm
        [ -f $id.jsonl ] || continue
        if [ $arm = base ]; then
          pk=$(peak_log $id.log)
          last=$(grep -o 'memory [0-9]* MB' $id.log | tail -1 | awk '{print $2}')
        else
          pk=$(peak_steps $id.steps.jsonl)
          last=$(jq -s 'last | .rss_mb' $id.steps.jsonl)
        fi
        echo "| SL24 N=$n target 0, 2 threads | $arm | $pk | $(sampled $id.rss) | $last | $(wall $id.jsonl) | $(muladds $id.jsonl) |"
      done
    done
    ;;
  steps)
    id=${2:?run id}
    echo "| step | degree | new elements | cols | B' entries | B' non-zero | non-zero share | muladds | step s | resident after (MB) | peak (MB) |"
    echo "|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|"
    jq -r '[.step,.degree,.fresh,.cols,.dense_entries,.dense_nonzeros,(if .dense_entries>0 then (100*.dense_nonzeros/.dense_entries|floor|tostring)+"%" else "-" end),.muladds,(.ms/1000|floor),.rss_mb,.peak_rss_mb] | "| " + (map(tostring)|join(" | ")) + " |"' $id.steps.jsonl
    ;;
esac
