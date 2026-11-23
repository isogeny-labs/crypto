#!/usr/bin/env python3
"""Parsers for msolve input/output and the -v 2 F4 trace."""
import re

ROUND = re.compile(r"^\s*(\d+)\s+(\d+)\s+(\d+)\s+(\d+) x (\d+)\s+([\d.]+)%\s+(\d+) new\s+(\d+) zero")


def parse_poly(s, names, p):
    """msolve polynomial text -> dict exponent-tuple -> coeff mod p"""
    idx = {n: i for i, n in enumerate(names)}
    nv = len(names)
    s = s.replace(" ", "").replace("\n", "")
    if not s:
        return {}
    # split into signed terms
    terms = re.findall(r"[+-]?[^+-]+", s)
    P = {}
    for term in terms:
        sign = 1
        if term[0] == "+":
            term = term[1:]
        elif term[0] == "-":
            sign = -1
            term = term[1:]
        coef = 1
        e = [0] * nv
        for f in term.split("*"):
            if not f:
                continue
            if re.fullmatch(r"\d+", f):
                coef = coef * int(f) % p
            else:
                if "^" in f:
                    v, k = f.split("^")
                    e[idx[v]] += int(k)
                else:
                    e[idx[f]] += 1
        k = tuple(e)
        P[k] = (P.get(k, 0) + sign * coef) % p
        if P[k] == 0:
            del P[k]
    return P


def read_ms(path):
    with open(path) as fh:
        lines = fh.read().split("\n")
    names = lines[0].strip().split(",")
    p = int(lines[1].strip())
    body = "\n".join(lines[2:])
    polys = [parse_poly(x, names, p) for x in body.split(",") if x.strip()]
    return names, p, polys


def read_gb(path, names, p):
    """msolve -g 2 output -> list of polys (dicts); [1] -> [{(0..):1}]"""
    with open(path) as fh:
        txt = fh.read()
    lines = [ln for ln in txt.split("\n") if not ln.startswith("#")]
    body = "\n".join(lines).strip()
    if body.endswith(":"):
        body = body[:-1]
    body = body.strip()
    assert body.startswith("[") and body.endswith("]"), body[:200]
    body = body[1:-1]
    return [parse_poly(x, names, p) for x in body.split(",") if x.strip()]


def parse_trace(path):
    rounds = []
    with open(path, errors="replace") as fh:
        for ln in fh:
            mm = ROUND.match(ln)
            if mm:
                d, sel, pairs, rows, cols, dens, new, zero = mm.groups()
                rounds.append(dict(deg=int(d), sel=int(sel), pairs=int(pairs), rows=int(rows),
                                   cols=int(cols), density=float(dens), new=int(new), zero=int(zero)))
    return rounds


def eval_poly(P, point, F):
    """evaluate dict poly at point (list of F_{p^2} pairs) using field object F"""
    acc = (0, 0)
    cache = {}
    for k, c in P.items():
        term = (c % F.p, 0)
        for i, e in enumerate(k):
            if e:
                key = (i, e)
                if key not in cache:
                    v = (1, 0)
                    for _ in range(e):
                        v = F.mul(v, point[i])
                    cache[key] = v
                term = F.mul(term, cache[key])
        acc = F.add(acc, term)
    return acc
