#!/usr/bin/env python3
"""Bind all 64 bridge base-dump copies to the fresh S3 factor-base objects."""

from __future__ import annotations

import argparse
import gzip
import hashlib
import json
from pathlib import Path
import zipfile

import blake3

HERE = Path(__file__).resolve().parent


def require(ok: bool, message: str) -> None:
    if not ok:
        raise ValueError(message)


def sha256(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def audit(archives: list[Path], downloads: Path,
          provider_audit: Path) -> dict:
    require(len(archives) == 4, "four provider archives required")
    plan = json.loads((HERE / "plan.json").read_text())
    build = json.loads((HERE / "receipts/build.json").read_text())
    provider = json.loads(provider_audit.read_text())
    require(plan["schema"] == "n83.bridge-s3-custody-plan/v1"
            and build["schema"] == "n83.bridge-s3-custody-build/v1"
            and build["plan_sha256"] == hashlib.sha256(
                (HERE / "plan.json").read_bytes()).hexdigest()
            and provider["schema"] == "n83.native-bridge-provider-audit/v1"
            and provider["status"] == "PASS_FOUR_BOUND_PROVIDER_ARCHIVES"
            and provider["run_id"] == int(plan["source_run_id"])
            and len(provider["strata"]) == 4,
            "custody or provider provenance differs")
    by_role = {row["role"]: row for row in build["objects"]}
    require(set(by_role) == {"k1107root11", "k2304root11"},
            "S3 factor-base coverage differs")
    summaries = []
    for arm in plan["arms"]:
        role = arm["role"]
        receipt = by_role[role]
        digest = receipt["compressed_blake3"]
        packed = downloads / f"{digest}.jsonl.gz"
        packed_bytes = packed.read_bytes()
        raw = gzip.decompress(packed_bytes)
        require(len(packed_bytes) == receipt["compressed_bytes"]
                and hashlib.sha256(packed_bytes).hexdigest() ==
                receipt["compressed_sha256"]
                and blake3.blake3(packed_bytes).hexdigest() == digest
                and len(raw) == arm["raw_bytes"]
                and hashlib.sha256(raw).hexdigest() == arm["raw_sha256"],
                f"fresh S3 base differs: {role}")
        copies = []
        for stratum, path in enumerate(archives):
            source = provider["strata"][stratum]
            require(source["stratum"] == stratum
                    and sha256(path) == source["artifact_sha256"],
                    f"provider archive differs: stratum {stratum}")
            with zipfile.ZipFile(path) as zipped:
                require(len(zipped.namelist()) == len(set(zipped.namelist())),
                        "duplicate provider ZIP member")
                for offset in range(8):
                    point = 8 * stratum + offset
                    member = (f"results/points/p{point:02d}/runs/"
                              f"{arm['provider_run_dir']}/base_dump.jsonl")
                    info = zipped.getinfo(member)
                    require(info.file_size == arm["raw_bytes"],
                            f"provider base size differs: point {point} {role}")
                    data = zipped.read(member)
                    require(hashlib.sha256(data).hexdigest() == arm["raw_sha256"],
                            f"provider base bytes differ: point {point} {role}")
                    copies.append({"point_index": point, "stratum": stratum,
                                   "member": member,
                                   "raw_sha256": arm["raw_sha256"]})
        require(len(copies) == 32, f"32 provider copies required: {role}")
        summaries.append({"role": role, "s3_uri": receipt["s3_uri"],
                          "compressed_sha256": receipt["compressed_sha256"],
                          "raw_sha256": arm["raw_sha256"],
                          "provider_copy_count": len(copies),
                          "provider_copies": copies})
    return {"schema": "n83.bridge-s3-all-copies-audit/v1",
            "status": "PASS_64_PROVIDER_COPIES_MATCH_TWO_S3_BASES",
            "source_run_id": plan["source_run_id"],
            "provider_audit_sha256": sha256(provider_audit),
            "objects": summaries, "selected_best_total_runtime": None}


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--archives", type=Path, nargs=4, required=True)
    parser.add_argument("--downloads", type=Path, required=True)
    parser.add_argument("--provider-audit", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    result = audit(args.archives, args.downloads, args.provider_audit)
    text = json.dumps(result, sort_keys=True, indent=2) + "\n"
    if args.output.exists():
        require(args.output.read_text() == text, "sealed all-copies audit differs")
    else:
        args.output.write_text(text)
    print(json.dumps({"status": result["status"],
                      "source_run_id": result["source_run_id"]}))


if __name__ == "__main__":
    main()
