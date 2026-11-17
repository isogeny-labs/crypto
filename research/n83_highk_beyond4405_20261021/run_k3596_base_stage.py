#!/usr/bin/env python3
"""Rebuild the root4 frontier base with the fixed31 comparison source."""

from research.n83_highk_beyond4405_20261021 import run_k1107_base_stage as base


def main() -> int:
    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument("--binary", type=base.Path, required=True)
    parser.add_argument("--output", type=base.Path, required=True)
    args = parser.parse_args()
    return base.run(
        args.binary, args.output, k=3596,
        expected_base_hash="9bdadb868c18ccaad3446a62316356e3fe21ecc76512c0c27ccc1fa24d661bc1",
    )


if __name__ == "__main__":
    raise SystemExit(main())
