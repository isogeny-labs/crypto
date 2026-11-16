#!/usr/bin/env python3
"""Rebuild split fibers inside Magma, transfer, and solve the target DLP."""
import argparse
import hashlib
import json
import ssl
import urllib.request
from pathlib import Path

import certifi

HERE = Path(__file__).resolve().parent
parser = argparse.ArgumentParser()
parser.add_argument("--benchmark", action="store_true",
                    help="time the same dynamic path without principal-divisor proof checks")
args = parser.parse_args()
source = json.loads((HERE / "source_4222.json").read_text())
canonical = json.loads((HERE / "canonical_4222.json").read_text())


def bits(n):
    return "+".join("1" if i == 0 else "z" if i == 1 else f"z^{i}"
                    for i in range(24) if n >> i & 1) or "0"


def kexpr(n):
    return f"k!(L!({bits(n)}))"


def kbig(n):
    return f"K!(L!({bits(n)}))"


terms = [f"({kexpr(c)})*Xq^{e[0]}*Yq^{e[1]}*Zq^{e[2]}"
         for e,c in zip(canonical["quartic_exponents"],
                        canonical["quartic_coefficients"])]
qexpr = "+".join(terms)
map_expr = ["+".join(f"({kbig(c)})*xx^{e[0]}*uu^{e[1]}"
                     for e,c in zip(exps,coeffs))
            for exps,coeffs in zip(canonical["map_exponents"],
                                   canonical["map_coefficients"])]
quotient = (HERE / "quotient_4222.magma").read_text()
prefix = quotient.split("for n in [1..100] do",1)[0]
assert "function SplitFiber(P)" in prefix
prefix = prefix.replace("K := FiniteField(2,12); Embed(K,L);",
                        "K := FiniteField(2,12); Embed(K,L);\n"
                        "k := FiniteField(2,4); Embed(k,K); Embed(k,L);")
prefix = "\n".join(line for line in prefix.splitlines()
                   if not any(tag in line for tag in
                              ('print "TO_E_EXPONENTS"', 'print "TO_E_COEFF_BITS"',
                               'print "ISO_BITS"'))) + "\n"
map_checks = "" if args.benchmark else """b0 := IsPrincipal(DD); br := IsPrincipal(2111*DD); b7 := IsPrincipal(TT-7*DD);
print "MAP_ORDER_CHECKS", b0, br, b7;
assert not b0 and br and b7;
"""
lift_checks = "" if args.benchmark else """c0 := IsPrincipal(D256); cr := IsPrincipal(2111*D256); c7 := IsPrincipal(T256-7*D256);
print "LIFT_ORDER_CHECKS", c0, cr, c7;
assert not c0 and cr and c7;
"""
code = "ticAll := Cputime();\n" + prefix + f"""
Target := 7*G;
P2q<Xq,Yq,Zq> := ProjectiveSpace(k,2);
Quartic := Curve(P2q,{qexpr});
assert IsNonsingular(Quartic) and Genus(Quartic) eq 3;
QK := BaseChange(Quartic,K);
RQ := CoordinateRing(Ambient(Quartic));
pp := {kbig(source['descent_p'])};
vv := {kbig(source['descent_v'])};
function MapPoint(xx,y0,y1)
    uu := pp*y0+vv*y1;
    pt := QK![{map_expr[0]},{map_expr[1]},{map_expr[2]}];
    assert pt in QK;
    return pt;
end function;
function OrbitDiv(pt)
    xx := pt[1]/pt[3]; yy := pt[2]/pt[3];
    xs := [xx,xx^16,xx^256]; ys := [yy,yy^16,yy^256];
    assert #Seqset(xs) eq 3;
    mx := &*[Tvar-xs[i]:i in [1..3]];
    iy := &+[ys[i]*(Tvar-xs[(i mod 3)+1])*(Tvar-xs[((i+1) mod 3)+1]) /
          ((xs[i]-xs[(i mod 3)+1])*(xs[i]-xs[((i+1) mod 3)+1])):i in [1..3]];
    assert Degree(iy) le 2;
    fx := &+[(RQ!(k!Coefficient(mx,i)))*Xq^i*Zq^(3-i):i in [0..3]];
    hy := Yq*Zq + &+[(RQ!(k!Coefficient(iy,i)))*Xq^i*Zq^(2-i):i in [0..2]];
    place_div := Divisor(Quartic,ideal<RQ|fx,hy>);
    assert Degree(place_div) eq 3;
    return place_div;
end function;
function FiberDiv(P)
    ok,xx,y0,y1a,y1b := SplitFiber(P);
    assert ok;
    return OrbitDiv(MapPoint(xx,y0,y1a)) + OrbitDiv(MapPoint(xx,y0,y1b));
end function;
dR := FiberDiv(2*G);
dG := FiberDiv(3*G);
dT := FiberDiv(2*G+Target);
DD := dG-dR; TT := dT-dR;
print "MAP_CPU", Cputime(ticAll);
{map_checks}
F256 := FiniteField(2,8); Embed(k,F256);
Q256 := BaseChange(Quartic,F256);
R256 := CoordinateRing(Ambient(Q256));
function Lift(V)
    return Divisor(Q256,ideal<R256|[R256!f:f in Basis(Ideal(V))]>);
end function;
D256 := Lift(dG)-Lift(dR); T256 := Lift(dT)-Lift(dR);
{lift_checks}
P0 := RationalPoints(Q256)[1]; D0 := Divisor(P0);
print "PRE_INDEX_CPU", Cputime(ticAll);
SetVerbose("CurveIndexcal",1); tic := Cputime();
answer := IndexCalculus(D256,T256,D0,2111,200,260);
print "INDEX_ANSWER",answer,"INDEX_CPU",Cputime(tic);
assert answer eq 7 and answer*G eq Target;
print "FULL_SOURCE_VERIFIED",true,"FULL_CPU",Cputime(ticAll);
"""
stem = "full_attack_4222" + ("_bench" if args.benchmark else "")
path = HERE / f"{stem}.magma"
path.write_text(code)
req = urllib.request.Request("https://calculator.magma-maths.org/execute",
                             json.dumps({"code":code}).encode(),
                             {"Content-Type":"application/json"},method="POST")
try:
    with urllib.request.urlopen(req,timeout=300,
                                context=ssl.create_default_context(cafile=certifi.where())) as resp:
        payload = json.loads(resp.read())
        status = resp.status
except Exception as exc:
    payload = {"error":repr(exc)}
    status = None
receipt = {"schema":"ghs-order4222-full-dynamic-attack-v1",
           "endpoint":"https://calculator.magma-maths.org/execute",
           "benchmark_without_principal_checks":args.benchmark,
           "http_status":status,"input_sha256":hashlib.sha256(code.encode()).hexdigest(),
           "source_sha256":hashlib.sha256((HERE/"source_4222.json").read_bytes()).hexdigest(),
           "canonical_sha256":hashlib.sha256((HERE/"canonical_4222.json").read_bytes()).hexdigest(),
           "response":payload}
(HERE/f"{stem}_receipt.json").write_text(json.dumps(receipt,indent=2)+"\n")
print(json.dumps({"status":status,"success":payload.get("success"),
                  "exit_code":payload.get("exit_code"),
                  "stdout_tail":payload.get("stdout","")[-3000:],
                  "stderr_tail":payload.get("stderr","")[-1000:],
                  "error":payload.get("error")},indent=2))
