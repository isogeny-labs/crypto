#!/usr/bin/env python3
"""Capped Sage rank subprocesses with explicit censored and error rows."""

import json
import os
import signal
import subprocess
import time
from pathlib import Path


ROOT = Path(__file__).resolve().parent
SAGE = "/Volumes/SSD990/cryptanalysis/sage"
TIMEOUT_SECONDS = 90


def run_one(curve_id):
    cmd = [SAGE, "-python", str(ROOT / "rank_sage.py"), curve_id]
    start = time.monotonic()
    process = subprocess.Popen(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                               text=True, start_new_session=True)
    try:
        stdout, stderr = process.communicate(timeout=TIMEOUT_SECONDS)
        result = json.loads(stdout) if process.returncode == 0 else None
        return {"id": curve_id, "status": "complete" if result else "error",
                "result": result, "exit_code": process.returncode,
                "stderr_tail": stderr[-2000:], "elapsed_s": round(time.monotonic()-start, 3)}
    except subprocess.TimeoutExpired:
        os.killpg(process.pid, signal.SIGTERM)
        try:
            stdout, stderr = process.communicate(timeout=5)
        except subprocess.TimeoutExpired:
            os.killpg(process.pid, signal.SIGKILL)
            stdout, stderr = process.communicate()
        return {"id": curve_id, "status": "timeout", "result": None,
                "exit_code": process.returncode, "stderr_tail": stderr[-2000:],
                "elapsed_s": round(time.monotonic()-start, 3)}


def main():
    fixtures = json.loads((ROOT / "fixtures.json").read_text())
    rows = [run_one(item["id"]) for item in fixtures["curves"]]
    print(json.dumps({"schema": "xedni-selected-rank-bounds-v1",
                      "timeout_seconds_per_curve": TIMEOUT_SECONDS,
                      "rows": rows}, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
