#!/usr/bin/env python3
"""Preserve the exact public Magma factorization check response."""
import hashlib
import json
import ssl
import urllib.request
from pathlib import Path

import certifi

ROOT = Path(__file__).resolve().parent
SOURCE = ROOT / "check_factor.magma"
URL = "https://calculator.magma-maths.org/execute"
source = SOURCE.read_bytes()
request = urllib.request.Request(
    URL, json.dumps({"code": source.decode()}).encode(),
    {"Content-Type": "application/json"}, method="POST"
)
try:
    with urllib.request.urlopen(
        request, timeout=120,
        context=ssl.create_default_context(cafile=certifi.where())
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
    "response": result,
}
(ROOT / "factor_receipt.json").write_text(json.dumps(receipt, indent=2) + "\n")
print(json.dumps(receipt, indent=2))
