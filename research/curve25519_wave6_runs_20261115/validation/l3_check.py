#!/usr/bin/env python3
"""Validator recomputation of L3 (flat vs chain) from the committed cells, written without analyse_l3.py.
Checks: git integrity (working tree == blobs at the validated sha; each cell added once), protocol freeze order, cell metadata
(seed rule, cmd, binary, limits), binary control (P5) per target, shared targets, flat SD = D + 1 (P1),
chain/flat ratios against the README table, P2-P4 verdicts, the non-preregistered OLS fits with a t quantile
computed here (no table), censored cells, wall times, and the wall-time per field operation (observation).
Usage: l3_check.py <repo> <sha> <out.json>   (every file read is compared with its blob at <sha>)"""
import glob, hashlib, json, math, os, subprocess, sys

REPO, SHA, OUT = sys.argv[1], sys.argv[2], sys.argv[3]
STUDY = "research/curve25519_wave6_runs_20261115"; L3 = f"{STUDY}/L3_flat"
BIN_SHA = "5abc70ba47b1192df71a5e93d697f095e59a0e5f0132cf47db1d0d34d34e3746"
BIN = "/home/user/ctarget-f1/release/curve25519_family"
def git(*a): return subprocess.run(["git", "-C", REPO, *a], capture_output=True, check=True).stdout
def jl(path): return json.load(open(os.path.join(REPO, path)))
res = {"checks": {}}

# ---- git integrity
head = git("rev-parse", SHA).decode().strip(); res["validated_sha"] = head; res["head_at_run"] = git("rev-parse", "HEAD").decode().strip()
files = sorted(git("ls-tree", "-r", "--name-only", head, L3).decode().split())
integ = {}
for f in files:
    wt = open(os.path.join(REPO, f), "rb").read(); hb = git("show", f"{head}:{f}")
    adds = git("log", head, "--diff-filter=A", "--format=%h %cI", "--", f).decode().split("\n")[0]
    mods = [l for l in git("log", head, "--format=%h", "--", f).decode().split() if l]
    integ[f] = {"wt_equals_sha": wt == hb, "sha256": hashlib.sha256(wt).hexdigest(), "added": adds, "n_commits": len(mods)}
res["git_integrity"] = integ
status = git("status", "--porcelain", "--", STUDY).decode().splitlines()
res["git_status_study"] = status
cells_dir = f"{L3}/cells"
res["checks"]["cells_unmodified_after_add"] = all(v["n_commits"] == 1 for k, v in integ.items() if "/cells/" in k)
res["checks"]["wt_equals_validated_sha"] = all(v["wt_equals_sha"] for v in integ.values())
proto_add = git("log", head, "--diff-filter=A", "--format=%H %cI", "--", f"{L3}/PROTOCOL.md").decode().split()
proto_same = git("diff", "c66e2b3f4f", head, "--", f"{L3}/PROTOCOL.md") == b""

# ---- cells
cells = {}
for f in sorted(glob.glob(os.path.join(REPO, cells_dir, "*.json"))):
    assert open(f, "rb").read() == git("show", f"{head}:{os.path.relpath(f, REPO)}")
    j = json.load(open(f)); cells[(j.get("kind") or j.get("base"), j["m"], j["d_or_t"])] = (os.path.relpath(f, REPO), j)
first_start = min(j["ext"]["started_utc"] for _, j in cells.values())
res["freeze"] = {"protocol_added": proto_add, "protocol_unchanged_at_head": proto_same, "first_cell_started_utc": first_start,
                 "freeze_precedes_data": proto_add[1] < first_start.replace("Z", "+00:00")}

meta = {}
for (kind, m, d), (path, j) in sorted(cells.items()):
    seed = 20261108 + 1000 * m + d
    cmd = f"ulimit -v 6000000; RAYON_NUM_THREADS=1 {BIN} f4sub 20 {kind} {d} 0 mont {m} sym 6 {seed} 1800"
    e = j["ext"]; pts = j.get("per_target", [])
    ok = {"cmd": e["cmd"] == cmd, "bin": e["bin_sha256"] == BIN_SHA, "mem": e["mem_kb"] == 6000000, "cap": e["cap_s"] == 1800}
    if not j.get("capped"):
        ok.update({"seed": j["seed"] == seed, "p": j["p"] == 1048573, "k": j["k"] == 20, "frame": j["frame"] == "mont",
                   "gidx": j["gidx"] == 0, "form": j["form"] == "sym", "base": j["base"] == kind, "v_size": j["v_size"] == d,
                   "done6": j["targets_done"] == 6 and len(pts) == 6,
                   "all_complete": all(t["complete"] and not t["timed_out"] for t in pts),
                   "mean_recomputed": abs(sum(t["field_ops"] for t in pts) / 6 - j["mean_field_ops"]) < 1e-6 * j["mean_field_ops"],
                   "exit0": e["exit"] == 0})
    meta[path] = ok
res["cell_metadata"] = meta
res["checks"]["cell_metadata_all_ok"] = all(all(v.values()) for v in meta.values())

# ---- recorded chain cells (frozen binary dc2c30a4)
rec = {}
for d0 in ("research/curve25519_exponent_study_20261108/cells", "research/curve25519_exponent_study_ext_20261110/cells"):
    for f in sorted(glob.glob(os.path.join(REPO, d0, "k20_m[23]_sym_d*_g0.json"))):
        rel = os.path.relpath(f, REPO); assert open(f, "rb").read() == git("show", f"{head}:{rel}"), rel
        j = json.load(open(f)); key = (j["m"], j["d_or_t"])
        if key in rec: rec[key + ("dup",)] = (os.path.relpath(f, REPO), j)
        else: rec[key] = (os.path.relpath(f, REPO), j)
res["recorded_duplicates"] = [k for k in rec if len(k) == 3]

def sdmax(j): return max(t["solving_degree_max"] for t in j["per_target"])
def tlist(j, keys=("target_x", "field_ops", "solving_degree_max", "max_cols", "max_rows", "steps", "inconsistent", "basis_len")):
    return [tuple(t[k] for k in keys) for t in j["per_target"]]

# ---- P5: binary control
p5 = {}
for (m, d) in ((3, 9), (3, 21), (2, 57)):
    _, cj = cells[("sub", m, d)]; rp, rj = rec[(m, d)]
    p5[f"m{m}_D{d}"] = {"recorded": rp, "per_target_equal": tlist(cj) == tlist(rj),
                        "field_ops": [t["field_ops"] for t in cj["per_target"]], "mean_new": cj["mean_field_ops"], "mean_rec": rj["mean_field_ops"],
                        "recorded_bin": rj.get("ext", {}).get("bin_sha256", "dc2c30a4 (parent study, frozen)")}
res["P5_binary_control"] = p5
res["checks"]["P5"] = all(v["per_target_equal"] for v in p5.values())

# ---- README table (claims) and recomputation
CLAIM = {  # (m, D): (flat T, flat SD, wall s, chain T, chain SD, ratio)
 (3, 9): ("7.59e6", 10, 0, "1.48e8", 7, 19.5), (3, 12): ("2.73e7", 13, 1, "7.75e8", 7, 28.4),
 (3, 14): ("4.69e7", 15, 2, "7.77e8", 10, 16.6), (3, 18): ("1.42e8", 19, 6, "2.43e10", 7, 170.5),
 (3, 21): ("2.65e8", 22, 12, "2.79e9", 11, 10.5), (3, 27): ("7.27e8", 28, 38, "2.63e11", 8, 362.6),
 (3, 28): ("1.00e9", 29, 47, "2.02e11", 11, 201.0), (3, 36): ("4.13e9", 37, 202, "censored", None, None),
 (2, 9): ("1.94e4", 10, 0, "5.22e4", 5, 2.7), (2, 21): ("1.20e5", 22, 0, "4.35e5", 9, 3.6),
 (2, 27): ("2.06e5", 28, 0, "1.79e6", 5, 8.7), (2, 57): ("1.07e6", 58, 2, "3.88e6", 21, 3.6),
 (2, 63): ("1.34e6", 64, 3, "2.23e7", 9, 16.7), (2, 133): ("7.65e6", 134, 143, "3.51e7", 22, 4.6),
 (2, 171): ("1.42e7", 172, 618, "3.83e8", 21, 27.1), (2, 189): ("1.81e7", 190, 973, "3.94e8", 10, 21.7),
 (2, 219): ("2.63e7", 220, 2232, "6.18e7", 75, 2.4)}
def e3(x):
    m, ex = f"{x:.2e}".split("e"); return f"{m}e{int(ex)}"
table, mism = [], []
for (m, d), cl in sorted(CLAIM.items()):
    _, fj = cells[("subflat", m, d)]; rp, rj = rec[(m, d)]
    fT = fj["mean_field_ops"]; fsd = sdmax(fj); fsdmin = min(t["solving_degree_max"] for t in fj["per_target"])
    wall = fj["ext"]["wall_s"]
    row = {"m": m, "D": d, "flat_T": fT, "flat_T_3sf": e3(fT), "flat_SD_max": fsd, "flat_SD_min": fsdmin, "wall_s": wall,
           "targets_equal_recorded": [t["target_x"] for t in fj["per_target"]] == ([t["target_x"] for t in rj["per_target"]] if rj.get("per_target") else None),
           "recorded": rp}
    if rj.get("capped"):
        row.update({"chain": "censored", "chain_failure": rj.get("failure"), "chain_mem_kb": rj.get("ext", {}).get("mem_kb"),
                    "targets_equal_recorded": "recorded cell censored (no targets)"})
    else:
        cT = rj["mean_field_ops"]; row.update({"chain_T": cT, "chain_T_3sf": e3(cT), "chain_SD_max": sdmax(rj), "ratio": cT / fT})
    agree = {"flat_T": row["flat_T_3sf"] == cl[0], "flat_SD": fsd == cl[1] and fsdmin == cl[1], "wall": f"{wall:.0f}" == str(cl[2])}
    if cl[3] == "censored": agree["chain"] = row.get("chain") == "censored"
    else:
        agree.update({"chain_T": row["chain_T_3sf"] == cl[3], "chain_SD": row["chain_SD_max"] == cl[4], "ratio": f"{row['ratio']:.1f}" == f"{cl[5]:.1f}"})
    row["agrees_with_README"] = agree
    if not all(agree.values()): mism.append({"m": m, "D": d, "claimed": cl, "agree": agree, "flat_T": fT})
    table.append(row)
res["table"] = table; res["README_table_mismatches"] = mism
res["checks"]["targets_shared_all_D"] = all(r["targets_equal_recorded"] is True for r in table if r.get("chain") != "censored")
# m = 3 D = 36: the recorded chain cell is censored, so the targets are checked against the seed law (l1_l3_targets_decomp.py)

# ---- P1
p1 = {f"m{r['m']}_D{r['D']}": (r["flat_SD_min"], r["flat_SD_max"]) for r in table}
res["P1"] = {"sd_min_max": p1, "holds": all(a == b == int(k.split("_D")[1]) + 1 for k, (a, b) in p1.items())}
# ---- P2
R = {(r["m"], r["D"]): r.get("ratio") for r in table}
res["P2"] = {"band_70_260": {d: (R[(3, d)], 70 <= R[(3, d)] <= 260) for d in (27, 28)},
             "above_10": {d: (R[(3, d)], R[(3, d)] > 10) for d in (9, 12, 14, 18, 21)},
             "min_ratio_D_le_21": min(R[(3, d)] for d in (9, 12, 14, 18, 21)),
             "refutation_rule_triggered (below 10 at D<=21 or below 70 at D=27,28)":
                 any(R[(3, d)] < 10 for d in (9, 12, 14, 18, 21)) or any(R[(3, d)] < 70 for d in (27, 28))}
res["P2"]["holds_as_written"] = all(v[1] for v in res["P2"]["band_70_260"].values()) and all(v[1] for v in res["P2"]["above_10"].values())
# ---- P3
_, j36 = cells[("subflat", 3, 36)]
res["P3"] = {"complete": not j36["capped"] and all(t["complete"] for t in j36["per_target"]), "exit": j36["ext"]["exit"],
             "max_target_s": max(t["ms"] for t in j36["per_target"]) / 1000, "cell_wall_s": j36["ext"]["wall_s"],
             "ulimit_v_kb": 6000000 if "ulimit -v 6000000" in j36["ext"]["cmd"] else None}
res["P3"]["holds"] = res["P3"]["complete"] and res["P3"]["exit"] == 0 and res["P3"]["max_target_s"] < 1800 and res["P3"]["ulimit_v_kb"] == 6000000
# ---- P4
def factor(n):
    out, f = [], 2
    while f * f <= n:
        while n % f == 0: out.append(f); n //= f
        f += 1
    if n > 1: out.append(n)
    return out
p4 = {}
for r in table:
    if r["m"] != 2: continue
    pred = 9.7e4 * (r["D"] / 19) ** 2.2
    p4[r["D"]] = {"factors": factor(r["D"]), "ratio_chain_flat": r["ratio"], "pred_prime_curve": pred, "flat_over_pred": r["flat_T"] / pred}
res["P4"] = {"rows": p4, "ratio_gt_1_where_2plus_factors": all(v["ratio_chain_flat"] > 1 for v in p4.values() if len(v["factors"]) >= 2),
             "within_3x_all": all(1 / 3 <= v["flat_over_pred"] <= 3 for v in p4.values()),
             "README_quoted": {"57": [1.1e6, 1.07e6], "133": [7.0e6, 7.65e6]}}
res["P4"]["holds"] = res["P4"]["ratio_gt_1_where_2plus_factors"] and res["P4"]["within_3x_all"]

# ---- t quantile (own): Student t CDF by Simpson integration of the density, inverted by bisection
def t_cdf(x, nu):
    c = math.exp(math.lgamma((nu + 1) / 2) - math.lgamma(nu / 2)) / math.sqrt(nu * math.pi)
    f = lambda t: c * (1 + t * t / nu) ** (-(nu + 1) / 2)
    n = 20000; h = x / n; s = f(0) + f(x)
    for i in range(1, n): s += (4 if i % 2 else 2) * f(i * h)
    return 0.5 + s * h / 3
def t_q975(nu):
    lo, hi = 0.0, 50.0
    for _ in range(80):
        mid = (lo + hi) / 2
        if t_cdf(mid, nu) < 0.975: lo = mid
        else: hi = mid
    return (lo + hi) / 2
def ols(xs, ys):
    n = len(xs); mx = sum(xs) / n; my = sum(ys) / n
    sxx = sum((x - mx) ** 2 for x in xs); sxy = sum((x - mx) * (y - my) for x, y in zip(xs, ys))
    b = sxy / sxx; a = my - b * mx; rss = sum((y - a - b * x) ** 2 for x, y in zip(xs, ys))
    se = math.sqrt(rss / (n - 2) / sxx); t = t_q975(n - 2)
    return {"n": n, "slope": b, "lo95": b - t * se, "hi95": b + t * se, "se": se, "t975": t, "df": n - 2}
fits = {}
for m, claim in ((3, (4.40, 4.10, 4.71)), (2, (2.26, 2.21, 2.31))):
    sel = sorted((r["D"], r["flat_T"]) for r in table if r["m"] == m and r["D"] >= 9)
    vs = [cells[("subflat", m, d)][1]["v_size"] for d, _ in sel]
    f = ols([math.log(v) for v in vs], [math.log(t) for _, t in sel])
    f.update({"D": [d for d, _ in sel], "span": sel[-1][0] / sel[0][0], "claimed": claim,
              "agrees_2dp": [round(f["slope"], 2), round(f["lo95"], 2), round(f["hi95"], 2)] == list(claim)})
    fits[f"m{m}"] = f
res["fits_non_preregistered"] = fits

# ---- censored cells
_, j399 = cells[("subflat", 2, 399)]
res["censored"] = {"m2_D399": {"capped": j399["capped"], "exit": j399["ext"]["exit"], "wall_s": j399["ext"]["wall_s"],
                               "stderr": j399.get("stderr"), "per_target_rows": len(j399.get("per_target", [])),
                               "note": "the engine prints one JSON object at the end; a crash loses any finished targets, so 'no target finished' is not observable from the cell"},
                   "m3_chain_stops": sorted(d for (k, m, d), (_, j) in cells.items() if k == "subflat" and m == 3 and not j.get("capped")),
                   "m2_chain_order": sorted(((j["ext"]["started_utc"], d) for (k, m, d), (_, j) in cells.items() if k == "subflat" and m == 2))}

# ---- observation: wall time per counted field operation (F4 solve time 'ms' per target; different binaries and loads)
obs = []
for r in table:
    _, fj = cells[("subflat", r["m"], r["D"])]; rp, rj = rec[(r["m"], r["D"])]
    f_ns = sum(t["ms"] for t in fj["per_target"]) * 1e6 / sum(t["field_ops"] for t in fj["per_target"])
    row = {"m": r["m"], "D": r["D"], "flat_ns_per_op": f_ns, "flat_solve_s": sum(t["ms"] for t in fj["per_target"]) / 1000}
    if rj.get("per_target"):
        c_ns = sum(t["ms"] for t in rj["per_target"]) * 1e6 / sum(t["field_ops"] for t in rj["per_target"])
        row.update({"chain_ns_per_op": c_ns, "chain_solve_s": sum(t["ms"] for t in rj["per_target"]) / 1000,
                    "solve_time_ratio_chain_over_flat": sum(t["ms"] for t in rj["per_target"]) / sum(t["ms"] for t in fj["per_target"])})
    obs.append(row)
res["wall_per_op_observation"] = obs
res["checks"].update({"P1": res["P1"]["holds"], "P2_as_written": res["P2"]["holds_as_written"], "P3": res["P3"]["holds"], "P4": res["P4"]["holds"]})
json.dump(res, open(OUT, "w"), indent=1, default=str)
print(json.dumps({"validated_sha": head, "head_at_run": res["head_at_run"], "checks": res["checks"], "freeze": res["freeze"], "mismatches": mism, "P2": res["P2"],
                  "fits": {k: {kk: v[kk] for kk in ("n", "slope", "lo95", "hi95", "t975", "span", "claimed", "agrees_2dp")} for k, v in fits.items()},
                  "P4": {d: (round(v["flat_over_pred"], 3), v["factors"]) for d, v in p4.items()}, "status": status}, indent=1, default=str))
for o in obs: print(o)
