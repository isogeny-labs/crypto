#!/usr/bin/env python3
"""Run the labeled N53 outcome with preflight, sources and binaries from one frozen root."""
from __future__ import annotations

import argparse
import json
import importlib
from pathlib import Path
import sys

sys.dont_write_bytecode = True

from n53_frozen_root import verify


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--repo", type=Path, required=True)
    parser.add_argument("--source-root", type=Path, required=True)
    parser.add_argument("--report", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()
    repo, source_root = args.repo.resolve(), args.source_root.resolve()
    selection = verify(repo, source_root, args.report)
    bundle = source_root / "research/sat_factor_base_review_20260908/autolab_n53_native_cyclic_l384_20260929"
    sys.path.insert(0, str(bundle))
    check = importlib.import_module("check_protocol")
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
    runner = importlib.import_module("run_panel")
    runner.REPO = repo
    runner.IC_SOURCE = source_root / "examples/koblitz_s5_sat_instance.rs"
    runner.RHO_SOURCE = source_root / "examples/koblitz_rho_batch_ks.rs"
    runner.IC_EXE = source_root / "target/release/examples/koblitz_s5_sat_instance"
    runner.RHO_EXE = source_root / "target/release/examples/koblitz_rho_batch_ks"
    runner.POINTS = bundle / "points_L384.jsonl"
    sys.argv = [str(bundle / "run_panel.py"), "--out", str(args.out)]
    runner.main()
    print(f"historical_run_source_root={source_root} selected_entries={len(selection['selected_sources'])} current_code_parity=not_assessed")


if __name__ == "__main__":
    main()
