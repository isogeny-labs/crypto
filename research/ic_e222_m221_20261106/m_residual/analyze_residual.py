#!/usr/bin/env python3
"""Residual of the exact pair-table model on the large sample (unvalidated, pre-gate).
Per instance i (K = 4 seeds x (fb+2) relations per arm = 136 geometric trial counts per arm):
   obs_i   = log( sum_seeds trials_on / sum_seeds trials_off )
   pred_i  = log( P_off / P_on )                      exact model, P from the actual tables
   delta_i = (P_on - P_off) / (2K)                    second-order bias of log of a ratio of geometric sums
   var_i   = ((1-P_on) + (1-P_off)) / K               delta-method variance of the same
   resid_i = obs_i - pred_i - delta_i
Reports mean resid with (a) bootstrap CI over instances and (b) the model's own standard error
sqrt(sum var_i)/N; per family and per rung; and the paired-median statistic of the earlier notes.
Usage: analyze_residual.py ROWS.jsonl MODEL.json OUT.json"""
import json, sys, math, random, statistics, collections
rows = [json.loads(l) for l in open(sys.argv[1])]
model = {r["name"]: r for r in json.load(open(sys.argv[2]))}
bad = [r for r in rows if not r["ok"] or r["status"] != "complete"]
nofb = [r for r in rows if r["rels"] != r["fb"] + 2]
obs = collections.defaultdict(lambda: collections.defaultdict(dict))   # name -> seed -> arm -> trials
for r in rows:
    obs[r["name"]][r["seed"]][r["arm"]] = r["trials"]
SEEDS = sorted({r["seed"] for r in rows})
K = len(SEEDS) * 34

def boot_mean(xs, rng, B=4000):
    ms = sorted(statistics.mean([rng.choice(xs) for _ in xs]) for _ in range(B))
    return ms[int(0.025 * B)], ms[int(0.975 * B) - 1]

out = {"n_rows": len(rows), "unrecovered_rows": len(bad), "rows_not_fb_plus_2_relations": len(nofb), "seeds": SEEDS, "K": K, "families": {}}
inst_out = []
for fam in ("M", "E"):
    ms = [m for m in model.values() if m["family"] == fam]
    if not ms:
        continue
    recs = []
    for m in ms:
        o = obs[m["name"]]
        on = sum(o[s]["on"] for s in SEEDS); off = sum(o[s]["off"] for s in SEEDS)
        Pon, Poff = m["P_on"], m["P_off"]
        r = math.log(on / off) - math.log(Poff / Pon) - (Pon - Poff) / (2 * K)
        v = ((1 - Pon) + (1 - Poff)) / K
        # earlier statistic: paired ratio per (instance, seed), first two seeds present
        pr = [o[s]["on"] / o[s]["off"] for s in SEEDS[:2]]
        recs.append(dict(name=m["name"], rung=m["rung"], resid=r, var=v, obs=on / off, pred=Poff / Pon, pairs12=pr, h=m["h"], Pon=Pon, Poff=Poff))
        inst_out.append(dict(name=m["name"], family=fam, rung=m["rung"], obs_pooled=on / off, pred=Poff / Pon, resid_log=r, var=v))
    rng = random.Random("resid-" + fam)
    def summ(sub):
        res = [x["resid"] for x in sub]
        mean = statistics.mean(res); lo, hi = boot_mean(res, rng)
        se_model = math.sqrt(sum(x["var"] for x in sub)) / len(sub)
        pairs = [q for x in sub for q in x["pairs12"]]
        return dict(n=len(sub), mean_log_resid=mean, boot_ci95=[lo, hi], model_se=se_model, z_model=mean / se_model,
                    mean_pct=100 * (math.exp(mean) - 1), emp_sd_of_resid=statistics.pstdev(res), model_sd_of_resid=math.sqrt(statistics.mean([x["var"] for x in sub])),
                    median_obs_pooled=statistics.median([x["obs"] for x in sub]), median_pred=statistics.median([x["pred"] for x in sub]),
                    median_paired_seeds12=statistics.median(pairs),
                    pooled_sum_ratio=sum(x["obs"] * 1 for x in sub) / len(sub))
    fo = dict(overall=summ(recs), by_rung={})
    for rung in sorted({x["rung"] for x in recs}):
        fo["by_rung"][str(rung)] = summ([x for x in recs if x["rung"] == rung])
    # residual vs sample size: first N instances (in name order within each rung) -> does the mean shrink?
    ordered = sorted(recs, key=lambda x: (x["rung"], int(x["name"].split("-")[1])))
    conv = {}
    for frac in (0.1, 0.25, 0.5, 1.0):
        sub = []
        for rung in sorted({x["rung"] for x in recs}):
            rr = [x for x in ordered if x["rung"] == rung]
            sub += rr[: max(1, int(len(rr) * frac))]
        res = [x["resid"] for x in sub]
        conv[str(frac)] = dict(n=len(sub), mean=statistics.mean(res), ci=boot_mean(res, rng, 2000))
    fo["growing_sample"] = conv
    out["families"][fam] = fo
json.dump(out, open(sys.argv[3], "w"), indent=1)
json.dump(inst_out, open(sys.argv[3].replace(".json", "_instances.json"), "w"))
print("rows", len(rows), "unrecovered", len(bad), "rows with relations != fb+2:", len(nofb))
for fam, fo in out["families"].items():
    s = fo["overall"]
    print(f"{fam}: n={s['n']}  mean log resid {s['mean_log_resid']:+.4f} ({s['mean_pct']:+.2f}%)  boot CI [{s['boot_ci95'][0]:+.4f},{s['boot_ci95'][1]:+.4f}]  "
          f"model se {s['model_se']:.4f}  z {s['z_model']:+.2f}  sd resid emp {s['emp_sd_of_resid']:.3f} vs model {s['model_sd_of_resid']:.3f}  "
          f"median obs {s['median_obs_pooled']:.4f} pred {s['median_pred']:.4f}  paired-median(seeds1,2) {s['median_paired_seeds12']:.4f}")
    for rung, s in fo["by_rung"].items():
        print(f"   rung {rung:>2}: n={s['n']:>3} mean resid {s['mean_log_resid']:+.4f} ({s['mean_pct']:+.2f}%) CI [{s['boot_ci95'][0]:+.4f},{s['boot_ci95'][1]:+.4f}] z {s['z_model']:+.2f}  med obs {s['median_obs_pooled']:.4f} pred {s['median_pred']:.4f}")
    print("   growing sample:", {k: (v['n'], round(v['mean'], 4), [round(c, 4) for c in v['ci']]) for k, v in fo["growing_sample"].items()})
