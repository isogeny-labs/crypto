#!/usr/bin/env python3
"""Explicit, bounded S3 software test. Its synthetic fixture is NOT research data."""
import argparse
import base64
from datetime import datetime, timezone
import json
from pathlib import Path
import subprocess
import tempfile

import consumer as c
from test_consumer import ConsumerTests


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--execute", action="store_true", help="authorize this software fixture's S3 writes")
    parser.add_argument("--evidence", type=Path, required=True, help="new local evidence directory")
    args = parser.parse_args()
    if not args.execute:
        parser.error("this test writes a small fixture to S3; pass --execute to run")
    args.evidence.mkdir(parents=True, exist_ok=False)
    test = ConsumerTests()
    test.setUp()
    try:
        # The unsupported outcome is intentional; this is not a solved ECDLP instance.
        test.add_public()
        manifest_bytes = (test.cell / "manifest.json").read_bytes()
        artifacts = list(test.members.values()) + [{"path": "manifest.json", "role": "cell-manifest",
            "encoding": "json", "bytes": len(manifest_bytes), "sha256": c.digest(manifest_bytes)}]
        prefix = "research/ml-dataset-validation/20261028/software-fixture-" + c.digest(manifest_bytes)
        record = {"schema": "ml-dataset.publication/v1", "remote": "s3://setec-astronomy-artifacts/" + prefix,
            "namespace": "software-format-fixture-NOT-RESEARCH-DATA", "manifest_sha256": c.digest(manifest_bytes),
            "objects": [{"artifact": a, "object_key": f"sha256/{a['sha256'][:2]}/{a['sha256']}"} for a in artifacts]}
        total = sum(a["bytes"] for a in artifacts)
        c.require(total < 100000 and len(artifacts) < 32, "software fixture exceeded fixed size limit")
        uploaded = set()
        def put(key, path):
            sha = c.digest(path.read_bytes())
            argv = ["aws", "--region", "us-east-2", "s3api", "put-object", "--bucket", "setec-astronomy-artifacts",
                    "--key", key, "--body", str(path), "--if-none-match", "*", "--checksum-algorithm", "SHA256",
                    "--checksum-sha256", base64.b64encode(bytes.fromhex(sha)).decode()]
            result = subprocess.run(argv, capture_output=True, text=True, timeout=60)
            c.require(result.returncode == 0 or "PreconditionFailed" in result.stderr, result.stderr)
        for obj in record["objects"]:
            if obj["object_key"] not in uploaded:
                put(prefix + "/" + obj["object_key"], test.cell / obj["artifact"]["path"])
                uploaded.add(obj["object_key"])
        receipt = args.evidence / "completion.json"
        receipt.write_bytes(c.compact(record))
        with tempfile.TemporaryDirectory(prefix="ml-consumer-s3-restore-") as restored:
            report = c.fetch_cell(receipt, c.digest(receipt.read_bytes()), Path(restored) / "run")
        # Publish the completion record only after every referenced object was GET/hash checked.
        receipt_key = prefix + "/completion.json"
        put(receipt_key, receipt)
        with tempfile.TemporaryDirectory(prefix="ml-consumer-receipt-readback-") as directory:
            fetched = Path(directory) / "completion.json"
            result = subprocess.run(["aws", "--region", "us-east-2", "s3api", "get-object", "--bucket",
                "setec-astronomy-artifacts", "--key", receipt_key, str(fetched)], capture_output=True, text=True, timeout=60)
            c.require(result.returncode == 0 and fetched.read_bytes() == receipt.read_bytes(), "receipt readback failed")
        evidence = {"schema": "ml-dataset.consumer-software-integration/v1", "utc": datetime.now(timezone.utc).isoformat(),
            "purpose": "S3 software fixture only; no research data or mathematical verification",
            "consumer_sha256": c.digest(Path(c.__file__).read_bytes()), "integration_script_sha256": c.digest(Path(__file__).read_bytes()),
            "receipt_uri": "s3://setec-astronomy-artifacts/" + receipt_key,
            "receipt_sha256": c.digest(receipt.read_bytes()), "receipt_readback_verified": True,
            "artifact_references": len(artifacts), "unique_objects": len(uploaded), "declared_artifact_bytes": total,
            "audit": report}
        (args.evidence / "s3-integration.json").write_text(json.dumps(evidence, indent=2) + "\n")
        print(json.dumps(evidence, indent=2))
    finally:
        test.doCleanups()


if __name__ == "__main__":
    main()
