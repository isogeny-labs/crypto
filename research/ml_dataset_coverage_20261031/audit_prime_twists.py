#!/usr/bin/env python3
"""Replay prime n23 twist identities from exact restricted custody archives."""

import collections
import hashlib
import json
import tarfile
from pathlib import Path


HERE = Path(__file__).resolve().parent
CAMPAIGN = HERE.parent / "ml_dataset_local_campaign_20261031"
DEFAULT_ARCHIVES = Path("/var/tmp/ml-dataset-local-campaign-20261031/archives")


def digest(data):
    return hashlib.sha256(data).hexdigest()


def read(path):
    return json.loads(path.read_text())


def main():
    import argparse
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--archives", type=Path, default=DEFAULT_ARCHIVES)
    args = parser.parse_args()
    plan = read(CAMPAIGN / "plan.json")
    assert plan["families"] == ["prime"] and plan["field_sizes"] == [23]
    assert plan["arities"] == [2] and plan["solvers"] == ["bsgs"]
    rows = []
    for index in range(64):
        checkpoint = read(CAMPAIGN / "checkpoints" / f"shard-{index:02d}.json")
        assert checkpoint["index"] == checkpoint["custody"]["index"] == index
        custody = checkpoint["custody"]
        archive = args.archives / f"shard-{index:02d}.tar.gz"
        data = archive.read_bytes()
        assert digest(data) == custody["archive_sha256"] and len(data) == custody["archive_bytes"]
        name = f"shard-{index:02d}/runner/attempt/cells/{index:012d}/manifest.json"
        with tarfile.open(archive, "r:gz") as tar:
            matches = [member for member in tar.getmembers() if member.name == name]
            assert len(matches) == 1 and matches[0].isfile(), f"shard {index}: manifest absent or ambiguous"
            stream = tar.extractfile(matches[0])
            assert stream is not None
            raw = stream.read()
        manifest = json.loads(raw)
        assert manifest["cell_index"] == index
        cfg = manifest["config"]
        assert cfg["family"] == "prime" and cfg["n"] == 23 and cfg["m"] == 2
        assert cfg["solver"] == "bsgs" and type(cfg["twist"]) is bool
        assert checkpoint["published"]["archive_fresh_get"] is True
        rows.append({"index": index, "twist": cfg["twist"],
            "generator_index": cfg["generator_index"], "key_index": cfg["key_index"],
            "archive_sha256": custody["archive_sha256"], "manifest_sha256": digest(raw),
            "s3_uri": custody["s3_uri"]})
    counts = collections.Counter(row["twist"] for row in rows)
    assert counts == {False: 32, True: 32}
    output = {"schema": "ml-dataset.prime23-twist-archive-audit/v1",
        "source": "64 local restricted archives, each byte-checked against its Git custody checkpoint",
        "campaign_plan_sha256": digest((CAMPAIGN / "plan.json").read_bytes()),
        "total": len(rows), "untwisted": counts[False], "twisted": counts[True], "rows": rows}
    target = HERE / "prime-twist-archive-audit.json"
    encoded = (json.dumps(output, sort_keys=True, indent=2) + "\n").encode()
    if target.exists() and target.read_bytes() != encoded:
        raise ValueError("existing prime twist audit differs")
    target.write_bytes(encoded)
    print(json.dumps({"total": len(rows), "untwisted": counts[False], "twisted": counts[True]}))


if __name__ == "__main__":
    main()
