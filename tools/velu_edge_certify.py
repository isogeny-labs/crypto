#!/usr/bin/env python3
"""Independent, exact certificate checker for prime-field Vélu l-isogeny edges (l = 2, 3).

Why this exists: ``src/bin/volcano_rho.rs`` checks each walked edge on *two
sample points* (image on codomain, phi(P+Q) = phi(P)+phi(Q), n*phi(P) = O) and
records only ``{"ell", "kernel_x"}`` per edge (and the older
``curves.jsonl`` records only ``parent``). Nothing re-derives, from the record
alone, that the kernel abscissa is a root of the l-division polynomial or that
the Vélu rational map sends E onto E' identically. This checker does that with
code that shares nothing with the producer (pure Python, standard library).

For an edge (p, a, b) --(l, x0)--> (p, a2, b2) it verifies, exactly:

1. ``psi_l(x0) = 0`` over F_p (l=2: x0^3+a x0+b; l=3: 3x^4+6ax^2+12bx-a^2),
   and for l=3 that the kernel point is not 2-torsion (rhs(x0) != 0); so
   <(x0, .)> is a subgroup of order exactly l, stable under Frobenius.
2. The Vélu map X = x + t/(x-x0) + u/(x-x0)^2,
   Y = y(1 - t/(x-x0)^2 - 2u/(x-x0)^3) satisfies Y^2 = X^3 + A X + B as an
   identity of polynomials in F_p[x] after clearing (x-x0)^6 and substituting
   y^2 = x^3+ax+b. This is the whole rational map, not samples.
3. The X-map has degree l (numerator degree l, kernel fibre at x0 only).
4. The recorded codomain (a2, b2) is F_p-isomorphic to the Vélu codomain
   (A, B): a2 = w^2 A, b2 = w^3 B with w = u^2 a nonzero square in F_p.
   A quadratic twist is rejected.

Scope: certifies that a recorded edge is an F_p-rational separable isogeny of
degree l with the stated kernel. It says nothing about ECDLP cost and is not a
speedup. It does not handle l > 3 kernels (no kernel-polynomial Vélu/Kohel).
"""
from __future__ import annotations

import argparse
import json
import sys
from dataclasses import dataclass


# --------------------------------------------------------------- F_p[x]
def _trim(f):
    f = list(f)
    while f and f[-1] == 0:
        f.pop()
    return f


def padd(f, g, p):
    n = max(len(f), len(g))
    return _trim([((f[i] if i < len(f) else 0) + (g[i] if i < len(g) else 0)) % p for i in range(n)])


def pscale(f, c, p):
    return _trim([(c * v) % p for v in f])


def pmul(f, g, p):
    if not f or not g:
        return []
    out = [0] * (len(f) + len(g) - 1)
    for i, u in enumerate(f):
        if u:
            for j, v in enumerate(g):
                out[i + j] = (out[i + j] + u * v) % p
    return _trim(out)


def ppow(f, e, p):
    r = [1]
    for _ in range(e):
        r = pmul(r, f, p)
    return r


def peval(f, x, p):
    acc = 0
    for c in reversed(f):
        acc = (acc * x + c) % p
    return acc


def is_square(v, p):
    v %= p
    return v == 0 or pow(v, (p - 1) // 2, p) == 1


# --------------------------------------------------------------- certificate
@dataclass
class Verdict:
    ok: bool
    reason: str
    codomain: tuple[int, int] | None = None


def division_poly(ell, a, b, p):
    if ell == 2:
        return [b % p, a % p, 0, 1]
    if ell == 3:
        return [(-a * a) % p, (12 * b) % p, (6 * a) % p, 0, 3]
    raise ValueError("only l in {2, 3}")


def velu_params(ell, a, b, x0, p):
    gx = (3 * x0 * x0 + a) % p
    if ell == 2:
        t, u = gx, 0
    else:
        t, u = (2 * gx) % p, (4 * (x0 ** 3 + a * x0 + b)) % p
    w = (u + x0 * t) % p
    return t, u, ((a - 5 * t) % p, (b - 7 * w) % p)


def check_map_identity(a, b, x0, t, u, A, B, p):
    """Exact check of Y^2 = X^3 + A X + B for the Vélu map, cleared by (x-x0)^6."""
    f = [b % p, a % p, 0, 1]                      # y^2
    d = [(-x0) % p, 1]                             # x - x0
    d2, d3 = pmul(d, d, p), ppow(d, 3, p)
    # X * d^2 = x d^2 + t d + u
    xn = padd(padd(pmul([0, 1], d2, p), pscale(d, t, p), p), [u % p], p)
    # Y * d^3 = y * (d^3 - t d - 2u)  => Y^2 d^6 = f * yn^2
    yn = padd(padd(d3, pscale(d, -t, p), p), [(-2 * u) % p], p)
    lhs = pmul(f, pmul(yn, yn, p), p)
    # (X^3 + A X + B) d^6 = xn^3 + A xn d^4 + B d^6
    rhs = padd(padd(ppow(xn, 3, p), pscale(pmul(xn, ppow(d2, 2, p), p), A, p), p),
               pscale(ppow(d, 6, p), B, p), p)
    return padd(lhs, pscale(rhs, -1, p), p) == [], xn


def certify_edge(p, a, b, ell, x0, a2=None, b2=None) -> Verdict:
    a, b, x0 = a % p, b % p, x0 % p
    if (4 * a ** 3 + 27 * b * b) % p == 0:
        return Verdict(False, "domain singular")
    if ell not in (2, 3):
        return Verdict(False, f"unsupported l={ell}")
    if peval(division_poly(ell, a, b, p), x0, p) != 0:
        return Verdict(False, f"x0 is not a root of psi_{ell}: not an {ell}-kernel")
    rhs0 = (x0 ** 3 + a * x0 + b) % p
    if ell == 3 and rhs0 == 0:
        return Verdict(False, "kernel point is 2-torsion, not of order 3")
    t, u, (A, B) = velu_params(ell, a, b, x0, p)
    if (4 * A ** 3 + 27 * B * B) % p == 0:
        return Verdict(False, "codomain singular")
    ok, xn = check_map_identity(a, b, x0, t, u, A, B, p)
    if not ok:
        return Verdict(False, "Velu map does not send E onto E' identically")
    # X = (x d^2 + t d + u) / d^2. If u = 0 the factor d cancels and X has
    # degree 2 (needs t != 0); otherwise X has degree 3.
    if u == 0 and t == 0:
        return Verdict(False, "degenerate map (t = u = 0)")
    reduced_deg = 2 if u == 0 else 3
    if reduced_deg != ell:
        return Verdict(False, f"x-map has degree {reduced_deg}, expected {ell}")
    if a2 is None:
        return Verdict(True, "ok (codomain = Velu model)", (A, B))
    a2, b2 = a2 % p, b2 % p
    if (A == 0) != (a2 == 0) or (B == 0) != (b2 == 0):
        return Verdict(False, "recorded codomain not isomorphic (j differs)")
    # find w = u^2 with a2 = w^2 A, b2 = w^3 B
    if A and B:
        w = b2 * A * pow(B * a2, -1, p) % p
        cands = [w]
    elif A:  # j = 1728: w^2 = a2/A
        r = a2 * pow(A, -1, p) % p
        cands = [c for c in _sqrts(r, p)]
    else:   # j = 0: w^3 = b2/B
        r = b2 * pow(B, -1, p) % p
        cands = [c for c in range(1, p) if pow(c, 3, p) == r] if p < 10 ** 6 else []
    for w in cands:
        if w and (w * w * A - a2) % p == 0 and (w ** 3 * B - b2) % p == 0:
            if is_square(w, p):
                return Verdict(True, "ok (codomain F_p-isomorphic to Velu model)", (A, B))
            return Verdict(False, "recorded codomain is a quadratic twist of the Velu codomain")
    return Verdict(False, "recorded codomain not isomorphic to Velu codomain")


def _sqrts(r, p):
    # Tonelli-Shanks-free small fallback is unnecessary for p = 3 mod 4; generic:
    if not is_square(r, p):
        return []
    q, s = p - 1, 0
    while q % 2 == 0:
        q, s = q // 2, s + 1
    z = 2
    while is_square(z, p):
        z += 1
    m, c, t, x = s, pow(z, q, p), pow(r, q, p), pow(r, (q + 1) // 2, p)
    while t != 1:
        i, tt = 0, t
        while tt != 1:
            tt, i = tt * tt % p, i + 1
        bb = pow(c, 1 << (m - i - 1), p)
        m, c, t, x = i, bb * bb % p, t * bb * bb % p, x * bb % p
    return [x, (-x) % p]


def kernel_roots(ell, a, b, p):
    """Rational roots of psi_l by exhaustive evaluation (small p) or gcd with x^p - x."""
    f = division_poly(ell, a, b, p)
    if p < 5000:
        return [x for x in range(p) if peval(f, x, p) == 0]
    g = _gcd(f, padd(_powmod([0, 1], p, f, p), [0, p - 1], p), p)
    return sorted(_split(g, p, 1))


def _pdivmod(f, g, p):
    f, g = _trim(f), _trim(g)
    inv = pow(g[-1], -1, p)
    q = [0] * max(len(f) - len(g) + 1, 0)
    r = list(f)
    while len(r) >= len(g) and r:
        c = r[-1] * inv % p
        k = len(r) - len(g)
        q[k] = c
        for i, v in enumerate(g):
            r[k + i] = (r[k + i] - c * v) % p
        r = _trim(r)
    return _trim(q), r


def _powmod(base, e, m, p):
    r, b = [1], _pdivmod(base, m, p)[1]
    while e:
        if e & 1:
            r = _pdivmod(pmul(r, b, p), m, p)[1]
        b = _pdivmod(pmul(b, b, p), m, p)[1]
        e >>= 1
    return r


def _gcd(f, g, p):
    f, g = _trim(f), _trim(g)
    while g:
        f, g = g, _pdivmod(f, g, p)[1]
    return pscale(f, pow(f[-1], -1, p), p) if f else f


def _split(g, p, seed):
    """Roots of a squarefree product of distinct linear factors (Cantor-Zassenhaus)."""
    if len(g) <= 1:
        return []
    if len(g) == 2:
        return [(-g[0]) % p]
    while True:
        seed += 1
        h = _powmod([seed % p, 1], (p - 1) // 2, g, p)
        d = _gcd(g, padd(h, [p - 1], p), p)
        if 1 < len(d) < len(g):
            return _split(d, p, seed) + _split(_pdivmod(g, d, p)[0], p, seed)


def certify_parent_child(p, parent, child, ell):
    """For records without kernel_x: some rational l-kernel of the parent must certify the child."""
    reasons = []
    for x0 in kernel_roots(ell, parent[0], parent[1], p):
        v = certify_edge(p, parent[0], parent[1], ell, x0, child[0], child[1])
        if v.ok:
            return Verdict(True, f"{v.reason}; kernel_x={x0}", v.codomain)
        reasons.append(v.reason)
    return Verdict(False, "no rational kernel certifies this edge: " + "; ".join(sorted(set(reasons))))


def certify_curves_jsonl(path, ell):
    rows = [json.loads(line) for line in open(path) if line.strip()]
    by = {(r["vertex"], r["model"]): r for r in rows}
    out = {"checked": 0, "failed": [], "parent_absent": 0}
    for r in rows:
        if r["parent"] is None or r["model"] != 0:
            continue
        par = by.get((r["parent"], 0))
        if par is None:  # record does not carry the parent: edge not re-derivable
            out["parent_absent"] += 1
            continue
        v = certify_parent_child(r["p"], (par["a"], par["b"]), (r["a"], r["b"]), ell)
        out["checked"] += 1
        if not v.ok:
            out["failed"].append({"vertex": r["vertex"], "reason": v.reason})
    return out


def certify_results_json(path):
    """volcano-rho results.json: every vertex edge {ell, kernel_x} against its parent, exactly."""
    d = json.load(open(path))
    p = d["problem"]["p"]
    vs = {v["id"]: v for v in d["vertices"]}
    out = {"checked": 0, "failed": [], "recorded_edges_verified": d["walk"].get("edges_verified")
           if isinstance(d.get("walk"), dict) else None}
    for v in d["vertices"]:
        e = v.get("edge")
        if v["parent"] is None or e is None:
            continue
        par = vs[v["parent"]]
        r = certify_edge(p, par["a"], par["b"], e["ell"], e["kernel_x"], v["a"], v["b"])
        out["checked"] += 1
        if not r.ok:
            out["failed"].append({"vertex": v["id"], "reason": r.reason})
    return out


def main(argv=None):
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--curves-jsonl", help="isogeny-volcano curves.jsonl to certify")
    ap.add_argument("--results-json", help="volcano-rho results.json (edges carry kernel_x)")
    ap.add_argument("--ell", type=int)
    ap.add_argument("--edge", help="p,a,b,x0[,a2,b2] single edge")
    args = ap.parse_args(argv)
    if args.edge:
        vals = [int(v) for v in args.edge.split(",")]
        v = certify_edge(vals[0], vals[1], vals[2], args.ell, vals[3], *vals[4:6])
        print(json.dumps({"ok": v.ok, "reason": v.reason, "codomain": v.codomain}))
        return 0 if v.ok else 1
    if args.results_json:
        res = certify_results_json(args.results_json)
        print(json.dumps(res))
        return 0 if not res["failed"] else 1
    res = certify_curves_jsonl(args.curves_jsonl, args.ell)
    print(json.dumps(res))
    return 0 if not res["failed"] else 1


if __name__ == "__main__":
    sys.exit(main())
