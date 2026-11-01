#!/usr/bin/env python3
"""Bind native coverage rows to the frozen experiment export contract.

The official exporter is supplied explicitly so its version can be pinned by
the caller.  This adapter copies only observed, independently verified native
coverage rows; it never synthesizes a missing measurement.
"""
import argparse
import importlib.util
import json
from pathlib import Path


def load_official(path):
    spec = importlib.util.spec_from_file_location("official_experiment_protocol", path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def rows_from_coverage(path):
    seen = set()
    rows = []
    for line in path.read_text().splitlines():
        if not line.strip():
            continue
        c = json.loads(line)
        axes = c["actual_axes"]
        assert axes["m"] == 2 and axes["solver"] == "enumerate"
        assert 26 <= axes["n"] <= 31
        assert axes["family"] in ("koblitz", "binary", "prime")
        instance = 2 * (axes["n"] - 26) + int(axes["twist"])
        key = (axes["family"], instance)
        if key in seen:
            raise ValueError(f"duplicate execution cell: {key}")
        seen.add(key)
        verified = (c["status"] == "verified" and c["artifact_hashes_verified"]
                    and c["recorded_labels"]["known_answer_verified"])
        rows.append({"run_id": c["attempt"].replace("/", "-"),
                     "arm": "native-enumerate-" + axes["family"],
                     "instance_id": str(instance), "seed": 20261101,
                     "replicate": 0, "warmup": False,
                     "status": "succeeded" if verified else "failed",
                     "verification": "verified" if verified else "not_run",
                     "tier": "none", "replay_pass": int(verified),
                     "native_status": c["status"],
                     "cell_index": c["cell_index"],
                     "manifest_sha256": c["manifest"]["sha256"],
                     "artifact_hashes_verified": c["artifact_hashes_verified"],
                     "known_answer_verified": c["recorded_labels"]["known_answer_verified"]})
    if len(rows) != 36 or len(seen) != 36:
        raise ValueError(f"expected 36 distinct observed native rows, found {len(rows)}")
    return sorted(rows, key=lambda r: (r["arm"], int(r["instance_id"])))


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--coverage", type=Path, required=True)
    ap.add_argument("--protocol", type=Path, required=True)
    ap.add_argument("--official-tool", type=Path, required=True)
    ap.add_argument("--out", type=Path, required=True)
    args = ap.parse_args()
    official = load_official(args.official_tool)
    protocol = json.loads(args.protocol.read_text())
    if protocol["freeze"]["status"] != "frozen":
        raise ValueError("protocol must be frozen before export")
    args.out.mkdir(parents=True, exist_ok=True)
    rows_path = args.out / "experiment-rows.jsonl"
    rows_path.write_text("".join(json.dumps(r, sort_keys=True) + "\n" for r in rows_from_coverage(args.coverage)))
    export = official.from_rows(rows_path, protocol, str(args.protocol),
                                "ML-N26-N31-CROSS-FAMILY-20261101", {}, "")
    # The generic official from-rows exporter leaves arm provenance null.
    # Bind only the values already frozen in each protocol arm.
    pinned = {a["id"]: a["instrument"] for a in protocol["arms"]}
    for arm in export["arms"]:
        arm["binary_sha256"] = pinned[arm["id"]]["binary_sha256"]
        arm["commit"] = pinned[arm["id"]]["commit"]
    export_path = args.out / "experiment-export.json"
    export_path.write_text(json.dumps(export, indent=2, sort_keys=True) + "\n")
    findings = official.lint_export(export, protocol, args.out, verify_files=True)
    if findings:
        raise ValueError("official export check failed: " + "; ".join(findings))
    print(json.dumps({"rows": len(export["runs"]), "export": str(export_path), "official_findings": 0}))


if __name__ == "__main__":
    main()
