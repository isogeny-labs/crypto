"""Independent GF(2^83), S3 custody and worker-receipt replay for the Linux gate."""

import base64
import copy
import gzip
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import sys

import blake3

HERE = Path(__file__).resolve().parent
D18 = HERE.parent
OBJECT_SHA = "eaef94ef45b3d2d4cd2ec2f40c1173b4cdb2a437b7e618a858c73fb1caf3902b"
MANIFEST_SHA = "1b93351febd47dac4de8c5dd39dbf58564d0057a044054baf448d978f6c0d616"
OBJECT_REL = "objects/c295da8898cfad29a053621c768ca059eea4263f89f5ca78b4f98af43c383f3f.jsonl.gz"
MEMORY_BYTES = 8 * 1024 ** 3
ORDER = 2417851639230796216685689

sys.dont_write_bytecode = True
spec = importlib.util.spec_from_file_location("n83_d18_linux_group", D18 / "root-pinned-20261025/verify.py")
group = importlib.util.module_from_spec(spec)
spec.loader.exec_module(group)


def read(path):
    return json.loads(Path(path).read_text())


def sha256(path):
    digest = hashlib.sha256()
    with Path(path).open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def verify_input(source, base, binary, expected_commit):
    source, base, binary = map(Path, (source, base, binary))
    object_path = base / OBJECT_REL
    assert object_path.stat().st_size == 160747529
    assert sha256(object_path) == OBJECT_SHA
    assert sha256(base / "s3-manifest.json") == MANIFEST_SHA
    producer = source / "research/koblitz_n83_factor_base_sweep_20261008/verification/d18-full-orbit-20261025/manifest.json"
    assert (base / "manifest.json").read_bytes() == producer.read_bytes()
    wrapper = read(base / "s3-manifest.json")
    assert wrapper["compressed_sha256"] == OBJECT_SHA
    assert wrapper["producer_manifest_sha256"] == sha256(producer)
    assert wrapper["columns"] == 32643 and wrapper["points"] == 5418738
    assert wrapper["curve_id"] == "icv1-f2m83-tm6151469093347-debefd74"
    assert wrapper["selected_best_total_runtime"] is None
    manifest_get = read(base / "manifest-get-metadata.json")
    object_get = read(base / "object-get-metadata.json")
    assert object_get["ContentLength"] == 160747529 and object_get["ServerSideEncryption"] == "AES256"
    assert object_get["ChecksumSHA256"] == base64.b64encode(bytes.fromhex(OBJECT_SHA)).decode()
    assert manifest_get["ChecksumSHA256"] == base64.b64encode(bytes.fromhex(MANIFEST_SHA)).decode()
    assert manifest_get["ServerSideEncryption"] == "AES256"
    assert sha256(source / "research/koblitz_n83_factor_base_sweep_20261008/pilot-01/probe-corpus.json") == "760ae5c2e88615152e2569653a0e0c0cf3b2f84691f2b0731247f9ee3759dcbd"
    return {"source_commit": expected_commit, "binary_sha256": sha256(binary), "object_sha256": OBJECT_SHA,
            "manifest_sha256": MANIFEST_SHA, "object_bytes": object_path.stat().st_size}


def replay_relation(result, object_path):
    indices = result["relation"]["indices"]
    assert len(indices) == 4 and all(isinstance(i, int) and i >= 0 for i in indices)
    wanted = set(indices)
    rows = {}
    with gzip.open(object_path, "rt") as stream:
        header = json.loads(next(stream))
        assert header["curve_a"] == 0 and header["subgroup_order"] == str(ORDER)
        for index, line in enumerate(stream):
            if index in wanted:
                rows[index] = json.loads(line)
            if len(rows) == len(wanted):
                break
    assert len(rows) == len(wanted)
    generator = tuple(map(int, header["generator"]))
    target = tuple(map(int, result["target"]))
    assert group.on_curve(generator) and group.scalar_mul(generator, ORDER) is None
    assert group.on_curve(target) and group.scalar_mul(target, ORDER) is None
    total = None
    coefficients = [0] * 32643
    for index in indices:
        row = rows[index]
        point = tuple(map(int, row["point"]))
        assert group.on_curve(point) and group.scalar_mul(point, ORDER) is None
        total = group.add(total, point)
        column = int(row["column"])
        assert 0 <= column < 32643
        coefficients[column] = (coefficients[column] + int(row["coefficient"])) % ORDER
    assert total == target and group.add(target, generator) != total
    row_digest = blake3.blake3(json.dumps([str(x) for x in coefficients], separators=(",", ":")).encode()).hexdigest()
    assert row_digest == result["relation"]["row_decimal_blake3"]
    assert result["relation"]["rank_after_insert"] == 1
    assert result["relation"]["tamper_rejected"] is True
    return {"indices": indices, "target": list(map(str, target)), "row_decimal_blake3": row_digest,
            "rank_after_insert": 1, "group_replay": "PASS"}


def _common_receipt(directory, mode, source_commit, binary_sha):
    directory = Path(directory)
    supervisor = read(directory / "supervisor.json")
    assert supervisor["source_commit"] == source_commit and supervisor["binary_sha256"] == binary_sha
    assert supervisor["mode"] == mode and supervisor["outer_cap_seconds"] == 1210
    assert supervisor["memory_limit_bytes"] == MEMORY_BYTES and supervisor["cpu_limit"] == 1
    assert supervisor["network"] == "none" and supervisor["worker_root_read_only"] is True
    command = read(directory / "command.json")
    assert command[0:2] == ["docker", "run"]
    assert command[command.index("--network") + 1] == "none"
    assert command[command.index("--cpus") + 1] == "1"
    assert command[command.index("--memory") + 1] == "8192m"
    assert command[command.index("--memory-swap") + 1] == "8192m"
    assert "--read-only" in command and "--cap-drop" in command
    assert command[-6:-4] == ["/worker", "primary-full-d18-m4-linux-shared-phase-first"]
    assert command[-4] == mode and command[-1] == "/out/result.json"
    assert supervisor["image_ref"] == command[-7]
    assert (directory / "exit-code.txt").read_text().strip() == str(supervisor["exit_code"])
    assert (directory / "outer-cap.txt").read_text().strip() == ("reached" if supervisor["outer_cap_reached"] else "not_reached")
    assert supervisor["started_utc"] < supervisor["ended_utc"]
    return supervisor


def _shape_and_phases(directory, mode, source_commit, binary_sha):
    shape = read(directory / "result.json.shape.json")
    assert shape["variables"] == 329726 and shape["xor_rows"] == 3901
    assert shape["clauses"] == (2807752 if mode == "positive" else 2807254)
    assert shape["root_encoding"] == "shared_root_bits"
    assert shape["search_policy"] == "phase_zero_first" and shape["choice_pin_policy"] == "none"
    phases = [json.loads(line) for line in (directory / "result.json.phases.jsonl").read_text().splitlines()]
    assert [row["phase"] for row in phases[:3]] == ["started", "import_checked", "model_constructed"]
    assert phases[0]["detail"]["mode"] == mode
    assert phases[0]["detail"]["source_commit"] == source_commit
    assert phases[0]["detail"]["binary_sha256"] == binary_sha
    assert phases[2]["detail"] == shape
    return shape, phases


def audit_arm(directory, mode, source_commit, binary_sha, object_path):
    directory = Path(directory)
    supervisor = _common_receipt(directory, mode, source_commit, binary_sha)
    if (directory / "result.json.cap.json").exists():
        cap = read(directory / "result.json.cap.json")
        assert supervisor["exit_code"] == 124 and not supervisor["outer_cap_reached"]
        assert cap["schema"] == "n83.full-d18-m4-cap/v1" and cap["status"] == "UNKNOWN_wall_cap"
        assert cap["mode"] == mode and cap["budget_seconds"] == 1200
        assert cap["isolation"] == "hard_8GiB_cgroup_zero_swap"
        assert cap["selected_best_total_runtime"] is None
        phases = [json.loads(line) for line in (directory / "result.json.phases.jsonl").read_text().splitlines()]
        assert phases[0]["phase"] == "started" and phases[0]["detail"]["source_commit"] == source_commit
        model = (directory / "result.json.shape.json")
        if model.exists():
            _shape_and_phases(directory, mode, source_commit, binary_sha)
        return {"status": "UNKNOWN_wall_cap", "result": None, "relation": None,
                "phase_count": len(phases), "process_wall_ms": cap["elapsed_process_wall_ms"]}
    assert supervisor["exit_code"] == 0 and not supervisor["outer_cap_reached"]
    result = read(directory / "result.json")
    assert read(directory / "stdout.json") == result
    assert result["source_commit"] == source_commit and result["binary_sha256"] == binary_sha
    assert result["source_attestation_mode"] == "clean_checkout"
    assert result["mode"] == mode and result["isolation"] == "hard_8GiB_cgroup_zero_swap"
    assert result["memory_cgroup_limit_bytes"] == MEMORY_BYTES
    assert 0 < result["memory_cgroup_peak_bytes"] <= MEMORY_BYTES
    assert result["selected_best_total_runtime"] is None
    assert result["root_encoding"] == "shared_root_bits" and result["search_policy"] == "phase_zero_first"
    assert result["choice_pin_policy"] == "none" and result["root_choice_pinned"] is False
    assert result["budget_seconds"] == 1200 and result["process_wall_ms"] < 1200000
    shape, phases = _shape_and_phases(directory, mode, source_commit, binary_sha)
    assert result["shape"] == shape and len(phases) == 4 and phases[3]["phase"] == "solver_returned"
    relation = None
    if mode == "positive":
        assert result["status"] == "PASS_CONSTRUCTED_CONTROL" and result["solver_answer"] == "Sat"
        assert result["relation"]["matches_planted_row"] is True
        relation = replay_relation(result, object_path)
        assert relation["indices"] == [0, 166, 332, 498]
        negative = {}
        for label in ("changed_target", "changed_row_digest"):
            mutated = copy.deepcopy(result)
            if label == "changed_target":
                mutated["target"][0] = str(int(mutated["target"][0]) ^ 1)
            else:
                mutated["relation"]["row_decimal_blake3"] = "0" * 64
            try:
                replay_relation(mutated, object_path)
            except AssertionError:
                negative[label] = "REJECTED"
            else:
                raise AssertionError(label)
        relation["negative_controls"] = negative
    elif result["status"] == "PENDING_INDEPENDENT_NATURAL_REPLAY":
        relation = replay_relation(result, object_path)
    else:
        assert result["status"] in ("UNKNOWN_search_cap", "MODEL_GROUP_REJECTED", "UNVERIFIED_UNSAT")
    return {"status": result["status"], "solver_answer": result["solver_answer"],
            "solver_conflicts": result["solver_conflicts"], "import_ms": shape["import_ms"],
            "model_ms": shape["model_ms"], "search_ms": result["search_ms"],
            "process_wall_ms": result["process_wall_ms"], "memory_peak_bytes": result["memory_cgroup_peak_bytes"],
            "relation": relation}


if __name__ == "__main__":
    if len(sys.argv) != 7:
        raise SystemExit("usage: audit.py SOURCE BASE BINARY SOURCE_COMMIT ARM_DIR MODE")
    source, base, binary, commit, arm_dir, mode = sys.argv[1:]
    identity = verify_input(source, base, binary, commit)
    print(json.dumps({"identity": identity, "arm": audit_arm(arm_dir, mode, commit,
          identity["binary_sha256"], Path(base) / OBJECT_REL)}, indent=2, sort_keys=True))
