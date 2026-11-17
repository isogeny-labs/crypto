"""Replay the retained full-D18 L0 positive-control cap receipt.

Pass the freshly fetched compressed S3 object as an optional argument to
rehash the 160 MB payload. Without it, this audits the committed raw records.
"""

import base64
import hashlib
import json
import pathlib
import sys


ROOT = pathlib.Path(__file__).resolve().parent
SOURCE = "dc5600493668544691a5884f5dcd474192a4d921"
OBJECT_SHA = "eaef94ef45b3d2d4cd2ec2f40c1173b4cdb2a437b7e618a858c73fb1caf3902b"
OBJECT_BYTES = 160_747_529
PRODUCER_SHA = "1d3650eabc4662d8f2597498d7ac92b7cd0a4134ac5f8155aaa49c7af3407dce"


def read_json(name):
    return json.loads((ROOT / name).read_text())


def sha256(path):
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def audit(object_path=None):
    wrapper = read_json("s3-manifest.json")
    object_get = read_json("object-get-metadata.json")
    manifest_get = read_json("manifest-get-metadata.json")
    cap = read_json("result.json.cap.json")
    shape = read_json("result.json.shape.json")
    phases = [json.loads(line) for line in (ROOT / "result.json.phases.jsonl").read_text().splitlines()]
    binary_sha = (ROOT / "binary-sha256.txt").read_text().split()[0]
    assert wrapper["producer_manifest_sha256"] == PRODUCER_SHA
    assert wrapper["compressed_sha256"] == OBJECT_SHA
    assert sha256(ROOT / "s3-manifest.json") == "1b93351febd47dac4de8c5dd39dbf58564d0057a044054baf448d978f6c0d616"
    assert manifest_get["ChecksumSHA256"] == base64.b64encode(bytes.fromhex(sha256(ROOT / "s3-manifest.json"))).decode()
    assert object_get["ContentLength"] == OBJECT_BYTES
    assert object_get["Metadata"]["sha256"] == OBJECT_SHA
    assert object_get["ChecksumSHA256"] == base64.b64encode(bytes.fromhex(OBJECT_SHA)).decode()
    assert object_get["ServerSideEncryption"] == "AES256"
    if object_path is not None:
        assert object_path.stat().st_size == OBJECT_BYTES
        assert sha256(object_path) == OBJECT_SHA
    assert [phase["phase"] for phase in phases] == ["started", "import_checked", "model_constructed"]
    assert phases[0]["detail"]["source_commit"] == SOURCE
    assert phases[0]["detail"]["binary_sha256"] == binary_sha
    assert phases[1]["detail"]["point_set_blake3"] == wrapper["point_set_blake3"]
    assert phases[2]["detail"]["variables"] == shape["variables"] == 329_726
    assert phases[2]["detail"]["clauses"] == shape["clauses"] == 11_425_240
    assert phases[2]["detail"]["xor_rows"] == shape["xor_rows"] == 3_901
    assert cap["status"] == "UNKNOWN_wall_cap"
    assert cap["budget_seconds"] == 300
    assert 300_000 <= cap["elapsed_process_wall_ms"] < 301_000
    assert cap["isolation"] == "L0_shared_host_no_hard_cgroup"
    assert cap["selected_best_total_runtime"] is None
    assert not (ROOT / "result.json").exists()
    assert (ROOT / "stdout.json").stat().st_size == 0
    assert (ROOT / "exit-code.txt").read_text().strip() == "1"
    stderr = (ROOT / "stderr-time.txt").read_text()
    assert "real" in stderr and "sysctl kern.clockrate: Operation not permitted" in stderr
    return {
        "schema": "n83.full-d18-m4-l0-positive-audit/v1",
        "status": "PASS_CENSORED_CAP_RECEIPT",
        "source_commit": SOURCE,
        "binary_sha256": binary_sha,
        "object_sha256": OBJECT_SHA,
        "object_rehashed": object_path is not None,
        "phase_sequence": [phase["phase"] for phase in phases],
        "sat_variables": shape["variables"],
        "sat_clauses": shape["clauses"],
        "sat_xor_rows": shape["xor_rows"],
        "worker_status": cap["status"],
        "solver_answer": None,
        "relation_checked": False,
        "selected_best_total_runtime": None,
        "timing_limit": "time -l could not read kern.clockrate in the sandbox; real/user/sys are retained, peak RSS unavailable",
    }


if __name__ == "__main__":
    if len(sys.argv) > 2:
        raise SystemExit("usage: audit.py [COMPRESSED_S3_OBJECT]")
    path = pathlib.Path(sys.argv[1]) if len(sys.argv) == 2 else None
    print(json.dumps(audit(path), indent=2, sort_keys=True))
