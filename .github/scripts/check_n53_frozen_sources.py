#!/usr/bin/env python3
"""Run N53 gates against selected historical sources in an isolated root."""
from __future__ import annotations

import argparse
import importlib
import json
import sys
from pathlib import Path

sys.dont_write_bytecode = True

from n53_frozen_root import prepare, verify


def context(repo: Path, source_root: Path, report: Path):
    repo, source_root = repo.resolve(), source_root.resolve()
    if report.is_file():
        selection = verify(repo, source_root, report)
    else:
        selection = prepare(repo, source_root)
        report.parent.mkdir(parents=True, exist_ok=True)
        report.write_text(json.dumps(selection, indent=2, sort_keys=True) + "\n")
    bundle_rel = "research/sat_factor_base_review_20260908/autolab_n53_native_cyclic_l384_20260929"
    bundle = source_root / bundle_rel
    sys.path.insert(0, str(bundle))
    sys.modules.pop("check_protocol", None)
    check = importlib.import_module("check_protocol")
    check.HERE = bundle
    check.REPO = repo
    check.ROOT = source_root / "research/sat_factor_base_review_20260908"
    check.ORBIT = check.ROOT / "autolab_orbit_extract_20260924"
    check.ROTATION = check.ROOT / "autolab_n53_rank_rotation_20260925"
    check.SHARED = check.ROOT / "autolab_shared_log_n53_20260925"
    check.RHO_STUDY = check.ROOT / "autolab_matched_point_rho_n53_20260925"
    check.OLD = check.ROOT / "autolab_combined_l384_certified_coldbase_n53_20260925"
    source_keys = set(json.loads((bundle / "FROZEN.json").read_text())["source_sha256"])
    check.sources = lambda: {row["key"]: source_root / row["path"]
                             for row in selection["selected_sources"] if row["key"] in source_keys}
    return selection, check, bundle


def annotate_dispatch_receipt(panel: Path, selection: dict) -> None:
    receipt_path = panel / "predispatch.json"
    if not receipt_path.is_file():
        return
    receipt = json.loads(receipt_path.read_text())
    receipt["historical_source_selection"] = {
        "schema": selection["schema"], "checkout_head": selection["checkout_head"],
        "source_root": selection["source_root"],
        "current_code_parity_status": selection["current_code_parity_status"],
        "selected_sources": selection["selected_sources"]}
    receipt_path.write_text(json.dumps(receipt, indent=2, sort_keys=True) + "\n")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--repo", type=Path, required=True)
    parser.add_argument("--source-root", type=Path, required=True)
    parser.add_argument("--report", type=Path, required=True)
    parser.add_argument("--require-release", action="store_true")
    parser.add_argument("--dispatch-panel", type=Path)
    args = parser.parse_args()
    selection, check, bundle = context(args.repo, args.source_root, args.report)
    checked = check.preflight(require_release=args.require_release or args.dispatch_panel is not None)
    if args.dispatch_panel is not None:
        gate = importlib.import_module("dispatch_gate")
        try:
            gate.run(args.dispatch_panel)
        except Exception:
            annotate_dispatch_receipt(args.dispatch_panel, selection)
            raise
        annotate_dispatch_receipt(args.dispatch_panel, selection)
        receipt = json.loads((args.dispatch_panel / "predispatch.json").read_text())
        status = receipt["status"]
    else:
        status = "HELD_HASH_INPUT_PREFLIGHT_PASS"
    print(json.dumps({"status": status, "preflight_status": checked["status"],
                      "historical_source_count": sum(not row["key"].startswith("input:") and row["key"] != "build_cargo_lock" for row in selection["selected_sources"]),
                      "current_code_parity_status": selection["current_code_parity_status"],
                      "source_root": selection["source_root"]}, sort_keys=True))


if __name__ == "__main__":
    main()
