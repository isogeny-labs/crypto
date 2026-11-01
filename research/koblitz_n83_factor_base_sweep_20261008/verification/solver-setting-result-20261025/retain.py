#!/usr/bin/env python3
"""Bind retained provider ZIPs, jobs and independent N83 setting audits."""

import argparse
from collections import Counter
import hashlib
import json
from pathlib import Path
import sys


HERE = Path(__file__).resolve().parent
PANEL = HERE.parent / "solver-setting-panel-20261025"
COLLECTOR = HERE.parent / "solver-setting-collector-20261025"
sys.path.insert(0, str(PANEL))
sys.path.insert(0, str(COLLECTOR))
from audit_artifact import audit  # noqa: E402
from collect import assess  # noqa: E402

PROTOCOL = json.loads((PANEL / "protocol.json").read_bytes())
RUN_ID = 40193751812
ATTEMPT = 1
HEAD = "a5af8d5023a644e9e3b347af4d8495540c8dea99"
ORDER = ("base", "vsids", "matrix48", "vsids_matrix48", "base_repeat")


def require(ok: bool, reason: str) -> None:
    if not ok:
        raise ValueError(reason)


def sha(raw: bytes) -> str:
    return hashlib.sha256(raw).hexdigest()


def retain(root: Path, source_dir: Path | None = None,
           solver_zip: Path | None = None) -> dict:
    require((source_dir is None) == (solver_zip is None),
            "source ZIPs and solver ZIP supplied together")
    expected = {row["id"]: row for row in PROTOCOL["strata"]}
    require(len(expected) == 8 and PROTOCOL["schema"]
            == "n83.solver-setting-screen-protocol/v1",
            "exact frozen eight-stratum protocol")
    all_directories = {path.name: path for path in root.iterdir()
                       if path.is_dir()}
    directories = {name: path for name, path in all_directories.items()
                   if name != "collector-audits"}
    require(directories and set(all_directories)
            <= set(expected) | {"collector-audits"},
            "only named provider strata retained")
    rows = []
    for stratum, directory in sorted(directories.items()):
        raw = (directory / "artifact.zip").read_bytes()
        meta = json.loads((directory / "artifact-metadata.json").read_bytes())
        jobs = json.loads((directory / "provider-jobs.json").read_bytes())
        prior = json.loads((directory / "audit.json").read_bytes())
        digest = sha(raw)
        selected = [job for job in jobs["jobs"]
                    if job["name"] == f"solve-setting-stratum ({stratum})"]
        require(meta["name"]
                == f"n83-solver-setting-{stratum}-{RUN_ID}-{ATTEMPT}"
                and meta["size_in_bytes"] == len(raw)
                and meta["digest"] == f"sha256:{digest}"
                and meta["workflow_run"]["id"] == RUN_ID
                and meta["workflow_run"]["head_sha"] == HEAD
                and not meta["expired"]
                and jobs["total_count"] == len(jobs["jobs"]) == 8
                and len(selected) == 1
                and selected[0]["status"] == "completed"
                and selected[0]["conclusion"] == "success"
                and selected[0]["run_id"] == RUN_ID
                and selected[0]["head_sha"] == HEAD,
                f"{stratum}: provider identity and terminal job")
        arms = prior["arms"]
        require(prior["schema"] == "n83.solver-setting-provider-audit/v1"
                and prior["run_id"] == RUN_ID
                and prior["attempt"] == ATTEMPT
                and prior["head_sha"] == HEAD
                and prior["stratum"] == stratum
                and prior["artifact_id"] == meta["id"]
                and prior["artifact_zip_sha256"] == digest
                and prior["source_artifact_id"]
                == expected[stratum]["source_artifact_id"]
                and prior["source_artifact_zip_sha256"]
                == expected[stratum]["source_artifact_sha256"]
                and 1 <= len(arms) <= 5
                and [arm["configuration"] for arm in arms]
                == list(ORDER[:len(arms)])
                and prior["selected_best_total_runtime"] is None,
                f"{stratum}: independent audit identity and arm order")
        if source_dir is not None:
            trial = expected[stratum]["trial"]
            rebuilt = audit(stratum, directory / "artifact.zip", digest,
                            directory / "artifact-metadata.json",
                            directory / "provider-jobs.json",
                            source_dir / f"t{trial}_m0.zip",
                            source_dir / f"t{trial}_m0-metadata.json",
                            solver_zip)
            require(rebuilt == prior,
                    f"{stratum}: independent raw provider ZIP replay")
        rows.append({"stratum": stratum, "artifact_id": meta["id"],
                     "artifact_zip_sha256": digest,
                     "artifact_bytes": len(raw),
                     "audit_status": prior["status"],
                     "arms": arms})
    statuses = Counter(row["audit_status"] for row in rows)
    solver = Counter(arm["status"] for row in rows for arm in row["arms"])
    wall = sum(arm["wall_seconds_L0"] for row in rows for arm in row["arms"])
    if len(rows) == 8:
        provider = [json.loads((root / name).read_bytes()) for name in
                    ("provider-run.json", "provider-jobs.json",
                     "provider-artifacts.json")]
        assessment = assess(*provider)
        require(assessment["status"] == "READY_TO_AUDIT"
                and assessment["artifact_ids"]
                == {row["stratum"]: row["artifact_id"] for row in rows},
                "full terminal provider run and artifact identities")
        collected = json.loads((root / "collection.json").read_bytes())
        require(collected["schema"] == "n83.solver-setting-run-collection/v1"
                and collected["run_id"] == RUN_ID
                and collected["attempt"] == ATTEMPT
                and collected["head_sha"] == HEAD
                and collected["provider_job_count"] == 8
                and collected["provider_artifact_count"] == 8
                and collected["audited_strata"] == 8
                and collected["audited_arms"] == sum(solver.values())
                and collected["audit_status_counts"] == dict(statuses)
                and collected["solver_status_counts"] == dict(solver)
                and abs(collected["summed_solver_wall_seconds_L0"] - wall)
                    < 1e-6
                and collected["selected_best_total_runtime"] is None,
                "independent complete collection summary")
        for stratum, directory in directories.items():
            independent = json.loads((root / "collector-audits" /
                                      f"{stratum}.audit.json").read_bytes())
            prior = json.loads((directory / "audit.json").read_bytes())
            require(independent == prior,
                    f"{stratum}: two independent audit passes agree")
        require(collected["status"] == (
            "PASS_EIGHT_CENSORED_STRATA" if statuses
            == {"PASS_FIVE_CENSORED_SETTINGS_REPLAY": 8}
            else "AUDITED_WITH_PENDING_OR_STOPPED_ARMS"),
            "complete collector outcome")
    all_censored = (statuses
                    == {"PASS_FIVE_CENSORED_SETTINGS_REPLAY": len(rows)})
    status = ("PASS_RETAINED_ALL_EIGHT_CENSORED" if
              len(rows) == 8 and all_censored else
              "PASS_RETAINED_PARTIAL_CENSORED" if all_censored else
              "RETAINED_PENDING_OR_NONCENSORED")
    if source_dir is None:
        status = "BINDING_ONLY_SOURCE_ZIPS_REQUIRED"
    return {"schema": "n83.solver-setting-retained/v1",
            "status": status,
            "run_id": RUN_ID, "attempt": ATTEMPT, "head_sha": HEAD,
            "retained_strata": len(rows),
            "missing_strata": sorted(set(expected) - set(directories)),
            "retained_arms": sum(len(row["arms"]) for row in rows),
            "audit_status_counts": dict(statuses),
            "solver_status_counts": dict(solver),
            "summed_solver_wall_seconds_L0": wall,
            "full_provider_run_reconciliation": len(rows) == 8,
            "full_provider_zip_replay": source_dir is not None,
            "rows": rows,
            "selected_best_total_runtime": None}


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", type=Path, required=True)
    parser.add_argument("--source-dir", type=Path)
    parser.add_argument("--solver-zip", type=Path)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()
    result = retain(args.root, args.source_dir, args.solver_zip)
    raw = json.dumps(result, indent=2, sort_keys=True) + "\n"
    if args.out:
        args.out.write_text(raw)
    print(json.dumps({key: value for key, value in result.items()
                      if key != "rows"}, indent=2, sort_keys=True))
