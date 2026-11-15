#!/usr/bin/env python3
"""Decomposability certificate for m = 2 rows (Amendment R4 draft, edit E4.3).

For each row it counts the ordered pairs (x1, x2) in V x V with S3(x1, x2, x_R) = 0. This is the same count as
research/pkm_tower_pilot_20260924/verify.py (m = 2 branch), and it imports that file's `s3_coeffs`. Instead of
enumerating |V|^2 pairs it solves, for each x1 in V, the quadratic S3(x1, x_R, U) = 0 in U over F_p (S3 is symmetric)
and counts the roots that lie in V. So it is exact and O(|V|). A row with count 0 is certified inconsistent (no
decomposition on the grid). A row with count > 0 is decomposable.

    python3 certify_m2.py OUT.jsonl ROWS.jsonl...      (writes one record per m = 2 tower row; exit 0)
    python3 certify_m2.py --selftest ROWS.jsonl...     (compares with verify.py's brute force on every row; exit 1 on any mismatch)
"""
import importlib.util, json, os, sys

_here = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location(
    "verify", os.path.join(_here, "../../pkm_tower_pilot_20260924/verify.py"))
verify = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(verify)


def sqrt_mod(n, p):
    """A square root of n mod an odd prime p (Tonelli-Shanks), or None."""
    n %= p
    if n == 0:
        return 0
    if pow(n, (p - 1) // 2, p) != 1:
        return None
    q, s = p - 1, 0
    while q % 2 == 0:
        q //= 2
        s += 1
    z = 2
    while pow(z, (p - 1) // 2, p) != p - 1:
        z += 1
    m, c, t, r = s, pow(z, q, p), pow(n, q, p), pow(n, (q + 1) // 2, p)
    while t != 1:
        i, t2 = 0, t
        while t2 != 1:
            t2 = t2 * t2 % p
            i += 1
        b = pow(c, 1 << (m - i - 1), p)
        m, c, t, r = i, b * b % p, t * b * b % p, r * b % p
    return r


def roots(A, B, C, p):
    """Distinct roots in F_p of A U^2 + B U + C, or 'all' for the zero polynomial."""
    if A == 0:
        if B == 0:
            return "all" if C == 0 else []
        return [(-C) * pow(B, p - 2, p) % p]
    disc = (B * B - 4 * A * C) % p
    r = sqrt_mod(disc, p)
    if r is None:
        return []
    inv = pow(2 * A, p - 2, p)
    return sorted({(-B + r) * inv % p, (-B - r) * inv % p})


def count_fast(row, v):
    p, a, b, xr = row["p"], row["curve"]["a"], row["curve"]["b"], row["x_r"]
    vs = set(v)
    n = 0
    for x1 in v:
        A, B, C = verify.s3_coeffs(x1, xr, a, b, p)
        rs = roots(A, B, C, p)
        n += len(v) if rs == "all" else sum(1 for u in rs if u in vs)
    return n


def rows(paths):
    for path in paths:
        for line in open(path):
            r = json.loads(line)
            if "summary" in r or "t" not in r or r.get("m") != 2 or r.get("control") != "tower":
                continue
            yield path, r


def main(argv):
    if argv[:1] == ["--selftest"]:
        bad = n = 0
        for path, r in rows(argv[1:]):
            v = verify.tower_v(r)
            fast, brute = count_fast(r, v), verify.count(r, v)
            n += 1
            if fast != brute:
                bad += 1
                print("MISMATCH", path, r["kind"], r["t"], r["target"], r["target_index"], fast, brute)
        print(f"selftest: {n} rows, {bad} mismatches")
        return 1 if bad or n == 0 else 0
    out = open(argv[0], "w")
    for path, r in rows(argv[1:]):
        v = verify.tower_v(r)
        n = count_fast(r, v)
        out.write(json.dumps({"file": os.path.basename(path), "kind": r["kind"], "p": r["p"], "t": r["t"],
                              "target": r["target"], "target_index": r["target_index"], "x_r": r["x_r"],
                              "curve": r["curve"], "solutions": n, "certified_inconsistent": n == 0}) + "\n")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
