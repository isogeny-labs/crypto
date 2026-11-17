#!/usr/bin/env python3
"""Check all genus-four GHS parameters for rational split points on the order-17 subgroup."""

import hashlib
import json
import ssl
import urllib.request
from pathlib import Path

import certifi

HERE = Path(__file__).resolve().parent
FRONTIER = HERE.parent / "ghs_nonsplit_order4222_20261028" / "frontier_q16.json"
row = next(x for x in json.loads(FRONTIER.read_text())["candidates"]
           if x["source_order"] == 3978)


def bits(n):
    return "+".join(
        "1" if i == 0 else "w" if i == 1 else f"w^{i}"
        for i in range(12) if n >> i & 1
    ) or "0"


code = f"""SetSeed(20261028); F2:=GF(2); P<X>:=PolynomialRing(F2);
K<w>:=ext<F2|X^12+X^7+X^6+X^5+X^3+X+1>;
k:=FiniteField(2,4); Embed(k,K);
E:=EllipticCurve([K|1,w^5,0,0,{bits(row['source_b'])}]);
G:=E![{bits(row['G'][0])},{bits(row['G'][1])}];
assert #E eq 3978 and G ne E!0 and 17*G eq E!0;
g4:=0; g6:=0; g7:=0; errors:=0;
for i in [0..4094] do
  try
    genus:=WeilDescentGenus(E,k,w^i);
    if genus eq 4 then
      C,h:=WeilDescent(E,k,w^i); D:=h(G);
      assert D ne Jacobian(C)!0 and 17*D eq Jacobian(C)!0;
      split_count:=0;
      for j in [1..16] do
        u:=(j*D)[1]; factors:=Factorization(u);
        if Degree(u) ge 3 and
           &and[Degree(t[1]) eq 1 and t[2] eq 1:t in factors] then
          split_count+:=1;
        end if;
      end for;
      assert split_count eq 0;
      g4+:=1;
    elif genus eq 6 then g6+:=1;
    elif genus eq 7 then g7+:=1;
    else assert false;
    end if;
  catch e
    errors+:=1;
  end try;
end for;
assert g4 eq 30 and g6 eq 450 and g7 eq 3585 and errors eq 30;
print "GENUS4_PARAMS",g4,"ALL_ZERO_SPLIT",true,"GENUS6",g6,"GENUS7",g7,
      "REJECTIONS",errors;
"""
(HERE / "smooth17.magma").write_text(code)
req = urllib.request.Request(
    "https://calculator.magma-maths.org/execute",
    json.dumps({"code": code}).encode(),
    {"Content-Type": "application/json"}, method="POST",
)
try:
    with urllib.request.urlopen(
        req, timeout=180,
        context=ssl.create_default_context(cafile=certifi.where()),
    ) as response:
        payload, status = json.load(response), response.status
except Exception as exc:
    payload, status = {"error": repr(exc)}, None
receipt = {
    "schema": "ghs-q16-order17-all-g4-split-obstruction-v1",
    "frontier_sha256": hashlib.sha256(FRONTIER.read_bytes()).hexdigest(),
    "input_sha256": hashlib.sha256(code.encode()).hexdigest(),
    "http_status": status,
    "response": payload,
}
(HERE / "smooth17_receipt.json").write_text(json.dumps(receipt, indent=2) + "\n")
print(json.dumps({"status": status, "magma": payload.get("magma"),
                  "stdout": payload.get("stdout")}, indent=2))
assert status == 200 and payload.get("success") is True
assert payload.get("exit_code") == 0 and payload.get("truncated") is False
assert "GENUS4_PARAMS 30 ALL_ZERO_SPLIT true" in payload.get("stdout", "")
