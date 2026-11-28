#!/usr/bin/env python3
"""Cap the C2 higher-effort rank check and preserve its status."""

import json
import os
import signal
import subprocess
import time
from pathlib import Path


ROOT = Path(__file__).resolve().parent
SAGE = "/Volumes/SSD990/cryptanalysis/sage"
TIMEOUT_SECONDS = 90


def main():
    cmd = [SAGE, "-python", str(ROOT / "rank_c2_pari_effort_sage.py")]
    start = time.monotonic()
    process = subprocess.Popen(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                               text=True, start_new_session=True)
    try:
        stdout, stderr = process.communicate(timeout=TIMEOUT_SECONDS)
        result = json.loads(stdout) if process.returncode == 0 else None
        row = {"id": "C2", "status": "complete" if result else "error",
               "result": result, "exit_code": process.returncode,
               "stderr_tail": stderr[-2000:],
               "elapsed_s": round(time.monotonic()-start, 3)}
    except subprocess.TimeoutExpired:
        os.killpg(process.pid, signal.SIGTERM)
        try:
            stdout, stderr = process.communicate(timeout=5)
        except subprocess.TimeoutExpired:
            os.killpg(process.pid, signal.SIGKILL)
            stdout, stderr = process.communicate()
        row = {"id": "C2", "status": "timeout", "result": None,
               "exit_code": process.returncode, "stderr_tail": stderr[-2000:],
               "elapsed_s": round(time.monotonic()-start, 3)}
    print(json.dumps({"schema": "xedni-selected-c2-pari-effort-v1",
                      "timeout_seconds": TIMEOUT_SECONDS, "row": row},
                     indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
