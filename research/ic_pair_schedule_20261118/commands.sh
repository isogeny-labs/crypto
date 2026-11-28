#!/bin/sh
set -eu
python3 run.py --out raw/B16.json --base 16 --seeds 24 --seed0 0x1c2026111850
python3 run.py --out raw/B32.json --base 32 --seeds 24 --seed0 0x1c2026111840
python3 audit.py raw/B16.json raw/B32.json
python3 summarize.py
sha256sum PROTOCOL.md run.py audit.py summarize.py raw/B16.json raw/B32.json SUMMARY.json
