#!/usr/bin/env python3
"""Independent check of the solved DLP scalars of the jv_cover runs (R2-D1, R2-D2).

Written by the validator; shares no code with the repository.  Own arithmetic
for F_p, F_{p^2} = F_p[i]/(i^2 - w), F_{p^6} = F_{p^2}[t]/(t^3 - s), and the
curve y^2 = x^3 + a2 x^2 + a4 x over F_{p^6} (affine group law, inversion by
Fermat).  Inputs:

  * the producer's dumps, read from commit --dump-commit with `git show`
    (validation/scalar-dump/instances-{sparse,fast}.jsonl, dlp-{sparse,fast}.json);
  * the recorded R2-D1 / R2-D2 rows, read from the snapshot --snapshot;
  * the validator's own dump-binary runs (validation/replay/out/dump_*), if present.

For every dumped instance:
  tower: w is a non-residue mod p (Euler, and no root of i^2 - w in F_p by
         exhaustion); s is a non-cube in F_{p^2} (s^((p^2-1)/3) != 1, and no
         root of t^3 - s in F_{p^2} by exhaustion), so both extensions are fields;
  curve: non-singular (16 a4^2 (a2^2 - 4 a4) != 0); G and Q on it;
  order: l prime (deterministic Miller-Rabin and trial division), G != O, l*G = O;
  log:   dd*G = Q and dd = d (and 0 < d < l);
  extra (reported, not gating): (4 l) R = O for random points R (cofactor 4),
         and 4 l inside the Hasse interval.
Self-tests of the arithmetic and negative controls (a residue w, a cube s,
(dd+1) G != Q, a perturbed G off the curve, a composite l) must behave as
expected, or the verifier fails.

Ties: every non-timing field of each dlp-*.json row equals the R2-D1/R2-D2
row; the dumped (p, seed, l) equal the row's; per (p, seed) the dumped
instance is identical across arms; the validator's own dump runs give
byte-identical instance lines and rows equal to R2 in every non-timing field.

    python3 -I verify_scalars.py --snapshot e9bf8737a3... --dump-commit ca7dc711fa... --out scalar-verification.json
"""
import argparse
import hashlib
import json
import random
import subprocess
from pathlib import Path

REPO = Path("/home/user/crypto")
STUDY = "research/postgb_linear_stage_20261026"
DUMP = f"{STUDY}/validation/scalar-dump"
OWN = REPO / STUDY / "validation/replay"
TIMING = {"f4_ms", "lin_ms", "wall_ms"}


def git_show(commit, rel) -> bytes:
    return subprocess.run(["git", "show", f"{commit}:{rel}"], cwd=REPO, capture_output=True, check=True).stdout


# ---------------------------------------------------------------- fields ---

class F2:
    """F_p[i]/(i^2 - w); elements (a, b) = a + b i."""

    def __init__(self, p, w):
        self.p, self.w = p, w % p
        self.zero, self.one = (0, 0), (1, 0)

    def add(self, x, y):
        p = self.p
        return ((x[0] + y[0]) % p, (x[1] + y[1]) % p)

    def sub(self, x, y):
        p = self.p
        return ((x[0] - y[0]) % p, (x[1] - y[1]) % p)

    def neg(self, x):
        return ((-x[0]) % self.p, (-x[1]) % self.p)

    def mul(self, x, y):
        p = self.p
        return ((x[0] * y[0] + self.w * x[1] * y[1]) % p, (x[0] * y[1] + x[1] * y[0]) % p)

    def scal(self, k, x):
        return ((k * x[0]) % self.p, (k * x[1]) % self.p)

    def pow(self, x, e):
        r = self.one
        while e:
            if e & 1:
                r = self.mul(r, x)
            x = self.mul(x, x)
            e >>= 1
        return r

    def elems(self):
        p = self.p
        for a in range(p):
            for b in range(p):
                yield (a, b)


class F6:
    """F2[t]/(t^3 - s); elements (c0, c1, c2) = c0 + c1 t + c2 t^2, ci in F2."""

    def __init__(self, f2: F2, s):
        self.f, self.s = f2, s
        self.zero = (f2.zero, f2.zero, f2.zero)
        self.one = (f2.one, f2.zero, f2.zero)
        self.q = f2.p ** 6

    def add(self, x, y):
        f = self.f
        return tuple(f.add(a, b) for a, b in zip(x, y))

    def sub(self, x, y):
        f = self.f
        return tuple(f.sub(a, b) for a, b in zip(x, y))

    def neg(self, x):
        return tuple(self.f.neg(a) for a in x)

    def mul(self, x, y):
        f = self.f
        c = [f.zero] * 5
        for i in range(3):
            for j in range(3):
                c[i + j] = f.add(c[i + j], f.mul(x[i], y[j]))
        # t^3 = s, t^4 = s t
        return (f.add(c[0], f.mul(self.s, c[3])), f.add(c[1], f.mul(self.s, c[4])), c[2])

    def scal(self, k, x):
        return tuple(self.f.scal(k, a) for a in x)

    def pow(self, x, e):
        r = self.one
        while e:
            if e & 1:
                r = self.mul(r, x)
            x = self.mul(x, x)
            e >>= 1
        return r

    def inv(self, x):
        if x == self.zero:
            raise ZeroDivisionError("inverse of 0 in F_{p^6}")
        return self.pow(x, self.q - 2)

    def rand(self, rng):
        p = self.f.p
        return tuple((rng.randrange(p), rng.randrange(p)) for _ in range(3))

    def sqrt(self, a, rng):
        """Tonelli-Shanks in F_{p^6}; None if a is not a square."""
        q = self.q
        if a == self.zero:
            return self.zero
        if self.pow(a, (q - 1) // 2) != self.one:
            return None
        e, m = 0, q - 1
        while m % 2 == 0:
            e, m = e + 1, m // 2
        while True:
            z = self.rand(rng)
            if z != self.zero and self.pow(z, (q - 1) // 2) == self.neg(self.one):
                break
        c, x, t, r = self.pow(z, m), self.pow(a, (m + 1) // 2), self.pow(a, m), e
        while t != self.one:
            i, tt = 0, t
            while tt != self.one:
                tt, i = self.mul(tt, tt), i + 1
            b = self.pow(c, 1 << (r - i - 1))
            x, c = self.mul(x, b), self.mul(b, b)
            t, r = self.mul(t, c), i
        return x


# ----------------------------------------------------------------- curve ---

class Curve:
    """y^2 = x^3 + a2 x^2 + a4 x over F6; O is None."""

    def __init__(self, F: F6, a2, a4):
        self.F, self.a2, self.a4 = F, a2, a4

    def rhs(self, x):
        F = self.F
        x2 = F.mul(x, x)
        return F.add(F.add(F.mul(x2, x), F.mul(self.a2, x2)), F.mul(self.a4, x))

    def on(self, P):
        if P is None:
            return True
        x, y = P
        return self.F.mul(y, y) == self.rhs(x)

    def neg(self, P):
        return None if P is None else (P[0], self.F.neg(P[1]))

    def add(self, P, Q):
        F = self.F
        if P is None:
            return Q
        if Q is None:
            return P
        (x1, y1), (x2, y2) = P, Q
        if x1 == x2:
            if F.add(y1, y2) == F.zero:
                return None
            num = F.add(F.add(F.scal(3, F.mul(x1, x1)), F.scal(2, F.mul(self.a2, x1))), self.a4)
            lam = F.mul(num, F.inv(F.scal(2, y1)))
        else:
            lam = F.mul(F.sub(y2, y1), F.inv(F.sub(x2, x1)))
        x3 = F.sub(F.sub(F.sub(F.mul(lam, lam), self.a2), x1), x2)
        y3 = F.sub(F.mul(lam, F.sub(x1, x3)), y1)
        return (x3, y3)

    def mul(self, k, P):
        if k < 0:
            return self.mul(-k, self.neg(P))
        R = None
        while k:
            if k & 1:
                R = self.add(R, P)
            P = self.add(P, P)
            k >>= 1
        return R

    def nonsingular(self):
        F = self.F
        disc = F.scal(16, F.mul(F.mul(self.a4, self.a4), F.sub(F.mul(self.a2, self.a2), F.scal(4, self.a4))))
        return disc != F.zero


# ------------------------------------------------------------ arithmetic ---

def is_prime(n):
    if n < 2:
        return False
    small = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37]
    for q in small:
        if n % q == 0:
            return n == q
    d, r = n - 1, 0
    while d % 2 == 0:
        d, r = d // 2, r + 1
    for a in small:  # deterministic for n < 3.3e24
        x = pow(a, d, n)
        if x in (1, n - 1):
            continue
        for _ in range(r - 1):
            x = x * x % n
            if x == n - 1:
                break
        else:
            return False
    return True


def trial_prime(n):
    if n < 2:
        return False
    i = 2
    while i * i <= n:
        if n % i == 0:
            return False
        i += 1 if i == 2 else 2
    return True


def e2(v):
    return (int(v[0]), int(v[1]))


def e6(v):
    return tuple(e2(c) for c in v)


def point(d):
    return None if d["inf"] else (e6(d["x"]), e6(d["y"]))


def verify_instance(inst, rng):
    p, w = inst["p"], inst["w"]
    out = {"p": p, "seed": inst["seed"], "lin": inst["lin"], "l": inst["l"], "d": inst["d"], "dd": inst["dd"]}
    f2 = F2(p, w)
    s = e2(inst["s"])
    # tower
    out["w_nonresidue_euler"] = pow(w % p, (p - 1) // 2, p) == p - 1
    out["w_no_root_exhaustive"] = all((a * a - w) % p != 0 for a in range(p))
    out["s_noncube_euler"] = p % 3 != 0 and s != f2.zero and f2.pow(s, (p * p - 1) // 3) != f2.one
    cubes = {f2.mul(f2.mul(t, t), t) for t in f2.elems()}
    out["s_no_root_exhaustive"] = s not in cubes
    F = F6(f2, s)
    E = Curve(F, e6(inst["a2"]), e6(inst["a4"]))
    G, Q, l, d, dd = point(inst["g"]), point(inst["q"]), int(inst["l"]), int(inst["d"]), int(inst["dd"])
    out["curve_nonsingular"] = E.nonsingular()
    out["G_on_curve"] = G is not None and E.on(G)
    out["Q_on_curve"] = Q is not None and E.on(Q)
    out["l_prime_miller_rabin"] = is_prime(l)
    out["l_prime_trial_division"] = trial_prime(l)
    out["G_not_O"] = G is not None
    out["lG_is_O"] = E.mul(l, G) is None
    out["ddG_eq_Q"] = E.mul(dd, G) == Q
    out["dd_eq_d"] = dd == d
    out["d_in_range"] = 0 < d < l
    # negative controls (each must come out False)
    t = F.rand(rng)
    neg = {
        "dd_plus_1_times_G_eq_Q": E.mul(dd + 1, G) == Q,
        "perturbed_G_on_curve": E.on((G[0], F.add(G[1], F.one))),
        "composite_3l_prime": is_prime(3 * l),
        "residue_4_passes_nonresidue_test": pow(4 % p, (p - 1) // 2, p) == p - 1,
        "cube_s_noncube": f2.pow(f2.mul(f2.mul((2, 1), (2, 1)), (2, 1)), (p * p - 1) // 3) != f2.one,
        "l_minus_1_times_G_is_O": E.mul(l - 1, G) is None,
    }
    out["negative_controls_all_false"] = not any(neg.values())
    out["negative_controls"] = neg
    # self-tests of the arithmetic on this field and curve
    x, y, z = F.rand(rng), F.rand(rng), F.rand(rng)
    a, b = rng.randrange(1, l), rng.randrange(1, l)
    st = {
        "mul_assoc": F.mul(F.mul(x, y), z) == F.mul(x, F.mul(y, z)),
        "distrib": F.mul(x, F.add(y, z)) == F.add(F.mul(x, y), F.mul(x, z)),
        "inverse": x == F.zero or F.mul(x, F.inv(x)) == F.one,
        "frobenius_order": F.pow(x, F.q) == x,
        "scalar_linearity": E.add(E.mul(a, G), E.mul(b, G)) == E.mul(a + b, G),
        "l_minus_1_is_neg": E.mul(l - 1, G) == E.neg(G),
        "results_on_curve": E.on(E.mul(a, G)) and E.on(E.add(G, Q)),
    }
    out["self_tests_all_true"] = all(st.values())
    out["self_tests"] = st
    # extra, not gating: cofactor 4 and Hasse
    q = F.q
    lo, hi = q + 1 - 2 * p ** 3, q + 1 + 2 * p ** 3
    out["extra_4l_in_hasse_interval"] = lo <= 4 * l <= hi
    pts = 0
    ok = True
    while pts < 3:
        xr = F.rand(rng)
        yr = F.sqrt(E.rhs(xr), rng)
        if yr is None:
            continue
        R = (xr, yr)
        ok &= E.on(R) and E.mul(4 * l, R) is None
        pts += 1
    out["extra_4l_kills_3_random_points"] = ok
    gating = ["w_nonresidue_euler", "w_no_root_exhaustive", "s_noncube_euler", "s_no_root_exhaustive",
              "curve_nonsingular", "G_on_curve", "Q_on_curve", "l_prime_miller_rabin", "l_prime_trial_division",
              "G_not_O", "lG_is_O", "ddG_eq_Q", "dd_eq_d", "d_in_range", "negative_controls_all_false",
              "self_tests_all_true"]
    out["verified"] = all(out[k] for k in gating)
    out["gating_checks"] = gating
    return out


def rows_equal(row, rec):
    fields = sorted(set(row) | set(rec))
    diffs = {f: [row.get(f), rec.get(f)] for f in fields if f not in TIMING and row.get(f) != rec.get(f)}
    return len([f for f in fields if f not in TIMING]), diffs


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--snapshot", required=True)
    ap.add_argument("--dump-commit", required=True)
    ap.add_argument("--out", required=True)
    a = ap.parse_args()
    rng = random.Random(int(hashlib.sha256((OWN / "nonce.txt").read_bytes().strip()).hexdigest(), 16))
    recorded = {"sparse": {(r["p"], r["seed"]): r for r in json.loads(git_show(a.snapshot, f"{STUDY}/runs/R2-D1.json"))},
                "fast": {(r["p"], r["seed"]): r for r in json.loads(git_show(a.snapshot, f"{STUDY}/runs/R2-D2.json"))}}
    report = {"schema": "postgb-la-scalar-verification/v1", "verifier": f"{STUDY}/validation/verify_scalars.py",
              "independent_arithmetic": True, "snapshot_commit": a.snapshot, "dump_commit": a.dump_commit,
              "inputs": {}, "instances": [], "ties": [], "own_runs": []}
    all_ok = True
    dumps = {}
    for arm in ("sparse", "fast"):
        for rel in (f"{DUMP}/instances-{arm}.jsonl", f"{DUMP}/dlp-{arm}.json"):
            report["inputs"][rel] = hashlib.sha256(git_show(a.dump_commit, rel)).hexdigest()
        lines = git_show(a.dump_commit, f"{DUMP}/instances-{arm}.jsonl").decode().splitlines()
        rows = {(r["p"], r["seed"]): r for r in json.loads(git_show(a.dump_commit, f"{DUMP}/dlp-{arm}.json"))}
        insts = [json.loads(x) for x in lines]
        dumps[arm] = {(i["p"], i["seed"]): (i, line) for i, line in zip(insts, lines)}
        keys = sorted(dumps[arm])
        ok_cover = keys == sorted(recorded[arm]) == [(53, 1), (53, 2), (101, 1), (101, 2)] and len(insts) == 4
        report["ties"].append({"check": f"{arm}: one dump per recorded (p, seed), 4 in all", "ok": ok_cover,
                               "keys": keys})
        all_ok &= ok_cover
        for key in keys:
            inst, _ = dumps[arm][key]
            res = verify_instance(inst, rng)
            res["lin_label_matches_arm"] = inst["lin"] == arm
            res["verified"] &= res["lin_label_matches_arm"]
            report["instances"].append(res)
            all_ok &= res["verified"]
            n, diffs = rows_equal(rows[key], recorded[arm][key])
            tie = {"check": f"{arm} {key}: dlp-{arm}.json row = R2-{'D1' if arm == 'sparse' else 'D2'} row "
                            f"(all non-timing fields)", "fields_compared": n, "differing": diffs,
                   "dump_l_eq_row_l": inst["l"] == recorded[arm][key]["l"],
                   "row_correct": rows[key]["correct"], "row_solved": rows[key]["solved"]}
            tie["ok"] = not diffs and tie["dump_l_eq_row_l"] and tie["row_correct"] and tie["row_solved"]
            report["ties"].append(tie)
            all_ok &= tie["ok"]
    for key in sorted(dumps["sparse"]):
        si, fi = dumps["sparse"][key][0], dumps["fast"][key][0]
        same = {k: si[k] == fi[k] for k in si if k != "lin"}
        ok = all(same.values())
        report["ties"].append({"check": f"{key}: dumped instance identical across JV_LIN arms", "ok": ok,
                               "fields": same})
        all_ok &= ok
    # the validator's own runs of the dump binary
    jpath = OWN / "journal-dump.json"
    if jpath.exists():
        for entry in json.loads(jpath.read_text()):
            arm = entry["env"]["JV_LIN"]
            js = REPO / [p for p in entry["outputs"] if p.endswith(".json")][0]
            jl = REPO / [p for p in entry["outputs"] if p.endswith(".jsonl")][0]
            run = {"name": entry["name"], "argv": entry["argv"], "env": entry["env"],
                   "binary_sha256": entry["binary_sha256"], "exit_code": entry["exit_code"], "checks": []}
            ok = entry["exit_code"] == 0 and all(
                (REPO / p).exists() and hashlib.sha256((REPO / p).read_bytes()).hexdigest() == h
                for p, h in entry["outputs"].items())
            own_lines = jl.read_text().splitlines()
            for line in own_lines:
                inst = json.loads(line)
                key = (inst["p"], inst["seed"])
                same_line = line == dumps[arm][key][1]
                run["checks"].append({"key": key, "instance_line_byte_identical_to_producer_dump": same_line})
                ok &= same_line
            for row in json.loads(js.read_text()):
                key = (row["p"], row["seed"])
                n, diffs = rows_equal(row, recorded[arm][key])
                run["checks"].append({"key": key, "row_equals_R2_row_non_timing_fields": not diffs,
                                      "fields_compared": n, "differing": diffs})
                ok &= not diffs
            ok &= len(own_lines) == len(json.loads(js.read_text()))
            run["ok"] = bool(ok)
            report["own_runs"].append(run)
            all_ok &= bool(ok)
    report["all_verified"] = bool(all_ok)
    Path(a.out).write_text(json.dumps(report, indent=1) + "\n")
    print("all_verified", all_ok)
    for r in report["instances"]:
        print(r["lin"], r["p"], r["seed"], "verified", r["verified"],
              {k: r[k] for k in r["gating_checks"] if not r[k]} or "", "extra cofactor", r["extra_4l_kills_3_random_points"],
              "hasse", r["extra_4l_in_hasse_interval"])
    for t in report["ties"]:
        if not t["ok"]:
            print("TIE FAIL", json.dumps(t)[:400])
    for r in report["own_runs"]:
        print("own", r["name"], r["ok"], json.dumps(r["checks"])[:300])


if __name__ == "__main__":
    main()
