#!/usr/bin/env python3
"""Build a content-addressed experiment.export/v1 record for chain-exact runs."""

from __future__ import annotations

import hashlib
import json
import os
import platform
import socket
import sys
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


STUDY = Path(__file__).resolve().parents[1]


def read_json(path: Path) -> Any:
    return json.loads(path.read_text(encoding="utf-8"))


def sha256(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def info(path: Path, kind: str) -> dict[str, Any]:
    return {
        "path": path.relative_to(STUDY).as_posix(),
        "kind": kind,
        "bytes": path.stat().st_size,
        "sha256": sha256(path),
    }


def case_input_path(case: dict[str, Any]) -> Path:
    # The frozen protocol records repository-relative input directories.
    repo = STUDY.parents[1]
    return repo / case["directory"]


def host_summary() -> dict[str, Any]:
    cpu = None
    try:
        for line in Path("/proc/cpuinfo").read_text().splitlines():
            if line.lower().startswith("model name"):
                cpu = line.split(":", 1)[1].strip()
                break
    except OSError:
        pass
    try:
        logical_cpus = len(os.sched_getaffinity(0))
    except (AttributeError, OSError):
        logical_cpus = os.cpu_count()
    return {
        "cpu_model": cpu,
        "kernel": platform.release(),
        "machine": platform.machine(),
        "logical_cpus_available_to_process": logical_cpus,
        "hostname": socket.gethostname(),
        "timing_tier": "L0",
        "timing_claim": False,
    }


def build(root: Path) -> None:
    root = root.resolve()
    protocol = read_json(root / "protocol.json")
    summary = read_json(root / "certificates" / "summary.json")
    protocol_cases = protocol["extensions"]["cases"]
    result_rows = {r["key"]: r for r in summary["rows"]}
    if len(protocol_cases) != 28 or set(result_rows) != {c["key"] for c in protocol_cases}:
        raise SystemExit("protocol/result case identities do not match the frozen 28-case panel")

    cert_dir = root / "certificates"
    global_path = cert_dir / "global.json"
    controls_path = cert_dir / "controls.json"
    summary_path = cert_dir / "summary.json"
    replay_path = root / "replay.json"
    shared = [info(global_path, "certificate"), info(controls_path, "control")]
    runs = []
    for case in protocol_cases:
        result = result_rows[case["key"]]
        input_dir = case_input_path(case)
        input_path = input_dir / "input.json"
        source_path = input_dir / "system.sing"
        if sha256(input_path) != case["input_sha256"] or sha256(source_path) != case["source_sha256"]:
            raise SystemExit(f"input/source hash mismatch for {case['key']}")
        artifacts = [*shared, info(summary_path, "summary")]
        if result.get("certificate"):
            cert = cert_dir / result["certificate"]
            artifacts.insert(0, info(cert, "certificate"))
        outcome = "exact_original_top" if result.get("verified") else "error"
        runs.append(
            {
                "arm": case["arm"],
                "artifacts": artifacts,
                "attempts": 1,
                "backend_ref": None,
                "checkpoint": None,
                "exclusion": {"excluded": False, "rule": None},
                "finished_at": None,
                "instance_id": case["id"],
                "isolation": {
                    "blockers": ["No elapsed-time or ECDLP cost claim"],
                    "contended": None,
                    "tier": "L0",
                },
                "measurements": {
                    "counted": {
                        "dreg_exact": result.get("dreg_exact"),
                        "dreg_lower_bound": result.get("dreg_lower_bound"),
                        "dreg_upper_bound": result.get("dreg_upper_bound"),
                        "prior_disjoint_upper_bound": result.get("prior_disjoint_upper_bound"),
                        "defect_socle_upper": result.get("defect_socle_upper"),
                    },
                    "counters": {},
                    "cpu_sys_seconds": None,
                    "cpu_user_seconds": None,
                    "max_rss_bytes": None,
                    "wall_seconds": None,
                    "wall_valid": False,
                },
                "outcome_class": outcome,
                "replicate": 0,
                "resumed": 0,
                "run_id": case["key"],
                "seed": None,
                "started_at": None,
                "status": "succeeded" if result.get("verified") else "failed",
                "verification": {
                    "status": "verified" if result.get("verified") else "failed",
                    "verifier": (
                        "Native source/top-form checks and fresh-process exact replay; "
                        "full Lean formalization and independent scientific review remain pending"
                    ),
                },
                "warmup": False,
                "worker": socket.gethostname(),
            }
        )

    completed = sum(r["status"] == "succeeded" for r in runs)
    host_path = root / "host.json"
    host_path.write_text(json.dumps({"schema": "n83.chain-exact-host/v1", "summary": host_summary()}, indent=2) + "\n", encoding="utf-8")
    files = []
    for p in sorted(root.rglob("*")):
        if not p.is_file() or p.name in {"export.json", "manifest.sha256"}:
            continue
        kind = "summary" if p == summary_path else "raw"
        if p == root / "protocol.json":
            kind = "protocol"
        elif p == root / "CHAIN_EXACT_DESIGN.md":
            kind = "design"
        elif p == replay_path:
            kind = "replay"
        elif p.parent == cert_dir and p.suffix == ".json":
            kind = "certificate"
        files.append(info(p, kind))

    protocol_file = root / "protocol.json"
    export = {
        "analysis": {
            "script": info(STUDY / "scripts" / "build_chain_exact_export.py", "analysis"),
            "outputs": [info(summary_path, "summary")],
            "blinded": False,
            "blinding_key_sha256": None,
            "unmasked_at": None,
        },
        "arms": [
            {
                "id": a["id"],
                "role": a["role"],
                "commit": a["instrument"]["commit"],
                "binary_sha256": a["instrument"]["binary_sha256"],
                "image_digest": None,
            }
            for a in protocol["arms"]
        ],
        "backend": "local",
        "created": datetime.now(timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z"),
        "export_id": protocol["extensions"]["export_id"],
        "files": files,
        "flow": {
            "planned": len(protocol_cases),
            "attempted": len(runs),
            "completed": completed,
            "not_completed": len(runs) - completed,
            "interrupted": 0,
            "resumed": 0,
            "excluded": [],
        },
        "host": {
            "class_id": "local-migration-host-L0",
            "path": host_path.relative_to(STUDY).as_posix(),
            "sha256": sha256(host_path),
            "summary": read_json(host_path)["summary"],
        },
        "notes": [
            {
                "by": "producer",
                "kind": "observation",
                "text": (
                    "No random draws or timing claims. Inputs are content-addressed in the "
                    "preserved spike38 S3 run index; this experiment creates new local/Git "
                    "certificates only."
                ),
            },
            {
                "by": "producer",
                "kind": "limitation",
                "text": "The certificate concerns the original highest-form ideal; it does not compute FFD, LFD, affine solving degree, or rank factor bases.",
            },
        ],
        "producer": {"runtime": "codex", "role": "producer", "model": None, "session": protocol["extensions"]["export_id"]},
        "protocol": {
            "path": protocol_file.relative_to(STUDY).as_posix(),
            "protocol_id": protocol["protocol_id"],
            "version": protocol["version"],
            "sha256": protocol["freeze"]["protocol_sha256"],
        },
        "reviews_dir": root.relative_to(STUDY).as_posix() + "/reviews",
        "runs": runs,
        "schema": "experiment.export/v1",
        "source": {
            "repo": "crypto",
            "commit": protocol["arms"][0]["instrument"]["commit"],
            "dirty": True,
        },
    }
    (root / "export.json").write_text(json.dumps(export, indent=2) + "\n", encoding="utf-8")
    manifest_rows = []
    for p in sorted(root.rglob("*")):
        if p.is_file() and p.name != "manifest.sha256":
            manifest_rows.append(f"{sha256(p)}  {p.relative_to(root).as_posix()}")
    (root / "manifest.sha256").write_text("\n".join(manifest_rows) + "\n", encoding="utf-8")
    print(json.dumps({"export": str(root / "export.json"), "runs": len(runs), "completed": completed, "files": len(files)}, sort_keys=True))


if __name__ == "__main__":
    if len(sys.argv) != 2:
        raise SystemExit(f"usage: {Path(sys.argv[0]).name} RUN_ROOT")
    build(Path(sys.argv[1]))
