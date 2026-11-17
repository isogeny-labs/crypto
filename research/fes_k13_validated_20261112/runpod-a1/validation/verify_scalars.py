#!/usr/bin/env python3
"""Validator-written scalar check for session ECBS1h695ef8e76c34 (no repository code).

For every record of the session this re-checks [d]G = Q on the curve, with
field and curve arithmetic written here from the textbook formulas:

  * curve  K_0 / F_(2^13):  y^2 + x*y = x^3 + a*x^2 + 1, a = 0 (the record's
    curve_spec {"kind": "koblitz", "a": 0, "n": 13});
  * the reduction polynomial is NOT taken from the repository: every
    irreducible degree-13 polynomial over F_2 is enumerated, and the ones on
    which the generator and every target lie on the curve with [r]G = O and
    [r]Q = O are kept (the check below requires exactly one survivor);
  * #E(F_(2^13)) is recounted by the trace criterion and must equal the
    record's group_order, r must be prime and divide it.

Usage:  python3 -I verify_scalars.py <session_dir> <out.json>
"""
import hashlib
import json
import sys


def deg(p):
    return p.bit_length() - 1


def pmod(a, f):
    df = deg(f)
    while a and deg(a) >= df:
        a ^= f << (deg(a) - df)
    return a


def clmul(a, b):
    r = 0
    while b:
        if b & 1:
            r ^= a
        a <<= 1
        b >>= 1
    return r


def pgcd(a, b):
    while b:
        a, b = b, pmod(a, b)
    return a


def is_irreducible(f):
    """Rabin's test for degree n: x^(2^n) = x mod f and gcd(x^(2^(n/q)) - x, f) = 1 for prime q | n."""
    n = deg(f)
    def x_pow_2k(k):
        t = 2  # the polynomial x
        for _ in range(k):
            t = pmod(clmul(t, t), f)
        return t
    if x_pow_2k(n) != 2:
        return False
    qs = [q for q in range(2, n + 1) if n % q == 0 and all(q % s for s in range(2, q))]
    for q in qs:
        if pgcd(f, x_pow_2k(n // q) ^ 2) != 1:
            return False
    return True


class Field:
    def __init__(self, f):
        self.f = f
        self.n = deg(f)

    def mul(self, a, b):
        return pmod(clmul(a, b), self.f)

    def sqr(self, a):
        return self.mul(a, a)

    def inv(self, a):
        if a == 0:
            raise ZeroDivisionError
        # a^(2^n - 2)
        e = (1 << self.n) - 2
        r, b = 1, a
        while e:
            if e & 1:
                r = self.mul(r, b)
            b = self.sqr(b)
            e >>= 1
        return r

    def trace(self, a):
        t, s = 0, a
        for _ in range(self.n):
            t ^= s
            s = self.sqr(s)
        assert t in (0, 1)
        return t


class Curve:
    """y^2 + x y = x^3 + a2 x^2 + a6 over F, points (x, y) or None for O."""

    def __init__(self, F, a2, a6):
        self.F, self.a2, self.a6 = F, a2, a6

    def on_curve(self, P):
        if P is None:
            return True
        F = self.F
        x, y = P
        lhs = F.sqr(y) ^ F.mul(x, y)
        rhs = F.mul(F.sqr(x), x) ^ F.mul(self.a2, F.sqr(x)) ^ self.a6
        return lhs == rhs

    def neg(self, P):
        return None if P is None else (P[0], P[0] ^ P[1])

    def add(self, P, Q):
        F = self.F
        if P is None:
            return Q
        if Q is None:
            return P
        x1, y1 = P
        x2, y2 = Q
        if x1 == x2:
            if y1 ^ y2 == x1:  # Q = -P (covers x1 = 0, y doubled)
                return None
            return self.dbl(P)
        lam = F.mul(y1 ^ y2, F.inv(x1 ^ x2))
        x3 = F.sqr(lam) ^ lam ^ x1 ^ x2 ^ self.a2
        y3 = F.mul(lam, x1 ^ x3) ^ x3 ^ y1
        return (x3, y3)

    def dbl(self, P):
        F = self.F
        if P is None:
            return None
        x1, y1 = P
        if x1 == 0:
            return None
        lam = x1 ^ F.mul(y1, F.inv(x1))
        x3 = F.sqr(lam) ^ lam ^ self.a2
        y3 = F.sqr(x1) ^ F.mul(lam ^ 1, x3)
        return (x3, y3)

    def mul(self, k, P):
        if k < 0:
            return self.mul(-k, self.neg(P))
        R = None
        for bit in bin(k)[2:]:
            R = self.dbl(R)
            if bit == "1":
                R = self.add(R, P)
        return R

    def count_points(self):
        """#E by the trace criterion: x = 0 gives y^2 = a6 (one y); x != 0 has two y iff
        Tr((x^3 + a2 x^2 + a6) / x^2) = 0 (substitute y = x z: z^2 + z = c)."""
        F = self.F
        n = 1  # O
        n += 1  # x = 0: y = sqrt(a6), unique
        for x in range(1, 1 << F.n):
            c = F.mul(F.mul(F.sqr(x), x) ^ F.mul(self.a2, F.sqr(x)) ^ self.a6, F.inv(F.sqr(x)))
            if F.trace(c) == 0:
                n += 2
        return n


def is_prime(m):
    if m < 2:
        return False
    i = 2
    while i * i <= m:
        if m % i == 0:
            return False
        i += 1
    return True


def h2i(s):
    return int(s, 16)


def main():
    sdir, out = sys.argv[1], sys.argv[2]
    recs_path = f"{sdir}/records.jsonl"
    raw = open(recs_path, "rb").read()
    records = [json.loads(l) for l in raw.decode().splitlines() if l.strip()]
    plan = json.load(open(f"{sdir}/plan.json"))
    plan_targets = {w["workload_id"]: (h2i(w["target"][0]), h2i(w["target"][1])) for w in plan["workloads"]}

    curves = {json.dumps(r["workload"]["curve"], sort_keys=True) for r in records}
    assert len(curves) == 1, "more than one curve in the session"
    cv = records[0]["workload"]["curve"]
    spec = records[0]["workload"]["curve_spec"]
    assert spec == {"kind": "koblitz", "a": 0, "n": 13}, spec
    n = cv["field_degree"]
    order, r, cof = cv["group_order"], cv["r"], cv["cofactor"]
    G = (h2i(cv["generator"][0]), h2i(cv["generator"][1]))
    a2, a6 = spec["a"], 1
    targets = set(plan_targets.values()) | {(h2i(x["workload"]["target"][0]), h2i(x["workload"]["target"][1])) for x in records}

    # Determine the field from the data, not from repository code.
    survivors = []
    irreducibles = 0
    for low in range(0, 1 << n):
        f = (1 << n) | low
        if not (f & 1):
            continue
        if not is_irreducible(f):
            continue
        irreducibles += 1
        E = Curve(Field(f), a2, a6)
        if not E.on_curve(G):
            continue
        if not all(E.on_curve(Q) for Q in targets):
            continue
        if E.mul(r, G) is not None:
            continue
        survivors.append(f)
    result = {
        "schema": "validator-scalar-check/v1",
        "tool": "research/fes_k13_validated_20261112/runpod-a1/validation/verify_scalars.py (validator-written, no repository imports)",
        "records_path": recs_path,
        "records_sha256": hashlib.sha256(raw).hexdigest(),
        "curve": {"equation": "y^2 + x*y = x^3 + %d*x^2 + 1 over F_(2^%d)" % (a2, n), "slug": cv["slug"],
                   "generator": cv["generator"], "group_order": order, "r": r, "cofactor": cof},
        "irreducible_degree13_polys_tested": irreducibles,
        "field_polys_consistent_with_data": [hex(f) for f in survivors],
    }
    ok = True
    problems = []
    if len(survivors) != 1:
        ok = False
        problems.append(f"expected exactly one consistent reduction polynomial, got {len(survivors)}")
    f = survivors[0] if survivors else None
    E = Curve(Field(f), a2, a6) if f else None
    if E:
        npts = E.count_points()
        result["recounted_group_order"] = npts
        result["r_is_prime"] = is_prime(r)
        checks = {
            "group_order_matches": npts == order,
            "order_eq_cofactor_times_r": order == cof * r,
            "r_prime": is_prime(r),
            "G_on_curve": E.on_curve(G),
            "G_not_identity": G is not None,
            "rG_is_O": E.mul(r, G) is None,
            "all_targets_on_curve": all(E.on_curve(Q) for Q in targets),
            "all_targets_in_r_subgroup": all(E.mul(r, Q) is None for Q in targets),
        }
        result["curve_checks"] = checks
        if not all(checks.values()):
            ok = False
            problems.append(f"curve checks failed: {[k for k, v in checks.items() if not v]}")
        per = []
        for x in records:
            Q = (h2i(x["workload"]["target"][0]), h2i(x["workload"]["target"][1]))
            rec = x["outcome"].get("recovered")
            entry = {"seq": x["seq"], "record_id": x["record_id"], "arm": x["arm"], "workload_id": x["workload"]["workload_id"],
                     "round": x["round"], "warmup": x["warmup"], "status": x["outcome"]["status"], "recovered": rec}
            good = True
            why = []
            if plan_targets.get(x["workload"]["workload_id"]) != Q:
                good = False
                why.append("target differs from plan.json")
            if rec is None:
                good = False
                why.append("no recovered scalar")
            else:
                d = int(rec)
                if not (0 <= d < r):
                    why.append("recovered scalar outside [0, r)")
                    good = False
                if E.mul(d, G) != Q:
                    good = False
                    why.append("[d]G != Q")
            if x["outcome"]["status"] != "verified" or x["outcome"].get("matches_target") is not True:
                good = False
                why.append("record does not claim verified/matches_target")
            entry["dG_eq_Q"] = good
            if why:
                entry["problems"] = why
                problems.append(f"seq {x['seq']}: {why}")
                ok = False
            per.append(entry)
        result["records"] = len(records)
        result["records_checked"] = len(per)
        result["records_dG_eq_Q"] = sum(1 for e in per if e["dG_eq_Q"])
        result["distinct_targets"] = len(targets)
        dl = {}
        for e in per:
            dl.setdefault(e["workload_id"], set()).add(e["recovered"])
        result["recovered_scalar_by_workload"] = {k: sorted(v) for k, v in sorted(dl.items())}
        result["one_scalar_per_workload"] = all(len(v) == 1 for v in dl.values())
        if not result["one_scalar_per_workload"]:
            ok = False
            problems.append("arms disagree on a workload's logarithm")
        result["per_record"] = per
    result["ok"] = ok
    result["problems"] = problems
    json.dump(result, open(out, "w"), indent=1, sort_keys=False)
    open(out, "a").write("\n")
    print(json.dumps({k: result[k] for k in result if k not in ("per_record",)}, indent=1))
    sys.exit(0 if ok else 1)


if __name__ == "__main__":
    main()
