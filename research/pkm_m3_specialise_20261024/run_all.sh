#!/bin/bash
# Usage: ./run_all.sh QUEUE_NAME...   runs run_one.py sequentially (start two queues for two concurrent runs).
cd "$(dirname "$0")"; for n in "$@"; do
  [ -f runs/$n.mem ] && continue
  echo "$(date -u +%FT%TZ) start $n" >> progress.txt; python3 run_one.py $n; echo "$(date -u +%FT%TZ) end $n $(cat runs/$n.mem)" >> progress.txt
done
