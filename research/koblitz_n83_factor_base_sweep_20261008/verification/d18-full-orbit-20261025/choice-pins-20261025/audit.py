"""Replay the two full-D18 choice-isolation controls with independent GF arithmetic.

Usage: PYTHONPATH=<blake3==1.0.8 directory> python3 audit.py OBJECT BINARY
"""

import base64
import gzip
import hashlib
import importlib.util
import json
import pathlib
import re
import sys

import blake3


ROOT = pathlib.Path(__file__).resolve().parent
PARENT = ROOT.parent
SOURCE = "d775b18dc427188e0279f4189c3c63b570f780fc"
OBJECT_SHA = "eaef94ef45b3d2d4cd2ec2f40c1173b4cdb2a437b7e618a858c73fb1caf3902b"
sys.dont_write_bytecode = True
spec = importlib.util.spec_from_file_location("n83_rootpin_verifier", PARENT / "root-pinned-20261025/verify.py")
group = importlib.util.module_from_spec(spec)
spec.loader.exec_module(group)


def sha256(path):
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def read_json(path):
    return json.loads(path.read_text())


def max_rss(path):
    match = re.search(r"(\d+)\s+maximum resident set size", path.read_text())
    assert match
    return int(match.group(1))


def audit(object_path, binary_path):
    wrapper = read_json(ROOT / "s3-manifest.json")
    get = read_json(ROOT / "object-get-metadata.json")
    manifest_get = read_json(ROOT / "manifest-get-metadata.json")
    host = read_json(ROOT / "host.json")
    assert sha256(object_path) == OBJECT_SHA == wrapper["compressed_sha256"]
    assert object_path.stat().st_size == get["ContentLength"] == 160_747_529
    assert get["ChecksumSHA256"] == base64.b64encode(bytes.fromhex(OBJECT_SHA)).decode()
    assert wrapper["producer_manifest_sha256"] == sha256(PARENT / "manifest.json")
    assert manifest_get["ChecksumSHA256"] == base64.b64encode(bytes.fromhex(sha256(ROOT / "s3-manifest.json"))).decode()
    binary_sha = sha256(binary_path)
    assert host["source_commit"] == SOURCE and host["binary_sha256"] == binary_sha
    assert host["isolation"] == "L0_shared_host_no_hard_cgroup"
    assert host["run_order"] == ["phase", "selector"]
    assert host["internal_process_wall_cap_seconds"] == 300
    assert host["independent_outer_cap_seconds"] == 310
    assert host["selected_best_total_runtime"] is None

    indices = [0, 166, 332, 498]
    rows = {}
    with gzip.open(object_path, "rt") as stream:
        header = json.loads(next(stream))
        for index, line in enumerate(stream):
            if index in indices:
                rows[index] = json.loads(line)
            if index >= indices[-1]:
                break
    assert header["subgroup_order"] == str(group.ORDER)
    assert header["curve_a"] == 0 and header["curve_b"] == "1"
    assert header["generator"] == ["5402229379304697578719232", "3600185575789834070382019"]
    generator = tuple(map(int, header["generator"]))
    assert group.on_curve(generator) and group.scalar_mul(generator, group.ORDER) is None
    total = None
    for column, index in enumerate(indices):
        row = rows[index]
        assert row["column"] == column and row["phase"] == 0
        assert row["negative"] is False and row["coefficient"] == "1"
        point = tuple(map(int, row["point"]))
        assert group.on_curve(point) and group.scalar_mul(point, group.ORDER) is None
        total = group.add(total, point)
    coefficients = ["0"] * 32_643
    coefficients[:4] = ["1"] * 4
    row_digest = blake3.blake3(json.dumps(coefficients, separators=(",", ":")).encode()).hexdigest()

    outcomes = {}
    for name, clauses in (("phase", 2_807_780), ("selector", 2_807_756)):
        directory = ROOT / name
        result = read_json(directory / "result.json")
        shape = read_json(directory / "result.json.shape.json")
        phases = [json.loads(line) for line in (directory / "result.json.phases.jsonl").read_text().splitlines()]
        expected_policy = "phase_zero" if name == "phase" else "selector"
        assert (directory / "exit-code.txt").read_text().strip() == "0"
        assert (directory / "binary-sha256.txt").read_text().split()[0] == binary_sha
        assert result["source_commit"] == SOURCE and result["binary_sha256"] == binary_sha
        assert result["source_attestation_mode"] == "clean_checkout"
        assert result["status"] == "PASS_CONSTRUCTED_CONTROL"
        assert result["root_encoding"] == "shared_root_bits"
        assert result["choice_pin_policy"] == expected_policy and result["root_choice_pinned"] is False
        assert result["mode"] == "positive" and result["solver_answer"] == "Sat"
        assert result["shape"] == shape and shape["clauses"] == clauses
        assert shape["variables"] == 329_726 and shape["xor_rows"] == 3_901
        assert [phase["phase"] for phase in phases] == ["started", "import_checked", "model_constructed", "solver_returned"]
        assert phases[0]["detail"]["source_commit"] == SOURCE
        assert phases[0]["detail"]["choice_pin_policy"] == expected_policy
        assert phases[2]["detail"] == shape and phases[3]["detail"]["answer"] == "Sat"
        assert result["relation"]["indices"] == indices
        assert result["relation"]["row_decimal_blake3"] == row_digest
        assert result["relation"]["rank_after_insert"] == 1
        assert result["relation"]["tamper_rejected"] is True
        assert result["relation"]["matches_planted_row"] is True
        target = tuple(map(int, result["target"]))
        assert target == total and group.on_curve(target)
        assert group.add(target, generator) != total
        assert result["selected_best_total_runtime"] is None
        assert result["process_wall_ms"] < 300_000 and result["search_ms"] < 300_000
        assert (directory / "start-utc.txt").read_text().strip() == host[f"{name}_start_utc"]
        assert (directory / "end-utc.txt").read_text().strip() == host[f"{name}_end_utc"]
        outcomes[name] = {
            "status": result["status"],
            "policy": expected_policy,
            "total_clauses": clauses,
            "search_ms": result["search_ms"],
            "solver_conflicts": result["solver_conflicts"],
            "process_wall_ms": result["process_wall_ms"],
            "supervisor_max_rss_bytes": max_rss(directory / "stderr-time.txt"),
        }
    return {
        "schema": "n83.full-d18-choice-pin-audit/v1",
        "status": "PASS_TWO_INDEPENDENT_GROUP_AND_ROW_REPLAYS",
        "source_commit": SOURCE,
        "binary_sha256": binary_sha,
        "object_sha256": OBJECT_SHA,
        "point_indices": indices,
        "target": list(map(str, total)),
        "row_decimal_blake3": row_digest,
        "arms": outcomes,
        "natural_search_executed": False,
        "selected_best_total_runtime": None,
    }


if __name__ == "__main__":
    if len(sys.argv) != 3:
        raise SystemExit("usage: audit.py COMPRESSED_S3_OBJECT SOURCE_BINARY")
    print(json.dumps(audit(pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2])), indent=2, sort_keys=True))
