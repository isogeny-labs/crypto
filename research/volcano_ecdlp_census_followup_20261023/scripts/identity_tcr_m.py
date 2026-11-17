#!/usr/bin/env python3
"""Identity checks for Tcr and M (the new families) and extra checks on ctl and N. Reads `ecbench plan --json` output
(plans in $SP/t/plans) and the census files; prints one JSON line per spec: {spec, family, curves, targets, mismatches, ...}.
Every mismatch counts: a target that is not the expected point, a generator not on its curve or not of order r, a model whose
j differs from its vertex's."""
import glob, json, os, sys
STUDY = os.environ["STUDY"]; SP = os.environ["SP"]
sys.path.insert(0, f"{STUDY}/scripts")
import importlib.util
spec = importlib.util.spec_from_file_location("tcr_m", f"{STUDY}/scripts/tcr_m.py"); T = importlib.util.module_from_spec(spec); spec.loader.exec_module(T)
def hx(s): return int(s, 16)
def plan(n): return json.load(open(f"{SP}/t/plans/{n}.json"))
def jl(p): return [json.loads(l) for l in open(p) if l.strip()]
out = []
# ---- Tcr: target_i == carried [k_i]psi_v(gn_v) on the census crater model
tcr = {t["id"]: t for t in jl(f"{STUDY}/census/V5/tcr.jsonl")}
for f in sorted(glob.glob(f"{STUDY}/specs/V5-Tcr-*.json")):
    n = os.path.basename(f)[:-5]; vid = int(n.rsplit("-c", 1)[1]); t = tcr[vid]; pl = plan(n); bad = 0; tg = 0
    for w in pl["workloads"]:
        cs = w["curve_spec"]; exp = T.pt(t["hk"][w["target_index"]])
        ok = (cs["a"] == 0 and (cs["gx"], cs["gy"]) == T.pt(t["g"]) and (hx(w["target"][0]), hx(w["target"][1])) == exp and
              w["planted"] == t["ks"][w["target_index"]] and T.on_curve(exp, 0, cs["b"], cs["p"]))
        tg += 1; bad += 0 if ok else 1
    out.append({"spec": n, "family": "Tcr", "curves": 1, "targets": tg, "mismatches": bad})
# ---- M: model isomorphic to the vertex curve; generator on it, order r, j equal; target_i == [k_i] G
for V in ("V5", "V6"):
    D = {x["id"]: x for x in jl(f"{STUDY}/census/{V}/transport/D.jsonl")}
    for c in json.load(open(f"{STUDY}/census/{V}/M.json"))["curves"]:
        n = f"{V}-M-L{c['level']}-c{c['id']}"; pl = plan(n); bad = 0; tg = 0; models = set(); v = D[c["id"]]
        for w in pl["workloads"]:
            cs = w["curve_spec"]; p, a, b = cs["p"], cs["a"], cs["b"]; G = (cs["gx"], cs["gy"]); models.add((a, b))
            exp = T.mul(w["planted"], G, a, p)
            ok = (T.on_curve(G, a, b, p) and T.mul(cs["r"], G, a, p) is None and T.jinv(a, b, p) == v["j"] == c["j"] and cs["r"] == v["r"]
                  and cs["group_order"] == v["order"] and (hx(w["target"][0]), hx(w["target"][1])) == exp
                  and any((m["a"], m["b"], m["gx"], m["gy"]) == (a, b, G[0], G[1]) for m in c["models"]))
            # model m is (u^4 a, u^6 b) with generator (u^2 x, u^3 y) of gn
            mm = [m for m in c["models"] if (m["a"], m["b"]) == (a, b)]
            if mm:
                u = mm[0]["u"]; gn = T.pt(v["gn"])
                ok = ok and u not in (1, p - 1) and (a, b) == (pow(u, 4, p) * v["a"] % p, pow(u, 6, p) * v["b"] % p) and G == (u * u % p * gn[0] % p, pow(u, 3, p) * gn[1] % p)
            else: ok = False
            tg += 1; bad += 0 if ok else 1
        if len(models) != 8: bad += 1
        out.append({"spec": n, "family": "M", "curves": len(models), "targets": tg, "mismatches": bad})
# ---- extra: ctl (generator and targets are the (u^2, u^3) image of D's g and hk) and N (gn on the vertex curve, order r)
for V in ("V5", "V6"):
    D = jl(f"{STUDY}/census/{V}/transport/D.jsonl"); byab = {(x["a"], x["b"]): x for x in D}
    ctl = {(k["a"], k["b"]): (x, k) for x in D for k in x["controls"]}
    ks = jl(f"{STUDY}/census/{V}/ks.txt") if False else [int(s) for s in open(f"{STUDY}/census/{V}/ks.txt").read().strip().split(",")]
    for f in sorted(glob.glob(f"{STUDY}/specs/{V}-ctl-*.json")) + sorted(glob.glob(f"{STUDY}/specs/{V}-N-*.json")):
        n = os.path.basename(f)[:-5]; fam = n.split("-")[1]; pl = plan(n); bad = 0; tg = 0; curves = set()
        for w in pl["workloads"]:
            cs = w["curve_spec"]; p, a, b = cs["p"], cs["a"], cs["b"]; G = (cs["gx"], cs["gy"]); curves.add((a, b, G)); T_ = (hx(w["target"][0]), hx(w["target"][1]))
            if fam == "ctl":
                x, k = ctl[(a, b)]; u = k["u"]; hk = T.pt(x["hk"][w["target_index"]])
                ok = (G == T.pt(k["g"]) and T.on_curve(G, a, b, p) and T.jinv(a, b, p) == x["j"] and
                      T_ == (u * u % p * hk[0] % p, pow(u, 3, p) * hk[1] % p) and pow(u, 4, p) * x["a"] % p == a and pow(u, 6, p) * x["b"] % p == b)
            else:
                x = byab[(a, b)]
                ok = G == T.pt(x["gn"]) and T.on_curve(G, a, b, p) and T.mul(cs["r"], G, a, p) is None and T_ == T.mul(w["planted"], G, a, p)
            tg += 1; bad += 0 if ok else 1
        out.append({"spec": n, "family": fam + "-extra", "curves": len(curves), "targets": tg, "mismatches": bad})
for o in out: print(json.dumps(o, separators=(",", ":")))
