"""Capped proof-enabled PARI rank with increased frozen effort."""

import hashlib
import json
import os
import signal
import subprocess
import time
from pathlib import Path

HERE = Path(__file__).resolve().parent
SAGE = "/Volumes/SSD990/cryptanalysis/sage"


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    protocol_path = HERE / "rank-pari-effort-protocol.json"
    protocol = json.loads(protocol_path.read_text())
    cmd = [SAGE, "-python", str(HERE / "rank_pari_effort_sage.py")]
    start = time.monotonic()
    process = subprocess.Popen(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                               text=True, start_new_session=True)
    try:
        stdout, stderr = process.communicate(timeout=protocol["timeout_seconds"])
        result = json.loads(stdout) if process.returncode == 0 else None
        status = "complete" if result else "error"
    except subprocess.TimeoutExpired:
        os.killpg(process.pid, signal.SIGTERM)
        try:
            stdout, stderr = process.communicate(timeout=5)
        except subprocess.TimeoutExpired:
            os.killpg(process.pid, signal.SIGKILL)
            stdout, stderr = process.communicate()
        result, status = None, "timeout"
    print(json.dumps({"schema": "xedni-crt-rank-pari-effort-result-v1", "status": status,
                      "protocol_sha256": sha(protocol_path),
                      "source_sha256": sha(HERE / "rank_pari_effort_sage.py"),
                      "runtime_sha256": sha(HERE / "sage-runtime-info.json"),
                      "result": result, "exit_code": process.returncode,
                      "elapsed_s": round(time.monotonic()-start, 3),
                      "stderr_tail": stderr[-2000:]}, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
