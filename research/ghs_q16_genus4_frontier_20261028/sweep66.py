#!/usr/bin/env python3
"""Exhaust Magma's GHS parameter on every q16 frontier source model."""

import argparse
import hashlib
import json
import ssl
import urllib.request
from pathlib import Path

import certifi

HERE = Path(__file__).resolve().parent
FRONTIER = HERE.parent / "ghs_nonsplit_order4222_20261028" / "frontier_q16.json"


def bits(n):
    return "+".join(
        "1" if i == 0 else "w" if i == 1 else f"w^{i}"
        for i in range(12) if n >> i & 1
    ) or "0"


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--order", type=int, choices=[3974, 3978, 4178], required=True)
    args = ap.parse_args()
    source = json.loads(FRONTIER.read_text())
    row = next(x for x in source["candidates"] if x["source_order"] == args.order)
    masks = row["all_source_b_masks"]
    code = """SetSeed(20261028); F2:=GF(2); P<X>:=PolynomialRing(F2);
K<w>:=ext<F2|X^12+X^7+X^6+X^5+X^3+X+1>;
k:=FiniteField(2,4); Embed(k,K);
"""
    for b in masks:
        code += f"""E:=EllipticCurve([K|1,w^5,0,0,{bits(b)}]);
assert #E eq {args.order};
counts:=[Integers()|0:i in [0..99]]; errors:=0;
for i in [0..4094] do
  try
    g:=WeilDescentGenus(E,k,w^i);
    assert g le 99;
    counts[g+1]+:=1;
  catch e
    errors+:=1;
  end try;
end for;
assert &+counts+errors eq 4095;
print "B",{b},"GENUS_HIST",[<g,counts[g+1]>:g in [0..99]|counts[g+1] ne 0],"ERRORS",errors;
"""
    script = HERE / f"sweep_{args.order}.magma"
    script.write_text(code)
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
        "schema": "ghs-q16-frontier-all-models-all-parameters-v1",
        "frontier_sha256": hashlib.sha256(FRONTIER.read_bytes()).hexdigest(),
        "input_sha256": hashlib.sha256(code.encode()).hexdigest(),
        "http_status": status,
        "response": payload,
    }
    (HERE / f"sweep_{args.order}_receipt.json").write_text(
        json.dumps(receipt, indent=2) + "\n"
    )
    print(json.dumps({"status": status, "magma": payload.get("magma"),
                      "stdout": payload.get("stdout")}, indent=2))
    assert status == 200 and payload.get("truncated") is False
    assert payload.get("success") is True and payload.get("exit_code") == 0
    lines = payload.get("stdout", "").splitlines()
    assert len(lines) == 2 * len(masks), len(lines)
    for i, b in enumerate(masks):
        joined = " ".join(lines[2*i:2*i+2])
        assert joined == (f"B {b} GENUS_HIST [ <4, 30>, <6, 450>, "
                          "<7, 3585> ] ERRORS 30"), joined


if __name__ == "__main__":
    main()
