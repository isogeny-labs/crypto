#!/bin/sh
# The frozen claim grid (PROTOCOL.md §3). Thin orchestration only: the native
# instrument does the work. One cell per process, one job at a time, so each
# record's VmHWM is its own cell's. Writes runs/ (must not exist).
set -eu
cd "$(dirname "$0")/../.."
B=target/release/examples/ic_algebraic_floor
S=research/groebner_free_matched_20261113
test ! -e $S/runs
mkdir -p $S/runs
sha256sum $B > $S/runs/binary.sha256
git rev-parse HEAD > $S/runs/commit.txt
for c in 0:13:5 1:17:6 1:19:7 1:23:8 1:29:10 0:31:11; do
  $B run --m 3 --cells $c --arms f4-chain,lin21,lin21-ht --pool 2048 --kyes 6 --kno 6 \
    --budget-secs 120 --mem-gib 4 --stop-after 4 --seed 20261014 --out $S/runs 2>> $S/runs.log
  $B linpool --cells $c --pool 2048 --seed 20261014 --out $S/runs 2>> $S/runs.log
done
for c in 0:37:13 0:41:14; do
  $B run --m 3 --cells $c --arms lin21,lin21-ht --pool 2048 --kyes 6 --kno 6 \
    --budget-secs 120 --mem-gib 4 --stop-after 4 --seed 20261014 --out $S/runs 2>> $S/runs.log
  $B linpool --cells $c --pool 2048 --seed 20261014 --out $S/runs 2>> $S/runs.log
done
$B linpool --cells 1:47:16 --pool 2048 --seed 20261014 --out $S/runs 2>> $S/runs.log
$B linpool --cells 0:53:18 --pool 1024 --seed 20261014 --out $S/runs 2>> $S/runs.log
$B linpool --cells 1:59:20 --pool 512 --seed 20261014 --out $S/runs 2>> $S/runs.log
