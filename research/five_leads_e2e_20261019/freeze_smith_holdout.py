#!/usr/bin/env python3
"""Select exact Smith-negative census identities before any odd-kernel test."""

import csv
import gzip
import hashlib
import json
import sys
from pathlib import Path

DEFAULT = Path("/home/ec2-user/crypto-w3b-smith-isogeny/research/smith_prop24_native_20261014/data/random_p2147483647_rows.csv.gz")
HERE = Path(__file__).resolve().parent


def main():
    source = Path(sys.argv[1]) if len(sys.argv) > 1 else DEFAULT
    sha = hashlib.sha256(source.read_bytes()).hexdigest()
    selected = {"no_tractable": [], "trigonal_but_no_rational_isogeny": []}
    with gzip.open(source, "rt") as f:
        for row in csv.DictReader(f):
            group = (
                "no_tractable" if int(row["n_tractable"]) == 0
                else "trigonal_but_no_rational_isogeny"
                if int(row["n_trig"]) > 0 and int(row["n_iso"]) == 0
                else None
            )
            if group is not None and len(selected[group]) < 10:
                assert row["full_success"] == "0"
                selected[group].append({
                    "index": int(row["index"]), "cycle_type": row["cycle_type"],
                    "n_tractable": int(row["n_tractable"]),
                    "n_trig": int(row["n_trig"]), "n_iso": int(row["n_iso"]),
                    "full_success": False,
                })
            if all(len(v) == 10 for v in selected.values()):
                break
    assert all(len(v) == 10 for v in selected.values())
    output = {
        "schema": "smith-negative-odd-kernel-holdout/v1",
        "p": 2147483647, "source_seed": 20261014,
        "source_generator": "smith_prop24::random_curve(p,seed,index)",
        "source_revision": "31d0beec8384887fdb494569090ca52d95b218ec",
        "source_census_gzip_sha256": sha,
        "selection_rule": "first ten n_tractable=0; first ten n_trig>0 and n_iso=0, in archived CSV order",
        "groups": selected,
        "odd_kernel_screen_executed": False,
    }
    target = HERE / "fixtures/smith-negative-holdout-v1.json"
    target.parent.mkdir(exist_ok=True)
    target.write_text(json.dumps(output, indent=2) + "\n")
    print(target)


if __name__ == "__main__":
    main()
