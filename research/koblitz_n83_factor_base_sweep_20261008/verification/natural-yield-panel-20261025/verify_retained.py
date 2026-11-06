#!/usr/bin/env python3
"""Replay committed small receipts from the frozen hosted N83 pattern panel."""

import gzip
import hashlib
import json
from pathlib import Path
import re


HERE = Path(__file__).resolve().parent
ROOT = HERE / "run-40185426122-attempt-1"
STRATA = ("t1_m0", "t1_m1", "t2_m0", "t2_m1")
ARMS = ("hash51_A1", "hash51_A2", "gray51", "hash42", "gray42", "hash51_A3")


def require(ok: bool, reason: str) -> None:
    if not ok:
        raise ValueError(reason)


def sha(raw: bytes) -> str:
    return hashlib.sha256(raw).hexdigest()


def read_json(path: Path) -> dict:
    return json.loads(path.read_bytes())


def replay(root: Path = ROOT) -> dict:
    manifest = read_json(root / "retained-manifest.json")
    require(manifest["schema"] == "n83.natural-pattern-retained-manifest/v1"
            and manifest["run_id"] == 40185426122
            and manifest["attempt"] == 1
            and manifest["head_sha"] == "fd9121107c40b159b13497c475f848ea9a21aa96"
            and set(manifest["strata"]) == set(STRATA)
            and manifest["selected_best_total_runtime"] is None,
            "frozen retained manifest identity")
    build_raw = (root / "build.json").read_bytes()
    build = json.loads(build_raw)
    require(sha(build_raw) == manifest["build_json_sha256"]
            and build["source_commit"] == "83393dd87ef48f11963c582cd79b89296c4a1bea"
            and build["worker_sha256"]
            == "d7e97aaac59ab37a454d73ee0782871b6f7f266222bfe167ff9b28ec2b461aeb"
            and build["build_wall_seconds_L0"] > 0,
            "source-built worker receipt")
    jobs = read_json(root / "provider-jobs.json")
    require(jobs["total_count"] == 5 and len(jobs["jobs"]) == 5
            and {row["name"] for row in jobs["jobs"]}
            == {"build-worker"} | {f"solve-stratum ({s})" for s in STRATA}
            and all(row["run_id"] == manifest["run_id"]
                    and row["head_sha"] == manifest["head_sha"]
                    and row["status"] == "completed"
                    and row["conclusion"] == "success" for row in jobs["jobs"]),
            "five terminal provider jobs")
    require(read_json(root / "provider-build.json")["id"]
            == manifest["build_artifact_id"], "provider build artifact")
    solver_wall = 0.0
    export_wall = 0.0
    for stratum in STRATA:
        entry = manifest["strata"][stratum]
        audit = read_json(root / f"audit-{stratum}.json")
        provider = read_json(root / f"provider-{stratum}.json")
        require(audit["status"] == "PASS_HOSTED_STRATUM_REPLAY"
                and audit["stratum"] == stratum
                and audit["run_id"] == manifest["run_id"]
                and audit["artifact_id"] == provider["id"] == entry["artifact_id"]
                and audit["artifact_zip_sha256"] == entry["artifact_zip_sha256"]
                and audit["unique_candidate_hashes"] == []
                and audit["selected_best_total_runtime"] is None,
                f"{stratum}: independent provider replay")
        members = entry["members"]
        require(len(members) == 60
                and len({row["member"] for row in members}) == 60,
                f"{stratum}: complete provider member inventory")
        by_name = {}
        for row in members:
            name = row["member"]
            require(name.startswith("n83-pattern-meta/")
                    or name.startswith("n83-pattern-results/"),
                    f"{stratum}: member namespace")
            if "retained_path" not in row:
                require(name.endswith(".xcnf")
                        or name.endswith(("apt-install.log", "apt-update.log")),
                        f"{stratum}: only large formulas and apt logs omitted")
                continue
            path = root / row["retained_path"]
            require(path.resolve().is_relative_to(root.resolve()),
                    f"{stratum}: safe retained path")
            saved = path.read_bytes()
            require(sha(saved) == row["retained_sha256"],
                    f"{stratum}: retained SHA-256 for {name}")
            require(row["compression"] in ("none", "gzip-mtime0"),
                    f"{stratum}: declared retention encoding")
            raw = gzip.decompress(saved) if row["compression"] == "gzip-mtime0" else saved
            require(len(raw) == row["bytes"] and sha(raw) == row["sha256"],
                    f"{stratum}: provider member SHA-256 for {name}")
            by_name[name] = raw
        prefix = "n83-pattern-results/"
        summary = json.loads(by_name[prefix + "summary.json"])
        require(summary["status"] == "COMPLETE_BOUNDED"
                and summary["completed_arms"] == list(ARMS)
                and summary["unique_group_checked_candidate_hashes"] == []
                and summary["selected_best_total_runtime"] is None,
                f"{stratum}: complete bounded summary")
        require([row["arm"] for row in audit["arms"]] == list(ARMS),
                f"{stratum}: frozen arm order")
        subtotal = 0.0
        for audited in audit["arms"]:
            arm = audited["arm"]
            receipt = json.loads(by_name[prefix + arm + "/process.json"])
            stdout = by_name[prefix + arm + "/solver.stdout"]
            statuses = re.findall(rb"(?m)^s (SATISFIABLE|UNSATISFIABLE|INDETERMINATE)\s*$", stdout)
            counts = re.findall(rb"(?m)^c conflicts\s+:\s+(\d+)\s*$", stdout)
            require(statuses == [b"INDETERMINATE"] and len(counts) == 1
                    and receipt["status"] == audited["status"] == "INDETERMINATE"
                    and receipt["conflicts"] == audited["conflicts"] == int(counts[0])
                    and receipt["wall_seconds_L0"] == audited["wall_seconds_L0"]
                    and receipt["stdout_sha256"] == sha(stdout)
                    and receipt["stderr_sha256"]
                    == sha(by_name[prefix + arm + "/solver.stderr"]),
                    f"{stratum}/{arm}: retained raw solver stream")
            subtotal += audited["wall_seconds_L0"]
        require(abs(subtotal - summary["summed_solver_wall_seconds_L0"]) < 1e-6,
                f"{stratum}: summed solver wall")
        solver_wall += subtotal
        export_wall += summary["summed_export_wall_seconds_L0"]
    return {"schema": "n83.natural-pattern-retained-replay/v1",
            "status": "PASS_RETAINED_24_CENSORED_ARMS",
            "run_id": manifest["run_id"],
            "strata": list(STRATA),
            "solver_arms": 24,
            "summed_solver_process_wall_seconds_L0": solver_wall,
            "summed_export_process_wall_seconds_L0": export_wall,
            "build_wall_seconds_L0": build["build_wall_seconds_L0"],
            "selected_best_total_runtime": None}


if __name__ == "__main__":
    print(json.dumps(replay(), indent=2, sort_keys=True))
