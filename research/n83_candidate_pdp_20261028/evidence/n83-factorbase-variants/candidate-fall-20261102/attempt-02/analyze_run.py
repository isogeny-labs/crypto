#!/usr/bin/env python3
"""Independently recompute the scoped candidate first-fall panel checks."""
import argparse
import hashlib
import json
from pathlib import Path

PROTOCOL_ID = "N83-FACTOR-BASE-PDP-FFD-LFD-20261031-V2"
PROTOCOL_DIGEST = "50cd6f8213789de2dfd54a3b33a067ed87cc0def761b7c98769be3e19aee47a9"


def sha256(path):
    h = hashlib.sha256()
    with Path(path).open("rb") as src:
        for block in iter(lambda: src.read(8 * 1024 * 1024), b""):
            h.update(block)
    return h.hexdigest()


def analyze(root):
    root = Path(root)
    preflight = json.loads((root / "preflight.json").read_text())
    producer_summary = json.loads((root / "summary.json").read_text())
    assert preflight["protocol_id"] == producer_summary["protocol_id"] == PROTOCOL_ID
    assert preflight["protocol_freeze_sha256"] == producer_summary["protocol_freeze_sha256"] == PROTOCOL_DIGEST
    expected = preflight.get("systems_planned", preflight.get("systems_planned_this_continuation"))
    raw = []
    problems = []
    for candidate in preflight["candidate_ids"]:
        for source in [r for r in json.loads((root / "source-index.json").read_text()) if r["candidate_id"] == candidate]:
            case_dir = root / "cases" / candidate / source["id"]
            attempt_path = case_dir / "attempt.json"
            if not attempt_path.is_file():
                problems.append(f"missing attempt: {candidate}/{source['id']}")
                continue
            attempt = json.loads(attempt_path.read_text())
            if attempt["system_sha256"] != source["source_sha256"] or attempt["system_bytes"] != source["source_bytes"]:
                problems.append(f"source mismatch: {candidate}/{source['id']}")
            cert = case_dir / "output" / f"{source['id']}.json"
            if attempt.get("certificate_sha256"):
                if not cert.is_file() or sha256(cert) != attempt["certificate_sha256"]:
                    problems.append(f"certificate hash mismatch: {candidate}/{source['id']}")
                else:
                    value = json.loads(cert.read_text())
                    if value.get("system_sha256") != source["source_sha256"]:
                        problems.append(f"certificate source mismatch: {candidate}/{source['id']}")
            if attempt.get("status") == "completed" and not attempt.get("replay", {}).get("verified"):
                problems.append(f"completed without fresh replay: {candidate}/{source['id']}")
            raw.append(attempt)
    status_counts = {s: sum(r["status"] == s for r in raw) for s in sorted({r["status"] for r in raw})}
    completed = sum(bool(r.get("certificate_sha256")) for r in raw)
    replayed = sum(bool(r.get("replay", {}).get("verified")) for r in raw)
    result = {
        "schema": "n83.candidate-fall-independent-validation/v1",
        "protocol_id": PROTOCOL_ID,
        "protocol_freeze_sha256": PROTOCOL_DIGEST,
        "producer_summary_sha256": sha256(root / "summary.json"),
        "systems_expected": expected,
        "systems_attempted": len(raw),
        "certificates_hashed_and_bound": completed,
        "fresh_replays_verified": replayed,
        "status_counts_recomputed": status_counts,
        "source_index_rows": len(json.loads((root / "source-index.json").read_text())),
        "problems": problems,
        "checks_pass": len(raw) == expected and completed == producer_summary["certificates_produced"] and replayed == producer_summary["fresh_replays_verified"] and not problems,
        "claim_scope": "Checks the 144 spike35--spike37 first-fall records only; does not verify LFD, affine degree of regularity, solving degree or factor-base ranking.",
    }
    return result


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("run_dir", type=Path)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()
    result = analyze(args.run_dir)
    output = args.out or args.run_dir / "independent-validation.json"
    output.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    print(json.dumps({"checks_pass": result["checks_pass"], "attempted": result["systems_attempted"], "certificates": result["certificates_hashed_and_bound"], "replayed": result["fresh_replays_verified"], "problems": result["problems"]}, separators=(",", ":")))
    if not result["checks_pass"]:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
