#!/usr/bin/env python3
"""Check the theory lane's solving-degree rules (W6-TH-1, W6-TH-2) against every complete exponent-study cell.
Rule A (prime d): SD = max(FF, 2^(m-1)) with FF = d+1 (sym), d+2 (plain, m=2), max(d,4)+8 (plain, m=3).
Rule B (composite d, sym): SD = e_max + c, c = (m-1) + ceil((m-1)(s+m-2)/4), s = sum(e_j - 1) over factors other than the largest
(with multiplicity). Also prints T/max_cols. Reads committed cells only."""
import glob, json, math, os
R = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..")
def factors(n):
    out, f = [], 2
    while n > 1:
        while n % f == 0: out.append(f); n //= f
        f += 1
    return out
rows = []
for f in sorted(glob.glob(os.path.join(R, "curve25519_exponent_study_20261108/cells/*.json"))) + \
         sorted(glob.glob(os.path.join(R, "curve25519_exponent_study_ext_20261110/cells/*.json"))):
    j = json.load(open(f))
    if j["capped"] or not j.get("per_target"): continue
    if j["d_or_t"] != j.get("v_size"): continue   # subgroup cells only (v_size = d)
    d, m, form = j["d_or_t"], j["m"], j["form"]
    sd = max(r["solving_degree_max"] for r in j["per_target"]); cols = max(r["max_cols"] for r in j["per_target"])
    fs = factors(d); emax = max(fs); s = sum(e - 1 for e in fs) - (emax - 1)
    if len(fs) == 1:
        ff = d + 1 if form == "sym" else (d + 2 if m == 2 else max(d, 4) + 8)
        pred = max(ff, 2 ** (m - 1)); rule = "A"
    elif form == "sym":
        pred = emax + (m - 1) + math.ceil((m - 1) * (s + m - 2) / 4); rule = "B"
    else:
        pred = None; rule = "-"
    rows.append((j["k"], m, form, d, fs, sd, pred, rule, j["mean_field_ops"] / cols))
okA = [r for r in rows if r[7] == "A"]; okB = [r for r in rows if r[7] == "B"]
for r in rows:
    print("k=%d m=%d %-5s d=%-4d %-14s SD=%-3d pred=%-4s rule %s  T/cols=%.3g%s" % (r[0], r[1], r[2], r[3], "x".join(map(str, r[4])), r[5], r[6], r[7], r[8], "" if r[6] in (None, r[5]) else "  <-- MISS"))
print("rule A (prime d): %d of %d" % (sum(r[5] == r[6] for r in okA), len(okA)))
print("rule B (composite d, sym): %d of %d" % (sum(r[5] == r[6] for r in okB), len(okB)))
