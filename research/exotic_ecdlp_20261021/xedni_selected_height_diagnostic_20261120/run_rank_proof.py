#!/usr/bin/env python3
"""Capped proof-enabled rank calls; preserve every completed and censored arm."""

import json
import os
import signal
import subprocess
import time
from pathlib import Path


ROOT = Path(__file__).resolve().parent
SAGE = "/Volumes/SSD990/cryptanalysis/sage"
TIMEOUT_SECONDS = 90
ALGORITHMS = ("pari", "mwrank_lib")


def run_one(curve_id, algorithm):
    cmd = [SAGE, "-python", str(ROOT / "rank_proof_sage.py"), curve_id, algorithm]
    start = time.monotonic()
    process = subprocess.Popen(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                               text=True, start_new_session=True)
    try:
        stdout, stderr = process.communicate(timeout=TIMEOUT_SECONDS)
        result = json.loads(stdout) if process.returncode == 0 else None
        return {"id": curve_id, "algorithm": algorithm,
                "status": "complete" if result else "error", "result": result,
                "exit_code": process.returncode, "stderr_tail": stderr[-2000:],
                "elapsed_s": round(time.monotonic()-start, 3)}
    except subprocess.TimeoutExpired:
        os.killpg(process.pid, signal.SIGTERM)
        try:
            stdout, stderr = process.communicate(timeout=5)
        except subprocess.TimeoutExpired:
            os.killpg(process.pid, signal.SIGKILL)
            stdout, stderr = process.communicate()
        return {"id": curve_id, "algorithm": algorithm, "status": "timeout",
                "result": None, "exit_code": process.returncode,
                "stderr_tail": stderr[-2000:],
                "elapsed_s": round(time.monotonic()-start, 3)}


def main():
    fixtures = json.loads((ROOT / "fixtures.json").read_text())
    rows = [run_one(item["id"], algorithm)
            for item in fixtures["curves"] for algorithm in ALGORITHMS]
    print(json.dumps({"schema": "xedni-selected-rank-proof-v1",
                      "timeout_seconds_per_arm": TIMEOUT_SECONDS,
                      "rows": rows}, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
