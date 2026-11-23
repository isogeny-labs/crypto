#!/bin/bash
cd "$(dirname "$0")"
( for s in 3 4 5 8 10 13 16 21 24 25; do echo "9 $s"; done
  for s in 3 6 13 19 30 32 34 37 52 55; do echo "6 $s"; done
  for s in 2 4 5 9 15 16 20 35 36 38; do echo "12 $s"; done ) | xargs -P 4 -L 1 ./job.sh
