#!/usr/bin/env python3
"""Independently replay the three current-source N83 factor bases."""

import argparse
import hashlib
import json
from pathlib import Path
import sys

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
sys.path.insert(0, str(ROOT))
from research.n83_indexed5_20261013 import analyze as group  # noqa: E402

SOURCE_SHA256 = "58e3ee492093de5f6b760b96e3aba819fa479cdecd22b1adee94f0ecb26026d8"
EXPECTED_BASE_HASHES = {
    4405: "d64fcee7ba4975873859d256302872681c82b99fbe687ff4fba0bf3ba3a4c8d2",
    4500: "6f3305328b265b70f7a2d53e97be2c4b70c3da291ef4cca4693dac6dae468f95",
    4549: "65bcc74cd45d33881bcaad17216fe724d2666072230230ed6868eb7187b968af",
}


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def canonical(value: object) -> bytes:
    return json.dumps(value, sort_keys=True, separators=(",", ":"),
                      ensure_ascii=False).encode()


def replay(stage: Path) -> dict:
    catalog = json.loads((HERE / "identities/catalog.json").read_text())
    if catalog["source_sha256"] != SOURCE_SHA256:
        raise ValueError("current candidate catalog has different source")
    current = {}
    first_dump = None
    binary_sha = None
    for k, base_hash in EXPECTED_BASE_HASHES.items():
        arm = stage / f"k{k}"
        receipt = json.loads((arm / "receipt.json").read_text())
        summary = json.loads((arm / "stdout.jsonl").read_text())
        dump = json.loads((arm / "base_dump.jsonl").read_text())
        if (receipt["status"] != "COMPLETE" or
                receipt["source_sha256"] != SOURCE_SHA256 or
                receipt["stdout_sha256"] != sha256(arm / "stdout.jsonl") or
                receipt["base_dump_sha256"] != sha256(arm / "base_dump.jsonl") or
                receipt["summary"] != summary or
                summary["kind"] != "compact_orbit_base_stage" or
                summary["n"] != group.N or summary["a"] != 1 or
                summary["subgroup_order"] != group.R or
                group.point_from_json(summary["generator"]) != group.G or
                summary["orbit_columns"] != k or
                summary["factor_base_points"] != 166 * k or
                summary["base_hash"] != base_hash or
                sorted(summary["field_modulus_low_terms"]) !=
                [i for i in range(group.N) if group.MOD & (1 << i)] or
                dump["base_hash"] != base_hash):
            raise ValueError(f"K{k} source, curve, count or archive differs")
        if binary_sha is None:
            binary_sha = receipt["binary_sha256"]
        elif receipt["binary_sha256"] != binary_sha:
            raise ValueError("base arms used different binaries")
        if first_dump is None:
            first_dump = dump
        else:
            for key in ("factor_base_point_coordinates", "factor_base_point_labels"):
                if dump[key][:166 * 4405] != first_dump[key]:
                    raise ValueError(f"K4405 prefix differs in K{k} {key}")
            if dump["factor_base_representatives"][:4405] != first_dump[
                    "factor_base_representatives"]:
                raise ValueError(f"K4405 representatives differ in K{k}")
        points = group.replay_base(dump, summary)
        ordered = [[str(x), str(y)] for x, y in sorted(points)]
        set_sha = hashlib.sha256(canonical(ordered)).hexdigest()
        matched = [row for row in catalog["candidates"] if row["k"] == k]
        if not matched:
            raise ValueError(f"K{k} candidate absent")
        for row in matched:
            manifest = json.loads((HERE / "identities" /
                                   f"candidate_{row['candidate_id']}.json").read_text())
            base = manifest["factor_base"]
            if (base["enumerated_set_sha256"] != set_sha or
                    base["ordered_base_dump_sha256"] != receipt["base_dump_sha256"] or
                    base["source_base_hash_blake3"] != base_hash):
                raise ValueError(f"K{k} candidate base differs")
        current[f"k{k}"] = {
            "status": "PASS_INDEPENDENT_BASE_REPLAY",
            "usable_points": len(points),
            "folded_columns": k,
            "base_hash_blake3": base_hash,
            "base_dump_sha256": receipt["base_dump_sha256"],
            "enumerated_set_sha256": set_sha,
            "ordered_k4405_prefix": True,
        }
        print(json.dumps({"k": k, "status": current[f"k{k}"]["status"]}),
              flush=True)
    return {"schema": "n83.fixed31-base-independent-replay/v1",
            "source_sha256": SOURCE_SHA256,
            "binary_sha256": binary_sha,
            "status": "PASS_INDEPENDENT_BASE_REPLAY",
            "arms": current}


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--base-stage", type=Path, required=True)
    arguments = parser.parse_args()
    report = replay(arguments.base_stage.resolve(strict=True))
    path = HERE / "base_stage_validation.json"
    blob = (json.dumps(report, indent=2, sort_keys=True) + "\n").encode()
    if path.exists() and path.read_bytes() != blob:
        raise ValueError("frozen independent replay differs")
    path.write_bytes(blob)
    print(json.dumps({"status": report["status"],
                      "arms": list(report["arms"])}, sort_keys=True))
