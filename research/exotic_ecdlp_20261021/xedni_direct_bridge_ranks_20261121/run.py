"""Run all frozen rank cells through the checked Sage launcher."""

import hashlib
import json
import subprocess
import time
from pathlib import Path

HERE = Path(__file__).resolve().parent
SOURCE = HERE.parent / "xedni_direct_word2_lifts_20261121" / "result.json"
SAGE = "/Volumes/SSD990/cryptanalysis/sage"


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    protocol = json.loads((HERE / "protocol.json").read_text())
    assert sha(SOURCE) == protocol["source_result_sha256"]
    source = json.loads(SOURCE.read_text())
    assert [r["source_index"] for r in source["rows"]] == protocol["source_indices"]
    rows = []
    for index in protocol["source_indices"]:
        for method in protocol["methods_in_order"]:
            start = time.monotonic_ns()
            try:
                cp = subprocess.run([SAGE, "-python", str(HERE / "rank_worker_sage.py"),
                                     str(index), method], capture_output=True, text=True,
                                    timeout=protocol["per_method_wall_cap_seconds"])
                elapsed = time.monotonic_ns()-start
                row = {"source_index": index, "method": method, "elapsed_ns": elapsed,
                       "exit_code": cp.returncode, "stdout": cp.stdout, "stderr": cp.stderr}
                if cp.returncode == 0:
                    try:
                        row["result"] = json.loads(cp.stdout)
                        row["status"] = "complete"
                    except json.JSONDecodeError:
                        row["status"] = "parse_error"
                else:
                    row["status"] = "error"
            except subprocess.TimeoutExpired as ex:
                elapsed = time.monotonic_ns()-start
                row = {"source_index": index, "method": method, "elapsed_ns": elapsed,
                       "status": "wall_cap", "stdout": (ex.stdout or b"").decode(errors="replace") if isinstance(ex.stdout, bytes) else (ex.stdout or ""),
                       "stderr": (ex.stderr or b"").decode(errors="replace") if isinstance(ex.stderr, bytes) else (ex.stderr or "")}
            rows.append(row)
            print(json.dumps(row, sort_keys=True), flush=True)
    (HERE / "summary.json").write_text(json.dumps({"schema": "xedni-direct-bridge-ranks-summary-v1",
        "protocol_sha256": sha(HERE / "protocol.json"), "source_sha256": sha(SOURCE),
        "cells": len(rows), "statuses": {status: sum(r["status"] == status for r in rows)
            for status in sorted({r["status"] for r in rows})}}, indent=2, sort_keys=True)+"\n")


if __name__ == "__main__":
    main()
