#!/usr/bin/env python3
"""Recompute bounded prime61 pilot claims from exact restricted archives."""

import argparse
import collections
import hashlib
import json
import tarfile
from pathlib import Path


HERE = Path(__file__).resolve().parent
DEFAULT_ARCHIVES = Path("/mnt/spot-codex-state/tmp/ml-dataset-prime61-root-enum-20261031/archives")
P = "2305843009213693951"
METHODS = ("root", "enumerate")


def digest(data):
    return hashlib.sha256(data).hexdigest()


def read(path):
    return json.loads(path.read_text())


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--archives", type=Path, default=DEFAULT_ARCHIVES)
    args = ap.parse_args()
    plan, protocol = read(HERE / "plan.json"), read(HERE / "protocol.json")
    assert protocol["freeze"]["status"] == "frozen"
    assert plan["protocol_sha256"] == digest((HERE / "protocol.json").read_bytes())
    assert plan["field_sizes"] == [61] and plan["families"] == ["prime"]
    assert plan["arities"] == [2] and plan["solvers"] == list(METHODS)
    rows, statuses = [], collections.Counter()
    for index in range(4):
        checkpoint = read(HERE / "checkpoints" / f"shard-{index:02d}.json")
        custody, published = checkpoint["custody"], checkpoint["published"]
        assert checkpoint["index"] == custody["index"] == index
        assert custody["plan_file_sha256"] == digest((HERE / "plan.json").read_bytes())
        assert custody["protocol_file_sha256"] == plan["protocol_sha256"]
        assert published["archive_fresh_get"] and published["pointer_fresh_get"]
        assert published["archive_sha256"] == custody["archive_sha256"]
        archive = args.archives / f"shard-{index:02d}.tar.gz"
        raw = archive.read_bytes()
        assert digest(raw) == custody["archive_sha256"] and len(raw) == custody["archive_bytes"]
        prefix = f"shard-{index:02d}/runner/attempt/cells/{index:012d}/"
        with tarfile.open(archive, "r:gz") as tar:
            manifest_members = [m for m in tar.getmembers() if m.name == prefix + "manifest.json"]
            public_members = [m for m in tar.getmembers() if m.name == prefix + "public.json"]
            assert len(manifest_members) == len(public_members) == 1
            assert manifest_members[0].isfile() and public_members[0].isfile()
            manifest_raw = tar.extractfile(manifest_members[0]).read()
            public_raw = tar.extractfile(public_members[0]).read()
        manifest, public = json.loads(manifest_raw), json.loads(public_raw)
        method, twist = METHODS[index % 2], bool((index // 2) % 2)
        assert manifest["cell_index"] == index
        assert manifest["config"]["family"] == "prime" and manifest["config"]["n"] == 61
        assert manifest["config"]["m"] == 2 and manifest["config"]["solver"] == method
        assert manifest["config"]["twist"] == twist and public["twist"] == twist
        assert public["field"]["modulus"] == P
        status = custody["native_status"]
        assert status["verified_and_exported"] and status["mersenne_modulus_verified"]
        assert status["cell_spec"] == {"solver": method, "twist": twist,
                                      "generator_index": 0, "key_index": 0}
        assert status["native_status"] == manifest["outcome"]["status"]
        assert not status["ecdlp_solve_promoted"]
        statuses[status["native_status"]] += 1
        rows.append({"index": index, "method": method, "twist": twist,
            "native_status": status["native_status"], "modulus": P,
            "archive_sha256": custody["archive_sha256"],
            "manifest_sha256": digest(manifest_raw), "public_sha256": digest(public_raw),
            "s3_uri": custody["s3_uri"]})
    assert statuses == {"timeout": 2, "verified": 2}
    export = read(HERE / "experiment-export.json")
    receipt = read(HERE / "experiment-export-producer-receipt.json")
    assert receipt["export_sha256"] == digest((HERE / "experiment-export.json").read_bytes())
    assert receipt["official_checker_exit_code"] == 0
    assert export["flow"] == {"planned": 4, "attempted": 4, "completed": 2,
                              "not_completed": 2, "interrupted": 0, "resumed": 0, "excluded": []}
    result = {"schema": "ml-dataset.prime61-root-enum-independent-audit/v1",
        "scope": "four fixed local prime n61 m2 cells; archive and native manifest replay, not a full matrix result",
        "protocol_file_sha256": plan["protocol_sha256"],
        "plan_file_sha256": digest((HERE / "plan.json").read_bytes()),
        "export_sha256": receipt["export_sha256"],
        "planned": 4, "retained": 4, "native_status_counts": dict(sorted(statuses.items())),
        "independent_ecdlp_solves": 0, "prime_modulus": P,
        "twist_counts": {"untwisted": 2, "twisted": 2}, "rows": rows}
    target = HERE / "AUDIT.json"
    encoded = (json.dumps(result, sort_keys=True, indent=2) + "\n").encode()
    if target.exists() and target.read_bytes() != encoded:
        raise ValueError("AUDIT.json differs; review source changes before replacing")
    target.write_bytes(encoded)
    print(json.dumps({"retained": 4, "native_status_counts": result["native_status_counts"],
                      "prime_modulus": P}, sort_keys=True))


if __name__ == "__main__":
    main()
