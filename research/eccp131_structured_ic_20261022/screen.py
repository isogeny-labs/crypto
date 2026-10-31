#!/usr/bin/env python3
"""Bounded exact ECCp-131 multiplicative-coset relation screen."""

import argparse
import hashlib
import json
import random
import re
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
CURVE_SOURCE = ROOT / "src/ecc/curve.rs"
TARGET_SOURCE = ROOT / "src/bin/ic/params.rs"
Point = tuple[int, int] | None


def parameters():
    curve_text = CURVE_SOURCE.read_text()
    block = curve_text.split("pub fn eccp131() -> Self {", 1)[1].split("pub fn secp256k1", 1)[0]
    vals = {k: int(v, 16) for k, v in re.findall(r'(\w+): hex\(b"([0-9A-F]+)"\)', block)}
    assert set(vals) == {"p", "a", "b", "gx", "gy", "n"}
    target_text = TARGET_SOURCE.read_text()
    block = target_text.split('"eccp-131" | "eccp131" => {', 1)[1].split('"ecc2k-130"', 1)[0]
    coords = re.findall(r'\b[xy]: "0x([0-9A-F]+)"', block)
    assert len(coords) == 2
    vals["qx"], vals["qy"] = [int(v, 16) for v in coords]
    return vals


def neg(P: Point, p: int) -> Point:
    return None if P is None else (P[0], (-P[1]) % p)


def add(P: Point, Q: Point, p: int, a: int) -> Point:
    if P is None:
        return Q
    if Q is None:
        return P
    x, y = P
    u, v = Q
    if x == u and (y + v) % p == 0:
        return None
    if x == u:
        lam = ((3 * x * x + a) * pow(2 * y, -1, p)) % p
    else:
        lam = ((v - y) * pow(u - x, -1, p)) % p
    w = (lam * lam - x - u) % p
    return (w, (lam * (x - w) - y) % p)


def mul(k: int, P: Point, p: int, a: int) -> Point:
    R = None
    while k:
        if k & 1:
            R = add(R, P, p, a)
        P = add(P, P, p, a)
        k >>= 1
    return R


def lift(x: int, v: dict) -> Point:
    p = v["p"]
    rhs = (x * x * x + v["a"] * x + v["b"]) % p
    # The published p is 3 modulo 4; verify each square root.
    assert p % 4 == 3
    y = pow(rhs, (p + 1) // 4, p)
    if y * y % p != rhs:
        return None
    return (x, min(y, (-y) % p))


def small_base(size: int, v: dict):
    out = []
    x = 0
    while len(out) < size:
        P = lift(x, v)
        if P is not None:
            out.append(P)
        x += 1
    return out


def rank(rows: list[list[int]], n: int) -> int:
    pivots = {}
    for row in rows:
        x = [v % n for v in row]
        for i, pivot in sorted(pivots.items()):
            if x[i]:
                factor = x[i]
                x = [(u - factor * w) % n for u, w in zip(x, pivot)]
        i = next((i for i, u in enumerate(x) if u), None)
        if i is not None:
            inverse = pow(x[i], -1, n)
            pivots[i] = [(u * inverse) % n for u in x]
    return len(pivots)


def screen_base(base, v, seed: int, setup_count: int, target_count: int):
    p, a, n = (v[k] for k in ("p", "a", "n"))
    G, Q = (v["gx"], v["gy"]), (v["qx"], v["qy"])
    signed = [(P if s == 1 else neg(P, p), i, s)
              for i, P in enumerate(base) for s in (1, -1)]
    pair = {}
    for P, i, s in signed:
        for R, j, t in signed:
            S = add(P, R, p, a)
            if S is not None:
                pair.setdefault(S, ((i, s), (j, t)))

    def decompose(T):
        for P, i, s in signed:
            R = add(T, neg(P, p), p, a)
            if R in pair:
                terms = pair[R] + ((i, s),)
                check = None
                for j, sign in terms:
                    check = add(check, base[j] if sign == 1 else neg(base[j], p), p, a)
                if check == T:
                    return terms
        return None

    rng = random.Random(seed)
    planted = []
    for _ in range(8):
        while True:
            chosen = [signed[rng.randrange(len(signed))] for _ in range(3)]
            T = None
            for P, _, _ in chosen:
                T = add(T, P, p, a)
            if T is not None:
                break
        terms = decompose(T)
        if terms is None:
            raise RuntimeError("planted relation was missed")
        planted.append({"point": [hex(T[0]), hex(T[1])], "terms": terms})

    rows, natural, target = [], [], []
    for target_dependent, count in ((False, setup_count), (True, target_count)):
        for index in range(count):
            scalar = 0 if target_dependent and index == 0 else rng.randrange(1, n)
            T = mul(scalar, G, p, a)
            if target_dependent:
                T = add(Q, T, p, a)
            terms = decompose(T)
            if terms is None:
                continue
            row = [0] * len(base)
            for j, sign in terms:
                row[j] += sign
            witness = {"index": index, "scalar": hex(scalar), "point": [hex(T[0]), hex(T[1])],
                       "terms": terms, "row": row}
            if target_dependent:
                target.append(witness)
            else:
                natural.append(witness)
                rows.append(row)
    return {"base": [[hex(x), hex(y)] for x, y in base],
            "base_sha256": hashlib.sha256(json.dumps(base, separators=(",", ":")).encode()).hexdigest(),
            "distinct_pair_points": len(pair), "planted_controls": planted,
            "setup_relations": natural, "setup_rank": rank(rows, n),
            "target_relations": target, "combined_rank": rank(rows + [w["row"] for w in target], n),
            "target_scalar": None}


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--seed", type=int, required=True)
    ap.add_argument("--setup", type=int, default=256)
    ap.add_argument("--target", type=int, default=256)
    ap.add_argument("--cosets", type=int, default=16)
    args = ap.parse_args()
    if (args.setup, args.target, args.cosets) != (256, 256, 16) or args.seed not in (131, 132):
        ap.error("the frozen protocol requires 256 setup, 256 target, 16 cosets, seed 131 or 132")
    start = time.perf_counter()
    v = parameters()
    p, a, n = (v[k] for k in ("p", "a", "n"))
    assert (p - 1) % 5 == 0 and (p - 1) % 4 != 0
    G, Q = (v["gx"], v["gy"]), (v["qx"], v["qy"])
    for P in (G, Q):
        assert P == lift(P[0], v) or P == neg(lift(P[0], v), p)
        assert mul(n, P, p, a) is None
    zeta = next(z for h in range(2, 100) if (z := pow(h, (p - 1) // 5, p)) != 1)
    assert pow(zeta, 5, p) == 1
    used = set()
    arms = []
    for g in range(1, args.cosets + 1):
        xs = sorted({g * pow(zeta, j, p) % p for j in range(5)})
        base = [P for x in xs if (P := lift(x, v)) is not None]
        if len(base) < 2:
            arms.append({"representative": g, "lifted": len(base), "status": "too_few_columns"})
            continue
        digest = tuple(x for x, _ in base)
        if digest in used:
            arms.append({"representative": g, "lifted": len(base), "status": "duplicate_coset"})
            continue
        used.add(digest)
        for kind, chosen in (("coset", base), ("small_x", small_base(len(base), v))):
            t = time.perf_counter()
            result = screen_base(chosen, v, args.seed, args.setup, args.target)
            arms.append({"representative": g, "kind": kind, "lifted": len(chosen),
                         "status": "complete", "elapsed_seconds": time.perf_counter() - t, **result})
    print(json.dumps({"schema": "eccp131-coset-screen/v1", "seed": args.seed,
                      "parameters": {k: hex(val) for k, val in v.items()},
                      "source_sha256": {str(path.relative_to(ROOT)): hashlib.sha256(path.read_bytes()).hexdigest()
                                        for path in (CURVE_SOURCE, TARGET_SOURCE)},
                      "kummer": {"v2_p_minus_1": 1, "zeta5": hex(zeta),
                                  "factorization_p_minus_1": [2, 5, 155003179783434785924857641481313994241]},
                      "arms": arms, "elapsed_seconds": time.perf_counter() - start},
                     separators=(",", ":")))


if __name__ == "__main__":
    main()
