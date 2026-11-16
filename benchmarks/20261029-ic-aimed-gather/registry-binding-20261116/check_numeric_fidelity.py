#!/usr/bin/env python3
"""Check inherited scientific JSON values with exact decimal parsing."""
from decimal import Decimal
import hashlib
import json
from pathlib import Path
import sys

ORIGINAL_SHA256 = "505327cf69bf6b6554244ee1214c71c87ee1331ba73dc74e1c36ebaa6388ee57"
REGISTRY_PATH = "../../docs/curves/registry.json"


def equal(a, b):
    if type(a) is not type(b):
        return False
    if isinstance(a, dict):
        return a.keys() == b.keys() and all(equal(a[k], b[k]) for k in a)
    if isinstance(a, list):
        return len(a) == len(b) and all(equal(x, y) for x, y in zip(a, b))
    return a == b


def check(original_path, successor_path):
    raw = Path(original_path).read_bytes()
    if hashlib.sha256(raw).hexdigest() != ORIGINAL_SHA256:
        raise ValueError("original report pin differs")
    original = json.loads(raw, parse_int=Decimal, parse_float=Decimal)
    successor = json.loads(Path(successor_path).read_bytes(), parse_int=Decimal, parse_float=Decimal)
    receipt = successor.pop("current_registry_source_equivalence")
    if receipt["original_report_sha256"] != ORIGINAL_SHA256:
        raise ValueError("receipt original report pin differs")
    before = [r for r in original["raw_data"] if r["path"] == REGISTRY_PATH]
    after = [r for r in successor["raw_data"] if r["path"] == REGISTRY_PATH]
    if len(before) != 1 or len(after) != 1:
        raise ValueError("registry binding is not unique")
    if after[0]["sha256"] != receipt["current_registry_sha256"]:
        raise ValueError("receipt current registry pin differs")
    after[0]["sha256"] = before[0]["sha256"]
    if not equal(original, successor):
        raise ValueError("inherited scientific JSON values changed")


if __name__ == "__main__":
    if len(sys.argv) != 3:
        raise SystemExit("usage: check_numeric_fidelity.py ORIGINAL SUCCESSOR")
    check(*sys.argv[1:])
    print("PASS: inherited scientific JSON values are exactly equal")
