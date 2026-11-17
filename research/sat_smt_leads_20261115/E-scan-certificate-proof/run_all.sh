#!/bin/bash
# Every measured run of this pilot after amendment 2, one process at a time. (17, 2), (17, 3) and
# target 0 of (17, 4) were produced earlier by: run.py raw/p17_$l.jsonl 17 $l 0 6 xor,xproof,cold,warm,free
cd "$(dirname "$0")"
nice python3 run.py raw/p17_4.jsonl 17 4 1 5 xor,xproof
nice python3 run.py raw/p17_5.jsonl 17 5 0 6 xor,xproof
for l in 2 3 4; do nice python3 run.py raw/f13_$l.jsonl 13 $l 0 6 xor,xproof,cold,warm,free; done
nice python3 run.py raw/s11_3.jsonl 11 3 0 6 xor,xproof,cold,warm,free
nice python3 run.py raw/p17_6.jsonl 17 6 0 6 xor,xproof
nice python3 min3.py raw/min_17_3.jsonl 17 3 0 6
E2ROUNDS=2 nice python3 min3.py raw/min_13_4.jsonl 13 4 0 6
echo RUN2DONE
