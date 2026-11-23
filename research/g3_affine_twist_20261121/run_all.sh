#!/bin/sh
# Sequential, single-threaded run of the frozen protocol.
PY=/tmp/claude-0/-home-user/c9b2fe43-c085-5dfa-9cde-b0311430c5d1/scratchpad/venv_g3/bin/python
cd "$(dirname "$0")"
for p in 17 19 29 31 37 41 43 47; do
  echo "control $p $(date -u +%T)"; $PY run.py control $p 6 data/control.jsonl || echo "FAIL control $p"
done
for p in 17 19 29 31 37 41 43 47; do
  echo "main $p $(date -u +%T)"; $PY run.py main $p 100 data/main.jsonl || echo "FAIL main $p"
done
echo "done $(date -u +%T)"
