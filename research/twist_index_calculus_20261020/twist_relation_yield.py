#!/usr/bin/env python3
"""Exact small-field control for twist points in Semaev f3 relation searches.

No random choices or external algebra system are used.  The experiment is
deliberately about *relation yield*, not a complete discrete-log pipeline.
"""

import json
import platform
from collections import Counter


A = 2
B = 3
PRIMES = (101, 211, 233, 307)
BASE_X_PER_COLOR = 24


def legendre(v, p):
    if v % p == 0:
        return 0
    z = pow(v % p, (p - 1) // 2, p)
    return -1 if z == p - 1 else z


def roots_table(p):
    roots = {}
    for y in range(p):
        roots.setdefault(y * y % p, []).append(y)
    return roots


def curve_rhs(x, p):
    return (x * x * x + A * x + B) % p


def f3(x1, x2, x3, p):
    """Semaev's third summation polynomial for y^2=x^3+A*x+B."""
    return (
        (x1 - x2) ** 2 * x3 ** 2
        - 2 * ((x1 + x2) * (x1 * x2 + A) + 2 * B) * x3
        + (x1 * x2 - A) ** 2
        - 4 * B * (x1 + x2)
    ) % p


def add_points(P, Q, p, d):
    """Group law on d*y^2=x^3+A*x+B; None denotes infinity."""
    if P is None:
        return Q
    if Q is None:
        return P
    x1, y1 = P
    x2, y2 = Q
    if x1 == x2 and (y1 + y2) % p == 0:
        return None
    if P == Q:
        slope = (3 * x1 * x1 + A) * pow(2 * d * y1, -1, p) % p
    else:
        slope = (y2 - y1) * pow((x2 - x1) % p, -1, p) % p
    x3 = (d * slope * slope - x1 - x2) % p
    y3 = (slope * (x1 - x3) - y1) % p
    assert d * y3 * y3 % p == curve_rhs(x3, p)
    return (x3, y3)


def direct_lift_exists(x1, x2, target_x, p, d, roots, inv_d):
    """Independently verify that some signed pair sums to ±target."""
    rhs1 = curve_rhs(x1, p) * inv_d % p
    rhs2 = curve_rhs(x2, p) * inv_d % p
    for y1 in roots.get(rhs1, []):
        for y2 in roots.get(rhs2, []):
            total = add_points((x1, y1), (x2, y2), p, d)
            if total is not None and total[0] == target_x:
                return True
    return False


def check_f3_from_group_law(xs, p, d, roots):
    """Check the converse direction independently on every base pair."""
    inv_d = pow(d, -1, p)
    checked = 0
    for x1 in xs:
        for x2 in xs:
            for y1 in roots[curve_rhs(x1, p) * inv_d % p]:
                for y2 in roots[curve_rhs(x2, p) * inv_d % p]:
                    total = add_points((x1, y1), (x2, y2), p, d)
                    if total is not None:
                        assert f3(x1, x2, total[0], p) == 0
                        checked += 1
    return checked


def fp2_mul(x, y, p, d):
    return ((x[0] * y[0] + d * x[1] * y[1]) % p,
            (x[0] * y[1] + x[1] * y[0]) % p)


def extension_order_bruteforce(p, d):
    """Count E(F_{p^2}) via all p^2 abscissas and the norm character."""
    order = 1
    for a in range(p):
        for b in range(p):
            x = (a, b)
            x2 = fp2_mul(x, x, p, d)
            x3 = fp2_mul(x2, x, p, d)
            f0 = (x3[0] + A * a + B) % p
            f1 = (x3[1] + A * b) % p
            if f0 == 0 and f1 == 0:
                order += 1
            else:
                norm = (f0 * f0 - d * f1 * f1) % p
                assert norm != 0
                order += 1 + legendre(norm, p)
    return order


def twist_padding_control(e_xs, t_xs, two_torsion_xs, p, d, roots):
    """Count twist pairs that vanish or reduce to known rational 2-torsion."""
    inv_d = pow(d, -1, p)
    zero_pairs = set()
    torsion_pairs = {x: set() for x in two_torsion_xs}
    for i, x1 in enumerate(t_xs):
        for x2 in t_xs[i:]:
            for y1 in roots[curve_rhs(x1, p) * inv_d % p]:
                for y2 in roots[curve_rhs(x2, p) * inv_d % p]:
                    total = add_points((x1, y1), (x2, y2), p, d)
                    if total is None:
                        zero_pairs.add((x1, x2))
                    elif total[1] == 0 and total[0] in torsion_pairs:
                        torsion_pairs[total[0]].add((x1, x2))
    shifted_targets = {}
    for tx, pairs in torsion_pairs.items():
        shifted = set()
        if pairs:
            for ex in e_xs:
                ey = roots[curve_rhs(ex, p)][0]
                point = add_points((ex, ey), (tx, 0), p, 1)
                assert point is not None
                shifted.add(point[0])
        shifted_targets[str(tx)] = {
            "twist_abscissa_pairs_summing_to_torsion": len(pairs),
            "shifted_E_abscissas": len(shifted),
            "shifted_E_abscissas_outside_original_base": len(shifted - set(e_xs)),
        }
    return {"twist_abscissa_pairs_summing_to_zero": len(zero_pairs),
            "nonzero_two_torsion_shifts": shifted_targets}


def pair_counts(target_xs, e_xs, t_xs, p, target_color, d, roots):
    colored = [(x, "E") for x in e_xs] + [(x, "T") for x in t_xs]
    counts = Counter()
    covered = {"E-E": 0, "E-T": 0, "T-T": 0}
    direct_checks = 0
    for target_x in target_xs:
        this = Counter()
        for i, (x1, c1) in enumerate(colored):
            for x2, c2 in colored[i:]:
                if f3(x1, x2, target_x, p) != 0:
                    continue
                label = "-".join(sorted((c1, c2)))
                this[label] += 1
                if c1 == c2 == target_color:
                    twist_d = 1 if target_color == "E" else d
                    inv_d = 1 if target_color == "E" else pow(d, -1, p)
                    assert direct_lift_exists(x1, x2, target_x, p,
                                              twist_d, roots, inv_d)
                    direct_checks += 1
                else:
                    # With no rational 2-torsion in this factor base, a
                    # mixed-color pair cannot equal a pure-color target.
                    raise AssertionError((p, target_color, x1, x2, target_x))
        counts.update(this)
        for label in covered:
            covered[label] += bool(this[label])
    return {"polynomial_hits": dict(counts),
            "target_abscissas_with_hit": covered,
            "direct_group_law_checks": direct_checks,
            "target_abscissas": len(target_xs)}


def experiment(p):
    assert (4 * A ** 3 + 27 * B ** 2) % p != 0
    d = next(z for z in range(2, p) if legendre(z, p) == -1)
    roots = roots_table(p)
    inv_d = pow(d, -1, p)
    e_all, t_all, shared_two_torsion = [], [], []
    order_e = order_t = 1
    for x in range(p):
        fx = curve_rhs(x, p)
        order_e += len(roots.get(fx, []))
        order_t += len(roots.get(fx * inv_d % p, []))
        if fx == 0:
            shared_two_torsion.append(x)
        elif legendre(fx, p) == 1:
            e_all.append(x)
        else:
            t_all.append(x)
    assert len(e_all) >= BASE_X_PER_COLOR
    assert len(t_all) >= BASE_X_PER_COLOR
    assert order_e + order_t == 2 * (p + 1)
    e_xs = e_all[:BASE_X_PER_COLOR]
    t_xs = t_all[:BASE_X_PER_COLOR]
    e_forward = check_f3_from_group_law(e_xs, p, 1, roots)
    t_forward = check_f3_from_group_law(t_xs, p, d, roots)
    n2 = extension_order_bruteforce(p, d)
    assert n2 == order_e * order_t
    e_counts = pair_counts(e_all, e_xs, t_xs, p, "E", d, roots)
    t_counts = pair_counts(t_all, e_xs, t_xs, p, "T", d, roots)
    padding = twist_padding_control(e_xs, t_xs, shared_two_torsion,
                                    p, d, roots)
    assert set(e_counts["polynomial_hits"]) <= {"E-E"}
    assert set(t_counts["polynomial_hits"]) <= {"T-T"}
    b = BASE_X_PER_COLOR
    return {
        "p": p, "a": A, "b": B, "nonsquare_d": d,
        "models": {"E": "y^2=x^3+a*x+b", "twist": "d*y^2=x^3+a*x+b"},
        "orders": {"E_Fp": order_e, "twist_Fp": order_t,
                   "E_Fp2_enumerated": n2, "order_product": order_e * order_t},
        "abscissas": {"E_non_two_torsion": len(e_all),
                      "twist_non_two_torsion": len(t_all),
                      "shared_two_torsion": shared_two_torsion},
        "factor_base": {"x_per_color": b, "E_x": e_xs, "twist_x": t_xs,
                        "E_only_candidate_pairs": b * (b + 1) // 2,
                        "augmented_candidate_pairs": (2 * b) * (2 * b + 1) // 2},
        "E_targets": e_counts, "twist_targets_control": t_counts,
        "three_factor_padding_control": padding,
        "forward_group_law_to_f3_checks": {"E": e_forward, "twist": t_forward},
        "validated": True,
    }


def main():
    output = {
        "protocol": "exact exhaustive f3 on all non-2-torsion target abscissas;"
                    " first 24 sorted abscissas per color; unordered pairs"
                    " with repetition; no timing claim",
        "python": platform.python_version(),
        "cases": [experiment(p) for p in PRIMES],
    }
    print(json.dumps(output, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
