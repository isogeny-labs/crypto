#!/usr/bin/env python3
"""Submit the bounded c-parameter scan and retain exact input/response."""
import hashlib
import json
import ssl
import urllib.request
from pathlib import Path

import certifi

ROOT = Path(__file__).resolve().parent
SOURCE = ROOT / "scan.magma"
URL = "https://calculator.magma-maths.org/execute"
request = urllib.request.Request(URL, json.dumps({"code": SOURCE.read_text()}).encode(),
                                 {"Content-Type": "application/json"}, method="POST")
try:
    with urllib.request.urlopen(request, timeout=120,
                                context=ssl.create_default_context(cafile=certifi.where())) as response:
        status = response.status
        result = json.loads(response.read())
except Exception as exc:
    status = None
    result = {"transport_error": repr(exc)}
receipt = {"endpoint": URL, "input_sha256": hashlib.sha256(SOURCE.read_bytes()).hexdigest(),
           "http_status": status, "response": result}
(ROOT / "scan_receipt.json").write_text(json.dumps(receipt, indent=2) + "\n")
stdout = result.get("stdout", "")
print(json.dumps({"http_status": status, "success": result.get("success"),
                  "exit_code": result.get("exit_code"), "rows": len(stdout.splitlines()),
                  "stdout_tail": stdout.splitlines()[-8:]}, indent=2))
