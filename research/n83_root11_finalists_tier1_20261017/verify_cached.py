#!/usr/bin/env python3
"""Run the frozen N83 finalist verifier with exact-content base replay caching.

The first occurrence of each distinct base dump receives the original full
independent replay. Later byte-identical JSON dumps reuse its coordinates,
while every run still checks its summary and target witness independently.
"""

import hashlib
import importlib.util
import json
import sys
from pathlib import Path

if len(sys.argv) < 3:
    raise SystemExit("usage: n83_cached_verify.py VERIFY_PY RESULTS --stratum N")

verify_path = Path(sys.argv.pop(1)).resolve()
spec = importlib.util.spec_from_file_location("n83_frozen_verify_cached", verify_path)
verifier = importlib.util.module_from_spec(spec)
spec.loader.exec_module(verifier)

original = verifier.group.replay_base
cache = {}


def cached_replay_base(dump, summary):
    encoded = json.dumps(dump, sort_keys=True, separators=(",", ":")).encode()
    digest = hashlib.sha256(encoded).hexdigest()
    key = (digest, summary["base_hash"], summary["factor_base_points"],
           summary["orbit_columns"])
    if key not in cache:
        cache[key] = original(dump, summary)
    return cache[key]


verifier.group.replay_base = cached_replay_base
verifier.main()
