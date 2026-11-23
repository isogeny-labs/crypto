#!/usr/bin/env python3
"""Discrete-log certificates for the sampled keys, checked with the validator's own group law (jac.py).

Genus 4 (p = 54293, arm x*): the prior validator's harness val_po (sha256 81bbb745..., source
research/genus4_prime_order_20261118/validation/scripts/val_po.rs) prints f, G, T, t and, per seed,
the recovered logarithm and the counted fields.  Genus 5: genus_ladder.blpt (sha256 e210549a...) with
GENUS_LADDER_EXPORT_CORE writes, per linear-algebra solve, t_secret, unit_t, h_inv and the solved
target coordinate coord_out; the recovered log is coord_out * h_inv mod r (to_log in the source).

For every sampled key the validator rebuilds (G, t, T) from (p, g, curve_seed, target_seed, h, r) with
jac.instance_spec, checks G valid, G != 0, [r]G = 0, T = [t]G, and then the solver's recovered log L:
L = t and [L]G = T.  The harness / export rows' counts must equal the committed rows (else the log is
not the committed run's).  Tamper controls: L + 1, a changed T coefficient and a count + 1 must each be
caught.  Writes validation/out/certs.json.
"""
import glob, json, os, sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import jac

V = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
S = os.path.dirname(V)
OUT = os.path.join(V, "out")
COUNTS = ["group_ops", "rel_mul", "rel_add", "la_mul", "la_add", "trials", "relations", "fb_cols", "certified", "ok"]


def rows(path):
    return [json.loads(l) for l in open(path) if l.strip()]


def committed(block):
    rr = rows(os.path.join(S, block + ".jsonl"))
    return ({(r["curve_seed"], r["target_seed"], r["seed"]): r for r in rr if r["kind"] == "run"},
            {r["curve_seed"]: r for r in rr if r["kind"] == "instance"})


def check_key(p, g, cs, ts, inst, L_list, J=None):
    J = J or jac.Jac(p, g, inst["f"])
    G, t, T = jac.instance_spec(J, inst["h"], inst["r"], ts)
    r = inst["r"]
    c = {"f_eq_random_f": inst["f"] == jac.random_f(p, g, cs), "G_valid": J.is_valid(G), "G_nonzero": G != J.ID,
         "rG_zero": J.mul(G, r) == J.ID, "T_eq_tG": J.mul(G, t) == T, "t": t,
         "logs": [], "tamper_caught": {}}
    for L in L_list:
        c["logs"].append({"L": L, "L_eq_t": L == t, "LG_eq_T": J.mul(G, L) == T})
    if L_list:
        L = L_list[0]
        c["tamper_caught"]["L+1"] = J.mul(G, (L + 1) % r) != T
        u, v = T
        T2 = (u, jac.padd(v, [1], p))
        c["tamper_caught"]["T_coeff"] = (not J.is_valid(T2)) or J.mul(G, L) != T2
    return c, G, T, J


out = {"genus4_valpo": [], "genus5_export": [], "summary": {}}
# ---- genus 4 via val_po
for f in sorted(glob.glob(os.path.join(OUT, "replay", "*_valpo.jsonl"))):
    tag = os.path.basename(f)[:-len("_valpo.jsonl")]
    block = "_".join(tag.split("_")[:3])
    cr, ci = committed(block)
    rr = rows(f)
    ins = [r for r in rr if r["kind"] == "instance"][0]
    runs = [r for r in rr if r["kind"] == "run"]
    p, cs, ts = ins["p"], ins["curve_seed"], ins["target_seed"]
    c, G, T, J = check_key(p, 4, cs, ts, ci[cs], [r["recovered"] for r in runs])
    c["harness_G_T_t_eq_validator"] = ([list(x) for x in G] == ins["G"] and [list(x) for x in T] == ins["T"] and c["t"] == ins["t"])
    c["harness_f_eq_committed"] = ins["f"] == ci[cs]["f"] and ins["r"] == ci[cs]["r"]
    c["counts_eq_committed"] = []
    for r in runs:
        com = cr[(cs, ts, r["seed"])]
        c["counts_eq_committed"].append({x: r[x] == com[x] for x in COUNTS})
    c["tamper_caught"]["count+1"] = any(cr[(cs, ts, runs[0]["seed"])][x] != runs[0][x] + 1 for x in ("group_ops",))
    c.update({"tag": tag, "p": p, "curve_seed": cs, "target_seed": ts})
    c["ok"] = (c["f_eq_random_f"] and c["G_valid"] and c["G_nonzero"] and c["rG_zero"] and c["T_eq_tG"]
               and c["harness_G_T_t_eq_validator"] and c["harness_f_eq_committed"] and len(runs) == 2
               and all(l["L_eq_t"] and l["LG_eq_T"] for l in c["logs"]) and all(all(d.values()) for d in c["counts_eq_committed"])
               and all(c["tamper_caught"].values()))
    out["genus4_valpo"].append(c)

# ---- genus 5 via the core export
for f in sorted(glob.glob(os.path.join(OUT, "replay", "*_blpt_core.jsonl"))):
    tag = os.path.basename(f)[:-len("_blpt_core.jsonl")]
    block = "_".join(tag.split("_")[:3])
    cr, ci = committed(block)
    rep = rows(os.path.join(OUT, "replay", tag + "_blpt.jsonl"))
    runs = [r for r in rep if r["kind"] == "run"]
    if not any(r["kind"] == "done" for r in rep):
        continue
    core = rows(f)
    cs, ts = runs[0]["curve_seed"], runs[0]["target_seed"]
    inst = ci[cs]
    Ls = []
    for e in core:
        r_ = e["r"]
        L = e["coord_out"] * e["h_inv"] % r_ if e["unit_t"] else e["coord_out"]
        Ls.append(L)
    c, G, T, J = check_key(inst["p"], 5, cs, ts, inst, Ls)
    c["export_lines"] = len(core)
    c["core_file"] = os.path.relpath(f, V)
    c["core_file_sha256"] = __import__("hashlib").sha256(open(f, "rb").read()).hexdigest()
    c["core_file_note"] = "over 64 KiB, not committed (validation/.gitignore); digest of each solve below"
    c["export_digest"] = [{k: e[k] for k in ("p", "g", "r", "h", "t_secret", "unit_t", "h_inv", "coord_out", "ncols", "ct", "wied_mul", "wied_add")}
                          for e in core]
    c["export_t_secret_eq_validator_t"] = all(e["t_secret"] == c["t"] for e in core)
    c["export_r_h_eq_instance"] = all(e["r"] == inst["r"] and e["h"] == inst["h"] and e["p"] == inst["p"] and e["g"] == 5 for e in core)
    c["runs"] = len(runs)
    c["attempts_total"] = sum(r["attempts"] for r in runs)
    c["export_wied_counts"] = [(e["wied_mul"], e["wied_add"]) for e in core]
    c["row_wied_counts"] = [(r["la_wied_mul"], r["la_wied_add"]) for r in runs]
    c["counts_eq_committed"] = [{x: r[x] == cr[(cs, ts, r["seed"])][x] for x in COUNTS} for r in runs]
    c.update({"tag": tag, "p": inst["p"], "curve_seed": cs, "target_seed": ts})
    c["ok"] = (c["f_eq_random_f"] and c["G_valid"] and c["G_nonzero"] and c["rG_zero"] and c["T_eq_tG"]
               and c["export_t_secret_eq_validator_t"] and c["export_r_h_eq_instance"] and len(core) == len(runs) == 2
               and all(l["L_eq_t"] and l["LG_eq_T"] for l in c["logs"]) and all(all(d.values()) for d in c["counts_eq_committed"])
               and all(c["tamper_caught"].values()))
    out["genus5_export"].append(c)

logs = [l for c in out["genus4_valpo"] + out["genus5_export"] for l in c["logs"]]
out["summary"] = {"keys": len(out["genus4_valpo"]) + len(out["genus5_export"]), "logs_checked": len(logs),
                  "all_logs_eq_t_and_LG_eq_T": bool(logs) and all(l["L_eq_t"] and l["LG_eq_T"] for l in logs),
                  "all_keys_ok": all(c["ok"] for c in out["genus4_valpo"] + out["genus5_export"]),
                  "genus4_keys": len(out["genus4_valpo"]), "genus5_keys": len(out["genus5_export"])}
json.dump(out, open(os.path.join(OUT, "certs.json"), "w"), indent=1)
print(json.dumps(out["summary"], indent=1))
for c in out["genus4_valpo"] + out["genus5_export"]:
    print(c["tag"], "ok" if c["ok"] else "FAIL", [(l["L_eq_t"], l["LG_eq_T"]) for l in c["logs"]], c["tamper_caught"],
          c.get("harness_G_T_t_eq_validator"), c.get("counts_eq_committed"))
