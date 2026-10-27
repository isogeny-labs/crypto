"""Independently replay every point and orbit label of an N83 base-only run."""

import argparse
import gzip
import hashlib
import json
from pathlib import Path
import sys


REPO = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO))
from research.n83_indexed5_20261013 import analyze as group  # noqa: E402


def sha256(path):
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def load_dump(root):
    plain = root / "base_dump.jsonl"
    compressed = root / "base_dump.jsonl.gz"
    if plain.exists():
        return json.loads(plain.read_text()), sha256(plain)
    if compressed.exists():
        digest = hashlib.sha256()
        with gzip.open(compressed, "rb") as stream:
            chunks = []
            for chunk in iter(lambda: stream.read(1024 * 1024), b""):
                digest.update(chunk)
                chunks.append(chunk)
        return json.loads(b"".join(chunks)), digest.hexdigest()
    raise ValueError("base dump is absent")


def replay(root):
    input_record = json.loads((root / "input.json").read_text())
    process = json.loads((root / "process_result.json").read_text())
    if process["status"] != "PASS_PROCESS" or process["exit_code"] != 0:
        return {"status": ("BOUNDED_UNKNOWN" if process["status"] in
                           ("TIMEOUT", "RSS_CAP") else "PRODUCER_FAILURE"),
                "process": process, "candidate_id": None}
    try:
        for name, path in (
            ("source_sha256", root / "source_snapshot.rs"),
            ("binary_sha256", root / "executable_snapshot"),
            ("runner_sha256", root / "runner.py"),
            ("target_sha256", root / "target.jsonl"),
        ):
            if sha256(path) != input_record[name]:
                raise ValueError(f"{name} differs from frozen input")
        if (process["sampled_peak_rss_bytes"] > input_record["rss_cap_bytes"] or
                (root / "targets_result.jsonl").stat().st_size != 0):
            raise ValueError("resource cap or stage-only output differs")
        lines = (root / "stdout.jsonl").read_text().splitlines()
        if len(lines) != 1:
            raise ValueError("base stage must emit exactly one summary")
        summary = json.loads(lines[0])
        dump, dump_sha256 = load_dump(root)
        k = input_record["k"]
        expected_low_terms = [i for i in range(group.N)
                              if group.MOD & (1 << i)]
        if (summary.get("kind") != "compact_orbit_base_stage" or
                summary.get("status") != "STAGE_DIAGNOSTIC_ONLY" or
                summary.get("n") != group.N or summary.get("a") != 1 or
                summary.get("subgroup_order") != group.R or
                group.point_from_json(summary.get("generator")) != group.G or
                sorted(summary.get("field_modulus_low_terms", [])) !=
                expected_low_terms or
                sorted(dump.get("field_modulus_low_terms", [])) !=
                expected_low_terms or
                summary.get("orbit_columns") != k or
                summary.get("factor_base_points") != 166 * k or
                dump.get("factor_base_points") != 166 * k or
                dump.get("base_hash") != summary.get("base_hash") or
                summary.get("base_dump_path") !=
                input_record["environment_flags"]["KIC_DUMP_BASE"]):
            raise ValueError("curve, subgroup, base shape, or dump identity differs")
        base = group.replay_base(dump, summary)
        if len(base) != 166 * k:
            raise ValueError("independent point replay count differs")
        return {"status": "PASS_INDEPENDENT_BASE_REPLAY",
                "candidate_id": None, "orbit_columns": k,
                "actual_usable_points": len(base),
                "base_hash": summary["base_hash"],
                "base_dump_sha256": dump_sha256,
                "base_scanned_x": summary["base_scanned_x"],
                "base_construction_ms": summary["base_construction_ms"],
                "process_wall_seconds": process["wall_seconds"],
                "sampled_peak_rss_bytes": process["sampled_peak_rss_bytes"],
                "source_sha256": input_record["source_sha256"],
                "binary_sha256": input_record["binary_sha256"]}
    except Exception as error:
        return {"status": "VERIFICATION_FAILED", "candidate_id": None,
                "error": str(error)}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("root", type=Path)
    args = parser.parse_args()
    root = args.root.resolve(strict=True)
    result = replay(root)
    (root / "verification.json").write_text(
        json.dumps(result, indent=2, sort_keys=True) + "\n")
    print(json.dumps(result, sort_keys=True), flush=True)
    return 0 if result["status"] == "PASS_INDEPENDENT_BASE_REPLAY" else 1


if __name__ == "__main__":
    raise SystemExit(main())
