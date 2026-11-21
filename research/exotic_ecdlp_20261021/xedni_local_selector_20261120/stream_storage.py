#!/usr/bin/env python3
"""Deterministic below-threshold storage for three frozen JSONL evidence streams."""

import argparse
import gzip
import hashlib
import json
from pathlib import Path

BASE = Path(__file__).resolve().parent.parent
LIMIT = 64 << 10
STREAMS = {
    "local": ("xedni_local_selector_20261120", "candidate-rows", 10000,
              "6d7506f8bb57ca02ab4a5b509d7a00a58d45de373fc8f88e6b439f2271c445c0",
              "a6bf51a4d155ac3acf26cb6aad43aa661c82e65768c7174e5ceee00eb18cddca"),
    "wide": ("xedni_wide_period_selector_20261120", "candidate-rows", 5000,
             "951b870c9c5ac12c2bb12e5f8639e3a1d3caa118f77cf2db64c01d7dbf099a1a",
             "420d20d8973726fccf028923c591c4ac660f6e7dc57bbc2d8f15cc1e28fe5720"),
    "relation": ("xedni_wide_period_relation_20261120", "rational-checks", 2000,
                 "b2475a108e69586d99c24e1f101ffee7256111d0b0ca015078ada001677c3e87",
                 "04c2edb25bb63bc111262d347a390402c3ac7340ea1eba9bf859a7d404e15978"),
}


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def paths(key):
    sub, stem, lines_per_part, raw_sha, packed_sha = STREAMS[key]
    directory = BASE / sub
    whole = directory / f"{stem}.jsonl.gz"
    parts = sorted(directory.glob(f"{stem}.part-*.jsonl.gz"))
    return directory, stem, lines_per_part, raw_sha, packed_sha, whole, parts


def verify(key):
    directory, stem, _, raw_sha, packed_sha, whole, parts = paths(key)
    assert parts, key
    assert all(p.stat().st_size <= LIMIT for p in parts)
    raw = b"".join(gzip.decompress(p.read_bytes()) for p in parts)
    assert sha(raw) == raw_sha, key
    if whole.exists():
        assert gzip.decompress(whole.read_bytes()) == raw, key
    return {"stream": key, "raw_sha256": raw_sha,
            "original_packed_sha256": packed_sha, "raw_bytes": len(raw),
            "parts": [{"name": p.name, "bytes": p.stat().st_size,
                       "sha256": sha(p.read_bytes())} for p in parts]}


def shard(key):
    directory, stem, lines_per_part, raw_sha, packed_sha, whole, parts = paths(key)
    assert whole.exists() and not parts, key
    original = whole.read_bytes()
    assert sha(original) == packed_sha, key
    raw = gzip.decompress(original)
    assert sha(raw) == raw_sha, key
    lines = raw.splitlines(keepends=True)
    assert b"".join(lines) == raw
    for index, start in enumerate(range(0, len(lines), lines_per_part)):
        piece = gzip.compress(b"".join(lines[start:start+lines_per_part]), mtime=0)
        assert len(piece) <= LIMIT, (key, index, len(piece))
        (directory / f"{stem}.part-{index:04d}.jsonl.gz").write_bytes(piece)
    verify(key)
    whole.unlink()
    return verify(key)


def hydrate(key):
    directory, stem, _, raw_sha, _, whole, parts = paths(key)
    assert not whole.exists() and parts, key
    raw = b"".join(gzip.decompress(p.read_bytes()) for p in parts)
    assert sha(raw) == raw_sha, key
    whole.write_bytes(gzip.compress(raw, mtime=0))
    return verify(key)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("action", choices=("shard", "hydrate", "verify"))
    parser.add_argument("--stream", choices=tuple(STREAMS))
    args = parser.parse_args()
    keys = (args.stream,) if args.stream else tuple(STREAMS)
    result = [globals()[args.action](key) for key in keys]
    print(json.dumps({"schema": "xedni-stream-storage-v1", "status": "PASS",
                      "action": args.action, "streams": result}, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
