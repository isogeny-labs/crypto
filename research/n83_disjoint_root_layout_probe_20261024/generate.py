#!/usr/bin/env python3
"""Deterministic public point disjoint from the frozen 16-point N83 panel."""

import hashlib
import importlib.util
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
DOMAIN = b"n83-modal-root-layout-probe-20261024-v1"


def main():
    source = ROOT / "research/n83_indexed5_20261013/analyze.py"
    spec = importlib.util.spec_from_file_location("n83_probe_group", source)
    group = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(group)
    seed = hashlib.sha256(DOMAIN + (0).to_bytes(4, "little")).digest()
    scalar = 1 + int.from_bytes(seed, "little") % (group.R - 1)
    point = group.multiply(group.G, scalar)
    if point is None or not group.on_curve(point) or group.multiply(point, group.R) is not None:
        raise ValueError("generated point is outside subgroup")
    prior = ROOT / "research/n83_fresh_cold_finalists_20261019/fresh_targets.jsonl"
    target = (json.dumps([str(value) for value in point], separators=(",", ":")) + "\n").encode()
    if target in prior.read_bytes().splitlines(keepends=True):
        raise ValueError("point overlaps frozen 16-point panel")
    answer = (json.dumps({"scalar": str(scalar), "seed_sha256": seed.hex()},
                         sort_keys=True, separators=(",", ":")) + "\n").encode()
    for name, content in (("target.jsonl", target), ("answer.jsonl", answer)):
        path = HERE / name
        if path.exists() and path.read_bytes() != content:
            raise ValueError(f"sealed {name} differs")
        path.write_bytes(content)
    print(json.dumps({"target_sha256": hashlib.sha256(target).hexdigest(),
                      "answer_sha256": hashlib.sha256(answer).hexdigest(),
                      "scalar": str(scalar)}, sort_keys=True))


if __name__ == "__main__":
    main()
