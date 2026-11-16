#!/usr/bin/env python3
"""Build the reviewed N53 producers from the same selected source root as preflight."""
from __future__ import annotations

import argparse
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
    parser.add_argument("--panel", type=Path, required=True)
    args = parser.parse_args()
    repo, source_root = args.repo.resolve(), args.source_root.resolve()
    selection = verify(repo, source_root, args.report)
    bundle = source_root / "research/sat_factor_base_review_20260908/autolab_n53_native_cyclic_l384_20260929"
    sys.path.insert(0, str(bundle))
    build = importlib.import_module("build")
    build.REPO = source_root
    build.HERE = bundle
    build.run(args.panel)
    print(f"historical_build_source_root={source_root} selected_entries={len(selection['selected_sources'])}")


if __name__ == "__main__":
    main()
