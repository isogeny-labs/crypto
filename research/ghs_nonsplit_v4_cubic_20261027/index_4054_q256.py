#!/usr/bin/env sage -python
"""Generate and run a full Magma line-sieve attempt for the q16 source."""
import hashlib
import argparse
import json
import ssl
import urllib.request
from pathlib import Path

import certifi
from sage.all import PolynomialRing

from nonsplit import L, dec, q

HERE = Path(__file__).resolve().parent
URL = "https://calculator.magma-maths.org/execute"
parser = argparse.ArgumentParser()
parser.add_argument("--factor-base", type=int, default=200)
parser.add_argument("--relations", type=int, default=260)
parser.add_argument("--benchmark", action="store_true",
                    help="omit independently checked principal-divisor assertions from timed path")
args = parser.parse_args()
assert 1 <= args.factor_base <= 256 and args.relations > args.factor_base
canonical = json.loads((HERE/"canonical_4054.json").read_text())
mapped = json.loads((HERE/"subgroup_map_4054.json").read_text())


def z_expr(n):
    return "+".join("1" if i == 0 else "z" if i == 1 else f"z^{i}"
                    for i in range(24) if (n >> i) & 1) or "0"


def l_code(x):
    return sum(int(c) << i for i, c in enumerate(L(x).polynomial().list()))


def k_expr(x):
    value = dec(x) if isinstance(x, int) else L(x)
    return f"k!(L!({z_expr(l_code(value))}))"


T = PolynomialRing(L, "T").gen()


def orbit_ideal(point):
    x, y, z = (dec(v) for v in point)
    assert z == 1
    xs = [x**(q**i) for i in range(3)]
    ys = [y**(q**i) for i in range(3)]
    assert len(set(xs)) == 3
    mx = (T-xs[0])*(T-xs[1])*(T-xs[2])
    iy = sum((ys[i]*
              ((T-xs[(i+1)%3])*(T-xs[(i+2)%3]))/
              ((xs[i]-xs[(i+1)%3])*(xs[i]-xs[(i+2)%3]))
              for i in range(3)), T.parent()(0))
    assert iy.degree() <= 2
    assert all(c**q == c for c in list(mx)+list(iy))
    f = "+".join(f"({k_expr(mx[i])})*X^{i}*Z^{3-i}"
                 for i in range(4) if mx[i])
    h = "Y*Z + " + " + ".join(
        f"({k_expr(iy[i])})*X^{i}*Z^{2-i}"
        for i in range(3) if iy[i])
    return f, h


qterms = [f"({k_expr(c)})*X^{e[0]}*Y^{e[1]}*Z^{e[2]}"
          for e, c in zip(canonical["quartic_exponents"],
                          canonical["quartic_coefficients"])]
qexpr = "+".join(qterms)
ideals = {label: [orbit_ideal(p) for p in points]
          for label, points in mapped["quartic_fibers"].items()}

code = f"""ticAll := Cputime();
F2 := GF(2);
P<U> := PolynomialRing(F2);
L<z> := ext<F2 | U^24+U^16+U^15+U^14+U^13+U^10+U^9+U^7+U^5+U^3+1>;
k := FiniteField(2,4); Embed(k,L);
P2<X,Y,Z> := ProjectiveSpace(k,2);
Q := Curve(P2,{qexpr});
assert IsNonsingular(Q) and Genus(Q) eq 3;
R := CoordinateRing(Ambient(Q));
"""
for label, rows in ideals.items():
    for j, (f, h) in enumerate(rows):
        code += f"d_{label}_{j} := Divisor(Q,ideal<R | {f}, {h}>);\n"
        code += f"assert Degree(d_{label}_{j}) eq 3;\n"
    code += f"d_{label} := d_{label}_0 + d_{label}_1;\n"
code += "D := d_R_plus_G-d_R; T := d_R_plus_Q-d_R;\n"
if not args.benchmark:
    code += "b0 := IsPrincipal(D); br := IsPrincipal(2027*D); b7 := IsPrincipal(T-7*D);\n"
    code += "print \"BASE_ORDER_CHECKS\", b0, br, b7; assert not b0 and br and b7;\n"
code += "F256 := FiniteField(2,8); Embed(k,F256); Q256 := BaseChange(Q,F256);\n"
code += "R256 := CoordinateRing(Ambient(Q256));\n"
code += "function Lift(V)\n"
code += "    return Divisor(Q256,ideal<R256 | [R256!f : f in Basis(Ideal(V))]>);\n"
code += "end function;\n"
for label in ideals:
    for j in range(2):
        code += f"e_{label}_{j} := Lift(d_{label}_{j});\n"
    code += f"e_{label} := e_{label}_0+e_{label}_1;\n"
code += "DD := e_R_plus_G-e_R; TT := e_R_plus_Q-e_R;\n"
if not args.benchmark:
    code += "c0 := IsPrincipal(DD); cr := IsPrincipal(2027*DD); c7 := IsPrincipal(TT-7*DD);\n"
    code += "print \"LIFT_ORDER_CHECKS\", c0, cr, c7; assert not c0 and cr and c7;\n"
code += "P0 := RationalPoints(Q256)[1]; D0 := Divisor(P0);\n"
code += "print \"PRE_INDEX_CPU\", Cputime(ticAll);\n"
code += "SetVerbose(\"CurveIndexcal\",1); tic := Cputime();\n"
code += f"answer := IndexCalculus(DD,TT,D0,2027,{args.factor_base},{args.relations});\n"
code += "print \"INDEX_ANSWER\", answer, \"INDEX_CPU\", Cputime(tic);\n"
code += "if answer eq 7 then print \"SOURCE_VERIFIED\", true; else error \"wrong logarithm\"; end if;\n"

stem = f"index_4054_q256_fb{args.factor_base}" + ("_bench" if args.benchmark else "")
path = HERE/f"{stem}.magma"
path.write_text(code)
req = urllib.request.Request(URL, json.dumps({"code": code}).encode(),
                             {"Content-Type": "application/json"}, method="POST")
try:
    with urllib.request.urlopen(req, timeout=300,
                                context=ssl.create_default_context(cafile=certifi.where())) as resp:
        payload = json.loads(resp.read())
        status = resp.status
except Exception as exc:
    payload = {"error": repr(exc)}
    status = None
receipt = {"schema": "ghs-nonsplit-q256-index-attempt-v1",
           "endpoint": URL, "http_status": status,
           "factor_base_requested": args.factor_base,
           "relations_requested": args.relations,
           "benchmark_without_principal_checks": args.benchmark,
           "input_sha256": hashlib.sha256(code.encode()).hexdigest(),
           "response": payload}
(HERE/f"{stem}_receipt.json").write_text(json.dumps(receipt, indent=2)+"\n")
print(json.dumps({"http_status": status, "success": payload.get("success"),
                  "exit_code": payload.get("exit_code"),
                  "stdout_tail": payload.get("stdout", "")[-3000:],
                  "stderr_tail": payload.get("stderr", "")[-1000:],
                  "error": payload.get("error")}, indent=2))
