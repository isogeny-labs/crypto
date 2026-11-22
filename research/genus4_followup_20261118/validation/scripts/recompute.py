#!/usr/bin/env python3
"""Validator's independent recompute of genus4_followup_20261118 (does not import or run the
producer's analyze.py / select.py / sensitivity.py).

Reads every raw file as the git blob at COMMIT (default 6213ecbc0d), so the working tree cannot
substitute data. Every record field is read with [] (lesson L-99ee952350b2): a missing key raises.

Usage: recompute.py REPO OUT_JSON [COMMIT]
"""
import gzip, hashlib, io, json, math, random, statistics, subprocess, sys

REPO, OUT = sys.argv[1], sys.argv[2]
COMMIT = sys.argv[3] if len(sys.argv) > 3 else "6213ecbc0da0ed4917ab75805258201d6629b477"
D = "research/genus4_followup_20261118"
LEAN = "research/genus4_lean_rho_20261116"
CHEAP = "research/genus4_cheap_ic_20261117"
P, P8 = 16411, 8209
W_ARM = "C_double_lp+pre+coord+unitT"
X_ARM = "C_double_lp+pre+coord+unitT+ferrari+sgecoord+sge(4,1.5)"
Q_ARM = "R_rho_plain_nowin"
CURVES16 = (172, 180, 2640)
CURVES8 = (363, 393, 407, 457, 474, 519)
NB, BSEED = 2000, 20261123
WS = (("0", 0.0), ("0.1", 0.1), ("1", 1.0))


def git(*a):
    return subprocess.run(["git", "-C", REPO, *a], check=True, capture_output=True).stdout


TRACKED = set(git("ls-tree", "-r", "--name-only", COMMIT).decode().splitlines())
PINS = {}


def load(path):
    assert path in TRACKED, f"{path} not tracked at {COMMIT}"
    raw = git("show", f"{COMMIT}:{path}")
    PINS[path] = hashlib.sha256(raw).hexdigest()
    txt = gzip.decompress(raw).decode() if path.endswith(".gz") else raw.decode()
    return [json.loads(l) for l in txt.splitlines() if l.strip()]


def runs(rows, arm, p):
    return [r for r in rows if r["kind"] == "run" and r["arm"] == arm and r["p"] == p and r["g"] == 4]


def key(r):
    return (r["p"], r["curve_seed"], r["target_seed"], r["seed"])


def kidx(k):
    """target index k from target_seed = cs + k * 1000003"""
    q, m = divmod(k[2] - k[1], 1000003)
    assert m == 0, k
    return q


def nowall(r):
    return {a: b for a, b in r.items() if a != "wall_ns"}


def merge(rowlists, arm, p, label, dup_log):
    """key -> row; a key present twice must be identical in every field except wall_ns."""
    out = {}
    for rows in rowlists:
        for r in runs(rows, arm, p):
            k = key(r)
            if k in out:
                same = nowall(out[k]) == nowall(r)
                dup_log.append({"set": label, "key": list(k), "identical_except_wall_ns": same,
                                "fields_compared": sorted(nowall(r))})
                assert same, (label, k)
            out[k] = r
    return out


res = {"commit": COMMIT, "checks": {}, "numbers": {}}
C, N = res["checks"], res["numbers"]

# ---------------- part A ----------------
GRID = ["0.5", "0.7", "1.0", "1.4", "2.0"]
partA = {}
for cc in GRID:
    rows = load(f"{D}/a_cc{cc}.jsonl.gz")
    cfg = [r for r in rows if r["kind"] == "config"]
    assert len(cfg) == 1
    cfg = cfg[0]
    C[f"A_cc{cc}_config"] = {"cc": cfg["cc"], "arms": cfg["arms"], "primes": cfg["primes"],
                            "genera": cfg["genera"], "targets": cfg["targets"], "target_from": cfg["target_from"],
                            "seeds": cfg["seeds"], "max_h": cfg["max_h"],
                            "ok": cfg["cc"] == float(cc) and cfg["arms"] == "x" and cfg["primes"] == [P8]
                            and cfg["genera"] == [4] and cfg["targets"] == 3 and cfg["target_from"] == 0 and cfg["seeds"] == 2}
    rs = runs(rows, X_ARM, P8)
    keys = {key(r) for r in rs}
    want = {(P8, cs, cs + k * 1000003, s) for cs in CURVES8 for k in range(3) for s in (1, 2)}
    nb = math.ceil(float(cc) * P8 ** 0.75)
    allok = all(r["certified"] is True and r["ok"] is True and r["exhausted"] is False for r in rs)
    cost = [r["group_ops"] + r["rel_mul"] + r["rel_add"] + r["la_mul"] + r["la_add"] for r in rs]
    partA[cc] = {
        "runs": len(rs), "keys_exact": keys == want and len(rs) == 36, "all_certified_ok": allok,
        "nb_formula": nb, "fb_cols": sorted({r["fb_cols"] for r in rs}),
        "mean_group_ops": statistics.fmean(r["group_ops"] for r in rs),
        "mean_rel_native": statistics.fmean(r["rel_mul"] + r["rel_add"] for r in rs),
        "mean_la_native": statistics.fmean(r["la_mul"] + r["la_add"] for r in rs),
        "mean_trials": statistics.fmean(r["trials"] for r in rs),
        "mean_cost_w1": sum(cost) / len(cost),
    }
base = partA["2.0"]["mean_cost_w1"]
for cc in GRID:
    partA[cc]["over_cc2"] = partA[cc]["mean_cost_w1"] / base
    partA[cc]["la_share_of_native"] = partA[cc]["mean_la_native"] / (partA[cc]["mean_la_native"] + partA[cc]["mean_rel_native"])
elig = [cc for cc in GRID if partA[cc]["keys_exact"] and partA[cc]["all_certified_ok"]]
best = sorted(elig, key=lambda c: (partA[c]["mean_cost_w1"], -float(c)))[0]
N["partA"] = partA
N["cc_star"] = best
tr = lambda a, b: math.log(partA[b]["mean_trials"] / partA[a]["mean_trials"]) / math.log(partA[b]["nb_formula"] / partA[a]["nb_formula"])
N["trials_exponent_cc1.0_to_2.0"] = tr("1.0", "2.0")
N["trials_exponent_cc0.5_to_1.0"] = tr("0.5", "1.0")

# ---------------- part B rows ----------------
bwx = load(f"{D}/b_wx.jsonl.gz")
bq = load(f"{D}/b_q.jsonl.gz")
bq1 = load(f"{D}/b_q.jsonl.r1.gz")
g3rows = load(f"{D}/b_q_g3.jsonl.gz")
lean_files = sorted(p for p in TRACKED if p.startswith(f"{LEAN}/p16411") and p.endswith(".jsonl"))
lean = [r for f in lean_files for r in load(f)]
cheap = load(f"{CHEAP}/p16411.jsonl.gz")

for nm, rows, want in (("b_wx", bwx, {"cc": 2.0, "arms": "wx", "targets": 18, "target_from": 0, "seeds": 2}),
                       ("b_q", bq, {"cc": 2.0, "arms": "Q", "targets": 15, "target_from": 3, "seeds": 2}),
                       ("b_q.r1", bq1, {"cc": 2.0, "arms": "Q", "targets": 15, "target_from": 3, "seeds": 2}),
                       ("b_q_g3", g3rows, {"cc": 2.0, "arms": "Q", "targets": 1, "target_from": 0, "seeds": 1})):
    cfg = [r for r in rows if r["kind"] == "config"]
    assert len(cfg) == 1
    C[f"{nm}_config"] = {k: cfg[0][k] for k in want} | {"genera": cfg[0]["genera"], "max_h": cfg[0]["max_h"],
                                                       "primes": cfg[0]["primes"], "rho_cap": cfg[0]["rho_cap"],
                                                       "ok": all(cfg[0][k] == v for k, v in want.items())
                                                       and cfg[0]["genera"] == [4] and cfg[0]["max_h"] == 2 and cfg[0]["primes"] == [P]}
    C[f"{nm}_has_done_row"] = any(r["kind"] == "done" for r in rows)
    C[f"{nm}_skip_rows"] = sum(r["kind"] == "skip" for r in rows)

# instance rows: identical across every file that has them
inst = {}
inst_src = []
for nm, rows in (("b_wx", bwx), ("b_q", bq), ("b_q.r1", bq1), ("b_q_g3", g3rows), ("lean", lean), ("cheap", cheap)):
    for r in rows:
        if r["kind"] == "instance" and r["p"] == P:
            cs = r["curve_seed"]
            if cs in inst:
                assert inst[cs] == r, ("instance rows differ", nm, cs)
            inst[cs] = r
            inst_src.append(nm)
C["instances_consistent"] = {"curves": sorted(inst), "rows_seen": len(inst_src)}
assert sorted(inst) == list(CURVES16)

dups = []
ARMS = {"w": merge([bwx], W_ARM, P, "w", dups), "x": merge([bwx], X_ARM, P, "x", dups)}
Qold = merge([lean], Q_ARM, P, "Q_lean", dups)
Qnew = merge([bq, bq1], Q_ARM, P, "Q_new", dups)
assert not set(Qold) & set(Qnew), "Q old/new overlap"
Q = {**Qold, **Qnew}
res["duplicates"] = dups
C["Q_resume_duplicates"] = {"n": sum(d["set"] == "Q_new" for d in dups),
                            "all_identical": all(d["identical_except_wall_ns"] for d in dups if d["set"] == "Q_new"),
                            "keys": [d["key"] for d in dups if d["set"] == "Q_new"]}
C["Q_counts"] = {"b_q_runs": len(runs(bq, Q_ARM, P)), "b_q_r1_runs": len(runs(bq1, Q_ARM, P)),
                 "Qnew_keys": len(Qnew), "Qold_keys": len(Qold), "lean_files": lean_files}

want_all = {(P, cs, cs + k * 1000003, s) for cs in CURVES16 for k in range(18) for s in (1, 2)}
C["key_sets"] = {"w_exact": set(ARMS["w"]) == want_all, "x_exact": set(ARMS["x"]) == want_all,
                 "Q_exact": set(Q) == want_all,
                 "Qold_is_k0_2": {kidx(k) for k in Qold} == {0, 1, 2},
                 "Qnew_is_k3_17": {kidx(k) for k in Qnew} == set(range(3, 18))}
keys = sorted(want_all)
for d in ARMS.values():
    for k in keys:
        assert d[k]["r"] == inst[k[1]]["r"] and d[k]["h"] == inst[k[1]]["h"]
for k in keys:
    assert Q[k]["r"] == inst[k[1]]["r"]

# ---------------- gates ----------------
g1 = []
for a, d in ARMS.items():
    for k in keys:
        r = d[k]
        if not (r["certified"] is True and r["ok"] is True and r["exhausted"] is False):
            g1.append((a, k))
q_not_ok = [k for k in keys if not (Q[k]["certified"] is True and Q[k]["ok"] is True)]
C["G1"] = {"ic_failures": g1, "q_not_certified_or_not_ok": q_not_ok, "pass": not g1 and not q_not_ok,
           "ic_runs": 2 * len(keys), "q_runs": len(keys)}
F2 = ["group_ops", "rel_mul", "rel_add", "la_mul", "la_add", "trials", "relations"]
g2 = {"compared": 0, "mismatch": [], "all_fields_compared": 0, "all_fields_mismatch": []}
for a, arm in (("w", W_ARM), ("x", X_ARM)):
    old = merge([cheap], arm, P, f"cheap_{a}", [])
    for k, o in old.items():
        n = ARMS[a][k]  # KeyError if missing
        g2["compared"] += 1
        for f in F2:
            if n[f] != o[f]:  # [] : a missing field raises
                g2["mismatch"].append((a, k, f))
        g2["all_fields_compared"] += 1
        if nowall(n) != nowall(o):
            g2["all_fields_mismatch"].append((a, k, sorted(set(nowall(n).items()) ^ set(nowall(o).items()), key=str)[:6]))
g2["pass"] = g2["compared"] == 36 and not g2["mismatch"]
C["G2"] = g2
g3r = runs(g3rows, Q_ARM, P)
C["G3"] = {"rows": len(g3r), "key": [list(key(r)) for r in g3r],
           "steps_group_ops_equal": all(r["steps"] == Qold[key(r)]["steps"] and r["group_ops"] == Qold[key(r)]["group_ops"] for r in g3r),
           "all_fields_equal_except_wall": all(nowall(r) == nowall(Qold[key(r)]) for r in g3r)}
C["G3"]["pass"] = len(g3r) == 1 and C["G3"]["steps_group_ops_equal"] and key(g3r[0]) == (P, 172, 172, 1)

# Q internal consistency: closed-form fields and no canonicalisation charged
C["Q_closed_form"] = all(Q[k]["hash_uncharged"] == Q[k]["steps"] and Q[k]["coef_adds_uncharged"] == 2 * Q[k]["steps"]
                         and Q[k]["compares_uncharged"] == 0 and Q[k]["native"] == Q[k]["steps"]
                         and Q[k]["group_ops"] >= Q[k]["steps"] for k in keys)
C["IC_group_ops_decomposition"] = all(d[k]["group_ops"] == d[k]["walk_ops"] + d[k]["setup_ops"] + d[k]["verify_ops"]
                                      for d in ARMS.values() for k in keys)


# ---------------- costs ----------------
def ic_native(r):
    return r["rel_mul"] + r["rel_add"] + r["la_mul"] + r["la_add"]


def ic(r, w):
    return r["group_ops"] + w * ic_native(r)


def qc(r, rd, w):
    if rd == "Q3":
        return r["group_ops"] + w * r["compares_uncharged"]
    return r["group_ops"] + w * (r["compares_uncharged"] + r["hash_uncharged"] + r["coef_adds_uncharged"])


def theo(k, neg):
    return math.sqrt(math.pi * inst[k[1]]["r"] / (4.0 if neg else 2.0))


def ratio(ks, num, den):
    return sum(num(k) for k in ks) / sum(den(k) for k in ks)


# ---------------- bootstrap (validator's implementation of PROTOCOL section 6) ----------------
def by_target(ks):
    t = {}
    for k in ks:
        t.setdefault(k[:3], []).append(k)
    return {a: sorted(b) for a, b in sorted(t.items())}


def resample_tf(rng, ks):
    """targets-first: draw all targets, then the seeds of each drawn target."""
    bt = by_target(ks)
    tl = list(bt)
    chosen = [rng.choice(tl) for _ in tl]
    out = []
    for t in chosen:
        out += [rng.choice(bt[t]) for _ in bt[t]]
    return out


def resample_il(rng, ks):
    """interleaved: draw a target, then its seeds, then the next target."""
    bt = by_target(ks)
    tl = list(bt)
    out = []
    for _ in tl:
        t = rng.choice(tl)
        out += [rng.choice(bt[t]) for _ in bt[t]]
    return out


def resample_curves(rng, ks):
    cs = sorted({k[1] for k in ks})
    chosen = [rng.choice(cs) for _ in cs]
    out = []
    for c in chosen:
        out += resample_tf(rng, [k for k in ks if k[1] == c])
    return out


def boot(stat, ks, seed=BSEED, how=resample_tf):
    rng = random.Random(seed)
    v = sorted(stat(how(rng, ks)) for _ in range(NB))
    return {"ratio": stat(ks), "lo": v[int(0.025 * NB)], "hi": v[int(0.975 * NB) - 1],
            "lo_interp": statistics.quantiles(v, n=40, method="inclusive")[0],
            "hi_interp": statistics.quantiles(v, n=40, method="inclusive")[-1]}


knew = [k for k in keys if kidx(k) >= 3]
arms_out = {}
for a, d in ARMS.items():
    o = {}
    for rd in ("Q3", "Q4"):
        for wn, w in WS:
            st = lambda ks, d=d, rd=rd, w=w: ratio(ks, lambda k: ic(d[k], w), lambda k: qc(Q[k], rd, w))
            o[f"{rd}_w{wn}"] = boot(st, keys)
            o[f"{rd}_w{wn}_interleaved"] = boot(st, keys, how=resample_il)
    st31 = lambda ks, d=d: ratio(ks, lambda k: ic(d[k], 1.0), lambda k: qc(Q[k], "Q3", 1.0))
    o["Q3_w1_new_only"] = boot(st31, knew)
    o["Q3_w1_curve_boot"] = boot(st31, keys, how=resample_curves)
    his = [boot(st31, keys, seed=s)["hi"] for s in range(1, 51)]
    o["q3_w1_hi_over_50_seeds"] = {"min": min(his), "median_upper": sorted(his)[25],
                                   "median": statistics.median(his), "max": max(his),
                                   "n_below_1": sum(h < 1 for h in his), "frac_below_1": sum(h < 1 for h in his) / 50}
    for nm, neg in (("plain_theory", False), ("neg_ideal", True)):
        for wn, w in WS:
            stt = lambda ks, d=d, w=w, neg=neg: ratio(ks, lambda k: ic(d[k], w), lambda k: theo(k, neg))
            o[f"vs_{nm}_w{wn}"] = boot(stt, keys)
    o["mean_cost_w0"] = statistics.fmean(ic(d[k], 0) for k in keys)
    o["mean_cost_w0.1"] = statistics.fmean(ic(d[k], 0.1) for k in keys)
    o["mean_cost_w1"] = statistics.fmean(ic(d[k], 1) for k in keys)
    o["share_la"] = statistics.fmean(d[k]["la_mul"] + d[k]["la_add"] for k in keys) / o["mean_cost_w1"]
    o["la_share_of_native"] = sum(d[k]["la_mul"] + d[k]["la_add"] for k in keys) / sum(ic_native(d[k]) for k in keys)
    o["fb_cols"] = sorted({d[k]["fb_cols"] for k in keys})
    o["below_rho_w1_frozen_rule"] = o["Q3_w1"]["hi"] < 1 and o["Q4_w1"]["hi"] < 1
    arms_out[a] = o
N["arms"] = arms_out
N["n_keys"], N["n_targets"] = len(keys), len(by_target(keys))
N["q_mean_over_plain_theory"] = statistics.fmean(Q[k]["group_ops"] for k in keys) / statistics.fmean(theo(k, False) for k in keys)
N["q_steps_mean_over_plain_theory"] = statistics.fmean(Q[k]["steps"] for k in keys) / statistics.fmean(theo(k, False) for k in keys)
N["mean_plain_theory"] = statistics.fmean(theo(k, False) for k in keys)
N["mean_neg_ideal"] = statistics.fmean(theo(k, True) for k in keys)
N["mean_Q_cost"] = {f"{rd}_w{wn}": statistics.fmean(qc(Q[k], rd, w) for k in keys) for rd in ("Q3", "Q4") for wn, w in WS}
N["instances"] = {cs: {"h": inst[cs]["h"], "r": inst[cs]["r"], "order": inst[cs]["order"]} for cs in CURVES16}
N["Q_per_curve_over_theory"] = {cs: statistics.fmean(Q[k]["group_ops"] for k in keys if k[1] == cs)
                                / statistics.fmean(theo(k, False) for k in keys if k[1] == cs) for cs in CURVES16}

# section 5a band against the ideal negation floor (validator's VALIDATION.json reference) and measured Q
band = {}
for wn, w in WS:
    band[wn] = {"ic_x_mean": statistics.fmean(ic(ARMS["x"][k], w) for k in keys),
                "ic_w_mean": statistics.fmean(ic(ARMS["w"][k], w) for k in keys),
                "rho_neg_ideal_floor_mean": N["mean_neg_ideal"],
                "rho_Q_Q3_mean": N["mean_Q_cost"][f"Q3_w{wn}"], "rho_Q_Q4_mean": N["mean_Q_cost"][f"Q4_w{wn}"]}
N["band"] = band
res["raw_pins"] = PINS
json.dump(res, open(OUT, "w"), indent=1, default=list)
print(json.dumps({"checks": C, "cc_star": best, "n_keys": N["n_keys"]}, indent=1, default=list)[:6000])
