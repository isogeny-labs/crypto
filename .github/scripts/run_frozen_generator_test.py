#!/usr/bin/env python3
"""Run the immutable generator controls with the matching frozen source root."""
from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--generator", type=Path, required=True)
    parser.add_argument("--source-root", type=Path, required=True)
    parser.add_argument("--test-module", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()
    test_dir = args.test_module.resolve().parent
    sys.path.insert(0, str(test_dir))
    import test_generator
    source_root = args.source_root.resolve()
    test_generator.ROOT = source_root
    # test_generator reads the selected fast-arithmetic source from source_root.
    # Its independent verifier retains the checkout root for the frozen archive
    # and verifier inputs; those are separate audit inputs, not producer code.
    result = test_generator.test(args.generator.resolve())
    args.out.write_text(json.dumps(result, sort_keys=True, indent=2) + "\n")
    print(f"frozen_generator_controls={result['status']} pinned_fast_arith_root={test_generator.ROOT}")


if __name__ == "__main__":
    main()
