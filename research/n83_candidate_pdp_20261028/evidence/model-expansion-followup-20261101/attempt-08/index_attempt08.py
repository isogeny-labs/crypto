#!/usr/bin/env python3
"""Write a path-, role-, hash-, and custody-aware index of attempt-08."""
from __future__ import annotations

import hashlib
import json
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parent
REPO = Path(subprocess.check_output(["git", "-C", str(ROOT), "rev-parse", "--show-toplevel"], text=True).strip())
PREFIX = ROOT.relative_to(REPO).as_posix() + "/"
tracked = set(subprocess.check_output(["git", "-C", str(REPO), "ls-files", "--", PREFIX], text=True).splitlines())
artifact_payloads: dict[str, dict[str, object]] = {}
for manifest in sorted(ROOT.rglob(".artifacts.json")):
    payload = json.loads(manifest.read_text())
    parent = manifest.parent.relative_to(ROOT).as_posix()
    for name, info in payload.get("files", {}).items():
        rel = f"{parent}/{name}" if parent != "." else name
        artifact_payloads[rel] = {
            "sha256": info["sha256"],
            "bytes": info["bytes"],
            "manifest": manifest.relative_to(ROOT).as_posix(),
        }

def role_for(rel: str) -> str:
    parts = rel.split("/")
    if parts[0] == "inputs":
        return "generated-pdp-input" if parts[-1] in {"input.json", "system.sing"} else "input-index-or-provenance"
    if parts[0] == "top":
        if "certificates" in parts and parts[-1].endswith(".json"):
            return "top-form-certificate"
        if parts[-1] == "replay-summary.json":
            return "fresh-process-replay-summary"
        if parts[-1] == "summary.json" or parts[-1] == "top-summary.json":
            return "candidate-certificate-summary"
        return "candidate-run-record-or-log"
    if parts[0] == "model":
        return {"rows.json": "training-rows", "predictions.json": "held-out-predictions", "result.json": "model-result"}.get(parts[-1], "model-output")
    if parts[0] == "base-sources":
        return "factor-base-source-binding"
    if parts[0].startswith("generate-") or parts[0].startswith("status-"):
        return "candidate-generation-log-or-status"
    if parts[0].startswith("preflight") or "preflight" in parts[0]:
        return "frozen-protocol-preflight-or-correction"
    if parts[0] == "__pycache__":
        return "reproducible-python-runtime-cache"
    if parts[-1].endswith(".py"):
        return "runner-source"
    if parts[-1].endswith(".log"):
        return "execution-log"
    return "run-metadata-or-provenance"

files = []
for path in sorted(p for p in ROOT.rglob("*") if p.is_file()):
    rel = path.relative_to(ROOT).as_posix()
    if rel in {"DATASET-INDEX.json", "DATASET-INDEX.sha256"}:
        continue
    data = path.read_bytes()
    digest = hashlib.sha256(data).hexdigest()
    expected = artifact_payloads.get(rel)
    if expected:
        if expected["sha256"] != digest or expected["bytes"] != len(data):
            raise SystemExit(f"artifact manifest mismatch: {rel}")
        custody = "content-addressed-artifact-store; remote verification was run for attempt-08"
    elif PREFIX + rel in tracked:
        custody = "git-tracked"
    else:
        custody = "local-runtime-cache; reproducible and not scientific input/output"
    files.append({
        "path": rel,
        "role": role_for(rel),
        "bytes": len(data),
        "sha256": digest,
        "custody": custody,
        "artifact_manifest": expected["manifest"] if expected else None,
    })

status = json.loads((ROOT / "model/result.json").read_text())
doc = {
    "schema": "n83.model-dataset-file-index/v1",
    "experiment": "model-expansion-followup-20261101-attempt-08",
    "protocol": {
        "path": "research/n83_candidate_pdp_20261028/protocol-model-full-domain-v5.json",
        "sha256": status["protocol_sha256"],
    },
    "scope": {
        "candidate_bases": ["spike27", "spike28", "spike29", "spike30", "spike31", "spike32", "spike33", "spike35", "spike36", "spike37"],
        "families": ["koblitz", "binary", "prime"],
        "m_values": list(range(2, 10)),
        "target_kinds": ["natural", "planted"],
        "systems": status["systems"],
        "new_attempt_08_systems": 144,
        "new_attempt_08_certificates_replayed": 144,
        "finite_interval_midpoint_proxy_rows": status["finite_interval_midpoint_rows"],
        "positive_dimensional_rows_without_numeric_label": status["positive_dimensional_rows"],
    },
    "model_result": {
        "grouped_out_of_base_folds": len(status["folds"]),
        "heldout_base_wins": status["heldout_base_wins"],
        "ridge_mae": status["aggregate_grouped_oof_mae"],
        "matched_configuration_baseline_mae": status["aggregate_matched_configuration_baseline_mae"],
        "admissible_exploratory_lead": status["admissible_exploratory_lead"],
        "label_scope": status["label_scope"],
    },
    "file_count": len(files),
    "file_bytes": sum(item["bytes"] for item in files),
    "custody_counts": {
        custody: sum(item["custody"].startswith(custody) for item in files)
        for custody in ["git-tracked", "content-addressed-artifact-store", "local-runtime-cache"]
    },
    "files": files,
}
out = ROOT / "DATASET-INDEX.json"
out.write_text(json.dumps(doc, indent=2, sort_keys=False) + "\n")
(ROOT / "DATASET-INDEX.sha256").write_text(f"{hashlib.sha256(out.read_bytes()).hexdigest()}  DATASET-INDEX.json\n")
print(json.dumps({"file_count": len(files), "bytes": doc["file_bytes"], "custody_counts": doc["custody_counts"], "index_sha256": hashlib.sha256(out.read_bytes()).hexdigest()}, sort_keys=True))
