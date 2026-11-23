#!/bin/sh
# kill duplicate xargs launches of cells already finished by run_extra.sh
cd "$(dirname "$0")"
while true; do
  for f in results/dlp_251_*.json results/dlp_509_*.json; do
    [ -s "$f" ] || continue
    b=$(basename $f .json); p=$(echo $b | cut -d_ -f2); c=$(echo $b | cut -d_ -f3)
    for pid in $(pgrep -f "g4lp_bin dlp $p $c "); do
      age=$(ps -o etimes= -p $pid | tr -d ' ')
      [ "$age" -lt 120 ] && kill $pid
    done
  done
  sleep 20
done
