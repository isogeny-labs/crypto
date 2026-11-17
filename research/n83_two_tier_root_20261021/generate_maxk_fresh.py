#!/usr/bin/env python3
"""Seal disjoint public N83 max-K screen and one-target holdout fixtures."""

import hashlib
import importlib.util
import json
from pathlib import Path


HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
DOMAINS = {
    "screen": (b"n83-maxk-screen192-20261021-v1", 192),
    "holdout": (b"n83-maxk-holdout8-20261021-v1", 8),
}
spec = importlib.util.spec_from_file_location(
    "n83_maxk_group", ROOT / "research/n83_indexed5_20261013/analyze.py")
group = importlib.util.module_from_spec(spec)
spec.loader.exec_module(group)


def digest(blob):
    return hashlib.sha256(blob).hexdigest()


def write_once(path, blob):
    if path.exists() and path.read_bytes() != blob:
        raise ValueError(f"frozen fixture changed: {path}")
    path.write_bytes(blob)


def generate():
    seen = set()
    manifest = {
        "schema": "n83.maxk-fixture/v1",
        "curve_id": "EC1N83Ckb1a1haacb69ee9f37",
        "subgroup_order": group.R,
        "scalar_rule": "1+little_endian(SHA256(domain||u32_le(index))) mod (r-1)",
        "generator_sha256": digest(Path(__file__).read_bytes()),
    }
    for kind, (domain, count) in DOMAINS.items():
        targets, answers = [], []
        for index in range(count):
            seed = hashlib.sha256(domain + index.to_bytes(4, "little")).digest()
            scalar = 1 + int.from_bytes(seed, "little") % (group.R - 1)
            point = group.multiply(group.G, scalar)
            if (point is None or not group.on_curve(point) or
                    group.multiply(point, group.R) is not None or point in seen):
                raise ValueError(f"{kind} target {index} is invalid or duplicate")
            seen.add(point)
            targets.append(json.dumps([str(x) for x in point], separators=(",", ":")) + "\n")
            answers.append(json.dumps({
                "fixture_index": index, "scalar": str(scalar),
                "seed_sha256": seed.hex(),
            }, separators=(",", ":"), sort_keys=True) + "\n")
        target_blob = "".join(targets).encode()
        answer_blob = "".join(answers).encode()
        write_once(HERE / f"maxk_{kind}_targets.jsonl", target_blob)
        write_once(HERE / f"maxk_{kind}_answers.jsonl", answer_blob)
        manifest[kind] = {
            "domain": domain.decode(), "point_count": count,
            "targets_sha256": digest(target_blob),
            "answers_sha256": digest(answer_blob),
        }
    manifest["status"] = "SEALED_UNMEASURED"
    write_once(HERE / "maxk_fresh_manifest.json",
               (json.dumps(manifest, indent=2, sort_keys=True) + "\n").encode())
    print(json.dumps(manifest, sort_keys=True))


if __name__ == "__main__":
    generate()
