#!/bin/bash
cd "$(dirname "$0")"
f=../out/exh/G_$(echo "$@" | tr ' ' _).json
[ -s "$f" ] && exit 0
python3 exhaustive_G.py "$@" > "$f.tmp" 2> "$f.err" && mv "$f.tmp" "$f"
