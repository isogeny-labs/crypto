#!/usr/bin/env python3
"""Independent validator recomputation for research/curve25519_exponent_study_ext_20261110.

Written by the validator. It does not import, call or copy the producer's scripts
(scripts/analyse_ext.py, scripts/figure.py); scripts/run_ext.py is parsed with `ast` only to read
its constants, never executed. Inputs: the study's cells/ and data/, the parent study's cells/,
data/ and validator-report.json, the archived binary (hashed, not run), the validator's replay
outputs in this directory, and read-only git commands.

Output: validation/recompute.json and a printed summary. Run: python3 -I validation/validate.py
"""
import ast
import glob
import hashlib
import json
import math
import os
import re
import subprocess
import sys
from datetime import datetime, timedelta, timezone

import numpy as np

HERE = os.path.dirname(os.path.abspath(__file__))
STUDY = os.path.normpath(os.path.join(HERE, ".."))
PARENT = os.path.normpath(os.path.join(STUDY, "..", "curve25519_exponent_study_20261108"))
BIN = "/home/user/bin-archive/curve25519_family-dc2c30a4"
REPO = subprocess.run(["git", "-C", STUDY, "rev-parse", "--show-toplevel"], capture_output=True,
                      text=True, check=True).stdout.strip()
REL = os.path.relpath(STUDY, REPO)
VALIDATED = "236e925fb7b4d6e179d8b39c660ab0a4a03fadc0"  # the dispatched, frozen commit
OUT = {}


def git(*args):
    return subprocess.run(["git", "-C", REPO, *args], capture_output=True, text=True,
                          check=True).stdout


def sha256_file(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for b in iter(lambda: f.read(1 << 20), b""):
            h.update(b)
    return h.hexdigest()


def utc(s):
    return datetime.strptime(s, "%Y-%m-%dT%H:%M:%SZ").replace(tzinfo=timezone.utc)


def commit_time(c):
    return datetime.fromisoformat(git("show", "-s", "--format=%cI", c).strip())


# ---------------------------------------------------------------- number theory (own code)
def is_prime(n):
    if n < 2:
        return False
    for q in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):
        if n % q == 0:
            return n == q
    d, s = n - 1, 0
    while d % 2 == 0:
        d //= 2
        s += 1
    for a in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):
        x = pow(a, d, n)
        if x in (1, n - 1):
            continue
        for _ in range(s - 1):
            x = x * x % n
            if x == n - 1:
                break
        else:
            return False
    return True


def factor(n):
    out, q = [], 2
    while q * q <= n:
        while n % q == 0:
            out.append(q)
            n //= q
        q += 1
    if n > 1:
        out.append(n)
    return out


def sqrt_mod(a, p):
    """Tonelli-Shanks; None if a is a non-residue."""
    a %= p
    if a == 0:
        return 0
    if pow(a, (p - 1) // 2, p) != 1:
        return None
    q, s = p - 1, 0
    while q % 2 == 0:
        q //= 2
        s += 1
    z = 2
    while pow(z, (p - 1) // 2, p) != p - 1:
        z += 1
    m, c, t, r = s, pow(z, q, p), pow(a, q, p), pow(a, (q + 1) // 2, p)
    while t != 1:
        i, t2 = 0, t
        while t2 != 1:
            t2 = t2 * t2 % p
            i += 1
        b = pow(c, 1 << (m - i - 1), p)
        m, c, t, r = i, b * b % p, t * b * b % p, r * b % p
    return r


def powmod_vec(v, e, p):
    res = np.ones_like(v)
    base = v.copy()
    while e:
        if e & 1:
            res = res * base % p
        base = base * base % p
        e >>= 1
    return res


def count_mont(p, A):
    """#E for y^2 = x^3 + A x^2 + x over F_p by a Legendre-symbol sum (numpy)."""
    x = np.arange(p, dtype=np.int64)
    x2 = x * x % p
    rhs = (x2 * x % p + (A % p) * x2 % p + x) % p
    leg = powmod_vec(rhs, (p - 1) // 2, p)
    return p + 1 + int(np.sum(leg == 1)) - int(np.sum(leg == p - 1))


class Curve:
    """Short Weierstrass y^2 = x^3 + a x + b, affine, None = point at infinity."""

    def __init__(self, p, a, b):
        self.p, self.a, self.b = p, a, b

    def add(self, P, Q):
        p = self.p
        if P is None:
            return Q
        if Q is None:
            return P
        (x1, y1), (x2, y2) = P, Q
        if x1 == x2:
            if (y1 + y2) % p == 0:
                return None
            lam = (3 * x1 * x1 + self.a) * pow(2 * y1, p - 2, p) % p
        else:
            lam = (y2 - y1) * pow(x2 - x1, p - 2, p) % p
        x3 = (lam * lam - x1 - x2) % p
        return (x3, (lam * (x1 - x3) - y1) % p)

    def mul(self, P, k):
        R = None
        while k:
            if k & 1:
                R = self.add(R, P)
            P = self.add(P, P)
            k >>= 1
        return R

    def on(self, P):
        return P is None or (P[1] * P[1] - (P[0] ** 3 + self.a * P[0] + self.b)) % self.p == 0


def family_curve(k, a_given=None):
    """The study family at bit size k: p the largest prime < 2^k with p = 5 mod 8; A = 2 mod 4 from 6
    upward with A^2 - 4 a non-square, #E = 8 r and twist = 4 r' (r, r' prime), r > 2^(k-3).
    With a_given the minimality search is skipped and only the conditions are checked."""
    p = (1 << k) - 1
    while p % 8 != 5:
        p -= 1
    while not is_prime(p):
        p -= 8
    cands = [a_given] if a_given else range(6, p, 4)
    tried = 0
    for A in cands:
        if pow((A * A - 4) % p, (p - 1) // 2, p) != p - 1:
            continue
        tried += 1
        n = count_mont(p, A)
        nt = 2 * p + 2 - n
        if n % 8 == 0 and nt % 4 == 0 and is_prime(n // 8) and is_prime(nt // 4) and n // 8 > (1 << (k - 3)):
            break
    else:
        raise SystemExit("no A")
    i3, i27 = pow(3, p - 2, p), pow(27, p - 2, p)
    aw = (3 - A * A) * i3 % p
    bw = (2 * A ** 3 - 9 * A) * i27 % p
    shift = A * i3 % p
    E = Curve(p, aw, bw)
    r = n // 8
    u = 2
    while True:
        y = sqrt_mod(u * (u * u + A * u + 1), p)
        if y is not None:
            G = ((u + shift) % p, min(y, p - y))
            if E.on(G) and E.mul(G, r) is None:
                break
        u += 1
    return dict(k=k, p=p, A=A, n=n, nt=nt, r=r, shift=shift, E=E, G=G, u=u,
                a_search="minimal (searched from 6)" if not a_given else "given, conditions checked",
                a_candidates_counted=tried)


def targets_x(C, seed, count):
    M = (1 << 64) - 1
    st, out = seed, []
    for _ in range(count):
        st = (st * 6364136223846793005 + 1442695040888963407) & M
        e = 1 + (st >> 11) % (C["r"] - 1)
        T = C["E"].mul(C["G"], e)
        out.append(None if T is None else T[0])
    return out


def s3_coeffs(x1, x2, a, b, p):
    """Semaev S_3(x1, x2, U) for y^2 = x^3 + a x + b as coefficients of U^2, U, 1."""
    return ((x1 - x2) ** 2 % p,
            -2 * ((x1 + x2) * (x1 * x2 + a) + 2 * b) % p,
            ((x1 * x2 - a) ** 2 - 4 * b * (x1 + x2)) % p)


def res22(f, g, p):
    """Sylvester resultant of two polynomials of formal degree 2 (coefficients high to low)."""
    a1, b1, c1 = f
    a2, b2, c2 = g
    return ((a1 * c2 - a2 * c1) ** 2 - (a1 * b2 - a2 * b1) * (b1 * c2 - b2 * c1)) % p


def mu(d, p):
    w = 2
    fs = set(factor(p - 1))
    while any(pow(w, (p - 1) // q, p) == 1 for q in fs):
        w += 1
    z = pow(w, (p - 1) // d, p)
    return sorted({pow(z, j, p) for j in range(d)})


# ---------------------------------------------------------------- statistics (own code)
def betacf(a, b, x):
    tiny, qab, qap, qam = 1e-300, a + b, a + 1, a - 1
    c, d = 1.0, 1 - qab * x / qap
    d = 1 / (d if abs(d) > tiny else tiny)
    h = d
    for m in range(1, 400):
        m2 = 2 * m
        aa = m * (b - m) * x / ((qam + m2) * (a + m2))
        d = 1 + aa * d
        d = 1 / (d if abs(d) > tiny else tiny)
        c = 1 + aa / c
        c = c if abs(c) > tiny else tiny
        h *= d * c
        aa = -(a + m) * (qab + m) * x / ((a + m2) * (qap + m2))
        d = 1 + aa * d
        d = 1 / (d if abs(d) > tiny else tiny)
        c = 1 + aa / c
        c = c if abs(c) > tiny else tiny
        de = d * c
        h *= de
        if abs(de - 1) < 1e-15:
            break
    return h


def betai(a, b, x):
    if x <= 0:
        return 0.0
    if x >= 1:
        return 1.0
    lbt = math.lgamma(a + b) - math.lgamma(a) - math.lgamma(b) + a * math.log(x) + b * math.log(1 - x)
    if x < (a + 1) / (a + b + 2):
        return math.exp(lbt) * betacf(a, b, x) / a
    return 1 - math.exp(lbt) * betacf(b, a, 1 - x) / b


def t_cdf(t, nu):
    x = nu / (nu + t * t)
    tail = 0.5 * betai(nu / 2, 0.5, x)
    return 1 - tail if t > 0 else tail


def t975(nu):
    lo, hi = 0.0, 100.0
    for _ in range(200):
        mid = (lo + hi) / 2
        if t_cdf(mid, nu) < 0.975:
            lo = mid
        else:
            hi = mid
    return (lo + hi) / 2


# closed form at nu = 2 and published values as a check of the quantile code
assert abs(t975(2) - math.sqrt(2 * 0.9025 / (1 - 0.9025))) < 1e-9
for nu, ref in ((4, 2.7764451051977987), (6, 2.4469118511449692), (8, 2.3060041350333704)):
    assert abs(t975(nu) - ref) < 1e-9, (nu, t975(nu))
TABLE3 = {2: 4.303, 4: 2.776, 6: 2.447, 8: 2.306}  # 3-decimal table (as printed in t tables)


def ols(xs, ys, tq=None):
    x, y = np.array(xs, float), np.array(ys, float)
    n = len(x)
    mx, my = x.mean(), y.mean()
    sxx = float(((x - mx) ** 2).sum())
    s = float(((x - mx) * (y - my)).sum()) / sxx
    c = my - s * mx
    rss = float(((y - c - s * x) ** 2).sum())
    se = math.sqrt(rss / (n - 2) / sxx)
    t = t975(n - 2) if tq is None else tq
    return {"n": n, "slope": s, "lo95": s - t * se, "hi95": s + t * se, "se": se, "t": t}


# ================================================================= A. hashes and tree state
tracked = git("ls-tree", "-r", "--name-only", VALIDATED, REL).split()
OUT["validated_sha"] = VALIDATED
OUT["head_at_run"] = git("rev-parse", "HEAD").strip()
OUT["study_diff_validated_to_head"] = git("diff", "--stat", VALIDATED, "HEAD", "--", REL).strip()
OUT["study_tree_clean"] = git("status", "--porcelain", "--", REL).strip() == "" or all(
    ln[3:].startswith((REL + "/validation", REL + "/validator-report.json"))
    for ln in git("status", "--porcelain", "--", REL).splitlines())
OUT["artifact_hashes_sha256"] = {os.path.relpath(os.path.join(REPO, f), STUDY): sha256_file(os.path.join(REPO, f))
                                 for f in tracked}
head_blob_equal = {}
for f in tracked:
    blob = subprocess.run(["git", "-C", REPO, "show", f"{VALIDATED}:{f}"], capture_output=True, check=True).stdout
    head_blob_equal[os.path.relpath(os.path.join(REPO, f), STUDY)] = (
        hashlib.sha256(blob).hexdigest() == sha256_file(os.path.join(REPO, f)))
OUT["working_tree_equals_validated_sha"] = all(head_blob_equal.values())

# ================================================================= B. freeze order
FREEZE, FIX = "0afe01c59d", "4424e037d8"
t_freeze, t_fix = commit_time(FREEZE), commit_time(FIX)
freeze_files = git("show", "--name-only", "--format=", FREEZE).split()
fix_diff = git("diff", FREEZE, FIX, "--", f"{REL}/scripts/run_ext.py")
fix_changed = [ln for ln in fix_diff.splitlines() if ln[:1] in "+-" and not ln.startswith(("+++", "---"))]
frozen_unchanged = git("diff", FIX, VALIDATED, "--", f"{REL}/PROTOCOL.md", f"{REL}/scripts/run_ext.py",
                       f"{REL}/data/frozen.sha256") == ""
protocol_unchanged = git("diff", FREEZE, VALIDATED, "--", f"{REL}/PROTOCOL.md", f"{REL}/data/frozen.sha256") == ""
history = {}
for ln in git("log", "--reverse", "--format=@%h %cI", "--name-status", f"{FREEZE}^..{VALIDATED}", "--", REL).splitlines():
    if ln.startswith("@"):
        cur = ln[1:].split()
    elif ln.strip():
        st, path = ln.split("\t")
        history.setdefault(os.path.relpath(os.path.join(REPO, path), STUDY), []).append((cur[0], cur[1], st))
cells_after = {}
for f, hs in history.items():
    if f.startswith("cells/"):
        cells_after[f] = {"commits": hs, "added_once_never_modified": len(hs) == 1 and hs[0][2] == "A",
                          "after_freeze_and_fix": datetime.fromisoformat(hs[0][1]) > t_fix}
OUT["freeze"] = {
    "freeze_commit": FREEZE, "freeze_time": t_freeze.isoformat(), "freeze_files": freeze_files,
    "fix_commit": FIX, "fix_time": t_fix.isoformat(), "fix_changed_lines": fix_changed,
    "frozen_files_unchanged_fix_to_validated": frozen_unchanged,
    "protocol_and_frozen_sha_unchanged_freeze_to_validated": protocol_unchanged,
    "cells": cells_after,
    "file_history": history,
}

# ================================================================= C. binary
bin_sha = sha256_file(BIN)
frozen_lines = open(os.path.join(STUDY, "data", "frozen.sha256")).read().splitlines()
frozen_bin = [ln.split()[0] for ln in frozen_lines if ln.strip().endswith("curve25519_family")]
cells = {os.path.basename(f)[:-5]: json.load(open(f)) for f in sorted(glob.glob(os.path.join(STUDY, "cells", "*.json")))}
pvr = json.load(open(os.path.join(PARENT, "validator-report.json")))
parent_raw = {r["path"]: r["sha256"] for r in pvr["raw_data"]}
OUT["binary"] = {
    "path": BIN, "sha256": bin_sha, "frozen_line": frozen_bin,
    "matches_frozen": frozen_bin == [bin_sha],
    "cells_bin_sha256_all_equal_frozen": all(c["ext"]["bin_sha256"] == bin_sha for c in cells.values()),
    "frozen_sha256_file_equals_parent": open(os.path.join(STUDY, "data", "frozen.sha256"), "rb").read()
    == open(os.path.join(PARENT, "data", "frozen.sha256"), "rb").read(),
    "frozen_sha256_file_hash": sha256_file(os.path.join(STUDY, "data", "frozen.sha256")),
    "parent_validator_recorded_frozen_sha256_file": parent_raw.get("data/frozen.sha256"),
}

# ================================================================= D. protocol vs runner (ast, not executed)
src = open(os.path.join(STUDY, "scripts", "run_ext.py")).read()
tree = ast.parse(src)
consts = {}
for node in tree.body:
    if isinstance(node, ast.Assign) and len(node.targets) == 1 and isinstance(node.targets[0], ast.Name):
        try:
            consts[node.targets[0].id] = ast.literal_eval(node.value)
        except Exception:
            pass
cmd_tpl = re.search(r'cmd = \((.*?)\)\n', src, re.S).group(1)
p20 = 1048573
even_m3 = [d for d in range(10, 57) if (p20 - 1) % d == 0 and d % 2 == 0]
even_m4 = [d for d in range(10, 19) if (p20 - 1) % d == 0 and d % 2 == 0]
OUT["runner"] = {
    "SEED": consts.get("SEED"), "CAP": consts.get("CAP"), "MEM_KB": consts.get("MEM_KB"), "N": consts.get("N"),
    "CHAINS": consts.get("CHAINS"), "command_template": cmd_tpl,
    "p20_minus_1_factors": factor(p20 - 1),
    "even_divisors_9_to_57": even_m3, "even_divisors_9_to_19": even_m4,
    "protocol_lists_match": even_m3 == [12, 14, 18, 28, 36, 38, 42, 54] and even_m4 == [12, 14, 18],
    "limits_match_protocol": consts.get("CAP") == 10800 and consts.get("MEM_KB") == 12000000 and consts.get("N") == 6
    and consts.get("SEED") == 20261108 and "ulimit -v {MEM_KB}" in cmd_tpl and "RAYON_NUM_THREADS=1" in cmd_tpl
    and "{SEED + 1000 * m + d}" in cmd_tpl and "mont {m} sym {N}" in cmd_tpl,
}

# ================================================================= E. cell structure and arithmetic
pcells = {os.path.basename(f)[:-5]: json.load(open(f)) for f in sorted(glob.glob(os.path.join(PARENT, "cells", "*.json")))}
p_complete_keys = {tuple(sorted(c.keys())) for c in pcells.values() if "per_target" in c and not c.get("failure")}
p_crash_keys = {tuple(c.keys()) for c in pcells.values() if c.get("failure")}
p_pt_keys = {tuple(sorted(t.keys())) for c in pcells.values() for t in c.get("per_target", [])}
EXT_KEYS = ["study", "cap_s", "mem_kb", "load_before", "load_after", "wall_s", "started_utc", "bin_sha256", "exit"]
cellrep = {}
for name, c in cells.items():
    raw = open(os.path.join(STUDY, "cells", name + ".json")).read()
    k, m, d = c["k"], c["m"], c["d_or_t"]
    r = {"reserialises_byte_identical": json.dumps(c, indent=1) == raw,
         "ext_keys_and_order_match_runner": list(c["ext"].keys()) == EXT_KEYS,
         "ext_limits": c["ext"]["cap_s"] == 10800 and c["ext"]["mem_kb"] == 12000000,
         "ext_study": c["ext"]["study"] == "curve25519_exponent_study_ext_20261110"}
    body = {kk: v for kk, v in c.items() if kk != "ext"}
    if c.get("failure"):
        r["kind"] = "crash"
        r["keys_match_parent_crash_cells"] = tuple(body.keys()) in p_crash_keys
        mm = re.match(r"memory allocation of (\d+) bytes failed", c["stderr"])
        r["alloc_failure_bytes"] = int(mm.group(1)) if mm else None
        r["exit"] = c["ext"]["exit"]
        r["failure_matches_exit"] = c["failure"] == "exit %d" % c["ext"]["exit"] == "exit -6"
        r["wall_below_cap"] = c["ext"]["wall_s"] < 10800
    else:
        pt = c["per_target"]
        fo = [t["field_ops"] for t in pt]
        L = len(factor(d))
        r.update({
            "kind": "complete",
            "keys_match_parent_complete_cells": tuple(sorted(body.keys())) in p_complete_keys,
            "per_target_keys_match_parent": all(tuple(sorted(t.keys())) in p_pt_keys for t in pt),
            "mean_equals_sum_over_n": c["mean_field_ops"] == sum(fo) / len(fo),
            "targets": len(pt), "targets_done": c["targets_done"],
            "all_complete_not_timed_out": all(t["complete"] and not t["timed_out"] for t in pt),
            "all_inconsistent": all(t["inconsistent"] for t in pt),
            "max_rel_spread": (max(fo) - min(fo)) / (sum(fo) / len(fo)),
            "v_size_eq_d": c["v_size"] == d, "levels": c["levels"], "omega_d": L,
            "levels_eq_omega": c["levels"] == L, "n_vars_eq_mL_plus_m": c["n_vars"] == m * L + m,
            "seed_eq_formula": c["seed"] == 20261108 + 1000 * m + d,
            "p": c["p"], "cap_s": c["cap_s"], "engine_study_field": c["study"],
            "solving_degree": sorted({t["solving_degree"] for t in pt}),
            "solving_degree_max": sorted({t["solving_degree_max"] for t in pt}),
            "degree_reached": sorted({t["degree_reached"] for t in pt}),
            "max_cols": sorted({t["max_cols"] for t in pt}),
            "max_ms_per_target": max(t["ms"] for t in pt),
        })
    cellrep[name] = r
OUT["cells"] = cellrep

# ================================================================= F. replay comparison
replay = {}
for f in sorted(glob.glob(os.path.join(HERE, "replay_*.out"))):
    name = os.path.basename(f)[len("replay_"):-4]
    j = json.loads(open(f).read().strip().splitlines()[-1])
    c = {kk: v for kk, v in cells[name].items() if kk != "ext"}
    strip = lambda o: {kk: ([{a: b for a, b in t.items() if a != "ms"} for t in v] if kk == "per_target" else v)
                       for kk, v in o.items()}
    diffs = [kk for kk in set(strip(j)) | set(strip(c)) if strip(j).get(kk) != strip(c).get(kk)]
    replay[name] = {"identical_except_ms": not diffs, "differing_keys": diffs,
                    "field_ops": [t["field_ops"] for t in j["per_target"]],
                    "target_x": [t["target_x"] for t in j["per_target"]],
                    "replay_ms_total": sum(t["ms"] for t in j["per_target"]),
                    "recorded_ms_total": sum(t["ms"] for t in c["per_target"])}
OUT["replay"] = replay

# ================================================================= G. curve, targets, consistency (independent)
C20 = family_curve(20)
C22 = family_curve(22, a_given=43186)
E = C20["E"]
# S_3 / S_4 checks on random relations P1 + P2 + P3 = O and P1 + P2 + P3 + P4 = O
import random
rng = random.Random(20261112)
s3_ok = s4_ok = pert_nonzero = 0
for _ in range(200):
    P1, P2, P4 = (E.mul(C20["G"], rng.randrange(1, C20["r"])) for _ in range(3))
    P3 = E.add(P1, P2)
    if P3 is None:
        continue
    x3 = P3[0]
    f = s3_coeffs(P1[0], P2[0], E.a, E.b, E.p)
    s3_ok += (f[0] * x3 * x3 + f[1] * x3 + f[2]) % E.p == 0
    pert_nonzero += (f[0] * (x3 + 1) ** 2 + f[1] * (x3 + 1) + f[2]) % E.p != 0
    S = E.add(P3, P4)  # P1 + P2 + P4 = S, so P1 + P2 + P4 - S = O
    if S is None:
        continue
    g = s3_coeffs(P4[0], S[0], E.a, E.b, E.p)
    s4_ok += res22(f, g, E.p) == 0
targets_regen, consistency = {}, {}
for name, c in cells.items():
    k, m, d = c["k"], c["m"], c["d_or_t"]
    C = C20 if k == 20 else C22
    xs = targets_x(C, 20261108 + 1000 * m + d, 6)
    if "per_target" in c:
        rec = [t["target_x"] for t in c["per_target"]]
        targets_regen[name] = {"regenerated": xs, "recorded": rec, "equal": xs == rec}
        if m == 3:
            V = [(C["shift"] + z) % C["p"] for z in mu(d, C["p"])]
            a, b, p = C["E"].a, C["E"].b, C["p"]
            pairs = [s3_coeffs(V[i], V[j], a, b, p) for i in range(len(V)) for j in range(i, len(V))]
            cons = []
            for X in rec:
                hit = False
                for x3 in V:
                    g = s3_coeffs(x3, X, a, b, p)
                    if any(res22(f, g, p) == 0 for f in pairs):
                        hit = True
                        break
                cons.append(hit)
            consistency[name] = {"V_size": len(V), "consistent": cons,
                                 "recorded_inconsistent": [t["inconsistent"] for t in c["per_target"]],
                                 "agree": [not h for h in cons] == [t["inconsistent"] for t in c["per_target"]]}
    else:
        targets_regen[name] = {"first_target_x_if_run": xs[0], "recorded": None}
# P6 reference: parent k20 m3 d57 first target 610025
pd57 = pcells["k20_m3_sym_d57_g0"]["per_target"][0]["target_x"]
OUT["independent_curve"] = {
    "k20": {kk: C20[kk] for kk in ("p", "A", "n", "nt", "r", "shift", "u", "a_search", "a_candidates_counted")}
    | {"G": list(C20["G"]), "aW": E.a, "bW": E.b},
    "k22": {kk: C22[kk] for kk in ("p", "A", "n", "nt", "r", "shift", "u", "a_search")} | {"G": list(C22["G"])},
    "s3_relation_checks": [s3_ok, pert_nonzero, s4_ok],
    "targets": targets_regen, "consistency_m3": consistency,
    "parent_d57_first_target": {"recorded": pd57, "regenerated": targets_x(C20, 20261108 + 3000 + 57, 1)[0]},
}

# ================================================================= H. union, chain rule, fits
GROUPS = [(20, 3), (20, 4), (22, 3)]


def group_cells(src, k, m):
    out = {}
    for name, c in src.items():
        if c.get("k") == k and c.get("m") == m and c.get("form") == "sym" and c.get("gidx") == 0:
            out[c["d_or_t"]] = c
    return out


union, replaced = {}, []
for k, m in GROUPS:
    pg, eg = group_cells(pcells, k, m), group_cells(cells, k, m)
    replaced += [(k, m, d) for d in eg if d in pg]
    u = dict(pg)
    u.update({d: dict(c, _origin="ext") for d, c in eg.items()})
    for d in pg:
        if d not in eg:
            u[d] = dict(pg[d], _origin="parent")
    union[(k, m)] = dict(sorted(u.items()))


def complete(c):
    return not c.get("capped") and c.get("mean_field_ops")


# Chain rule on the union in d order (PROTOCOL "Cells and order")
planned = {(20, 3): [12, 14, 18, 28, 36, 38, 42, 54, 57, 63, 73, 133],
           (20, 4): [12, 14, 18, 19, 21, 27, 28, 36, 38, 42, 54, 57],
           (22, 3): [55, 75, 93]}
chain = {}
for (k, m), plan in planned.items():
    pg = group_cells(pcells, k, m)
    seq = sorted(set(plan) | {d for d, c in pg.items() if complete(c)})
    should, stop = [], None
    for d in seq:
        if d in pg and complete(pg[d]):
            continue  # parent complete: not rerun
        should.append(d)
        c = group_cells(cells, k, m).get(d)
        if c is None or not complete(c):
            stop = d
            break
    ran = sorted(group_cells(cells, k, m))
    chain[f"k{k}_m{m}"] = {"union_rule_should_run": should, "first_censored": stop, "ran": ran,
                           "ran_outside_union_rule": [d for d in ran if d not in should]}
OUT["chain_rule"] = chain

fits, readings = {}, {}
for (k, m), u in union.items():
    ok = [(d, c) for d, c in u.items() if complete(c)]
    T = {d: c["mean_field_ops"] for d, c in ok}
    Tchk = {d: sum(t["field_ops"] for t in c["per_target"]) / len(c["per_target"]) for d, c in ok}
    assert T == Tchk
    sel9 = [d for d in T if d >= 9]
    allc = list(T)
    span = lambda ds: max(ds) / min(ds) if ds else 0
    g = {"complete_d": allc, "origin": {d: u[d]["_origin"] for d in u},
         "censored_d": [d for d, c in u.items() if not complete(c)],
         "n_d_ge_9": len(sel9), "span_d_ge_9": span(sel9), "n_all": len(allc), "span_all": span(allc),
         "estimator_applicable": len(sel9) >= 4 and span(sel9) >= 4}
    def f(ds, tq=False):
        if len(ds) < 3:
            return None
        xs, ys = [math.log(d) for d in ds], [math.log(T[d]) for d in ds]
        r = ols(xs, ys)
        if tq and (len(ds) - 2) in TABLE3:
            r3 = ols(xs, ys, TABLE3[len(ds) - 2])
            r["lo95_table3"], r["hi95_table3"] = r3["lo95"], r3["hi95"]
        r["d"] = ds
        return r
    g["fit_d_ge_9"] = f(sel9, True)
    g["fit_all"] = f(allc, True)
    g["fit_even"] = f([d for d in allc if d % 2 == 0], True)
    g["fit_odd_d_ge_9"] = f([d for d in sel9 if d % 2 == 1], True)
    g["local_slopes"] = [[a, b, math.log(T[b] / T[a]) / math.log(b / a)] for a, b in zip(allc, allc[1:])]
    # exploratory: ln T on ln d and Omega(d)
    if len(allc) >= 4:
        X = np.column_stack([np.ones(len(allc)), np.log(allc), [len(factor(d)) for d in allc]])
        y = np.log([T[d] for d in allc])
        beta, res, *_ = np.linalg.lstsq(X, y, rcond=None)
        dof = len(allc) - 3
        if dof > 0:
            s2 = float(((y - X @ beta) ** 2).sum()) / dof
            cov = s2 * np.linalg.inv(X.T @ X)
            tq = t975(dof)
            g["exploratory_lnd_plus_omega"] = {
                "n": len(allc), "slope_lnd": float(beta[1]),
                "ci_lnd": [float(beta[1] - tq * math.sqrt(cov[1, 1])), float(beta[1] + tq * math.sqrt(cov[1, 1]))],
                "coef_omega": float(beta[2]),
                "ci_omega": [float(beta[2] - tq * math.sqrt(cov[2, 2])), float(beta[2] + tq * math.sqrt(cov[2, 2]))]}
    g["T"] = T
    fits[f"k{k}_m{m}"] = g
OUT["union_replaced"] = replaced
OUT["fits"] = fits

T20 = fits["k20_m3"]["T"]
ratios = {"d18_over_d19": T20[18] / T20[19], "d12_over_d9": T20[12] / T20[9],
          "d14_over_d12": T20[14] / T20[12], "d28_over_d27": T20[28] / T20[27],
          "d14_over_d19": T20[14] / T20[19]}
OUT["ratios"] = ratios

# parent comparison: parent analysis.json and the parent's 4-cell d >= 9 fit
pan = json.load(open(os.path.join(PARENT, "data", "analysis.json")))["groups"]
OUT["parent_comparison"] = {
    "parent_analysis_k20_m3_fit_all": pan["k20_m3_sym_g0"]["fit_all"],
    "parent_analysis_k22_m3_fit_all": pan["k22_m3_sym_g0"]["fit_all"],
    "parent_k20_m3_d_ge_9_four_cells": fits["k20_m3"]["fit_odd_d_ge_9"],
}

# ================================================================= I. timeline
runlog = open(os.path.join(STUDY, "data", "run.log")).read().splitlines()
envtxt = open(os.path.join(STUDY, "data", "env.txt")).read()
tl = {}
for ln in runlog:
    mm = re.match(r"(\d\d):(\d\d):(\d\d)Z (k\d+_m\d_sym_d\d+_g0) (\S+) (\d+)s$", ln)
    if not mm:
        continue
    name = mm.group(4)
    c = cells[name]
    t0 = utc(c["ext"]["started_utc"])
    w = c["ext"]["wall_s"]
    end_lo, end_hi = t0 + timedelta(seconds=w - 0.05), t0 + timedelta(seconds=w + 1.05)
    logged = end_lo.replace(hour=int(mm.group(1)), minute=int(mm.group(2)), second=int(mm.group(3)), microsecond=0)
    if logged < end_lo - timedelta(hours=12):
        logged += timedelta(days=1)
    if logged > end_hi + timedelta(hours=12):
        logged -= timedelta(days=1)
    add_commit = cells_after.get(f"cells/{name}.json", {}).get("commits", [[None, None]])[0]
    tl[name] = {"started": c["ext"]["started_utc"], "wall_s": w, "end_window": [end_lo.isoformat(), end_hi.isoformat()],
                "log_time": logged.isoformat(), "log_in_window": end_lo.replace(microsecond=0) <= logged <= end_hi,
                "log_value_matches": (mm.group(5) == "capped") == bool(c.get("capped")) and (
                    c.get("capped") or float(mm.group(5)) == c["mean_field_ops"]),
                "log_seconds": int(mm.group(6)), "round_wall": round(w),
                "add_commit": add_commit[0], "add_commit_time": add_commit[1],
                "committed_after_end": add_commit[1] is not None and datetime.fromisoformat(add_commit[1]) >= end_lo,
                "load_before_1min": c["ext"]["load_before"][0], "load_after_1min": c["ext"]["load_after"][0]}
order = sorted(tl, key=lambda n: tl[n]["started"])
contig = []
for a, b in zip(order, order[1:]):
    gap = (utc(tl[b]["started"]) - (utc(tl[a]["started"]) + timedelta(seconds=tl[a]["wall_s"]))).total_seconds()
    contig.append([a, b, round(gap, 1), cells[a]["ext"]["load_after"] == cells[b]["ext"]["load_before"]])
relaunch = re.findall(r"relaunch (\S+Z)", envtxt)
OUT["timeline"] = {
    "cells": tl, "order": order, "gaps_between_cells_s_and_load_continuity": contig,
    "env_launch": envtxt.splitlines()[0], "env_relaunch_times": relaunch,
    "env_kernels": re.findall(r"6\.18\.\S+", envtxt),
    "binary_ok_lines": sum(1 for ln in runlog if ln.startswith("binary ok")),
    "refused_launch_lines": sum(1 for ln in runlog if ln.startswith("binary sha256")),
    "all_done": runlog[-1] == "ALL_DONE",
    "d18_first_attempt_latest_end_if_uninterrupted": (utc("2026-11-10T15:40:17Z") + timedelta(seconds=cells["k20_m3_sym_d18_g0"]["ext"]["wall_s"])).isoformat(),
    "m4_d12_first_attempt_latest_end_if_uninterrupted": (utc("2026-11-10T19:46:53Z") + timedelta(seconds=cells["k20_m4_sym_d12_g0"]["ext"]["wall_s"])).isoformat(),
}

# ================================================================= J. predictions (protocol wording)
f20 = fits["k20_m3"]
pred = {
    "P1": "not testable: k20 m3 d57 was not rerun (chain stopped at d36)" if "k20_m3_sym_d57_g0" not in cells else "run",
    "P2": {"even_complete": [d for d in (12, 14, 18, 28, 36, 38, 42, 54) if d in T20],
           "estimator_applicable": f20["estimator_applicable"], "outcome": "failed" if not f20["estimator_applicable"] else "?"},
    "P3": {"censored": bool(cells["k20_m4_sym_d19_g0"]["capped"]), "cause": cells["k20_m4_sym_d19_g0"]["failure"],
           "within_union_chain_rule": 19 in chain["k20_m4"]["union_rule_should_run"]},
    "P4": {"m4_complete_ext": [d for d in (12, 14, 18) if complete(cells.get(f"k20_m4_sym_d{d}_g0", {}))], "outcome": "failed"},
    "P5": {"censored": bool(cells["k22_m3_sym_d55_g0"]["capped"]), "memory": "memory allocation" in cells["k22_m3_sym_d55_g0"]["stderr"]},
    "P6": {"binary_ok_lines": OUT["timeline"]["binary_ok_lines"], "cells_hash_ok": OUT["binary"]["cells_bin_sha256_all_equal_frozen"],
           "d57_rerun": "k20_m3_sym_d57_g0" in cells,
           "first_target_if_run": {n: targets_regen[n].get("first_target_x_if_run") for n in ("k20_m4_sym_d19_g0", "k22_m3_sym_d55_g0")}},
}
OUT["predictions"] = pred

# ================================================================= K. README numbers
fa, f9, fe, fo = f20["fit_all"], f20["fit_d_ge_9"], f20["fit_even"], f20["fit_odd_d_ge_9"]
k22 = fits["k22_m3"]
ls = {(a, b): s for a, b, s in f20["local_slopes"]}
claims = [
    ("k20 m3 span d>=9", 3.1, f20["span_d_ge_9"], 0.05), ("k20 m3 n d>=9", 8, f20["n_d_ge_9"], 0),
    ("k22 m3 span d>=9", 3.7, k22["span_d_ge_9"], 0.05), ("k22 m3 n d>=9", 6, k22["n_d_ge_9"], 0),
    ("k20 m3 span all", 9.3, f20["span_all"], 0.05), ("k22 m3 span all", 13.7, k22["span_all"], 0.05),
    ("k20 m4 span all", 3.0, fits["k20_m4"]["span_all"], 0.05), ("k20 m4 n all", 3, fits["k20_m4"]["n_all"], 0),
    ("k20 m3 d>=9 s", 6.08, f9["slope"], 0.005), ("k20 m3 d>=9 lo", 1.69, f9["lo95"], 0.005),
    ("k20 m3 d>=9 hi", 10.47, f9["hi95"], 0.005),
    ("k20 m3 all s", 6.87, fa["slope"], 0.005), ("k20 m3 all lo", 5.01, fa["lo95"], 0.005),
    ("k20 m3 all hi", 8.73, fa["hi95"], 0.005),
    ("k20 m3 even s", 7.15, fe["slope"], 0.005), ("k20 m3 even lo", 1.38, fe["lo95"], 0.005),
    ("k20 m3 even hi", 12.92, fe["hi95"], 0.005),
    ("k20 m3 odd d>=9 s", 5.56, fo["slope"], 0.005), ("k20 m3 odd d>=9 lo (README -8.76)", -8.76, fo["lo95"], 0.005),
    ("k20 m3 odd d>=9 hi", 19.88, fo["hi95"], 0.005),
    ("parent k20 m3 d>=9 lo (README -8.75)", -8.75, fo["lo95"], 0.005),
    ("k22 m3 d>=9 s", 4.51, k22["fit_d_ge_9"]["slope"], 0.005), ("k22 m3 d>=9 lo", -1.27, k22["fit_d_ge_9"]["lo95"], 0.005),
    ("k22 m3 d>=9 hi (README 10.28)", 10.28, k22["fit_d_ge_9"]["hi95"], 0.005),
    ("k22 m3 all s", 5.76, k22["fit_all"]["slope"], 0.005), ("k22 m3 all lo", 3.77, k22["fit_all"]["lo95"], 0.005),
    ("k22 m3 all hi", 7.75, k22["fit_all"]["hi95"], 0.005),
    ("parent k20 m3 all s (6 cells)", 6.44, pan["k20_m3_sym_g0"]["fit_all"]["slope"], 0.005),
    ("parent k20 m3 all lo", 3.49, pan["k20_m3_sym_g0"]["fit_all"]["lo95"], 0.005),
    ("parent k20 m3 all hi", 9.39, pan["k20_m3_sym_g0"]["fit_all"]["hi95"], 0.005),
    ("ratio d18/d19 (110x)", 110, ratios["d18_over_d19"], 0.5), ("ratio d12/d9 (5x)", 5, ratios["d12_over_d9"], 0.5),
    ("ratio d14/d12 (same)", 1.0, ratios["d14_over_d12"], 0.005), ("ratio d28/d27 (0.77x)", 0.77, ratios["d28_over_d27"], 0.005),
    ("T d18 (2.43e10)", 2.43e10, T20[18], 0.005e10), ("T d19 (2.20e8)", 2.20e8, T20[19], 0.005e8),
    ("T d12 (7.75e8)", 7.75e8, T20[12], 0.005e8), ("T d9 (1.48e8)", 1.48e8, T20[9], 0.005e8),
    ("T d12 table 7.748e8", 7.748e8, T20[12], 0.0005e8), ("T d14 table 7.766e8", 7.766e8, T20[14], 0.0005e8),
    ("T d18 table 2.4275e10", 2.4275e10, T20[18], 0.00005e10), ("T d28 table 2.0161e11", 2.0161e11, T20[28], 0.00005e11),
    ("slope 9-12", 5.8, ls[(9, 12)], 0.05), ("slope 12-14", 0.0, ls[(12, 14)], 0.05), ("slope 14-18", 13.7, ls[(14, 18)], 0.05),
    ("slope 18-19", -87.0, ls[(18, 19)], 0.05), ("slope 19-21", 25.4, ls[(19, 21)], 0.05),
    ("slope 21-27", 18.1, ls[(21, 27)], 0.05), ("slope 27-28", -7.4, ls[(27, 28)], 0.05),
]
walls = {"k20_m3_sym_d12_g0": 16, "k20_m3_sym_d14_g0": 18, "k20_m3_sym_d18_g0": 298, "k20_m3_sym_d28_g0": 3693,
         "k20_m3_sym_d36_g0": 4521, "k20_m4_sym_d12_g0": 6226, "k20_m4_sym_d19_g0": 1027, "k22_m3_sym_d55_g0": 5286}
for n, w in walls.items():
    claims.append((f"wall {n}", w, cells[n]["ext"]["wall_s"], 0.5))
table = {"k20_m3_sym_d12_g0": (7, 7759), "k20_m3_sym_d14_g0": (10, 10651), "k20_m3_sym_d18_g0": (7, 13616),
         "k20_m3_sym_d28_g0": (11, 54949)}
for n, (sd, mc) in table.items():
    claims.append((f"solving degree (= solving_degree_max) {n}", sd, cellrep[n]["solving_degree_max"][0], 0))
    claims.append((f"max columns {n}", mc, cellrep[n]["max_cols"][0], 0))
loads = {"d18 before (3.0)": (3.0, cells["k20_m3_sym_d18_g0"]["ext"]["load_before"][0], 0.05),
         "k22 d55 before (4.6)": (4.6, cells["k22_m3_sym_d55_g0"]["ext"]["load_before"][0], 0.05)}
for n, (cl, rv, tol) in loads.items():
    claims.append((f"load {n}", cl, rv, tol))
chk = [{"name": n, "claimed": cl, "recomputed": rv, "tol": tol, "agrees": abs(cl - rv) <= tol + 1e-12}
       for n, cl, rv, tol in claims]
OUT["readme_claims"] = chk
OUT["balanced_total"] = {
    "min_point_estimate_all_readings": min(f9["slope"], fa["slope"], fe["slope"], fo["slope"],
                                           k22["fit_d_ge_9"]["slope"], k22["fit_all"]["slope"]),
    "m3_denominator_max": 4 - min(f9["slope"], fa["slope"], fe["slope"], fo["slope"], k22["fit_d_ge_9"]["slope"], k22["fit_all"]["slope"]),
    "m4_local_slope_3_7": fits["k20_m4"]["local_slopes"][0][2],
    "m4_denominator_at_local_slope": 5 - fits["k20_m4"]["local_slopes"][0][2],
    "m4_three_cell_fit": ols([math.log(d) for d in (3, 7, 9)], [math.log(fits["k20_m4"]["T"][d]) for d in (3, 7, 9)]),
}

# ================================================================= L. README vs REPORT.typ numbers
num = re.compile(r"(?<![\w.])-?\d[\d,]*(?:\.\d+)?(?:e-?\d+)?")
rd = open(os.path.join(STUDY, "README.md")).read()
rt = open(os.path.join(STUDY, "REPORT.typ")).read()
norm = lambda s: s.replace(",", "")
tok_r = {norm(t) for t in num.findall(rd)}
tok_t = {norm(t) for t in num.findall(rt.replace("\\", ""))}
missing = sorted(t for t in tok_r - tok_t if not re.fullmatch(r"\d", t))
OUT["readme_numbers_missing_from_report_typ"] = missing

# ================================================================= M. parent cells unchanged since parent validation
pc_hash = {f"cells/{n}.json": sha256_file(os.path.join(PARENT, "cells", n + ".json")) for n in pcells}
OUT["parent_cells_match_parent_validator_hashes"] = all(parent_raw.get(p) == h for p, h in pc_hash.items())
OUT["parent_analysis_json_matches_parent_validator_hash"] = (
    parent_raw.get("data/analysis.json") == sha256_file(os.path.join(PARENT, "data", "analysis.json")))

# ================================================================= N. producer analysis.json vs own recomputation
an = json.load(open(os.path.join(STUDY, "data", "analysis.json")))
cmp = []
for g, key, mine in (("k20_m3_sym_g0", "fit_all", fa), ("k22_m3_sym_g0", "fit_all", k22["fit_all"])):
    cmp.append([g, key, an["groups"][g][key]["slope"], mine["slope"], an["groups"][g][key]["lo95"], mine.get("lo95_table3"), mine["lo95"]])
for g, key, mine in (("k20_m3_sym_g0", "d_ge_9_despite_gate", f9), ("k20_m3_sym_g0", "even_d_only", fe),
                     ("k20_m3_sym_g0", "odd_d_ge_9_only", fo), ("k22_m3_sym_g0", "d_ge_9_despite_gate", k22["fit_d_ge_9"])):
    a = an["groups"][g]["other_readings"][key]
    cmp.append([g, key, a["slope"], mine["slope"], a["lo95"], mine.get("lo95_table3"), mine["lo95"]])
OUT["producer_analysis_vs_own"] = {
    "rows[g,key,prod_s,own_s,prod_lo,own_lo_table3,own_lo_exact]": cmp,
    "max_abs_slope_diff": max(abs(r[2] - r[3]) for r in cmp),
    "max_abs_lo_diff_vs_table3": max(abs(r[4] - r[5]) for r in cmp),
    "replaced_parent_cells_equal": sorted(map(tuple, an["replaced_parent_cells"])) == sorted((k, m, "sym", 0, d) for k, m, d in replaced),
}

json.dump(OUT, open(os.path.join(HERE, "recompute.json"), "w"), indent=1, default=str, sort_keys=False)

# ---------------------------------------------------------------- summary
print("validated", VALIDATED, "HEAD at run", OUT["head_at_run"], "study diff to HEAD:", repr(OUT["study_diff_validated_to_head"]),
      "tree clean:", OUT["study_tree_clean"], "files == validated sha:", OUT["working_tree_equals_validated_sha"])
print("freeze", OUT["freeze"]["freeze_time"], "fix", OUT["freeze"]["fix_time"], "fix lines", len(fix_changed),
      "frozen unchanged", frozen_unchanged, protocol_unchanged)
print("cells added once & after fix:", all(v["added_once_never_modified"] and v["after_freeze_and_fix"] for v in cells_after.values()), len(cells_after))
print("binary", OUT["binary"]["matches_frozen"], OUT["binary"]["cells_bin_sha256_all_equal_frozen"], OUT["binary"]["frozen_sha256_file_equals_parent"])
print("runner", OUT["runner"]["limits_match_protocol"], OUT["runner"]["protocol_lists_match"], OUT["runner"]["CHAINS"])
for n, r in cellrep.items():
    print(" ", n, {k: v for k, v in r.items() if k in ("kind", "reserialises_byte_identical", "keys_match_parent_complete_cells",
                                                       "keys_match_parent_crash_cells", "mean_equals_sum_over_n", "all_inconsistent",
                                                       "max_rel_spread", "levels_eq_omega", "seed_eq_formula", "alloc_failure_bytes", "exit")})
print("replay", {n: (r["identical_except_ms"], r["differing_keys"]) for n, r in replay.items()})
print("curve k20", OUT["independent_curve"]["k20"])
print("curve k22", OUT["independent_curve"]["k22"])
print("S3/S4 checks", OUT["independent_curve"]["s3_relation_checks"])
print("targets regen", {n: r.get("equal", r.get("first_target_x_if_run")) for n, r in targets_regen.items()})
print("consistency", {n: (r["consistent"], r["agree"]) for n, r in consistency.items()})
print("parent d57", OUT["independent_curve"]["parent_d57_first_target"])
print("chain", chain)
for g, v in fits.items():
    print(g, "n9", v["n_d_ge_9"], "span9 %.3f" % v["span_d_ge_9"], "nall", v["n_all"], "spanall %.3f" % v["span_all"], "applicable", v["estimator_applicable"])
    for key in ("fit_d_ge_9", "fit_all", "fit_even", "fit_odd_d_ge_9"):
        r = v[key]
        if r:
            print("   %-15s n=%d s=%.4f [%.4f, %.4f] table3 [%.4f, %.4f] d=%s" % (key, r["n"], r["slope"], r["lo95"], r["hi95"],
                  r.get("lo95_table3", float("nan")), r.get("hi95_table3", float("nan")), r["d"]))
    print("   local", [(a, b, round(s, 3)) for a, b, s in v["local_slopes"]])
    if "exploratory_lnd_plus_omega" in v:
        print("   lnd+omega", v["exploratory_lnd_plus_omega"])
print("ratios", ratios)
print("claims failing:", [c for c in chk if not c["agrees"]])
print("balanced", OUT["balanced_total"])
print("timeline", {n: (v["log_in_window"], v["log_value_matches"], v["committed_after_end"], v["log_seconds"] == v["round_wall"]) for n, v in tl.items()})
print("gaps", contig)
print("env", OUT["timeline"]["env_relaunch_times"], OUT["timeline"]["env_kernels"], "ok lines", OUT["timeline"]["binary_ok_lines"])
print("first-attempt bounds", OUT["timeline"]["d18_first_attempt_latest_end_if_uninterrupted"], OUT["timeline"]["m4_d12_first_attempt_latest_end_if_uninterrupted"])
print("predictions", pred)
print("README numbers not in REPORT.typ:", missing)
print("parent cells unchanged:", OUT["parent_cells_match_parent_validator_hashes"], OUT["parent_analysis_json_matches_parent_validator_hash"])
print("producer vs own", OUT["producer_analysis_vs_own"]["max_abs_slope_diff"], OUT["producer_analysis_vs_own"]["max_abs_lo_diff_vs_table3"],
      OUT["producer_analysis_vs_own"]["replaced_parent_cells_equal"])
