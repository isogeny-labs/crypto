#!/usr/bin/env bash
# smoke, not data: smallest case (k = 16, N = 18), B = 40 bases, one core, <= 60 s CPU.
cd "$(dirname "$0")/.." && ( time python3 run_l1.py --out smoke/out --k 16 --sizes 0 --B 40 --smoke ) 2> smoke/smoke.log
