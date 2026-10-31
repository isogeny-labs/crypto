#!/usr/bin/env python3
"""Address public proof targets by numeric index across GitHub Actions jobs.

GitHub can mistake a public target path for a secret and suppress the entire
job output. Only indexes cross the job boundary; each worker discovers the
same tracked targets at the same commit before selecting its target.
"""

import argparse
import json
from pathlib import Path
import subprocess


def discover():
    root = Path(__file__).resolve().parents[2]
    output = subprocess.check_output(
        [str(root / "tools/lean/proof-gate.sh"), "--list"], cwd=root, text=True
    )
    return [line for line in output.splitlines() if line]


def address(targets, index=None):
    if len(set(targets)) != len(targets):
        raise ValueError("proof discovery returned duplicate targets")
    if index is None:
        return json.dumps(list(range(len(targets))), separators=(",", ":"))
    if index < 0 or index >= len(targets):
        raise ValueError(f"target index {index} is outside {len(targets)} discovered targets")
    return targets[index]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("mode", choices=("matrix", "select"))
    parser.add_argument("index", type=int, nargs="?")
    args = parser.parse_args()
    if (args.mode == "select") != (args.index is not None):
        parser.error("select requires an index; matrix takes no index")
    try:
        print(address(discover(), args.index))
    except (ValueError, subprocess.CalledProcessError) as exc:
        parser.exit(1, f"proof target discovery failed: {exc}\n")


if __name__ == "__main__":
    main()
