#!/usr/bin/env python3
"""Submit one frozen q32 Magma source and retain the immutable public response."""

import hashlib
import json
import ssl
import sys
import time
import urllib.request
from pathlib import Path

import certifi

ROOT = Path(__file__).resolve().parent
URL = "https://calculator.magma-maths.org/execute"


def main():
    if len(sys.argv) != 2 or sys.argv[1] not in {"factor"}:
        raise SystemExit("usage: run_validation.py factor")
    case = sys.argv[1]
    source = (ROOT / f"{case}.magma").read_bytes()
    output = ROOT / "receipts" / "factor1.json"
    if output.exists():
        raise SystemExit(f"refusing to overwrite {output}")
    request = urllib.request.Request(URL, json.dumps({"code": source.decode()}).encode(), {"Content-Type": "application/json"}, method="POST")
    start = time.time()
    try:
        with urllib.request.urlopen(request, timeout=180, context=ssl.create_default_context(cafile=certifi.where())) as response:
            status = response.status
            result = json.loads(response.read())
    except Exception as exc:
        status = None
        result = {"transport_error": repr(exc)}
    receipt = {"case": case, "endpoint": URL, "input_sha256": hashlib.sha256(source).hexdigest(), "http_status": status, "client_wall_sec": time.time()-start, "response": result}
    output.parent.mkdir(exist_ok=True)
    output.write_text(json.dumps(receipt, indent=2)+"\n")
    print(json.dumps(receipt, indent=2))


if __name__ == "__main__":
    main()
