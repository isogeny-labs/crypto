#!/usr/bin/env python3
"""Target 3: compare the numbers in PUSHFORWARD.md tables ("Factor base and pushed
relations", "Comparison per prime", "Gap to matched rho") with medians and counts
recomputed by the validator from pushforward.json (primary run, 3 seeds).
Usage: check_pushforward_md.py PUSHFORWARD.md pushforward.json"""
import json, sys, statistics as st
md = open(sys.argv[1]).read().splitlines(); doc = json.load(open(sys.argv[2]))
rows = {r["p"]: r for r in doc["rows"]}
def tables():
    out = []; cur = None
    for l in md:
        if l.startswith("|"):
            c = [x.strip() for x in l.strip().strip("|").split("|")]
            cur = [c] if cur is None else cur + [c]
        else:
            if cur: out.append(cur)
            cur = None
    if cur: out.append(cur)
    return out
T = tables(); errs = []; n = 0
def num(s): return float(s.replace(",", ""))
def cmp(a, s, what):
    global n; n += 1
    s = s.strip(); dec = len(s.split(".")[1]) if "." in s else 0
    if abs(round(a, dec) - num(s)) > 10 ** (-dec) * 0.51: errs.append((what, a, s))
med = st.median
t_cmp = next(t for t in T if t[0][:3] == ["p", "r", "rho E1"])
for c in t_cmp[2:]:
    p = int(c[0]); S = rows[p]["seeds"]
    cmp(med(s["rho"]["ops"] for s in S), c[2], f"{p} rho")
    cmp(med(s["ic_jc"]["group_ops"] for s in S), c[3], f"{p} icjc")
    cmp(med(s["ic_jc"]["trials"] for s in S), c[4], f"{p} trials")
    cmp(med(s["ic_jc"]["relations"] for s in S), c[5], f"{p} rel")
    cmp(med(s["ic_jc"]["trials"] / s["ic_jc"]["relations"] for s in S), c[6], f"{p} tr/rel")
    cmp(med(s["ic_e1_fibre"]["run"]["group_ops"] for s in S), c[7], f"{p} fibre ops")
    two, bb = c[8].split("(")
    cmp(med(s["ic_e1_two_term"]["run"]["group_ops"] for s in S), two, f"{p} 2term ops")
    cmp(med(s["ic_e1_two_term"]["run"]["base_build_ops"] for s in S), bb.rstrip(")"), f"{p} base build")
    cmp(med(s["ic_e1_two_term"]["run"]["trials"] / s["ic_e1_two_term"]["run"]["relations"] for s in S), c[9], f"{p} 2t tr/rel")
    cmp(med(s["ic_e1_two_term"]["run"]["relations"] for s in S), c[10], f"{p} 2t rel")
    cmp(med(s["ic_e1_two_term"]["run"]["cols"] for s in S), c[11], f"{p} 2t cols")
t_gap = next(t for t in T if t[0][:4] == ["p", "IC J(C)", "IC E1 fibre", "IC E1 two-term"])
for c in t_gap[2:]:
    p = int(c[0]); S = rows[p]["seeds"]
    for col, arm in ((1, "ic_jc"), (2, "ic_e1_fibre_full"), (3, "ic_e1_two_term")):
        for j, kk in enumerate(("w0", "w0.1", "w1")):
            cmp(med(s["arms"][arm]["totals"]["ratio_" + kk] for s in S), c[col].split("/")[j], f"{p} gap {arm} {kk}")
t_fb = next(t for t in T if t[0][:3] == ["p", "r", "#E1"])
for c in t_fb[2:]:
    p = int(c[0]); R = rows[p]
    cmp(R["e1_order"], c[2], f"{p} #E1"); cmp(R["b"]["b_size"], c[3], f"{p} B")
    cmp(R["b"]["b_size"] / R["e1_order"], c[4], f"{p} B/E1"); cmp(R["b_in_subgroup"] / R["r"], c[5], f"{p} Bsub/r")
    tot = {k: sum(s["pushed"][k] for s in R["seeds"]) for k in ("e1_valid", "e2_valid", "e2_rows_carrying_target", "fibre_type_e1")}
    rel = sum(s["ic_jc"]["relations"] for s in R["seeds"])
    for col, k in ((8, "e1_valid"), (9, "e2_valid"), (11, "fibre_type_e1")):
        a, b = c[col].split("/"); cmp(tot[k], a, f"{p} {k}"); cmp(rel, b, f"{p} {k} denom")
    cmp(tot["e2_rows_carrying_target"], c[10], f"{p} e2 carrying")
print(f"PUSHFORWARD.md numbers compared {n}, mismatches {len(errs)}")
for e in errs[:30]: print("  ", e)
