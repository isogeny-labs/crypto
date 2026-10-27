# Row 67 recompute: 20261015-ic-improve2-02-freerel-production. Reads committed files only. Run from repo root with python3 -I.
import csv, json, os, statistics
D = "research/ic_improve_20261015/02-freerel-production/data"
rows = list(csv.DictReader(open(os.path.join(D, "rows.csv"))))
test = [r for r in rows if r["split"] == "test"]
res = json.load(open("research/ic_improve_20261015/02-freerel-production/results.json"))
KD = {n: res["sizes"]["n%d" % n]["K"] for n in (41, 53, 61)}
print("K per size:", KD)
def sel(n, arm, k, col):
    return [float(r[col]) for r in test if int(r["n"]) == n and r["arm"] == arm and int(r["K"]) == k]
def mean(x): return statistics.fmean(x)
tot_free = tot_inc = 0.0
for n in (41, 53, 61):
    kf, ki = KD[n]["K_free"], KD[n]["K_inc"]
    f = mean(sel(n, "free", kf, "c_total")); i = mean(sel(n, "inc", ki, "c_total"))
    wf = mean(sel(n, "free", kf, "w_total")); wi = mean(sel(n, "inc", ki, "w_total"))
    tot_free += f; tot_inc += i
    print("n=%d seeds=%d free@%d/inc@%d counted=%.5f wall=%.5f" % (n, len(sel(n, "free", kf, "c_total")), kf, ki, f / i, wf / wi))
print("pooled b counted (sum of per-size means) = %.5f" % (tot_free / tot_inc))
# same-K pooled ratios
for label, kf in [("K_inc", lambda n: KD[n]["K_inc"]), ("K_free", lambda n: KD[n]["K_free"]), ("K_def", lambda n: KD[n]["K_def"])]:
    num = den = 0.0
    for n in (41, 53, 61):
        k = kf(n)
        num += mean(sel(n, "free", k, "c_total")) if sel(n, "free", k, "c_total") else float("nan")
        den += mean(sel(n, "inc", k, "c_total"))
    print("same-K pooled (%s) free/inc = %.5f" % (label, num / den))
# incumbent retune alone at n=41
print("inc retune n41 K_inc/K_def = %.5f" % (mean(sel(41, "inc", KD[41]["K_inc"], "c_total")) / mean(sel(41, "inc", KD[41]["K_def"], "c_total"))))
print("stored results.json sizes.n41.inc_tuned_vs_inc_K_def ratio:", res["sizes"]["n41"]["inc_tuned_vs_inc_K_def"].get("ratio", None) if isinstance(res["sizes"]["n41"]["inc_tuned_vs_inc_K_def"], dict) else None)
print("stored pooled b counted ratio:", res["pooled"]["b_tuned_vs_tuned"]["counted_total"]["ratio"], "wall:", res["pooled"]["b_tuned_vs_tuned"]["wall_total"]["ratio"])
print("stored pooled same-K counted (K_inc, K_free, K_def):", res["pooled"]["a_same_K_K_inc"]["counted_total"]["ratio"], res["pooled"]["a_same_K_K_free"]["counted_total"]["ratio"], res["pooled"]["a_same_K_K_def"]["counted_total"]["ratio"])
# per-size stored b counted
for n in (41, 53, 61):
    print("stored n=%d b counted:" % n, res["sizes"]["n%d" % n]["b_tuned_vs_tuned"].get("ratio", None))
# wall pooled
wf = wi = 0.0
for n in (41, 53, 61):
    wf += mean(sel(n, "free", KD[n]["K_free"], "w_total")); wi += mean(sel(n, "inc", KD[n]["K_inc"], "w_total"))
print("pooled b wall (sum of per-size means) = %.5f" % (wf / wi))
# A/A opt-out
aa = json.load(open(os.path.join(D, "aa_optout.json")))
print("aa_optout pairs_checked=%s pairs_identical=%s rule=%s" % (aa["pairs_checked"], aa["pairs_identical"], aa["rule"]))
# census
for f in ["torsion_n41_K255.json", "torsion_n53_K220.json", "torsion_n61_K500.json"]:
    c = json.load(open(os.path.join(D, "census", f)))
    print(f, "torsion_keys_looked_up=%s torsion_table_matches=%s torsion_relations_verified=%s control_ok=%s/%s" % (c["torsion_keys_looked_up"], c["torsion_table_matches"], c["torsion_relations_verified"], c["control_x_of_q_plus_t2_ok"], c["control_x_of_q_plus_t2_checked"]))
# verified run count
import glob
runs_test = sum(1 for r in test if True)
print("rows.csv test rows:", len(test), "train rows:", sum(1 for r in rows if r["split"] == "train"))
import json as _j
verified = 0; total = 0
for f in glob.glob(os.path.join(D, "runs", "*.jsonl")):
    if "_aa" in os.path.basename(f) or "summary" in f:
        continue
    for line in open(f):
        if not line.strip():
            continue
        rec = _j.loads(line)
        if rec.get("kind") == "compact_orbit_dlp_target":
            total += 1; verified += int(bool(rec.get("group_verified")))
print("target records in non-A/A run files: %d, group_verified true: %d" % (total, verified))
