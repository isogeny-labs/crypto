"""Audit the retained bounded local-VM N83 circuit comparison receipts."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path
import tarfile


ROOT = Path(__file__).resolve().parent / "evidence-01"
SOURCE_COMMIT = "88893c8deb6fa19e490c945f5d89d2a7220bf8c8"
SCHEDULE = ((1182, "schoolbook"), (2048, "shallow"),
            (1182, "shallow"), (2048, "schoolbook"))


def digest(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def read_json(path: Path) -> dict:
    return json.loads(path.read_text(encoding="utf-8"))


def main() -> None:
    manifest = read_json(ROOT / "manifest.json")["sha256"]
    actual = {str(path.relative_to(ROOT)): digest(path.read_bytes())
              for path in ROOT.rglob("*") if path.is_file()
              and path.name != "manifest.json"}
    assert actual == manifest, "evidence file set or hash changed"

    environment = read_json(ROOT / "environment.json")
    plan = read_json(ROOT / "rosetta-plan.json")
    assert environment["source_commit"] == plan["source_commit"] == SOURCE_COMMIT
    assert plan["schedule"] == [
        {"columns": k, "kernel": "shallow_karatsuba" if mode == "shallow"
         else "schoolbook"} for k, mode in SCHEDULE]

    with tarfile.open(ROOT / "source-snapshot.tar.gz", "r:gz") as archive:
        sources = {member.name.removeprefix("source/"): archive.extractfile(member).read()
                   for member in archive.getmembers() if member.isfile()}
    targets: set[tuple[str, str]] = set()
    rows = []
    for architecture in ("rosetta", "arm64"):
        for number, (columns, mode) in enumerate(SCHEDULE, start=1):
            directory = ROOT / "runs" / architecture / f"arm{number}-k{columns}-{mode}"
            config = read_json(directory / "config.json")
            outer = read_json(directory / "outer.json")
            assert config["source_commit"] == SOURCE_COMMIT
            assert config["binary_sha256"] == environment["worker_sha256"][architecture]
            assert config["columns"] == columns and config["summands"] == 5
            assert config["domain_encoding"] == "frobenius_orbit_phase"
            assert config["product_kernel"] == ("shallow_karatsuba" if mode == "shallow"
                                                else "schoolbook")
            assert config["selected_object_sha256"] == plan["panels"][str(columns)]["object_sha256"]
            assert config["fixture_sha256"] == plan["fixture_sha256"]
            assert config["conflict_budget"] == 5000
            assert config["memory_cgroup_limit_bytes"] == 4 * 1024**3
            assert config["memory_cgroup_swap_limit_bytes"] == 0
            assert config["cpu_limit"] == 1.0 and config["wall_seconds"] == 280
            assert digest((directory / "config.json").read_bytes()) == outer["config_sha256"]
            assert not any(outer[key] for key in ("source_changed", "config_changed",
                                                   "binary_changed", "input_changed"))
            assert outer["cleanup_error"] is None and outer["launch_error"] is None
            assert outer["selected_best_total_runtime"] is None
            for name, hexdigest in config["source_sha256"].items():
                assert name in sources and digest(sources[name]) == hexdigest

            if mode == "shallow" and columns == 2048:
                assert outer["status"] == "UNKNOWN_wall_cap"
                assert outer["worker_exit_code"] == 137
                assert outer["report"] is None and outer["group_replay"] is None
                assert not (directory / "run/summary.json").exists()
                assert b"RelationCollectionStarted" in (directory / "run/events.jsonl").read_bytes()
            else:
                summary = read_json(directory / "run/summary.json")
                worker_config = read_json(directory / "run/config.json")
                assert outer["status"] == summary["status"] == "UNKNOWN_solver_cap"
                assert outer["group_replay"]["status"] == "PASS"
                assert outer["worker_exit_code"] == 0
                assert digest((directory / "run/summary.json").read_bytes()) == outer["worker_sha256"]
                assert digest((directory / "run/config.json").read_bytes()) == outer["worker_config_sha256"]
                assert summary["report"]["sat_conflicts"] == 5000
                assert all(summary["report"][key] == 0 for key in
                           ("sat_models", "relations", "terminal_matrix_rank"))
                assert worker_config["fixture"] == summary["fixture"] == 0
                targets.add(tuple(summary["attempt_records"][0]["target"]))
            rows.append({"architecture": architecture, "columns": columns,
                         "kernel": mode, "status": outer["status"],
                         "outer_wall_ms": round(outer["process_wall_ms"], 3),
                         "sat_conflicts": (outer["report"] or {}).get("sat_conflicts")})
    assert targets == {("6261116474805304534310590",
                        "4505575220639647215230954")}
    print(json.dumps({"status": "PASS_bounded_receipts_only", "rows": rows,
                      "completed_arm_target": list(next(iter(targets)))},
                     sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
