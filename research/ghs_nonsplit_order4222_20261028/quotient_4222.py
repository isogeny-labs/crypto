#!/usr/bin/env sage -python
"""Construct the elliptic quotient and split rational fibers for order 4222."""
import hashlib
import json
import ssl
import urllib.request
from pathlib import Path

import certifi
from sage.all import PolynomialRing

from source_4222 import L, dec, code, q

HERE = Path(__file__).resolve().parent
source = json.loads((HERE / "source_4222.json").read_text())
K = L.subfield(12)


def bits(n):
    return "+".join("1" if i == 0 else "z" if i == 1 else f"z^{i}"
                    for i in range(24) if n >> i & 1) or "0"


def kval(n):
    return f"K!(L!({bits(n)}))"


A, t = dec(source["residue_A"]), dec(source["pole_t"])
R = PolynomialRing(L, "x")
x = R.gen()
f = A/(x+t) + A**(q**3)/(x+t**q)
D, N = f.denominator(), f.numerator()
assert D.degree() == 2 and N.degree() <= 1
assert all(c**(q**3) == c for c in list(D)+list(N))
y0 = None
xx0 = None
for xx in K:
    val = K(f(L(xx)))
    roots = (PolynomialRing(K, "y").gen()**2 + PolynomialRing(K, "y").gen() + val).roots()
    if roots:
        xx0, y0 = xx, roots[0][0]
        break
assert y0 is not None

dh = "+".join(f"{kval(code(D[i]))}*x^{i}*z0^{2-i}"
              for i in range(3) if D[i])
nh = "+".join(f"{kval(code(N[i]))}*x^{i}*z0^{4-i}"
              for i in range(2) if N[i])
code_magma = f"""F2 := GF(2);
P<X> := PolynomialRing(F2);
L<z> := ext<F2 | X^24+X^16+X^15+X^14+X^13+X^10+X^9+X^7+X^5+X^3+1>;
K := FiniteField(2,12); Embed(K,L);
P2<x,y,z0> := ProjectiveSpace(K,2);
C0 := Curve(P2,({dh})*(y^2+y*z0)+({nh}));
print "GENUS", Genus(C0);
pt := C0![{kval(code(L(xx0)))},{kval(code(L(y0)))},K!1];
E0,toE := EllipticCurve(C0,pt);
E := EllipticCurve([K|1,{kval(source['source_a'])},0,0,{kval(source['source_b'])}]);
print "SOURCE_ORDER", #E;
ok,iso := IsIsomorphic(E0,E);
print "ISOMORPHIC",ok;
assert ok;
polys := DefiningPolynomials(toE);
print "TO_E_EXPONENTS";print [[Exponents(m):m in Monomials(g)]:g in polys];
print "TO_E_COEFF_BITS";print [[Eltseq(L!v):v in Coefficients(g)]:g in polys];
print "ISO_BITS";print [Eltseq(L!v):v in IsomorphismData(iso)];
G := E![{kval(source['source_G'][0])},{kval(source['source_G'][1])}];
assert G ne E!0 and 2111*G eq E!0;
T<Tvar> := PolynomialRing(K);
A := L!({bits(source['residue_A'])});
pole := L!({bits(source['pole_t'])});
function SplitFiber(P)
  P0 := P@@iso;
  pre := P0@@toE;
  for cp in RationalPoints(pre) do
    if cp[3] ne 0 then
      pc := C0!cp;
      if iso(toE(pc)) eq P then
        xx := pc[1]/pc[3]; yy := pc[2]/pc[3];
        f1 := K!(A^16/(L!xx+pole^16) + A^65536/(L!xx+pole));
        rr := Roots(Tvar^2+Tvar+f1);
        if #rr eq 2 then
          return true,xx,yy,rr[1][1],rr[2][1];
        end if;
      end if;
    end if;
  end for;
  return false,K!0,K!0,K!0,K!0;
end function;
for n in [1..100] do
  ok0,x0,u0,v0,w0 := SplitFiber(n*G);
  ok1,x1,u1,v1,w1 := SplitFiber((n+1)*G);
  ok7,x7,u7,v7,w7 := SplitFiber((n+7)*G);
  if ok0 and ok1 and ok7 then
    print "SCALAR",n;
    print "FIBERS";print [Eltseq(L!u):u in [x0,u0,v0,w0,x1,u1,v1,w1,x7,u7,v7,w7]];
    break;
  end if;
end for;
"""
input_path = HERE / "quotient_4222.magma"
input_path.write_text(code_magma)
req = urllib.request.Request("https://calculator.magma-maths.org/execute",
                             json.dumps({"code": code_magma}).encode(),
                             {"Content-Type": "application/json"}, method="POST")
with urllib.request.urlopen(req, timeout=300,
                            context=ssl.create_default_context(cafile=certifi.where())) as resp:
    payload = json.loads(resp.read())
    status = resp.status
receipt = {"schema": "ghs-order4222-quotient-v1", "http_status": status,
           "input_sha256": hashlib.sha256(code_magma.encode()).hexdigest(),
           "response": payload}
(HERE / "quotient_4222_receipt.json").write_text(json.dumps(receipt, indent=2) + "\n")
assert status == 200 and payload.get("success") and payload.get("exit_code") == 0, payload
out = payload["stdout"]
print(out)
