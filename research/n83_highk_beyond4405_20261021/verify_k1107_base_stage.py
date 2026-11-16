#!/usr/bin/env python3
"""Replay the current-source K1107 base and its K4405 prefix."""

import argparse
import hashlib
import json
from pathlib import Path
import tarfile

from research.n83_highk_beyond4405_20261021 import run_k1107_base_stage as runner
from research.n83_indexed5_20261013 import analyze as group

PRIOR_SET_SHA256 = "d5584f33527fa38f174007aeaf071a42ab772be1d1021b586fd4142b80e31e2b"
K4405_ARCHIVE_SHA256 = "5b13e49152b0f120400930fa181a5e8cd3e8e45640bc0134389b10945b3ce333"
K4405_MEMBER = ("n83_fixed31_base_stage_merged_20261021_1750/"
                "k4405/base_dump.jsonl")


def canonical(value: object) -> bytes:
    return json.dumps(value, sort_keys=True, separators=(",", ":"),
                      ensure_ascii=False).encode()


def verify(stage: Path, archive: Path) -> dict:
    receipt = json.loads((stage / "receipt.json").read_text())
    summary = json.loads((stage / "stdout.jsonl").read_text())
    dump = json.loads((stage / "base_dump.jsonl").read_text())
    custody = json.loads((runner.HERE / "source_binary_custody.json").read_text())
    command = receipt.get("command", [])
    if (receipt.get("status") != "PROCESS_EXITED" or
            receipt.get("source_sha256") != runner.SOURCE_SHA256 or
            receipt.get("binary_sha256") != custody["binary_sha256"] or
            custody.get("status") != "MATCHED_LOCAL_AND_FRESH_S3_DOWNLOAD" or
            receipt.get("exit_code") != 0 or
            receipt.get("cap_seconds") != runner.CAP_SECONDS or
            len(command) != 5 or
            command[1:4] != [f"construct:83:1:{runner.K}",
                             str(runner.TARGET), "29"] or
            receipt.get("environment_flags") !=
            {"KIC_BASE_BUILD_ONLY": "1",
             "KIC_DUMP_BASE": str(stage / "base_dump.jsonl")} or
            receipt.get("target_sha256") != runner.sha256(runner.TARGET) or
            receipt.get("stdout_sha256") != runner.sha256(stage / "stdout.jsonl") or
            receipt.get("base_dump_sha256") !=
            runner.sha256(stage / "base_dump.jsonl") or
            receipt.get("summary") != summary or
            runner.sha256(runner.SOURCE) != runner.SOURCE_SHA256 or
            summary.get("kind") != "compact_orbit_base_stage" or
            summary.get("n") != group.N or summary.get("a") != 1 or
            summary.get("subgroup_order") != group.R or
            group.point_from_json(summary["generator"]) != group.G or
            summary.get("orbit_columns") != runner.K or
            summary.get("factor_base_points") != 166 * runner.K or
            summary.get("base_hash") != runner.PRIOR_BASE_HASH or
            dump.get("base_hash") != runner.PRIOR_BASE_HASH or
            sorted(summary["field_modulus_low_terms"]) !=
            [i for i in range(group.N) if group.MOD & (1 << i)]):
        raise ValueError("K1107 source, field, subgroup or base receipt differs")
    points = group.replay_base(dump, summary)
    if len(points) != 166 * runner.K:
        raise ValueError("K1107 usable count differs")
    ordered = [[str(x), str(y)] for x, y in sorted(points)]
    set_sha = hashlib.sha256(canonical(ordered)).hexdigest()
    if set_sha != PRIOR_SET_SHA256:
        raise ValueError("K1107 enumerated set differs from prior finalist")
    if runner.sha256(archive) != K4405_ARCHIVE_SHA256:
        raise ValueError("current-source K4405 archive differs")
    with tarfile.open(archive, "r:gz") as bundle:
        stream = bundle.extractfile(K4405_MEMBER)
        if stream is None:
            raise ValueError("K4405 archive member absent")
        k4405 = json.load(stream)
    for key, count in (("factor_base_point_coordinates", 166 * runner.K),
                       ("factor_base_point_labels", 166 * runner.K),
                       ("factor_base_representatives", runner.K)):
        if dump[key] != k4405[key][:count]:
            raise ValueError(f"K1107 ordered K4405 prefix differs: {key}")
    return {
        "schema": "n83.fixed31-lowerk-base-replay/v1",
        "status": "PASS_INDEPENDENT_BASE_REPLAY",
        "source_sha256": runner.SOURCE_SHA256,
        "binary_sha256": receipt["binary_sha256"],
        "usable_points": len(points),
        "folded_columns": runner.K,
        "base_hash_blake3": summary["base_hash"],
        "base_dump_sha256": receipt["base_dump_sha256"],
        "enumerated_set_sha256": set_sha,
        "k4405_archive_sha256": K4405_ARCHIVE_SHA256,
        "ordered_k4405_prefix": True,
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--base-stage", type=Path, required=True)
    parser.add_argument("--k4405-archive", type=Path, required=True)
    args = parser.parse_args()
    stage = args.base_stage.resolve(strict=True)
    result = verify(stage, args.k4405_archive.resolve(strict=True))
    output = stage / "independent_replay.json"
    blob = json.dumps(result, indent=2, sort_keys=True) + "\n"
    if output.exists() and output.read_text() != blob:
        raise ValueError("K1107 independent replay differs")
    output.write_text(blob)
    print(json.dumps({"status": result["status"],
                      "usable_points": result["usable_points"],
                      "ordered_k4405_prefix": result["ordered_k4405_prefix"]},
                     sort_keys=True))


if __name__ == "__main__":
    main()
