#!/usr/bin/env python3
"""Frozen post-run analysis for the ECBench E2E frontier shard campaign.

The driver delegates cryptographic replay, pricing and paired bootstrap work to
the exact pinned ``ecbench`` executable.  It never drops an incomplete session:
flow, verification failures, missing specs and every comparison command remain
in ``analysis.json``.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent
MANIFEST = ROOT / "execution" / "shard-manifest.json"
PREFLIGHT = ROOT / "execution" / "shard-preflight-plans.json"
WEIGHTS = ("0", "0.1", "1")
RESAMPLES = "4000"


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def safe(text: str) -> str:
    return re.sub(r"[^A-Za-z0-9_.-]+", "_", text)


def command(argv: list[str]) -> dict:
    completed = subprocess.run(argv, check=False, capture_output=True, text=True)
    return {
        "argv": argv,
        "returncode": completed.returncode,
        "stdout": completed.stdout,
        "stderr": completed.stderr,
    }


def load_campaign(binary: Path) -> tuple[dict, dict, dict[str, dict]]:
    manifest = json.loads(MANIFEST.read_text())
    preflight = json.loads(PREFLIGHT.read_text())
    actual_binary = sha256(binary)
    if actual_binary != manifest["binary_sha256"]:
        raise ValueError(
            f"binary SHA-256 mismatch: expected {manifest['binary_sha256']}, got {actual_binary}"
        )
    if preflight["status"] != "valid" or preflight["failures"]:
        raise ValueError("shard planner receipt is not valid")
    if preflight["binary_sha256"] != actual_binary:
        raise ValueError("planner receipt names a different executable")

    plan_rows = {row["spec_id"]: row for row in preflight["specs"]}
    if len(plan_rows) != len(manifest["jobs"]):
        raise ValueError("planner receipt/spec manifest cardinality mismatch")
    specs: dict[str, dict] = {}
    for job in manifest["jobs"]:
        spec_path = ROOT / job["spec"]
        spec = json.loads(spec_path.read_text())
        matching = [row for row in preflight["specs"] if row["spec"] == job["spec"]]
        if len(matching) != 1:
            raise ValueError(f"missing or duplicate preflight row for {job['spec']}")
        row = matching[0]
        if sha256(spec_path) != row["file_sha256"]:
            raise ValueError(f"spec bytes changed after preflight: {job['spec']}")
        specs[row["spec_id"]] = {"job": job, "spec": spec, "preflight": row}
    return manifest, preflight, specs


def discover_sessions(root: Path, specs: dict[str, dict]) -> tuple[dict[str, Path], list[dict]]:
    sessions: dict[str, Path] = {}
    observations: list[dict] = []
    for session_file in sorted(root.rglob("session.json")):
        try:
            session = json.loads(session_file.read_text())
        except (OSError, json.JSONDecodeError) as exc:
            observations.append({"path": str(session_file), "error": f"unreadable: {exc}"})
            continue
        spec_id = session.get("spec_id")
        if spec_id not in specs:
            observations.append({"path": str(session_file), "spec_id": spec_id, "state": "out_of_campaign"})
            continue
        if spec_id in sessions:
            observations.append(
                {
                    "spec_id": spec_id,
                    "error": "duplicate_session",
                    "paths": [str(sessions[spec_id]), str(session_file.parent)],
                }
            )
            continue
        if session.get("spec_sha256") != specs[spec_id]["preflight"]["spec_sha256"]:
            observations.append(
                {
                    "spec_id": spec_id,
                    "path": str(session_file.parent),
                    "error": "spec_sha256_mismatch",
                }
            )
            continue
        sessions[spec_id] = session_file.parent
    return sessions, observations


def compare_seed(spec_id: str, reference: str, candidate: str, weight: str) -> str:
    material = f"{spec_id}\0{reference}\0{candidate}\0{weight}".encode()
    return str(int.from_bytes(hashlib.sha256(material).digest()[:8], "big"))


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--ecbench-bin", type=Path, required=True)
    parser.add_argument("--sessions-root", type=Path)
    parser.add_argument("--out", type=Path)
    parser.add_argument("--check-only", action="store_true")
    parser.add_argument("--allow-partial", action="store_true")
    args = parser.parse_args()

    binary = args.ecbench_bin.resolve()
    manifest, preflight, specs = load_campaign(binary)
    if args.check_only:
        print(
            json.dumps(
                {
                    "status": "valid",
                    "specs": len(specs),
                    "planned_executions": preflight["total_executions"],
                    "binary_sha256": sha256(binary),
                    "weights": WEIGHTS,
                    "resamples": int(RESAMPLES),
                },
                sort_keys=True,
            )
        )
        return 0
    if args.sessions_root is None or args.out is None:
        parser.error("--sessions-root and --out are required unless --check-only is used")
    if args.out.exists():
        raise SystemExit(f"refusing to overwrite analysis output: {args.out}")

    sessions, discovery = discover_sessions(args.sessions_root, specs)
    missing = sorted(set(specs) - set(sessions))
    duplicates = [item for item in discovery if item.get("error") == "duplicate_session"]
    args.out.mkdir(parents=True)
    (args.out / "verifications").mkdir()
    (args.out / "resources").mkdir()
    (args.out / "comparisons").mkdir()

    verifications = []
    resources = []
    comparisons = []
    hard_failures = [item for item in discovery if "error" in item]
    for spec_id, session_dir in sorted(sessions.items()):
        info = specs[spec_id]
        session = json.loads((session_dir / "session.json").read_text())
        if session.get("binary_sha256") != manifest["binary_sha256"]:
            hard_failures.append(
                {"spec_id": spec_id, "path": str(session_dir), "error": "binary_sha256_mismatch"}
            )
            continue

        verify_out = args.out / "verifications" / f"{spec_id}.json"
        verified = command(
            [str(binary), "verify", "--dir", str(session_dir), "--replay-all", "--out", str(verify_out)]
        )
        verifications.append(
            {
                "spec_id": spec_id,
                "session_id": session.get("session_id"),
                "session_status": session.get("status"),
                "receipt": str(verify_out),
                "command": verified,
            }
        )
        if verified["returncode"] != 0:
            hard_failures.append({"spec_id": spec_id, "error": "verify_failed"})

        resource_out = args.out / "resources" / f"{spec_id}.json"
        resource_cmd = command(
            [str(binary), "resources", "--dir", str(session_dir), "--out", str(resource_out)]
        )
        resources.append({"spec_id": spec_id, "output": str(resource_out), "command": resource_cmd})
        if resource_cmd["returncode"] != 0:
            hard_failures.append({"spec_id": spec_id, "error": "resources_failed"})

        spec_arms = info["spec"]["arms"]
        references = [item for item in spec_arms if item["role"] == "reference"]
        candidates = [item for item in spec_arms if item["role"] in {"candidate", "baseline"}]
        controls = [item for item in spec_arms if item["role"] == "control"]
        pairs = [(reference, candidate, "candidate") for reference in references for candidate in candidates]
        for reference in references:
            for control in controls:
                if control["method"] == reference["method"]:
                    pairs.append((reference, control, "aa"))

        for reference, candidate, comparison_role in pairs:
            for weight in WEIGHTS:
                seed = compare_seed(spec_id, reference["name"], candidate["name"], weight)
                result = command(
                    [
                        str(binary),
                        "compare",
                        "--dir",
                        str(session_dir),
                        "--a",
                        reference["name"],
                        "--b",
                        candidate["name"],
                        "--resamples",
                        RESAMPLES,
                        "--seed",
                        seed,
                        "--weight",
                        weight,
                        "--json",
                        "--allow-interrupted",
                    ]
                )
                rel = Path("comparisons") / (
                    f"{spec_id}__{safe(reference['name'])}__{safe(candidate['name'])}__w{safe(weight)}.json"
                )
                (args.out / rel).write_text(result["stdout"])
                comparisons.append(
                    {
                        "spec_id": spec_id,
                        "role": comparison_role,
                        "reference": reference["name"],
                        "candidate": candidate["name"],
                        "weight": weight,
                        "bootstrap_seed": seed,
                        "output": str(rel),
                        "returncode": result["returncode"],
                        "stderr": result["stderr"],
                    }
                )
                if result["returncode"] != 0:
                    hard_failures.append(
                        {
                            "spec_id": spec_id,
                            "error": "comparison_failed",
                            "reference": reference["name"],
                            "candidate": candidate["name"],
                            "weight": weight,
                        }
                    )

    band = None
    if sessions:
        band_out = args.out / "band.json"
        argv = [str(binary), "band"]
        for session_dir in sessions.values():
            argv.extend(["--dir", str(session_dir)])
        argv.extend(["--weights", ",".join(WEIGHTS), "--json", str(band_out), "--ranks"])
        band = command(argv)
        (args.out / "band.txt").write_text(band["stdout"])
        if band["returncode"] != 0:
            hard_failures.append({"error": "band_failed"})

    result = {
        "schema": "ecbench.frontier-shard-analysis/v1",
        "protocol_id": "ECBENCH-E2E-FRONTIER-20261031",
        "source_commit": manifest["source_commit"],
        "binary_sha256": manifest["binary_sha256"],
        "manifest_sha256": sha256(MANIFEST),
        "preflight_sha256": sha256(PREFLIGHT),
        "weights": list(WEIGHTS),
        "resamples": int(RESAMPLES),
        "flow": {
            "planned_specs": len(specs),
            "observed_specs": len(sessions),
            "missing_specs": missing,
            "duplicate_specs": duplicates,
            "discovery_observations": discovery,
        },
        "verifications": verifications,
        "resources": resources,
        "comparisons": comparisons,
        "band": band,
        "failures": hard_failures,
        "status": "complete" if not missing and not hard_failures else "incomplete",
    }
    (args.out / "analysis.json").write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps({"status": result["status"], **result["flow"]}, sort_keys=True))
    if hard_failures:
        return 1
    if missing and not args.allow_partial:
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
