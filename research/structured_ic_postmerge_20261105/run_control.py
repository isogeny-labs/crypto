#!/usr/bin/env python3
"""Retain one bounded native/software control's command, status and raw output."""
from datetime import datetime, timezone
from pathlib import Path
import hashlib
import json
import os
import subprocess
import sys

out = Path(os.environ.get("STRUCTURED_IC_AUDIT_OUTPUT", str(Path(__file__).resolve().parent))) / "checks"
out.mkdir(parents=True, exist_ok=True)
name, cwd, *command = sys.argv[1:]
if not name.replace("-", "").isalnum() or not command:
    raise SystemExit("usage: run_control.py NAME CWD COMMAND [ARG ...]")
started = datetime.now(timezone.utc).isoformat()
log = out / (name + ".txt")
if log.exists() or log.with_suffix(".txt.gz").exists() or (out / (name + ".json")).exists():
    raise SystemExit("control already recorded; choose a fresh output directory/name")
with log.open("wb") as stream:
    result = subprocess.run(command, cwd=cwd, stdout=stream, stderr=subprocess.STDOUT)
record = {
    "standing": "producer_assertion",
    "name": name,
    "cwd": cwd,
    "command": command,
    "started_at": started,
    "completed_at": datetime.now(timezone.utc).isoformat(),
    "exit_code": result.returncode,
    "raw": log.name,
    "raw_sha256": hashlib.sha256(log.read_bytes()).hexdigest(),
}
(out / (name + ".json")).write_text(json.dumps(record, indent=2) + "\n")
print(json.dumps(record))
raise SystemExit(result.returncode)
