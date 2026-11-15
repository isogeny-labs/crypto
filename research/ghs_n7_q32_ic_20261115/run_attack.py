#!/usr/bin/env python3
"""Submit one immutable public-Magma q32 attack attempt and preserve its response."""

import hashlib
import json
import ssl
import sys
import time
import urllib.request
from pathlib import Path

import certifi

ROOT = Path(__file__).resolve().parent
SOURCE = ROOT / "attack.magma"
URL = "https://calculator.magma-maths.org/execute"


def main() -> None:
    if len(sys.argv) != 2 or not sys.argv[1].replace("-", "").isalnum():
        raise SystemExit("usage: run_attack.py ATTEMPT_TAG")
    tag = sys.argv[1]
    output = ROOT / "receipts" / f"{tag}.json"
    if output.exists():
        raise SystemExit(f"refusing to overwrite {output}")
    source = SOURCE.read_bytes()
    request = urllib.request.Request(
        URL,
        json.dumps({"code": source.decode()}).encode(),
        {"Content-Type": "application/json"},
        method="POST",
    )
    started = time.time()
    try:
        with urllib.request.urlopen(
            request, timeout=180, context=ssl.create_default_context(cafile=certifi.where())
        ) as response:
            status = response.status
            result = json.loads(response.read())
    except Exception as exc:
        status = None
        result = {"transport_error": repr(exc)}
    receipt = {
        "endpoint": URL,
        "input_sha256": hashlib.sha256(source).hexdigest(),
        "http_status": status,
        "client_wall_sec": time.time() - started,
        "response": result,
    }
    output.parent.mkdir(exist_ok=True)
    output.write_text(json.dumps(receipt, indent=2) + "\n")
    print(json.dumps({"receipt": str(output), "http_status": status, "response": result}, indent=2))


if __name__ == "__main__":
    main()
