#!/usr/bin/env python3
"""Exhaust all Magma GHS descent parameters on three exact q16 sources."""

import hashlib
import json
import re
import ssl
import urllib.request
from pathlib import Path

import certifi

HERE = Path(__file__).resolve().parent
source = json.loads((HERE / "frontier_q16.json").read_text())
assert [r["source_order"] for r in source["candidates"]] == [3974, 3978, 4178]


def bits(n):
    return "+".join("1" if i == 0 else "w" if i == 1 else f"w^{i}"
                    for i in range(12) if n >> i & 1) or "0"


code = """F2 := GF(2); P<X> := PolynomialRing(F2);
K<w> := ext<F2|X^12+X^7+X^6+X^5+X^3+X+1>;
assert Order(w) eq 4095;
k := FiniteField(2,4); Embed(k,K);
"""
for row in source["candidates"]:
    order = row["source_order"]
    b = row["source_b"]
    code += f"E := EllipticCurve([K|1,({bits(32)}),0,0,({bits(b)})]);\n"
    code += f"assert #E eq {order};\n"
    code += "counts := [Integers()|0:i in [0..99]]; first3 := -1; errors := 0;\n"
    code += """for i in [0..4094] do
    try
        g := WeilDescentGenus(E,k,w^i);
        assert g le 99;
        counts[g+1] +:= 1;
        if g eq 3 and first3 eq -1 then first3 := i; end if;
    catch e
        errors +:= 1;
    end try;
end for;
assert &+counts + errors eq 4095;
"""
    code += (f'print "ORDER",{order},"GENUS_HIST",'
             '[<g,counts[g+1]>:g in [0..99]|counts[g+1] ne 0],'
             '"FIRST3",first3,"ERRORS",errors;\n')

script_path = HERE / "magma_genus_frontier_q16.magma"
script_path.write_text(code)
request = urllib.request.Request(
    "https://calculator.magma-maths.org/execute",
    json.dumps({"code": code}).encode(),
    {"Content-Type": "application/json"}, method="POST")
try:
    with urllib.request.urlopen(
        request, timeout=180,
        context=ssl.create_default_context(cafile=certifi.where())
    ) as response:
        payload = json.loads(response.read())
        http_status = response.status
except Exception as exc:
    payload = {"error": repr(exc)}
    http_status = None

receipt = {
    "schema": "ghs-q16-frontier-complete-magma-genus-sweep-v1",
    "endpoint": "https://calculator.magma-maths.org/execute",
    "input_sha256": hashlib.sha256(code.encode()).hexdigest(),
    "frontier_sha256": hashlib.sha256((HERE / "frontier_q16.json").read_bytes()).hexdigest(),
    "protocol_sha256": hashlib.sha256(
        (HERE / "Q16_FRONTIER_PROTOCOL.md").read_bytes()).hexdigest(),
    "http_status": http_status,
    "response": payload,
}
receipt_path = HERE / "magma_genus_frontier_q16_receipt.json"
receipt_path.write_text(json.dumps(receipt, indent=2) + "\n")
assert http_status == 200 and payload.get("success") is True
assert payload.get("exit_code") == 0 and payload.get("truncated") is False
lines = payload.get("stdout", "").splitlines()
assert len(lines) == 6, lines
pairs = [" ".join(lines[2 * i:2 * i + 2]) for i in range(3)]
for line, row in zip(pairs, source["candidates"]):
    match = re.fullmatch(
        r"ORDER (\d+) GENUS_HIST \[ <4, 30>, <6, 450>, <7, 3585> \] "
        r"FIRST3 -1 ERRORS 30", line)
    assert match and int(match.group(1)) == row["source_order"], line
print(json.dumps({"http_status": http_status, "magma": payload.get("magma"),
                  "stdout": pairs, "receipt": str(receipt_path)}, indent=2))
