#!/usr/bin/env python3
"""Run explicit Floyd rho on the same source in the Magma calculator."""
import hashlib
import json
import ssl
import urllib.request
from pathlib import Path

import certifi

HERE = Path(__file__).resolve().parent
URL = "https://calculator.magma-maths.org/execute"
source = json.loads((HERE/"nonsplit_4054.json").read_text())


def z_expr(n):
    return "+".join("1" if i == 0 else "z" if i == 1 else f"z^{i}"
                    for i in range(24) if (n >> i) & 1) or "0"


def k_expr(n):
    return f"K!(L!({z_expr(n)}))"


a, b = source["source_a"], source["source_b"]
gx, gy = source["source_G"]
qx, qy = source["source_Q"]
code = f"""F2 := GF(2);
P<U> := PolynomialRing(F2);
L<z> := ext<F2 | U^24+U^16+U^15+U^14+U^13+U^10+U^9+U^7+U^5+U^3+1>;
K := FiniteField(2,12); Embed(K,L);
E := EllipticCurve([K|1,{k_expr(a)},0,0,{k_expr(b)}]);
G := E![{k_expr(gx)},{k_expr(gy)},1];
Q := E![{k_expr(qx)},{k_expr(qy)},1];
r := 2027;
assert #E eq 4054 and Order(G) eq r and Q eq 7*G;

function Step(st)
    pt := st[1]; aa := st[2]; bb := st[3];
    if pt eq E!0 then
        h := 0;
    else
        bits := Eltseq(K!(pt[1]/pt[3]));
        h := (&+[Integers()!bits[i]*2^(i-1) : i in [1..#bits]]) mod 3;
    end if;
    if h eq 0 then return <pt+G,(aa+1) mod r,bb>; end if;
    if h eq 1 then return <pt+Q,aa,(bb+1) mod r>; end if;
    return <2*pt,(2*aa) mod r,(2*bb) mod r>;
end function;

function Solve(seed)
    aa := (37*seed+11) mod r; bb := (101*seed+5) mod r;
    st := <aa*G+bb*Q,aa,bb>;
    tortoise := Step(st); hare := Step(Step(st)); calls := 3;
    while tortoise[1] ne hare[1] do
        tortoise := Step(tortoise);
        hare := Step(Step(hare));
        calls +:= 3;
        assert calls lt 50*r;
    end while;
    den := (hare[3]-tortoise[3]) mod r;
    if den eq 0 then return false,calls,0; end if;
    ans := Integers()!((Integers(r)!(tortoise[2]-hare[2]))/(Integers(r)!den));
    assert ans*G eq Q and ans eq 7;
    return true,calls,ans;
end function;

tic := Cputime(); calls := []; solved := 0; singular := 0;
for seed in [0..99] do
    ok,n,ans := Solve(seed);
    if ok then Append(~calls,n); solved +:= 1; else singular +:= 1; end if;
end for;
Sort(~calls);
print "RHO_SOLVED", solved, "SINGULAR", singular;
print "RHO_STEP_CALLS_MEDIAN", calls[(#calls+1) div 2];
print "RHO_STEP_CALLS_TOTAL", &+calls;
print "RHO_CPU_SECONDS", Cputime(tic);
"""
path = HERE/"rho_magma_4054.magma"
path.write_text(code)
req = urllib.request.Request(URL,json.dumps({"code":code}).encode(),
                             {"Content-Type":"application/json"},method="POST")
with urllib.request.urlopen(req,timeout=300,
                            context=ssl.create_default_context(cafile=certifi.where())) as resp:
    status = resp.status
    payload = json.loads(resp.read())
receipt = {"schema":"ghs-nonsplit-magma-rho-v1", "endpoint":URL,
           "http_status":status,
           "input_sha256":hashlib.sha256(code.encode()).hexdigest(),
           "response":payload}
(HERE/"rho_magma_4054_receipt.json").write_text(json.dumps(receipt,indent=2)+"\n")
print(json.dumps({"http_status":status,"success":payload.get("success"),
                  "stdout":payload.get("stdout"),"error":payload.get("error"),
                  "magma":payload.get("magma")},indent=2))
