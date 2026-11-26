"""Measure relation-row diversity on the fixed M31 six-summand toy panel."""

import argparse
import json
import os
from pathlib import Path
import statistics
import subprocess
import time
from contextlib import redirect_stdout
from io import StringIO

from verify_chart_export import verify


P = 2_147_483_647


def rank(rows):
    pivots = {}
    for original in rows:
        row = [value % P for value in original]
        while any(row):
            lead = next(i for i, value in enumerate(row) if value)
            if lead not in pivots:
                inverse = pow(row[lead], -1, P)
                pivots[lead] = [(value * inverse) % P for value in row]
                break
            pivot = pivots[lead]
            factor = row[lead]
            row = [(value - factor * basis) % P for value, basis in zip(row, pivot)]
    return len(pivots)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("binary", type=Path)
    parser.add_argument("basis_dir", type=Path)
    parser.add_argument("records", type=Path)
    parser.add_argument("--start-seed", type=int, default=11_781_661)
    parser.add_argument("--count", type=int, default=16)
    parser.add_argument("--analyze-only", action="store_true")
    args = parser.parse_args()
    args.basis_dir.mkdir(parents=True, exist_ok=True)
    rows = []
    if args.analyze_only:
        rows = [json.loads(line) for line in args.records.read_text().splitlines()]
    else:
        for offset in range(args.count):
            for kind in ("planted", "unplanted"):
                env = os.environ.copy()
                env["S3_BASIS_DIR"] = str(args.basis_dir.resolve())
                if offset % 2:
                    env["S3_DIRECT_FIRST"] = "1"
                completed = subprocess.run(
                    [str(args.binary.resolve()), "larger-six", str(args.start_seed + offset), kind],
                    env=env,
                    capture_output=True,
                    text=True,
                    check=True,
                )
                row = json.loads(completed.stdout)
                row["offset"] = offset
                rows.append(row)
        args.records.write_text(
            "".join(json.dumps(row, separators=(",", ":")) + "\n" for row in rows)
        )
    for kind in ("planted", "unplanted"):
        sample = [row for row in rows if row["kind"] == kind]
        vectors = [vector for row in sample for vector in row["relation_rows"]]
        started = time.perf_counter()
        incidence_rank = rank(vectors)
        rank_seconds = time.perf_counter() - started
        started = time.perf_counter()
        with redirect_stdout(StringIO()):
            for row in sample:
                verify(row, args.basis_dir)
        basis_check_seconds = time.perf_counter() - started
        print(
            json.dumps(
                {
                    "kind": kind,
                    "targets": len(sample),
                    "hits": sum(bool(row["signed_witnesses"]) for row in sample),
                    "distinct_rows": len(set(map(tuple, vectors))),
                    "incidence_rank_mod_p": incidence_rank,
                    "rank_seconds": rank_seconds,
                    "basis_check_seconds": basis_check_seconds,
                    "total_indexed_seconds": sum(row["indexed_total_seconds"] for row in sample),
                    "total_direct_seconds": sum(row["direct_total_seconds"] for row in sample),
                    "median_indexed_seconds": statistics.median(
                        row["indexed_total_seconds"] for row in sample
                    ),
                },
                separators=(",", ":"),
            )
        )


if __name__ == "__main__":
    main()
