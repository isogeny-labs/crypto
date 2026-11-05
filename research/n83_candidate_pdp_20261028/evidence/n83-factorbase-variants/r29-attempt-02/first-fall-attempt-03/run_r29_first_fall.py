#!/usr/bin/env python3
"""Run the frozen N83 V2 first-fall procedure on saved spike29 inputs.

This continuation reads only the 48 source systems already bound by the
spike29 source-binding audit. It does not regenerate factor bases or PDP
systems. Each case is independently bounded, uploaded and replayed.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import platform
import shutil
import subprocess
import sys
import tarfile
import time
from pathlib import Path


HERE = Path(__file__).resolve().parent
REPO = next(p for p in HERE.parents if (p / "research/n83_candidate_pdp_20261028/protocol-pdp-factorbase-v2.json").is_file())
STUDY = REPO / "research/n83_candidate_pdp_20261028"
SOURCE_ROOT = STUDY / "evidence/n83-factorbase-variants/r29-attempt-02/pdp-inputs/attempt-02/candidate-structure"
AUDIT_DIR = STUDY / "evidence/n83-factorbase-variants/r29-attempt-02/source-binding-audit-20261103"
AUDIT_SCRIPT = AUDIT_DIR / "audit.py"
AUDIT_JSON = AUDIT_DIR / "audit.json"
PROTOCOL = STUDY / "protocol-pdp-factorbase-v2.json"
BIN = STUDY / "target/release/fall"
PROTOCOL_ID = "N83-FACTOR-BASE-PDP-FFD-LFD-20261031-V2"
PROTOCOL_FREEZE_SHA256 = "50cd6f8213789de2dfd54a3b33a067ed87cc0def761b7c98769be3e19aee47a9"
PROTOCOL_FILE_SHA256 = "8a9143e9588ce1ec5c204362b734f790696de90569ce1de3c086e4645e605885"
FALL_SHA256 = "33c859d58f30f6facf483b0df61c9ffe7339298513c15001b6583605c11c1a14"
SOURCE_COMMIT = "52665c7a6e8308f86201e10c9305afa346fd134b"
S3_ROOT = "s3://setec-astronomy-artifacts/runs/n83-ffd-lfd-factorbase-panel-20261105/v2/spike29/attempt-01"
EXPECTED_SOURCE_INDEX_SHA256 = "f7325211302b4ca21f98ea9a7022d2dcc023b29ea2fad9329867e90b75a01c82"


def sha256(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            h.update(block)
    return h.hexdigest()


def canonical_sha(value: object) -> str:
    raw = json.dumps(value, sort_keys=True, separators=(",", ":")).encode()
    return hashlib.sha256(raw).hexdigest()


def load_inputs() -> tuple[dict, list[dict], list[dict]]:
    protocol = json.loads(PROTOCOL.read_text())
    if protocol["freeze"]["protocol_sha256"] != PROTOCOL_FREEZE_SHA256:
        raise RuntimeError("frozen protocol digest mismatch")
    if sha256(PROTOCOL) != PROTOCOL_FILE_SHA256:
        raise RuntimeError("frozen protocol file hash mismatch")
    if protocol["freeze"]["commit"] != SOURCE_COMMIT:
        raise RuntimeError("frozen source commit mismatch")
    if sha256(BIN) != FALL_SHA256:
        raise RuntimeError("fall binary hash mismatch")
    audit = subprocess.run([sys.executable, str(AUDIT_SCRIPT), "--check"], cwd=REPO, text=True, capture_output=True)
    if audit.returncode != 0:
        raise RuntimeError(f"source binding audit failed: {audit.stdout}{audit.stderr}")
    audit_record = json.loads(AUDIT_JSON.read_text())
    if audit_record["pdp_inputs"]["input_anchor_sha256"] != EXPECTED_SOURCE_INDEX_SHA256:
        raise RuntimeError("spike29 source anchor mismatch")
    rows = json.loads((SOURCE_ROOT / "inputs.json").read_text())
    if len(rows) != 48:
        raise RuntimeError(f"expected 48 source systems, got {len(rows)}")
    expected_ids = {
        f"{family}-m{m}-{target}"
        for family in ("koblitz", "binary", "prime")
        for m in range(2, 10)
        for target in ("natural", "planted")
    }
    if {row["id"] for row in rows} != expected_ids:
        raise RuntimeError("source case cross is incomplete or contains extras")
    arms = {a["id"]: a for a in protocol["arms"]}
    for family in ("koblitz", "binary", "prime"):
        arm = arms.get(f"spike29-{family}")
        if arm is None or arm["parameters"]["candidate_id"] != "spike29":
            raise RuntimeError(f"frozen V2 protocol has no spike29 {family} arm")
    checks = []
    for row in sorted(rows, key=lambda x: x["id"]):
        system = SOURCE_ROOT / row["id"] / "system.sing"
        if not system.is_file() or sha256(system) != row["source_sha256"]:
            raise RuntimeError(f"source system mismatch: {row['id']}")
        checks.append({
            "id": row["id"],
            "family": row["family"],
            "m": row["m"],
            "target_kind": row["target_kind"],
            "input_row_sha256": canonical_sha(row),
            "system_sha256": row["source_sha256"],
            "system_bytes": system.stat().st_size,
            "factor_base_manifest_sha256": row["factor_base_manifest_sha256"],
        })
    return audit_record, rows, checks


def run_bounded(command: list[str], env: dict[str, str], timeout: int = 60) -> tuple[list[str], subprocess.CompletedProcess, float]:
    wrapped = ["timeout", "--kill-after=5s", f"{timeout}s", "prlimit", "--as=8589934592", "--", *command]
    start = time.monotonic()
    proc = subprocess.run(wrapped, text=True, capture_output=True, env=env)
    return wrapped, proc, round(time.monotonic() - start, 3)


def s3_copy(source: Path, destination: str) -> None:
    subprocess.run(["aws", "s3", "cp", str(source), destination, "--only-show-errors", "--region", "us-east-2"], check=True)


def assert_s3_prefix_empty() -> None:
    result = subprocess.run(
        ["aws", "s3api", "list-objects-v2", "--bucket", "setec-astronomy-artifacts",
         "--prefix", S3_ROOT.removeprefix("s3://setec-astronomy-artifacts/"),
         "--max-keys", "1", "--query", "KeyCount", "--output", "text", "--region", "us-east-2"],
        text=True, capture_output=True, check=True,
    )
    if result.stdout.strip() != "0":
        raise RuntimeError("S3 output prefix is not empty; preserve and choose a fresh attempt")


def preflight(out: Path, publication_sha: str) -> None:
    audit, rows, checks = load_inputs()
    assert_s3_prefix_empty()
    if out.exists():
        raise RuntimeError("preflight output path already exists; preserve it and choose a fresh attempt")
    out.mkdir(parents=True)
    control = subprocess.run([str(BIN), "selfcheck"], text=True, capture_output=True, timeout=60)
    if control.returncode != 0:
        raise RuntimeError(f"fall selfcheck failed: {control.stdout}{control.stderr}")
    control_result = json.loads(control.stdout.strip())
    if control_result.get("positive", {}).get("first_fall_degree") != 2:
        raise RuntimeError("fall selfcheck positive control was not detected")
    pre = {
        "schema": "n83.spike29-first-fall-preflight/v1",
        "created_utc": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
        "protocol": {"id": PROTOCOL_ID, "frozen_sha256": PROTOCOL_FREEZE_SHA256, "file_sha256": sha256(PROTOCOL), "source_commit": SOURCE_COMMIT},
        "published_runner_commit": publication_sha,
        "runner_sha256": sha256(Path(__file__).resolve()),
        "source_binding_audit_sha256": sha256(AUDIT_JSON),
        "source_anchor_sha256": audit["pdp_inputs"]["input_anchor_sha256"],
        "source_index_sha256": sha256(SOURCE_ROOT / "inputs.json"),
        "factor_base_index_sha256": sha256(SOURCE_ROOT / "factor-base-index.json"),
        "fall_binary_sha256": sha256(BIN),
        "candidate_id": "spike29",
        "systems_planned": len(rows),
        "cases": checks,
        "controls": {"selfcheck_sha256": canonical_sha(control_result), "selfcheck": control_result},
        "execution": {
            "backend": "local existing Spot workspace",
            "cpus": 1,
            "memory_limit_bytes": 8589934592,
            "per_case_timeout_seconds": 60,
            "fresh_replay_timeout_seconds": 60,
            "global_deadline_seconds": 7200,
            "infrastructure_retries": 0,
            "max_attempts_per_case": 1,
            "max_gross_compute_cost_usd": 2.00,
            "max_s3_artifact_cost_usd": 0.25,
            "checkpoint": "upload each certificate and attempt.json before advancing; preserve all terminal failures",
        },
        "launch_command": f"timeout --signal=TERM --kill-after=10s 7200s python3 {Path(__file__).resolve()} --out {out.resolve()} --run",
        "s3_root": S3_ROOT,
        "existing_prefix_checked_empty": True,
        "s3_prefix_check_utc": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
        "host": {"platform": platform.platform(), "python": sys.version, "uname": subprocess.check_output(["uname", "-a"], text=True).strip()},
        "analysis": "Count certificate-backed first-fall degrees and explicit lower-bound outcomes; no exact LFD, affine degree of regularity, solving-degree, or factor-base ranking is inferred.",
        "decision": "Run the 48 source-bound spike29 systems once each under frozen V2 case semantics; retain failures and do not retry or overwrite this prefix.",
    }
    (out / "preflight.json").write_text(json.dumps(pre, indent=2, sort_keys=True) + "\n")
    (out / "source-index.json").write_text(json.dumps(checks, indent=2, sort_keys=True) + "\n")
    binding = {
        "schema": "n83.spike29-first-fall-binding/v1",
        "protocol_sha256": PROTOCOL_FREEZE_SHA256,
        "preflight_sha256": sha256(out / "preflight.json"),
        "runner_sha256": sha256(Path(__file__).resolve()),
        "source_binding_audit_sha256": sha256(AUDIT_JSON),
        "filed_before_data": True,
    }
    (out / "script-binding.json").write_text(json.dumps(binding, indent=2, sort_keys=True) + "\n")
    print(json.dumps({"preflight_sha256": sha256(out / "preflight.json"), "runner_sha256": binding["runner_sha256"], "systems": len(rows)}, separators=(",", ":")))


def execute(out: Path) -> None:
    audit, rows, checks = load_inputs()
    pre_path = out / "preflight.json"
    binding_path = out / "script-binding.json"
    if not pre_path.is_file() or not binding_path.is_file():
        raise RuntimeError("committed preflight and script binding are required before data collection")
    pre = json.loads(pre_path.read_text())
    binding = json.loads(binding_path.read_text())
    if binding["runner_sha256"] != sha256(Path(__file__).resolve()) or binding["preflight_sha256"] != sha256(pre_path):
        raise RuntimeError("published runner/preflight binding mismatch")
    if pre["protocol"]["frozen_sha256"] != PROTOCOL_FREEZE_SHA256 or pre["s3_root"] != S3_ROOT:
        raise RuntimeError("protocol or output-prefix mismatch")
    if len(rows) != pre["systems_planned"] or len(checks) != len(pre["cases"]):
        raise RuntimeError("source cohort changed after preflight")
    if (out / "cases").exists() or (out / "summary.json").exists():
        raise RuntimeError("execution outputs already exist; preserve and reconcile before retry")
    assert_s3_prefix_empty()
    (out / "cases").mkdir()
    # Bind and upload the plan before the first measured case.
    for name in ("preflight.json", "source-index.json", "script-binding.json"):
        s3_copy(out / name, f"{S3_ROOT}/metadata/{name}")
    s3_copy(Path(__file__).resolve(), f"{S3_ROOT}/metadata/run_r29_first_fall.py")
    s3_copy(AUDIT_JSON, f"{S3_ROOT}/metadata/source-binding-audit.json")
    env = os.environ.copy()
    env["ISOLAB_RESULTS_URI"] = S3_ROOT
    results = []
    for row in sorted(rows, key=lambda x: x["id"]):
        case_id = row["id"]
        case_dir = out / "cases" / case_id
        input_root = case_dir / "input"
        output_root = case_dir / "output"
        case_dir.mkdir()
        input_root.mkdir()
        (input_root / "inputs.json").write_text(json.dumps([row], separators=(",", ":")) + "\n")
        (input_root / case_id).mkdir()
        system = SOURCE_ROOT / case_id / "system.sing"
        (input_root / case_id / "system.sing").symlink_to(system)
        command, proc, elapsed = run_bounded([str(BIN), "run", str(input_root), str(output_root)], env)
        cert = output_root / f"{case_id}.json"
        record = {
            "candidate_id": "spike29", "id": case_id, "family": row["family"], "m": row["m"],
            "target_kind": row["target_kind"], "system_sha256": sha256(system), "input_row_sha256": canonical_sha(row),
            "command": command, "returncode": proc.returncode, "elapsed_seconds": elapsed,
            "stdout": proc.stdout, "stderr": proc.stderr,
            "status": "completed" if proc.returncode == 0 and cert.is_file() else ("timeout" if proc.returncode == 124 else "failed"),
        }
        if cert.is_file():
            data = json.loads(cert.read_text())
            record["certificate_sha256"] = sha256(cert)
            record["first_fall_degree"] = data.get("first_fall_degree")
            record["first_fall_lower_bound"] = data.get("first_fall_lower_bound")
            readback = case_dir / "s3-readback.json"
            try:
                subprocess.run(["aws", "s3", "cp", f"{S3_ROOT}/certificates/{case_id}.json", str(readback), "--only-show-errors", "--region", "us-east-2"], check=True, capture_output=True)
                record["s3_readback_sha256"] = sha256(readback)
                if record["s3_readback_sha256"] != record["certificate_sha256"]:
                    record["status"] = "s3_readback_mismatch"
            except subprocess.CalledProcessError as error:
                record["s3_readback_error"] = (error.stderr or b"").decode(errors="replace")[-2000:]
                record["status"] = "s3_readback_failed"
            replay_command, replay, replay_elapsed = run_bounded([str(BIN), "replay", str(input_root), str(cert)], os.environ.copy())
            record["replay"] = {"command": replay_command, "returncode": replay.returncode, "elapsed_seconds": replay_elapsed, "stdout": replay.stdout, "stderr": replay.stderr, "verified": replay.returncode == 0}
            if not record["replay"]["verified"]:
                record["status"] = "replay_failed"
        attempt_path = case_dir / "attempt.json"
        attempt_path.write_text(json.dumps(record, indent=2, sort_keys=True) + "\n")
        results.append(record)
        try:
            s3_copy(attempt_path, f"{S3_ROOT}/attempts/{case_id}.json")
        except subprocess.CalledProcessError as error:
            failure = {"schema": "n83.spike29-upload-failure/v1", "case": case_id, "attempt_sha256": sha256(attempt_path), "error": str(error)}
            (case_dir / "upload-failure.json").write_text(json.dumps(failure, indent=2, sort_keys=True) + "\n")
            print(json.dumps({"terminal": True, "stop_reason": "attempt upload failed", "completed_cases": len(results), "case": case_id}, separators=(",", ":")), flush=True)
            break
        print(json.dumps({"id": case_id, "status": record["status"], "returncode": proc.returncode, "elapsed_seconds": elapsed}, separators=(",", ":")), flush=True)
    summary = {
        "schema": "n83.spike29-first-fall-run/v1", "protocol_sha256": PROTOCOL_FREEZE_SHA256,
        "preflight_sha256": sha256(pre_path), "systems_planned": 48, "systems_attempted": len(results),
        "certificates": sum("certificate_sha256" in r for r in results),
        "fresh_replays_verified": sum(bool(r.get("replay", {}).get("verified")) for r in results),
        "s3_readbacks_verified": sum(r.get("certificate_sha256") == r.get("s3_readback_sha256") for r in results),
        "status_counts": {s: sum(r["status"] == s for r in results) for s in sorted({r["status"] for r in results})},
        "rows": [{k: r.get(k) for k in ("id", "family", "m", "target_kind", "status", "first_fall_degree", "first_fall_lower_bound", "system_sha256", "certificate_sha256", "s3_readback_sha256", "replay")} for r in results],
        "scope": "spike29 first-fall records only; does not establish exact LFD, affine PDP regularity, solving degree, or factor-base ranking",
    }
    summary_path = out / "summary.json"
    summary_path.write_text(json.dumps(summary, indent=2, sort_keys=True) + "\n")
    (out / "README.md").write_text("# spike29 first-fall attempt 01\n\nThis run applies frozen V2 first-fall semantics to 48 source-bound spike29 systems. Each attempt and certificate is stored separately; certificates are freshly replayed and read back from S3. See `FILE_LAYOUT.json` and `custody.json` for byte-level inventory.\n\nThis is first-fall evidence only. It does not assert exact LFD, affine degree of regularity, solving degree, or factor-base ranking.\n")
    files = []
    for path in sorted(out.rglob("*")):
        if path.name in {"FILE_LAYOUT.json", "custody.json", "panel-complete.tar.gz"}:
            continue
        if path.is_symlink():
            files.append({"path": path.relative_to(out).as_posix(), "type": "symlink", "target": os.readlink(path), "source_sha256": sha256(path.resolve())})
        elif path.is_file():
            files.append({"path": path.relative_to(out).as_posix(), "type": "file", "bytes": path.stat().st_size, "sha256": sha256(path)})
    layout = {"schema": "n83.file-layout/v1", "root": str(out.relative_to(REPO)), "file_count": len(files), "files": files, "exclusions": {"FILE_LAYOUT.json": "self-reference", "custody.json": "binds archive and layout after creation", "panel-complete.tar.gz": "bound by custody.json"}}
    layout_path = out / "FILE_LAYOUT.json"
    layout_path.write_text(json.dumps(layout, indent=2, sort_keys=True) + "\n")
    archive = out / "panel-complete.tar.gz"
    with tarfile.open(archive, "w:gz") as tar:
        for path in [Path(x["path"]) for x in files if x["type"] == "file"] + [Path("FILE_LAYOUT.json")]:
            tar.add(out / path, arcname=path.as_posix())
    custody = {"schema": "n83.s3-custody/v1", "s3_root": S3_ROOT, "archive": {"key": f"{S3_ROOT}/panel-complete.tar.gz", "bytes": archive.stat().st_size, "sha256": sha256(archive)}, "file_layout_sha256": sha256(layout_path), "summary_sha256": sha256(summary_path), "certificate_readbacks_verified": summary["s3_readbacks_verified"], "attempts": summary["systems_attempted"]}
    custody_path = out / "custody.json"
    custody_path.write_text(json.dumps(custody, indent=2, sort_keys=True) + "\n")
    for path, key in ((summary_path, "summary.json"), (layout_path, "FILE_LAYOUT.json"), (Path(out / "README.md"), "README.md"), (archive, "panel-complete.tar.gz"), (custody_path, "custody.json")):
        s3_copy(path, f"{S3_ROOT}/{key}")
    print(json.dumps({"terminal": True, "attempted": len(results), "certificates": summary["certificates"], "replayed": summary["fresh_replays_verified"], "s3_readbacks": summary["s3_readbacks_verified"], "archive_sha256": custody["archive"]["sha256"]}, separators=(",", ":")))


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", required=True, type=Path)
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--preflight-only", action="store_true")
    mode.add_argument("--run", action="store_true")
    parser.add_argument("--publication-sha", default="")
    args = parser.parse_args()
    if args.preflight_only:
        if not args.publication_sha:
            parser.error("--publication-sha is required for preflight")
        preflight(args.out, args.publication_sha)
    else:
        execute(args.out)


if __name__ == "__main__":
    main()
