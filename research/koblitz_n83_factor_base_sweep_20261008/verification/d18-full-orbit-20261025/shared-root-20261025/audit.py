"""Independently replay the retained full-D18 shared-root model and cap receipts."""

import base64
import csv
import datetime as dt
import hashlib
import json
import pathlib
import re
import sys


ROOT = pathlib.Path(__file__).resolve().parent
PARENT = ROOT.parent
SOURCE = "1f04629635101e04ac662bef28ce589b92bc929b"
OBJECT_SHA = "eaef94ef45b3d2d4cd2ec2f40c1173b4cdb2a437b7e618a858c73fb1caf3902b"
OBJECT_BYTES = 160_747_529


def read_json(path):
    return json.loads(path.read_text())


def sha256(path):
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def peak_rss(path):
    match = re.search(r"(\d+)\s+maximum resident set size", path.read_text())
    assert match
    return int(match.group(1))


def timestamp(path):
    return dt.datetime.fromisoformat(path.read_text().strip().replace("Z", "+00:00"))


def audit(object_path=None):
    wrapper = read_json(ROOT / "s3-manifest.json")
    object_get = read_json(ROOT / "object-get-metadata.json")
    manifest_get = read_json(ROOT / "manifest-get-metadata.json")
    assert sha256(ROOT / "s3-manifest.json") == "1b93351febd47dac4de8c5dd39dbf58564d0057a044054baf448d978f6c0d616"
    assert wrapper["producer_manifest_sha256"] == sha256(PARENT / "manifest.json")
    assert wrapper["compressed_sha256"] == OBJECT_SHA
    producer = read_json(PARENT / "manifest.json")
    assert sha256(PARENT / "orbit-classes.csv") == producer["selected_csv_sha256"]
    with (PARENT / "orbit-classes.csv").open() as stream:
        representatives = [int(row["orbit_key_decimal"]) for row in csv.DictReader(stream)]
    assert len(representatives) == producer["columns"] == 32_643
    mask = (1 << 83) - 1
    common_ones = mask
    common_zeros = mask
    for representative in representatives:
        common_ones &= representative
        common_zeros &= (~representative) & mask
    common_bits = (common_ones | common_zeros).bit_count()
    assert common_ones == 0 and common_bits == 66
    assert 4 * common_bits * (len(representatives) - 1) == 8_617_488
    assert object_get["ContentLength"] == OBJECT_BYTES
    assert object_get["Metadata"]["sha256"] == OBJECT_SHA
    assert object_get["ChecksumSHA256"] == base64.b64encode(bytes.fromhex(OBJECT_SHA)).decode()
    assert manifest_get["ChecksumSHA256"] == base64.b64encode(bytes.fromhex(sha256(ROOT / "s3-manifest.json"))).decode()
    if object_path is not None:
        assert object_path.stat().st_size == OBJECT_BYTES
        assert sha256(object_path) == OBJECT_SHA

    model = read_json(ROOT / "model/result.json")
    preflight = read_json(ROOT / "model/result.json.preflight.json")
    model_binary = (ROOT / "model/binary-sha256.txt").read_text().split()[0]
    assert (ROOT / "model/exit-code.txt").read_text().strip() == "0"
    assert model["status"] == "PASS_MODEL_CONSTRUCTION_ONLY"
    assert model["preflight"] == preflight
    assert preflight["source_commit"] == SOURCE
    assert preflight["binary_sha256"] == model_binary
    assert preflight["root_encoding"] == "shared_root_bits"
    assert preflight["required_max_variables"] == model["sat_variables"] == 329_726
    assert preflight["required_domain_clauses"] == 2_621_168
    assert model["sat_clauses"] == 2_807_254
    assert model["sat_xor_rows"] == 3_901
    assert model["solver_search_executed"] is False
    assert model["selected_best_total_runtime"] is None

    phases = [json.loads(line) for line in (ROOT / "positive/result.json.phases.jsonl").read_text().splitlines()]
    shape = read_json(ROOT / "positive/result.json.shape.json")
    cap = read_json(ROOT / "positive/result.json.cap.json")
    positive_binary = (ROOT / "positive/binary-sha256.txt").read_text().split()[0]
    assert positive_binary == model_binary
    assert [p["phase"] for p in phases] == ["started", "import_checked", "model_constructed"]
    assert phases[0]["detail"]["source_commit"] == SOURCE
    assert phases[0]["detail"]["binary_sha256"] == model_binary
    assert phases[0]["detail"]["root_encoding"] == "shared_root_bits"
    assert phases[1]["detail"]["point_set_blake3"] == wrapper["point_set_blake3"]
    assert phases[2]["detail"] == shape
    assert shape["variables"] == 329_726 and shape["clauses"] == 2_807_752
    assert shape["xor_rows"] == 3_901 and shape["root_encoding"] == "shared_root_bits"
    assert cap["status"] == "UNKNOWN_wall_cap" and cap["budget_seconds"] == 300
    assert cap["root_encoding"] == "shared_root_bits"
    assert 300_000 <= cap["elapsed_process_wall_ms"] < 301_000
    assert cap["selected_best_total_runtime"] is None
    assert not (ROOT / "positive/result.json").exists()
    assert (ROOT / "positive/stdout.json").stat().st_size == 0
    assert (ROOT / "positive/exit-code.txt").read_text().strip() == "124"
    assert timestamp(ROOT / "model/start-utc.txt") < timestamp(ROOT / "model/end-utc.txt")
    assert timestamp(ROOT / "model/end-utc.txt") < timestamp(ROOT / "positive/start-utc.txt")
    assert timestamp(ROOT / "positive/start-utc.txt") < timestamp(ROOT / "positive/end-utc.txt")

    baseline_model = read_json(PARENT / "model-only-20261025.json")
    baseline_control = read_json(PARENT / "l0-positive-20261025/result.json.shape.json")
    assert baseline_model["sat_clauses"] - model["sat_clauses"] == 8_617_488
    assert baseline_control["clauses"] - shape["clauses"] == 8_617_488
    return {
        "schema": "n83.full-d18-shared-root-audit/v1",
        "status": "PASS_MODEL_AND_CENSORED_CONTROL_RECEIPTS",
        "source_commit": SOURCE,
        "binary_sha256": model_binary,
        "object_sha256": OBJECT_SHA,
        "object_rehashed": object_path is not None,
        "common_root_bits": common_bits,
        "clauses_removed_from_each_m4_model": 8_617_488,
        "model_total_clauses": model["sat_clauses"],
        "model_construction_ms": model["model_ms"],
        "model_process_wall_ms": model["process_wall_ms"],
        "model_supervisor_max_rss_bytes": peak_rss(ROOT / "model/stderr-time.txt"),
        "positive_total_clauses": shape["clauses"],
        "positive_supervisor_max_rss_bytes": peak_rss(ROOT / "positive/stderr-time.txt"),
        "positive_worker_status": cap["status"],
        "positive_solver_answer": None,
        "relation_checked": False,
        "selected_best_total_runtime": None,
    }


if __name__ == "__main__":
    if len(sys.argv) > 2:
        raise SystemExit("usage: audit.py [COMPRESSED_S3_OBJECT]")
    object_path = pathlib.Path(sys.argv[1]) if len(sys.argv) == 2 else None
    print(json.dumps(audit(object_path), indent=2, sort_keys=True))
