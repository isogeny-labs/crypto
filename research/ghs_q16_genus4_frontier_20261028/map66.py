#!/usr/bin/env python3
"""Verify a c=1 GHS subgroup homomorphism on every q16 frontier equation."""

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
    r, cofactor = row["subgroup_prime"], row["cofactor"]
    code = """SetSeed(20261028); F2:=GF(2); P<X>:=PolynomialRing(F2);
K<w>:=ext<F2|X^12+X^7+X^6+X^5+X^3+X+1>;
k:=FiniteField(2,4); Embed(k,K);
"""
    for b in masks:
        code += f"""E:=EllipticCurve([K|1,w^5,0,0,{bits(b)}]);
assert #E eq {args.order};
G:=E!0;
for tries in [1..100] do
  G:={cofactor}*Random(E);
  if G ne E!0 then break; end if;
end for;
assert G ne E!0 and {r}*G eq E!0;
Q:=7*G;
assert WeilDescentGenus(E,k,K!1) eq 4;
C,h:=WeilDescent(E,k,K!1); J:=Jacobian(C);
D:=h(G); T:=h(Q);
assert D ne J!0 and {r}*D eq J!0 and T eq 7*D;
print "B",{b},"J_ORDER",#J,"G",G,"D",D,"CHECK",true;
"""
    script = HERE / f"map_{args.order}.magma"
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
        "schema": "ghs-q16-frontier-all-models-explicit-g4-map-v1",
        "frontier_sha256": hashlib.sha256(FRONTIER.read_bytes()).hexdigest(),
        "input_sha256": hashlib.sha256(code.encode()).hexdigest(),
        "http_status": status,
        "response": payload,
    }
    (HERE / f"map_{args.order}_receipt.json").write_text(
        json.dumps(receipt, indent=2) + "\n"
    )
    print(json.dumps({"status": status, "magma": payload.get("magma"),
                      "stdout": payload.get("stdout")}, indent=2))
    assert status == 200 and payload.get("truncated") is False
    assert payload.get("success") is True and payload.get("exit_code") == 0
    out = payload.get("stdout", "")
    assert out.count(" CHECK true") == len(masks)
    for b in masks:
        assert f"B {b} J_ORDER " in out


if __name__ == "__main__":
    main()
