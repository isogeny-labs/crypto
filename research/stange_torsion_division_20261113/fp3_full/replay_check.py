#!/usr/bin/env python3
"""Compare a rerun with the saved run, ignoring wall-time fields. usage: replay_check.py saved.json rerun.json"""
import json, sys
def strip(o):
    if isinstance(o, dict):
        return {k: strip(v) for k, v in o.items() if not (k.endswith('sec') or k == 'sec')}
    if isinstance(o, list):
        return [strip(x) for x in o]
    return o
a = strip(json.load(open(sys.argv[1]))); b = strip(json.load(open(sys.argv[2])))
print(sys.argv[1], 'identical (timing fields ignored)' if a == b else 'DIFFERENT')
