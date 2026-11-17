"""Independent custody, GF(2^83) and retained-receipt replay.

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
SOURCE = "57c20963924ab0097111b08fc6cb157400e87e53"
BINARY_SHA = "064699dfd6fcf19592f7f0de7b82fb816426bb3be27bc2766757296d9a58222d"
OBJECT_SHA = "eaef94ef45b3d2d4cd2ec2f40c1173b4cdb2a437b7e618a858c73fb1caf3902b"
sys.dont_write_bytecode = True
spec = importlib.util.spec_from_file_location("n83_phase_first_group", PARENT / "root-pinned-20261025/verify.py")
group = importlib.util.module_from_spec(spec)
spec.loader.exec_module(group)


def read_json(path):
    return json.loads(path.read_text())


def sha256(path):
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def replay(object_path, binary_path):
    wrapper = read_json(ROOT / "s3-manifest.json")
    object_get = read_json(ROOT / "object-get-metadata.json")
    manifest_get = read_json(ROOT / "manifest-get-metadata.json")
    host = read_json(ROOT / "host.json")
    assert sha256(object_path) == OBJECT_SHA == wrapper["compressed_sha256"]
    assert object_path.stat().st_size == object_get["ContentLength"] == 160_747_529
    assert object_get["ChecksumSHA256"] == base64.b64encode(bytes.fromhex(OBJECT_SHA)).decode()
    assert sha256(ROOT / "s3-manifest.json") == "1b93351febd47dac4de8c5dd39dbf58564d0057a044054baf448d978f6c0d616"
    assert manifest_get["ChecksumSHA256"] == base64.b64encode(bytes.fromhex(sha256(ROOT / "s3-manifest.json"))).decode()
    assert wrapper["producer_manifest_sha256"] == sha256(PARENT / "manifest.json")
    assert sha256(binary_path) == BINARY_SHA
    assert host["source_commit"] == SOURCE and host["binary_sha256"] == BINARY_SHA
    assert host["isolation"] == "L0_shared_host_no_hard_cgroup"
    assert host["internal_process_wall_cap_seconds"] == 300
    assert host["independent_outer_cap_seconds"] == 310
    assert host["selected_best_total_runtime"] is None
    prior = PARENT / "shared-root-20261025/positive"
    prior_shape = read_json(prior / "result.json.shape.json")
    prior_cap = read_json(prior / "result.json.cap.json")
    assert prior_cap["status"] == "UNKNOWN_wall_cap" and prior_cap["budget_seconds"] == 300
    assert prior_shape["root_encoding"] == "shared_root_bits"
    assert prior_shape["clauses"] == 2_807_752
    assert prior_shape["variables"] == 329_726 and prior_shape["xor_rows"] == 3_901
    assert wrapper["object_key"].endswith(prior_shape["object"])

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

    arms = {}
    for name in ("first", "repeat"):
        directory = ROOT / name
        result = read_json(directory / "result.json")
        shape = read_json(directory / "result.json.shape.json")
        phases = [json.loads(line) for line in (directory / "result.json.phases.jsonl").read_text().splitlines()]
        command = read_json(directory / "command.json")
        assert command[3] == "primary-full-d18-m4-l0-shared-phase-first"
        assert (directory / "binary-sha256.txt").read_text().split()[0] == BINARY_SHA
        assert (directory / "outer-cap.txt").read_text().strip() == "not_reached"
        assert result["source_commit"] == SOURCE and result["binary_sha256"] == BINARY_SHA
        assert result["source_attestation_mode"] == "clean_checkout"
        assert result["status"] == "PASS_CONSTRUCTED_CONTROL" and result["solver_answer"] == "Sat"
        assert result["mode"] == "positive" and result["root_encoding"] == "shared_root_bits"
        assert result["choice_pin_policy"] == "none" and result["root_choice_pinned"] is False
        assert result["search_policy"] == "phase_zero_first"
        assert result["shape"] == shape and shape["clauses"] == 2_807_752
        assert shape["variables"] == 329_726 and shape["xor_rows"] == 3_901
        assert shape["choice_pin_policy"] == "none" and shape["search_policy"] == "phase_zero_first"
        assert [phase["phase"] for phase in phases] == ["started", "import_checked", "model_constructed", "solver_returned"]
        assert phases[0]["detail"]["source_commit"] == SOURCE
        assert phases[0]["detail"]["search_policy"] == "phase_zero_first"
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
        assert result["solver_conflicts"] == 0
        assert result["process_wall_ms"] < 300_000
        assert read_json(directory / "stdout.json") == result
        if name == "first":
            assert (directory / "exit-code.txt").read_text().strip() == "1"
            assert "time: sysctl kern.clockrate: Operation not permitted" in (directory / "stderr-time.txt").read_text()
            max_rss = None
        else:
            assert (directory / "exit-code.txt").read_text().strip() == "0"
            match = re.search(r"(\d+)\s+maximum resident set size", (directory / "stderr-time.txt").read_text())
            assert match
            max_rss = int(match.group(1))
            assert max_rss == 3_861_577_728
        arms[name] = {
            "status": result["status"],
            "supervisor_exit_code": int((directory / "exit-code.txt").read_text()),
            "search_ms": result["search_ms"],
            "conflicts": result["solver_conflicts"],
            "import_ms": shape["import_ms"],
            "process_wall_ms": result["process_wall_ms"],
            "supervisor_max_rss_bytes": max_rss,
        }
    return {
        "schema": "n83.full-d18-phase-first-audit/v1",
        "status": "PASS_TWO_WORKER_GROUP_REPLAYS_ONE_SUPERVISOR_VALID",
        "source_commit": SOURCE,
        "binary_sha256": BINARY_SHA,
        "object_sha256": OBJECT_SHA,
        "target": list(map(str, total)),
        "point_indices": indices,
        "row_decimal_blake3": row_digest,
        "arms": arms,
        "prior_shared_root_planted_status": prior_cap["status"],
        "prior_shared_root_planted_wall_cap_seconds": prior_cap["budget_seconds"],
        "natural_search_executed": False,
        "selected_best_total_runtime": None,
    }


if __name__ == "__main__":
    if len(sys.argv) != 3:
        raise SystemExit("usage: audit.py COMPRESSED_S3_OBJECT SOURCE_BINARY")
    print(json.dumps(replay(pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2])), indent=2, sort_keys=True))
