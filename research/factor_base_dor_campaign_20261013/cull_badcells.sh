#!/bin/bash
# Cull the two known engine-budget/segfault cells so the rerun proceeds.
cd "$(dirname "$0")"
while true; do
  for pid in $(pgrep -f "dim 4 --symmetric-s4"); do
    ps -p $pid -o etime= 2>/dev/null | rg -q "^[ ]*[0-9]{2,}:" && kill $pid 2>/dev/null
  done
  pgrep -f "summands 3 --dstar" | while read pid; do kill $pid 2>/dev/null; done
  grep -q "ALLROUNDS DONE" rerun345.log 2>/dev/null && { echo done; break; }
  sleep 30
done
