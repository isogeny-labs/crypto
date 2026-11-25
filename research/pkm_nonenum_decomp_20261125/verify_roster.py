#!/usr/bin/env python3
"""Independently replay the frozen PKM curve and target roster with stdlib math."""

import argparse
import hashlib
import json
from pathlib import Path


def require(condition, message):
    if not condition:
        raise ValueError(message)


def is_prime(n):
    if n < 2:
        return False
    if n % 2 == 0:
        return n == 2
    d = 3
    while d * d <= n:
        if n % d == 0:
            return False
        d += 2
    return True


def selected_primes(t):
    width = 1 << t
    start = (width**3 + 5) // 6
    found = []
    p = start
    while len(found) < 3:
        if p % width == 1 and is_prime(p):
            found.append(p)
        p += 1
    return start, found


def curve_order(p, a, b, root_counts):
    return 1 + sum(root_counts[(pow(x, 3, p) + a * x + b) % p] for x in range(p))


def first_curve(p):
    root_counts = [0] * p
    least_roots = [None] * p
    for y in range(p):
        square = y * y % p
        root_counts[square] += 1
        if least_roots[square] is None:
            least_roots[square] = y
    rejected = 0
    for a in range(1, p):
        for b in range(1, p):
            discriminant = (4 * pow(a, 3, p) + 27 * b * b) % p
            if discriminant != 0:
                r = curve_order(p, a, b, root_counts)
                if r != p and is_prime(r):
                    generator = next((x, least_roots[(pow(x, 3, p) + a * x + b) % p])
                                     for x in range(p)
                                     if least_roots[(pow(x, 3, p) + a * x + b) % p] is not None)
                    return a, b, r, generator, rejected, least_roots
            rejected += 1
    raise ValueError(f"no eligible curve at p={p}")


def add(left, right, a, p):
    if left is None:
        return right
    if right is None:
        return left
    x1, y1 = left
    x2, y2 = right
    if x1 == x2 and (y1 + y2) % p == 0:
        return None
    if x1 == x2:
        numerator, denominator = 3 * x1 * x1 + a, 2 * y1
    else:
        numerator, denominator = y2 - y1, x2 - x1
    slope = numerator * pow(denominator % p, -1, p) % p
    x3 = (slope * slope - x1 - x2) % p
    return x3, (slope * (x1 - x3) - y1) % p


def multiply(k, point, a, p):
    result = None
    while k:
        if k & 1:
            result = add(result, point, a, p)
        point = add(point, point, a, p)
        k >>= 1
    return result


def on_curve(point, p, a, b):
    x, y = point
    return 0 <= x < p and 0 <= y < p and (y * y - x * x * x - a * x - b) % p == 0


def first_zeta(p, width):
    for candidate in range(2, p):
        root = pow(candidate, (p - 1) // width, p)
        if pow(root, width // 2, p) != 1:
            return root
    raise ValueError(f"no root of order {width} at p={p}")


def factor_base(p, a, b, width, zeta, least_roots):
    xs = sorted({pow(zeta, j, p) for j in range(width)})
    points = []
    for x in xs:
        y = least_roots[(pow(x, 3, p) + a * x + b) % p]
        if y is not None:
            points.append((x, y))
            if y:
                points.append((x, p - y))
    return xs, sorted(points)


def target_scalar(t, curve_index, target_index, used, r):
    counter = 0
    while True:
        source = f"pkm-native-f4-v1|{t}|{curve_index}|{target_index}|{counter}".encode()
        digest = hashlib.sha256(source).digest()
        q = 1 + int.from_bytes(digest, "big") % (r - 1)
        if q not in used:
            used.add(q)
            return counter, digest.hex(), q
        counter += 1


def verify(roster):
    require(roster.get("schema") == "pkm-native-f4-roster/v1", "roster schema")
    require(roster.get("selection") == "ROSTER_SELECTION_20261125.md", "selection document")
    require(roster.get("curve_index_base") == 0, "curve index base")
    curves = roster.get("curves")
    require(isinstance(curves, list) and len(curves) == 9, "expected 3 curves at each of 3 heights")
    checked = []
    for t in (3, 4, 5):
        p_min, primes = selected_primes(t)
        width = 1 << t
        for index, p in enumerate(primes):
            row = curves[(t - 3) * 3 + index]
            label = f"t={t} curve={index}"
            require(row.get("t") == t and row.get("curve_index") == index, f"{label}: index")
            require(row.get("p_min") == p_min and row.get("p") == p, f"{label}: prime selection")
            a, b, r, generator, rejected, roots = first_curve(p)
            require((row.get("a"), row.get("b"), row.get("r")) == (a, b, r), f"{label}: curve/order")
            require(row.get("group_order") == r and row.get("rejected_pairs") == rejected,
                    f"{label}: group order or rejection count")
            require(row.get("generator") == list(generator), f"{label}: generator")
            require(on_curve(generator, p, a, b) and multiply(r, generator, a, p) is None and
                    multiply(r - 1, generator, a, p) is not None, f"{label}: generator order")
            require(row.get("v2_p_minus_1") == ((p - 1) & -(p - 1)).bit_length() - 1,
                    f"{label}: 2-adic valuation")
            zeta = first_zeta(p, width)
            require(row.get("g") == 1 and row.get("map_degree") == width and row.get("zeta") == zeta,
                    f"{label}: map root")
            xs, points = factor_base(p, a, b, width, zeta, roots)
            require(row.get("kummer_abscissae") == xs, f"{label}: abscissae")
            require(row.get("factor_base_points") == [list(point) for point in points],
                    f"{label}: factor-base points")
            require(all(on_curve(point, p, a, b) for point in points), f"{label}: base point off curve")
            targets = row.get("targets")
            require(isinstance(targets, list) and len(targets) == 16, f"{label}: target count")
            used = set()
            for j, target in enumerate(targets):
                counter, digest, scalar = target_scalar(t, index, j, used, r)
                seed = hashlib.sha256(f"pkm-native-f4-relations-v1|{t}|{index}|{j}".encode()).hexdigest()
                require(target.get("target_index") == j and target.get("counter") == counter and
                        target.get("digest_sha256") == digest and target.get("scalar") == scalar and
                        target.get("relation_seed_sha256") == seed, f"{label} target={j}: hash/scalar")
                point = tuple(target.get("point", []))
                require(on_curve(point, p, a, b) and multiply(scalar, generator, a, p) == point,
                        f"{label} target={j}: point relation")
            checked.append({"t": t, "curve_index": index, "p": p, "r": r,
                            "base_abscissae": len(xs), "base_points": len(points),
                            "targets_checked": len(targets)})
    return checked


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("roster", type=Path)
    args = parser.parse_args()
    data = args.roster.read_bytes()
    rows = verify(json.loads(data))
    print(json.dumps({"status": "pass", "roster_sha256": hashlib.sha256(data).hexdigest(),
                      "verifier_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
                      "curves_checked": rows}, sort_keys=True))


if __name__ == "__main__":
    main()
