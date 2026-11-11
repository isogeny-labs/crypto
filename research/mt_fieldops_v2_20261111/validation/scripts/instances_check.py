#!/usr/bin/env python3
"""Validator: instance validity and freshness (independent of the producer's gen_instances.py).
usage: instances_check.py <frozen study dir> <repo root> <out json>"""
import json, os, sys, hashlib, random

STUDY, REPO, OUT = sys.argv[1], sys.argv[2], sys.argv[3]

def is_prime(n):
    if n < 2:
        return False
    small = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37]
    for q in small:
        if n % q == 0:
            return n == q
    d, s = n - 1, 0
    while d % 2 == 0:
        d //= 2; s += 1
    for a in small:  # deterministic for n < 3.3e24
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

def trial_prime(n):
    """second, unrelated primality test: trial division (n < 2^27)"""
    if n < 2:
        return False
    i = 2
    while i * i <= n:
        if n % i == 0:
            return False
        i += 1
    return True

# projective (X:Y:Z) arithmetic, no inversions, written independently of the harness's affine code
def on_curve(P, a, b, p):
    x, y = P
    return (y * y - (x * x * x + a * x + b)) % p == 0

def padd(P, Q, a, p):
    # affine with Python's modular inverse; None = O
    if P is None: return Q
    if Q is None: return P
    x1, y1 = P; x2, y2 = Q
    if x1 == x2:
        if (y1 + y2) % p == 0:
            return None
        lam = (3 * x1 * x1 + a) * pow(2 * y1, -1, p) % p
    else:
        lam = (y2 - y1) * pow(x2 - x1, -1, p) % p
    x3 = (lam * lam - x1 - x2) % p
    return (x3, (lam * (x1 - x3) - y1) % p)

def ladder(P, k, a, p):
    # Montgomery ladder (different structure from the harness's double-and-add)
    R0, R1 = None, P
    for bit in bin(k)[2:]:
        if bit == '1':
            R0, R1 = padd(R0, R1, a, p), padd(R1, R1, a, p)
        else:
            R1, R0 = padd(R0, R1, a, p), padd(R0, R0, a, p)
    return R0

def toint(v):
    if isinstance(v, bool):
        return None
    if isinstance(v, int):
        return v
    if isinstance(v, str):
        s = v.strip()
        try:
            return int(s, 16) if s.lower().startswith("0x") else int(s)
        except ValueError:
            return None
    return None

def curves_in(obj, out):
    if isinstance(obj, dict):
        p = None
        if isinstance(obj.get("field"), dict):
            p = toint(obj["field"].get("modulus", obj["field"].get("p")))
        if p is None:
            for k in ("p", "modulus", "prime", "q"):
                if k in obj:
                    p = toint(obj[k]); break
        a = toint(obj.get("a")) if "a" in obj else None
        b = toint(obj.get("b")) if "b" in obj else None
        if p and a is not None and b is not None and p > 3:
            n = None
            for k in ("subgroup_order", "n", "order", "group_order", "r"):
                if k in obj:
                    n = toint(obj[k]);
                    if n: break
            out.append((p, a % p, b % p, n))
        for v in obj.values():
            curves_in(v, out)
    elif isinstance(obj, list):
        for v in obj:
            curves_in(v, out)

def jinv(a, b, p):
    num = 1728 * 4 * pow(a, 3, p) % p
    den = (4 * pow(a, 3, p) + 27 * b * b) % p
    try:
        return num * pow(den, -1, p) % p
    except ValueError:
        return None

res = {"instances": [], "errors": []}
man = json.load(open(os.path.join(STUDY, "instances", "MANIFEST.json")))
man_by = {m["file"]: m for m in man}
inst = []
for fn in sorted(os.listdir(os.path.join(STUDY, "instances"))):
    if fn == "MANIFEST.json":
        continue
    raw = open(os.path.join(STUDY, "instances", fn), "rb").read()
    d = json.loads(raw)
    p = int(d["field"]["modulus"]); a = int(d["a"]); b = int(d["b"]); n = int(d["subgroup_order"])
    G = (int(d["generator"]["x"]), int(d["generator"]["y"]))
    Pt = (int(d["point"]["x"]), int(d["point"]["y"])); kl = int(d["fixture"]["known_log"])
    chk = {
        "file": fn, "p": p, "a": a, "b": b, "n": n,
        "p_prime_mr": is_prime(p), "p_prime_trial": trial_prime(p),
        "n_prime_mr": is_prime(n), "n_prime_trial": trial_prime(n),
        "disc_nonzero": (4 * a ** 3 + 27 * b * b) % p != 0,
        "G_on_curve": on_curve(G, a, b, p),
        "nG_is_O": ladder(G, n, a, p) is None,
        "G_not_O": G is not None,
        "hasse": abs(p + 1 - n) <= 2 * int(p ** 0.5) + 2,
        "hasse_width_lt_n": 4 * (int(p ** 0.5) + 1) < n,
        "cofactor_1": int(d.get("cofactor", 1)) == 1,
        "group_order_eq_n": int(d.get("group_order", n)) == n,
        "fixture_point_ok": ladder(G, kl, a, p) == Pt,
        "manifest_sha_ok": man_by.get(fn, {}).get("sha256") == hashlib.sha256(raw).hexdigest(),
        "name_matches_file": d.get("name") + ".json" == fn,
        "p_bits": p.bit_length(), "n_bits": n.bit_length(),
    }
    # extra: random points have order dividing n (consistency with #E = n)
    rnd = random.Random(fn)
    ok_rand = True
    for _ in range(3):
        while True:
            x = rnd.randrange(p); rhs = (x ** 3 + a * x + b) % p
            if rhs == 0 or pow(rhs, (p - 1) // 2, p) == 1:
                break
        # sqrt by brute exponent if p % 4 == 3 else Tonelli via sympy-free loop
        if p % 4 == 3:
            y = pow(rhs, (p + 1) // 4, p)
        else:
            # Tonelli-Shanks (own)
            q, s = p - 1, 0
            while q % 2 == 0: q //= 2; s += 1
            z = 2
            while pow(z, (p - 1) // 2, p) != p - 1: z += 1
            m, c, t, r = s, pow(z, q, p), pow(rhs, q, p), pow(rhs, (q + 1) // 2, p)
            while t != 1:
                i, tt = 0, t
                while tt != 1: tt = tt * tt % p; i += 1
                bb = pow(c, 1 << (m - i - 1), p)
                m, c, t, r = i, bb * bb % p, t * bb * bb % p, r * bb % p
            y = r
        assert (y * y - rhs) % p == 0
        if ladder((x, y), n, a, p) is not None:
            ok_rand = False
    chk["random_points_killed_by_n"] = ok_rand
    chk["valid"] = all(v for k, v in chk.items() if isinstance(v, bool))
    res["instances"].append(chk)
    inst.append((p, a % p, b % p, n, fn))

# freshness: every JSON (and curve-like JSONL lines) under research/ outside this study
found = []  # (p,a,b,n,path)
nfiles = 0; nskip = 0
for root, dirs, files in os.walk(os.path.join(REPO, "research")):
    if os.path.abspath(root).startswith(os.path.abspath(os.path.join(REPO, "research", "mt_fieldops_v2_20261111"))):
        continue
    dirs[:] = [d for d in dirs if d not in (".git", "node_modules", "target")]
    for fn in files:
        if not (fn.endswith(".json") or fn.endswith(".jsonl")):
            continue
        path = os.path.join(root, fn)
        try:
            if os.path.getsize(path) > 50_000_000:
                nskip += 1; continue
            txt = open(path, encoding="utf-8", errors="replace").read()
        except OSError:
            nskip += 1; continue
        nfiles += 1
        out = []
        if fn.endswith(".json"):
            try:
                curves_in(json.loads(txt), out)
            except Exception:
                nskip += 1
        else:
            for line in txt.splitlines():
                line = line.strip()
                if not line.startswith("{"):
                    continue
                if '"a"' not in line or '"b"' not in line:
                    continue
                try:
                    curves_in(json.loads(line), out)
                except Exception:
                    pass
        for c in out:
            found.append(c + (os.path.relpath(path, REPO),))
docs_curves = []
dc = os.path.join(REPO, "docs", "curves")
if os.path.isdir(dc):
    for fn in os.listdir(dc):
        if fn.endswith(".json"):
            try:
                o = []; curves_in(json.load(open(os.path.join(dc, fn))), o)
                docs_curves += [c + ("docs/curves/" + fn,) for c in o]
            except Exception:
                pass
uniq = {(c[0], c[1], c[2]) for c in found}
inst_keys = {(c[0], c[1], c[2]): c[4] for c in inst}
exact = [c for c in found if (c[0], c[1], c[2]) in inst_keys]
exact_docs = [c for c in docs_curves if (c[0], c[1], c[2]) in inst_keys]
# isomorphism class over the same p: same j and same n (twists differ in n)
inst_j = {}
for p, a, b, n, fn in inst:
    if (4 * a ** 3 + 27 * b * b) % p:
        inst_j[(p, jinv(a, b, p), n)] = fn
iso = []
for c in found + docs_curves:
    p, a, b, n = c[:4]
    if (4 * pow(a, 3, p) + 27 * b * b) % p == 0:
        continue
    key = (p, jinv(a, b, p), n)
    if key in inst_j and n is not None:
        iso.append((c[4], inst_j[key]))
    # same j without an order: still report
same_j_no_n = []
inst_pj = {(p, jinv(a, b, p)): fn for p, a, b, n, fn in inst}
for c in found + docs_curves:
    p, a, b, n = c[:4]
    if (4 * pow(a, 3, p) + 27 * b * b) % p == 0:
        continue
    if (p, jinv(a, b, p)) in inst_pj:
        same_j_no_n.append((c[4], inst_pj[(p, jinv(a, b, p))], n))
same_pn = sorted({(c[4], inst_pn) for c in found for inst_pn in [next((f for (p, a, b, n, f) in inst if p == c[0] and n == c[3]), None)] if inst_pn})
inst_dupes = len(inst_keys) != len(inst)
res["freshness"] = {
    "json_files_scanned": nfiles, "files_skipped_or_unparsed": nskip,
    "curve_records_found": len(found), "distinct_curves_found_(p,a,b)": len(uniq),
    "docs_curves_records": len(docs_curves),
    "exact_(p,a,b)_overlap_research": [(c[5] if len(c) > 5 else c[4]) for c in exact],
    "exact_(p,a,b)_overlap_docs_curves": [c[4] for c in exact_docs],
    "isomorphic_(p,j,n)_overlap": iso,
    "same_(p,j)_any_n": same_j_no_n,
    "same_(p,n)_overlap": same_pn,
    "instances_distinct_among_themselves": not inst_dupes,
    "instances_distinct_j": len({(p, jinv(a, b, p)) for p, a, b, n, fn in inst}) == len(inst),
}
res["all_valid"] = all(c["valid"] for c in res["instances"])
json.dump(res, open(OUT, "w"), indent=1)
print("valid", res["all_valid"], sum(c["valid"] for c in res["instances"]), "/", len(res["instances"]))
print(json.dumps(res["freshness"], indent=1)[:3000])
