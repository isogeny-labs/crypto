#!/usr/bin/env python3
"""Seal one new public N83 point for the local five-base screen."""

import hashlib
import importlib.util
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
DOMAIN = b"n83-local-five-base-screen-20261024-v1"
PRIOR_SINGLE_SHA256 = "5378b8e1b3b5e01f4590ddd89610ab39b3fca28ecaaa050013f2031dc3c1bb1b"


def main():
    spec = importlib.util.spec_from_file_location(
        "n83_five_base_group", ROOT / "research/n83_indexed5_20261013/analyze.py")
    group = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(group)
    seed = hashlib.sha256(DOMAIN + (0).to_bytes(4, "little")).digest()
    scalar = 1 + int.from_bytes(seed, "little") % (group.R - 1)
    point = group.multiply(group.G, scalar)
    if point is None or not group.on_curve(point) or group.multiply(point, group.R) is not None:
        raise ValueError("generated point is outside subgroup")
    target = (json.dumps([str(value) for value in point], separators=(",", ":")) + "\n").encode()
    prior = ROOT / "research/n83_fresh_cold_finalists_20261019/fresh_targets.jsonl"
    if (target in prior.read_bytes().splitlines(keepends=True) or
            hashlib.sha256(target).hexdigest() == PRIOR_SINGLE_SHA256):
        raise ValueError("new point overlaps measured points")
    answer = (json.dumps({"scalar": str(scalar), "seed_sha256": seed.hex()},
                         sort_keys=True, separators=(",", ":")) + "\n").encode()
    for name, content in (("target.jsonl", target), ("answer.jsonl", answer)):
        path = HERE / name
        if path.exists() and path.read_bytes() != content:
            raise ValueError(f"sealed {name} differs")
        path.write_bytes(content)
    print(json.dumps({"target_sha256": hashlib.sha256(target).hexdigest(),
                      "answer_sha256": hashlib.sha256(answer).hexdigest()}, sort_keys=True))


if __name__ == "__main__":
    main()
