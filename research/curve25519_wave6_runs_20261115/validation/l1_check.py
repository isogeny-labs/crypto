#!/usr/bin/env python3
"""Validator recomputation of L1 (faithful member) from the committed cells, written without analyse_l1.py.
Checks: git integrity at <sha>; protocol freeze and the pre-data amendment (append-only, committed before the first
cell started); curve arithmetic by trial division and a primitive-root character sum (#E, twist, primality, size
bound); cell metadata (cmd, seed rule, binary, limits); every (m, D) shared with the recorded smooth-family cells
(any k, sym, gidx 0, complete) with four-digit agreement of mean field_ops and SD agreement; P1-P4; censoring and
completion claims; the divisor set of D.  Usage: l1_check.py <repo> <sha> <out.json>"""
import glob, hashlib, json, os, subprocess, sys
import numpy as np
REPO, SHA, OUT = sys.argv[1:4]
STUDY = "research/curve25519_wave6_runs_20261115"; L1 = f"{STUDY}/L1_faithful"
BIN = "/home/user/ctarget-f1/release/curve25519_family"; BIN_SHA = "5abc70ba47b1192df71a5e93d697f095e59a0e5f0132cf47db1d0d34d34e3746"
P, A = 16777141, 7842
def git(*a): return subprocess.run(["git", "-C", REPO, *a], capture_output=True, check=True).stdout
head = git("rev-parse", SHA).decode().strip()
res = {"validated_sha": head, "head_at_run": git("rev-parse", "HEAD").decode().strip(), "checks": {}}
# ---- git integrity
integ = {}
for f in sorted(git("ls-tree", "-r", "--name-only", head, L1).decode().split()) + [f"{STUDY}/checks/faithful_full.py", f"{STUDY}/README.md", f"{STUDY}/run_cells.py"]:
    wt = open(os.path.join(REPO, f), "rb").read()
    integ[f] = {"wt_equals_sha": wt == git("show", f"{head}:{f}"), "sha256": hashlib.sha256(wt).hexdigest(),
                "added": git("log", head, "--diff-filter=A", "--format=%h %cI", "--", f).decode().split("\n")[0],
                "n_commits": len([l for l in git("log", head, "--format=%h", "--", f).decode().split() if l])}
res["git_integrity"] = integ
res["checks"]["wt_equals_validated_sha"] = all(v["wt_equals_sha"] for v in integ.values())
res["checks"]["cells_unmodified_after_add"] = all(v["n_commits"] == 1 for k, v in integ.items() if "/cells/" in k)
# ---- freeze and amendment
frozen = git("show", f"c66e2b3f4f:{L1}/PROTOCOL.md"); amended = git("show", f"96ac7772bf:{L1}/PROTOCOL.md"); now = git("show", f"{head}:{L1}/PROTOCOL.md")
amend_time = git("show", "-s", "--format=%cI %aI", "96ac7772bf").decode().split()
freeze_time = git("show", "-s", "--format=%cI", "c66e2b3f4f").decode().strip()
cells = {}
for f in sorted(glob.glob(os.path.join(REPO, L1, "cells", "*.json"))):
    j = json.load(open(f)); cells[(j["m"], j["d_or_t"])] = (os.path.relpath(f, REPO), j)
first = min(j["ext"]["started_utc"] for _, j in cells.values())
cell_commits = sorted(set(v["added"] for k, v in integ.items() if "/cells/" in k))
res["freeze"] = {"freeze_commit_time": freeze_time, "amendment_commit_time_committer_author": amend_time, "first_cell_started_utc": first,
                 "amendment_precedes_first_cell_s": None, "amendment_append_only": amended.startswith(frozen),
                 "protocol_unchanged_since_amendment": amended == now, "cell_add_commits": cell_commits,
                 "earlier_L1_paths_in_history": sorted(set(git("log", head, "--format=", "--name-only", "--", L1).decode().split()) - set(integ))}
from datetime import datetime
ta = datetime.fromisoformat(amend_time[0]); tf = datetime.fromisoformat(first.replace("Z", "+00:00"))
res["freeze"]["amendment_precedes_first_cell_s"] = (tf - ta).total_seconds()
res["checks"]["amendment_before_data"] = res["freeze"]["amendment_precedes_first_cell_s"] > 0 and res["freeze"]["amendment_append_only"] and res["freeze"]["protocol_unchanged_since_amendment"]
# ---- arithmetic
def is_prime_td(n):
    if n < 2: return False
    if n % 2 == 0: return n == 2
    f = 3
    while f * f <= n:
        if n % f == 0: return False
        f += 2
    return True
def factor(n):
    o, f = [], 2
    while f * f <= n:
        while n % f == 0: o.append(f); n //= f
        f += 1
    if n > 1: o.append(n)
    return o
def v2(n):
    c = 0
    while n % 2 == 0: n //= 2; c += 1
    return c
qs = sorted(set(factor(P - 1))); w = 2
while any(pow(w, (P - 1) // q, P) == 1 for q in qs): w += 1
B = 4096; small = np.empty(B, dtype=np.int64); v = 1
for i in range(B): small[i] = v; v = v * w % P
nb = (P - 1 + B - 1) // B; big = np.empty(nb, dtype=np.int64); gb = pow(w, B, P); v = 1
for i in range(nb): big[i] = v; v = v * gb % P
pw = (np.outer(big, small) % P).ravel()[: P - 1]
chi = np.zeros(P, dtype=np.int8); chi[pw] = np.where(np.arange(P - 1) % 2 == 0, 1, -1).astype(np.int8)
assert np.count_nonzero(chi) == P - 1
u = np.arange(P, dtype=np.int64); u2 = u * u % P; f = (u2 * u % P + A * u2 % P + u) % P
S = int(chi[f].sum(dtype=np.int64)); n = P + 1 + S; nt = 2 * P + 2 - n
ar = {"p": P, "p_eq_2^24-75": P == 2 ** 24 - 75, "p_prime_trial_division": is_prime_td(P), "p_mod_8": P % 8,
      "p_minus_1_factors": factor(P - 1), "279619_prime": is_prime_td(279619), "v2_p_plus_1": v2(P + 1),
      "A": A, "A_mod_4": A % 4, "A2_minus_4_euler": pow((A * A - 4) % P, (P - 1) // 2, P), "A2_minus_4_chi_table": int(chi[(A * A - 4) % P]),
      "char_sum": S, "n_E": n, "n_E_over_8": n / 8, "r": n // 8, "r_prime": n % 8 == 0 and is_prime_td(n // 8),
      "n_twist": nt, "r_twist": nt // 4, "r_twist_prime": nt % 4 == 0 and is_prime_td(nt // 4), "r_gt_2^21": n // 8 > 2 ** 21,
      "claims": {"n_E": 8 * 2098009, "n_twist": 4 * 4192553}}
ar["ok"] = (ar["p_prime_trial_division"] and ar["p_mod_8"] == 5 and ar["p_minus_1_factors"] == [2, 2, 3, 5, 279619] and ar["279619_prime"]
            and ar["v2_p_plus_1"] == 1 and ar["A_mod_4"] == 2 and ar["A2_minus_4_euler"] == P - 1 and ar["A2_minus_4_chi_table"] == -1
            and n == 8 * 2098009 and nt == 4 * 4192553 and ar["r_prime"] and ar["r_twist_prime"] and ar["r_gt_2^21"])
res["arithmetic"] = ar; res["checks"]["arithmetic"] = ar["ok"]
# ---- divisor set
Ds = sorted(d for (_, d) in cells)
res["divisors"] = {"D_set": sorted(set(Ds)), "divide_p_minus_1": all((P - 1) % d == 0 for d in set(Ds)),
                   "divide_(p-1)/4": {d: ((P - 1) // 4) % d == 0 for d in sorted(set(Ds))},
                   "divisors_of_60_ge_3": [d for d in range(3, 61) if 60 % d == 0]}
# ---- cell metadata
meta = {}
for (m, d), (path, j) in sorted(cells.items()):
    seed = 20261108 + 1000 * m + d
    cmd = f"ulimit -v 6000000; RAYON_NUM_THREADS=1 {BIN} f4sub p=16777141,A=7842 sub {d} 0 mont {m} sym 6 {seed} 1800"
    e = j["ext"]; ok = {"cmd": e["cmd"] == cmd, "bin": e["bin_sha256"] == BIN_SHA, "mem": e["mem_kb"] == 6000000, "cap": e["cap_s"] == 1800}
    if not j.get("capped"):
        pts = j["per_target"]
        ok.update({"seed": j["seed"] == seed, "p": j["p"] == P, "a_mont": j.get("a_mont") == A, "k": j["k"] == 24, "base": j["base"] == "sub",
                   "gidx": j["gidx"] == 0, "frame": j["frame"] == "mont", "form": j["form"] == "sym", "v_size": j["v_size"] == d,
                   "done6": j["targets_done"] == 6 and len(pts) == 6, "complete": all(t["complete"] and not t["timed_out"] for t in pts),
                   "mean": abs(sum(t["field_ops"] for t in pts) / 6 - j["mean_field_ops"]) <= 1e-9 * j["mean_field_ops"], "exit0": e["exit"] == 0})
    meta[path] = ok
res["cell_metadata"] = meta; res["checks"]["cell_metadata_all_ok"] = all(all(v.values()) for v in meta.values())
# ---- recorded smooth-family cells
rec = {}
for d0 in ("research/curve25519_exponent_study_20261108/cells", "research/curve25519_exponent_study_ext_20261110/cells"):
    for fpath in sorted(glob.glob(os.path.join(REPO, d0, "*.json"))):
        rel = os.path.relpath(fpath, REPO); assert open(fpath, "rb").read() == git("show", f"{head}:{rel}"), rel
        j = json.load(open(fpath))
        if j.get("form") != "sym" or j.get("gidx") != 0 or j.get("capped") or not j.get("per_target") or j.get("base", "sub") != "sub": continue
        rec.setdefault((j["m"], j["d_or_t"]), {})[(j["k"], rel)] = j
shared = []
for (m, d), (path, j) in sorted(cells.items()):
    if j.get("capped") or (m, d) not in rec: continue
    T = j["mean_field_ops"]; sd = sorted(set(t["solving_degree_max"] for t in j["per_target"]))
    for (k, rel), rj in sorted(rec[(m, d)].items()):
        rT = rj["mean_field_ops"]; rsd = sorted(set(t["solving_degree_max"] for t in rj["per_target"]))
        shared.append({"m": m, "D": d, "k": k, "recorded": rel, "faithful_T": T, "recorded_T": rT, "ratio": T / rT,
                       "four_digit_equal": f"{T:.4g}" == f"{rT:.4g}", "exact_equal": T == rT, "faithful_SD": sd, "recorded_SD": rsd, "SD_equal": sd == rsd})
# de-duplicate identical recorded files present in both study dirs (same k, m, D)
uniq = {}
for s in shared: uniq.setdefault((s["m"], s["D"], s["k"]), s)
res["shared"] = shared; res["shared_unique_mDk"] = len(uniq)
res["checks"]["shared_four_digits_and_SD"] = all(s["four_digit_equal"] and s["SD_equal"] for s in shared)
# ---- predictions
def T(m, d): return cells[(m, d)][1]["mean_field_ops"]
def SD(m, d): return sorted(set(t["solving_degree_max"] for t in cells[(m, d)][1]["per_target"]))
comp = {(2, 3): 1.65e3, (2, 5): 5.34e3, (3, 3): 1.90e4, (3, 5): 2.02e5, (3, 12): 7.75e8}
compsrc = {}
for (m, d), c in comp.items():
    k = 20 if (m, d) == (3, 12) else 22
    rj = [v for (kk, rel), v in rec[(m, d)].items() if kk == k][0]
    compsrc[f"m{m}_D{d}"] = {"protocol_value": c, "record_k": k, "record_T": rj["mean_field_ops"], "protocol_value_matches_record_3sf": f"{rj['mean_field_ops']:.2e}" == f"{c:.2e}"}
res["comparators"] = compsrc
p1 = {f"m{m}_D{d}": T(m, d) / comp[(m, d)] for m in (2, 3) for d in (3, 5)}
p2 = {f"m{m}_D{d}": SD(m, d) for m in (2, 3) for d in (3, 5)}
m3 = {d: j for (m, d), (_, j) in cells.items() if m == 3}
cens = sorted(d for d, j in m3.items() if j.get("capped")); done9 = sorted(d for d, j in m3.items() if not j.get("capped") and d >= 9)
res["P1"] = {"ratio_to_comparator": p1, "holds": all(0.5 <= v <= 2 for v in p1.values())}
res["P2"] = {"SD": p2, "holds": all(v == [int(k.split("_D")[1]) + 1] for k, v in p2.items())}
res["P3"] = {"m3_censored_at": cens, "m3_complete_D_ge_9": done9, "span": done9[-1] / done9[0], "estimator_applicable": len(done9) >= 4 and done9[-1] / done9[0] >= 4,
             "holds": bool(cens) and cens[0] <= 60 and not (len(done9) >= 4 and done9[-1] / done9[0] >= 4)}
res["P4"] = {"T_m3_D12": T(3, 12), "ratio_to_7.75e8": T(3, 12) / 7.75e8, "holds": 0.5 <= T(3, 12) / 7.75e8 <= 2}
j60 = cells[(3, 60)][1]
res["censoring"] = {"m3_D60": {"capped": j60.get("capped"), "exit": j60["ext"]["exit"], "wall_s": j60["ext"]["wall_s"], "stderr": j60.get("stderr"),
                               "per_target_rows": len(j60.get("per_target", []))},
                    "m3_D20": {"T": T(3, 20), "T_3sf": f"{T(3, 20):.2e}", "wall_s": cells[(3, 20)][1]["ext"]["wall_s"], "SD": SD(3, 20)},
                    "m3_D30": {"T": T(3, 30), "T_3sf": f"{T(3, 30):.2e}", "wall_s": cells[(3, 30)][1]["ext"]["wall_s"], "SD": SD(3, 30)},
                    "m2_all_complete": all(not j.get("capped") for (m, d), (_, j) in cells.items() if m == 2),
                    "chain_order_m3": sorted((j["ext"]["started_utc"], d) for d, j in m3.items())}
res["checks"].update({k: res[k]["holds"] for k in ("P1", "P2", "P3", "P4")})
json.dump(res, open(OUT, "w"), indent=1, default=str)
print(json.dumps({k: res[k] for k in ("validated_sha", "head_at_run", "checks", "freeze", "arithmetic", "divisors", "comparators", "P1", "P2", "P3", "P4", "censoring", "shared_unique_mDk")}, indent=1, default=str))
for s in shared: print(s["m"], s["D"], s["k"], s["faithful_T"], s["recorded_T"], round(s["ratio"], 7), s["four_digit_equal"], s["exact_equal"], s["faithful_SD"], s["recorded_SD"], s["recorded"].split("/")[1])
