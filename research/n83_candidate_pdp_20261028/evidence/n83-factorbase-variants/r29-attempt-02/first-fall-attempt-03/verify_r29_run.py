#!/usr/bin/env python3
"""Independently check the saved spike29 first-fall case records and hashes."""

from __future__ import annotations

import argparse
import hashlib
import json
from collections import Counter
from pathlib import Path


def sha256(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            h.update(block)
    return h.hexdigest()


def load(path: Path):
    return json.loads(path.read_text())


def verify(run: Path) -> dict:
    repo = Path(__file__).resolve()
    while not (repo / "research/n83_candidate_pdp_20261028/protocol-pdp-factorbase-v2.json").is_file():
        repo = repo.parent
    study = repo / "research/n83_candidate_pdp_20261028"
    run = run.resolve()
    source_root = study / "evidence/n83-factorbase-variants/r29-attempt-02/pdp-inputs/attempt-02/candidate-structure"
    preflight = load(run / "preflight.json")
    binding = load(run / "script-binding.json")
    producer = load(run / "summary.json")
    checks = load(run / "source-index.json")
    problems = []
    if preflight["protocol"]["frozen_sha256"] != "50cd6f8213789de2dfd54a3b33a067ed87cc0def761b7c98769be3e19aee47a9":
        problems.append("protocol hash mismatch")
    if len(checks) != 48 or producer.get("systems_planned") != 48 or producer.get("systems_attempted") != 48:
        problems.append("planned/attempted cohort is not 48")
    if binding.get("runner_sha256") != preflight.get("runner_sha256"):
        problems.append("runner hash binding mismatch")
    if binding.get("preflight_sha256") != sha256(run / "preflight.json"):
        problems.append("preflight hash binding mismatch")
    if binding.get("planned_layout_sha256") != sha256(run / "PRE-RUN-FILE-LAYOUT.json"):
        problems.append("planned file layout hash binding mismatch")
    expected = {f"{fam}-m{m}-{target}" for fam in ("koblitz", "binary", "prime")
                for m in range(2, 10) for target in ("natural", "planted")}
    if {x["id"] for x in checks} != expected:
        problems.append("source index case cross mismatch")
    rows = []
    for source in checks:
        case_id = source["id"]
        case_dir = run / "cases" / case_id
        input_path = case_dir / "input/inputs.json"
        input_row = load(input_path)[0] if input_path.is_file() else None
        system = source_root / case_id / "system.sing"
        attempt_path = case_dir / "attempt.json"
        cert = case_dir / "output" / f"{case_id}.json"
        readback = case_dir / "s3-readback.json"
        if input_row is None or input_row["id"] != case_id:
            problems.append(f"single-case input missing or misbound: {case_id}")
            continue
        actual_system_sha = sha256(system) if system.is_file() else None
        if actual_system_sha != source["system_sha256"] or actual_system_sha != input_row["source_sha256"]:
            problems.append(f"source system hash mismatch: {case_id}")
        if not attempt_path.is_file() or not cert.is_file() or not readback.is_file():
            problems.append(f"attempt/certificate/readback missing: {case_id}")
            continue
        attempt = load(attempt_path)
        certificate = load(cert)
        cert_sha = sha256(cert)
        if attempt.get("returncode") != 0 or attempt.get("status") != "completed":
            problems.append(f"producer failed: {case_id}")
        if attempt.get("system_sha256") != actual_system_sha or attempt.get("certificate_sha256") != cert_sha:
            problems.append(f"attempt source/certificate hash mismatch: {case_id}")
        if sha256(readback) != cert_sha or attempt.get("s3_readback_sha256") != cert_sha:
            problems.append(f"S3 readback hash mismatch: {case_id}")
        if certificate.get("system_sha256") != actual_system_sha or certificate.get("input", {}).get("id") != case_id:
            problems.append(f"certificate source binding mismatch: {case_id}")
        replay = attempt.get("replay", {})
        if replay.get("returncode") != 0 or not replay.get("verified"):
            problems.append(f"fresh-process replay failed: {case_id}")
        if certificate.get("first_fall_degree") != attempt.get("first_fall_degree") or certificate.get("first_fall_lower_bound") != attempt.get("first_fall_lower_bound"):
            problems.append(f"certificate/attempt outcome mismatch: {case_id}")
        rows.append({"id": case_id, "family": source["family"], "m": source["m"],
                     "target_kind": source["target_kind"], "status": attempt.get("status"),
                     "first_fall_degree": certificate.get("first_fall_degree"),
                     "first_fall_lower_bound": certificate.get("first_fall_lower_bound"),
                     "source_sha256": actual_system_sha, "certificate_sha256": cert_sha,
                     "s3_readback_sha256": sha256(readback), "fresh_replay_verified": bool(replay.get("verified"))})
    family_counts = {}
    for family in ("koblitz", "binary", "prime"):
        selected = [r for r in rows if r["family"] == family]
        family_counts[family] = {
            "rows": len(selected),
            "first_fall_outcomes": [
                {"degree": degree, "lower_bound": lower, "count": count}
                for (degree, lower), count in sorted(Counter((r["first_fall_degree"], r["first_fall_lower_bound"]) for r in selected).items(), key=str)
            ],
            "exact_lfd_rows": 0,
        }
    custody = load(run / "custody.json")
    if custody.get("certificate_readbacks_verified") != 48 or custody.get("attempts") != 48:
        problems.append("producer custody count mismatch")
    result = {
        "schema": "n83.spike29-first-fall-independent-validation/v1",
        "validation_scope": "Recompute the 48 saved spike29 first-fall records, source and certificate hashes, fresh-process replay receipts, and per-certificate S3 readback hashes.",
        "validator_filed_after_data": True,
        "preflight_sha256": sha256(run / "preflight.json"),
        "validator_sha256": sha256(Path(__file__).resolve()),
        "producer_summary_sha256": sha256(run / "summary.json"),
        "source_index_sha256": sha256(run / "source-index.json"),
        "systems_checked": len(rows),
        "certificates_checked": sum(bool(r.get("certificate_sha256")) for r in rows),
        "fresh_replays_verified": sum(r["fresh_replay_verified"] for r in rows),
        "s3_certificate_readbacks_verified": sum(r.get("certificate_sha256") == r.get("s3_readback_sha256") for r in rows),
        "status": "pass" if not problems and len(rows) == 48 else "fail",
        "problems": problems,
        "by_family": family_counts,
        "rows": sorted(rows, key=lambda r: r["id"]),
        "claim_boundary": "First-fall certificates and prime lower bounds only. No exact LFD, affine PDP degree of regularity, solving degree, or factor-base ranking is derived.",
    }
    return result


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("run_dir", type=Path)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()
    result = verify(args.run_dir)
    output = args.out or args.run_dir / "independent-validation.json"
    output.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    print(json.dumps({"status": result["status"], "systems": result["systems_checked"],
                      "certificates": result["certificates_checked"],
                      "replays": result["fresh_replays_verified"],
                      "s3_readbacks": result["s3_certificate_readbacks_verified"],
                      "problems": result["problems"]}, separators=(",", ":")))
    return 0 if result["status"] == "pass" else 1


if __name__ == "__main__":
    raise SystemExit(main())
