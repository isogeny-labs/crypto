#!/usr/bin/env python3
"""Run the frozen first-fall procedure on already-generated candidate systems.

This runner is intentionally narrower than the V2 protocol's full 624-system
panel: it executes the 144 already-generated systems for spike35--spike37.
It does not infer LFD from the fall certificate or rank factor bases.
"""
import argparse
import hashlib
import json
import os
import platform
import shutil
import subprocess
import sys
import time
from pathlib import Path

REPO = Path(__file__).resolve().parents[5]
STUDY = REPO / "research/n83_candidate_pdp_20261028"
SOURCE_ROOT = STUDY / "evidence/model-expansion-followup-20261101/attempt-08/inputs"
DATASET_INDEX = STUDY / "evidence/model-expansion-followup-20261101/attempt-08/DATASET-INDEX.json"
PROTOCOL = STUDY / "protocol-pdp-factorbase-v2.json"
BIN = STUDY / "target/release/fall"
EXPECTED_PROTOCOL_ID = "N83-FACTOR-BASE-PDP-FFD-LFD-20261031-V2"
EXPECTED_PROTOCOL_DIGEST = "50cd6f8213789de2dfd54a3b33a067ed87cc0def761b7c98769be3e19aee47a9"
EXPECTED_BINARY_SHA256 = "33c859d58f30f6facf483b0df61c9ffe7339298513c15001b6583605c11c1a14"
DEFAULT_CANDIDATES = ["spike35", "spike36", "spike37"]
S3_ROOT = "s3://setec-astronomy-artifacts/runs/n83-ffd-lfd-factorbase-panel-20261102/v2"


def sha256(path):
    h = hashlib.sha256()
    with Path(path).open("rb") as src:
        for block in iter(lambda: src.read(8 * 1024 * 1024), b""):
            h.update(block)
    return h.hexdigest()


def file_record(path):
    return {"path": str(path), "bytes": path.stat().st_size, "sha256": sha256(path)}


def load_and_verify(candidates):
    protocol = json.loads(PROTOCOL.read_text())
    if protocol["protocol_id"] != EXPECTED_PROTOCOL_ID:
        raise RuntimeError("unexpected protocol id")
    if protocol["freeze"]["protocol_sha256"] != EXPECTED_PROTOCOL_DIGEST:
        raise RuntimeError("protocol freeze digest mismatch")
    if sha256(BIN) != EXPECTED_BINARY_SHA256:
        raise RuntimeError("fall binary digest mismatch")
    index = json.loads(DATASET_INDEX.read_text())
    indexed = {item["path"]: item for item in index["files"]}
    selected = []
    source_checks = []
    for candidate in candidates:
        root = SOURCE_ROOT / candidate
        inputs_path = root / "inputs.json"
        indexed_input = indexed.get(f"inputs/{candidate}/inputs.json")
        if not indexed_input or sha256(inputs_path) != indexed_input["sha256"]:
            raise RuntimeError(f"inputs.json index mismatch: {candidate}")
        inputs = json.loads(inputs_path.read_text())
        if len(inputs) != 48:
            raise RuntimeError(f"expected 48 systems, found {len(inputs)} for {candidate}")
        for row in inputs:
            system = root / row["id"] / "system.sing"
            indexed_system = indexed.get(f"inputs/{candidate}/{row['id']}/system.sing")
            if not indexed_system:
                raise RuntimeError(f"system missing from dataset index: {candidate}/{row['id']}")
            if system.stat().st_size != indexed_system["bytes"] or sha256(system) != indexed_system["sha256"]:
                raise RuntimeError(f"system index mismatch: {candidate}/{row['id']}")
            if indexed_system["sha256"] != row["source_sha256"]:
                raise RuntimeError(f"system binding mismatch: {candidate}/{row['id']}")
            selected.append((candidate, row, system))
            source_checks.append({
                "candidate_id": candidate,
                "id": row["id"],
                "family": row["family"],
                "m": row["m"],
                "target_kind": row["target_kind"],
                "source_bytes": system.stat().st_size,
                "source_sha256": indexed_system["sha256"],
                "data_index_sha256": indexed_system["sha256"],
                "input_row_sha256": hashlib.sha256(json.dumps(row, sort_keys=True, separators=(",", ":")).encode()).hexdigest(),
            })
    if len(selected) != len(candidates) * 48:
        raise RuntimeError("cohort size mismatch")
    return protocol, index, selected, source_checks


def write_preflight(out, candidates, protocol, index, selected, source_checks):
    out.mkdir(parents=True, exist_ok=False)
    record = {
        "schema": "n83.candidate-fall-preflight/v1",
        "protocol_id": EXPECTED_PROTOCOL_ID,
        "protocol_freeze_sha256": EXPECTED_PROTOCOL_DIGEST,
        "protocol_file_sha256": sha256(PROTOCOL),
        "source_commit": protocol["freeze"]["commit"],
        "fall_binary_sha256": sha256(BIN),
        "dataset_index_sha256": sha256(DATASET_INDEX),
        "dataset_index_scope": index["scope"],
        "candidate_ids": candidates,
        "systems_planned": len(selected),
        "cases": source_checks,
        "s3_prefixes": {c: f"{S3_ROOT}/{c}/first-fall-attempt-01" for c in candidates},
        "execution": "local existing spot workspace; one process; 8 GiB address-space cap; 60 second per-case timeout; no automatic retries",
        "host": {
            "platform": platform.platform(),
            "python": sys.version,
            "uname": subprocess.check_output(["uname", "-a"], text=True).strip(),
        },
        "note": "This is a scoped execution of the already frozen 624-system V2 panel. It covers 144 available systems for spike35--spike37; it does not claim full-panel completion or any LFD/regularity value.",
    }
    target = out / "preflight.json"
    target.write_text(json.dumps(record, indent=2, sort_keys=True) + "\n")
    (out / "source-index.json").write_text(json.dumps(source_checks, indent=2, sort_keys=True) + "\n")
    return record


def bounded(cmd, env, timeout=60):
    wrapped = ["timeout", "--kill-after=5s", f"{timeout}s", "prlimit", "--as=8589934592", "--", *cmd]
    start = time.time()
    proc = subprocess.run(wrapped, text=True, capture_output=True, env=env)
    return wrapped, proc, round(time.time() - start, 3)


def s3_copy(source, target):
    subprocess.run(["aws", "s3", "cp", str(source), target, "--only-show-errors", "--region", "us-east-2"], check=True)


def run(out, candidates, preflight, selected):
    metadata_root = f"{S3_ROOT}/_metadata"
    metadata_files = [
        PROTOCOL,
        out / "preflight.json",
        out / "source-index.json",
        out / "script-binding.json",
        Path(__file__).resolve(),
        Path(__file__).resolve().with_name("analyze_run.py"),
        Path(__file__).resolve().with_name("README.md"),
        Path(__file__).resolve().with_name("FILE_LAYOUT.json"),
    ]
    for path in metadata_files:
        relative = path.relative_to(REPO) if path.is_relative_to(REPO) else Path("protocol-pdp-factorbase-v2.json")
        s3_copy(path, f"{metadata_root}/{relative.as_posix()}")
    rows = []
    for candidate, row, source in selected:
        case = row["id"]
        case_dir = out / "cases" / candidate / case
        input_root = case_dir / "input"
        output_root = case_dir / "output"
        input_root.mkdir(parents=True)
        (input_root / "inputs.json").write_text(json.dumps([row], separators=(",", ":")) + "\n")
        (input_root / case).mkdir()
        os.symlink(source, input_root / case / "system.sing")
        candidate_s3 = f"{S3_ROOT}/{candidate}/first-fall-attempt-01"
        env = os.environ.copy()
        env["ISOLAB_RESULTS_URI"] = candidate_s3
        command, proc, elapsed = bounded([str(BIN), "run", str(input_root), str(output_root)], env)
        cert = output_root / f"{case}.json"
        record = {
            "candidate_id": candidate,
            "id": case,
            "family": row["family"],
            "m": row["m"],
            "target_kind": row["target_kind"],
            "status": "completed" if proc.returncode == 0 and cert.is_file() else ("timeout" if proc.returncode == 124 else "failed"),
            "returncode": proc.returncode,
            "elapsed_seconds": elapsed,
            "command": command,
            "stdout": proc.stdout,
            "stderr": proc.stderr,
            "input_row_sha256": hashlib.sha256(json.dumps(row, sort_keys=True, separators=(",", ":")).encode()).hexdigest(),
            "system_sha256": sha256(source),
            "system_bytes": source.stat().st_size,
            "s3_certificate_prefix": f"{candidate_s3}/certificates/{case}.json",
        }
        if cert.is_file():
            cert_data = json.loads(cert.read_text())
            record["certificate_sha256"] = sha256(cert)
            record["first_fall_degree"] = cert_data.get("first_fall_degree")
            record["first_fall_lower_bound"] = cert_data.get("first_fall_lower_bound")
            replay_cmd, replay_proc, replay_elapsed = bounded([str(BIN), "replay", str(input_root), str(cert)])
            record["replay"] = {
                "command": replay_cmd,
                "returncode": replay_proc.returncode,
                "elapsed_seconds": replay_elapsed,
                "stdout": replay_proc.stdout,
                "stderr": replay_proc.stderr,
                "verified": replay_proc.returncode == 0,
            }
            if replay_proc.returncode != 0:
                record["status"] = "replay_failed"
        attempt = case_dir / "attempt.json"
        attempt.write_text(json.dumps(record, indent=2, sort_keys=True) + "\n")
        s3_copy(attempt, f"{candidate_s3}/attempts/{case}.json")
        rows.append(record)
        print(json.dumps({"candidate_id": candidate, "id": case, "status": record["status"], "returncode": proc.returncode, "elapsed_seconds": elapsed}, separators=(",", ":")), flush=True)
    summary = {
        "schema": "n83.candidate-fall-panel-run/v1",
        "protocol_id": EXPECTED_PROTOCOL_ID,
        "protocol_freeze_sha256": EXPECTED_PROTOCOL_DIGEST,
        "preflight_sha256": sha256(out / "preflight.json"),
        "systems_planned": len(selected),
        "systems_attempted": len(rows),
        "certificates_produced": sum(bool(r.get("certificate_sha256")) for r in rows),
        "fresh_replays_verified": sum(bool(r.get("replay", {}).get("verified")) for r in rows),
        "status_counts": {s: sum(r["status"] == s for r in rows) for s in sorted({r["status"] for r in rows})},
        "rows": [{k: r.get(k) for k in ("candidate_id", "id", "family", "m", "target_kind", "status", "first_fall_degree", "first_fall_lower_bound", "system_sha256", "certificate_sha256", "replay")} for r in rows],
        "scope": "First-fall certificates or explicit incomplete outcomes for the 144 available spike35--spike37 systems only. LFD, top-form Dreg, affine solving degree and candidate ranking remain unclaimed.",
    }
    (out / "summary.json").write_text(json.dumps(summary, indent=2, sort_keys=True) + "\n")
    s3_copy(out / "summary.json", f"{S3_ROOT}/_metadata/attempt-01/summary.json")
    return summary


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--out", required=True, type=Path)
    parser.add_argument("--candidates", nargs="+", default=DEFAULT_CANDIDATES)
    parser.add_argument("--preflight-only", action="store_true")
    parser.add_argument("--run", action="store_true")
    args = parser.parse_args()
    if args.preflight_only == args.run:
        parser.error("choose exactly one of --preflight-only or --run")
    protocol, index, selected, source_checks = load_and_verify(args.candidates)
    if args.preflight_only:
        preflight = write_preflight(args.out, args.candidates, protocol, index, selected, source_checks)
        print(json.dumps({"preflight": str(args.out / "preflight.json"), "systems": len(selected), "sha256": sha256(args.out / "preflight.json")}, separators=(",", ":")))
    else:
        preflight_path = args.out / "preflight.json"
        if not preflight_path.is_file():
            raise RuntimeError("frozen preflight must exist before data collection")
        preflight = json.loads(preflight_path.read_text())
        if preflight["candidate_ids"] != args.candidates or preflight["systems_planned"] != len(selected):
            raise RuntimeError("execution cohort differs from frozen preflight")
        if preflight["dataset_index_sha256"] != sha256(DATASET_INDEX) or preflight["fall_binary_sha256"] != sha256(BIN):
            raise RuntimeError("frozen source or instrument changed")
        binding = json.loads((args.out / "script-binding.json").read_text())
        if binding["runner_sha256"] != sha256(Path(__file__)) or binding["analysis_sha256"] != sha256(Path(__file__).with_name("analyze_run.py")):
            raise RuntimeError("runner or analysis source differs from pre-data binding")
        if (args.out / "cases").exists() or (args.out / "summary.json").exists():
            raise RuntimeError("output data already exist; use a fresh attempt directory")
        if sha256(args.out / "source-index.json") != hashlib.sha256(json.dumps(source_checks, indent=2, sort_keys=True).encode() + b"\n").hexdigest():
            raise RuntimeError("committed source index differs from inputs")
        result = run(args.out, args.candidates, preflight, selected)
        print(json.dumps({"terminal": True, "attempted": result["systems_attempted"], "certificates": result["certificates_produced"], "replayed": result["fresh_replays_verified"]}, separators=(",", ":")))


if __name__ == "__main__":
    main()
