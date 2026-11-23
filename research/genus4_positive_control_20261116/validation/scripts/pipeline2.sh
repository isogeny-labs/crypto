#!/usr/bin/env bash
# Replays with the hash-verified copies of the producer's binaries (equal to binary.sha256),
# sequential, 1 thread each; then the dumper once the validator-built val_g4dump exists.
G=/tmp/claude-0/-home-user/c9b2fe43-c085-5dfa-9cde-b0311430c5d1/scratchpad/g4val
B=$G/bin/producer/genus_ladder.g4 TAG=_prodbin $G/scripts/replay_s1.sh > $G/out/replay_s1_prodbin.out 2>&1
B=$G/bin/producer/genus_ladder.rhorefs TAG=_prodbin $G/scripts/replay_s2.sh > $G/out/replay_s2_prodbin.out 2>&1
until [ -f $G/bin/val_g4dump.a99e7267f1 ]; do sleep 20; done
sha256sum $G/bin/val_g4dump.a99e7267f1 > $G/out/dump.sha256
$G/bin/val_g4dump.a99e7267f1 < $G/dump_in_main.txt > $G/out/dump_main.jsonl 2> $G/out/dump_main.err
echo "dump_main rc=$? $(date -u +%FT%TZ)" >> $G/out/dump.sha256
$G/bin/val_g4dump.a99e7267f1 < $G/dump_in_rho.txt > $G/out/dump_rho.jsonl 2> $G/out/dump_rho.err
echo "dump_rho rc=$? $(date -u +%FT%TZ)" >> $G/out/dump.sha256
echo "pipeline2 done $(date -u +%FT%TZ)"
