"""Independent original-field replay of the local extended D18 pair."""

import argparse
import base64
import copy
import gzip
import hashlib
import importlib.util
import json
from pathlib import Path
import re

import blake3

HERE = Path(__file__).resolve().parent
VERIFY = HERE.parent
OBJECT_NAME = "c295da8898cfad29a053621c768ca059eea4263f89f5ca78b4f98af43c383f3f.jsonl.gz"
OBJECT_SHA = "eaef94ef45b3d2d4cd2ec2f40c1173b4cdb2a437b7e618a858c73fb1caf3902b"
MANIFEST_SHA = "1b93351febd47dac4de8c5dd39dbf58564d0057a044054baf448d978f6c0d616"


def read(path):
    return json.loads(path.read_text())


def sha(path):
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def group_module():
    spec = importlib.util.spec_from_file_location(
        "n83_independent_group", VERIFY / "root-pinned-20261025/verify.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def replay_relation(result, obj, group):
    indices = result["relation"]["indices"]
    assert len(indices) == 4 and all(isinstance(index, int) and index >= 0 for index in indices)
    wanted, rows = set(indices), {}
    with gzip.open(obj, "rt") as stream:
        header = json.loads(next(stream))
        assert header["curve_a"] == 0 and header["subgroup_order"] == str(group.ORDER)
        for index, line in enumerate(stream):
            if index in wanted:
                rows[index] = json.loads(line)
            if len(rows) == len(wanted):
                break
    assert len(rows) == len(wanted)
    generator = tuple(map(int, header["generator"]))
    target = tuple(map(int, result["target"]))
    assert group.on_curve(generator) and group.scalar_mul(generator, group.ORDER) is None
    assert group.on_curve(target) and group.scalar_mul(target, group.ORDER) is None
    total, coeffs = None, [0] * 32643
    for index in indices:
        row = rows[index]
        point = tuple(map(int, row["point"]))
        assert group.on_curve(point) and group.scalar_mul(point, group.ORDER) is None
        total = group.add(total, point)
        column = int(row["column"])
        assert 0 <= column < 32643
        coeffs[column] = (coeffs[column] + int(row["coefficient"])) % group.ORDER
    assert total == target
    digest = blake3.blake3(json.dumps([str(c) for c in coeffs], separators=(",", ":")).encode()).hexdigest()
    assert digest == result["relation"]["row_decimal_blake3"]
    assert result["relation"]["rank_after_insert"] == 1
    assert result["relation"]["tamper_rejected"] is True
    assert group.add(target, generator) != total
    return {"indices": indices, "row_decimal_blake3": digest,
            "group_replay": "PASS", "tamper_rejected": True}


def audit_arm(path, mode, source_commit, binary_sha, obj, group):
    supervisor = read(path / "supervisor.json")
    assert supervisor["source_commit"] == source_commit
    assert supervisor["binary_sha256"] == binary_sha
    assert supervisor["outer_cap_seconds"] == (310 if mode == "positive" else 1210)
    assert supervisor["outer_cap_reached"] is False
    assert supervisor["isolation"] == "L0_shared_host_no_hard_cgroup"
    assert supervisor["selected_best_total_runtime"] is None
    command = read(path / "command.json")
    expected_mode = "primary-full-d18-m4-l0-shared-phase-first" + (
        "-natural-1200" if mode == "natural" else "")
    assert command[0:2] == ["/usr/bin/time", "-l"] and command[3] == expected_mode
    shape = read(path / "result.json.shape.json")
    assert shape["variables"] == 329726 and shape["xor_rows"] == 3901
    assert shape["clauses"] == (2807752 if mode == "positive" else 2807254)
    assert shape["root_encoding"] == "shared_root_bits"
    assert shape["choice_pin_policy"] == "none" and shape["search_policy"] == "phase_zero_first"
    phases = [json.loads(line) for line in (path / "result.json.phases.jsonl").read_text().splitlines()]
    assert phases[0]["phase"] == "started"
    assert phases[0]["detail"]["source_commit"] == source_commit
    assert phases[0]["detail"]["binary_sha256"] == binary_sha
    assert phases[2]["phase"] == "model_constructed" and phases[2]["detail"] == shape
    rss_match = re.search(r"(\d+)\s+maximum resident set size", (path / "stderr-time.txt").read_text())
    assert rss_match
    if (path / "result.json.cap.json").exists():
        assert mode == "natural" and supervisor["exit_code"] == 124
        assert not (path / "result.json").exists()
        assert [item["phase"] for item in phases] == ["started", "import_checked", "model_constructed"]
        cap = read(path / "result.json.cap.json")
        assert cap["status"] == "UNKNOWN_wall_cap" and cap["budget_seconds"] == 1200
        assert cap["mode"] == "natural" and cap["selected_best_total_runtime"] is None
        return {"status": "UNKNOWN_wall_cap", "import_ms": shape["import_ms"],
                "model_ms": shape["model_ms"], "search_ms": None,
                "post_model_window_ms": cap["elapsed_process_wall_ms"] - phases[2]["elapsed_process_wall_ms"],
                "process_wall_ms": cap["elapsed_process_wall_ms"],
                "supervisor_max_rss_bytes": int(rss_match.group(1)), "relation": None}
    assert supervisor["exit_code"] == 0
    assert [item["phase"] for item in phases] == ["started", "import_checked", "model_constructed", "solver_returned"]
    result = read(path / "result.json")
    assert read(path / "stdout.json") == result
    assert result["source_commit"] == source_commit and result["binary_sha256"] == binary_sha
    assert result["mode"] == mode and result["shape"] == shape
    assert result["source_attestation_mode"] == "clean_checkout"
    assert result["budget_seconds"] == (300 if mode == "positive" else 1200)
    assert result["selected_best_total_runtime"] is None
    relation = None
    if mode == "positive":
        assert result["status"] == "PASS_CONSTRUCTED_CONTROL"
        assert result["solver_answer"] == "Sat" and result["relation"]["matches_planted_row"] is True
        relation = replay_relation(result, obj, group)
    elif result["status"] == "PENDING_INDEPENDENT_NATURAL_REPLAY":
        relation = replay_relation(result, obj, group)
    else:
        assert result["status"] in ("UNKNOWN_search_cap", "MODEL_GROUP_REJECTED", "UNVERIFIED_UNSAT", "INVALID_MODEL")
    return {"status": result["status"], "import_ms": shape["import_ms"],
            "model_ms": shape["model_ms"], "search_ms": result["search_ms"],
            "process_wall_ms": result["process_wall_ms"],
            "supervisor_max_rss_bytes": int(rss_match.group(1)), "relation": relation}


def audit(base, binary, output):
    identity = read(output / "identity.json")
    source_commit, binary_sha = identity["source_commit"], identity["binary_sha256"]
    assert len(source_commit) == 40 and sha(binary) == binary_sha
    obj = base / "objects" / OBJECT_NAME
    assert obj.stat().st_size == 160747529 and sha(obj) == OBJECT_SHA
    assert sha(base / "s3-manifest.json") == MANIFEST_SHA
    wrapper = read(base / "s3-manifest.json")
    assert wrapper["compressed_sha256"] == OBJECT_SHA
    assert wrapper["producer_manifest_sha256"] == sha(VERIFY / "manifest.json")
    object_get, manifest_get = read(base / "object-get-metadata.json"), read(base / "manifest-get-metadata.json")
    assert object_get["ChecksumSHA256"] == base64.b64encode(bytes.fromhex(OBJECT_SHA)).decode()
    assert object_get["ContentLength"] == 160747529 and object_get["ServerSideEncryption"] == "AES256"
    assert manifest_get["ChecksumSHA256"] == base64.b64encode(bytes.fromhex(MANIFEST_SHA)).decode()
    group = group_module()
    positive = audit_arm(output / "positive", "positive", source_commit, binary_sha, obj, group)
    original = read(output / "positive/result.json")
    negative_controls = {}
    for label in ("changed_target", "changed_row_digest"):
        altered = copy.deepcopy(original)
        if label == "changed_target":
            altered["target"][0] = str(int(altered["target"][0]) ^ 1)
        else:
            altered["relation"]["row_decimal_blake3"] = "0" * 64
        try:
            replay_relation(altered, obj, group)
        except AssertionError:
            negative_controls[label] = "REJECTED"
        else:
            raise AssertionError(label)
    natural = (audit_arm(output / "natural", "natural", source_commit, binary_sha, obj, group)
               if (output / "natural").exists() else None)
    status = ("PASS_CONSTRUCTED_CONTROL" if natural is None else
              "PASS_REPLAYED_NATURAL_RELATION" if natural["relation"] else
              "PASS_CENSORED_NATURAL_WALL_CAP" if natural["status"] == "UNKNOWN_wall_cap" else
              "PASS_CENSORED_NATURAL_OTHER")
    return {"schema": "n83.d18-local-extended-independent-audit/v1",
            "status": status, "source_commit": source_commit, "binary_sha256": binary_sha,
            "object_sha256": OBJECT_SHA, "positive": positive, "natural": natural,
            "negative_controls": negative_controls,
            "selected_best_total_runtime": None}


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("base", type=Path)
    parser.add_argument("binary", type=Path)
    parser.add_argument("output", type=Path)
    args = parser.parse_args()
    print(json.dumps(audit(args.base.resolve(), args.binary.resolve(), args.output.resolve()),
                     sort_keys=True, indent=2))
