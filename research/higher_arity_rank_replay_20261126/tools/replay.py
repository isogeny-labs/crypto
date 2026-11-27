#!/usr/bin/env python3
"""Exact projected-rank replay of the B677 four-sum stream under two admission rules.

fb2: admit a relation only if it has >= 2 factor-base terms (the #5097 consumer rule).
all: admit every replayed relation (higher large-prime arity included).

Online elimination over Z/n: large-prime columns first, then factor-base columns, then the
target column (coefficient -b, right-hand side a), exactly as the #5097 consumer orders
pivots. Reports LP rank, point-only projected rank, and whether the target column has a
pivot (target identifiable), at every query boundary and fixed row prefixes. Independent
Python arithmetic; every relation is replayed on the curve first.
"""
import json, sys
N = 2147446831; P = 2147483647; A = 847514846
stream = json.load(open(sys.argv[1]))
pts, is_fb = [], []
for line in open(sys.argv[2]):
    x, y, t = line.strip().split(","); pts.append((int(x), int(y))); is_fb.append(t == "fb")
fb_cols, lp_cols, col = {}, {}, {}
for i, f in enumerate(is_fb):
    (fb_cols if f else lp_cols)[i] = None
FB = sum(is_fb); LP = len(pts) - FB
k_fb = k_lp = 0
for i, f in enumerate(is_fb):
    if f: col[i] = k_fb; k_fb += 1
    else: col[i] = FB + k_lp; k_lp += 1
TARGET, RHS = FB + LP, FB + LP + 1

def add(u, v):
    if u is None: return v
    if v is None: return u
    (x1, y1), (x2, y2) = u, v
    if x1 == x2 and (y1 + y2) % P == 0: return None
    if x1 == x2: l = (3 * x1 * x1 + A) * pow(2 * y1, P - 2, P) % P
    else: l = (y2 - y1) * pow(x2 - x1, P - 2, P) % P
    x3 = (l * l - x1 - x2) % P
    return (x3, (l * (x1 - x3) - y1) % P)
def mul(k, u):
    r = None
    while k:
        if k & 1: r = add(r, u)
        u = add(u, u); k >>= 1
    return r
G = (625037715, 2100627251); Q = (976979610, 1228979761)
queries = {q["label"]: q for q in stream["queries"]}
R = {l: add(mul(q["a"], G), mul(q["b"], Q)) for l, q in queries.items()}

class Rank:
    def __init__(s): s.piv = {}; s.lp = s.fb = 0; s.target = None
    def add(s, row):
        order = sorted(c for c in row if c < TARGET and c >= FB) + sorted(c for c in row if c < FB) + ([TARGET] if TARGET in row else [])
        row = dict(row)
        # Eliminate in pivot order: LP columns, then FB, then target.
        def keys(r): return sorted([c for c in r if FB <= c < TARGET]) + sorted([c for c in r if c < FB]) + ([TARGET] if TARGET in r else [])
        while True:
            ks = keys(row)
            if not ks:
                assert row.get(RHS, 0) == 0, "inconsistent"
                return False
            c = ks[0]
            if c in s.piv:
                f = row[c]; pr = s.piv[c]
                for k, v in pr.items():
                    nv = (row.get(k, 0) - f * v) % N
                    if nv: row[k] = nv
                    else: row.pop(k, None)
            else:
                inv = pow(row[c], N - 2, N)
                s.piv[c] = {k: v * inv % N for k, v in row.items()}
                if c >= FB and c < TARGET: s.lp += 1
                elif c < FB: s.fb += 1
                else: s.target = s.piv[c][RHS] if RHS in s.piv[c] else 0
                return True

out = {"rows": len(stream["relation_rows"]), "rules": {}}
for rule in ("fb2", "all"):
    rk = Rank(); admitted = 0; first_target_row = None; first_target_query = None
    boundary = []; prefixes = {}; trajectory = []
    rows = stream["relation_rows"]
    for idx, rel in enumerate(rows):
        lab = rel["label"]; terms = rel["terms"]
        s = None
        for i, sg in terms:
            p = pts[i]; s = add(s, p if sg == 1 else (p[0], (P - p[1]) % P))
        assert s == R[lab], "relation must replay"
        nfb = sum(is_fb[i] for i, _ in terms)
        if rule == "all" or nfb >= 2:
            admitted += 1
            row = {}
            for i, sg in terms:
                row[col[i]] = (row.get(col[i], 0) + (1 if sg == 1 else N - 1)) % N
            row[TARGET] = (-queries[lab]["b"]) % N; row[RHS] = queries[lab]["a"] % N
            rk.add(row)
            if rk.target is not None and first_target_row is None:
                first_target_row, first_target_query = idx + 1, lab + 1
                assert mul(rk.target, G) == Q, "recovered log must verify"
        trajectory.append([idx + 1, admitted, rk.lp, rk.fb, int(rk.target is not None)])
        if idx + 1 in (256, 512, 613, 645, len(rows)):
            prefixes[idx + 1] = dict(admitted=admitted, lp_rank=rk.lp, point_projected_rank=rk.fb, target=rk.target is not None)
        if idx + 1 == len(rows) or rows[idx + 1]["label"] != lab:
            boundary.append(dict(queries=lab + 1, rows=idx + 1, admitted=admitted, lp_rank=rk.lp,
                                 point_projected_rank=rk.fb, target_identifiable=rk.target is not None))
    out["rules"][rule] = dict(first_target_row=first_target_row, first_target_query=first_target_query,
                              recovered=rk.target, by_query=boundary, prefixes=prefixes,
                              trajectory_columns=["rows", "admitted", "lp_rank", "point_projected_rank", "target"],
                              trajectory=trajectory)
print(json.dumps(out, indent=1))
