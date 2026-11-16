#!/usr/bin/env python3
"""Continue only the 143 untouched cases after attempt-01's runner error."""
import argparse
import hashlib
import json
import os
import platform
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parent))
import run_first_fall as base

CANDIDATES = ["spike35", "spike36", "spike37"]
EXCLUDED_CASE = ("spike35", "koblitz-m2-planted")
CONTINUATION_S3_ROOT = "s3://setec-astronomy-artifacts/runs/n83-ffd-lfd-factorbase-panel-20261102/v2/attempt-02"


def sha(path):
    h = hashlib.sha256()
    with Path(path).open("rb") as src:
        for block in iter(lambda: src.read(8 * 1024 * 1024), b""):
            h.update(block)
    return h.hexdigest()


def selected_rows():
    protocol, index, selected, checks = base.load_and_verify(CANDIDATES)
    kept = [(candidate, row, path) for candidate, row, path in selected if (candidate, row["id"]) != EXCLUDED_CASE]
    kept_checks = [row for row in checks if (row["candidate_id"], row["id"]) != EXCLUDED_CASE]
    if len(kept) != 143 or len(kept_checks) != 143:
        raise RuntimeError("continuation must contain exactly 143 untouched systems")
    return protocol, index, kept, kept_checks


def preflight(out):
    protocol, index, selected, checks = selected_rows()
    previous = json.loads((HERE.parent / "attempt-01/cases/spike35/koblitz-m2-planted/attempt.json").read_text())
    if not previous["replay"]["verified"] or previous["certificate_sha256"] != previous["s3_readback_sha256"]:
        raise RuntimeError("attempt-01 recovered certificate is not verified and readback-bound")
    out.mkdir(parents=True, exist_ok=True)
    if any((out / name).exists() for name in ("preflight.json", "source-index.json", "script-binding.json", "cases", "summary.json")):
        raise RuntimeError("continuation preflight/output already exists; preserve it and use a fresh attempt")
    pre = {
        "schema": "n83.candidate-fall-continuation-preflight/v1",
        "protocol_id": base.EXPECTED_PROTOCOL_ID,
        "protocol_freeze_sha256": base.EXPECTED_PROTOCOL_DIGEST,
        "protocol_file_sha256": sha(base.PROTOCOL),
        "source_commit": protocol["freeze"]["commit"],
        "fall_binary_sha256": sha(base.BIN),
        "dataset_index_sha256": sha(base.DATASET_INDEX),
        "candidate_ids": CANDIDATES,
        "systems_in_original_panel": 144,
        "already_produced_and_replayed": [{"candidate_id": EXCLUDED_CASE[0], "id": EXCLUDED_CASE[1], "certificate_sha256": previous["certificate_sha256"], "fresh_replay_verified": True, "s3_readback_verified": True}],
        "systems_planned_this_continuation": len(selected),
        "systems_planned": len(selected),
        "excluded_from_rerun": [list(EXCLUDED_CASE)],
        "systems": checks,
        "s3_root": CONTINUATION_S3_ROOT,
        "execution": "existing spot workspace; one process; 8 GiB address-space cap; 60 seconds per first-fall and replay; no automatic retries; each attempt and certificate checkpointed to S3",
        "host": {"platform": platform.platform(), "python": sys.version, "uname": subprocess.check_output(["uname", "-a"], text=True).strip()},
        "decision": "The attempt-01 first certificate was not recomputed. This continuation processes only the other 143 systems, once each, under the unchanged frozen V2 scientific procedure and a fresh S3 namespace.",
    }
    (out / "preflight.json").write_text(json.dumps(pre, indent=2, sort_keys=True) + "\n")
    (out / "source-index.json").write_text(json.dumps(checks, indent=2, sort_keys=True) + "\n")
    binding = {
        "schema": "n83.candidate-fall-script-binding/v1",
        "protocol_id": base.EXPECTED_PROTOCOL_ID,
        "protocol_freeze_sha256": base.EXPECTED_PROTOCOL_DIGEST,
        "preflight_sha256": sha(out / "preflight.json"),
        "runner_path": str(Path(__file__).resolve()),
        "runner_sha256": sha(Path(__file__)),
        "analysis_path": str(HERE / "analyze_run.py"),
        "analysis_sha256": sha(HERE / "analyze_run.py"),
        "filed_before_data": True,
    }
    (out / "script-binding.json").write_text(json.dumps(binding, indent=2, sort_keys=True) + "\n")
    print(json.dumps({"preflight": str(out / "preflight.json"), "systems": len(selected), "runner_sha256": binding["runner_sha256"]}, separators=(",", ":")))


def execute(out):
    protocol, index, selected, checks = selected_rows()
    preflight_path = out / "preflight.json"
    binding_path = out / "script-binding.json"
    if not preflight_path.is_file() or not binding_path.is_file():
        raise RuntimeError("committed continuation preflight and script binding are required")
    pre = json.loads(preflight_path.read_text())
    binding = json.loads(binding_path.read_text())
    if pre["systems_planned_this_continuation"] != 143 or len(selected) != 143:
        raise RuntimeError("continuation system count mismatch")
    if binding["runner_sha256"] != sha(Path(__file__)) or binding["analysis_sha256"] != sha(HERE / "analyze_run.py"):
        raise RuntimeError("continuation script binding mismatch")
    if binding["preflight_sha256"] != sha(preflight_path):
        raise RuntimeError("continuation preflight digest mismatch")
    if sha(out / "source-index.json") != hashlib.sha256((json.dumps(checks, indent=2, sort_keys=True) + "\n").encode()).hexdigest():
        raise RuntimeError("continuation source index mismatch")
    if (out / "cases").exists() or (out / "summary.json").exists():
        raise RuntimeError("continuation outputs already exist; preserve and reconcile them before any retry")
    base.S3_ROOT = CONTINUATION_S3_ROOT
    base.__file__ = str(Path(__file__).resolve())
    original_bounded = base.bounded

    def bounded_with_default_env(command, env=None, timeout=60):
        return original_bounded(command, env or os.environ.copy(), timeout)

    base.bounded = bounded_with_default_env
    summary = base.run(out, CANDIDATES, pre, selected)
    print(json.dumps({"terminal": True, "attempted": summary["systems_attempted"], "certificates": summary["certificates_produced"], "replayed": summary["fresh_replays_verified"]}, separators=(",", ":")))


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--out", type=Path, default=HERE)
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--preflight-only", action="store_true")
    mode.add_argument("--run", action="store_true")
    args = parser.parse_args()
    if args.preflight_only:
        preflight(args.out)
    else:
        execute(args.out)


if __name__ == "__main__":
    main()
