#!/usr/bin/env python3
"""Independently certify orders in the unpromoted full-tree prime-curve delta.

For each model, a point of order d with d > 4 sqrt(p) makes the claimed
group order unique inside Hasse's interval. This script checks the finite
point arithmetic and records a compact certificate; it does not check that
the candidate registry was faithfully extracted from every original source.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import math
from pathlib import Path

HERE = Path(__file__).resolve().parent
REPO = HERE.parents[2]
BASE = REPO / "docs/curves/registry.json"
RECEIPT = HERE / "source-scope-summary.json"


def require(ok: bool, message: str) -> None:
    if not ok:
        raise ValueError(message)


def prime(n: int) -> bool:
    if n < 2:
        return False
    if n % 2 == 0:
        return n == 2
    if n % 3 == 0:
        return n == 3
    d = 5
    while d * d <= n:
        if n % d == 0 or n % (d + 2) == 0:
            return False
        d += 6
    return True


def factors(n: int) -> list[int]:
    require(n > 0, "nonpositive order")
    result = []
    d = 2
    while d * d <= n:
        if n % d == 0:
            result.append(d)
            while n % d == 0:
                n //= d
        d = 3 if d == 2 else d + 2
    if n > 1:
        result.append(n)
    return result


def sqrt_mod(v: int, p: int) -> int | None:
    v %= p
    if v == 0:
        return 0
    if pow(v, (p - 1) // 2, p) != 1:
        return None
    if p % 4 == 3:
        return pow(v, (p + 1) // 4, p)
    q, s = p - 1, 0
    while q % 2 == 0:
        q //= 2
        s += 1
    z = 2
    while pow(z, (p - 1) // 2, p) != p - 1:
        z += 1
    m, c, t, r = s, pow(z, q, p), pow(v, q, p), pow(v, (q + 1) // 2, p)
    while t != 1:
        i, u = 1, t * t % p
        while i < m and u != 1:
            u = u * u % p
            i += 1
        require(i < m, "Tonelli-Shanks failed")
        b = pow(c, 1 << (m - i - 1), p)
        m, c, t, r = i, b * b % p, t * b * b % p, r * b % p
    require(r * r % p == v, "square root check failed")
    return r


def add(p: int, a: int, left: tuple[int, int] | None,
        right: tuple[int, int] | None) -> tuple[int, int] | None:
    if left is None:
        return right
    if right is None:
        return left
    x1, y1 = left
    x2, y2 = right
    if x1 == x2 and (y1 + y2) % p == 0:
        return None
    if x1 == x2:
        slope = (3 * x1 * x1 + a) * pow(2 * y1, -1, p) % p
    else:
        slope = (y2 - y1) * pow((x2 - x1) % p, -1, p) % p
    x3 = (slope * slope - x1 - x2) % p
    return (x3, (slope * (x1 - x3) - y1) % p)


def mul(p: int, a: int, k: int, point: tuple[int, int]) -> tuple[int, int] | None:
    require(k >= 0, "negative scalar")
    out = None
    while k:
        if k & 1:
            out = add(p, a, out, point)
        point = add(p, a, point, point)
        k >>= 1
    return out


def point_order(p: int, a: int, claimed: int,
                point: tuple[int, int], prime_factors: list[int]) -> int:
    require(mul(p, a, claimed, point) is None, "claimed order does not annihilate point")
    order = claimed
    for q in prime_factors:
        while order % q == 0 and mul(p, a, order // q, point) is None:
            order //= q
    require(mul(p, a, order, point) is None, "derived point order failed")
    for q in prime_factors:
        if order % q == 0:
            require(mul(p, a, order // q, point) is not None,
                    "derived point order is not minimal")
    return order


def certify(entry: dict) -> dict:
    p = int(entry["params"]["p"])
    a = int(entry["params"]["a"])
    b = int(entry["params"]["b"])
    claimed = int(entry["order"])
    require(prime(p), "field modulus is composite")
    require(p >= 5 and 0 <= a < p and 0 <= b < p, "invalid prime model")
    disc = (4 * a * a * a + 27 * b * b) % p
    require(disc != 0, "singular curve")
    require((claimed - (p + 1)) ** 2 <= 4 * p, "order outside Hasse interval")
    field = f"fp-{p}"
    model = json.dumps({"a": str(a), "b": str(b), "field": field,
                        "form": "y^2=x^3+a*x+b", "p": str(p), "v": "1"},
                       sort_keys=True, separators=(",", ":"), ensure_ascii=True)
    digest = hashlib.sha256(model.encode("ascii")).hexdigest()
    trace = p + 1 - claimed
    j = 1728 * 4 * pow(a, 3, p) * pow(disc, -1, p) % p
    expected_id = f"ICV1:{field}:{trace}:{claimed}:{j}:unk:unk:r:{digest[:12]}"
    trace_tag = f"tm{-trace}" if trace < 0 else f"t{trace}"
    expected_slug = f"icv1-fp{p.bit_length()}-{trace_tag}-{digest[:8]}"
    require(entry["model_json"] == model and entry["icv1"] == expected_id and
            entry["slug"] == expected_slug and entry["trace"] == trace and
            entry["j"] == str(j) and entry["end"] == "unk",
            "ICV1 identity differs from independently normalized model")
    if p <= 1000:
        exact = 1
        for x in range(p):
            v = (x * x * x + a * x + b) % p
            exact += 1 if v == 0 else 2 if pow(v, (p - 1) // 2, p) == 1 else 0
        require(exact == claimed, "full-field point count differs")
        return {"icv1": entry["icv1"], "status": "certified",
                "method": "full_field_count", "p": p, "a": a, "b": b,
                "claimed_order": claimed, "counted_order": exact,
                "first_source": entry["sources"][0]}
    fs = factors(claimed)
    start = int.from_bytes(hashlib.sha256(entry["icv1"].encode()).digest()[:8], "big") % p
    exponent = 1
    witnesses = []
    for offset in range(32):
        x = (start + offset) % p
        y = sqrt_mod((x * x * x + a * x + b) % p, p)
        if y is None:
            continue
        point = (x, y)
        require((y * y - x * x * x - a * x - b) % p == 0, "point off curve")
        d = point_order(p, a, claimed, point, fs)
        exponent = math.lcm(exponent, d)
        witnesses.append({"point": [x, y], "order": d})
        if exponent * exponent > 16 * p:
            return {"icv1": entry["icv1"], "status": "certified",
                    "method": "point_order_hasse", "p": p,
                    "a": a, "b": b, "claimed_order": claimed,
                    "claimed_order_prime_factors": fs, "witness_exponent": exponent,
                    "witnesses": witnesses, "first_source": entry["sources"][0]}
    return {"icv1": entry["icv1"], "status": "unresolved", "p": p,
            "a": a, "b": b, "claimed_order": claimed,
            "claimed_order_prime_factors": fs, "witness_exponent": exponent,
            "witnesses": witnesses, "first_source": entry["sources"][0]}


def selftest() -> None:
    for p in (5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43):
        for a, b in ((0, 1), (1, 1), (2, 3)):
            if (4 * a**3 + 27 * b**2) % p == 0:
                continue
            points = [(x, y) for x in range(p) for y in range(p)
                      if (y * y - x**3 - a * x - b) % p == 0]
            count = len(points) + 1
            require((count - (p + 1)) ** 2 <= 4 * p, "brute Hasse control failed")
            for point in points:
                require(mul(p, a, count, point) is None, "brute order control failed")
                require(mul(p, a, count + 1, point) == point, "off-by-one control failed")
                require(point_order(p, a, count, point, factors(count)) > 0,
                        "point-order control failed")
            for x in range(p):
                y = sqrt_mod((x**3 + a * x + b) % p, p)
                require((y is not None) == any(px == x for px, _ in points),
                        "square-root control failed")


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("candidate", type=Path)
    ap.add_argument("certificates", type=Path)
    args = ap.parse_args()
    selftest()
    source = args.candidate.read_bytes()
    receipt = json.loads(RECEIPT.read_text())
    require(hashlib.sha256(source).hexdigest() == receipt["candidate_registry_sha256"],
            "candidate SHA-256 differs from pinned scratch output")
    candidate = json.loads(source)["curves"]
    baseline = json.loads(BASE.read_text())["curves"]
    known = {entry["icv1"] for entry in baseline}
    new = [entry for entry in candidate if entry["icv1"] not in known]
    require((len(baseline), len(candidate), len(new)) ==
            (receipt["baseline_count"], receipt["candidate_count"], receipt["new_count"]),
            "candidate inventory differs from receipt")
    counts = {"certified": 0, "unresolved": 0, "rejected": 0}
    with args.certificates.open("w") as out:
        for entry in new:
            try:
                certificate = certify(entry)
            except (ValueError, KeyError, TypeError) as err:
                certificate = {"icv1": entry.get("icv1"), "status": "rejected",
                               "error": str(err), "first_source": entry.get("sources", [None])[0]}
            counts[certificate["status"]] += 1
            out.write(json.dumps(certificate, sort_keys=True) + "\n")
    summary = {"candidate_sha256": receipt["candidate_registry_sha256"],
               "certificates_sha256": hashlib.sha256(args.certificates.read_bytes()).hexdigest(),
               "counts": counts, "total": len(new), "small-field_selftest": "pass",
               "icv1_identities_recomputed": len(new)}
    print(json.dumps(summary, sort_keys=True))
    return 0 if counts["rejected"] == counts["unresolved"] == 0 else 1


if __name__ == "__main__":
    raise SystemExit(main())
