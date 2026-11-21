#!/usr/bin/env python3
"""Independently replay a completed hosted canonical N83 4+2 pair artifact."""

from __future__ import annotations

import argparse
from copy import deepcopy
import hashlib
import json
import math
from pathlib import Path
import sys

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]
PRIOR = HERE.parent / "hosted-m6-paired-run-39054543374"
sys.path.insert(0, str(PRIOR))
import replay as prior  # noqa: E402

SOURCE_COMMIT = "cdaa0a96391084de881064e99ca7a8da0f527370"
PLAN_SHA256 = "7788514127328a78cf2b0f634003ddae3ff18a9ff0c0c8cd75b50fcc006152cd"
IMAGE = "rust:1.98.1-bookworm@sha256:93ce27a88655056a51dbdd8f5f2d7ddc071c7b0070fb288a37b5a285fc83971e"
ARCHIVE_DIR = "n83-archives"
SETUP_DIR = "n83-m6-canonical-setup"
RUN_DIR = "n83-m6-canonical-pair"
S3_RECEIPT = PRIOR / "factor-bases-s3-check.json"
SOURCE_PATHS = {
    ".github/workflows/n83-hosted-m6-canonical-pair.yml",
    "examples/koblitz_n83_mixed_orbit_worker.rs",
    "src/cryptanalysis/binary_semaev_chain_sat.rs",
    "src/cryptanalysis/koblitz_large_prime.rs",
    "research/koblitz_n83_factor_base_sweep_20261008/verification/hosted-m6-paired-protocol-20261017/plan.py",
    "research/koblitz_n83_factor_base_sweep_20261008/verification/hosted-m6-paired-protocol-20261017/plan.json",
    "research/koblitz_n83_factor_base_sweep_20261008/verification/hosted-m6-paired-protocol-20261017/run.py",
    "research/koblitz_n83_factor_base_sweep_20261008/verification/hosted-m6-paired-run-39054543374/factor-bases-s3-check.json",
    "research/koblitz_n83_factor_base_sweep_20261008/verification/canonical-m6-paired-20261017/PROTOCOL.md",
    "research/koblitz_n83_factor_base_sweep_20261008/verification/canonical-m6-paired-20261017/run.py",
    "Cargo.lock",
}

require = prior.require
read_json = prior.read_json
same = prior.same
sha256 = prior.sha256


def artifact_tree(root: Path) -> dict:
    """Bind every raw hosted artifact byte, including failed-stage logs."""
    digest = hashlib.sha256()
    files_sha256 = {}
    total_bytes = 0
    for path in sorted(root.rglob("*")):
        require(not path.is_symlink(), f"artifact symlink differs: {path}")
        if path.is_dir():
            continue
        require(path.is_file(), f"artifact entry is not a file: {path}")
        relative = path.relative_to(root).as_posix()
        size = path.stat().st_size
        file_sha256 = sha256(path)
        digest.update(f"{relative}\0{size}\0{file_sha256}\n".encode())
        files_sha256[relative] = file_sha256
        total_bytes += size
    require(files_sha256, "artifact tree is empty")
    return {"sha256": digest.hexdigest(), "file_count": len(files_sha256),
            "total_bytes": total_bytes, "files_sha256": files_sha256}


def audit_mounts(command: list, inspected: dict, arm: dict) -> None:
    require(isinstance(command, list) and command[:2] == ["docker", "run"]
            and command.count("--mount") == 6, "Docker invocation differs")
    mounted = {}
    for index, token in enumerate(command[:-1]):
        if token != "--mount":
            continue
        parts = command[index + 1].split(",")
        fields = dict(part.split("=", 1) for part in parts if "=" in part)
        fields["readonly"] = "readonly" in parts
        require(fields.get("type") == "bind" and "dst" in fields
                and "src" in fields and fields["dst"] not in mounted,
                "Docker bind syntax differs")
        mounted[fields["dst"]] = fields
    expected = {
        "/worker": (f"/{SETUP_DIR}/worker", True),
        "/base.jsonl.gz": (f"/{ARCHIVE_DIR}/{arm['base_object_blake3']}.jsonl.gz", True),
        "/pool.jsonl.gz": (f"/{ARCHIVE_DIR}/{arm['pool_object_blake3']}.jsonl.gz", True),
        "/pilot-manifest.json": (
            "/research/koblitz_n83_factor_base_sweep_20261008/pilot-01/manifest.json", True),
        "/corpus.json": (
            "/research/koblitz_n83_factor_base_sweep_20261008/pilot-01/probe-corpus.json", True),
        "/out": (f"/{RUN_DIR}/{arm['role']}", False),
    }
    require(set(mounted) == set(expected), "Docker bind targets differ")
    actual = {item["Destination"]: item for item in inspected["Mounts"]
              if item["Type"] == "bind"}
    require(set(actual) == set(expected), "inspected Docker binds differ")
    for target, (suffix, readonly) in expected.items():
        row = mounted[target]
        require(row["src"].endswith(suffix)
                and row["readonly"] is readonly
                and actual[target]["Source"] == row["src"]
                and actual[target]["RW"] is not readonly,
                f"inspected Docker bind differs: {target}")


def audit_stage(arm_dir: Path, arm: dict, stage: str, argv: list) -> dict:
    row = read_json(arm_dir / f"{stage}.stage.json")
    same(row, {"schema": "n83.hosted-m6-paired-stage/v1",
               "stage": stage, "role": arm["role"],
               "wall_cap_seconds": 300 if stage == "import" else 2400,
               "inspect_exit_code": 0, "cleanup_exit_code": 0},
         f"{stage} stage")
    require(type(row.get("elapsed_wall_seconds_L0")) in (int, float)
            and math.isfinite(row["elapsed_wall_seconds_L0"])
            and 0 <= row["elapsed_wall_seconds_L0"] <= row["wall_cap_seconds"] + 30,
            f"{stage} wall counter differs")
    require(sha256(arm_dir / f"{stage}.stdout") == row["stdout_sha256"]
            and sha256(arm_dir / f"{stage}.stderr") == row["stderr_sha256"],
            f"{stage} raw output digest differs")
    require((arm_dir / f"{stage}.cleanup.stdout").read_text().strip() ==
            row["cleanup_stdout"], f"{stage} cleanup output differs")
    inspected = json.loads((arm_dir / f"{stage}.inspect.json").read_text())
    require(isinstance(inspected, list) and len(inspected) == 1,
            f"{stage} container inspection missing")
    require(row["command"][-len(argv):] == argv
            and row["command"][-len(argv) - 1] == IMAGE,
            f"{stage} worker argv or image differs")
    audit_mounts(row["command"], inspected[0], arm)
    state, host = inspected[0]["State"], inspected[0]["HostConfig"]
    require(state["OOMKilled"] is False and host["Memory"] == 4294967296
            and host["MemorySwap"] == 4294967296
            and host["NanoCpus"] == 1000000000
            and host["PidsLimit"] == 64 and host["NetworkMode"] == "none"
            and host["ReadonlyRootfs"] is True
            and [cap.upper() for cap in host["CapDrop"]] == ["ALL"]
            and any(option.startswith("no-new-privileges")
                    for option in host["SecurityOpt"]),
            f"{stage} container limits differ")
    return row


def audit_search(config: dict, result: dict, arm: dict,
                 base_points: list, residual: set) -> str:
    same(config, {"schema": "n83.cross-stream-pilot-partial-config/v2",
                  "ordering": "equal_domain_runs"}, "canonical search config")
    same(result, {"schema": "n83.cross-stream-pilot-partial-result/v2",
                  "ordering": "equal_domain_runs"}, "canonical search result")
    require("order_first_pair" not in config, "legacy first-pair flag in canonical config")
    normalized_config = dict(config)
    normalized_config["schema"] = "n83.cross-stream-pilot-partial-config/v1"
    normalized_config.pop("ordering")
    normalized_config["order_first_pair"] = True
    normalized_result = dict(result)
    normalized_result["schema"] = "n83.cross-stream-pilot-partial-result/v1"
    normalized_result.pop("ordering")
    capacity = deepcopy(arm)
    # The prior plan includes one 82-wire comparator; this arm has four.
    capacity["source_capacity"]["total_variables"] += 3 * 82
    status = prior.audit_search(normalized_config, normalized_result, capacity,
                                base_points, residual)
    candidate = result.get("candidate")
    if candidate is not None:
        xs = [base_points[i][0] for i in candidate["base_point_indices"]]
        xs.extend(prior.prior_audit.PRIOR.parse_point(p)[0]
                  for p in candidate["residual_points"])
        require(xs[:4] == sorted(xs[:4]) and xs[4:] == sorted(xs[4:]),
                "candidate does not satisfy canonical ordering")
    return status


def audit_artifact(root: Path, s3_bases: Path, run_id: str) -> dict:
    require(root.is_dir() and run_id.isdecimal(), "artifact or run ID differs")
    expected = prior.plan.build()
    require(sha256(prior.plan.SAVED) == PLAN_SHA256
            and read_json(prior.plan.SAVED) == expected,
            "frozen 4+2 plan differs")
    require(tuple(map(int, expected["public_target"])) == prior.prior_audit.PUBLIC_TARGET
            and expected["first_trial_a"] == "1833078656608906180176350"
            and expected["first_trial_b"] == "163089659398503559150141",
            "public target or first-trial coefficients differ")
    prior.audit_s3_bases(s3_bases, expected)
    setup = root / SETUP_DIR
    require((setup / "source-commit.txt").read_text().strip() == SOURCE_COMMIT
            and (setup / "source-status.txt").read_text() == "",
            "hosted source commit or cleanliness differs")
    same(read_json(setup / "plan-check.json"),
         {"status": "PASS", "sha256": PLAN_SHA256}, "plan check")
    source_lines = (setup / "sources.sha256").read_text().splitlines()
    require(len(source_lines) == len(SOURCE_PATHS), "hosted source-list size differs")
    actual_paths = set()
    for line in source_lines:
        digest, relative = line.split("  ", 1)
        require(not Path(relative).is_absolute()
                and sha256(ROOT / relative) == digest,
                f"hosted source hash differs: {relative}")
        actual_paths.add(relative)
    require(actual_paths == SOURCE_PATHS, "hosted source-list paths differ")
    image_id, arch, system, repo_digests = (
        setup / "image-identity.txt").read_text().strip().split(" ", 3)
    require(image_id.startswith("sha256:") and len(image_id) == 71
            and arch == "arm64" and system == "linux"
            and any(value.endswith("@" + IMAGE.split("@", 1)[1])
                    for value in json.loads(repo_digests)),
            "pinned Linux ARM image identity differs")
    regenerated = read_json(setup / "regeneration.json")
    same(regenerated, {"schema": "n83.hosted-regenerated-archives/v1",
                       "status": "PASS_3_FROZEN_OBJECTS",
                       "source_commit": SOURCE_COMMIT,
                       "workflow_dispatch_head_sha": SOURCE_COMMIT,
                       "pilot_completed_base_count": 54}, "regeneration")
    manifest = root / "n83-pilot-parent/panel/manifest.json"
    same(read_json(manifest), {"schema": "n83.factor-base-panel/v1",
                               "status": "completed_factor_base_panel",
                               "source_commit": SOURCE_COMMIT,
                               "completed_base_count": 54,
                               "expected_base_count": 54}, "pilot manifest")
    require(sha256(manifest) == regenerated["pilot_manifest_sha256"],
            "regenerated pilot manifest digest differs")
    archive_hashes = {}
    for arm in expected["arms"]:
        for kind in ("base", "pool"):
            key = arm[f"{kind}_object_blake3"]
            digest = sha256(root / ARCHIVE_DIR / f"{key}.jsonl.gz")
            require(digest == arm[f"{kind}_compressed_sha256"],
                    f"regenerated archive differs: {key}")
            archive_hashes[f"{key}.jsonl.gz"] = digest
    require(regenerated["archive_sha256"] == archive_hashes,
            "regeneration archive receipt differs")
    config = read_json(root / RUN_DIR / "config.json")
    same(config, {"schema": "n83.hosted-m6-canonical-pair-config/v1",
                  "source_commit": SOURCE_COMMIT,
                  "plan_sha256": PLAN_SHA256,
                  "protocol_sha256": sha256(HERE / "PROTOCOL.md"),
                  "runner_sha256": sha256(HERE / "run.py"),
                  "s3_custody_receipt_sha256": sha256(S3_RECEIPT),
                  "worker_sha256": sha256(setup / "worker"),
                  "manifest_sha256": sha256(prior.PILOT_MANIFEST),
                  "corpus_sha256": sha256(prior.PUBLIC_CORPUS),
                  "image": IMAGE, "run_id": run_id,
                  "ordering": "equal_domain_runs",
                  "conservative_full_charge_seconds": 7500,
                  "selected_best_total_runtime": None}, "outer config")
    require((setup / "worker.sha256").read_text().startswith(
        config["worker_sha256"] + "  "), "worker binary hash receipt differs")
    outcomes = []
    for arm in expected["arms"]:
        arm_dir = root / RUN_DIR / arm["role"]
        imported = audit_stage(arm_dir, arm, "import", arm["worker_verify_argv"])
        same(imported, {"exit_code": 0, "timed_out": False}, "import stage")
        prior.prior_audit.audit_verify(read_json(arm_dir / "import.stdout"), arm)
        base = root / ARCHIVE_DIR / f"{arm['base_object_blake3']}.jsonl.gz"
        pool = root / ARCHIVE_DIR / f"{arm['pool_object_blake3']}.jsonl.gz"
        base_points, residual = prior.prior_audit.domains(
            base, pool, arm["shared_signed_orbits"])
        argv = list(arm["worker_search_argv"])
        require(argv[1] == "run-cross-pilot", "prior search command changed")
        argv[1] = "run-cross-pilot-canonical"
        searched = audit_stage(arm_dir, arm, "search", argv)
        if searched["timed_out"]:
            require(searched["exit_code"] is None, "outer cap has process code")
            status = "UNKNOWN_outer_wall_cap"
        elif searched["exit_code"] == 124:
            same(read_json(arm_dir / "search/cap.json"),
                 {"schema": "n83.cross-stream-pilot-partial-cap/v1",
                  "status": "UNKNOWN_budget", "budget_seconds": 2300},
                 "worker cap")
            status = "UNKNOWN_worker_wall_cap"
        else:
            require(searched["exit_code"] == 0, "search container failed")
            status = audit_search(read_json(arm_dir / "search/config.json"),
                                  read_json(arm_dir / "search/result.json"),
                                  arm, base_points, residual)
        arm_outer = read_json(arm_dir / "outer.json")
        require(arm_outer == {"role": arm["role"], "status": status,
                              "import": imported, "search": searched},
                "per-arm outer receipt differs")
        outcomes.append({"role": arm["role"], "status": status,
                         "search_exit_code": searched["exit_code"],
                         "elapsed_search_wall_seconds_L0": searched["elapsed_wall_seconds_L0"]})
    outer = read_json(root / RUN_DIR / "outer.json")
    same(outer, {"schema": "n83.hosted-m6-canonical-pair-outer/v1",
                 "status": "BOUNDED_SCREEN_complete_pair",
                 "source_commit": SOURCE_COMMIT, "run_id": run_id,
                 "conservative_full_charge_seconds": 7500,
                 "natural_partial_yield": None, "matrix_rank": None,
                 "total_index_calculus_runtime_ms": None,
                 "selected_best_total_runtime": None}, "outer")
    require([row["role"] for row in outer["arms"]] == expected["arm_order"]
            and [row["status"] for row in outer["arms"]] ==
            [row["status"] for row in outcomes]
            and type(outer["elapsed_supervisor_wall_seconds_L0"]) in (int, float)
            and math.isfinite(outer["elapsed_supervisor_wall_seconds_L0"])
            and 0 <= outer["elapsed_supervisor_wall_seconds_L0"] <= 7500,
            "paired outer receipt differs")
    return {"schema": "n83.hosted-m6-canonical-pair-audit/v1",
            "status": "PASS_MATCHED_CANONICAL_ONE_QUERY_REPLAY",
            "source_commit": SOURCE_COMMIT, "plan_sha256": PLAN_SHA256,
            "raw_artifact_tree": artifact_tree(root),
            "archives_sha256": archive_hashes, "outcomes": outcomes,
            "s3_factor_bases_status": "PASS_3_FROZEN_S3_OBJECTS",
            "conservative_full_charge_seconds": 7500,
            "selected_best_total_runtime": None}


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--artifact", required=True, type=Path)
    parser.add_argument("--s3-bases", required=True, type=Path)
    parser.add_argument("--run-id", required=True)
    args = parser.parse_args()
    print(json.dumps(audit_artifact(args.artifact, args.s3_bases, args.run_id),
                     sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
